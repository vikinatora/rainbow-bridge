const { expect } = require('chai');
const { ethers } = require('hardhat');
const { borshifyOutcomeProof } = require(`rainbow-bridge-utils`);
const fs = require('fs').promises;
const { computeMerkleRoot } = require('../utils/utils');
const bs58 = require('bs58');

let NearProver, NearXMock;

beforeEach(async function () {
    NearXMock = await (await ethers.getContractFactory('NearXMock')).deploy();
    NearProver = await (await ethers.getContractFactory('NearProver')).deploy(
        NearXMock.address,
        ethers.constants.AddressZero,
        0
    );
});

async function testProof(proofPath) {
    let proof = require(proofPath);
    let blockHash = ethers.utils.hexlify(bs58.decode(proof.outcome_proof.block_hash));
    // Set the latest header to match the proof's block hash
    await NearXMock.setLatestHeader(blockHash);
    // Borshify the outcome proof
    proof = borshifyOutcomeProof(proof);

    expect(await NearProver.proveOutcome(proof)).to.be.true;
}


it('should be ok', async function () {
    await testProof('./proof2.json');
    await testProof('./proof3.json');
    await testProof('./proof4.json');
    await testProof('./proof5.json');
    await testProof('./proof6.json');
    await testProof('./proof7.json');
});

if (process.env['NEAR_PROOFS_DIR']) {
    it('should be able to verify proofs from dump', async function () {
        this.timeout(0);
        let proofFiles = await fs.readdir(process.env['NEAR_PROOFS_DIR']);

        for (let i = 0; i < proofFiles.length; i++) {
            let proofPath = process.env['NEAR_PROOFS_DIR'] + '/' + proofFiles[i];
            await testProof(proofPath);
            console.log('proved proof ' + proofFiles[i]);
        }
    });
}
