extern crate logger_utc as logger;

use std::{env, error, io};
use std::fs::OpenOptions;
use std::io::{BufWriter, Write};
use std::sync::{Arc, Mutex};

use chrono::Utc;
use logger::log;
use tonic::{Request, Response, Status};
use tonic::transport::Server;

use crate::comm::{MsgRequest, MsgResponse};
use crate::comm::communication_server::{Communication, CommunicationServer};

const FILE_PATH: &'static str = "/feedback/";
const FILE_NAME: &'static str = "feedback.txt";
const PORT: u16 = 8080;

pub mod comm {
    tonic::include_proto!("comm");
}

#[derive(Debug, Default)]
pub struct CommService {
    lock: Arc<Mutex<()>>,
    pwd: String,
}

#[tonic::async_trait]
impl Communication for CommService {
    async fn send_msg(&self, request: Request<MsgRequest>)
        -> Result<Response<MsgResponse>, Status> {
        log("New connection");

        log(format!("Got request: {:?}", &request));

        let req = request.into_inner();

        if req.auth != self.pwd {
            log(format!("Invalid password: {}", req.auth));
            let e = Status::unauthenticated("Invalid authentication");
            return Err(e);
        }

        log("Valid password");

        log(format!("Got msg: {}", &req.msg));

        let res;
        {
            let _lock = self.lock.lock().unwrap();
            res = match logic(&req.msg) {
                Ok(_) => MsgResponse {
                    code: 202,
                    msg: String::from("Msg received"),
                },
                Err(e) => {
                    log(format!("Got error: {e}"));
                    let res = Status::internal(
                        format!("An error occurred: {e}")
                    );
                    log(format!("Returning {res}"));
                    return Err(res);
                }
            }
        }

        log(format!("Created response {res:?}, closing connection"));

        Ok(Response::new(res))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn error::Error>> {
    log("Checking environment variables");
    let _ = env::var("PWD");
    log("Environment variables are set");

    log("Starting Server");

    let pwd = env::var("PWD").expect("PWD must be set");

    let addr = format!("0.0.0.0:{PORT}").parse()?;
    let lock = Arc::new(Mutex::new(()));
    let msg_service = CommService {
        lock,
        pwd,
    };

    Server::builder()
        .add_service(CommunicationServer::new(msg_service))
        .serve(addr)
        .await?;

    Ok(())
}

fn logic(to_log: &str) -> Result<(), Box<dyn error::Error>> {
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
        .map_err(|_| -> Box<dyn error::Error> {
            let err_msg = String::from(
                "Unable to open file (probably didn't bind the correct path in Docker)"
            );
            Box::new(io::Error::new(
                io::ErrorKind::NotFound, err_msg,
            ))
        })?;

    let mut writer = BufWriter::new(file);

    writeln!(writer, "{}", "-".repeat(50))?;

    let current_datetime_str = Utc::now()
        .format("[%-Y-%m-%d - %-H:%M:%S]z")
        .to_string();

    writeln!(writer, "{current_datetime_str}")?;

    for line in to_log.lines() {
        log(format!("Writing line: {line}"));
        writeln!(writer, "{line}")?;
    }

    writeln!(writer, "{}\n", "-".repeat(50))?;
    log("Finished writing, closing file");
    Ok(())
}
