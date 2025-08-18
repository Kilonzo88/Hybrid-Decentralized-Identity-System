# DIDs + Hedera File Service for Patient Data Security

A decentralized identity (DID) system built on Hedera blockchain for securing patient healthcare data through Verifiable Credentials (VCs) stored on Hedera File Service (HFS).

## 🏗️ Architecture Overview

```
Patient DID → Smart Contract (Claims Registry) → Hedera File Service (EHR Storage)
     ↓                    ↓                           ↓
Identity Resolution    Claim Metadata            Encrypted EHR Data
```

## 🔐 DID Specification

### **Format: `did:hedera:testnet:<account_or_file_id>`**

- **`did:hedera`** - Method identifier for Hedera blockchain
- **`testnet`** - Network identifier (testnet/mainnet/previewnet)
- **`<account_or_file_id>`** - Hedera account ID or file ID

### **Examples:**
- Patient: `did:hedera:testnet:0.0.1234567`
- Provider: `did:hedera:testnet:0.0.7654321`
- EHR File: `did:hedera:testnet:0.0.9999999`

## 📋 Claims Registry Structure

### **Claim Types (Healthcare Data)**
- `EHR` - Electronic Health Records
- `LAB_RESULTS` - Laboratory test results
- `PRESCRIPTION` - Medication prescriptions
- `MEDICAL_IMAGING` - X-rays, MRIs, CT scans
- `VACCINATION` - Vaccination records
- `ALLERGY` - Allergy information
- `MEDICAL_HISTORY` - Medical history summary
- `INSURANCE` - Insurance information
- `CONSENT` - Patient consent forms
- `OTHER` - Other medical documents

## 🗄️ Hedera File Service Integration

### **Why HFS for EHR Storage?**
- **Immutable** - Once written, files cannot be modified
- **Cost-effective** - ~$0.001 per file
- **HIPAA-compliant** - Supports encryption and access controls
- **Scalable** - Handles large medical files (X-rays, MRIs, etc.)

## 🔄 Complete Patient Data Flow

1. **Patient Registration** - Create DID on Hedera
2. **EHR Creation** - Healthcare provider creates Verifiable Credential
3. **HFS Storage** - Encrypted EHR stored on Hedera File Service
4. **On-chain Anchoring** - Claim metadata stored in smart contract
5. **Verification** - Anyone can verify EHR authenticity using DID + hash

## 🚀 Getting Started

### **Deployment**
```bash
# Set your private key
export PRIVATE_KEY="your_private_key_here"

# Deploy the system
forge script script/DeployDIDSystem.s.sol --rpc-url hedera_testnet --broadcast
```

## 🔗 Useful Links

- [Hedera Documentation](https://docs.hedera.com/)
- [Hedera File Service](https://docs.hedera.com/hedera/core-concepts/file-storage)
- [W3C DID Specification](https://www.w3.org/TR/did-core/)
- [W3C Verifiable Credentials](https://www.w3.org/TR/vc-data-model/)
