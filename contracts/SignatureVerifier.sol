// SPDX-License-Identifier: MIT
pragma solidity ^0.8.27;

import "@openzeppelin/contracts/utils/cryptography/EIP712.sol";
import "@openzeppelin/contracts/utils/cryptography/ECDSA.sol";

contract SignatureVerifier is EIP712 {
    string private constant SIGNING_DOMAIN = "AssetOwnership";
    string private constant SIGNATURE_VERSION = "1";

    bytes32 private constant ASSET_TYPE_HASH = keccak256("Asset(string name,string serial,address owner)");

    error INVALID_SIGNATURE(address, address);


    constructor() EIP712(SIGNING_DOMAIN, SIGNATURE_VERSION) {}

    struct Asset {
        string name;
        string serial;
        address owner;
    }

    function verifyAssetSignature(
        Asset memory asset,
        bytes memory signature,
        address expectedSigner
    ) public view returns (bool) {
        // Build the struct hash
        bytes32 structHash = keccak256(abi.encode(
            ASSET_TYPE_HASH,
            keccak256(bytes(asset.name)),
            keccak256(bytes(asset.serial)),
            asset.owner
        ));

        // Hash with domain separator
        bytes32 digest = _hashTypedDataV4(structHash);

        address signer = ECDSA.recover(digest, signature);

        if (signer != expectedSigner) revert INVALID_SIGNATURE(signer, expectedSigner);

        return true;
    }
}
