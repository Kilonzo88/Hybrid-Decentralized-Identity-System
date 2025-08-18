use crate::models::*;
use serde_json::Value;
use std::collections::HashMap;

// TODO: Add MongoDB driver dependencies in Cargo.toml
// use mongodb::{Client, Collection, Database};
// use bson::{doc, Document};

pub struct Storage {
    // TODO: Add MongoDB client
    // client: Client,
    // database: Database,
    // For MVP, we'll use in-memory storage
    bundles: HashMap<String, Bundle>,
}

impl Storage {
    pub fn new() -> Self {
        Self {
            bundles: HashMap::new(),
        }
    }

    /// Store a FHIR Bundle in the database
    pub async fn store_bundle(&mut self, bundle: Bundle) -> Result<String, Box<dyn std::error::Error>> {
        let bundle_id = bundle.id.clone();
        self.bundles.insert(bundle_id.clone(), bundle);
        
        println!("💾 Stored FHIR Bundle: {}", bundle_id);
        Ok(bundle_id)
    }

    /// Retrieve a FHIR Bundle by ID
    pub async fn get_bundle(&self, bundle_id: &str) -> Result<Option<&Bundle>, Box<dyn std::error::Error>> {
        Ok(self.bundles.get(bundle_id))
    }

    /// List all stored bundles
    pub async fn list_bundles(&self) -> Result<Vec<String>, Box<dyn std::error::Error>> {
        Ok(self.bundles.keys().cloned().collect())
    }

    /// Delete a FHIR Bundle
    pub async fn delete_bundle(&mut self, bundle_id: &str) -> Result<bool, Box<dyn std::error::Error>> {
        let existed = self.bundles.remove(bundle_id).is_some();
        if existed {
            println!("🗑️ Deleted FHIR Bundle: {}", bundle_id);
        }
        Ok(existed)
    }

    /// Search bundles by patient ID
    pub async fn search_by_patient(&self, patient_id: &str) -> Result<Vec<&Bundle>, Box<dyn std::error::Error>> {
        let mut results = Vec::new();
        
        for bundle in self.bundles.values() {
            for entry in &bundle.entry {
                if let Resource::Patient(patient) = &entry.resource {
                    if patient.id == patient_id {
                        results.push(bundle);
                        break;
                    }
                }
            }
        }
        
        Ok(results)
    }

    /// Search bundles by practitioner ID
    pub async fn search_by_practitioner(&self, practitioner_id: &str) -> Result<Vec<&Bundle>, Box<dyn std::error::Error>> {
        let mut results = Vec::new();
        
        for bundle in self.bundles.values() {
            for entry in &bundle.entry {
                if let Resource::Practitioner(practitioner) = &entry.resource {
                    if practitioner.id == practitioner_id {
                        results.push(bundle);
                        break;
                    }
                }
            }
        }
        
        Ok(results)
    }

    /// Get bundle statistics
    pub async fn get_statistics(&self) -> Result<BundleStats, Box<dyn std::error::Error>> {
        let total_bundles = self.bundles.len();
        let mut patient_count = 0;
        let mut practitioner_count = 0;
        let mut encounter_count = 0;
        let mut observation_count = 0;
        let mut condition_count = 0;
        let mut medication_count = 0;
        
        for bundle in self.bundles.values() {
            for entry in &bundle.entry {
                match &entry.resource {
                    Resource::Patient(_) => patient_count += 1,
                    Resource::Practitioner(_) => practitioner_count += 1,
                    Resource::Encounter(_) => encounter_count += 1,
                    Resource::Observation(_) => observation_count += 1,
                    Resource::Condition(_) => condition_count += 1,
                    Resource::MedicationRequest(_) => medication_count += 1,
                }
            }
        }
        
        Ok(BundleStats {
            total_bundles,
            patient_count,
            practitioner_count,
            encounter_count,
            observation_count,
            condition_count,
            medication_count,
        })
    }
}

#[derive(Debug)]
pub struct BundleStats {
    pub total_bundles: usize,
    pub patient_count: usize,
    pub practitioner_count: usize,
    pub encounter_count: usize,
    pub observation_count: usize,
    pub condition_count: usize,
    pub medication_count: usize,
}

impl std::fmt::Display for BundleStats {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Bundle Statistics:\n")?;
        write!(f, "  Total Bundles: {}\n", self.total_bundles)?;
        write!(f, "  Patients: {}\n", self.patient_count)?;
        write!(f, "  Practitioners: {}\n", self.practitioner_count)?;
        write!(f, "  Encounters: {}\n", self.encounter_count)?;
        write!(f, "  Observations: {}\n", self.observation_count)?;
        write!(f, "  Conditions: {}\n", self.condition_count)?;
        write!(f, "  Medications: {}\n", self.medication_count)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::*;
    
    #[tokio::test]
    async fn test_storage_operations() {
        let mut storage = Storage::new();
        
        // Create a test bundle
        let bundle = Bundle::new(
            "test-bundle-1".to_string(),
            "document".to_string(),
            "2024-07-30T10:30:00Z".to_string(),
        );
        
        // Test store and retrieve
        let bundle_id = storage.store_bundle(bundle).await.unwrap();
        assert_eq!(bundle_id, "test-bundle-1");
        
        let retrieved = storage.get_bundle(&bundle_id).await.unwrap();
        assert!(retrieved.is_some());
        
        // Test list bundles
        let bundles = storage.list_bundles().await.unwrap();
        assert_eq!(bundles.len(), 1);
        assert_eq!(bundles[0], "test-bundle-1");
        
        // Test delete
        let deleted = storage.delete_bundle(&bundle_id).await.unwrap();
        assert!(deleted);
        
        let retrieved_after_delete = storage.get_bundle(&bundle_id).await.unwrap();
        assert!(retrieved_after_delete.is_none());
    }
}
