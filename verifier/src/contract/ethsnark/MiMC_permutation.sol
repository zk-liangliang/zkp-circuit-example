// Copyright (c) 2019 HarryR
// License: LGPL-3.0+

pragma solidity ^0.5.0;


/**
* Implements MiMC-p/p over the altBN scalar field used by zkSNARKs
*
* See: https://eprint.iacr.org/2016/492.pdf
*
* Round constants are generated in sequence from a seed
*/
library MiMC_permutation
{
    function ScalarField()
        internal pure returns( uint256 p )
    {
        return 0x30644e72e131a029b85045b68181585d2833e84879b9709143e1f593f0000001;  
    }

    function MiMCpe7( uint256 in_x, uint256 in_k )
        internal pure returns( uint256 out_x )
    {
        assembly {
            // Initialise round constants
            let c := mload(0x40)
            mstore(0x40, add(c, 32))
            mstore(c, 0xb6e489e6b37224a50bebfddbe7d89fa8fdcaa84304a70bd13f79b5d9f7951e9e)  // keccak_256('mimc')

            let localQ := 0x30644e72e131a029b85045b68181585d2833e84879b9709143e1f593f0000001
            let t
            let a

            for { let i := 91 } gt(i, 0) { i := sub(i, 1) } {
                mstore(c, keccak256(c, 32))
                t := addmod(addmod(in_x, mload(c), localQ), in_k, localQ)              // t = x + c_i + k
                a := mulmod(t, t, localQ)                                              // t^2
                in_x := mulmod(mulmod(a, mulmod(a, a, localQ), localQ), t, localQ)     // t^7
            }

            // Result adds key again to blind the result
            out_x := addmod(in_x, in_k, localQ)
        }
    }

    function MiMCpe5( uint256 in_x, uint256 in_k )
        internal pure returns( uint256 out_x )
    {
        assembly {
            // Initialise round constants
            let c := mload(0x40)
            mstore(0x40, add(c, 32))
            mstore(c, 0xb6e489e6b37224a50bebfddbe7d89fa8fdcaa84304a70bd13f79b5d9f7951e9e)  // keccak_256('mimc')

            let localQ := 0x30644e72e131a029b85045b68181585d2833e84879b9709143e1f593f0000001
            let t
            let a

            for { let i := 110 } gt(i, 0) { i := sub(i, 1) } {
                mstore(c, keccak256(c, 32))
                t := addmod(addmod(in_x, mload(c), localQ), in_k, localQ)  // t = x + c_i + k
                a := mulmod(t, t, localQ)                                  // t^2
                in_x := mulmod(mulmod(a, a, localQ), t, localQ)            // t^5
            }

            // Result adds key again to blind the result
            out_x := addmod(in_x, in_k, localQ)
        }
    }
}
