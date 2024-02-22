extern crate logger_utc as logger;

use std::{env, error};
use std::fs::OpenOptions;
use std::io::{BufWriter, Write};
use std::sync::{Arc, Mutex};

use chrono::Utc;
use lazy_static::lazy_static;
use logger::log;
use tonic::{Request, Response, Status};
use tonic::transport::Server;

use comm::communication_server::Communication;

use crate::comm::{MsgRequest, MsgResponse};
use crate::comm::communication_server::CommunicationServer;

const FILE_PATH: &'static str = "/feedback/";
const FILE_NAME: &'static str = "feedback.txt";
const PORT: u16 = 8080;

lazy_static! {
    static ref LOCK: Arc<Mutex<()>> = Arc::new(Mutex::new(()));
}

pub mod comm {
    tonic::include_proto!("comm");
}

#[derive(Debug, Default)]
pub struct CommService {}

#[tonic::async_trait]
impl Communication for CommService {
    async fn send_msg(&self, request: Request<MsgRequest>)
                      -> Result<Response<MsgResponse>, Status> {
        log("New connection");

        let pwd: String = env::var("PWD").expect("PWD must be set");

        let req = request.into_inner();

        if req.auth != pwd {
            log(&format!("Invalid password: {}", req.auth));
            let res = MsgResponse {
                success: false,
                err_msg: Some(String::from("Invalid password")),
            };
            return Ok(Response::new(res));
        }

        log("Valid password");

        logic(&req.msg);

        let res = MsgResponse {
            success: true,
            err_msg: None,
        };

        Ok(Response::new(res))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn error::Error>> {
    println!("Starting Server");

    let addr = format!("0.0.0.0:{PORT}").parse()?;
    let msg_service = CommService::default();
    
    Server::builder()
        .add_service(CommunicationServer::new(msg_service))
        .serve(addr)
        .await?;
    
    Ok(())
}

fn logic(to_log: &str) {
    let lock = Arc::clone(&*LOCK);
    let _lock = lock.lock().unwrap(); // Get lock

    let current_date_str = Utc::now()
        .format("%Y-%m-%d")
        .to_string();
    let file_name = format!("{FILE_PATH}{current_date_str}-{FILE_NAME}");

    let file = OpenOptions::new()
        .write(true)
        .create(true)
        .append(true)
        .open(file_name)
        .expect("Unable to open file (probably didn't bind the correct path in Docker)");

    let mut writer = BufWriter::new(file);

    writeln!(writer, "{}", "-".repeat(50)).unwrap();

    let current_datetime_str = Utc::now()
        .format("[%-Y-%m-%d - %-H:%M:%S]z")
        .to_string();

    writeln!(writer, "{current_datetime_str}").unwrap();

    for line in to_log.lines() {
        log(&format!("Got line: {line}"));
        writeln!(writer, "{line}").unwrap();
    }

    writeln!(writer, "{}\n", "-".repeat(50)).unwrap();
    log("Finished writing, closing connection");
} // Drop lock
