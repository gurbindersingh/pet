use crate::hybrid_enc::HybridCiphertext;
use crate::keys::KeyPair;
use crate::schnorr::SchnorrSignature;
use crate::serializers::*;
use aead::OsRng;
use curve25519_dalek::constants::RISTRETTO_BASEPOINT_POINT;
use curve25519_dalek::ristretto::{CompressedRistretto, RistrettoPoint};
use curve25519_dalek::scalar::Scalar;
use serde::{Deserialize, Serialize};
use serde_json;
use std::fs::File;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    pub version: u8, // The version number of the message (1 byte)

    #[serde(
        serialize_with = "serialize_base64",
        deserialize_with = "deserialize_base64"
    )]
    pub payload: Vec<u8>, // The message content (or payload) stored as a Base64-encoded string in JSON.
    #[serde(
        serialize_with = "serialize_fixed_base64",
        deserialize_with = "deserialize_fixed_base64"
    )]
    pub recipient: [u8; 32], // The recipient's identifier (stored as Vec<u8> to serialize easily)
    #[serde(
        serialize_with = "serialize_fixed_base64",
        deserialize_with = "deserialize_fixed_base64"
    )]
    pub sender: [u8; 32], // The recipient's identifier (stored as Vec<u8> to serialize easily)
    #[serde(
        serialize_with = "serialize_schnorr_signature",
        deserialize_with = "deserialize_schnorr_signature"
    )]
    pub signature: SchnorrSignature,
}

impl Message {
    /// Creates a new message with a version, payload, and recipient (CompressedRistretto converted to Vec<u8>)
    pub fn new(
        version: u8,
        payload: Vec<u8>,
        sender: CompressedRistretto,
        recipient: CompressedRistretto,
        signature: SchnorrSignature,
    ) -> Self {
        Self {
            version: version,
            payload: payload,
            recipient: recipient.to_bytes(),
            sender: sender.to_bytes(),
            signature: signature,
        }
    }

    /// Writes the message to a JSON file
    pub fn to_file(&self, filepath: &str) -> std::io::Result<()> {
        let file = File::create(filepath)?;
        serde_json::to_writer_pretty(file, &self)?; // Write JSON in a human-readable format
        Ok(())
    }

    /// Encrypts the whole message using hybrid encryption
    pub fn encrypt(&mut self, elgamal_public_key: &RistrettoPoint) -> Result<(), String> {
        match serialize_message_to_bytes(self) {
            Ok(serialized_message) => {
                match HybridCiphertext::encrypt(&serialized_message, elgamal_public_key) {
                    Ok(ciphertext) => {
                        self.payload = ciphertext.serialize();
                        self.version += 1;
                        // sender should be set to default sender, whatever that means
                        self.sender = RistrettoPoint::random(&mut OsRng).compress().to_bytes();
                        self.recipient = elgamal_public_key.compress().to_bytes();
                        Ok(())
                    }
                    Err(err) => Err(err),
                }
            }
            Err(err) => Err(err),
        }
    }

    /// Decrypts the payload using hybrid decryption, sets version back to 0
    pub fn decrypt(&mut self, elgamal_private_key: &Scalar) -> Result<(), String> {
        match HybridCiphertext::deserialize(&self.payload) {
            Err(err) => Err(err),
            Ok(deserialized_ciphertext) => {
                match deserialized_ciphertext.decrypt(elgamal_private_key) {
                    Err(err) => Err(err),
                    Ok(plaintext_bytes) => match deserialize_message_from_bytes(&plaintext_bytes) {
                        Err(err) => Err(err),
                        Ok(deserialized_message) => {
                            self.version = deserialized_message.version;
                            self.payload = deserialized_message.payload;
                            self.sender = deserialized_message.sender;
                            self.recipient = deserialized_message.recipient;
                            self.signature = deserialized_message.signature;
                            Ok(())
                        }
                    },
                }
            }
        }
    }

    /// signs the payload using Schnorr signatures, sets the signing public key as sender
    pub fn sign(&mut self, signing_key: &Scalar) {
        let vk = RISTRETTO_BASEPOINT_POINT * signing_key;
        self.signature = SchnorrSignature::sign(&self.payload, signing_key);
        self.sender = vk.compress().to_bytes();
    }

    pub fn verify(&self) -> bool {
        let vk = self.sender;
        match CompressedRistretto::from_slice(&vk) {
            Err(_) => false,
            Ok(compressed_pk) => SchnorrSignature::verify(
                &self.signature,
                &self.payload,
                &compressed_pk.decompress().expect("Failed to deserialize public key"),
            ),
        }
    }

    pub fn display(&self) {
        println!("Message version: {}", self.version);

        match std::str::from_utf8(&self.payload) {
            Ok(text) => println!("Payload (UTF-8): {}", text),
            Err(_) => println!("Payload (raw bytes): {:?}", self.payload),
        }

        // Print the sender and recipient as base64 or hex
        let sender_b64 = base64::encode(self.sender);
        let recipient_b64 = base64::encode(self.recipient);
        println!("Sender (base64): {}", sender_b64);
        println!("Recipient (base64): {}", recipient_b64);

        // Print the signature (Debug-derive or as raw bytes)
        println!("Signature: {:?}", self.signature);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use curve25519_dalek::ristretto::RistrettoPoint;
    use rand::rngs::OsRng;

    #[test]
    fn test_message_creation() {
        // Create a sample payload and recipient
        let payload = b"Hello, this is a message!".to_vec();
        let mut csprng = OsRng;
        let recipient = RistrettoPoint::random(&mut csprng).compress();

        // Create a new message
        let version: u8 = 1;
        let message = Message::new(
            version,
            payload.clone(),
            recipient,
            recipient,
            SchnorrSignature::empty_signature(),
        );

        // Check if the fields match
        assert_eq!(message.version, version);
        assert_eq!(message.payload, payload);
        assert_eq!(message.recipient, recipient.to_bytes());

        // Display the message
        message.display();
    }

    #[test]
    fn test_message_encryption_and_decryption() {
        // Sample message to encrypt
        let payload = b"Hello, hybrid encryption!".to_vec();

        // Generate ElGamal keypair
        let keypair = KeyPair::generate();

        // Create a new message with version 0
        let mut message = Message::new(
            0,
            payload.clone(),
            keypair.public_key.compress(),
            keypair.public_key.compress(),
            SchnorrSignature::empty_signature(),
        );

        // Encrypt the message
        message
            .encrypt(&keypair.public_key)
            .expect("Encryption failed");

        // Ensure the message version is 1 after encryption
        assert_eq!(message.version, 1, "Version should be 1 after encryption");

        // Ensure the payload is not the same as the original (it should be encrypted)
        assert_ne!(
            message.payload, payload,
            "Encrypted payload should not match the original payload"
        );

        // Decrypt the message
        message
            .decrypt(&keypair.private_key)
            .expect("Decryption failed");

        // Ensure the message version is back to 0 after decryption
        assert_eq!(message.version, 0, "Version should be 0 after decryption");

        // Ensure the decrypted message matches the original payload
        assert_eq!(
            message.payload, payload,
            "Decrypted payload should match the original payload"
        );
    }
}
