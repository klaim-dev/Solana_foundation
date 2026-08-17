use solana_keypair::{Keypair, Signature};
use solana_signer::Signer;

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
    use super::*;

    #[test]
    fn test_verify_the_signature_happy_path() {
        let (pubkey_as_array, signature, message) = signature();
        let verify = signature.verify(&pubkey_as_array, &message);
        assert_eq!(verify, true);
    }

    #[test]
    fn test_verify_the_signature_negative() {
        let (pubkey_as_array, signature, _message) = signature();
        let new_message = b"hella";
        let verify = signature.verify(&pubkey_as_array, new_message);
        assert_eq!(verify, false);
    }
}
