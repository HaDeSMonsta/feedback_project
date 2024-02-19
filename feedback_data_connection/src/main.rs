use std::env;
use std::fs::OpenOptions;
use std::io::{BufRead, BufReader, BufWriter, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;
use std::sync::{Arc, Mutex};
use chrono::Utc;

static FILE_ENDING: &'static str = ".txt";
static FILE_NAME: &'static str = "/feedback/feedback";
static PORT: u16 = 8080;

fn main() {
    println!("Starting Server");

    let listener = TcpListener::bind(("localhost", PORT)).unwrap();

    let mutex = Arc::new(Mutex::new(()));

    loop {
        match listener.accept() {
            Ok((stream, _)) => {
                let clone = Arc::clone(&mutex);
                thread::spawn(move || authenticate(stream, clone));
            }
            Err(e) => {
                eprintln!("Unable to accept connection: {}", e);
            }
        }
    }
}

fn logic(reader: BufReader<TcpStream>, mutex: Arc<Mutex<()>>) {
    let current_date_str = Utc::now().format("%Y-%m-%d").to_string();
    let file_name = format!("{}-{}{}", FILE_NAME, current_date_str, FILE_ENDING);

    {
        let _lock = mutex.lock().unwrap(); // Get lock

        let file = OpenOptions::new()
            .write(true)
            .create(true)
            .append(true)
            .open(file_name)
            .expect("Unable to open file");

        let mut writer = BufWriter::new(file);

        writeln!(writer, "{}", "-".repeat(50)).unwrap();
        let current_datetime_str = Utc::now()
            .format("[%-Y-%m-%d - %-H:%M:%S]Z")
            .to_string();
        writeln!(writer, "{}", current_datetime_str).unwrap();

        let lines: Vec<String> = reader.lines()
                                       .map(|line| line.unwrap())
                                       .collect();

        for line in lines.iter() {
            writeln!(writer, "{}", line).unwrap();
        }

        writeln!(writer, "{}\n", "-".repeat(50)).unwrap();
    } // Drop lock
}

fn authenticate(stream: TcpStream, mutex: Arc<Mutex<()>>) {
    let mut reader = BufReader::new(stream);
    let mut pwd_line = String::new();

    if let Ok(_) = reader.read_line(&mut pwd_line) {
        let pwd = env::var("PWD").expect("Unable to get Password from env");
        if pwd_line.trim() == pwd.trim() { logic(reader, mutex) }
    }
}