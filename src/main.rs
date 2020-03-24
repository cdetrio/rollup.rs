use std::str::FromStr;

use std::time::{Instant};

extern crate ewasm_api;

use ethereum_bn128::bn128_pairing;
use num_bigint::{BigUint, BigInt, Sign};

const SIZE_F: usize = 32;

type G1 = [u8; SIZE_F * 2];
type G2 = [u8; SIZE_F * 4];

fn negate(x: BigUint, y: BigUint) -> G1 {
    let mut res = [0u8; 64];
    let q = BigUint::from_str("21888242871839275222246405745257275088696311157297823662689037894645226208583").unwrap();
    let mut x_bytes = x.to_bytes_le();
    let mut y_bytes = (&q - (y % &q)).to_bytes_le();
    y_bytes.extend(x_bytes.iter().cloned());
    res.clone_from_slice(&y_bytes[0..64]);
    res.reverse();
    res
}


//#[no_mangle]
//pub extern "C" fn main() {

pub fn main() {
    let start_pairing = Instant::now();

    // proof_a 96 bytes
    // 089f41b0e239736338dbacf5893756a5a97ccbacb0f6ba326767b161018a803f26e20505b4f4a99859be674e5fc17025a6b81236302e6c21a59f95e0873b9fa4
    // 089f41b0e239736338dbacf5893756a5a97ccbacb0f6ba326767b161018a803f  26e20505b4f4a99859be674e5fc17025a6b81236302e6c21a59f95e0873b9fa4

    let proof_a = hex::decode("089f41b0e239736338dbacf5893756a5a97ccbacb0f6ba326767b161018a803f26e20505b4f4a99859be674e5fc17025a6b81236302e6c21a59f95e0873b9fa4").unwrap();

    let vk_a = hex::decode("167595c7e7cd0c935e3a275f254340f7c5a28f5edfa92963a1627e04398fe14c24963f8ac35ad1fa13d850fb61eb3c1d2766572452248b14c8e392591b14342b1a995764699581e0c41626103f9b9a675a503148f4d0b67cbaf1f7ef0b1cc41d25b7f1627599cac3ac91731ff8653662c70afe283da733cd885e12b2be54d313").unwrap();

    let proof_a_p_x = BigUint::from_str("14567039575197480528119959961327714497845927467213154926371421015062300663263").unwrap();
    let proof_a_p_y = BigUint::from_str("1000373253310235159656639300753059983014930924649970939079246556995097557245").unwrap();

    let neg_a_p = negate(proof_a_p_x, proof_a_p_y);
    //println!("negated is {}", hex::encode(&neg_a_p[..]));

    let p2 = hex::decode("198e9393920d483a7260bfb731fb5d25f1aa493335a9e71297e485b7aef312c21800deef121f1e76426a00665e5c4479674322d4f75edadd46debd5cd992f6ed090689d0585ff075ec9e99ad690c3395bc4b313370b38ef355acdadcd122975b12c85ea5db8c6deb4aab71808dcb408fe3d1e7690c43d37b4ce6cc0166fa7daa").unwrap();

    let mut output = [0u8; 32];

    // do this for two pairing ops
    let pairing_input = [&proof_a[..], &vk_a[..], &neg_a_p[..], &p2[..]].concat();

    //println!("doing pairing operation...");
    bn128_pairing(&pairing_input, &mut output).expect("pairing check failed");


    /* ** for scout build **/
    //let post_state_root = ewasm_api::types::Bytes32::from(output);
    //ewasm_api::eth2::save_post_state_root(&post_state_root);



    let pairing_duration = start_pairing.elapsed();
    println!("pairing check time: {:?}", pairing_duration);
}
