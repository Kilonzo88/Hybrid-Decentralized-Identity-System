use crate::models::*;
use std::fs::File;
use std::io::Write;

pub struct PDFGenerator {
    output_path: String,
}

impl PDFGenerator {
    pub fn new(output_path: String) -> Self {
        Self { output_path }
    }

    /// Generate a PDF from FHIR Bundle data
    pub fn generate_pdf(&self, bundle: &Bundle) -> Result<(), Box<dyn std::error::Error>> {
        // For MVP, we'll generate a simple text-based PDF-like document
        // In production, you'd use a proper PDF library like printpdf or wkhtmltopdf
        
        let mut content = String::new();
        
        // Add header
        content.push_str("PATIENT VISIT SUMMARY & PRESCRIPTION\n");
        content.push_str("=====================================\n\n");
        
        // Extract and format patient information
        if let Some(patient) = self.extract_patient(bundle) {
            content.push_str("PATIENT INFORMATION\n");
            content.push_str("-------------------\n");
            content.push_str(&format!("Name: {}\n", self.format_patient_name(&patient)));
            content.push_str(&format!("Gender: {}\n", patient.gender));
            content.push_str(&format!("Date of Birth: {}\n", patient.birth_date));
            
            if !patient.identifier.is_empty() {
                content.push_str(&format!("MRN: {}\n", patient.identifier[0].value));
            }
            
            if let Some(ref telecom) = patient.telecom {
                for contact in telecom {
                    content.push_str(&format!("{}: {}\n", contact.system, contact.value));
                }
            }
            content.push_str("\n");
        }
        
        // Extract and format practitioner information
        if let Some(practitioner) = self.extract_practitioner(bundle) {
            content.push_str("PRACTITIONER INFORMATION\n");
            content.push_str("------------------------\n");
            content.push_str(&format!("Doctor: {}\n", self.format_practitioner_name(&practitioner)));
            
            if !practitioner.identifier.is_empty() {
                content.push_str(&format!("NPI: {}\n", practitioner.identifier[0].value));
            }
            content.push_str("\n");
        }
        
        // Extract and format encounter information
        if let Some(encounter) = self.extract_encounter(bundle) {
            content.push_str("VISIT DETAILS\n");
            content.push_str("-------------\n");
            content.push_str(&format!("Encounter ID: {}\n", encounter.id));
            content.push_str(&format!("Status: {}\n", encounter.status));
            content.push_str(&format!("Class: {}\n", encounter.class.display));
            
            if !encounter.reason_code.is_empty() {
                if let Some(ref text) = encounter.reason_code[0].text {
                    content.push_str(&format!("Reason: {}\n", text));
                }
            }
            content.push_str("\n");
        }
        
        // Extract and format observations
        if let Some(observation) = self.extract_observation(bundle) {
            content.push_str("OBSERVATIONS\n");
            content.push_str("------------\n");
            content.push_str(&format!("Type: {}\n", observation.code.text.as_ref().unwrap_or(&"".to_string())));
            
            if let Some(ref components) = observation.component {
                for component in components {
                    content.push_str(&format!("{}: {} {}\n", 
                        component.code.text.as_ref().unwrap_or(&"".to_string()),
                        component.value_quantity.value,
                        component.value_quantity.unit));
                }
            }
            content.push_str("\n");
        }
        
        // Extract and format conditions
        if let Some(condition) = self.extract_condition(bundle) {
            content.push_str("DIAGNOSIS\n");
            content.push_str("---------\n");
            content.push_str(&format!("Condition: {}\n", condition.code.text.as_ref().unwrap_or(&"".to_string())));
            content.push_str(&format!("Recorded Date: {}\n", condition.recorded_date));
            content.push_str("\n");
        }
        
        // Extract and format medication request
        if let Some(medication) = self.extract_medication_request(bundle) {
            content.push_str("PRESCRIPTION\n");
            content.push_str("------------\n");
            content.push_str(&format!("Medication: {}\n", 
                medication.medication_codeable_concept.text.as_ref().unwrap_or(&"".to_string())));
            content.push_str(&format!("Status: {}\n", medication.status));
            content.push_str(&format!("Intent: {}\n", medication.intent));
            content.push_str(&format!("Authored On: {}\n", medication.authored_on));
            
            if !medication.dosage_instruction.is_empty() {
                content.push_str(&format!("Instructions: {}\n", medication.dosage_instruction[0].text));
            }
            content.push_str("\n");
        }
        
        // Add signature information
        if let Some(ref signature) = bundle.signature {
            content.push_str("DIGITAL SIGNATURE\n");
            content.push_str("-----------------\n");
            content.push_str(&format!("Signed By: {}\n", signature.who.reference));
            content.push_str(&format!("Date: {}\n", signature.when));
            content.push_str(&format!("Signature Hash: {}\n", signature.data));
        }
        
        // Write to file (for MVP, we'll create a text file that can be converted to PDF)
        let mut file = File::create(&self.output_path)?;
        file.write_all(content.as_bytes())?;
        
        println!("Generated document: {}", self.output_path);
        println!("Note: For MVP, this is a text file. In production, use a PDF library.");
        
        Ok(())
    }
    
    fn extract_patient(&self, bundle: &Bundle) -> Option<&Patient> {
        for entry in &bundle.entry {
            if let Resource::Patient(patient) = &entry.resource {
                return Some(patient);
            }
        }
        None
    }
    
    fn extract_practitioner(&self, bundle: &Bundle) -> Option<&Practitioner> {
        for entry in &bundle.entry {
            if let Resource::Practitioner(practitioner) = &entry.resource {
                return Some(practitioner);
            }
        }
        None
    }
    
    fn extract_encounter(&self, bundle: &Bundle) -> Option<&Encounter> {
        for entry in &bundle.entry {
            if let Resource::Encounter(encounter) = &entry.resource {
                return Some(encounter);
            }
        }
        None
    }
    
    fn extract_observation(&self, bundle: &Bundle) -> Option<&Observation> {
        for entry in &bundle.entry {
            if let Resource::Observation(observation) = &entry.resource {
                return Some(observation);
            }
        }
        None
    }
    
    fn extract_condition(&self, bundle: &Bundle) -> Option<&Condition> {
        for entry in &bundle.entry {
            if let Resource::Condition(condition) = &entry.resource {
                return Some(condition);
            }
        }
        None
    }
    
    fn extract_medication_request(&self, bundle: &Bundle) -> Option<&MedicationRequest> {
        for entry in &bundle.entry {
            if let Resource::MedicationRequest(medication) = &entry.resource {
                return Some(medication);
            }
        }
        None
    }
    
    fn format_patient_name(&self, patient: &Patient) -> String {
        if let Some(name) = patient.name.first() {
            let given = name.given.join(" ");
            format!("{} {}", given, name.family)
        } else {
            "Unknown".to_string()
        }
    }
    
    fn format_practitioner_name(&self, practitioner: &Practitioner) -> String {
        if let Some(name) = practitioner.name.first() {
            let given = name.given.join(" ");
            let prefix = name.prefix.as_ref().map(|p| p.join(" ")).unwrap_or_default();
            format!("{} {} {}", prefix, given, name.family)
        } else {
            "Unknown".to_string()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::*;
    
    #[test]
    fn test_pdf_generator_creation() {
        let generator = PDFGenerator::new("test_output.txt".to_string());
        assert_eq!(generator.output_path, "test_output.txt");
    }
    
    #[test]
    fn test_format_patient_name() {
        let patient = Patient::new(
            "test-123".to_string(),
            "Doe".to_string(),
            vec!["John".to_string(), "Michael".to_string()],
            "male".to_string(),
            "1980-01-15".to_string(),
        );
        
        let generator = PDFGenerator::new("test.txt".to_string());
        let formatted_name = generator.format_patient_name(&patient);
        assert_eq!(formatted_name, "John Michael Doe");
    }
}
