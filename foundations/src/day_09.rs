use sha2::{Digest, Sha256};




fn test_hash() {


let hash = Sha256::digest(b"global:initialize");
let discriminator = &hash[0..8];
let anchor_discrimenator: [u8; 8] = [175, 175, 109, 31, 13, 152, 155, 237];

assert_eq!(discriminator, anchor_discrimenator);
}


#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn test() {
        test_hash();
    }
}