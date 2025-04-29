require("@nomicfoundation/hardhat-toolbox");
const {vars} = require("hardhat/config");

/** @type import('hardhat/config').HardhatUserConfig */


module.exports = {
  // solidity: "0.8.27",

  solidity: {
    version: "0.8.27",
    settings: {
      optimizer: {
        enabled: true,
        runs: 1000,
      },
    },
  },

  networks: {
    base: {
      url: vars.get("BASE_URL"),
      accounts: [`0x${vars.get("PRIVATE_KEY")}`],
    },
  },
  etherscan: {
    apiKey: vars.get("BASE_SCAN_API"),
  },
};