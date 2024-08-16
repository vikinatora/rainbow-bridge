// SPDX-License-Identifier: MIT
pragma solidity >=0.8.0;

interface INearX {
    function latestHeader() external view returns (bytes32);
}