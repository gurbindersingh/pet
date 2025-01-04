mod aes;
mod elgamal;
mod hybrid_enc;
mod keys;
mod message;
mod schnorr;
mod serializers;

use crate::keys::KeyPair;
use crate::message::Message;
use curve25519_dalek::ristretto::{CompressedRistretto};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let signing_key = KeyPair::sk_from_file("signing_key.txt")?;
    let encryption_key = KeyPair::pk_from_file("encryption_key.txt")?;

    let group_id = b"Group ID: 59".to_vec();
    let sender_dummy = CompressedRistretto::default();
    let recipient_dummy = CompressedRistretto::from_slice(&encryption_key.compress().to_bytes())?;

    let mut message = Message::new(0, group_id, sender_dummy, recipient_dummy, Default::default());

    message
        .encrypt(&encryption_key)
        .expect("Failed to encrypt the message");

    message.sign(&signing_key);

    message
        .to_file("signed_encrypted_message.json")
        .expect("Failed to write message to file");

    println!("Message successfully encrypted, signed, and saved to 'signed_encrypted_message.json'!");

    Ok(())
}

// Additional tests from the group
#[cfg(test)]
mod tests {
    use crate::hybrid_enc::HybridCiphertext;
    use crate::schnorr::SchnorrSignature;
    use super::*;

    // Additional test to ensure the message can be decrypted and verified like in the sample above
    #[test]
    fn test_with_sample() {
        // Generate keypair for encryption and signing
        let encryption_pair: KeyPair = HybridCiphertext::keygen();
        let signing_pair = SchnorrSignature::keygen();

        // Prepare message for encryption and signing
        let group_id = b"Group ID: 59".to_vec();
        let sender_dummy = CompressedRistretto::default();
        let recipient_dummy = CompressedRistretto::from_slice(&encryption_pair.public_key.compress().to_bytes()).expect("Failed to compress recipient public key");

        let mut message = Message::new(0, group_id, sender_dummy, recipient_dummy, Default::default());

        // Encrypt message
        message
            .encrypt(&encryption_pair.public_key)
            .expect("Failed to encrypt the message");

        assert_ne!(
            message.payload, b"Group ID: 59".to_vec(),
            "Decrypted payload should not match the original payload as it should be encrypted"
        );

        // Sign message
        message.sign(&signing_pair.private_key);

        // Decrypt message
        message
            .decrypt(&encryption_pair.private_key)
            .expect("Failed to decrypt the message");

        // Ensure the message version is back to 0 after decryption
        assert_eq!(message.version, 0, "Version should be 0 after decryption");

        // Ensure the decrypted message matches the original payload
        assert_eq!(
            message.payload, b"Group ID: 59".to_vec(),
            "Decrypted payload should match the original payload"
        );

        // Verify message signature
        let is_valid = message.verify();

        assert!(is_valid, "The signature should be valid for the original message");

        println!("Encrypted and signed message is valid and has decrypted value: {:?}", std::str::from_utf8(&message.payload).unwrap());
    }
}
