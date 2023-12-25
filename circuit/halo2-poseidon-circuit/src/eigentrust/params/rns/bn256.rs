// Rns
// bit_len_lookup: 17,
// wrong_modulus:
// 21888242871839275222246405745257275088696311157297823662689037894645226208583,
// native_modulus:
// 21888242871839275222246405745257275088548364400416034343698204186575808495617,
// binary_modulus:
// 7588550360256754183279148073529370729071901715047420004889892225542594864082845696,
// crt_modulus:
// 1661000333304832637718917699744950972288079041304113932603045769717696232214372508635029\
/// 51190734612532350192541290114096968998888229427253981337422670103314432,
// right_shifters: [
//     0x0000000000000000000000000000000000000000000000000000000000000001,
//     0x0b603a5609b3f6f81dbc9c192fc7933ab42e346981868e480f8e4610fb396ee5,
//     0x1b7c016fe8acfaed1a908db2cea9b991a31a140f219532a9568bea8e0766f9dd,
//     0x0523513296c10199338287b1e0bedd9955a33201cd88df51769b0bf04e2f27cc,
// ],
// left_shifters: [
//     0x0000000000000000000000000000000000000000000000000000000000000001,
//     0x0000000000000000000000000000000000000000000000100000000000000000,
//     0x0000000000000000000000000000010000000000000000000000000000000000,
//     0x0000000000001000000000000000000000000000000000000000000000000000,
// ],
// base_aux: [
//     488280579659007654542,
//     510955945554286098768,
//     301160387202582250159,
//     1702635872462387,
// ],
// negative_wrong_modulus_decomposed: [
//     0x000000000000000000000000000000000000000000000002c3df73e9278302b9,
//     0x00000000000000000000000000000000000000000000000a2687e956e978e357,
//     0x00000000000000000000000000000000000000000000000fd647afba497e7ea7,
//     0x00000000000000000000000000000000000000000000000ffffcf9bb18d1ece5,
// ],
// wrong_modulus_decomposed: [
//     0x00000000000000000000000000000000000000000000000d3c208c16d87cfd47,
//     0x000000000000000000000000000000000000000000000005d97816a916871ca8,
//     0x00000000000000000000000000000000000000000000000029b85045b6818158,
//     0x00000000000000000000000000000000000000000000000000030644e72e131a,
// ],
// wrong_modulus_minus_one: [
//     0x00000000000000000000000000000000000000000000000d3c208c16d87cfd46,
//     0x000000000000000000000000000000000000000000000005d97816a916871ca8,
//     0x00000000000000000000000000000000000000000000000029b85045b6818158,
//     0x00000000000000000000000000000000000000000000000000030644e72e131a,
// ],
// wrong_modulus_in_native_modulus:
// 0x000000000000000000000000000000006f4d8248eeb859fbf83e9682e87cfd46,
// max_reduced_limb: 295147905179352825855,
// max_unreduced_limb: 5070602400912917605986812821503,
// max_remainder:
// 28948022309329048855892746252171976963317496166410141009864396001978282409983,
// max_operand:
// 7410693711188236507108543040556026102609279018600996098525285376506440296955903,
// max_mul_quotient:
// 3794275180128377091639574036764685364535950857523710002444946112771297432041422847,
// max_most_significant_reduced_limb: 1125899906842623,
// max_most_significant_operand_limb: 288230376151711743,
// max_most_significant_mul_quotient_limb: 147573952589676412927,
// mul_v_bit_len: 71,
// red_v_bit_len: 69,
use super::*;
use halo2_proofs::halo2curves::bn256::{Fq, Fr};

/// Structure for the Bn256_4_68
#[derive(Debug, Clone, PartialEq, Default)]
pub struct Bn256_4_68;

impl RnsParams<Fq, Fr, 4, 68> for Bn256_4_68 {
    fn wrong_modulus() -> BigUint {
        BigUint::from_str(
            "21888242871839275222246405745257275088696311157297823662689037894645226208583",
        )
        .unwrap()
    }

    fn wrong_modulus_in_native_modulus() -> Fr {
        Fr::from_u128(147946756881789318990833708069417712966)
    }

    fn negative_wrong_modulus_decomposed() -> [Fr; 4] {
        let limb0 = Fr::from_u128(51007615349848998585);
        let limb1 = Fr::from_u128(187243884991886189399);
        let limb2 = Fr::from_u128(292141664167738113703);
        let limb3 = Fr::from_u128(295147053861416594661);
        [limb0, limb1, limb2, limb3]
    }

    fn right_shifters() -> [Fr; 4] {
        let limb0 = Fr::from_u128(1);
        let limb1 = Fr::from_raw([
            0xf8e4610fb396ee5,
            0xb42e346981868e48,
            0x1dbc9c192fc7933a,
            0xb603a5609b3f6f8,
        ]);
        let limb2 = Fr::from_raw([
            0x568bea8e0766f9dd,
            0xa31a140f219532a9,
            0x1a908db2cea9b991,
            0x1b7c016fe8acfaed,
        ]);
        let limb3 = Fr::from_raw([
            0x769b0bf04e2f27cc,
            0x55a33201cd88df51,
            0x338287b1e0bedd99,
            0x523513296c10199,
        ]);
        [limb0, limb1, limb2, limb3]
    }

    fn left_shifters() -> [Fr; 4] {
        let limb0 = Fr::from_u128(1);
        let limb1 = Fr::from_raw([0x0, 0x10, 0x0, 0x0]);
        let limb2 = Fr::from_raw([0x0, 0x0, 0x100, 0x0]);
        let limb3 = Fr::from_raw([0x0, 0x0, 0x0, 0x1000]);
        [limb0, limb1, limb2, limb3]
    }
}
