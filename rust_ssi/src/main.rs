// Bring ssi prelude for easy imports
use ssi::prelude::*;

// Define your credential subject data
#[derive(Serialize, Deserialize)]
struct MyClaims {
    name: String,
    role: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 1. Build the claims we want to include
    let claims = MyClaims { name: "Alice".into(), role: "Verifier".into() };
    let jwt_claims = JWTClaims::from_private_claims(claims);

    // 2. Generate a signing key (ECDSA P-256); reuse your existing DID key if available
    let mut key = JWK::generate_p256()?;
    let did = DIDJWK::generate_url(&key.to_public());
    key.key_id = Some(did.clone().into());

    // 3. Sign the claims as JWT-VC
    let jwt = jwt_claims.sign(&key).await?;

    // 4. Prepare the Verifiable Credential (JSON-LD Data-Integrity)
    let mut vc = ssi::claims::vc::v1::Credential {
        context: vec![ssi::VC_CONTEXT.clone()],
        id: Some(URI::String("urn:uuid:1234".into())),
        types: vec!["VerifiableCredential".into()],
        issuer: Some(Issuer::URI(URI::String(did.clone()))),
        issuance_date: Some(chrono::Utc::now()),
        credential_subject: serde_json::json!({ "id": "did:hedera:0.0.1234", "claims": jwt }),
        proof: None,
        expiration_date: None,
        credential_status: None,
    };

    // 5. Sign the VC using Data Integrity proof
    ssi::claims::vc::data_integrity::Credential::add_proof(&mut vc, &key, None).await?;

    // 6. Serialize to JSON
    let vc_json = serde_json::to_string_pretty(&vc)?;
    println!("Signed VC JSON:\n{}", vc_json);

    // 7. Compute hash to anchor on-chain
    let data_hash = sha2::Sha256::digest(vc_json.as_bytes());
    println!("SHA-256 data hash: 0x{}", hex::encode(data_hash));

    Ok(())
}
