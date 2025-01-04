use base64::prelude::*;
use curve25519_dalek::constants::RISTRETTO_BASEPOINT_POINT;
use curve25519_dalek::ristretto::{CompressedRistretto, RistrettoPoint};
use curve25519_dalek::scalar::Scalar;
use rand::rngs::OsRng;
use std::fs::File;
use std::io::Write;
use std::io::{self, Read};

/// Struct to hold public and private key pair
#[derive(Debug)]
pub struct KeyPair {
    pub private_key: Scalar,
    pub public_key: RistrettoPoint,
}

impl KeyPair {
    /// Generate a Schnorr signature key pair
    pub fn generate() -> KeyPair {
        let private_key = Scalar::random(&mut OsRng);
        let public_key = RISTRETTO_BASEPOINT_POINT * private_key;

        KeyPair{
            private_key,
            public_key
        }
    }

    /// Write the private key and public key to a single file.
    /// The file will contain two base64-encoded lines:
    ///  1) private key (32 bytes)
    ///  2) public key (32 bytes, compressed Ristretto)
    pub fn write_sk_to_file(&self, path: &str) -> io::Result<()> {
        let mut file = File::create(path)?;

        // 1) Encode the private key to base64
        let sk_bytes = self.private_key.to_bytes();
        let sk_b64 = BASE64_STANDARD.encode(sk_bytes);

        // 2) Encode the public key (in compressed form) to base64
        let pk_comp = self.public_key.compress();
        let pk_bytes = pk_comp.to_bytes();
        let pk_b64 = BASE64_STANDARD.encode(pk_bytes);

        // Write them in separate lines
        writeln!(file, "{}", sk_b64)?;
        writeln!(file, "{}", pk_b64)?;

        Ok(())
    }

    /// Write only the public key to a file (single base64-encoded line).
    pub fn write_pk_to_file(&self, path: &str) -> io::Result<()> {
        let mut file = File::create(path)?;

        let pk_comp = self.public_key.compress();
        let pk_bytes = pk_comp.to_bytes();
        let pk_b64 = BASE64_STANDARD.encode(pk_bytes);

        writeln!(file, "{}", pk_b64)?;
        Ok(())
    }

    /// Read a KeyPair (private_key + public_key) from file.
    /// Expects two base64 lines in the file (first is SK, second is PK).
    pub fn from_file(path: &str) -> io::Result<KeyPair> {
        let mut file = File::open(path)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;

        let mut lines = contents.lines();

        // --- Private key ---
        let sk_b64 = lines
            .next()
            .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "Missing private key line"))?;
        let sk_bytes = BASE64_STANDARD
            .decode(sk_b64.trim())
            .map_err(|_| io::Error::new(io::ErrorKind::InvalidData, "Invalid base64 for SK"))?;

        let ct_scalar = Scalar::from_canonical_bytes(
            sk_bytes
                .try_into()
                .map_err(|_| io::Error::new(io::ErrorKind::InvalidData, "Invalid SK length"))?,
        );

        // Manually check if ct_scalar is valid
        let private_key = if ct_scalar.is_some().unwrap_u8() == 1 {
            ct_scalar.unwrap()
        } else {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "Invalid scalar bytes",
            ));
        };

        // --- Public key ---
        let pk_b64 = lines
            .next()
            .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "Missing public key line"))?;
        let pk_bytes = BASE64_STANDARD
            .decode(pk_b64.trim())
            .map_err(|_| io::Error::new(io::ErrorKind::InvalidData, "Invalid base64 for PK"))?;

        let compressed = CompressedRistretto(
            pk_bytes
                .try_into()
                .map_err(|_| io::Error::new(io::ErrorKind::InvalidData, "Invalid PK length"))?,
        );
        let public_key = compressed.decompress().ok_or_else(|| {
            io::Error::new(io::ErrorKind::InvalidData, "Could not decompress PK")
        })?;

        Ok(KeyPair {
            private_key,
            public_key,
        })
    }

    /// Read a public key from file (single base64-encoded line).
    pub fn pk_from_file(path: &str) -> io::Result<RistrettoPoint> {
        let mut file = File::open(path)?;
        let mut pk_b64 = String::new();
        file.read_to_string(&mut pk_b64)?;

        let pk_bytes = BASE64_STANDARD
            .decode(pk_b64.trim())
            .map_err(|_| io::Error::new(io::ErrorKind::InvalidData, "Invalid base64 for PK"))?;

        let compressed = CompressedRistretto(
            pk_bytes
                .try_into()
                .map_err(|_| io::Error::new(io::ErrorKind::InvalidData, "Invalid PK length"))?,
        );
        let public_key = compressed
            .decompress()
            .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "Could not decompress PK"))?;

        Ok(public_key)
    }

    /// Read a private key from file (single base64-encoded line).
    pub fn sk_from_file(path: &str) -> io::Result<Scalar> {
        let mut file = File::open(path)?;
        let mut sk_b64 = String::new();
        file.read_to_string(&mut sk_b64)?;

        let sk_bytes = BASE64_STANDARD
            .decode(sk_b64.trim())
            .map_err(|_| io::Error::new(io::ErrorKind::InvalidData, "Invalid base64 for SK"))?;

        let ct_scalar = Scalar::from_canonical_bytes(
            sk_bytes
                .try_into()
                .map_err(|_| io::Error::new(io::ErrorKind::InvalidData, "Invalid SK length"))?,
        );

        // Check if the scalar is valid
        if ct_scalar.is_some().unwrap_u8() == 1 {
            Ok(ct_scalar.unwrap())
        } else {
            Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "Invalid scalar bytes",
            ))
        }
    }
}

// Unit tests for keys module
#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_generate_keypair() {
        let keypair = KeyPair::generate();
        assert!(
            keypair.public_key != RistrettoPoint::default(),
            "Public key should not be default"
        );
        assert!(
            keypair.private_key != Scalar::default(),
            "Private key should not be default"
        );
        assert!(
            keypair.private_key * &RISTRETTO_BASEPOINT_POINT == keypair.public_key,
            "Public key should be g^private_key"
        )
    }

    #[test]
    fn test_write_and_read_keypair() {
        let keypair = KeyPair::generate();
        let pk_filepath = "pk_test.txt";
        let sk_filepath = "sk_test.txt";

        // Write the keypair to a file
        keypair
            .write_sk_to_file(&sk_filepath)
            .expect("Failed to write sk to file");
        keypair
            .write_pk_to_file(&pk_filepath)
            .expect("Failed to write pk to file");

        // Read the keypair back from the file
        let read_keypair =
            KeyPair::from_file(&sk_filepath).expect("Failed to read keypair from file");

        let read_pk = KeyPair::pk_from_file(&pk_filepath).expect("Failed to read pk from file");

        // Check if the written and read key pairs are equal
        assert_eq!(
            keypair.private_key, read_keypair.private_key,
            "Private keys should match"
        );
        assert_eq!(
            keypair.public_key, read_keypair.public_key,
            "Public keys should match"
        );
        assert_eq!(keypair.public_key, read_pk, "Public keys should match");

        // Clean up the test file
        fs::remove_file(&sk_filepath).expect("Failed to remove sk test file");
        fs::remove_file(&pk_filepath).expect("Failed to remove pk test file");
    }
}
