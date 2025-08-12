// SPDX-License-Identifier: MIT
pragma solidity ^0.8.21;

import "./DIDRegistry.sol";

/**
 * @title ClaimsRegistry
 * @dev Manages Verifiable Credential claims with Hedera File Service integration
 * Stores compact claim metadata on-chain while keeping credential data off-chain (HFS)
 */
contract ClaimsRegistry {
    // Reference to the DID Registry
    DIDRegistry public didRegistry;
    
    // owner address that can perform administrative tasks
    address public owner;

    // Claim struct holds minimal on-chain information to verify and locate VC
    struct Claim {
        address issuer;           // who issued the claim (recovered from signature)
        uint256 topic;            // a numeric code representing claim type
        bytes32 dataHash;         // hash of the full VC JSON stored off-chain
        string uri;               // HFS file ID or encrypted URI pointing to the VC
        uint256 validFrom;        // UNIX timestamp from which the claim is valid
        uint256 validTo;          // UNIX timestamp when claim expires (0 = no expiry)
        bool revoked;             // whether claim has been revoked
        ClaimType claimType;      // type of claim (EHR, lab results, prescriptions, etc.)
        string hederaFileId;      // specific HFS file ID if using Hedera File Service
    }
    
    // Claim types for healthcare data
    enum ClaimType {
        EHR,                    // Electronic Health Record
        LAB_RESULTS,           // Laboratory test results
        PRESCRIPTION,          // Medication prescriptions
        MEDICAL_IMAGING,       // X-rays, MRIs, CT scans
        VACCINATION,           // Vaccination records
        ALLERGY,               // Allergy information
        MEDICAL_HISTORY,       // Medical history summary
        INSURANCE,             // Insurance information
        CONSENT,               // Patient consent forms
        OTHER                  // Other medical documents
    }
    
    // HFS File metadata for tracking files on Hedera
    struct HFSFile {
        string fileId;         // Hedera File Service file ID
        string mimeType;       // MIME type of the file
        uint256 size;          // File size in bytes
        uint256 uploadedAt;    // When file was uploaded
        bool encrypted;        // Whether file is encrypted
        string encryptionKeyHash; // Hash of encryption key (if encrypted)
    }

    // holder -> claimId -> Claim
    mapping(address => mapping(bytes32 => Claim)) public claims;
    
    // HFS file tracking: fileId -> HFSFile metadata
    mapping(string => HFSFile) public hfsFiles;
    
    // mapping of trusted issuers (optional whitelist). Admin can add trusted issuers.
    mapping(address => bool) public trustedIssuers;

    // events to index claim lifecycle
    event ClaimAdded(address indexed holder, bytes32 indexed claimId, address indexed issuer, uint256 topic, ClaimType claimType);
    event ClaimRevoked(address indexed holder, bytes32 indexed claimId, address indexed revokedBy);
    event TrustedIssuerSet(address indexed issuer, bool trusted);
    event HFSFileRegistered(string indexed fileId, string mimeType, uint256 size, bool encrypted);
    event OwnershipTransferred(address indexed previousOwner, address indexed newOwner);

    // constructor sets the deployer as the initial owner and DID registry
    constructor(address _didRegistry) {
        owner = msg.sender;
        didRegistry = DIDRegistry(_didRegistry);
        emit OwnershipTransferred(address(0), msg.sender);
    }

    // modifier to restrict access to owner-only functions
    modifier onlyOwner() {
        require(msg.sender == owner, "ClaimsRegistry: caller is not the owner");
        _;
    }

    // transfer ownership to a new address
    function transferOwnership(address newOwner) public onlyOwner {
        require(newOwner != address(0), "ClaimsRegistry: new owner is the zero address");
        emit OwnershipTransferred(owner, newOwner);
        owner = newOwner;
    }

    // helper to allow contract owner to mark an issuer as trusted (optional governance)
    function setTrustedIssuer(address issuer, bool trusted) external onlyOwner {
        trustedIssuers[issuer] = trusted;
        emit TrustedIssuerSet(issuer, trusted);
    }
    
    // Register HFS file metadata for tracking
    function registerHFSFile(
        string calldata fileId,
        string calldata mimeType,
        uint256 size,
        bool encrypted,
        string calldata encryptionKeyHash
    ) external onlyOwner {
        require(bytes(fileId).length > 0, "ClaimsRegistry: fileId cannot be empty");
        require(size > 0, "ClaimsRegistry: file size must be greater than 0");
        
        hfsFiles[fileId] = HFSFile({
            fileId: fileId,
            mimeType: mimeType,
            size: size,
            uploadedAt: block.timestamp,
            encrypted: encrypted,
            encryptionKeyHash: encryptionKeyHash
        });
        
        emit HFSFileRegistered(fileId, mimeType, size, encrypted);
    }

    /**
     * @dev addClaimSigned - Add a signed claim with HFS integration
     * @param holder - The subject of the claim
     * @param topic - Claim type identifier
     * @param dataHash - Hash of the VC data
     * @param uri - URI or HFS file ID
     * @param validFrom - Validity start timestamp
     * @param validTo - Validity end timestamp
     * @param claimType - Type of healthcare claim
     * @param hederaFileId - HFS file ID
     * @param signature - ECDSA signature from issuer
     * @return claimId - Unique identifier for the claim
     */
    function addClaimSigned(
        address holder,
        uint256 topic,
        bytes32 dataHash,
        string calldata uri,
        uint256 validFrom,
        uint256 validTo,
        ClaimType claimType,
        string calldata hederaFileId,
        bytes calldata signature
    ) external returns (bytes32) {
        // Build the claimId deterministically so it can be referenced later.
        // We include the holder, topic and dataHash in the claimId to make it unique.
        bytes32 claimId = keccak256(abi.encodePacked(holder, topic, dataHash));

        // ensure no existing active claim with same id (unless revoked)
        require(!claims[holder][claimId].revoked && claims[holder][claimId].issuer == address(0), "ClaimsRegistry: claim exists");

        // Recreate the signed message hash used off-chain.
        // IMPORTANT: match this exact structure on the off-chain signer (EIP-191/EIP-712 recommended).
        bytes32 messageHash = keccak256(abi.encodePacked(holder, topic, dataHash, uri, validFrom, validTo, claimType, hederaFileId, address(this)));
        bytes32 ethSignedMessageHash = prefixed(messageHash);

        // Recover the signer (issuer) from the signature
        address issuer = recoverSigner(ethSignedMessageHash, signature);

        // OPTIONAL: enforce that issuer is a trusted issuer if the registry uses whitelisting
        if (hasAnyTrustedIssuers()) {
            require(trustedIssuers[issuer], "ClaimsRegistry: issuer not trusted");
        }

        // Store the claim compactly
        claims[holder][claimId] = Claim({
            issuer: issuer,
            topic: topic,
            dataHash: dataHash,
            uri: uri,
            validFrom: validFrom,
            validTo: validTo,
            revoked: false,
            claimType: claimType,
            hederaFileId: hederaFileId
        });

        // Emit event for indexers and off-chain services
        emit ClaimAdded(holder, claimId, issuer, topic, claimType);

        return claimId;
    }

    /**
     * @dev revokeClaim - Allows the issuer or the contract owner to revoke a claim.
     * @param holder - The subject of the claim
     * @param claimId - The claim identifier to revoke
     */
    function revokeClaim(address holder, bytes32 claimId) external {
        Claim storage c = claims[holder][claimId];
        require(c.issuer != address(0), "ClaimsRegistry: claim not found");

        // Only issuer or contract owner may revoke
        require(msg.sender == c.issuer || msg.sender == owner, "ClaimsRegistry: not authorized to revoke");

        // mark as revoked
        c.revoked = true;
        emit ClaimRevoked(holder, claimId, msg.sender);
    }

    // helper to check if any trusted issuer has been set
    function hasAnyTrustedIssuers() public pure returns (bool) {
        // For now, return false to allow any issuer
        // In production, you'd track a trustedIssuerCount variable
        return false;
    }

    // Read helper: getClaim returns the raw Claim struct for a holder and claimId
    function getClaim(address holder, bytes32 claimId) external view returns (Claim memory) {
        return claims[holder][claimId];
    }
    
    // Get HFS file metadata by file ID
    function getHFSFile(string calldata fileId) external view returns (HFSFile memory) {
        return hfsFiles[fileId];
    }
    
    // Check if a claim references a specific HFS file
    function isClaimReferencingHFSFile(address holder, bytes32 claimId, string calldata fileId) external view returns (bool) {
        Claim storage claim = claims[holder][claimId];
        return keccak256(abi.encodePacked(claim.hederaFileId)) == keccak256(abi.encodePacked(fileId));
    }

    /**
     * @dev Signature helpers
     * prefixed: Adds the standard Ethereum message prefix before hashing
     * recoverSigner: recovers signer from an ethSignedMessageHash and signature
     */
    function prefixed(bytes32 hash) internal pure returns (bytes32) {
        // EIP-191: "\x19Ethereum Signed Message:\n32" + hash
        return keccak256(abi.encodePacked("\x19Ethereum Signed Message:\n32", hash));
    }

    function recoverSigner(bytes32 ethSignedMessageHash, bytes memory sig) internal pure returns (address) {
        // Check the signature length
        require(sig.length == 65, "ClaimsRegistry: invalid signature length");

        bytes32 r;
        bytes32 s;
        uint8 v;

        // solhint-disable-next-line no-inline-assembly
        assembly {
            r := mload(add(sig, 32))
            s := mload(add(sig, 64))
            v := byte(0, mload(add(sig, 96)))
        }

        // Version of signature should be 27 or 28
        if (v < 27) {
            v += 27;
        }

        require(v == 27 || v == 28, "ClaimsRegistry: invalid v value");

        // recover the signer
        address signer = ecrecover(ethSignedMessageHash, v, r, s);
        require(signer != address(0), "ClaimsRegistry: invalid signer");
        return signer;
    }
}
