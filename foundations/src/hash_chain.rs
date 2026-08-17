#[cfg(test)]
mod tests {
    use std::time::Instant;

    use sha2::{Digest, Sha256};

    #[test]
    fn sequential_hash_chain() {
        let start = Instant::now();
        let mut hash = Sha256::digest(b"hello");
        for _ in 1..1000 {
            hash = Sha256::digest(&hash);
        }

        let elapsed = start.elapsed();
        println!("{elapsed:?}")
    }
}
