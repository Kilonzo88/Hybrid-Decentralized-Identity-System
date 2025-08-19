use serde::{Deserialize, Serialize};
use super::common::{Coding, CodeableConcept, Reference, Period};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Encounter {
    pub resource_type: String,
    pub id: String,
    pub status: String,
    pub class: Coding,
    #[serde(rename = "type")]
    pub encounter_type: Vec<CodeableConcept>,
    pub subject: Reference,
    pub participant: Vec<EncounterParticipant>,
    pub period: Period,
    pub reason_code: Vec<CodeableConcept>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EncounterParticipant {
    pub individual: Reference,
}

impl Encounter {
    pub fn new(id: String, status: String, class: Coding, subject: Reference, period: Period) -> Self {
        Self {
            resource_type: "Encounter".to_string(),
            id,
            status,
            class,
            encounter_type: Vec::new(),
            subject,
            participant: Vec::new(),
            period,
            reason_code: Vec::new(),
        }
    }

    pub fn add_type(&mut self, coding: Vec<Coding>, text: Option<String>) {
        self.encounter_type.push(CodeableConcept { coding, text });
    }

    pub fn add_participant(&mut self, individual: Reference) {
        self.participant.push(EncounterParticipant { individual });
    }

    pub fn add_reason(&mut self, coding: Vec<Coding>, text: Option<String>) {
        self.reason_code.push(CodeableConcept { coding, text });
    }
}
