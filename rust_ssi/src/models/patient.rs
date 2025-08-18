use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Patient {
    pub resource_type: String,
    pub id: String,
    pub identifier: Vec<Identifier>,
    pub name: Vec<HumanName>,
    pub gender: String,
    pub birth_date: String,
    pub telecom: Option<Vec<ContactPoint>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Identifier {
    pub system: String,
    pub value: String,
    pub use: Option<String>,
    #[serde(rename = "type")]
    pub identifier_type: Option<CodeableConcept>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HumanName {
    pub family: String,
    pub given: Vec<String>,
    pub use: Option<String>,
    pub prefix: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContactPoint {
    pub system: String,
    pub value: String,
    pub use: Option<String>,
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

impl Patient {
    pub fn new(id: String, family: String, given: Vec<String>, gender: String, birth_date: String) -> Self {
        Self {
            resource_type: "Patient".to_string(),
            id,
            identifier: Vec::new(),
            name: vec![HumanName {
                family,
                given,
                use: Some("official".to_string()),
                prefix: None,
            }],
            gender,
            birth_date,
            telecom: None,
        }
    }

    pub fn add_identifier(&mut self, system: String, value: String) {
        self.identifier.push(Identifier {
            system,
            value,
            use: Some("official".to_string()),
            identifier_type: None,
        });
    }

    pub fn add_contact(&mut self, system: String, value: String, use: String) {
        if self.telecom.is_none() {
            self.telecom = Some(Vec::new());
        }
        if let Some(ref mut telecom) = self.telecom {
            telecom.push(ContactPoint { system, value, use: Some(use) });
        }
    }
}
