use crate::models::*;
use crate::models::observation::ObservationComponent;
use crate::models::medication::DosageInstruction;
use serde_json::Value;
use std::collections::HashMap;

pub struct FHIRHandler {
    resources: HashMap<String, Value>,
}

impl FHIRHandler {
    pub fn new() -> Self {
        Self {
            resources: HashMap::new(),
        }
    }

    /// Parse FHIR JSON and create structured FHIR resources
    pub fn parse_fhir_json(&mut self, json_str: &str) -> Result<Bundle, Box<dyn std::error::Error>> {
        let json_value: Value = serde_json::from_str(json_str)?;
        
        if let Some(bundle) = json_value.as_object() {
            let mut bundle_struct = Bundle::new(
                bundle.get("id").and_then(|v| v.as_str()).unwrap_or("").to_string(),
                bundle.get("type").and_then(|v| v.as_str()).unwrap_or("").to_string(),
                bundle.get("timestamp").and_then(|v| v.as_str()).unwrap_or("").to_string(),
            );

            if let Some(entries) = bundle.get("entry").and_then(|v| v.as_array()) {
                for entry in entries {
                    if let Some(resource) = entry.get("resource") {
                        let resource_type = resource.get("resourceType")
                            .and_then(|v| v.as_str())
                            .unwrap_or("Unknown");
                        
                        match resource_type {
                            "Patient" => {
                                let patient = self.parse_patient(resource)?;
                                bundle_struct.add_entry(Resource::Patient(patient));
                            },
                            "Practitioner" => {
                                let practitioner = self.parse_practitioner(resource)?;
                                bundle_struct.add_entry(Resource::Practitioner(practitioner));
                            },
                            "Encounter" => {
                                let encounter = self.parse_encounter(resource)?;
                                bundle_struct.add_entry(Resource::Encounter(encounter));
                            },
                            "Observation" => {
                                let observation = self.parse_observation(resource)?;
                                bundle_struct.add_entry(Resource::Observation(observation));
                            },
                            "Condition" => {
                                let condition = self.parse_condition(resource)?;
                                bundle_struct.add_entry(Resource::Condition(condition));
                            },
                            "MedicationRequest" => {
                                let medication = self.parse_medication_request(resource)?;
                                bundle_struct.add_entry(Resource::MedicationRequest(medication));
                            },
                            _ => {
                                eprintln!("Unknown resource type: {}", resource_type);
                            }
                        }
                    }
                }
            }

            // Parse signature if present
            if let Some(signature) = bundle.get("signature") {
                let signature_struct = self.parse_signature(signature)?;
                bundle_struct.add_signature(signature_struct);
            }

            Ok(bundle_struct)
        } else {
            Err("Invalid FHIR Bundle JSON".into())
        }
    }

    fn parse_patient(&self, resource: &Value) -> Result<Patient, Box<dyn std::error::Error>> {
        let id = resource.get("id").and_then(|v| v.as_str()).unwrap_or("").to_string();
        let family = resource.get("name")
            .and_then(|n| n.as_array())
            .and_then(|arr| arr.first())
            .and_then(|n| n.get("family"))
            .and_then(|v| v.as_str())
            .unwrap_or("")
            .to_string();
        
        let given = resource.get("name")
            .and_then(|n| n.as_array())
            .and_then(|arr| arr.first())
            .and_then(|n| n.get("given"))
            .and_then(|v| v.as_array())
            .map(|arr| arr.iter().filter_map(|v| v.as_str()).map(|s| s.to_string()).collect())
            .unwrap_or(vec![]);
        
        let gender = resource.get("gender").and_then(|v| v.as_str()).unwrap_or("").to_string();
        let birth_date = resource.get("birthDate").and_then(|v| v.as_str()).unwrap_or("").to_string();
        
        let mut patient = Patient::new(id, family, given, gender, birth_date);
        
        // Parse identifiers
        if let Some(identifiers) = resource.get("identifier").and_then(|v| v.as_array()) {
            for identifier in identifiers {
                let system = identifier.get("system").and_then(|v| v.as_str()).unwrap_or("").to_string();
                let value = identifier.get("value").and_then(|v| v.as_str()).unwrap_or("").to_string();
                patient.add_identifier(system, value);
            }
        }
        
        // Parse telecom
        if let Some(telecom) = resource.get("telecom").and_then(|v| v.as_array()) {
            for contact in telecom {
                let system = contact.get("system").and_then(|v| v.as_str()).unwrap_or("").to_string();
                let value = contact.get("value").and_then(|v| v.as_str()).unwrap_or("").to_string();
                let use_value = contact.get("use").and_then(|v| v.as_str()).unwrap_or("").to_string();
                patient.add_contact(system, value, use_value);
            }
        }
        
        Ok(patient)
    }

    fn parse_practitioner(&self, resource: &Value) -> Result<Practitioner, Box<dyn std::error::Error>> {
        let id = resource.get("id").and_then(|v| v.as_str()).unwrap_or("").to_string();
        let family = resource.get("name")
            .and_then(|n| n.as_array())
            .and_then(|arr| arr.first())
            .and_then(|n| n.get("family"))
            .and_then(|v| v.as_str())
            .unwrap_or("")
            .to_string();
        
        let given = resource.get("name")
            .and_then(|n| n.as_array())
            .and_then(|arr| arr.first())
            .and_then(|n| n.get("given"))
            .and_then(|v| v.as_array())
            .map(|arr| arr.iter().filter_map(|v| v.as_str()).map(|s| s.to_string()).collect())
            .unwrap_or(vec![]);
        
        let mut practitioner = Practitioner::new(id, family, given);
        
        // Parse identifiers
        if let Some(identifiers) = resource.get("identifier").and_then(|v| v.as_array()) {
            for identifier in identifiers {
                let system = identifier.get("system").and_then(|v| v.as_str()).unwrap_or("").to_string();
                let value = identifier.get("value").and_then(|v| v.as_str()).unwrap_or("").to_string();
                practitioner.add_identifier(system, value);
            }
        }
        
        Ok(practitioner)
    }

    fn parse_encounter(&self, resource: &Value) -> Result<Encounter, Box<dyn std::error::Error>> {
        let id = resource.get("id").and_then(|v| v.as_str()).unwrap_or("").to_string();
        let status = resource.get("status").and_then(|v| v.as_str()).unwrap_or("").to_string();
        
        let class = if let Some(class_obj) = resource.get("class") {
            Coding {
                system: class_obj.get("system").and_then(|v| v.as_str()).unwrap_or("").to_string(),
                code: class_obj.get("code").and_then(|v| v.as_str()).unwrap_or("").to_string(),
                display: class_obj.get("display").and_then(|v| v.as_str()).unwrap_or("").to_string(),
            }
        } else {
            Coding {
                system: "".to_string(),
                code: "".to_string(),
                display: "".to_string(),
            }
        };
        
        let subject = Reference {
            reference: resource.get("subject")
                .and_then(|s| s.get("reference"))
                .and_then(|v| v.as_str())
                .unwrap_or("")
                .to_string(),
            display: None,
        };
        
        let period = if let Some(period_obj) = resource.get("period") {
            Period {
                start: period_obj.get("start").and_then(|v| v.as_str()).unwrap_or("").to_string(),
                end: period_obj.get("end").and_then(|v| v.as_str()).unwrap_or("").to_string(),
            }
        } else {
            Period {
                start: "".to_string(),
                end: "".to_string(),
            }
        };
        
        let mut encounter = Encounter::new(id, status, class, subject, period);
        
        // Parse participant
        if let Some(participants) = resource.get("participant").and_then(|v| v.as_array()) {
            for participant in participants {
                if let Some(individual) = participant.get("individual") {
                    let reference = Reference {
                        reference: individual.get("reference").and_then(|v| v.as_str()).unwrap_or("").to_string(),
                        display: None,
                    };
                    encounter.add_participant(reference);
                }
            }
        }
        
        // Parse reason code
        if let Some(reasons) = resource.get("reasonCode").and_then(|v| v.as_array()) {
            for reason in reasons {
                let coding = vec![Coding {
                    system: "".to_string(),
                    code: "".to_string(),
                    display: "".to_string(),
                }];
                let text = reason.get("text").and_then(|v| v.as_str()).map(|s| s.to_string());
                encounter.add_reason(coding, text);
            }
        }
        
        Ok(encounter)
    }

    fn parse_observation(&self, resource: &Value) -> Result<Observation, Box<dyn std::error::Error>> {
        let id = resource.get("id").and_then(|v| v.as_str()).unwrap_or("").to_string();
        let status = resource.get("status").and_then(|v| v.as_str()).unwrap_or("").to_string();
        
        let code = CodeableConcept {
            coding: vec![],
            text: resource.get("code").and_then(|c| c.get("text")).and_then(|v| v.as_str()).map(|s| s.to_string()),
        };
        
        let subject = Reference {
            reference: resource.get("subject")
                .and_then(|s| s.get("reference"))
                .and_then(|v| v.as_str())
                .unwrap_or("")
                .to_string(),
            display: None,
        };
        
        let mut observation = Observation::new(id, status, code, subject);
        
        // Parse category
        if let Some(categories) = resource.get("category").and_then(|v| v.as_array()) {
            for category in categories {
                let coding = vec![Coding {
                    system: category.get("coding")
                        .and_then(|c| c.as_array())
                        .and_then(|arr| arr.first())
                        .and_then(|c| c.get("system"))
                        .and_then(|v| v.as_str())
                        .unwrap_or("")
                        .to_string(),
                    code: category.get("coding")
                        .and_then(|c| c.as_array())
                        .and_then(|arr| arr.first())
                        .and_then(|c| c.get("code"))
                        .and_then(|v| v.as_str())
                        .unwrap_or("")
                        .to_string(),
                    display: category.get("coding")
                        .and_then(|c| c.as_array())
                        .and_then(|arr| arr.first())
                        .and_then(|c| c.get("display"))
                        .and_then(|v| v.as_str())
                        .unwrap_or("")
                        .to_string(),
                }];
                let category_concept = CodeableConcept { coding, text: None };
                observation.add_category(category_concept);
            }
        }
        
        // Parse encounter
        if let Some(encounter) = resource.get("encounter") {
            let encounter_ref = Reference {
                reference: encounter.get("reference").and_then(|v| v.as_str()).unwrap_or("").to_string(),
                display: None,
            };
            observation.set_encounter(encounter_ref);
        }
        
        // Parse effective date time
        if let Some(effective_date_time) = resource.get("effectiveDateTime") {
            observation.set_effective_date_time(effective_date_time.as_str().unwrap_or("").to_string());
        }
        
        // Parse performer
        if let Some(performers) = resource.get("performer").and_then(|v| v.as_array()) {
            for performer in performers {
                let reference = Reference {
                    reference: performer.get("reference").and_then(|v| v.as_str()).unwrap_or("").to_string(),
                    display: None,
                };
                observation.add_performer(reference);
            }
        }
        
        // Parse components
        if let Some(components) = resource.get("component").and_then(|v| v.as_array()) {
            let mut observation_components = vec![];
            for component in components {
                let component_code = CodeableConcept {
                    coding: vec![],
                    text: component.get("code").and_then(|c| c.get("text")).and_then(|v| v.as_str()).map(|s| s.to_string()),
                };
                
                let value_quantity = if let Some(value_obj) = component.get("valueQuantity") {
                    Quantity {
                        value: value_obj.get("value").and_then(|v| v.as_f64()).unwrap_or(0.0),
                        unit: value_obj.get("unit").and_then(|v| v.as_str()).unwrap_or("").to_string(),
                        system: None,
                        code: None,
                    }
                } else {
                    Quantity {
                        value: 0.0,
                        unit: "".to_string(),
                        system: None,
                        code: None,
                    }
                };
                
                observation_components.push(ObservationComponent {
                    code: component_code,
                    value_quantity,
                });
            }
            observation.set_components(observation_components);
        }
        
        Ok(observation)
    }

    fn parse_condition(&self, resource: &Value) -> Result<Condition, Box<dyn std::error::Error>> {
        let id = resource.get("id").and_then(|v| v.as_str()).unwrap_or("").to_string();
        let recorded_date = resource.get("recordedDate").and_then(|v| v.as_str()).unwrap_or("").to_string();
        
        let code = CodeableConcept {
            coding: vec![],
            text: resource.get("code").and_then(|c| c.get("text")).and_then(|v| v.as_str()).map(|s| s.to_string()),
        };
        
        let subject = Reference {
            reference: resource.get("subject")
                .and_then(|s| s.get("reference"))
                .and_then(|v| v.as_str())
                .unwrap_or("")
                .to_string(),
            display: None,
        };
        
        let mut condition = Condition::new(id, code, subject, recorded_date);
        
        // Parse clinical status
        if let Some(clinical_status) = resource.get("clinicalStatus") {
            let coding = vec![Coding {
                system: clinical_status.get("coding")
                    .and_then(|c| c.as_array())
                    .and_then(|arr| arr.first())
                    .and_then(|c| c.get("system"))
                    .and_then(|v| v.as_str())
                    .unwrap_or("")
                    .to_string(),
                code: clinical_status.get("coding")
                    .and_then(|c| c.as_array())
                    .and_then(|arr| arr.first())
                    .and_then(|c| c.get("code"))
                    .and_then(|v| v.as_str())
                    .unwrap_or("")
                    .to_string(),
                display: clinical_status.get("coding")
                    .and_then(|c| c.as_array())
                    .and_then(|arr| arr.first())
                    .and_then(|c| c.get("display"))
                    .and_then(|v| v.as_str())
                    .unwrap_or("")
                    .to_string(),
            }];
            let clinical_status_concept = CodeableConcept { coding, text: None };
            condition.set_clinical_status(clinical_status_concept);
        }
        
        // Parse encounter
        if let Some(encounter) = resource.get("encounter") {
            let encounter_ref = Reference {
                reference: encounter.get("reference").and_then(|v| v.as_str()).unwrap_or("").to_string(),
                display: None,
            };
            condition.set_encounter(encounter_ref);
        }
        
        // Parse asserter
        if let Some(asserter) = resource.get("asserter") {
            let asserter_ref = Reference {
                reference: asserter.get("reference").and_then(|v| v.as_str()).unwrap_or("").to_string(),
                display: None,
            };
            condition.set_asserter(asserter_ref);
        }
        
        Ok(condition)
    }

    fn parse_medication_request(&self, resource: &Value) -> Result<MedicationRequest, Box<dyn std::error::Error>> {
        let id = resource.get("id").and_then(|v| v.as_str()).unwrap_or("").to_string();
        let status = resource.get("status").and_then(|v| v.as_str()).unwrap_or("").to_string();
        let intent = resource.get("intent").and_then(|v| v.as_str()).unwrap_or("").to_string();
        let authored_on = resource.get("authoredOn").and_then(|v| v.as_str()).unwrap_or("").to_string();
        
        let medication = CodeableConcept {
            coding: vec![],
            text: resource.get("medicationCodeableConcept")
                .and_then(|m| m.get("text"))
                .and_then(|v| v.as_str())
                .map(|s| s.to_string()),
        };
        
        let subject = Reference {
            reference: resource.get("subject")
                .and_then(|s| s.get("reference"))
                .and_then(|v| v.as_str())
                .unwrap_or("")
                .to_string(),
            display: None,
        };
        
        let mut medication_request = MedicationRequest::new(id, status, intent, medication, subject, authored_on);
        
        // Parse encounter
        if let Some(encounter) = resource.get("encounter") {
            let encounter_ref = Reference {
                reference: encounter.get("reference").and_then(|v| v.as_str()).unwrap_or("").to_string(),
                display: None,
            };
            medication_request.set_encounter(encounter_ref);
        }
        
        // Parse requester
        if let Some(requester) = resource.get("requester") {
            let requester_ref = Reference {
                reference: requester.get("reference").and_then(|v| v.as_str()).unwrap_or("").to_string(),
                display: None,
            };
            medication_request.set_requester(requester_ref);
        }
        
        // Parse dosage instructions
        if let Some(dosage_instructions) = resource.get("dosageInstruction").and_then(|v| v.as_array()) {
            for instruction in dosage_instructions {
                let text = instruction.get("text").and_then(|v| v.as_str()).unwrap_or("").to_string();
                medication_request.add_dosage_instruction(text);
            }
        }
        
        Ok(medication_request)
    }

    fn parse_signature(&self, signature: &Value) -> Result<Signature, Box<dyn std::error::Error>> {
        let signature_types = if let Some(types) = signature.get("type").and_then(|v| v.as_array()) {
            types.iter().map(|t| SignatureType {
                system: t.get("system").and_then(|v| v.as_str()).unwrap_or("").to_string(),
                code: t.get("code").and_then(|v| v.as_str()).unwrap_or("").to_string(),
                display: t.get("display").and_then(|v| v.as_str()).unwrap_or("").to_string(),
            }).collect()
        } else {
            vec![]
        };
        
        let when = signature.get("when").and_then(|v| v.as_str()).unwrap_or("").to_string();
        let who = Reference {
            reference: signature.get("who")
                .and_then(|w| w.get("reference"))
                .and_then(|v| v.as_str())
                .unwrap_or("")
                .to_string(),
            display: None,
        };
        let data = signature.get("data").and_then(|v| v.as_str()).unwrap_or("").to_string();
        let sig_format = signature.get("sigFormat").and_then(|v| v.as_str()).map(|s| s.to_string());
        
        Ok(Signature {
            signature_type: signature_types,
            when,
            who,
            data,
            sig_format,
        })
    }

    /// Validate FHIR resources for completeness and correctness
    pub fn validate_bundle(&self, bundle: &Bundle) -> Result<(), Vec<String>> {
        let mut errors = Vec::new();
        
        // Check if bundle has required fields
        if bundle.id.is_empty() {
            errors.push("Bundle ID is required".to_string());
        }
        
        if bundle.bundle_type.is_empty() {
            errors.push("Bundle type is required".to_string());
        }
        
        if bundle.timestamp.is_empty() {
            errors.push("Bundle timestamp is required".to_string());
        }
        
        // Check if bundle has at least one entry
        if bundle.entry.is_empty() {
            errors.push("Bundle must have at least one entry".to_string());
        }
        
        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }
}
