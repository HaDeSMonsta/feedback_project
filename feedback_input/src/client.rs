use std::error;
use std::fs::OpenOptions;
use std::io::{BufReader, prelude::*};

use comm::{communication_client::CommunicationClient, MsgRequest};

mod comm {
    tonic::include_proto!("comm");
}

pub(crate) async fn send_msg(msg: &str, ip_path: &str, port: u16, auth: &str)
    -> Result<(), Box<dyn error::Error>> {
    let ip = read_ip_from_file(ip_path)?;

    let mut client = CommunicationClient::connect(
        format!("https://{ip}:{port}")
    ).await?;

    let msg_string = String::from(msg);
    let auth_string = String::from(auth);

    let request = tonic::Request::new(
        MsgRequest {
            auth: auth_string,
            msg: msg_string,
        }
    );

    client.send_msg(request).await?;

    Ok(())
}

fn read_ip_from_file(path: &str) -> Result<String, Box<dyn error::Error>> {
    let file = OpenOptions::new()
        .read(true)
        .open(path)?;
    let mut buf_reader = BufReader::new(file);
    let mut ip = String::new();
    buf_reader.read_line(&mut ip)?;
    Ok(String::from(ip.trim()))
}
