// CLI CLIENT FOR:
// LINK: https://github.com/DavidBalishyan/chat.rs

use std::env;
use std::io::{self, BufRead, BufReader, Write};
use std::net::TcpStream;
use std::process;
use std::sync::{Arc, atomic::{AtomicBool, Ordering}};
use std::thread;
use regex::Regex;
use ctrlc;

const COLOR_RESET: &str = "\x1b[0m";
const COLOR_RED: &str = "\x1b[31m";
const COLOR_GREEN: &str = "\x1b[32m";
const COLOR_YELLOW: &str = "\x1b[33m";
const COLOR_CYAN: &str = "\x1b[36m";

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("{}Usage: cargo run -- <host:port>{}", COLOR_RED, COLOR_RESET);
        return;
    }

    let server_addr = &args[1];

    if !is_valid_host_port(server_addr) {
        eprintln!("{}Invalid address format. Use <host:port>{}", COLOR_RED, COLOR_RESET);
        return;
    }

    let stream = match TcpStream::connect(server_addr) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("{}Failed to connect:{} {}", COLOR_RED, COLOR_RESET, e);
            return;
        }
    };

    println!("{}Connected to {}{}", COLOR_YELLOW, server_addr, COLOR_RESET);
    println!("{}Type your message and press Enter. Use /quit to exit.{}", COLOR_YELLOW, COLOR_RESET);

    let stream = Arc::new(stream);
    let running = Arc::new(AtomicBool::new(true));
    let reader_running = running.clone();
    let reader_stream = stream.clone();

    // Handle Ctrl+C
    {
        let running = running.clone();
        let stream = stream.clone();
        ctrlc::set_handler(move || {
            let _ = stream.shutdown(std::net::Shutdown::Both);
            running.store(false, Ordering::SeqCst);
            process::exit(0);
        }).expect("Error setting Ctrl-C handler");
    }

    // Spawn a thread to listen for incoming messages
    thread::spawn(move || {
        let reader = BufReader::new(&*reader_stream);
        for line in reader.lines() {
            match line {
                Ok(msg) => {
                    println!("\r{}user: {}{}", COLOR_CYAN, msg, COLOR_RESET);
                    print!("{}You: {}", COLOR_GREEN, COLOR_RESET);
                    let _ = io::stdout().flush();
                },
                Err(_) => {
                    println!("{}Disconnected from server.{}", COLOR_RED, COLOR_RESET);
                    reader_running.store(false, Ordering::SeqCst);
                    process::exit(0);
                }
            }
        }
    });

    // Main loop for user input
    let stdin = io::stdin();
    while running.load(Ordering::SeqCst) {
        print!("{}You: {}", COLOR_GREEN, COLOR_RESET);
        let _ = io::stdout().flush();
        let mut input = String::new();
        if stdin.read_line(&mut input).is_err() {
            break;
        }
        let text = input.trim();
        if text.is_empty() {
            continue;
        }
        if text == "/quit" {
            break;
        }
        if let Err(e) = writeln!(&*stream, "{}", text) {
            eprintln!("{}Error sending message:{} {}", COLOR_RED, COLOR_RESET, e);
            break;
        }
    }

    let _ = stream.shutdown(std::net::Shutdown::Both);
}

fn is_valid_host_port(addr: &str) -> bool {
    let re = Regex::new(r"^([a-zA-Z0-9\.\-]+):([0-9]{1,5})$").unwrap();
    re.is_match(addr)
}
