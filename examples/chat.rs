use anyhow::Result;
use dashmap::DashMap;
use futures::{SinkExt, StreamExt, stream::SplitStream};
use std::{fmt, net::SocketAddr, sync::Arc};
use tokio::{
    net::{TcpListener, TcpStream},
    sync::mpsc,
};
use tokio_util::codec::{Framed, LinesCodec};
use tracing::{info, level_filters::LevelFilter, warn};
use tracing_subscriber::{Layer as _, fmt::Layer, layer::SubscriberExt, util::SubscriberInitExt};

const MAX_MESSAGES: usize = 128;
#[derive(Debug, Default)]
struct State {
    peers: DashMap<SocketAddr, mpsc::Sender<Arc<Message>>>,
}

#[derive(Debug)]
struct Peer {
    username: String,
    stream: SplitStream<Framed<TcpStream, LinesCodec>>,
}
#[derive(Debug)]
enum Message {
    UserJoined(String),
    UserLeft(String),
    Chat{
        sender: String,
        content: String,
    }
}

#[tokio::main]

async fn main() -> Result<()> {
    let layer = Layer::new().with_filter(LevelFilter::INFO);
    tracing_subscriber::registry().with(layer).init();

    let addr = "0.0.0.0:8080";
    let listener = TcpListener::bind(addr).await?;
    info!("Starting chat server on {}", addr);
    let state = Arc::new(State::default()); //arc clone by reference count
    loop {
        let (stream, addr) = listener.accept().await?;
        info!("New connection from {}", addr);
        let state_cloned = state.clone();
        tokio::spawn(async move {
            if let Err(e) = handle_client(state_cloned, addr, stream).await {
                warn!("Error handling client {}: {}", addr, e);
            }
        });
    }
}

async fn handle_client(state: Arc<State>, addr: SocketAddr, stream: TcpStream) -> Result<()> {
    let mut framed = Framed::new(stream, LinesCodec::new());
    framed.send("Enter your username:".to_string()).await?;
    let username = match framed.next().await {
        Some(Ok(name)) => name,
        Some(Err(e)) => return Err(e.into()),
        None => return Ok(()),
    };

    let mut peer = state.add(addr, framed, username).await;

    let message = Arc::new(Message::user_joined(&peer.username));
    state.broadcast(addr, message).await;
    
    while let Some(line) = peer.stream.next().await {
        let line = match line {
            Ok(line) => line,
            Err(e) => {
                warn!("Failed to read line from {}: {}", addr, e);
                break;
            }
        };

        let message = Arc::new(Message::chat(&peer.username, &line));
        state.broadcast(addr, message).await;
    }

    //when loop exits, peer has left chat or line reading failed
    state.peers.remove(&addr);

    //notify other peers that this peer has left
    let message = Arc::new(Message::user_left(&peer.username));
    info!("{}", message);
    state.broadcast(addr, message).await;
    
    state.peers.remove(&addr);
    Ok(())
}

impl State {
    async fn broadcast(&self, addr: SocketAddr, message: Arc<Message>) {
        for peer in self.peers.iter() {
            if peer.key() == &addr {
                continue;
            }
            if let Err(e) = peer.value().send(message.clone()).await {
                warn!("Failed to send message to {}: {}", peer.key(), e);
                self.peers.remove(peer.key()); //remove peer if sending fails
            }
        }
    }
    //add a new peer and receive messages from other peers and then send them to the client
    async fn add(
        &self,
        addr: SocketAddr,
        stream: Framed<TcpStream, LinesCodec>,
        username: String,
    ) -> Peer {
        let (tx, mut rx) = mpsc::channel(MAX_MESSAGES);
        self.peers.insert(addr, tx);

        let (mut stream_sender, stream_receiver) = stream.split();
        tokio::spawn(async move {
            while let Some(message) = rx.recv().await {
                if let Err(e) = stream_sender.send(message.to_string()).await {
                    warn!("Failed to send message to {}: {}", addr, e);
                    break;
                }
            }
        });
        return Peer {
            username,
            stream: stream_receiver, //that why we split the stream
        };
    }
}

impl Message {
    fn user_joined(username: &str) -> Self {
        let content = format!("{} has joined the chat", username);
        Self::UserJoined(content)
    }

    fn user_left(username: &str) -> Self {
        let content = format!("{} has left the chat", username);
        Self::UserLeft(content)
    }

    fn chat(sender: impl Into<String>, content: impl Into<String>) -> Self {
        Self::Chat {
            sender: sender.into(),
            content: content.into(),
        }
    }
}


impl fmt::Display for Message {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Message::UserJoined(content) => write!(f, "{}", content),
            Message::UserLeft(content) => write!(f, "{}", content),
            Message::Chat { sender, content } => write!(f, "{} {}", sender, content),
        }
    }
}
