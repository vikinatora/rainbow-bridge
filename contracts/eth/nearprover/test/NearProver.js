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

async function testProof(proofPath) {
    let proof = require(proofPath);
    let blockHash = ethers.utils.hexlify(bs58.decode(proof.latestHeader));
    // Set the latest header to match the proof's block hash
    await NearXMock.setLatestHeader(blockHash);
    // Borshify the outcome proof
    proof = borshifyOutcomeProofWithBlockRootAndLcHeader(proof);

    expect(await NearProver.proveOutcome(proof)).to.be.true;
}

function borshifyOutcomeProofWithBlockRootAndLcHeader (proof) {
    const statusToBuffer = (status) => {
      if ('SuccessValue' in status) {
        const data = Buffer.from(status.SuccessValue, 'base64')
        return Buffer.concat([
          Buffer.from([2]),
          Web3.utils.toBN(data.length).toBuffer('le', 4),
          data
        ])
      } else if ('SuccessReceiptId' in status) {
        return Buffer.concat([
          Buffer.from([3]),
          bs58.decode(status.SuccessReceiptId)
        ])
      } else {
        throw new Error('status not supported')
      }
    }
    return Buffer.concat([
      bs58.decode(proof.latestHeader),
      bs58.decode(proof.headMerkleRoot),
      Web3.utils.toBN(proof.outcome_proof.proof.length).toBuffer('le', 4),
      Buffer.concat(
        proof.outcome_proof.proof.map((p) =>
          Buffer.concat([
            bs58.decode(p.hash),
            Buffer.from([p.direction === 'Right' ? 1 : 0])
          ])
        )
      ),
  
      bs58.decode(proof.outcome_proof.block_hash),
  
      bs58.decode(proof.outcome_proof.id),
  
      Buffer.concat([
        Web3.utils
          .toBN(proof.outcome_proof.outcome.logs.length)
          .toBuffer('le', 4),
  
        Web3.utils
          .toBN(proof.outcome_proof.outcome.receipt_ids.length)
          .toBuffer('le', 4),
        Buffer.concat(
          proof.outcome_proof.outcome.receipt_ids.map((r) => bs58.decode(r))
        ),
  
        Web3.utils.toBN(proof.outcome_proof.outcome.gas_burnt).toBuffer('le', 8),
        Web3.utils
          .toBN(proof.outcome_proof.outcome.tokens_burnt)
          .toBuffer('le', 16),
        Web3.utils
          .toBN(proof.outcome_proof.outcome.executor_id.length)
          .toBuffer('le', 4),
        Buffer.from(proof.outcome_proof.outcome.executor_id, 'utf8'),
  
        statusToBuffer(proof.outcome_proof.outcome.status),
  
        Web3.utils.toBN(proof.outcome_root_proof.length).toBuffer('le', 4),
        Buffer.concat(
          proof.outcome_root_proof.map((orp) =>
            Buffer.concat([
              bs58.decode(orp.hash),
              Buffer.from([orp.direction === 'Right' ? 1 : 0])
            ])
          )
        ),
  
        bs58.decode(proof.block_header_lite.prev_block_hash),
        bs58.decode(proof.block_header_lite.inner_rest_hash),
        Web3.utils
          .toBN(proof.block_header_lite.inner_lite.height)
          .toBuffer('le', 8),
        bs58.decode(proof.block_header_lite.inner_lite.epoch_id),
        bs58.decode(proof.block_header_lite.inner_lite.next_epoch_id),
        bs58.decode(proof.block_header_lite.inner_lite.prev_state_root),
        bs58.decode(proof.block_header_lite.inner_lite.outcome_root),
        // for backward compatible in tests with old dumps
        Web3.utils
          .toBN(
            proof.block_header_lite.inner_lite.timestamp_nanosec ||
              proof.block_header_lite.inner_lite.timestamp
          )
          .toBuffer('le', 8),
        bs58.decode(proof.block_header_lite.inner_lite.next_bp_hash),
        bs58.decode(proof.block_header_lite.inner_lite.block_merkle_root),
  
        Web3.utils.toBN(proof.block_proof.length).toBuffer('le', 4),
        Buffer.concat(
          proof.block_proof.map((bp) =>
            Buffer.concat([
              bs58.decode(bp.hash),
              Buffer.from([bp.direction === 'Right' ? 1 : 0])
            ])
          )
        )
      ])
    ])
  }
  


it('should be ok', async function () {
    await testProof('./proof2.json');
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
