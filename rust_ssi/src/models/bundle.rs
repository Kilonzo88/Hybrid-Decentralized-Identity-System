use serde::{Deserialize, Serialize};
use crate::models::{Patient, Practitioner, Encounter, Observation, Condition, MedicationRequest};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Bundle {
    pub resource_type: String,
    pub id: String,
    #[serde(rename = "type")]
    pub bundle_type: String,
    pub timestamp: String,
    pub entry: Vec<BundleEntry>,
    pub signature: Option<Signature>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BundleEntry {
    pub resource: Resource,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Resource {
    Patient(Patient),
    Practitioner(Practitioner),
    Encounter(Encounter),
    Observation(Observation),
    Condition(Condition),
    MedicationRequest(MedicationRequest),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Signature {
    #[serde(rename = "type")]
    pub signature_type: Vec<SignatureType>,
    pub when: String,
    pub who: Reference,
    pub data: String,
    pub sig_format: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SignatureType {
    pub system: String,
    pub code: String,
    pub display: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Reference {
    pub reference: String,
    pub display: Option<String>,
}

impl Bundle {
    pub fn new(id: String, bundle_type: String, timestamp: String) -> Self {
        Self {
            resource_type: "Bundle".to_string(),
            id,
            bundle_type,
            timestamp,
            entry: Vec::new(),
            signature: None,
        }
    }

    pub fn add_entry(&mut self, resource: Resource) {
        self.entry.push(BundleEntry { resource });
    }

    pub fn add_signature(&mut self, signature: Signature) {
        self.signature = Some(signature);
    }
}
