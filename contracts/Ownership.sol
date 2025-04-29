// SPDX-License-Identifier: MIT
pragma solidity ^0.8.27;

contract Ownership {

    mapping(string => address) public assetOwner;

    event Owner(address owner);

    function registerAsset(string memory assetId) external {
        require(assetOwner[assetId] == address(0), "Already registered");
        assetOwner[assetId] = msg.sender;

        emit Owner(msg.sender);
    }

    function verifyOwnership(string memory assetId) external view returns (bool) {
        return assetOwner[assetId] == msg.sender;
    }

    function getOwner(string memory assetId) external view returns(address) {
        return assetOwner[assetId];
    }
}
