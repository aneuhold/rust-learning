use reqwest;
use std::error::Error;
use std::io::prelude::*;
use std::net::TcpStream;

/// Uses the simpler crate reqwest to make a request to the Rust in Action
/// website.
#[allow(dead_code)]
pub fn run_reqwest_request() -> Result<(), Box<dyn Error>> {
    let url = "http://www.rustinaction.com/";
    let mut response = reqwest::get(url)?;
    let content = response.text()?;
    print!("{}", content);

    Ok(())
}

/// Runs a manual get request using the built in TcpStream type.
#[allow(dead_code)]
pub fn run_manual_get_request() -> std::io::Result<()> {
    let host = "www.rustinaction.com:80";
    let mut conn = TcpStream::connect(host)?;
    conn.write_all(b"GET / HTTP/1.0")?;
    conn.write_all(b"\r\n")?;

    conn.write_all(b"Host: www.rustinaction.com")?;
    // Two blank new lines signify end of request
    conn.write_all(b"\r\n\r\n")?;

    std::io::copy(&mut conn, &mut std::io::stdout())?;
    Ok(())
}
