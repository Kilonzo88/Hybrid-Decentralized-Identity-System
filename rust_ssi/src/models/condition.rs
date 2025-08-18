use serde::{Deserialize, Serialize};

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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CodeableConcept {
    pub coding: Vec<Coding>,
    pub text: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Coding {
    pub system: String,
    pub code: String,
    pub display: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Reference {
    pub reference: String,
    pub display: Option<String>,
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
