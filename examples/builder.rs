use chrono::{DateTime, Datelike, Utc};
use derive_builder::Builder;
use anyhow::{Result};

#[allow(unused)]
#[derive(Builder, Debug)]
#[builder(build_fn(name = "_priv_build"))]
struct User {
    #[builder(setter(into))]
    name: String,
    #[builder(setter(into, strip_option))]
    email: Option<String>,
    #[builder(setter(custom))]
    dob: DateTime<Utc>,
    #[builder(setter(skip))]
    age: u32,
    #[builder(default = "vec![]", setter(each(name = "skill", into)))]
    skills: Vec<String>,
}

fn main() -> Result<()> {
    //let user = UserBuilder::default()
    let user = User::build()
        .name("Alice")
        .email("xxxxxxxxx")
        .dob("2002-02-15T00:00:00Z" )
        .skill("Rust")
        .skill("Go")
        .cal_age()?;

    println!("{:?}", user);
    Ok(())
}

impl User {
    fn build() -> UserBuilder {
            UserBuilder::default()
    }
}

impl UserBuilder {
    pub fn cal_age(&self) -> Result<User> {
        let mut user = self._priv_build()?;
        user.age = (Utc::now().year() - user.dob.year()) as _;
        Ok(user)
    }
    pub fn dob(&mut self, dob: &str) -> &mut Self {
        let parsed_dob = dob.parse::<DateTime<Utc>>().expect("Invalid date format");
        self.dob = Some(parsed_dob);
        self
    }
}