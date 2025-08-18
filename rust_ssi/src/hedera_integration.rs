use hedera_sdk_rust::{
    Client, FileId, FileCreateTransaction, FileAppendTransaction, FileContentsQuery,
    PrivateKey, AccountId, TransactionResponse, Status
};
use aes_gcm::{Aes256Gcm, Key, Nonce};
use aes_gcm::aead::{Aead, NewAead};
use rand::Rng;
use serde::{Serialize, Deserialize};
use std::error::Error;

/// Represents an encrypted EHR file stored on Hedera File Service
#[derive(Debug, Serialize, Deserialize)]
pub struct EncryptedEHR {
    pub file_id: String,
    pub encryption_key_hash: String,
    pub mime_type: String,
    pub size: u64,
    pub created_timestamp: i64,
}

/// Hedera File Service integration for storing encrypted patient EHR data
pub struct HederaFileService {
    client: Client,
    operator_account: AccountId,
    operator_key: PrivateKey,
}

impl HederaFileService {
    /// Create a new Hedera File Service instance
    pub fn new(
        network: &str,
        operator_account: &str,
        operator_private_key: &str,
    ) -> Result<Self, Box<dyn Error>> {
        let client = Client::for_name(network)?;
        let operator_account = operator_account.parse()?;
        let operator_key = PrivateKey::from_string(operator_private_key)?;
        
        Ok(Self {
            client,
            operator_account,
            operator_key,
        })
    }

    /// Store encrypted EHR data on Hedera File Service
    pub async fn store_ehr(
        &self,
        encrypted_data: &[u8],
        mime_type: &str,
    ) -> Result<EncryptedEHR, Box<dyn Error>> {
        // Create a new file
        let file_create_tx = FileCreateTransaction::new()
            .keys([&self.operator_key.public_key()])
            .contents(encrypted_data)
            .max_automatic_token_associations(0);

        let file_create_response = file_create_tx
            .sign(&self.operator_key)
            .execute(&self.client)
            .await?;

        let file_id = file_create_response.get_receipt(&self.client).await?
            .file_id
            .ok_or("Failed to get file ID")?;

        // Get file info
        let file_contents_query = FileContentsQuery::new()
            .file_id(file_id);

        let file_contents = file_contents_query.execute(&self.client).await?;

        Ok(EncryptedEHR {
            file_id: file_id.to_string(),
            encryption_key_hash: "".to_string(), // In production, store encrypted key hash
            mime_type: mime_type.to_string(),
            size: file_contents.contents.len() as u64,
            created_timestamp: chrono::Utc::now().timestamp(),
        })
    }

    /// Retrieve encrypted EHR data from Hedera File Service
    pub async fn retrieve_ehr(&self, file_id: &str) -> Result<Vec<u8>, Box<dyn Error>> {
        let file_id: FileId = file_id.parse()?;
        
        let file_contents_query = FileContentsQuery::new()
            .file_id(file_id);

        let file_contents = file_contents_query.execute(&self.client).await?;
        
        Ok(file_contents.contents)
    }
}

/// Encryption utilities for patient EHR data
pub struct EHREncryption {
    key: Key<Aes256Gcm>,
}

impl EHREncryption {
    /// Create new encryption instance with a random key
    pub fn new() -> Result<Self, Box<dyn Error>> {
        let mut rng = rand::thread_rng();
        let key_bytes: [u8; 32] = rng.gen();
        let key = Key::from_slice(&key_bytes);
        
        Ok(Self { key: *key })
    }

    /// Encrypt EHR data
    pub fn encrypt(&self, data: &[u8]) -> Result<(Vec<u8>, Vec<u8>), Box<dyn Error>> {
        let cipher = Aes256Gcm::new(&self.key);
        let mut rng = rand::thread_rng();
        let nonce_bytes: [u8; 12] = rng.gen();
        let nonce = Nonce::from_slice(&nonce_bytes);
        
        let encrypted_data = cipher.encrypt(nonce, data)?;
        
        Ok((encrypted_data, nonce_bytes.to_vec()))
    }

    /// Decrypt EHR data
    pub fn decrypt(&self, encrypted_data: &[u8], nonce: &[u8]) -> Result<Vec<u8>, Box<dyn Error>> {
        let cipher = Aes256Gcm::new(&self.key);
        let nonce = Nonce::from_slice(nonce);
        
        let decrypted_data = cipher.decrypt(nonce, encrypted_data)?;
        
        Ok(decrypted_data)
    }

    /// Get the encryption key for storage/transmission
    pub fn get_key(&self) -> Vec<u8> {
        self.key.as_slice().to_vec()
    }
}

/// Patient EHR data structure
#[derive(Debug, Serialize, Deserialize)]
pub struct PatientEHR {
    pub patient_did: String,
    pub provider_did: String,
    pub ehr_type: String,
    pub data: serde_json::Value,
    pub timestamp: i64,
    pub valid_until: Option<i64>,
}

impl PatientEHR {
    /// Create new EHR record
    pub fn new(
        patient_did: String,
        provider_did: String,
        ehr_type: String,
        data: serde_json::Value,
        valid_until: Option<i64>,
    ) -> Self {
        Self {
            patient_did,
            provider_did,
            ehr_type,
            data,
            timestamp: chrono::Utc::now().timestamp(),
            valid_until,
        }
    }

    /// Convert to JSON bytes for encryption
    pub fn to_bytes(&self) -> Result<Vec<u8>, Box<dyn Error>> {
        Ok(serde_json::to_vec(self)?)
    }

    /// Create from JSON bytes
    pub fn from_bytes(bytes: &[u8]) -> Result<Self, Box<dyn Error>> {
        Ok(serde_json::from_slice(bytes)?)
    }
}
