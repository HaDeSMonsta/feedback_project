use crate::comm::communication_server::{Communication, CommunicationServer};
use crate::comm::{MsgRequest, MsgResponse};
use anyhow::{Context, Result};
use chrono::Utc;
use std::cell::LazyCell;
use std::env;
use std::fs::OpenOptions;
use std::io::{BufWriter, Write};
use std::net::{IpAddr, Ipv6Addr, SocketAddr};
use std::sync::Mutex;
use tonic::transport::Server;
use tonic::{Request, Response, Status};
use tracing::{debug, error, info, subscriber, Level};
use tracing_subscriber::FmtSubscriber;

const FILE_PATH: &'static str = "/feedback/";
const FILE_NAME: &'static str = "feedback.txt";
const PORT: u16 = 8080;

const LOG_LEVEL: LazyCell<Level> = LazyCell::new(|| {
    const ENV_KEY: &'static str = "LOG_LEVEL";
    let Ok(log_level) = env::var(ENV_KEY) else {
        #[cfg(debug_assertions)]
        return Level::DEBUG;
        #[cfg(not(debug_assertions))]
        return Level::INFO;
    };
    match log_level.trim().to_lowercase().as_str() {
        "trace" => Level::TRACE,
        "debug" => Level::DEBUG,
        "info" => Level::INFO,
        "warn" => Level::WARN,
        "error" => Level::ERROR,
        _ => {
            println!("WARNING: {ENV_KEY} is set, but the value is invalid, using default");
            #[cfg(debug_assertions)]
            return Level::DEBUG;
            #[cfg(not(debug_assertions))]
            return Level::INFO;
        }
    }
});

pub mod comm {
    tonic::include_proto!("comm");
}

pub struct CommService {
    lock: Mutex<()>,
    pwd: String,
}

#[tonic::async_trait]
impl Communication for CommService {
    async fn send_msg(
        &self,
        request: Request<MsgRequest>,
    ) -> Result<Response<MsgResponse>, Status> {
        info!("New connection");

        debug!("Got request: {request:?}");

        let req = request.into_inner();

        if req.auth != self.pwd {
            info!("Invalid authentication");
            debug!("Password: {}", req.auth);
            debug!("Msg: {}", &req.msg);
            return Err(Status::unauthenticated("Invalid authentication"));
        }

        info!("Valid password");
        debug!("Locking");
        let _lock = self.lock.lock().unwrap();

        info!("Got msg: {}", &req.msg);

        if let Err(e) = logic(&req.msg) {
            error!("Got error: {e}");
            let res = Status::internal(format!("An error occurred: {e}"));
            debug!("Returning err {res}");
            return Err(res);
        }

        info!("Wrote feedback, closing connection");

        Ok(Response::new(MsgResponse {}))
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    subscriber::set_global_default(
        FmtSubscriber::builder()
            .with_max_level(*LOG_LEVEL)
            .finish()
    ).context("Unable to set default subscriber")?;
    debug!("Successfully set default subscriber");

    debug!("Checking environment variables");
    const PWD_KEY: &str = "PWD";
    let pwd = env::var(PWD_KEY)
        .with_context(|| format!("Environment variable {PWD_KEY} is not set"))?;
    info!("Required environment variable is set");

    info!("Starting Server");

    const ADDR: SocketAddr = SocketAddr::new(IpAddr::V6(Ipv6Addr::UNSPECIFIED), PORT);
    let lock = Mutex::new(());
    let msg_service = CommService { lock, pwd };

    Server::builder()
        .add_service(CommunicationServer::new(msg_service))
        .serve(ADDR)
        .await
        .context("Server broke :(")
}

fn logic(to_log: &str) -> Result<()> {
    let current_date_str = Utc::now().format("%Y-%m-%d").to_string();
    let file_name = format!("{FILE_PATH}{current_date_str}-{FILE_NAME}");

    debug!("Opening file {file_name}");

    let file = OpenOptions::new()
        .write(true)
        .create(true)
        .append(true)
        .open(&file_name)
        .with_context(|| {
            format!(
                "Unable to open file {file_name} \
        (probably didn't bind the correct path in Docker)"
            )
        })?;

    let mut writer = BufWriter::new(file);

    writeln!(writer, "{}", "-".repeat(50))?;

    let current_datetime_str = Utc::now()
        .format("[%-Y-%m-%d - %-H:%M:%S]z")
        .to_string();

    writeln!(writer, "{current_datetime_str}")?;

    for line in to_log.lines() {
        debug!("Writing line: {line}");
        writeln!(writer, "{line}")?;
    }

    writeln!(writer, "{}\n", "-".repeat(50))?;
    debug!("Finished writing, closing file");
    Ok(())
}
