use aes::cipher::crypto_common;
use byteorder::{LittleEndian, WriteBytesExt};

use crate::{messages, util::cryptoutil};
use anyhow::Result;
use std::io::Write;

pub struct Session {
    pub session_id: u16,
    pub counter: u32,
    pub local_node: Option<Vec<u8>>,
    pub remote_node: Option<Vec<u8>>,
    pub encrypt_key: Option<crypto_common::Key<Aes128Ccm>>,
    pub decrypt_key: Option<crypto_common::Key<Aes128Ccm>>,
}
type Aes128Ccm = ccm::Ccm<aes::Aes128, ccm::consts::U16, ccm::consts::U13>;
impl Session {
    pub fn new() -> Self {
        Self {
            session_id: 0,
            counter: rand::random(),
            local_node: Some([0, 0, 0, 0, 0, 0, 0, 0].to_vec()),
            remote_node: None,
            encrypt_key: None,
            decrypt_key: None,
        }
    }
    pub fn set_encrypt_key(&mut self, k: &[u8]) {
        self.encrypt_key = Some(*crypto_common::Key::<Aes128Ccm>::from_slice(k))
    }
    pub fn set_decrypt_key(&mut self, k: &[u8]) {
        self.decrypt_key = Some(*crypto_common::Key::<Aes128Ccm>::from_slice(k))
    }

    pub fn encode_message(&mut self, data: &[u8]) -> Result<Vec<u8>> {
        let mg = messages::MessageHeader {
            flags: 0,
            security_flags: 0,
            session_id: self.session_id,
            message_counter: self.counter,
            source_node_id: self.local_node.clone(),
            destination_node_id: self.remote_node.clone(),
        };
        let mut b = mg.encode()?;
        match self.encrypt_key {
            Some(key) => {
                let nonce = self.make_nonce3()?;
                let enc = cryptoutil::aes128_ccm_encrypt(&key, &nonce, &b, data)?;
                b.extend_from_slice(&enc);
            }
            None => b.extend_from_slice(data),
        };

        self.counter += 1;
        Ok(b)
    }

    pub fn decode_message(&mut self, data: &[u8]) -> Result<Vec<u8>> {
        if self.decrypt_key.is_none() {
            return Ok(data.to_vec());
        }
        let (header, rest) = messages::MessageHeader::decode(data)?;
        let nonce = Self::make_nonce3_extern(header.message_counter, self.remote_node.as_deref())?;
        let add = &data[..data.len() - rest.len()];
        let decoded = cryptoutil::aes128_ccm_decrypt(
            &self.decrypt_key.unwrap_or_default(),
            &nonce,
            add,
            &rest,
        )?;
        let mut out = Vec::new();
        out.extend_from_slice(add);
        out.extend_from_slice(&decoded);
        Ok(out)
    }

    fn make_nonce3(&self) -> Result<Vec<u8>> {
        Self::make_nonce3_extern(self.counter, self.local_node.as_deref())
    }

    fn make_nonce3_extern(counter: u32, node: Option<&[u8]>) -> Result<Vec<u8>> {
        let mut out = Vec::with_capacity(128);
        out.write_u8(0)?;
        out.write_u32::<LittleEndian>(counter)?;
        match node {
            Some(s) => out.write_all(s)?,
            None => out.write_all(&[0, 0, 0, 0, 0, 0, 0, 0])?,
        };

        Ok(out)
    }
}

impl Default for Session {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_session_creation() {
        let session = Session::new();
        assert_eq!(session.session_id, 0);
        assert!(session.local_node.is_some());
        assert!(session.remote_node.is_none());
        assert!(session.encrypt_key.is_none());
        assert!(session.decrypt_key.is_none());
    }

    #[test]
    fn test_session_default() {
        let session = Session::default();
        assert_eq!(session.session_id, 0);
        assert!(session.local_node.is_some());
    }

    #[test]
    fn test_set_keys() {
        let mut session = Session::new();
        let key = [0x01u8; 16];
        
        session.set_encrypt_key(&key);
        assert!(session.encrypt_key.is_some());
        
        session.set_decrypt_key(&key);
        assert!(session.decrypt_key.is_some());
    }

    #[test]
    fn test_make_nonce3() {
        let session = Session::new();
        let nonce = session.make_nonce3();
        assert!(nonce.is_ok());
        
        let nonce_data = nonce.unwrap();
        assert_eq!(nonce_data.len(), 13); // 1 + 4 + 8 bytes
        assert_eq!(nonce_data[0], 0); // First byte should be 0
    }

    #[test]
    fn test_make_nonce3_extern() {
        let counter = 0x12345678u32;
        let node = vec![0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08];
        
        let nonce = Session::make_nonce3_extern(counter, Some(&node));
        assert!(nonce.is_ok());
        
        let nonce_data = nonce.unwrap();
        assert_eq!(nonce_data.len(), 13);
        assert_eq!(nonce_data[0], 0);
        
        // Check counter (little-endian)
        let counter_bytes = &nonce_data[1..5];
        let decoded_counter = u32::from_le_bytes(counter_bytes.try_into().unwrap());
        assert_eq!(decoded_counter, counter);
        
        // Check node
        assert_eq!(&nonce_data[5..], &node[..]);
    }

    #[test]
    fn test_encode_message_unencrypted() {
        let mut session = Session::new();
        session.session_id = 123;
        
        let data = b"test message";
        let encoded = session.encode_message(data);
        assert!(encoded.is_ok());
        
        let encoded_data = encoded.unwrap();
        assert!(encoded_data.len() > data.len());
        assert!(encoded_data.ends_with(data));
    }

    #[test]
    fn test_counter_increment() {
        let mut session = Session::new();
        let initial_counter = session.counter;
        
        let _ = session.encode_message(b"test");
        assert_eq!(session.counter, initial_counter + 1);
        
        let _ = session.encode_message(b"test2");
        assert_eq!(session.counter, initial_counter + 2);
    }

    #[test]
    fn test_encode_decode_with_encryption() {
        let mut session1 = Session::new();
        let mut session2 = Session::new();
        
        // Set up symmetric keys (in real use, these would be derived from key exchange)
        let encrypt_key = [0x01u8; 16];
        let decrypt_key = [0x01u8; 16];
        
        session1.set_encrypt_key(&encrypt_key);
        session2.set_decrypt_key(&decrypt_key);
        
        // Set matching node IDs
        let node1 = vec![0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88];
        let node2 = vec![0x88, 0x77, 0x66, 0x55, 0x44, 0x33, 0x22, 0x11];
        session1.local_node = Some(node1.clone());
        session1.remote_node = Some(node2.clone());
        session2.local_node = Some(node2);
        session2.remote_node = Some(node1);
        
        // Sync counters for this test
        session2.counter = session1.counter;
        
        let original_message = b"secret data";
        let encoded = session1.encode_message(original_message).unwrap();
        
        // The encoded message should be different from original (encrypted)
        assert!(!encoded.ends_with(original_message));
        
        let decoded = session2.decode_message(&encoded).unwrap();
        
        // Extract the payload from decoded message
        let (_, payload) = messages::MessageHeader::decode(&decoded).unwrap();
        assert_eq!(payload, original_message);
    }
}
