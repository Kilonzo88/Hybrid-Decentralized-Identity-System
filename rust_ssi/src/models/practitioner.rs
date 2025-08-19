use serde::{Deserialize, Serialize};
use super::common::{Identifier, HumanName, ContactPoint};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Practitioner {
    pub resource_type: String,
    pub id: String,
    pub identifier: Vec<Identifier>,
    pub name: Vec<HumanName>,
    pub telecom: Option<Vec<ContactPoint>>,
}

impl Practitioner {
    pub fn new(id: String, family: String, given: Vec<String>) -> Self {
        Self {
            resource_type: "Practitioner".to_string(),
            id,
            identifier: Vec::new(),
            name: vec![HumanName {
                family,
                given,
                //use_field: Some("official".to_string()),
                prefix: Some(vec!["Dr.".to_string()]),
            }],
            telecom: None,
        }
    }

    pub fn add_identifier(&mut self, system: String, value: String) {
        self.identifier.push(Identifier {
            system,
            value,
            //use_field: Some("official".to_string()),
            //identifier_type: None,
        });
    }

    pub fn add_contact(&mut self, system: String, value: String, use_value: String) {
        if self.telecom.is_none() {
            self.telecom = Some(Vec::new());
        }
        if let Some(ref mut telecom) = self.telecom {
            telecom.push(ContactPoint { system, value, use_field: Some(use_value) });
        }
    }
}
