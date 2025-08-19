// // SPDX-License-Identifier: MIT
// pragma solidity ^0.8.21;

// /**
//  * @title FHIRUtils
//  * @dev Utilities for generating and validating FHIR (Fast Healthcare Interoperability Resources) bundles
//  * This contract provides helper functions for creating standardized healthcare documents
//  */
// contract FHIRUtils {
    
//     // FHIR Resource Types that correspond to our ClaimType enum
//     enum FHIRResourceType {
//         Composition,           // EHR Summary
//         DiagnosticReport,     // Lab Results
//         MedicationRequest,    // Prescriptions
//         Media,                // Medical Imaging
//         Immunization,         // Vaccinations
//         AllergyIntolerance,   // Allergies
//         Coverage,             // Insurance
//         Consent,              // Patient Consent
//         DocumentReference,    // Other documents
//         Observation           // Individual test results
//     }
    
//     // FHIR Bundle Types
//     enum BundleType {
//         document,             // Document bundle (what we use)
//         message,              // Message bundle
//         transaction,          // Transaction bundle
//         transaction_response, // Transaction response
//         batch,                // Batch bundle
//         batch_response,       // Batch response
//         history,              // History bundle
//         searchset,            // Search results
//         collection            // Collection bundle
//     }
    
//     // FHIR Status values
//     enum FHIRStatus {
//         active,
//         inactive,
//         error,
//         final,
//         preliminary,
//         registered,
//         cancelled,
//         entered_in_error,
//         unknown
//     }
    
//     // FHIR Intent values for MedicationRequest
//     enum FHIRIntent {
//         proposal,
//         plan,
//         order,
//         original_order,
//         reflex_order,
//         filler_order,
//         instance_order,
//         option
//     }
    
//     // FHIR Criticality values for AllergyIntolerance
//     enum FHIRCriticality {
//         low,
//         high,
//         unable_to_assess
//     }
    
//     /**
//      * @dev Generates a FHIR Bundle JSON string for a given claim type
//      * @param claimType The type of healthcare claim
//      * @param patientId The patient identifier
//      * @param timestamp The timestamp for the document
//      * @param hederaFileId The Hedera File Service file ID
//      * @return The FHIR Bundle JSON as a string
//      */
//     function generateFHIRBundle(
//         uint8 claimType,
//         string memory patientId,
//         string memory timestamp,
//         string memory hederaFileId
//     ) external pure returns (string memory) {
//         if (claimType == 0) { // EHR
//             return _generateEHRBundle(patientId, timestamp, hederaFileId);
//         } else if (claimType == 1) { // LAB_RESULTS
//             return _generateLabResultsBundle(patientId, timestamp, hederaFileId);
//         } else if (claimType == 2) { // PRESCRIPTION
//             return _generatePrescriptionBundle(patientId, timestamp, hederaFileId);
//         } else if (claimType == 3) { // MEDICAL_IMAGING
//             return _generateMedicalImagingBundle(patientId, timestamp, hederaFileId);
//         } else if (claimType == 4) { // VACCINATION
//             return _generateVaccinationBundle(patientId, timestamp, hederaFileId);
//         } else if (claimType == 5) { // ALLERGY
//             return _generateAllergyBundle(patientId, timestamp, hederaFileId);
//         } else if (claimType == 6) { // MEDICAL_HISTORY
//             return _generateMedicalHistoryBundle(patientId, timestamp, hederaFileId);
//         } else if (claimType == 7) { // INSURANCE
//             return _generateInsuranceBundle(patientId, timestamp, hederaFileId);
//         } else if (claimType == 8) { // CONSENT
//             return _generateConsentBundle(patientId, timestamp, hederaFileId);
//         } else { // OTHER
//             return _generateOtherBundle(patientId, timestamp, hederaFileId);
//         }
//     }
    
//     /**
//      * @dev Generates a comprehensive EHR bundle with multiple resource types
//      */
//     function _generateEHRBundle(
//         string memory patientId,
//         string memory timestamp,
//         string memory hederaFileId
//     ) internal pure returns (string memory) {
//         return string(abi.encodePacked(
//             '{"resourceType":"Bundle","type":"document","identifier":{"system":"did:hedera:testnet","value":"', hederaFileId, '"},"timestamp":"', timestamp, '","entry":[',
//             '{"resource":{"resourceType":"Composition","status":"final","type":{"text":"EHR Summary"},"subject":{"reference":"Patient/', patientId, '"},"date":"', _extractDate(timestamp), '","title":"Patient Health Record"}},',
//             '{"resource":{"resourceType":"DiagnosticReport","status":"final","code":{"text":"Comprehensive Lab Panel"},"result":[{"reference":"Observation/obs-001"},{"reference":"Observation/obs-002"}]}},',
//             '{"resource":{"resourceType":"Observation","status":"final","code":{"text":"Blood Pressure"},"valueQuantity":{"value":120,"unit":"mmHg"}}},',
//             '{"resource":{"resourceType":"Observation","status":"final","code":{"text":"Heart Rate"},"valueQuantity":{"value":72,"unit":"bpm"}}},',
//             '{"resource":{"resourceType":"MedicationRequest","status":"active","intent":"order","medicationCodeableConcept":{"text":"Lisinopril 10mg"}}},',
//             '{"resource":{"resourceType":"Immunization","status":"completed","vaccineCode":{"text":"Influenza Vaccine"},"occurrenceDateTime":"', _extractDate(timestamp), '"}},',
//             '{"resource":{"resourceType":"AllergyIntolerance","code":{"text":"No Known Allergies"},"criticality":"low"}},',
//             '{"resource":{"resourceType":"Coverage","status":"active","payor":[{"display":"Health Insurance Provider"}]}},',
//             '{"resource":{"resourceType":"Consent","status":"active","scope":{"text":"Treatment Consent"}}}',
//             ']}'
//         ));
//     }
    
//     /**
//      * @dev Generates a lab results bundle
//      */
//     function _generateLabResultsBundle(
//         string memory patientId,
//         string memory timestamp,
//         string memory hederaFileId
//     ) internal pure returns (string memory) {
//         return string(abi.encodePacked(
//             '{"resourceType":"Bundle","type":"document","identifier":{"system":"did:hedera:testnet","value":"', hederaFileId, '"},"timestamp":"', timestamp, '","entry":[',
//             '{"resource":{"resourceType":"Composition","status":"final","type":{"text":"Laboratory Report"},"subject":{"reference":"Patient/', patientId, '"},"date":"', _extractDate(timestamp), '","title":"Lab Results Report"}},',
//             '{"resource":{"resourceType":"DiagnosticReport","status":"final","code":{"text":"Complete Blood Count"},"result":[{"reference":"Observation/obs-001"},{"reference":"Observation/obs-002"},{"reference":"Observation/obs-003"}]}},',
//             '{"resource":{"resourceType":"Observation","status":"final","code":{"text":"Hemoglobin"},"valueQuantity":{"value":14.2,"unit":"g/dL"}}},',
//             '{"resource":{"resourceType":"Observation","status":"final","code":{"text":"White Blood Cell Count"},"valueQuantity":{"value":7.5,"unit":"K/uL"}}},',
//             '{"resource":{"resourceType":"Observation","status":"final","code":{"text":"Platelet Count"},"valueQuantity":{"value":250,"unit":"K/uL"}}}',
//             ']}'
//         ));
//     }
    
//     /**
//      * @dev Generates a prescription bundle
//      */
//     function _generatePrescriptionBundle(
//         string memory patientId,
//         string memory timestamp,
//         string memory hederaFileId
//     ) internal pure returns (string memory) {
//         return string(abi.encodePacked(
//             '{"resourceType":"Bundle","type":"document","identifier":{"system":"did:hedera:testnet","value":"', hederaFileId, '"},"timestamp":"', timestamp, '","entry":[',
//             '{"resource":{"resourceType":"Composition","status":"final","type":{"text":"Prescription"},"subject":{"reference":"Patient/', patientId, '"},"date":"', _extractDate(timestamp), '","title":"Medication Prescription"}},',
//             '{"resource":{"resourceType":"MedicationRequest","status":"active","intent":"order","medicationCodeableConcept":{"text":"Metformin 500mg"},"dosageInstruction":[{"text":"Take 1 tablet twice daily with meals"}]}},',
//             '{"resource":{"resourceType":"MedicationRequest","status":"active","intent":"order","medicationCodeableConcept":{"text":"Atorvastatin 20mg"},"dosageInstruction":[{"text":"Take 1 tablet daily at bedtime"}]}}',
//             ']}'
//         ));
//     }
    
//     /**
//      * @dev Generates a medical imaging bundle
//      */
//     function _generateMedicalImagingBundle(
//         string memory patientId,
//         string memory timestamp,
//         string memory hederaFileId
//     ) internal pure returns (string memory) {
//         return string(abi.encodePacked(
//             '{"resourceType":"Bundle","type":"document","identifier":{"system":"did:hedera:testnet","value":"', hederaFileId, '"},"timestamp":"', timestamp, '","entry":[',
//             '{"resource":{"resourceType":"Composition","status":"final","type":{"text":"Medical Imaging Report"},"subject":{"reference":"Patient/', patientId, '"},"date":"', _extractDate(timestamp), '","title":"Imaging Report"}},',
//             '{"resource":{"resourceType":"DiagnosticReport","status":"final","code":{"text":"Chest X-Ray"},"result":[{"reference":"Media/img-001"}]}},',
//             '{"resource":{"resourceType":"Media","status":"completed","type":"photo","content":{"contentType":"image/jpeg","data":"BASE64_IMAGE_DATA"}}}',
//             ']}'
//         ));
//     }
    
//     /**
//      * @dev Generates a vaccination bundle
//      */
//     function _generateVaccinationBundle(
//         string memory patientId,
//         string memory timestamp,
//         string memory hederaFileId
//     ) internal pure returns (string memory) {
//         return string(abi.encodePacked(
//             '{"resourceType":"Bundle","type":"document","identifier":{"system":"did:hedera:testnet","value":"', hederaFileId, '"},"timestamp":"', timestamp, '","entry":[',
//             '{"resource":{"resourceType":"Composition","status":"final","type":{"text":"Vaccination Record"},"subject":{"reference":"Patient/', patientId, '"},"date":"', _extractDate(timestamp), '","title":"Immunization Record"}},',
//             '{"resource":{"resourceType":"Immunization","status":"completed","vaccineCode":{"text":"COVID-19 Vaccine (Pfizer-BioNTech)"},"occurrenceDateTime":"', _extractDate(timestamp), '","lotNumber":"LOT123456"}},',
//             '{"resource":{"resourceType":"Immunization","status":"completed","vaccineCode":{"text":"Influenza Vaccine"},"occurrenceDateTime":"', _extractDate(timestamp), '","lotNumber":"LOT789012"}}',
//             ']}'
//         ));
//     }
    
//     /**
//      * @dev Generates an allergy bundle
//      */
//     function _generateAllergyBundle(
//         string memory patientId,
//         string memory timestamp,
//         string memory hederaFileId
//     ) internal pure returns (string memory) {
//         return string(abi.encodePacked(
//             '{"resourceType":"Bundle","type":"document","identifier":{"system":"did:hedera:testnet","value":"', hederaFileId, '"},"timestamp":"', timestamp, '","entry":[',
//             '{"resource":{"resourceType":"Composition","status":"final","type":{"text":"Allergy Assessment"},"subject":{"reference":"Patient/', patientId, '"},"date":"', _extractDate(timestamp), '","title":"Allergy Information"}},',
//             '{"resource":{"resourceType":"AllergyIntolerance","code":{"text":"Peanut Allergy"},"criticality":"high","reaction":[{"manifestation":[{"text":"Anaphylaxis"}]}]}},',
//             '{"resource":{"resourceType":"AllergyIntolerance","code":{"text":"Penicillin Allergy"},"criticality":"high","reaction":[{"manifestation":[{"text":"Rash"}]}}}',
//             ']}'
//         ));
//     }
    
//     /**
//      * @dev Generates a medical history bundle
//      */
//     function _generateMedicalHistoryBundle(
//         string memory patientId,
//         string memory timestamp,
//         string memory hederaFileId
//     ) internal pure returns (string memory) {
//         return string(abi.encodePacked(
//             '{"resourceType":"Bundle","type":"document","identifier":{"system":"did:hedera:testnet","value":"', hederaFileId, '"},"timestamp":"', timestamp, '","entry":[',
//             '{"resource":{"resourceType":"Composition","status":"final","type":{"text":"Medical History Summary"},"subject":{"reference":"Patient/', patientId, '"},"date":"', _extractDate(timestamp), '","title":"Patient Medical History"}},',
//             '{"resource":{"resourceType":"Condition","status":"active","code":{"text":"Type 2 Diabetes"},"onsetDateTime":"2020-01-01"}},',
//             '{"resource":{"resourceType":"Condition","status":"active","code":{"text":"Hypertension"},"onsetDateTime":"2019-06-15"}},',
//             '{"resource":{"resourceType":"Procedure","status":"completed","code":{"text":"Appendectomy"},"performedDateTime":"2018-03-20"}}',
//             ']}'
//         ));
//     }
    
//     /**
//      * @dev Generates an insurance bundle
//      */
//     function _generateInsuranceBundle(
//         string memory patientId,
//         string memory timestamp,
//         string memory hederaFileId
//     ) internal pure returns (string memory) {
//         return string(abi.encodePacked(
//             '{"resourceType":"Bundle","type":"document","identifier":{"system":"did:hedera:testnet","value":"', hederaFileId, '"},"timestamp":"', timestamp, '","entry":[',
//             '{"resource":{"resourceType":"Composition","status":"final","type":{"text":"Insurance Information"},"subject":{"reference":"Patient/', patientId, '"},"date":"', _extractDate(timestamp), '","title":"Insurance Coverage"}},',
//             '{"resource":{"resourceType":"Coverage","status":"active","payor":[{"display":"National Health Insurance Fund"}],"class":[{"type":{"text":"Group"},"value":"GOLD123","name":"Premium Plan"}]}},',
//             '{"resource":{"resourceType":"Coverage","status":"active","payor":[{"display":"Dental Insurance Co"}],"class":[{"type":{"text":"Group"},"value":"DENTAL456","name":"Dental Coverage"}]}}',
//             ']}'
//         ));
//     }
    
//     /**
//      * @dev Generates a consent bundle
//      */
//     function _generateConsentBundle(
//         string memory patientId,
//         string memory timestamp,
//         string memory hederaFileId
//     ) internal pure returns (string memory) {
//         return string(abi.encodePacked(
//             '{"resourceType":"Bundle","type":"document","identifier":{"system":"did:hedera:testnet","value":"', hederaFileId, '"},"timestamp":"', timestamp, '","entry":[',
//             '{"resource":{"resourceType":"Composition","status":"final","type":{"text":"Patient Consent"},"subject":{"reference":"Patient/', patientId, '"},"date":"', _extractDate(timestamp), '","title":"Consent Documentation"}},',
//             '{"resource":{"resourceType":"Consent","status":"active","scope":{"text":"Treatment Consent"},"category":[{"text":"Treatment"}],"patient":{"reference":"Patient/', patientId, '"}}},',
//             '{"resource":{"resourceType":"Consent","status":"active","scope":{"text":"Privacy Consent"},"category":[{"text":"Privacy"}],"patient":{"reference":"Patient/', patientId, '"}}}',
//             ']}'
//         ));
//     }
    
//     /**
//      * @dev Generates a generic other document bundle
//      */
//     function _generateOtherBundle(
//         string memory patientId,
//         string memory timestamp,
//         string memory hederaFileId
//     ) internal pure returns (string memory) {
//         return string(abi.encodePacked(
//             '{"resourceType":"Bundle","type":"document","identifier":{"system":"did:hedera:testnet","value":"', hederaFileId, '"},"timestamp":"', timestamp, '","entry":[',
//             '{"resource":{"resourceType":"Composition","status":"final","type":{"text":"Medical Document"},"subject":{"reference":"Patient/', patientId, '"},"date":"', _extractDate(timestamp), '","title":"Medical Document"}},',
//             '{"resource":{"resourceType":"DocumentReference","status":"current","description":"Other medical document","content":[{"attachment":{"contentType":"application/pdf","data":"BASE64_PDF_DATA"}}]}}',
//             ']}'
//         ));
//     }
    
//     /**
//      * @dev Helper function to extract date from timestamp
//      */
//     function _extractDate(string memory timestamp) internal pure returns (string memory) {
//         // Extract YYYY-MM-DD from ISO timestamp
//         bytes memory timestampBytes = bytes(timestamp);
//         require(timestampBytes.length >= 10, "Invalid timestamp format");
        
//         bytes memory dateBytes = new bytes(10);
//         for (uint i = 0; i < 10; i++) {
//             dateBytes[i] = timestampBytes[i];
//         }
        
//         return string(dateBytes);
//     }
    
//     /**
//      * @dev Validates that a string is valid JSON (basic check)
//      * @param jsonString The JSON string to validate
//      * @return True if the string appears to be valid JSON
//      */
//     function isValidJSON(string memory jsonString) external pure returns (bool) {
//         bytes memory jsonBytes = bytes(jsonString);
//         if (jsonBytes.length == 0) return false;
        
//         // Basic JSON validation - check for opening and closing braces
//         uint256 openBraces = 0;
//         uint256 openBrackets = 0;
        
//         for (uint256 i = 0; i < jsonBytes.length; i++) {
//             if (jsonBytes[i] == "{") {
//                 openBraces++;
//             } else if (jsonBytes[i] == "}") {
//                 if (openBraces == 0) return false;
//                 openBraces--;
//             } else if (jsonBytes[i] == "[") {
//                 openBrackets++;
//             } else if (jsonBytes[i] == "]") {
//                 if (openBrackets == 0) return false;
//                 openBrackets--;
//             }
//         }
        
//         return openBraces == 0 && openBrackets == 0;
//     }
    
//     /**
//      * @dev Gets the FHIR resource type for a given claim type
//      * @param claimType The claim type enum value
//      * @return The corresponding FHIR resource type
//      */
//     function getFHIRResourceType(uint8 claimType) external pure returns (FHIRResourceType) {
//         if (claimType == 0) return FHIRResourceType.Composition;        // EHR
//         if (claimType == 1) return FHIRResourceType.DiagnosticReport;   // LAB_RESULTS
//         if (claimType == 2) return FHIRResourceType.MedicationRequest;  // PRESCRIPTION
//         if (claimType == 3) return FHIRResourceType.Media;              // MEDICAL_IMAGING
//         if (claimType == 4) return FHIRResourceType.Immunization;       // VACCINATION
//         if (claimType == 5) return FHIRResourceType.AllergyIntolerance; // ALLERGY
//         if (claimType == 6) return FHIRResourceType.Composition;        // MEDICAL_HISTORY
//         if (claimType == 7) return FHIRResourceType.Coverage;           // INSURANCE
//         if (claimType == 8) return FHIRResourceType.Consent;            // CONSENT
//         return FHIRResourceType.DocumentReference;                       // OTHER
//     }
// }
