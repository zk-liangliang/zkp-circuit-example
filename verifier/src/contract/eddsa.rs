use super::*;
use alloc::vec::Vec;
use once_cell::sync::Lazy;
use tiny_keccak::{Hasher, Sha3};
use zkp_u256::U256;

static GENERATE: Lazy<Vec<U256>> = Lazy::new(|| {
    let generator = [
        "17777552123799933955779906779655732241715742912184938656739573121738514868268",
        "2626589144620713026669568689430873010625803728049924121243784502389097019475",
    ];
    generator
        .iter()
        .map(|g| U256::from_decimal_str(g).unwrap())
        .collect::<Vec<U256>>()
});

pub fn hash_to_u256(data: &[u8]) -> U256 {
    let mut hashed = [0u8; 32];
    let mut sha256 = Sha3::v256();
    sha256.update(data);
    sha256.finalize(&mut hashed);

    let hash_u256 = U256::from_bytes_be(&hashed);
    // (2<<249) - 1
    let mask = U256::from_decimal_str(
        "1809251394333065553493296640760748560207343510400633813116524750123642650623",
    )
        .unwrap();

    // hash mod mask.
    return hash_u256 & mask;
}

pub fn verify(hashed_msg: U256, public_key: [U256; 2], r: [U256; 2], s: U256) -> bool {
    let mut input = Vec::with_capacity(32 * 5);
    input.extend_from_slice(&r[0].to_bytes_be());
    input.extend_from_slice(&r[1].to_bytes_be());
    input.extend_from_slice(&public_key[0].to_bytes_be());
    input.extend_from_slice(&public_key[1].to_bytes_be());
    input.extend_from_slice(&hashed_msg.to_bytes_be());
    println!("**************************************************************************************************************************");
    println!("s*G:");
    if let Some(lhs) = dbg!(scalar_mult(GENERATE[0].clone(), GENERATE[1].clone(), s)) {
        let t = hash_to_u256(&input[..]);
        println!("hash*PK:");
        if let Some((pk_x, pk_y)) = dbg!(scalar_mult(public_key[0].clone(), public_key[1].clone(), t)) {
            let [r_x, r_y] = r;
            let etec_point = etec_add(
                &point_to_etec(r_x, r_y, Q.clone()),
                &point_to_etec(pk_x, pk_y, Q.clone()),
                &*Q,
                &JUBJUB_A.into(),
                &JUBJUB_D.into(),
            );
            println!("R+hash*Pk");
            if let Some(rhs) = dbg!(etec_to_point(etec_point, Q.clone())) {
                return lhs == rhs;
            }
        }
    }
    false
}

#[test]
fn test_eddsa_verify() {
    let message = hash_to_u256(b"mimc");
    let sk = U256::from(46565516551654u128);
    let (pk_x,pk_y) = scalar_mult(GENERATE[0].clone(), GENERATE[1].clone(), sk.clone()).unwrap();

    // According to sk and message, generate a deterministic random number.
    let mut deterministic_random_number = Vec::new();
    deterministic_random_number.extend_from_slice(&message.to_bytes_be());
    deterministic_random_number.extend_from_slice(&sk.to_bytes_be());
    let r = hash_to_u256(&deterministic_random_number[..]);
    let (r_x,r_y) = scalar_mult(GENERATE[0].clone(), GENERATE[1].clone(), r.clone()).unwrap();

    let mut input = Vec::with_capacity(32*5);
    input.extend_from_slice(&r_x.to_bytes_be());
    input.extend_from_slice(&r_y.to_bytes_be());
    input.extend_from_slice(&pk_x.to_bytes_be());
    input.extend_from_slice(&pk_y.to_bytes_be());
    input.extend_from_slice(&message.to_bytes_be());

    let hash = hash_to_u256(&input[..]);
    println!("hash*Pk:");
    dbg!(scalar_mult(pk_x.clone(), pk_y.clone(), hash.clone()).unwrap());
    println!("sk*(hash*G):");
    let (h_x, h_y) = scalar_mult(GENERATE[0].clone(), GENERATE[1].clone(), hash.clone()).unwrap();
    dbg!(scalar_mult(h_x.clone(),h_y.clone(),sk.clone()).unwrap());
    // s = r + sk * hash
    let s = (&r + sk.mulmod(&hash,&Q)) % &*Q;

    println!("(sk*hash)*G:");
    dbg!(scalar_mult(GENERATE[0].clone(), GENERATE[1].clone(), sk.mulmod(&hash,&Q)).unwrap());
    // println!("s*G:");
    // dbg!(scalar_mult(GENERATE[0].clone(), GENERATE[1].clone(), (&r + sk.mulmod(&hash,&Q)) % &*Q).unwrap());
    assert!(verify(message, [pk_x,pk_y],[r_x, r_y], s));
}
