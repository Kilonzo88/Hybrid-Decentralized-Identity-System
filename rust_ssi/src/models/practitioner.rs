use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Practitioner {
    pub resource_type: String,
    pub id: String,
    pub identifier: Vec<Identifier>,
    pub name: Vec<HumanName>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Identifier {
    pub system: String,
    pub value: String,
    pub use: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HumanName {
    pub family: String,
    pub given: Vec<String>,
    pub use: Option<String>,
    pub prefix: Option<Vec<String>>,
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
                use: Some("official".to_string()),
                prefix: Some(vec!["Dr.".to_string()]),
            }],
        }
    }

    pub fn add_identifier(&mut self, system: String, value: String) {
        self.identifier.push(Identifier {
            system,
            value,
            use: Some("official".to_string()),
        });
    }
}
