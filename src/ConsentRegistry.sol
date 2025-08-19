// SPDX-License-Identifier: MIT
pragma solidity ^0.8.26;

import {EIP712} from "@openzeppelin/contracts/utils/cryptography/EIP712.sol";
import {ECDSA}  from "@openzeppelin/contracts/utils/cryptography/ECDSA.sol";
import {Ownable} from "@openzeppelin/contracts/access/Ownable.sol";

/**
 * @title ConsentRegistry
 * @notice On-chain, patient-centric consent for accessing off-chain medical data.
 * - Store only minimal consent facts (no PHI).
 * - Two ways to grant consent:
 *    (a) Direct: patient calls grantConsent() (simplest).
 *    (b) Delegated: doctor submits patient's EIP-712 signature grantConsentSigned().
 */
contract ConsentRegistry is EIP712, Ownable {
    using ECDSA for bytes32;

    // Keep scopes tight for MVP; you can extend later
    enum Scope {
        VisitSummary,  // access to visit summary
        Prescription   // access to prescription
    }

    struct Consent {
        uint64 validFrom; // unix seconds
        uint64 validTo;   // 0 = no expiry
        bool revoked;     // explicit revoke flag
        bool exists;      // existence marker
    }

    // holder => requester => scope => Consent
    mapping(address => mapping(address => mapping(uint8 => Consent))) public consents;

    // Nonces for EIP-712 delegated approvals (replay protection)
    mapping(address => uint256) public nonces;

    // EIP-712 typehash for the delegated consent message
    // NOTE: Keep the struct immutable; changing fields requires a new TYPEHASH/version
    bytes32 private constant CONSENT_PERMIT_TYPEHASH = keccak256(
        "ConsentPermit(address holder,address requester,uint8 scope,uint64 validFrom,uint64 validTo,uint256 nonce,address verifyingContract,uint256 chainId)"
    );

    event ConsentGranted(address indexed holder, address indexed requester, uint8 indexed scope, uint64 validFrom, uint64 validTo);
    event ConsentRevoked(address indexed holder, address indexed requester, uint8 indexed scope);

    constructor(address _owner) EIP712("ConsentRegistry", "1") Ownable(_owner) {}

    // --- Direct grant (patient submits tx) ---
    function grantConsent(address requester, Scope scope, uint64 validFrom, uint64 validTo) external {
        _writeConsent(msg.sender, requester, uint8(scope), validFrom, validTo);
    }

    // --- Delegated grant (doctor submits patient's signature) ---
    function grantConsentSigned(
        address holder,
        address requester,
        Scope scope,
        uint64 validFrom,
        uint64 validTo,
        bytes calldata signature
    ) external {
        // Build the EIP-712 typed data digest
        bytes32 structHash = keccak256(
            abi.encode(
                CONSENT_PERMIT_TYPEHASH,
                holder,
                requester,
                uint8(scope),
                validFrom,
                validTo,
                nonces[holder],
                address(this),
                block.chainid
            )
        );
        bytes32 digest = _hashTypedDataV4(structHash);

        // Recover signer and bump nonce
        address signer = ECDSA.recover(digest, signature);
        require(signer == holder, "ConsentRegistry: invalid signer");
        unchecked { nonces[holder]++; }

        _writeConsent(holder, requester, uint8(scope), validFrom, validTo);
    }

    // --- Revoke (only the holder) ---
    function revokeConsent(address requester, Scope scope) external {
        Consent storage c = consents[msg.sender][requester][uint8(scope)];
        require(c.exists, "ConsentRegistry: not found");
        c.revoked = true;
        emit ConsentRevoked(msg.sender, requester, uint8(scope));
    }

    // --- Read helper (enforce time window + not revoked) ---
    function hasConsent(address holder, address requester, Scope scope) external view returns (bool) {
        Consent storage c = consents[holder][requester][uint8(scope)];
        if (!c.exists || c.revoked) return false;
        if (c.validFrom != 0 && block.timestamp < c.validFrom) return false;
        if (c.validTo != 0 && block.timestamp > c.validTo) return false;
        return true;
    }

    // --- Internal writer (single source of truth) ---
    function _writeConsent(
        address holder,
        address requester,
        uint8 scope,
        uint64 validFrom,
        uint64 validTo
    ) internal {
        require(requester != address(0), "ConsentRegistry: bad requester");
        Consent storage c = consents[holder][requester][scope];
        c.validFrom = validFrom;
        c.validTo = validTo;
        c.revoked = false;  // overwrite any previous revoke
        c.exists = true;
        emit ConsentGranted(holder, requester, scope, validFrom, validTo);
    }
}
