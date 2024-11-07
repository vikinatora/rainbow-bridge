const { expect } = require('chai');
const { ethers } = require('hardhat');
const fs = require('fs').promises;
const bs58 = require('bs58');
const Web3 = require('web3');

let NearProver, NearXMock;

beforeEach(async function () {
    NearXMock = await (await ethers.getContractFactory('NearXMock')).deploy();
    NearProver = await (await ethers.getContractFactory('NearProver')).deploy(
        NearXMock.address,
        ethers.constants.AddressZero,
        0
    );
});

async function testProof(proofPath, latestHeader) {
    let proof = await fs.readFile(proofPath);
    let blockHash = ethers.utils.hexlify(bs58.decode(latestHeader));
    // Set the latest header to match the proof's block hash
    await NearXMock.setLatestHeader(blockHash);
    expect(await NearProver.proveOutcome(proof)).to.be.true;
}

it('should be ok', async function () {
    await testProof('./test/inclusion_proof1', "4Fv8JTaenUE9C2czwCCb4F4qyJNpsPHbZeyt74evn3mv");
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
