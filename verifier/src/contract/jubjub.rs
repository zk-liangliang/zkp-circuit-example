use zkp_u256::{U256, Zero, One};
use once_cell::sync::Lazy;
use super::etec::{point_to_etec,etec_to_point,etec_add,etec_double};

pub(crate) static JUBJUB_A:u64 = 168700;
pub(crate) static JUBJUB_D:u64 = 168696;

static COFACTOR:u64 = 8;

pub(crate) static Q:Lazy<U256> = Lazy::new(||
    U256::from_decimal_str("21888242871839275222246405745257275088548364400416034343698204186575808495617").unwrap()
);
static L:Lazy<U256> = Lazy::new(||
    U256::from_decimal_str("21888242871839275222246405745257275088548364400416034343698204186575808495617").unwrap()
);

// fn w_naf5(x:U256,y:U256,w:[[U256;4];32]){
//     point_to_etec(x,y,Q,w[17])
// }

pub fn scalar_mult(x:U256, y:U256, mut value:U256) -> Option<(U256, U256)> {
    let mut p = point_to_etec(x, y, Q.clone());

    let mut a = [U256::zero(),U256::one(),U256::zero(),U256::one()];
    while !value.is_zero() {
        if !(&value & U256::one()).is_zero() {
            a = etec_add(&a,&p,&Q,&U256::from(JUBJUB_A),&U256::from(JUBJUB_D));
        }
        p = etec_double(&p,&Q,&U256::from(JUBJUB_A));
        value /= U256::from(2);
    }
    etec_to_point(a, Q.clone())
}

