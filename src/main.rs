use std::str::FromStr;

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

fn roll_up_pairing() {
  let preP = ["7d606bcc64db4cb18b49fcbd7b888e3a00d67cdf3d222c1318ca756ac2d5e1e4", "b7af417ad896dca6be6da26987088fda2852fa8dbf599d930f6f8e037bae8416"];
  let G2s= [["894a255738a19f999d6bdf64724aeafc223ff986ff957e35ee4f23683f8fecc8", "84572ecc2e61fad9684b6b7eb77ee83b2c3a941876fbbc0240f6f628a2cdb538"], ["d1a12342db967b7e551e819546d2629205d0d7ba0d694f414cfe2bf6179db7f3", 
  "30c3d0682c0665cc69e448cdf84976790c2e49ee650b607591c18c2ecbca7603"]];

//7d606bcc64db4cb18b49fcbd7b888e3a00d67cdf3d222c1318ca756ac2d5e1e4b7af417ad896dca6be6da26987088fda2852fa8dbf599d930f6f8e037bae8416894a255738a19f999d6bdf64724aeafc223ff986ff957e35ee4f23683f8fecc884572ecc2e61fad9684b6b7eb77ee83b2c3a941876fbbc0240f6f628a2cdb538d1a12342db967b7e551e819546d2629205d0d7ba0d694f414cfe2bf6179db7f330c3d0682c0665cc69e448cdf84976790c2e49ee650b607591c18c2ecbca7603

}

fn main() {
    let proof_a = hex::decode("089f41b0e239736338dbacf5893756a5a97ccbacb0f6ba326767b161018a803f26e20505b4f4a99859be674e5fc17025a6b81236302e6c21a59f95e0873b9fa4").unwrap();

    let vk_a = hex::decode("167595c7e7cd0c935e3a275f254340f7c5a28f5edfa92963a1627e04398fe14c24963f8ac35ad1fa13d850fb61eb3c1d2766572452248b14c8e392591b14342b1a995764699581e0c41626103f9b9a675a503148f4d0b67cbaf1f7ef0b1cc41d25b7f1627599cac3ac91731ff8653662c70afe283da733cd885e12b2be54d313").unwrap();

    let proof_a_p_x = BigUint::from_str("14567039575197480528119959961327714497845927467213154926371421015062300663263").unwrap();
    let proof_a_p_y = BigUint::from_str("1000373253310235159656639300753059983014930924649970939079246556995097557245").unwrap();

    let neg_a_p = negate(proof_a_p_x, proof_a_p_y);
    println!("negated is {}", hex::encode(&neg_a_p[..]));

    let p2 = hex::decode("198e9393920d483a7260bfb731fb5d25f1aa493335a9e71297e485b7aef312c21800deef121f1e76426a00665e5c4479674322d4f75edadd46debd5cd992f6ed090689d0585ff075ec9e99ad690c3395bc4b313370b38ef355acdadcd122975b12c85ea5db8c6deb4aab71808dcb408fe3d1e7690c43d37b4ce6cc0166fa7daa").unwrap();

    let mut output = [0u8; 32];
    let pairing_input = [&proof_a[..], &vk_a[..], &neg_a_p[..], &p2[..]].concat();

    bn128_pairing(&pairing_input, &mut output).expect("pairing check failed");
}
