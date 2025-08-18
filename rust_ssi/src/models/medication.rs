use serde::{Deserialize, Serialize};

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
pub struct Period {
    pub start: String,
    pub end: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DoseAndRate {
    #[serde(rename = "type")]
    pub dose_type: Option<CodeableConcept>,
    pub dose_quantity: Option<Quantity>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Quantity {
    pub value: f64,
    pub unit: String,
    pub system: Option<String>,
    pub code: Option<String>,
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

    pub fn add_dosage_instruction(&mut self, instruction: DosageInstruction) {
        self.dosage_instruction.push(instruction);
    }

    pub fn set_dispense_request(&mut self, dispense_request: DispenseRequest) {
        self.dispense_request = Some(dispense_request);
    }
}
