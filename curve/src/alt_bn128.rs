use bn::arith::U256;
use bn::{pairing_batch, AffineG1, AffineG2, Fq, Fq2, Group, Gt, G1, G2};
use std::io::{self, Read};
use super::Curve;

pub struct AltBn128;

impl Curve for AltBn128{
    const PRIME_FIELD: &'static str = "21888242871839275222246405745257275088696311157297823662689037894645226208583";
    const SCALAR_FIELD: &'static str = "21888242871839275222246405745257275088548364400416034343698204186575808495617";
    const FQ_BYTES_LENGTH: usize = 32;
    type Point = [u8;64];
    fn fq_bytes_length() ->usize{ 32 }

    // Can fail if any of the 2 points does not belong the bn128 curve
    fn point_add(input: &[u8]) -> Result<Self::Point, &'static str> {
        let mut padded_input = input.chain(io::repeat(0));
        let p1 = read_point(&mut padded_input)?;
        let p2 = read_point(&mut padded_input)?;

        let mut write_buf = [0u8; 64];
        if let Some(sum) = AffineG1::from_jacobian(p1 + p2) {
            // point not at infinity
            sum.x()
                .to_big_endian(&mut write_buf[0..32])
                .expect("Cannot success since 0..32 is 32-byte length");
            sum.y()
                .to_big_endian(&mut write_buf[32..64])
                .expect("Cannot success since 32..64 is 32-byte length");
        }

        Ok(write_buf)
    }

    // Can fail if first parameter (bn128 curve point) does not actually belong to the curve
    fn point_scalar_mul(input: &[u8]) -> Result<Self::Point, &'static str> {
        let mut padded_input = input.chain(io::repeat(0));
        let p = read_point(&mut padded_input)?;
        let fr = read_fr(&mut padded_input)?;

        let mut write_buf = [0u8; 64];
        if let Some(sum) = AffineG1::from_jacobian(p * fr) {
            // point not at infinity
            sum.x()
                .to_big_endian(&mut write_buf[0..32])
                .expect("Cannot fail since 0..32 is 32-byte length");
            sum.y()
                .to_big_endian(&mut write_buf[32..64])
                .expect("Cannot fail since 32..64 is 32-byte length");
        }
        Ok(write_buf)
    }

    fn point_pairing(input: &[u8]) -> Result<bool, &'static str> {
        if input.len() % 192 != 0 {
            return Err("Invalid input length, must be multiple of 192 (3 * (32*2))");
        }

        let ret_val = if input.is_empty() {
            U256::one()
        } else {
            // (a, b_a, b_b - each 64-byte affine coordinates)
            let elements = input.len() / 192;
            let mut vals = Vec::new();
            for idx in 0..elements {
                let a_x = Fq::from_slice(&input[idx * 192..idx * 192 + 32])
                    .map_err(|_| "Invalid a argument x coordinate")?;

                let a_y = Fq::from_slice(&input[idx * 192 + 32..idx * 192 + 64])
                    .map_err(|_| "Invalid a argument y coordinate")?;

                let b_a_y = Fq::from_slice(&input[idx * 192 + 64..idx * 192 + 96])
                    .map_err(|_| "Invalid b argument imaginary coeff x coordinate")?;

                let b_a_x = Fq::from_slice(&input[idx * 192 + 96..idx * 192 + 128])
                    .map_err(|_| "Invalid b argument imaginary coeff y coordinate")?;

                let b_b_y = Fq::from_slice(&input[idx * 192 + 128..idx * 192 + 160])
                    .map_err(|_| "Invalid b argument real coeff x coordinate")?;

                let b_b_x = Fq::from_slice(&input[idx * 192 + 160..idx * 192 + 192])
                    .map_err(|_| "Invalid b argument real coeff y coordinate")?;

                let b_a = Fq2::new(b_a_x, b_a_y);
                let b_b = Fq2::new(b_b_x, b_b_y);
                let b = if b_a.is_zero() && b_b.is_zero() {
                    G2::zero()
                } else {
                    G2::from(AffineG2::new(b_a, b_b).map_err(|_| "Invalid b argument - not on curve")?)
                };
                let a = if a_x.is_zero() && a_y.is_zero() {
                    G1::zero()
                } else {
                    G1::from(AffineG1::new(a_x, a_y).map_err(|_| "Invalid a argument - not on curve")?)
                };
                vals.push((a, b));
            }

            let mul = pairing_batch(&vals);

            if mul == Gt::one() {
                U256::one()
            } else {
                U256::zero()
            }
        };
        // assert_eq!(ret_val, U256::one());
        Ok(ret_val == U256::one())
    }
}

fn read_fr(reader: &mut io::Chain<&[u8], io::Repeat>) -> Result<bn::Fr, &'static str> {
    let mut buf = [0u8; 32];

    reader
        .read_exact(&mut buf[..])
        .expect("reading from zero-extended memory cannot fail; qed");
    bn::Fr::from_slice(&buf[0..32]).map_err(|_| "Invalid field element")
}

fn read_point(reader: &mut io::Chain<&[u8], io::Repeat>) -> Result<bn::G1, &'static str> {
    let mut buf = [0u8; 32];

    reader
        .read_exact(&mut buf[..])
        .expect("reading from zero-extended memory cannot fail; qed");
    let px = Fq::from_slice(&buf[0..32]).map_err(|_| "Invalid point x coordinate")?;

    reader
        .read_exact(&mut buf[..])
        .expect("reading from zero-extended memory cannot fail; qed");
    let py = Fq::from_slice(&buf[0..32]).map_err(|_| "Invalid point y coordinate")?;
    Ok(if px == Fq::zero() && py == Fq::zero() {
        G1::zero()
    } else {
        AffineG1::new(px, py)
            .map_err(|_| "Invalid curve point")?
            .into()
    })
}

#[test]
fn test_alt_bn128_add() {
    use hex_literal::hex;

    // zero-points additions
    {
        let input = hex!(
            "
				0000000000000000000000000000000000000000000000000000000000000000
				0000000000000000000000000000000000000000000000000000000000000000
				0000000000000000000000000000000000000000000000000000000000000000
				0000000000000000000000000000000000000000000000000000000000000000"
        );

        let expected = hex!(
            "
				0000000000000000000000000000000000000000000000000000000000000000
				0000000000000000000000000000000000000000000000000000000000000000"
        );

        assert_eq!(
            &expected[..],
            AltBn128::point_add(&input[..]).expect("Builtin should not fail")
        );
    }

    // no input, should not fail
    {
        let input = [0u8; 0];
        let expected = hex!(
            "
				0000000000000000000000000000000000000000000000000000000000000000
				0000000000000000000000000000000000000000000000000000000000000000"
        );

        assert_eq!(
            &expected[..],
            AltBn128::point_add(&input[..]).expect("Builtin should not fail")
        );
    }

    // should fail - point not on curve
    {
        let input = hex!(
            "
				1111111111111111111111111111111111111111111111111111111111111111
				1111111111111111111111111111111111111111111111111111111111111111
				1111111111111111111111111111111111111111111111111111111111111111
				1111111111111111111111111111111111111111111111111111111111111111"
        );

        let res = AltBn128::point_add(&input[..]);
        assert!(res.is_err(), "There should be built-in error here");
    }
}
#[test]
fn test_alt_bn128_mul() {
    use hex_literal::hex;
    // zero-point multiplication
    {
        let input = hex!(
            "
				0000000000000000000000000000000000000000000000000000000000000000
				0000000000000000000000000000000000000000000000000000000000000000
				0200000000000000000000000000000000000000000000000000000000000000"
        );

        let expected = hex!(
            "
				0000000000000000000000000000000000000000000000000000000000000000
				0000000000000000000000000000000000000000000000000000000000000000"
        );

        assert_eq!(
            &expected[..],
            AltBn128::point_scalar_mul(&input[..]).expect("Builtin should not fail")
        );
    }

    // should fail - point not on curve
    {
        let input = hex!(
            "
				1111111111111111111111111111111111111111111111111111111111111111
				1111111111111111111111111111111111111111111111111111111111111111
				0f00000000000000000000000000000000000000000000000000000000000000"
        );

        let res = AltBn128::point_scalar_mul(&input[..]);
        assert!(res.is_err(), "There should be built-in error here");
    }
}

#[test]
fn add() {
    use bn::{Fr, Group, G1};
    use hex_literal::hex;
    use rand::{SeedableRng, StdRng};

    let seed = [
        0, 0, 0, 0, 0, 0, 64, 13, // 103245
        0, 0, 0, 0, 0, 0, 176, 2, // 191922
        0, 0, 0, 0, 0, 0, 0, 13, // 1293
        0, 0, 0, 0, 0, 0, 96, 7u8, // 192103
    ];

    let p1 = G1::random(&mut StdRng::from_seed(seed));

    println!("p1:{:?}", p1);
    println!("p1 + p1:{:?}", p1 + p1);
    println!("p1 * 2: {:?}", p1 * Fr::from_str("2").unwrap());

    let p1_2times = AltBn128::point_scalar_mul(
        &hex!("0230644e72e131a029b85045b68181585d97816a916871ca8d3c208c16d87cfd46")[..],
    );
    println!("{:?}", p1_2times);
}
