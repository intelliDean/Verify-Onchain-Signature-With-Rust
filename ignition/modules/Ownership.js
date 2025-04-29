// This setup uses Hardhat Ignition to manage smart contract deployments.
// Learn more about it at https://hardhat.org/ignition

const { buildModule } = require("@nomicfoundation/hardhat-ignition/modules");



module.exports = buildModule("OwnershipModule", (m) => {
  // const unlockTime = m.getParameter("unlockTime", JAN_1ST_2030);
  // const lockedAmount = m.getParameter("lockedAmount", ONE_GWEI);

  // const lock = m.contract("Ownership", [unlockTime], {
  //   value: lockedAmount,
  // });

  const ownership = m.contract("SignatureVerifier");
  return { ownership };

});
