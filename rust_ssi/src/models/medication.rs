use serde::{Deserialize, Serialize};
use super::common::{CodeableConcept, Coding, Reference, Quantity, Period};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MedicationRequest {
    pub resource_type: String,
    pub id: String,
    pub status: String,
    pub intent: String,
    pub medication_codeable_concept: CodeableConcept,
    pub subject: Reference,
    pub encounter: Option<Reference>,
    pub authored_on: String,
    pub requester: Option<Reference>,
    pub dosage_instruction: Vec<DosageInstruction>,
    pub dispense_request: Option<DispenseRequest>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DosageInstruction {
    pub text: String,
    pub timing: Option<Timing>,
    pub route: Option<CodeableConcept>,
    pub dose_and_rate: Option<Vec<DoseAndRate>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Timing {
    pub repeat: Option<TimingRepeat>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimingRepeat {
    pub frequency: Option<u32>,
    pub period: Option<u32>,
    pub period_unit: Option<String>,
    pub duration: Option<u32>,
    pub duration_unit: Option<String>,
    pub bounds_period: Option<Period>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DoseAndRate {
    #[serde(rename = "type")]
    pub dose_type: Option<CodeableConcept>,
    pub dose_quantity: Option<Quantity>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DispenseRequest {
    pub quantity: Quantity,
    pub number_of_repeats_allowed: Option<u32>,
}

impl MedicationRequest {
    pub fn new(
        id: String,
        status: String,
        intent: String,
        medication: CodeableConcept,
        subject: Reference,
        authored_on: String,
    ) -> Self {
        Self {
            resource_type: "MedicationRequest".to_string(),
            id,
            status,
            intent,
            medication_codeable_concept: medication,
            subject,
            encounter: None,
            authored_on,
            requester: None,
            dosage_instruction: Vec::new(),
            dispense_request: None,
        }
    }

    pub fn set_encounter(&mut self, encounter: Reference) {
        self.encounter = Some(encounter);
    }

    pub fn set_requester(&mut self, requester: Reference) {
        self.requester = Some(requester);
    }

    pub fn add_dosage_instruction(&mut self, text: String) {
        self.dosage_instruction.push(DosageInstruction {
            text,
            timing: None,
            route: None,
            dose_and_rate: None,
        });
    }

    pub fn set_dispense_request(&mut self, quantity: Quantity, number_of_repeats: Option<u32>) {
        self.dispense_request = Some(DispenseRequest {
            quantity,
            number_of_repeats_allowed: number_of_repeats,
        });
    }
}
