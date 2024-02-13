use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::net::TcpStream;
use std::io::Result;

pub fn send_msg(msg: String, ip_path: &str, port: u16) -> Result<()> {

    let ip = read_ip_from_file(ip_path)?;

    let mut stream = TcpStream::connect(format!("{ip}:{port}"))?;
    stream.write_all(msg.trim().as_bytes())?;
    stream.flush()
}

fn read_ip_from_file(path: &str) -> Result<String> {
    let file = File::open(path)?;
    let mut buf_reader = BufReader::new(file);
    let mut ip = String::new();
    buf_reader.read_to_string(&mut ip)?;
    Ok(
        ip
            .trim()
            .to_string()
            .replace("set $ip", "")
            .replace(";", "")
            .trim()
            .to_string()
    )
}
