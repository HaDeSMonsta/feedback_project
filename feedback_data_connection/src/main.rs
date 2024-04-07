extern crate logger_utc as logger;

use std::env;
use std::error::Error;
use std::fs::OpenOptions;
use std::io::{BufWriter, Write};
use std::sync::{Arc, Mutex};

use chrono::Utc;
use logger::{log, log_string};
use tonic::{Request, Response, Status};
use tonic::transport::Server;

use comm::communication_server::Communication;

use crate::comm::{MsgRequest, MsgResponse};
use crate::comm::communication_server::CommunicationServer;

const FILE_PATH: &'static str = "/feedback/";
const FILE_NAME: &'static str = "feedback.txt";
const PORT: u16 = 8080;

pub mod comm {
    tonic::include_proto!("comm");
}

#[derive(Debug, Default)]
pub struct CommService {
    lock: Arc<Mutex<()>>,
}

#[tonic::async_trait]
impl Communication for CommService {
    async fn send_msg(&self, request: Request<MsgRequest>)
        -> Result<Response<MsgResponse>, Status> {
        log("New connection");

        let pwd = env::var("PWD").expect("PWD must be set");

        log_string(format!("Got request: {:?}", &request));

        let req = request.into_inner();

        if req.auth != pwd {
            log_string(format!("Invalid password: {}", req.auth));
            let e = Status::unauthenticated("Invalid authentication");
            return Err(e);
        }

        log("Valid password");

        log_string(format!("Got msg: {}", &req.msg));

        {
            let _lock = self.lock.lock().unwrap();
            logic(&req.msg);
        }

        let res = MsgResponse {};

        Ok(Response::new(res))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    log("Checking environment variables");
    let _ = env::var("PWD");
    log("Environment variables are set");

    log("Starting Server");

    let addr = format!("0.0.0.0:{PORT}").parse()?;
    let lock = Arc::new(Mutex::new(()));
    let msg_service = CommService {
        lock,
    };

    Server::builder()
        .add_service(CommunicationServer::new(msg_service))
        .serve(addr)
        .await?;

    Ok(())
}

fn logic(to_log: &str) {

    let current_date_str = Utc::now()
        .format("%Y-%m-%d")
        .to_string();
    let file_name = format!("{FILE_PATH}{current_date_str}-{FILE_NAME}");

    log("Opening file");

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
        log_string(format!("Writing line: {line}"));
        writeln!(writer, "{line}").unwrap();
    }

    writeln!(writer, "{}\n", "-".repeat(50)).unwrap();
    log("Finished writing, closing connection");
}
