extern crate libc;
extern crate logger;
extern crate memory_model;
extern crate openssl;
extern crate serde;
#[macro_use]
extern crate serde_derive;

use std::io::{self, Read, Seek, Write};

use memory_model::{GuestAddress, GuestMemory, GuestMemoryError};
use openssl::error::ErrorStack;
use openssl::symm::decrypt as openssl_decrypt;
use openssl::symm::encrypt as openssl_encrypt;
use openssl::symm::Cipher;
use serde::de::{Deserialize, Deserializer, Error};
use serde::ser::{Serialize, Serializer};

const SECTOR_SIZE: usize = 512;
//static mut INITIAL_BUFFER: [u8; SECTOR_SIZE] = [0; SECTOR_SIZE];

#[derive(Debug)]
pub enum EncryptionError {
    /// Failure in accessing the block device
    IOError(io::Error),
    /// Failure in accessing the memory
    MemoryError(GuestMemoryError),
    /// Failure in encrypting/decrypting with the cipher
    OpensslError(ErrorStack),
}

///The algorithm used for encryption
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum EncryptionAlgorithm {
    /// XEX-based Tweaked codebook with ciphertext Stealing Mode for Advanced Encryption Standard
    /// Algorithm, with 512 bits key length
    AES256XTS,
}

/// Use this structure to set up the parameters used for encryption and decryption of data.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct EncryptionDescription {
    /// The key used for data encryption
    #[serde(deserialize_with = "deserialize")]
    #[serde(serialize_with = "serialize")]
    pub key: Vec<u8>,
    ///The algorithm used for data encryption
    pub algorithm: EncryptionAlgorithm,
}

pub fn parse_str<S>(s: &S) -> Result<Vec<u8>, &str>
where
    S: AsRef<str> + ?Sized,
{
    if s.as_ref().len() % 2 == 1 {
        return Err(s.as_ref());
    }
    let mut bytes = Vec::with_capacity(s.as_ref().len() / 2);
    for i in (0..s.as_ref().len()).step_by(2) {
        bytes.push(u8::from_str_radix(&s.as_ref()[i..i + 2], 16).map_err(|_| s.as_ref())?);
    }
    Ok(bytes)
}

pub fn to_string(byte_array: &Vec<u8>) -> String {
    let mut hex_string = String::new();
    for i in 0..byte_array.len() {
        hex_string += &format!("{:02x}", byte_array[i]);
    }
    hex_string
}

fn serialize<S>(hex_array: &Vec<u8>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    to_string(hex_array).serialize(serializer)
}

fn deserialize<'de, D>(deserializer: D) -> Result<Vec<u8>, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    parse_str(&s).map_err(|_| D::Error::custom("The provided string is invalid."))
}

pub struct EncryptionContext {
    encryption_description: EncryptionDescription,
    cipher: Cipher,
    initial_buffer: [u8; SECTOR_SIZE],
    final_buffer: Vec<u8>,
}

impl EncryptionContext {
    pub fn new(encryption_description: EncryptionDescription) -> Self {
        EncryptionContext {
            encryption_description,
            cipher: Cipher::aes_256_xts(),
            initial_buffer: [0u8; SECTOR_SIZE],
            final_buffer: Vec::new(),
        }
    }
    pub fn decrypt<T: Seek + Read + Write>(
        &mut self,
        disk: &mut T,
        mem: &GuestMemory,
        data_addr: GuestAddress,
        data_len: usize,
        no_sector: u64,
    ) -> Result<(), EncryptionError> {
        let num_sectors = (data_len / SECTOR_SIZE) as u64;
        let addr = &mut GuestAddress(data_addr.offset());
        for sector_offset in 0..num_sectors {
            let iv: [u8; 16] = ((no_sector + sector_offset) as u128).to_le_bytes();

            // Read_exact will fill the buffer or return an error, so we don't have to worry
            // about dealing with partial reads.
            disk.read_exact(&mut self.initial_buffer)
                .map_err(EncryptionError::IOError)?;

            self.final_buffer = openssl_decrypt(
                self.cipher,
                &self.encryption_description.key,
                Some(&iv),
                &mut self.initial_buffer,
            )
            .map_err(EncryptionError::OpensslError)?;

            mem.write_slice_at_addr(&self.final_buffer, *addr)
                .map_err(EncryptionError::MemoryError)?;

            // We should check for error here at some point instead of unwrapping.
            *addr = addr.checked_add(SECTOR_SIZE).unwrap();
        }

        Ok(())
    }

    pub fn encrypt<T: Seek + Read + Write>(
        &mut self,
        disk: &mut T,
        mem: &GuestMemory,
        data_addr: GuestAddress,
        data_len: usize,
        no_sector: u64,
    ) -> Result<(), EncryptionError> {
        let num_sectors = (data_len / SECTOR_SIZE) as u64;
        let addr = &mut GuestAddress(data_addr.offset());
        for sector_offset in 0..num_sectors {
            let iv: [u8; 16] = ((no_sector + sector_offset) as u128).to_le_bytes();

            mem.read_slice_at_addr(&mut self.initial_buffer, *addr)
                .map_err(EncryptionError::MemoryError)?;

            self.final_buffer = openssl_encrypt(
                self.cipher,
                &self.encryption_description.key,
                Some(&iv),
                &mut self.initial_buffer,
            )
            .map_err(|e| EncryptionError::OpensslError(e))?;

            // Write_all attempts to write everything or returns an error, so we don't have to
            // worry about dealing with partial writes.
            disk.write_all(&mut self.final_buffer)
                .map_err(EncryptionError::IOError)?;

            // We should check for error here at some point instead of unwrapping.
            *addr = addr.checked_add(SECTOR_SIZE).unwrap();
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {

    extern crate serde_json;

    use super::*;
    use openssl::aes::{aes_ige, AesKey};
    use openssl::symm::Mode;

    #[test]
    fn test_encryption_parameter_str() {
        // odd length string
        assert!(parse_str("01234567890123456").is_err());

        // invalid hex
        assert!(parse_str("x123456789012345").is_err());

        let bytes = parse_str("0123456789012345").unwrap();
        assert_eq!(bytes, [0x01, 0x23, 0x45, 0x67, 0x89, 0x01, 0x23, 0x45]);
    }

    #[test]
    fn test_encr_desc_serialization_and_deserialization() {
        let json = r#"{
            "key":"0123456789012345678901234567890A",
            "algorithm":"AES256XTS"
        }"#;

        let encr_desc: EncryptionDescription =
            serde_json::from_str(json).expect("Deserialization failed");

        assert_eq!(
            encr_desc.key,
            [
                0x01, 0x23, 0x45, 0x67, 0x89, 0x01, 0x23, 0x45, 0x67, 0x89, 0x01, 0x23, 0x45, 0x67,
                0x89, 0x0A
            ]
        );

        let s = serde_json::to_string(&encr_desc).expect("Serialization failed.");

        let json = r#"{"key":"0123456789012345678901234567890a","algorithm":"AES256XTS"}"#;
        assert_eq!(s, json);
    }
}
