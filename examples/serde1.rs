use std::fmt::Display;
use std::str::FromStr;

use anyhow::Result;
use chacha20poly1305::aead::{Aead, OsRng};
use chacha20poly1305::{AeadCore, ChaCha20Poly1305, KeyInit};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use base64::Engine;
use base64::engine::general_purpose::URL_SAFE_NO_PAD;
use serde_with::{serde_as, DisplayFromStr};
const KEY: &[u8] = b"01234567890123456789012345678901";
#[serde_as]
#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
struct User {
    name: String,
    #[serde(rename = "private_age")]
    age: u8,
    date_of_birth: DateTime<Utc>,
    skills: Vec<String>,
    state: WorkState,
    #[serde(serialize_with = "b64_encode", deserialize_with = "b64_decode")]
    data: Vec<u8>,

    // #[serde(serialize_with = "serialize_encrypt", deserialize_with = "deserialize_decrypt")]
    #[serde_as(as = "DisplayFromStr")]
    sensitive: SensitiveData,
    #[serde_as(as = "DisplayFromStr")]
    url: http::Uri,
}
#[derive(Debug,PartialEq)]
struct SensitiveData(String);
#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
enum WorkState {
    Working(String),
    OnLeave(DateTime<Utc>),
    Terminated,
}

fn main() -> Result<()> {
    let state = WorkState::Working("Rust Engineer".to_string());
    //let state1 =
    let user = User {
        name: "Alice".to_string(),
        age: 30,
        date_of_birth: Utc::now(),
        skills: vec!["Rust".to_string(), "Serde".to_string()],
        state,
        data: vec![1, 2, 3, 4, 5],
        sensitive: SensitiveData::new("secret"),
        url: "https://example.com/path?query=1".parse().unwrap(),
    };
    let json = serde_json::to_string(&user)?;
    println!("{}", json);
    let user: User = serde_json::from_str(&json)?;
    println!("{:?}", user);
    println!("{:?}",user.url.host());
    Ok(())
}

fn b64_encode<S>(data: &Vec<u8>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    let encoded = URL_SAFE_NO_PAD.encode(data);
    serializer.serialize_str(&encoded)
}

fn b64_decode<'de, D>(deserializer: D) -> Result<Vec<u8>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let s: &str = serde::Deserialize::deserialize(deserializer)?;
    let decoded = URL_SAFE_NO_PAD
        .decode(s)
        .map_err(serde::de::Error::custom)?;
    Ok(decoded)
}

fn encrypt(data: &[u8]) -> Result<String> {
    let cipher = ChaCha20Poly1305::new(KEY.into());
    let nonce = ChaCha20Poly1305::generate_nonce(&mut OsRng);

    let ciphertext = cipher.encrypt(&nonce, data).unwrap();
    let nonce_cyphertext: Vec<u8> = nonce.to_vec().into_iter().chain(ciphertext.into_iter()).collect();
    let encoded = URL_SAFE_NO_PAD.encode(&nonce_cyphertext);
    Ok(encoded)
}

fn decrypt(data: &str) -> Result<Vec<u8>> {
    let cipher = ChaCha20Poly1305::new(KEY.into());
    let decoded = URL_SAFE_NO_PAD.decode(data.as_bytes())?;
    let nonce = decoded[..12].try_into().unwrap();
    let decrypted = cipher.decrypt(nonce, &decoded[12..]).unwrap();
    Ok(decrypted)
}

// fn serialize_encrypt<S>(data: &String, serializer: S) -> Result<S::Ok, S::Error>
// where
//     S: serde::Serializer,
// {
//     let encrypted = encrypt(data.as_bytes()).map_err(serde::ser::Error::custom)?;
//     serializer.serialize_str(&encrypted)
// }

// fn deserialize_decrypt<'de, D>(deserializer: D) -> Result<String, D::Error>
// where
//     D: serde::Deserializer<'de>,
// {
//     let s: &str = serde::Deserialize::deserialize(deserializer)?;
//     let decrypted = decrypt(s).map_err(serde::de::Error::custom)?;
//     let result = String::from_utf8(decrypted).map_err(serde::de::Error::custom)?;
//     Ok(result)
// }

impl Display for SensitiveData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let encrypted = encrypt(self.0.as_bytes()).unwrap();
        write!(f, "{}", encrypted)
    }
}

impl FromStr for SensitiveData {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let decrypted = decrypt(s)?;
        let result = String::from_utf8(decrypted)?;
        Ok(SensitiveData(result))
    }
}

impl  SensitiveData {
    fn new(s: &str) -> Self {
        SensitiveData(s.to_string())
    }
}