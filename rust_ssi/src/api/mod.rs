// FHIR Models Module
// Re-exports all FHIR resource structs

pub mod bundle;
pub mod patient;
pub mod practitioner;
pub mod encounter;
pub mod observation;
pub mod condition;
pub mod medication;

// Re-export main structs for easy access
pub use bundle::Bundle;
pub use patient::Patient;
pub use practitioner::Practitioner;
pub use encounter::Encounter;
pub use observation::Observation;
pub use condition::Condition;
pub use medication::MedicationRequest;
