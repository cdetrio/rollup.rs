use std::str::FromStr;

//use std::time::{Instant};

//extern crate wee_alloc;
//#[global_allocator]
//static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

extern crate ewasm_api;

use ethereum_bn128::bn128_pairing;
use num_bigint::{BigUint, BigInt, Sign};

const SIZE_F: usize = 32;

type G1 = [u8; SIZE_F * 2];
type G2 = [u8; SIZE_F * 4];

fn negate(x_bytes_raw: &[u8], y_bytes_raw: &[u8]) -> G1 {
    let mut res = [0u8; 64];
    let q = BigUint::from_str("21888242871839275222246405745257275088696311157297823662689037894645226208583").unwrap();
    let y = BigUint::from_bytes_be(y_bytes_raw);
    let x = BigUint::from_bytes_be(x_bytes_raw);
    let mut x_bytes = x.to_bytes_le();
    let mut y_bytes = (&q - (y % &q)).to_bytes_le();
    y_bytes.extend(x_bytes.iter().cloned());
    res.clone_from_slice(&y_bytes[0..64]);
    res.reverse();
    res
}


#[no_mangle]
pub extern "C" fn main() {
    //let start_pairing = Instant::now();

    // proof_a 96 bytes
    // 089f41b0e239736338dbacf5893756a5a97ccbacb0f6ba326767b161018a803f26e20505b4f4a99859be674e5fc17025a6b81236302e6c21a59f95e0873b9fa4
    // 089f41b0e239736338dbacf5893756a5a97ccbacb0f6ba326767b161018a803f  26e20505b4f4a99859be674e5fc17025a6b81236302e6c21a59f95e0873b9fa4

    let block_data = ewasm_api::eth2::acquire_block_data();

    //let proof_a = hex::decode("089f41b0e239736338dbacf5893756a5a97ccbacb0f6ba326767b161018a803f26e20505b4f4a99859be674e5fc17025a6b81236302e6c21a59f95e0873b9fa4").unwrap();
    let proof_a = &block_data[0 .. 64];

    //let mut output_proof_a = [0u8; 32];
    //output_proof_a.copy_from_slice(&proof_a[0 .. 32]);
    //let post_state_root_proof_a = ewasm_api::types::Bytes32::from(output_proof_a);
    //ewasm_api::eth2::save_post_state_root(&post_state_root_proof_a);

    //let vk_a = hex::decode("167595c7e7cd0c935e3a275f254340f7c5a28f5edfa92963a1627e04398fe14c24963f8ac35ad1fa13d850fb61eb3c1d2766572452248b14c8e392591b14342b1a995764699581e0c41626103f9b9a675a503148f4d0b67cbaf1f7ef0b1cc41d25b7f1627599cac3ac91731ff8653662c70afe283da733cd885e12b2be54d313").unwrap();
    let vk_a = &block_data[64 .. 192];

    // 0x2034a6f7e573a3b1d2c16934721c754bc50fc8c232a4e83c8a7dfa8770311ddf == 14567039575197480528119959961327714497845927467213154926371421015062300663263
    //let proof_a_p_x = BigUint::from_str("14567039575197480528119959961327714497845927467213154926371421015062300663263").unwrap();
    let proof_a_p_x = &block_data[192 .. 224];

    // 0x023630f23dda7cbc29e510ee9b92e16e16277736a32b01c670bbc77f4c6978fd == 1000373253310235159656639300753059983014930924649970939079246556995097557245
    //let proof_a_p_y = BigUint::from_str("1000373253310235159656639300753059983014930924649970939079246556995097557245").unwrap();
    let proof_a_p_y = &block_data[224 .. 256];

    let neg_a_p = negate(proof_a_p_x, proof_a_p_y);
    //println!("negated is {}", hex::encode(&neg_a_p[..]));
    /*
    let mut output_negap = [0u8; 32];
    output_negap.copy_from_slice(&neg_a_p[0 .. 32]);
    let post_state_root_negap = ewasm_api::types::Bytes32::from(output_negap);
    ewasm_api::eth2::save_post_state_root(&post_state_root_negap);
    */



    //let p2 = hex::decode("198e9393920d483a7260bfb731fb5d25f1aa493335a9e71297e485b7aef312c21800deef121f1e76426a00665e5c4479674322d4f75edadd46debd5cd992f6ed090689d0585ff075ec9e99ad690c3395bc4b313370b38ef355acdadcd122975b12c85ea5db8c6deb4aab71808dcb408fe3d1e7690c43d37b4ce6cc0166fa7daa").unwrap();
    let p2 = &block_data[256 .. 384];
    /*
    let mut output_p2 = [0u8; 32];
    output_p2.copy_from_slice(&p2[96 .. 128]);
    let post_state_root_p2 = ewasm_api::types::Bytes32::from(output_p2);
    ewasm_api::eth2::save_post_state_root(&post_state_root_p2);
    */




    let mut output = [0u8; 32];


    /* ** in block data format ** */
    // proof_a 089f41b0e239736338dbacf5893756a5a97ccbacb0f6ba326767b161018a803f26e20505b4f4a99859be674e5fc17025a6b81236302e6c21a59f95e0873b9fa4
    // vk_a 167595c7e7cd0c935e3a275f254340f7c5a28f5edfa92963a1627e04398fe14c24963f8ac35ad1fa13d850fb61eb3c1d2766572452248b14c8e392591b14342b1a995764699581e0c41626103f9b9a675a503148f4d0b67cbaf1f7ef0b1cc41d25b7f1627599cac3ac91731ff8653662c70afe283da733cd885e12b2be54d313
    // proof_a_p_x 2034a6f7e573a3b1d2c16934721c754bc50fc8c232a4e83c8a7dfa8770311ddf
    // proof_a_p_y 023630f23dda7cbc29e510ee9b92e16e16277736a32b01c670bbc77f4c6978fd
    // p2 198e9393920d483a7260bfb731fb5d25f1aa493335a9e71297e485b7aef312c21800deef121f1e76426a00665e5c4479674322d4f75edadd46debd5cd992f6ed090689d0585ff075ec9e99ad690c3395bc4b313370b38ef355acdadcd122975b12c85ea5db8c6deb4aab71808dcb408fe3d1e7690c43d37b4ce6cc0166fa7daa


    // scout bn128pairing test case: 3f808a0161b1676732baf6b0accb7ca9a5563789f5acdb38637339e2b0419f08a49f3b87e0959fa5216c2e303612b8a62570c15f4e67be5998a9f4b40505e22601000000000000000000000000000000000000000000000000000000000000002b34141b5992e3c8148b2452245766271d3ceb61fb50d813fad15ac38a3f96244ce18f39047e62a16329a9df5e8fa2c5f74043255f273a5e930ccde7c795751613d354beb2125e88cd33a73d28fe0ac7623665f81f7391acc3ca997562f1b7251dc41c0beff7f1ba7cb6d0f44831505a679a9b3f102616c4e08195696457991a01000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000df1d317087fa7d8a3ce8a432c2c80fc54b751c723469c1d2b1a373e5f7a63420fd78694c7fc7bb70c6012ba3367727166ee1929bee10e529bc7cda3df23036020100000000000000000000000000000000000000000000000000000000000000edf692d95cbdde46ddda5ef7d422436779445c5e66006a42761e1f12efde0018c212f3aeb785e49712e7a9353349aaf1255dfb31b7bf60723a480d9293938e19aa7dfa6601cce64c7bd3430c69e7d1e38f40cb8d8071ab4aeb6d8cdba55ec8125b9722d1dcdaac55f38eb37033314bbc95330c69ad999eec75f05f58d089060901000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000


    // rollup_rs scout.yaml block data (same as bn128pairing except in Big Endian and points don't have the z coordinate): 089f41b0e239736338dbacf5893756a5a97ccbacb0f6ba326767b161018a803f26e20505b4f4a99859be674e5fc17025a6b81236302e6c21a59f95e0873b9fa4167595c7e7cd0c935e3a275f254340f7c5a28f5edfa92963a1627e04398fe14c24963f8ac35ad1fa13d850fb61eb3c1d2766572452248b14c8e392591b14342b1a995764699581e0c41626103f9b9a675a503148f4d0b67cbaf1f7ef0b1cc41d25b7f1627599cac3ac91731ff8653662c70afe283da733cd885e12b2be54d3132034a6f7e573a3b1d2c16934721c754bc50fc8c232a4e83c8a7dfa8770311ddf023630f23dda7cbc29e510ee9b92e16e16277736a32b01c670bbc77f4c6978fd198e9393920d483a7260bfb731fb5d25f1aa493335a9e71297e485b7aef312c21800deef121f1e76426a00665e5c4479674322d4f75edadd46debd5cd992f6ed090689d0585ff075ec9e99ad690c3395bc4b313370b38ef355acdadcd122975b12c85ea5db8c6deb4aab71808dcb408fe3d1e7690c43d37b4ce6cc0166fa7daa



    // do this for two pairing ops
    let pairing_input = [&proof_a[..], &vk_a[..], &neg_a_p[..], &p2[..]].concat();

    //println!("doing pairing operation...");
    bn128_pairing(&pairing_input, &mut output).expect("pairing check failed");


    /* ** for scout build **/
    let post_state_root = ewasm_api::types::Bytes32::from(output);
    ewasm_api::eth2::save_post_state_root(&post_state_root);


    // for native benchmarking
    //let pairing_duration = start_pairing.elapsed();
    //println!("pairing check time: {:?}", pairing_duration);
}
