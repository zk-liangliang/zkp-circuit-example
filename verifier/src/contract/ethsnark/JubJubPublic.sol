// Copyright (c) 2018 @HarryR
// Copyright (c) 2018 @yondonfu
// License: LGPL-3.0+

pragma solidity ^0.5.0;

import "./JubJub.sol";

contract JubJubPublic
{
    function pointAdd(uint256[2] memory a, uint256[2] memory b)
        public view returns (uint256[2] memory)
    {
        return JubJub.pointAdd(a, b);
    }

    function scalarMult(uint256[2] memory a, uint256 s)
        public view returns (uint256, uint256)
    {
        return JubJub.scalarMult(a[0], a[1], s);
    }
}
