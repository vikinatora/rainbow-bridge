// SPDX-License-Identifier: GPL-3.0-or-later
pragma solidity ^0.8;

import "rainbow-bridge-sol/nearbridge/contracts/AdminControlled.sol";
import "rainbow-bridge-sol/nearbridge/contracts/INearX.sol";
import "rainbow-bridge-sol/nearbridge/contracts/NearDecoder.sol";
import "./ProofDecoder.sol";
import "./INearProver.sol";
import "hardhat/console.sol";

contract NearProver is INearProver, AdminControlled {
    using Borsh for Borsh.Data;
    using NearDecoder for Borsh.Data;
    using ProofDecoder for Borsh.Data;

    INearX public nearX;

    constructor(
        address _nearX,
        address _admin,
        uint _pausedFlags
    ) AdminControlled(_admin, _pausedFlags) {
        nearX = INearX(_nearX);
    }

    uint constant UNPAUSE_ALL = 0;
    uint constant PAUSED_VERIFY = 1;

    function proveOutcome(bytes memory proofData) public view returns (bool) {
        Borsh.Data memory borsh = Borsh.from(proofData);
        ProofDecoder.FullOutcomeProof memory fullOutcomeProof = borsh.decodeFullOutcomeProof();
        borsh.done();        

        // Step 1. Verify the block header hash against the latestHeader
        // The header hash is precalculated inside block_header_lite.hash,
        // so we can just use it instead of recalculating.
        require(fullOutcomeProof.block_header_lite.hash == nearX.latestHeader(), "NearProver: block header does not match latest header");

        // Step 2: Verify the outcome proof within the block (unchanged)
        bytes32 hash = _computeRoot(
            fullOutcomeProof.outcome_proof.outcome_with_id.hash,
            fullOutcomeProof.outcome_proof.proof
        );

        hash = sha256(abi.encodePacked(hash));

        hash = _computeRoot(hash, fullOutcomeProof.outcome_root_proof);

        require(
            hash == fullOutcomeProof.block_header_lite.inner_lite.outcome_root,
            "NearProver: outcome merkle proof is not valid"
        );

        // Step 3. Verify the block merkle root
        // bytes32 expectedBlockMerkleRoot = bridge.blockMerkleRoots(blockHeight);

        // We have verified the block header, so we can use the block merkle root 
        // from the light client header instead of fetching it from the contract.
        bytes32 expectedBlockMerkleRoot = fullOutcomeProof.block_header_lite.inner_lite.block_merkle_root;
        
        require(
            _computeRoot(fullOutcomeProof.block_header_lite.hash, fullOutcomeProof.block_proof) ==
                expectedBlockMerkleRoot,
            "NearProver: block proof is not valid"
        );

        return true;
    }

    function _computeRoot(bytes32 node, ProofDecoder.MerklePath memory proof) internal pure returns (bytes32 hash) {
        hash = node;
        for (uint i = 0; i < proof.items.length; i++) {
            ProofDecoder.MerklePathItem memory item = proof.items[i];
            if (item.direction == 0) {
                hash = sha256(abi.encodePacked(item.hash, hash));
            } else {
                hash = sha256(abi.encodePacked(hash, item.hash));
            }
        }
    }
}
