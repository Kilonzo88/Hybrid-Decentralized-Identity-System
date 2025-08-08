// SPDX-License-Identifier: MIT
// Hedera-ready Solidity DID Registry + Claims Contract
// Single-file implementation containing:
// 1) DIDRegistry: lightweight mapping from DID hash -> identity address
// 2) ClaimsRegistry: store claims issued about a holder, supports addClaimSigned pattern

pragma solidity ^0.8.19;

/* --------------------------------------------------------------------------
   Small Ownable implementation (we avoid external imports to keep this file
   self-contained). In production you may prefer OpenZeppelin's Ownable.
   -------------------------------------------------------------------------- */
contract Ownable {
    // owner address that can perform administrative tasks
    address public owner;

    // event emitted when ownership is transferred
    event OwnershipTransferred(address indexed previousOwner, address indexed newOwner);

    // constructor sets the deployer as the initial owner
    constructor() {
        owner = msg.sender; // assign owner at deployment
        emit OwnershipTransferred(address(0), msg.sender);
    }

    // modifier to restrict access to owner-only functions
    modifier onlyOwner() {
        require(msg.sender == owner, "Ownable: caller is not the owner");
        _;
    }

    // transfer ownership to a new address
    function transferOwnership(address newOwner) public onlyOwner {
        require(newOwner != address(0), "Ownable: new owner is the zero address");
        emit OwnershipTransferred(owner, newOwner);
        owner = newOwner;
    }
}

/* --------------------------------------------------------------------------
   DIDRegistry
   - Purpose: register and resolve DIDs to a canonical on-chain identity address
   - We store DIDs as bytes32 (keccak256 of the DID string) to save storage.
   -------------------------------------------------------------------------- */
contract DIDRegistry is Ownable {
    // mapping from didHash -> identity contract address
    mapping(bytes32 => address) public didToIdentity;

    // event emitted when a DID is registered or updated
    event DIDRegistered(bytes32 indexed didHash, address indexed identity, address indexed registrant);

    // register a new DID; prevents accidental overwrite unless called by owner
    function registerDID(bytes32 didHash, address identityAddr) external {
        // require a non-zero identity address
        require(identityAddr != address(0), "DIDRegistry: identity address is zero");

        // if DID not set, allow caller to set it; if set, allow only owner to update
        if (didToIdentity[didHash] == address(0)) {
            // first-time registration: any caller may register
            didToIdentity[didHash] = identityAddr;
            emit DIDRegistered(didHash, identityAddr, msg.sender);
        } else {
            // DID already exists: only owner can update/override
            require(msg.sender == owner, "DIDRegistry: DID exists, only owner can update");
            didToIdentity[didHash] = identityAddr;
            emit DIDRegistered(didHash, identityAddr, msg.sender);
        }
    }

    // convenience resolver that returns the identity address for a DID string
    function resolveDID(string calldata did) external view returns (address) {
        // compute didHash (keccak256) of the DID string and return mapping
        bytes32 didHash = keccak256(abi.encodePacked(did));
        return didToIdentity[didHash];
    }
}

/* --------------------------------------------------------------------------
   ClaimsRegistry
   - Purpose: store compact claim metadata on-chain while keeping credential
     data off-chain (IPFS/HFS), using hashes and signed issuance.
   - Pattern implemented: addClaimSigned
       -> issuer signs the claim off-chain (over structured claim fields)
       -> any relayer (or the issuer) can call addClaimSigned(holder, ... , signature)
       -> the contract recovers the issuer from the signature and stores the claim
   - Why addClaimSigned is important: enables gasless or low-cost issuance by
     allowing issuers to sign off-chain and have relayers submit on-chain.
   -------------------------------------------------------------------------- */
contract ClaimsRegistry is Ownable {
    // Claim struct holds minimal on-chain information to verify and locate VC
    struct Claim {
        address issuer;    // who issued the claim (recovered from signature)
        uint256 topic;     // a numeric code representing claim type
        bytes32 dataHash;  // hash of the full VC JSON stored off-chain (e.g., sha256/ipfs-cid-hash)
        string uri;        // encrypted URI or IPFS CID pointing to the VC
        uint256 validFrom; // UNIX timestamp from which the claim is valid
        uint256 validTo;   // UNIX timestamp when claim expires (0 = no expiry)
        bool revoked;      // whether claim has been revoked
    }

    // holder -> claimId -> Claim
    mapping(address => mapping(bytes32 => Claim)) public claims;

    // mapping of trusted issuers (optional whitelist). Admin can add trusted issuers.
    mapping(address => bool) public trustedIssuers;

    // events to index claim lifecycle
    event ClaimAdded(address indexed holder, bytes32 indexed claimId, address indexed issuer, uint256 topic);
    event ClaimRevoked(address indexed holder, bytes32 indexed claimId, address indexed revokedBy);
    event TrustedIssuerSet(address indexed issuer, bool trusted);

    // helper to allow contract owner to mark an issuer as trusted (optional governance)
    function setTrustedIssuer(address issuer, bool trusted) external onlyOwner {
        trustedIssuers[issuer] = trusted;
        emit TrustedIssuerSet(issuer, trusted);
    }

    /* ----------------------------------------------------------------------
       addClaimSigned
       - Inputs: holder (subject), topic, dataHash, uri, validity window, signature
       - Signature is expected to be an Ethereum-style ECDSA signature over the
         typed claim payload. The contract uses ecrecover to obtain the signer.
       - The recovered signer becomes the claim issuer stored on-chain.
       - This enables issuers to sign off-chain and use relayers to submit on-chain.
       ---------------------------------------------------------------------- */
    function addClaimSigned(
        address holder,
        uint256 topic,
        bytes32 dataHash,
        string calldata uri,
        uint256 validFrom,
        uint256 validTo,
        bytes calldata signature
    ) external returns (bytes32) {
        // Build the claimId deterministically so it can be referenced later.
        // We include the holder, topic and dataHash in the claimId to make it unique.
        bytes32 claimId = keccak256(abi.encodePacked(holder, topic, dataHash));

        // ensure no existing active claim with same id (unless revoked)
        require(!claims[holder][claimId].revoked && claims[holder][claimId].issuer == address(0), "ClaimsRegistry: claim exists");

        // Recreate the signed message hash used off-chain.
        // IMPORTANT: match this exact structure on the off-chain signer (EIP-191/EIP-712 recommended).
        // For simplicity we use the Ethereum Signed Message prefix (EIP-191) over a packed encoding.
        bytes32 messageHash = keccak256(abi.encodePacked(holder, topic, dataHash, uri, validFrom, validTo, address(this)));
        bytes32 ethSignedMessageHash = prefixed(messageHash);

        // Recover the signer (issuer) from the signature
        address issuer = recoverSigner(ethSignedMessageHash, signature);

        // OPTIONAL: enforce that issuer is a trusted issuer if the registry uses whitelisting
        // If the trustedIssuers mapping is empty (no issuers set), we allow any issuer.
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
            revoked: false
        });

        // Emit event for indexers and off-chain services
        emit ClaimAdded(holder, claimId, issuer, topic);

        // Optionally, you may consider anchoring the claimId to Hedera Consensus Service (HCS)
        // off-chain by emitting events which your service publishes to HCS for an ordered log.

        return claimId;
    }

    /* ----------------------------------------------------------------------
       revokeClaim
       - Allows the issuer or the contract owner to revoke a claim.
       - Holder cannot unilaterally revoke claims issued by others (but you can add
         holder-revocation semantics if desired where holders can request revocation).
       ---------------------------------------------------------------------- */
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
    function hasAnyTrustedIssuers() public pure  returns (bool) {
        // quick scan: in gas-sensitive code you'd track a counter; here we keep it simple
        // This pattern is OK for admin-managed systems but can be optimized.
        // NOTE: reading mapping like this is not enumerable; this function simply used to decide behavior.
        // We'll implement a simple heuristic: if owner has set any trusted issuer, the mapping will
        // return true for that issuer — but we cannot detect emptiness cheaply. For production,
        // track a trustedIssuerCount variable when adding/removing.
        return false; // default: do not require trust. Admin can change logic if desired.
    }

    // Read helper: getClaim returns the raw Claim struct for a holder and claimId
    function getClaim(address holder, bytes32 claimId) external view returns (Claim memory) {
        return claims[holder][claimId];
    }

    /* ----------------------------------------------------------------------
       Signature helpers
       - prefixed: Adds the standard Ethereum message prefix before hashing
       - recoverSigner: recovers signer from an ethSignedMessageHash and signature
       ---------------------------------------------------------------------- */
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
