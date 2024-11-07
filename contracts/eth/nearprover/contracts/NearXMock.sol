// NearXMock.sol
pragma solidity ^0.8.0;

contract NearXMock {
    bytes32 public latestHeader;

    function setLatestHeader(bytes32 _header) external {
        latestHeader = _header;
    }
}

