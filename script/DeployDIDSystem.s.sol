// SPDX-License-Identifier: MIT
pragma solidity ^0.8.19;

import "../src/DIDRegistry.sol";
import "../src/ClaimsRegistry.sol";

/**
 * @title DeployDIDSystem
 * @dev Deployment script for the DID Registry + Claims system
 * This script demonstrates how to deploy and initialize the system
 */
contract DeployDIDSystem is Script {
    DIDRegistry public didRegistry;
    ClaimsRegistry public claimsRegistry;
    
    // Example DID format: did:hedera:testnet:0.0.1234567
    string constant NETWORK = "testnet"; // This will be stored in the DIDMetadata.network field
    
    function run() external {
        uint256 deployerPrivateKey = vm.envUint("PRIVATE_KEY");
        address deployer = vm.addr(deployerPrivateKey);
        
        vm.startBroadcast(deployerPrivateKey);
        
        // Deploy DID Registry first
        didRegistry = new DIDRegistry();
        console.log("DID Registry deployed at:", address(didRegistry));
        
        // Deploy Claims Registry with reference to DID Registry
        claimsRegistry = new ClaimsRegistry(address(didRegistry));
        console.log("Claims Registry deployed at:", address(claimsRegistry));
        
        // Example: Register a healthcare provider DID
        // In production, this would be done by the actual healthcare provider
        string memory providerDID = "did:hedera:testnet:0.0.1234567";
        bytes32 providerDIDHash = keccak256(abi.encodePacked(providerDID));
        address providerAddress = 0x1234567890123456789012345678901234567890; // Example address
        
        didRegistry.registerDID(
            providerDIDHash,
            providerAddress,
            NETWORK,
            "0.0.1234567" // Hedera account ID
        );
        
        console.log("Provider DID registered:", providerDID);
        
        // Example: Register a patient DID
        string memory patientDID = "did:hedera:testnet:0.0.7654321";
        bytes32 patientDIDHash = keccak256(abi.encodePacked(patientDID));
        address patientAddress = 0x0987654321098765432109876543210987654321; // Example address
        
        didRegistry.registerDID(
            patientDIDHash,
            patientAddress,
            NETWORK,
            "0.0.7654321" // Hedera account ID
        );
        
        console.log("Patient DID registered:", patientDID);
        
        // Example: Register an HFS file (this would normally be done after uploading to HFS)
        // File ID format: 0.0.1234567 (Hedera File Service file ID)
        claimsRegistry.registerHFSFile(
            "0.0.1234567", // HFS file ID
            "application/json", // MIME type
            1024, // File size in bytes
            true, // Encrypted
            "0x1234567890abcdef..." // Encryption key hash
        );
        
        console.log("HFS file registered: 0.0.1234567");
        
        vm.stopBroadcast();
        
        console.log("=== Deployment Complete ===");
        console.log("DID Registry:", address(didRegistry));
        console.log("Claims Registry:", address(claimsRegistry));
        console.log("Network:", NETWORK);
        console.log("");
        console.log("=== Next Steps ===");
        console.log("1. Upload patient EHR data to Hedera File Service");
        console.log("2. Create Verifiable Credentials with the EHR data");
        console.log("3. Use addClaimSigned to register claims on-chain");
        console.log("4. Implement off-chain services for DID resolution and VC verification");
    }
}
