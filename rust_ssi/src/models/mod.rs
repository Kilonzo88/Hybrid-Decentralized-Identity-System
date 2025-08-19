pub mod common;
pub mod patient;
pub mod practitioner;
pub mod observation;
pub mod medication;
pub mod encounter;
pub mod condition;
pub mod bundle;

// Re-export common types to avoid duplication
pub use common::{Identifier, HumanName, ContactPoint, CodeableConcept, Coding, Reference, Quantity, Period};
pub use patient::Patient;
pub use practitioner::Practitioner;
pub use observation::Observation;
pub use medication::MedicationRequest;
pub use encounter::Encounter;
pub use condition::Condition;
pub use bundle::{Bundle, BundleEntry, Resource, Signature, SignatureType};
