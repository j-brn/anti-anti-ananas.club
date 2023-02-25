use actix_cors::Cors;
use actix_files::NamedFile;
use actix_web::{
    get,
    middleware::{Compress, NormalizePath},
    web::{self, Data},
    App, HttpResponse, HttpServer, Responder,
};
use anyhow::{anyhow, Context};
use rand::{seq::SliceRandom, thread_rng};
use serde::{Deserialize, Serialize};
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::str::FromStr;
use std::sync::Arc;

#[derive(Debug, Clone)]
struct MessageProvider {
    messages: Arc<Vec<Message>>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
struct Message {
    preamble: String,
    message: String,
}

impl FromStr for Message {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.split_once(';')
            .ok_or_else(|| anyhow!("delimiter ';' not found"))
            .map(|(preamble, message)| Message {
                preamble: preamble.to_owned(),
                message: message.to_owned(),
            })
    }
}

impl MessageProvider {
    pub fn from_messages_file<P: AsRef<Path>>(path: P) -> anyhow::Result<Self> {
        let path = path.as_ref();
        let reader = File::open(path).with_context(|| {
            format!("failed to open message file {path}", path = &path.display())
        })?;

        let messages = BufReader::new(reader)
            .lines()
            .enumerate()
            .map(|(line, data)| {
                let line = data.with_context(|| format!("failed to read line {}", line))?;

                Message::from_str(&line)
            })
            .collect::<anyhow::Result<Vec<Message>>>()?;

        Ok(Self {
            messages: Arc::new(messages),
        })
    }

    pub fn get_random_message(&self) -> Option<&Message> {
        self.messages.choose(&mut thread_rng())
    }
}

#[get("/api/message")]
async fn message_endpoint(provider: web::Data<MessageProvider>) -> HttpResponse {
    HttpResponse::Ok().json(provider.get_random_message().unwrap())
}

#[get("/")]
async fn index() -> impl Responder {
    NamedFile::open("./assets/index.html")
}

#[actix_rt::main]
async fn main() -> anyhow::Result<()> {
    let listen_address = env::var("LISTEN_ADDRESS").context("LISTEN_ADDRESS not set")?;
    let message_file = env::var("MESSAGE_FILE").context("MESSAGE_FILE not set")?;

    let message_provider = Data::new(
        MessageProvider::from_messages_file(message_file)
            .context("failed to create message provider")?,
    );

    HttpServer::new({
        move || {
            App::new()
                .wrap(Compress::default())
                .wrap(NormalizePath::default())
                .wrap(Cors::permissive())
                .app_data(message_provider.clone())
                .service(message_endpoint)
                .service(index)
                .service(actix_files::Files::new("/", "./assets"))
        }
    })
    .bind(&listen_address)
    .with_context(|| format!("failed to bind to {}", &listen_address))?
    .run()
    .await
    .context("failed to start http server")
}
