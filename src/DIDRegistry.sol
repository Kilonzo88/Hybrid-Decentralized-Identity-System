// SPDX-License-Identifier: MIT
pragma solidity ^0.8.21;

/**
 * @title DIDRegistry
 * @dev Manages Decentralized Identifiers (DIDs) on Hedera blockchain
 * Supports did:hedera:testnet:<account_or_file_id> format
 */
contract DIDRegistry {
    // owner address that can perform administrative tasks
    address public owner;

    // mapping from didHash -> identity contract address
    mapping(bytes32 => address) public didToIdentity;
    
    // mapping from didHash -> Hedera account/file ID for HFS integration
    mapping(bytes32 => string) public didToHederaId;
    
    // mapping from didHash -> DID metadata (network, type, etc.)
    mapping(bytes32 => DIDMetadata) public didMetadata;

    // DID metadata structure
    struct DIDMetadata {
        string network;        // "testnet", "mainnet", "previewnet"
        string hederaId;       // Hedera account ID or file ID
        uint256 created;       // timestamp when DID was created
        bool active;           // whether DID is active
    }

    // event emitted when a DID is registered or updated
    event DIDRegistered(bytes32 indexed didHash, address indexed identity, address indexed registrant, string hederaId);
    event DIDMetadataUpdated(bytes32 indexed didHash, string network, string hederaId);
    event OwnershipTransferred(address indexed previousOwner, address indexed newOwner);

    // constructor sets the deployer as the initial owner
    constructor() {
        owner = msg.sender;
        emit OwnershipTransferred(address(0), msg.sender);
    }

    // modifier to restrict access to owner-only functions
    modifier onlyOwner() {
        require(msg.sender == owner, "DIDRegistry: caller is not the owner");
        _;
    }

    // transfer ownership to a new address
    function transferOwnership(address newOwner) public onlyOwner {
        require(newOwner != address(0), "DIDRegistry: new owner is the zero address");
        emit OwnershipTransferred(owner, newOwner);
        owner = newOwner;
    }

    // register a new DID with Hedera integration
    function registerDID(
        bytes32 didHash, 
        address identityAddr,
        string calldata network,
        string calldata hederaId
    ) external {
        // require a non-zero identity address
        require(identityAddr != address(0), "DIDRegistry: identity address shouldn't be zero");
        require(bytes(network).length > 0, "DIDRegistry: network cannot be empty");
        require(bytes(hederaId).length > 0, "DIDRegistry: hederaId cannot be empty");

        // if DID not set, allow caller to set it; if set, allow only owner to update
        if (didToIdentity[didHash] == address(0)) {
            // first-time registration: any caller may register
            didToIdentity[didHash] = identityAddr;
            didToHederaId[didHash] = hederaId;
            didMetadata[didHash] = DIDMetadata({
                network: network,
                hederaId: hederaId,
                created: block.timestamp,
                active: true
            });
            emit DIDRegistered(didHash, identityAddr, msg.sender, hederaId);
        } else {
            // DID already exists: only owner can update/override
            require(msg.sender == owner, "DIDRegistry: DID exists, only owner can update");
            didToIdentity[didHash] = identityAddr;
            didToHederaId[didHash] = hederaId;
            didMetadata[didHash] = DIDMetadata({
                network: network,
                hederaId: hederaId,
                created: didMetadata[didHash].created, // preserve creation time
                active: true
            });
            emit DIDRegistered(didHash, identityAddr, msg.sender, hederaId);
        }
    }

    // convenience resolver that returns the identity address for a DID string
    function resolveDID(string calldata did) external view returns (address) {
        // compute didHash (keccak256) of the DID string and return mapping
        bytes32 didHash = keccak256(abi.encodePacked(did));
        return didToIdentity[didHash];
    }
    
    // get Hedera ID associated with a DID
    function getHederaId(string calldata did) external view returns (string memory) {
        bytes32 didHash = keccak256(abi.encodePacked(did));
        return didToHederaId[didHash];
    }
    
    // get complete DID metadata
    function getDIDMetadata(string calldata did) external view returns (DIDMetadata memory) {
        bytes32 didHash = keccak256(abi.encodePacked(did));
        return didMetadata[didHash];
    }
    
    // deactivate a DID (only owner can do this)
    function deactivateDID(string calldata did) external onlyOwner {
        bytes32 didHash = keccak256(abi.encodePacked(did));
        require(didToIdentity[didHash] != address(0), "DIDRegistry: DID does not exist");
        
        didMetadata[didHash].active = false;
        emit DIDMetadataUpdated(didHash, didMetadata[didHash].network, didMetadata[didHash].hederaId);
    }
    
    // check if a DID is active
    function isDIDActive(string calldata did) external view returns (bool) {
        bytes32 didHash = keccak256(abi.encodePacked(did));
        return didMetadata[didHash].active;
    }
    
    // get DID hash for a given DID string
    function getDIDHash(string calldata did) external pure returns (bytes32) {
        return keccak256(abi.encodePacked(did));
    }
}
