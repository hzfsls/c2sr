pub(crate) mod vos_softfp;

use crate::vos_softfp::*;

//tests

// #[test]
// fn test_vos_fp_float_s_new() {
//     let f_value = 1.0;
//     let mut vos_fp_float_s = VosFpFloatS::new(f_value);
//     assert_eq!(vos_fp_float_s.f_value, f_value);
//     assert_eq!(vos_fp_float_s.f_value.to_bits(), 0b0111111100000000000000000000000);
//     assert_eq!(vos_fp_float_s.get_st_bit_ui_sign(), 0x0);
//     assert_eq!(vos_fp_float_s.get_st_bit_ui_exp(), 0b01111111);
//     assert_eq!(vos_fp_float_s.get_st_bit_ui_frac(), 0x0);

//     vos_fp_float_s.set_st_bit_ui_frac(0x780000);
//     assert_eq!(vos_fp_float_s.f_value, 1.9375);


//     let d_value = 1.0;
//     let mut vos_fp_double_s = VosFpDoubleS::new(d_value);
//     assert_eq!(vos_fp_double_s.d_value, d_value);
//     assert_eq!(vos_fp_double_s.d_value.to_bits(), 0x3FF0000000000000);
//     assert_eq!(vos_fp_double_s.get_st_bit_ui_sign(), 0x0);
//     assert_eq!(vos_fp_double_s.get_st_bit_ui_exp(), 0b01111111111);
//     assert_eq!(vos_fp_double_s.get_st_bit_ui_frac_0(), 0x0);
//     assert_eq!(vos_fp_double_s.get_st_bit_ui_frac_1(), 0x0);

//     vos_fp_double_s.set_st_bit_ui_frac_1(0xF0000);
//     assert_eq!(vos_fp_double_s.d_value, 1.9375);
// }

// #[test]
// fn test() {
//     let mut res = 1.313;
//     let mut ss = String::from("e+1");
//     vos_soft_fp_str2float_ieee754(&mut ss, &mut res);
//     println!("ss: {} res: {}", ss, res);

//     let mut ss1 = String::from("1.3137328e+100");
//     assert_eq!(vos_soft_fp_str2float(&mut ss1, None), 131.37328);
// }