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
        ProofDecoder.FullOutcomeProofWithBlockRoot memory fullOutcomeProof = borsh.decodeFullOutcomeProofWithBlockRoot();
        borsh.done();        
        
        // Step 1: Verify the outcome proof
        bytes32 expectedOutcomeRoot = _computeRoot(
            sha256(
                abi.encodePacked(
                    _computeRoot(
                        fullOutcomeProof.outcome_proof.outcome_with_id.hash,
                        fullOutcomeProof.outcome_proof.proof
                    )
                )
            ),
            fullOutcomeProof.outcome_root_proof
        );

        require(
            expectedOutcomeRoot == fullOutcomeProof.block_header_lite.inner_lite.outcome_root,
            "NearProver: outcome merkle proof is not valid"
        );

        // Step 2. Verify the block merkle root
        bytes32 expectedBlockMerkleRoot = fullOutcomeProof.head_merkle_root;
        bytes32 lcHeader = nearX.latestHeader();
        bytes32 computedBlockMerkleRoot = _computeRoot(lcHeader, fullOutcomeProof.block_proof);
        console.log("expectedBlockMerkleRoot");
        console.logBytes32(expectedBlockMerkleRoot);
        console.log("computedBlockMerkleRoot");
        console.logBytes32(computedBlockMerkleRoot);
        require(computedBlockMerkleRoot == expectedBlockMerkleRoot,
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
