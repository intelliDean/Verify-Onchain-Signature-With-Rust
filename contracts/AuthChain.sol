// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

import "@openzeppelin/contracts/utils/cryptography/EIP712.sol";
import "@openzeppelin/contracts/utils/cryptography/ECDSA.sol";

contract AuthChain is EIP712 {
    string private constant SIGNING_DOMAIN = "CertificateAuth";
    string private constant SIGNATURE_VERSION = "1";

    bytes32 private constant CERTIFICATE_TYPE_HASH =
    keccak256(
        "Certificate(string name,string uniqueId,string serial,uint256 date,address owner,string[] metadata)"
    );

    address private immutable owner;

    struct Certificate {
        string name;
        string uniqueId;
        string serial;
        uint256 date;
        address owner;
        string[] metadata;
    }

    struct Item {
        string name;
        bytes32 uniqueId;
        string serial;
        uint256 date;
        address owner;
    }

    error InvalidSignature(address signer, bool result);

    mapping(bytes32 => Item) public items;
    event ItemCreated(bytes32 structHash);
    event DebugHash(bytes32 structHash, bytes32 digest, address signer);

    constructor(address _owner) EIP712(SIGNING_DOMAIN, SIGNATURE_VERSION) {
        owner = _owner;
    }

    function createItem(Certificate memory certificate, bytes memory signature) external {
        (bool is_valid, bytes32 structHash) = verifyAssetSignature(certificate, signature);

        if (!is_valid) {
            revert InvalidSignature(certificate.owner, is_valid);
        }

        Item storage item = items[structHash];
        item.name = certificate.name;
        item.serial = certificate.serial;
        item.uniqueId = structHash;
        item.owner = certificate.owner;
        item.date = certificate.date;

        emit ItemCreated(structHash);
    }

    function getItem(bytes32 structHash) external view returns (Item memory) {
        return items[structHash];
    }

    function verifyAssetSignature(Certificate memory certificate, bytes memory signature)
    public
    returns (bool, bytes32)
    {
        bytes32 metadataHash = keccak256(abi.encode(certificate.metadata));
        bytes32 structHash = keccak256(
            abi.encode(
                CERTIFICATE_TYPE_HASH,
                keccak256(bytes(certificate.name)),
                keccak256(bytes(certificate.uniqueId)),
                keccak256(bytes(certificate.serial)),
                certificate.date,
                certificate.owner,
                metadataHash
            )
        );

        bytes32 digest = _hashTypedDataV4(structHash);
        address signer = ECDSA.recover(digest, signature);

        // Emit debug info
        emit DebugHash(structHash, digest, signer);

        return (signer == owner, structHash);
    }

    function getOwner() external view returns (address) {
        return owner;
    }
}