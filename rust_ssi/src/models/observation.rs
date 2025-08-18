use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Observation {
    pub resource_type: String,
    pub id: String,
    pub status: String,
    pub category: Vec<CodeableConcept>,
    pub code: CodeableConcept,
    pub subject: Reference,
    pub encounter: Option<Reference>,
    pub effective_date_time: Option<String>,
    pub performer: Vec<Reference>,
    pub component: Option<Vec<ObservationComponent>>,
    pub value_quantity: Option<Quantity>,
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
pub struct ObservationComponent {
    pub code: CodeableConcept,
    pub value_quantity: Quantity,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Quantity {
    pub value: f64,
    pub unit: String,
    pub system: Option<String>,
    pub code: Option<String>,
}

impl Observation {
    pub fn new(id: String, status: String, code: CodeableConcept, subject: Reference) -> Self {
        Self {
            resource_type: "Observation".to_string(),
            id,
            status,
            category: Vec::new(),
            code,
            subject,
            encounter: None,
            effective_date_time: None,
            performer: Vec::new(),
            component: None,
            value_quantity: None,
        }
    }

    pub fn add_category(&mut self, category: CodeableConcept) {
        self.category.push(category);
    }

    pub fn set_encounter(&mut self, encounter: Reference) {
        self.encounter = Some(encounter);
    }

    pub fn set_effective_date_time(&mut self, date_time: String) {
        self.effective_date_time = Some(date_time);
    }

    pub fn add_performer(&mut self, performer: Reference) {
        self.performer.push(performer);
    }

    pub fn set_components(&mut self, components: Vec<ObservationComponent>) {
        self.component = Some(components);
    }

    pub fn set_value_quantity(&mut self, value: f64, unit: String) {
        self.value_quantity = Some(Quantity {
            value,
            unit,
            system: Some("http://unitsofmeasure.org".to_string()),
            code: None,
        });
    }
}
