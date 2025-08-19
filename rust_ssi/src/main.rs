mod models;
mod fhir_handler;
mod pdf_generator;

use models::*;
use fhir_handler::FHIRHandler;
use pdf_generator::PDFGenerator;
use std::fs;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🚀 FHIR to PDF Generator - MVP Version");
    println!("=====================================\n");
    
    // Read FHIR JSON from file
    let fhir_json = fs::read_to_string("FHIR/FHIRBundle.json")?;
    println!("📖 Loaded FHIR JSON from file");
    
    // Parse FHIR JSON into structured data
    let mut handler = FHIRHandler::new();
    let bundle = handler.parse_fhir_json(&fhir_json)?;
    println!("✅ Successfully parsed FHIR JSON into structured data");
    
    // Validate the bundle
    match handler.validate_bundle(&bundle) {
        Ok(()) => println!("✅ FHIR Bundle validation passed"),
        Err(errors) => {
            println!("❌ FHIR Bundle validation failed:");
            for error in errors {
                println!("   - {}", error);
            }
        }
    }
    
    // Generate PDF (text file for MVP)
    let output_path = "patient_visit_summary.txt";
    let pdf_generator = PDFGenerator::new(output_path.to_string());
    pdf_generator.generate_pdf(&bundle)?;
    
    println!("\n🎉 Successfully generated patient visit summary!");
    println!("📄 Output file: {}", output_path);
    
    // Display some extracted information
    println!("\n📊 Extracted Information Summary:");
    println!("--------------------------------");
    
    if let Some(patient) = extract_patient(&bundle) {
        println!("👤 Patient: {} {}", 
            patient.name.first().map(|n| n.given.join(" ")).unwrap_or_default(),
            patient.name.first().map(|n| &n.family).unwrap_or(&"".to_string())
        );
    }
    
    if let Some(practitioner) = extract_practitioner(&bundle) {
        println!("👨‍⚕️ Practitioner: {} {}", 
            practitioner.name.first().map(|n| n.prefix.as_ref().unwrap_or(&vec![]).join(" ")).unwrap_or_default(),
            practitioner.name.first().map(|n| &n.family).unwrap_or(&"".to_string())
        );
    }
    
    if let Some(encounter) = extract_encounter(&bundle) {
        println!("🏥 Visit Type: {}", encounter.class.display);
    }
    
    if let Some(medication) = extract_medication_request(&bundle) {
        println!("💊 Medication: {}", 
            medication.medication_codeable_concept.text.as_ref().unwrap_or(&"".to_string())
        );
    }
    
    Ok(())
}

// Helper functions to extract resources from bundle
fn extract_patient(bundle: &Bundle) -> Option<&Patient> {
    for entry in &bundle.entry {
        if let Resource::Patient(patient) = &entry.resource {
            return Some(patient);
        }
    }
    None
}

fn extract_practitioner(bundle: &Bundle) -> Option<&Practitioner> {
    for entry in &bundle.entry {
        if let Resource::Practitioner(practitioner) = &entry.resource {
            return Some(practitioner);
        }
    }
    None
}

fn extract_encounter(bundle: &Bundle) -> Option<&Encounter> {
    for entry in &bundle.entry {
        if let Resource::Encounter(encounter) = &entry.resource {
            return Some(encounter);
        }
    }
    None
}

fn extract_medication_request(bundle: &Bundle) -> Option<&MedicationRequest> {
    for entry in &bundle.entry {
        if let Resource::MedicationRequest(medication) = &entry.resource {
            return Some(medication);
        }
    }
    None
}
