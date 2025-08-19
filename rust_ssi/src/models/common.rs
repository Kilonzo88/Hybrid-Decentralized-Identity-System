use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Identifier {
    pub system: String,
    pub value: String,
    // #[serde(rename = "use")]
    // pub use_field: Option<String>,
    // #[serde(rename = "type")]
    // pub identifier_type: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HumanName {
    pub family: String,
    pub given: Vec<String>,
    // #[serde(rename = "use")]
    // pub use_field: Option<String>,
    pub prefix: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContactPoint {
    pub system: String,
    pub value: String,
    #[serde(rename = "use")]
    pub use_field: Option<String>,
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
pub struct Quantity {
    pub value: f64,
    pub unit: String,
    pub system: Option<String>,
    pub code: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Period {
    pub start: String,
    pub end: String,
}
