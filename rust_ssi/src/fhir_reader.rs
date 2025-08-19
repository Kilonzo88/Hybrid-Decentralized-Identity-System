// use std::fs;
// use crate::fhir_handler::FHIRHandler;

// fn main() {
//     // 1. Read the JSON file (FHIR bundle)
//     let json_str = fs::read_to_string("FHIR/FHIRBundle.json")
//         .expect("Unable to read FHIR bundle file");

//     // 2. Parse using your handler
//     let mut handler = FHIRHandler::new();
//     let bundle = handler.parse_fhir_json(&json_str)
//         .expect("Failed to parse FHIR JSON");

//     // 3. Validate the bundle
//     match handler.validate_bundle(&bundle) {
//         Ok(_) => println!("FHIR bundle is valid ✅"),
//         Err(errors) => println!("Validation errors: {:?}", errors),
//     }

//     // 4. (Optional) Print a summary
//     println!("Bundle ID: {}", bundle.id);
//     println!("Bundle Type: {}", bundle.bundle_type);
//     println!("Number of entries: {}", bundle.entry.len());
// }
