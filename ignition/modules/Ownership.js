// This setup uses Hardhat Ignition to manage smart contract deployments.
// Learn more about it at https://hardhat.org/ignition

const { buildModule } = require("@nomicfoundation/hardhat-ignition/modules");



module.exports = buildModule("OwnershipModule", (m) => {
  // const unlockTime = m.getParameter("unlockTime", JAN_1ST_2030);
  // const lockedAmount = m.getParameter("lockedAmount", ONE_GWEI);

  // const lock = m.contract("Ownership", [unlockTime], {
  //   value: lockedAmount,
  // });

  const ownership = m.contract("AuthChain", ["0xF2E7E2f51D7C9eEa9B0313C2eCa12f8e43bd1855"]);
  return { ownership };

});
