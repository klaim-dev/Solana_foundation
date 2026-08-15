use std::{
    io::{Read, Write},
    net::TcpListener,
};

use solana_keypair::{Keypair, Signature};
use solana_signer::Signer;

pub fn run() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8080")?;

    let (mut stream, addr) = listener.accept()?;

    println!("Connected: {addr}");

    let mut buffer = [0_u8; 1024];
    let bytes_read = stream.read(&mut buffer)?;

    println!(
        "{}",
        String::from_utf8_lossy(&buffer[..bytes_read])
    );

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


pub fn signature() -> ([u8; 32], Signature, [u8; 5]) {
    let keypair = Keypair::new();
    let message: [u8; 5] = *b"hello";
    let signature = keypair.sign_message(&message);
    let key = keypair.pubkey();
    let pubkey_as_array: [u8; 32] = *key.as_array();
    (pubkey_as_array, signature, message)
}

#[cfg(test)]
mod tests {
    use std::time::Instant;

use sha2::{Digest, Sha256};

use super::*;

    #[test]
    #[ignore]
    fn my_test_data() -> std::io::Result<()> {
        run()
    }

    #[test]
    fn test_verify_the_signature_happy_path() {
    let (pubkey_as_array, signature, message) = signature();
    let verify = signature.verify(&pubkey_as_array, &message);
    assert_eq!(verify, true);

    }

       #[test]
    fn test_verify_the_signature_negative() {
        let (pubkey_as_array, signature, message) = signature();
        let new_message = b"hella";
        let verify = signature.verify(&pubkey_as_array, new_message);
        assert_eq!(verify, false);
        
    }

    #[test]
    fn sequential_hash_chain() {
        let start = Instant::now();
        let mut h = Sha256::digest(b"hello");
        for _ in 1..1000 {
            h = Sha256::digest(&h);
        }

        let elapsed = start.elapsed();
        println!("{elapsed:?}")
    }
}
