#![allow(unused_imports)]
#![allow(unused_variables)]
extern crate bellman;
#[macro_use]
extern crate pairing;
extern crate rand;
extern crate ff;
extern crate group;
extern crate curve;

use bellman::{ Circuit, ConstraintSystem, SynthesisError,
    groth16::{
      create_random_proof, generate_random_parameters,
      prepare_verifying_key, Proof, Parameters ,
      verify_proof as raw_verify_proof,
    }
};
use pairing::{Engine,bls12_381::{Bls12, Fr}};
use ff::{SqrtField,PrimeField, Field,PrimeFieldRepr};
use group::CurveAffine;
use curve::{verify_proof,bls12_381::{Bls381}};
use std::io::{self,Write};
use rand::thread_rng;

mod cube;
mod multiply;

fn main(){

    println!("Prove that I know x such that x^3 + x + 5 == 35.");
    
    let rng = &mut thread_rng();
    
    println!("Creating parameters...");
    
    // Create parameters for our circuit
    let mut params = {
        let c = cube::CubeDemo::<Bls12> {
            x: None
        };

        generate_random_parameters(c, rng).unwrap()
    };
    
    // Prepare the verification key (for proof verification)
    let vk = prepare_verifying_key(&params.vk);

    println!("Creating proofs...");
    
    // Create an instance of circuit
    let c = cube::CubeDemo::<Bls12> {
        x: Fr::from_str("3")
    };
    
    // Create a groth16 proof with our parameters.
    let proof = create_random_proof(c, &params, rng).unwrap();
    // // proof encode
    // let mut proof_encode = vec![0u8;384];
    // proof_write(&proof, &mut proof_encode);
    // // vk encode
    // let mut vk_encode = vec![0u8;672];
    // vk_write(&mut vk_encode, &params);
    // // vk_ic encode
    // let vk_not_prepared = params.vk.ic.iter().map(|ic| ic.into_uncompressed().as_ref().to_vec()).collect::<Vec<_>>();
    // let vk_ic = vk_not_prepared.iter().map(|ic| &ic[..]).collect::<Vec<_>>();
    // // inputs
    // let mut input = vec![0u8; 32];
    // Fr::from_str("35").unwrap().into_repr().write_le(&mut input[..]);
    // println!("input:{:?}",input);
    // println!("input:{}",input.len());

    //
    // assert!(verify_proof::<Bls381>(
    //     &*vk_ic,
    //     &*vk_encode,
    //     &*proof_encode,
    //     &[&input])
    //     .expect("verify_proof fail"));

    assert!(raw_verify_proof(
        &vk,
        &proof,
        &[Fr::from_str("35").unwrap()]
    ).unwrap());
    println!("Verify proofs successfully");
}