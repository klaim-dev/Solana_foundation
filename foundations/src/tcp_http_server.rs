use std::{
    io::{Read, Write},
    net::TcpListener,
};

pub fn run() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8080")?;

    let (mut stream, addr) = listener.accept()?;

    println!("Connected: {addr}");

    let mut buffer = [0_u8; 1024];
    let bytes_read = stream.read(&mut buffer)?;

    println!("{}", String::from_utf8_lossy(&buffer[..bytes_read]));

    let body = "hello";

    let response = format!(
        "HTTP/1.1 200 OK\r\n\
         Content-Length: {}\r\n\
         Connection: close\r\n\
         \r\n\
         {}",
        body.len(),
        body
    );

    stream.write_all(response.as_bytes())?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore]
    fn my_test_data() -> std::io::Result<()> {
        run()
    }
}
