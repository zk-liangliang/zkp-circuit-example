use bellman_ce::{groth16::{Parameters, Proof},
         pairing::{bn256::Bn256,CurveAffine}
};

mod mimc;
mod sapling_sha265;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

pub fn proof_write(proof: &mut Proof<Bn256>, proof_encode:&mut Vec<u8>){
    proof_encode[0..32*2].copy_from_slice(proof.a.into_uncompressed().as_ref());
    proof_encode[32*2..32*6].copy_from_slice(proof.b.into_uncompressed().as_ref());
    proof_encode[32*6..32*8].copy_from_slice(proof.c.into_uncompressed().as_ref());

    println!("proof : {:?}", proof_encode);
    println!("proof_encode: {:?}", proof_encode.len());
}

pub fn vk_write(vk_encode:&mut Vec<u8>,params:&Parameters<Bn256>){
    vk_encode[0..32*4].copy_from_slice(params.vk.gamma_g2.into_uncompressed().as_ref());
    vk_encode[32*4..32*8].copy_from_slice(params.vk.delta_g2.into_uncompressed().as_ref());
    vk_encode[32*8..32*10].copy_from_slice(params.vk.alpha_g1.into_uncompressed().as_ref());
    vk_encode[32*10..32*14].copy_from_slice(params.vk.beta_g2.into_uncompressed().as_ref());
    println!("vk.ic : {:?}", vk_encode.len());
}