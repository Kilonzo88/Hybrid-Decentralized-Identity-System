use serde::{Deserialize, Serialize};
use super::common::{CodeableConcept, Coding, Reference};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Condition {
    pub resource_type: String,
    pub id: String,
    pub clinical_status: Option<CodeableConcept>,
    pub verification_status: Option<CodeableConcept>,
    pub code: CodeableConcept,
    pub subject: Reference,
    pub encounter: Option<Reference>,
    pub recorded_date: String,
    pub asserter: Option<Reference>,
}

impl Condition {
    pub fn new(id: String, code: CodeableConcept, subject: Reference, recorded_date: String) -> Self {
        Self {
            resource_type: "Condition".to_string(),
            id,
            clinical_status: None,
            verification_status: None,
            code,
            subject,
            encounter: None,
            recorded_date,
            asserter: None,
        }
    }

    pub fn set_clinical_status(&mut self, status: CodeableConcept) {
        self.clinical_status = Some(status);
    }

    pub fn set_verification_status(&mut self, status: CodeableConcept) {
        self.verification_status = Some(status);
    }

    pub fn set_encounter(&mut self, encounter: Reference) {
        self.encounter = Some(encounter);
    }

    pub fn set_asserter(&mut self, asserter: Reference) {
        self.asserter = Some(asserter);
    }
}
