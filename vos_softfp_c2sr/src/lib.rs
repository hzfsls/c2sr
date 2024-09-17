pub(crate) mod vos_softfp;

use crate::vos_softfp::*;


#[cfg(test)]
mod tests {
    use super::*;
   
    #[test]
    fn test_vos_soft_fp_invalid_check() {
        let mut vos_fp_float_s = VosFpFloatS::new(1.0);
        assert_eq!(vos_soft_fp_invalid_check(Box::new(vos_fp_float_s)), false);

        let mut vos_fp_float_s = VosFpFloatS::new(f32::NAN);
        assert_eq!(vos_soft_fp_invalid_check(Box::new(vos_fp_float_s)), true);

        let mut vos_fp_float_s = VosFpFloatS::new(f32::INFINITY);
        assert_eq!(vos_soft_fp_invalid_check(Box::new(vos_fp_float_s)), true);
    }

    #[test]
    fn test_vos_soft_fp_str_2_float_ieee754() {
        let mut ss = String::from("e+1");
        let mut res = 1.313;
        assert_eq!(vos_soft_fp_str_2_float_ieee754(&mut ss, &mut res), vos_ok!());
        assert_eq!((res - 13.13).abs() < 0.0000001, true);

        let mut ss = String::from("e-1");
        let mut res = 1.313;
        assert_eq!(vos_soft_fp_str_2_float_ieee754(&mut ss, &mut res), vos_ok!());
        assert_eq!((res - 0.1313).abs() < 0.0000001, true);

        let mut ss = String::from("e+2");
        let mut res = 1.313;
        assert_eq!(vos_soft_fp_str_2_float_ieee754(&mut ss, &mut res), vos_ok!());
        assert_eq!((res - 131.3).abs() < 0.0000001, true);

        let mut ss = String::from("e-2");
        let mut res = 1.313;
        assert_eq!(vos_soft_fp_str_2_float_ieee754(&mut ss, &mut res), vos_ok!());
        assert_eq!((res - 0.01313).abs() < 0.0000001, true);

        let mut ss = String::from("e+3");
        let mut res = 1.313;
        assert_eq!(vos_soft_fp_str_2_float_ieee754(&mut ss, &mut res), vos_ok!());
        assert_eq!((res - 1313.0).abs() < 0.0000001, true);

        let mut ss = String::from("e-3");
        let mut res = 1.313;
        assert_eq!(vos_soft_fp_str_2_float_ieee754(&mut ss, &mut res), vos_ok!());
        assert_eq!((res - 0.001313).abs() < 0.0000001, true);
    }

    #[test]
    fn test_vos_soft_fp_str_2_float() {
        let mut ss = String::from("1.313");
        assert_eq!((vos_soft_fp_str_2_float(&mut ss, None) - 1.313).abs() < 0.0000001, true);

        let mut ss = String::from("1.3137328e+1");
        assert_eq!((vos_soft_fp_str_2_float(&mut ss, None) - 13.137328).abs() < 0.0000001, true);

        let mut ss = String::from("1.3137328e-1");
        assert_eq!((vos_soft_fp_str_2_float(&mut ss, None) - 0.13137328).abs() < 0.0000001, true);

        let mut ss = String::from("1.3137328e+2");
        assert_eq!((vos_soft_fp_str_2_float(&mut ss, None) - 131.37328).abs() < 0.0000001, true);

        let mut ss = String::from("1.3137328e-2");
        assert_eq!((vos_soft_fp_str_2_float(&mut ss, None) - 0.013137328).abs() < 0.0000001, true);

        let mut ss = String::from("1.3137328e+3");
        assert_eq!((vos_soft_fp_str_2_float(&mut ss, None) - 1313.7328).abs() < 0.0000001, true);

        let mut ss = String::from("1.3137328e-3");
        assert_eq!((vos_soft_fp_str_2_float(&mut ss, None) - 0.0013137328).abs() < 0.0000001, true);

        let mut ss = String::from("1.3137328e+3");
        let mut result = String::new();
        assert_eq!((vos_soft_fp_str_2_float(&mut ss, Some(&mut result)) - 1313.7328).abs() < 0.0000001, true);
        assert_eq!(result, "e+3");
    }
    
    #[test]
    fn test_vos_soft_fp_s_eva() {
        let mut ss = String::from("1.313");
        let mut vos_fp_float_s = Box::new(VosFpFloatS::new(0.0));
        assert_eq!(vos_soft_fp_s_eva(&mut ss, &mut vos_fp_float_s), vos_ok!());
        assert_eq!((vos_fp_float_s.f_value - 1.313).abs() < 0.0000001, true);
    }
    
    #[test]
    fn test_vos_soft_fp_s_cmp() {
        let mut vos_fp_float_s1 = Box::new(VosFpFloatS::new(1.313));
        let mut vos_fp_float_s2 = Box::new(VosFpFloatS::new(1.313));
        assert_eq!(vos_soft_fp_s_cmp(&mut vos_fp_float_s1, &mut vos_fp_float_s2), vos_fp_value_eq!());

        let mut vos_fp_float_s1 = Box::new(VosFpFloatS::new(1.314));
        let mut vos_fp_float_s2 = Box::new(VosFpFloatS::new(1.313));
        assert_eq!(vos_soft_fp_s_cmp(&mut vos_fp_float_s1, &mut vos_fp_float_s2), vos_fp_value_gt!());

        let mut vos_fp_float_s1 = Box::new(VosFpFloatS::new(1.313));
        let mut vos_fp_float_s2 = Box::new(VosFpFloatS::new(1.314));
        assert_eq!(vos_soft_fp_s_cmp(&mut vos_fp_float_s1, &mut vos_fp_float_s2), vos_fp_value_lt!());
    }

    #[test]
    fn test_vos_soft_fp_s_add() {
        let mut vos_fp_float_s1 = Box::new(VosFpFloatS::new(1.313));
        let mut vos_fp_float_s2 = Box::new(VosFpFloatS::new(1.313));
        let mut vos_fp_float_s3 = Box::new(VosFpFloatS::new(0.0));
        assert_eq!(vos_soft_fp_s_add(&mut vos_fp_float_s1, &mut vos_fp_float_s2, &mut vos_fp_float_s3), vos_ok!());
        assert_eq!((vos_fp_float_s3.f_value - 2.626).abs() < 0.0000001, true);
    }

    #[test]
    fn test_vos_soft_fp_s_sub() {
        let mut vos_fp_float_s1 = Box::new(VosFpFloatS::new(1.313));
        let mut vos_fp_float_s2 = Box::new(VosFpFloatS::new(1.313));
        let mut vos_fp_float_s3 = Box::new(VosFpFloatS::new(0.0));
        assert_eq!(vos_soft_fp_s_sub(&mut vos_fp_float_s1, &mut vos_fp_float_s2, &mut vos_fp_float_s3), vos_ok!());
        assert_eq!((vos_fp_float_s3.f_value - 0.0).abs() < 0.0000001, true);

        let mut vos_fp_float_s1 = Box::new(VosFpFloatS::new(1.313));
        let mut vos_fp_float_s2 = Box::new(VosFpFloatS::new(1.314));
        let mut vos_fp_float_s3 = Box::new(VosFpFloatS::new(0.0));
        assert_eq!(vos_soft_fp_s_sub(&mut vos_fp_float_s1, &mut vos_fp_float_s2, &mut vos_fp_float_s3), vos_ok!());
        assert_eq!((vos_fp_float_s3.f_value + 0.001).abs() < 0.0000001, true);
    }

    #[test]
    fn test_vos_soft_fp_s_mul() {
        let mut vos_fp_float_s1 = Box::new(VosFpFloatS::new(2.0));
        let mut vos_fp_float_s2 = Box::new(VosFpFloatS::new(2.0));
        let mut vos_fp_float_s3 = Box::new(VosFpFloatS::new(0.0));
        assert_eq!(vos_soft_fp_s_mul(&mut vos_fp_float_s1, &mut vos_fp_float_s2, &mut vos_fp_float_s3), vos_ok!());
        assert_eq!((vos_fp_float_s3.f_value - 4.0).abs() < 0.0000001, true);
    }

    #[test]
    fn test_vos_soft_fp_s_div() {
        let mut vos_fp_float_s1 = Box::new(VosFpFloatS::new(2.0));
        let mut vos_fp_float_s2 = Box::new(VosFpFloatS::new(2.0));
        let mut vos_fp_float_s3 = Box::new(VosFpFloatS::new(0.0));
        assert_eq!(vos_soft_fp_s_div(&mut vos_fp_float_s1, &mut vos_fp_float_s2, &mut vos_fp_float_s3), vos_ok!());
        assert_eq!((vos_fp_float_s3.f_value - 1.0).abs() < 0.0000001, true);

        let mut vos_fp_float_s1 = Box::new(VosFpFloatS::new(2.0));
        let mut vos_fp_float_s2 = Box::new(VosFpFloatS::new(0.0));
        let mut vos_fp_float_s3 = Box::new(VosFpFloatS::new(0.0));
        assert_eq!(vos_soft_fp_s_div(&mut vos_fp_float_s1, &mut vos_fp_float_s2, &mut vos_fp_float_s3), vos_errno_inval!());
    }

    #[test]
    fn test_vos_soft_fp_s_log() {
        let mut vos_fp_float_s1 = Box::new(VosFpFloatS::new(2.0));
        let mut vos_fp_float_s2 = Box::new(VosFpFloatS::new(0.0));
        assert_eq!(vos_soft_fp_s_log(&mut vos_fp_float_s1, &mut vos_fp_float_s2), vos_ok!());
        assert_eq!((vos_fp_float_s2.f_value - 0.6931471805599453).abs() < 0.0000001, true);

        let mut vos_fp_float_s1 = Box::new(VosFpFloatS::new(0.0));
        let mut vos_fp_float_s2 = Box::new(VosFpFloatS::new(0.0));
        assert_eq!(vos_soft_fp_s_log(&mut vos_fp_float_s1, &mut vos_fp_float_s2), vos_errno_inval!());
    }

    #[test]
    fn test_vos_soft_fp_s_log10() {
        let mut vos_fp_float_s1 = Box::new(VosFpFloatS::new(2.0));
        let mut vos_fp_float_s2 = Box::new(VosFpFloatS::new(0.0));
        assert_eq!(vos_soft_fp_s_log10(&mut vos_fp_float_s1, &mut vos_fp_float_s2), vos_ok!());
        assert_eq!((vos_fp_float_s2.f_value - 0.3010299956639812).abs() < 0.0000001, true);

        let mut vos_fp_float_s1 = Box::new(VosFpFloatS::new(0.0));
        let mut vos_fp_float_s2 = Box::new(VosFpFloatS::new(0.0));
        assert_eq!(vos_soft_fp_s_log10(&mut vos_fp_float_s1, &mut vos_fp_float_s2), vos_errno_inval!());
    }

    #[test]
    fn test_vos_soft_fp_s_sqrtf() {
        let mut vos_fp_float_s1 = Box::new(VosFpFloatS::new(4.0));
        let mut vos_fp_float_s2 = Box::new(VosFpFloatS::new(0.0));
        assert_eq!(vos_soft_fp_s_sqrtf(&mut vos_fp_float_s1, &mut vos_fp_float_s2), vos_ok!());
        assert_eq!((vos_fp_float_s2.f_value - 2.0).abs() < 0.0000001, true);

        let mut vos_fp_float_s1 = Box::new(VosFpFloatS::new(-4.0));
        let mut vos_fp_float_s2 = Box::new(VosFpFloatS::new(0.0));
        assert_eq!(vos_soft_fp_s_sqrtf(&mut vos_fp_float_s1, &mut vos_fp_float_s2), vos_errno_inval!());
    }

    #[test]
    fn test_vos_soft_fp_d_eva() {
        let mut ss = String::from("1.313");
        let mut vos_fp_double_s = Box::new(VosFpDoubleS::new(0.0));
        assert_eq!(vos_soft_fp_d_eva(&mut ss, &mut vos_fp_double_s), vos_ok!());
        assert_eq!((vos_fp_double_s.d_value - 1.313).abs() < 0.0000001, true);
    }
    
    #[test]
    fn test_vos_soft_fp_d_cmp() {
        let mut vos_fp_double_s1 = Box::new(VosFpDoubleS::new(1.313));
        let mut vos_fp_double_s2 = Box::new(VosFpDoubleS::new(1.313));
        assert_eq!(vos_soft_fp_d_cmp(&mut vos_fp_double_s1, &mut vos_fp_double_s2), vos_fp_value_eq!());

        let mut vos_fp_double_s1 = Box::new(VosFpDoubleS::new(1.314));
        let mut vos_fp_double_s2 = Box::new(VosFpDoubleS::new(1.313));
        assert_eq!(vos_soft_fp_d_cmp(&mut vos_fp_double_s1, &mut vos_fp_double_s2), vos_fp_value_gt!());

        let mut vos_fp_double_s1 = Box::new(VosFpDoubleS::new(1.313));
        let mut vos_fp_double_s2 = Box::new(VosFpDoubleS::new(1.314));
        assert_eq!(vos_soft_fp_d_cmp(&mut vos_fp_double_s1, &mut vos_fp_double_s2), vos_fp_value_lt!());
    }

    #[test]
    fn test_vos_soft_fp_d_add() {
        let mut vos_fp_double_s1 = Box::new(VosFpDoubleS::new(1.313));
        let mut vos_fp_double_s2 = Box::new(VosFpDoubleS::new(1.313));
        let mut vos_fp_double_s3 = Box::new(VosFpDoubleS::new(0.0));
        assert_eq!(vos_soft_fp_d_add(&mut vos_fp_double_s1, &mut vos_fp_double_s2, &mut vos_fp_double_s3), vos_ok!());
        assert_eq!((vos_fp_double_s3.d_value - 2.626).abs() < 0.0000001, true);
    }

    #[test]
    fn test_vos_soft_fp_d_sub() {
        let mut vos_fp_double_s1 = Box::new(VosFpDoubleS::new(1.313));
        let mut vos_fp_double_s2 = Box::new(VosFpDoubleS::new(1.313));
        let mut vos_fp_double_s3 = Box::new(VosFpDoubleS::new(0.0));
        assert_eq!(vos_soft_fp_d_sub(&mut vos_fp_double_s1, &mut vos_fp_double_s2, &mut vos_fp_double_s3), vos_ok!());
        assert_eq!((vos_fp_double_s3.d_value - 0.0).abs() < 0.0000001, true);

        let mut vos_fp_double_s1 = Box::new(VosFpDoubleS::new(1.313));
        let mut vos_fp_double_s2 = Box::new(VosFpDoubleS::new(1.314));
        let mut vos_fp_double_s3 = Box::new(VosFpDoubleS::new(0.0));
        assert_eq!(vos_soft_fp_d_sub(&mut vos_fp_double_s1, &mut vos_fp_double_s2, &mut vos_fp_double_s3), vos_ok!());
        assert_eq!((vos_fp_double_s3.d_value + 0.001).abs() < 0.0000001, true);
    }

    #[test]
    fn test_vos_soft_fp_d_mul() {
        let mut vos_fp_double_s1 = Box::new(VosFpDoubleS::new(2.0));
        let mut vos_fp_double_s2 = Box::new(VosFpDoubleS::new(2.0));
        let mut vos_fp_double_s3 = Box::new(VosFpDoubleS::new(0.0));
        assert_eq!(vos_soft_fp_d_mul(&mut vos_fp_double_s1, &mut vos_fp_double_s2, &mut vos_fp_double_s3), vos_ok!());
        assert_eq!((vos_fp_double_s3.d_value - 4.0).abs() < 0.0000001, true);
    }

    #[test]
    fn test_vos_soft_fp_d_div() {
        let mut vos_fp_double_s1 = Box::new(VosFpDoubleS::new(2.0));
        let mut vos_fp_double_s2 = Box::new(VosFpDoubleS::new(2.0));
        let mut vos_fp_double_s3 = Box::new(VosFpDoubleS::new(0.0));
        assert_eq!(vos_soft_fp_d_div(&mut vos_fp_double_s1, &mut vos_fp_double_s2, &mut vos_fp_double_s3), vos_ok!());
        assert_eq!((vos_fp_double_s3.d_value - 1.0).abs() < 0.0000001, true);

        let mut vos_fp_double_s1 = Box::new(VosFpDoubleS::new(2.0));
        let mut vos_fp_double_s2 = Box::new(VosFpDoubleS::new(0.0));
        let mut vos_fp_double_s3 = Box::new(VosFpDoubleS::new(0.0));
        assert_eq!(vos_soft_fp_d_div(&mut vos_fp_double_s1, &mut vos_fp_double_s2, &mut vos_fp_double_s3), vos_errno_inval!());
    }

    #[test]
    fn test_vos_soft_fp_d_log() {
        let mut vos_fp_double_s1 = Box::new(VosFpDoubleS::new(2.0));
        let mut vos_fp_double_s2 = Box::new(VosFpDoubleS::new(0.0));
        assert_eq!(vos_soft_fp_d_log(&mut vos_fp_double_s1, &mut vos_fp_double_s2), vos_ok!());
        assert_eq!((vos_fp_double_s2.d_value - 0.6931471805599453).abs() < 0.0000001, true);

        let mut vos_fp_double_s1 = Box::new(VosFpDoubleS::new(0.0));
        let mut vos_fp_double_s2 = Box::new(VosFpDoubleS::new(0.0));
        assert_eq!(vos_soft_fp_d_log(&mut vos_fp_double_s1, &mut vos_fp_double_s2), vos_errno_inval!());
    }

    #[test]
    fn test_vos_soft_fp_d_log10() {
        let mut vos_fp_double_s1 = Box::new(VosFpDoubleS::new(2.0));
        let mut vos_fp_double_s2 = Box::new(VosFpDoubleS::new(0.0));
        assert_eq!(vos_soft_fp_d_log10(&mut vos_fp_double_s1, &mut vos_fp_double_s2), vos_ok!());
        assert_eq!((vos_fp_double_s2.d_value - 0.3010299956639812).abs() < 0.0000001, true);

        let mut vos_fp_double_s1 = Box::new(VosFpDoubleS::new(0.0));
        let mut vos_fp_double_s2 = Box::new(VosFpDoubleS::new(0.0));
        assert_eq!(vos_soft_fp_d_log10(&mut vos_fp_double_s1, &mut vos_fp_double_s2), vos_errno_inval!());
    }

    #[test]
    fn test_vos_soft_fp_d_sqrt() {
        let mut vos_fp_double_s1 = Box::new(VosFpDoubleS::new(4.0));
        let mut vos_fp_double_s2 = Box::new(VosFpDoubleS::new(0.0));
        assert_eq!(vos_soft_fp_d_sqrt(&mut vos_fp_double_s1, &mut vos_fp_double_s2), vos_ok!());
        assert_eq!((vos_fp_double_s2.d_value - 2.0).abs() < 0.0000001, true);

        let mut vos_fp_double_s1 = Box::new(VosFpDoubleS::new(-4.0));
        let mut vos_fp_double_s2 = Box::new(VosFpDoubleS::new(0.0));
        assert_eq!(vos_soft_fp_d_sqrt(&mut vos_fp_double_s1, &mut vos_fp_double_s2), vos_errno_inval!());
    }

    #[test]
    fn test_vos_soft_fp_convertion() {
        let mut vos_fp_double_s = Box::new(VosFpDoubleS::new(1.313));
        let mut us: u16 = 0;
        assert_eq!(vos_soft_fp_double_2_ushort(&mut vos_fp_double_s, &mut us), vos_ok!());
        assert_eq!(us, 1);

        let mut vos_fp_double_s = Box::new(VosFpDoubleS::new(1.313));
        let mut i: i32 = 0;
        assert_eq!(vos_soft_fp_double_2_int32(&mut vos_fp_double_s, &mut i), vos_ok!());
        assert_eq!(i, 1);

        let mut i: i32 = 1;
        let mut vos_fp_double_s = Box::new(VosFpDoubleS::new(0.0));
        assert_eq!(vos_soft_fp_int32_2_double(&mut i, &mut vos_fp_double_s), vos_ok!());
        assert_eq!((vos_fp_double_s.d_value - 1.0).abs() < 0.0000001, true);

        let mut ui: u32 = 1;
        let mut vos_fp_double_s = Box::new(VosFpDoubleS::new(0.0));
        assert_eq!(vos_soft_fp_uint32_2_double(&mut ui, &mut vos_fp_double_s), vos_ok!());
        assert_eq!((vos_fp_double_s.d_value - 1.0).abs() < 0.0000001, true);

        let mut vos_fp_double_s = Box::new(VosFpDoubleS::new(1.313));
        let mut ui: u32 = 0;
        assert_eq!(vos_soft_fp_double_2_uint32(&mut vos_fp_double_s, &mut ui), vos_ok!());
        assert_eq!(ui, 1);

        let mut ui: u32 = 1;
        let mut vos_fp_float_s = Box::new(VosFpFloatS::new(0.0));
        assert_eq!(vos_soft_fp_uint32_2_float(&mut ui, &mut vos_fp_float_s), vos_ok!());
        assert_eq!((vos_fp_float_s.f_value - 1.0).abs() < 0.0000001, true);

        let mut vos_fp_float_s = Box::new(VosFpFloatS::new(1.313));
        let mut ui: u32 = 0;
        assert_eq!(vos_soft_fp_float_2_uint32(&mut vos_fp_float_s, &mut ui), vos_ok!());
        assert_eq!(ui, 1);

        let mut i: i32 = 1;
        let mut vos_fp_float_s = Box::new(VosFpFloatS::new(0.0));
        assert_eq!(vos_soft_fp_int32_2_float(&mut i, &mut vos_fp_float_s), vos_ok!());
        assert_eq!((vos_fp_float_s.f_value - 1.0).abs() < 0.0000001, true);

        let mut vos_fp_float_s = Box::new(VosFpFloatS::new(1.313));
        let mut i: i32 = 0;
        assert_eq!(vos_soft_fp_float_2_int32(&mut vos_fp_float_s, &mut i), vos_ok!());
        assert_eq!(i, 1);

        let mut us: u16 = 1;
        let mut vos_fp_double_s = Box::new(VosFpDoubleS::new(0.0));
        assert_eq!(vos_soft_fp_ushort_2_double(&mut us, &mut vos_fp_double_s), vos_ok!());
        assert_eq!((vos_fp_double_s.d_value - 1.0).abs() < 0.0000001, true);

        let mut s: i16 = 1;
        let mut vos_fp_double_s = Box::new(VosFpDoubleS::new(0.0));
        assert_eq!(vos_soft_fp_short_2_double(&mut s, &mut vos_fp_double_s), vos_ok!());

        let mut vos_fp_double_s = Box::new(VosFpDoubleS::new(1.313));
        let mut s: i16 = 0;
        assert_eq!(vos_soft_fp_double_2_short(&mut vos_fp_double_s, &mut s), vos_ok!());
        assert_eq!(s, 1);

        let mut vos_fp_double_s = Box::new(VosFpDoubleS::new(1.313));
        let mut ascii = String::new();
        assert_eq!(vos_soft_fp_double_2_ascii(&mut vos_fp_double_s, &mut ascii), vos_ok!());
        assert_eq!(ascii, "1.313");
    }
    
    #[test]
    fn test_vos_soft_fp_convertion_2() {
        let mut vos_fp_double_s = Box::new(VosFpDoubleS::new(1.313));
        let mut ull: u64 = 0;
        assert_eq!(vos_soft_fp_double_2_uint64(&mut vos_fp_double_s, &mut ull), vos_ok!());
        assert_eq!(ull, 1);

        let mut ull: u64 = 1;
        let mut vos_fp_double_s = Box::new(VosFpDoubleS::new(0.0));
        assert_eq!(vos_soft_fp_uint64_2_double(&mut ull, &mut vos_fp_double_s), vos_ok!());
        assert_eq!((vos_fp_double_s.d_value - 1.0).abs() < 0.0000001, true);

        let mut sll: i64 = 1;
        let mut vos_fp_double_s = Box::new(VosFpDoubleS::new(0.0));
        assert_eq!(vos_soft_fp_int64_2_double(&mut sll, &mut vos_fp_double_s), vos_ok!());
        assert_eq!((vos_fp_double_s.d_value - 1.0).abs() < 0.0000001, true);

        let mut vos_fp_double_s = Box::new(VosFpDoubleS::new(1.313));
        let mut sll: i64 = 0;
        assert_eq!(vos_soft_fp_double_2_int64(&mut vos_fp_double_s, &mut sll), vos_ok!());
        assert_eq!(sll, 1);
    }


    #[test]
    fn test_vos_soft_dp_d_ceil() {
        let mut vos_fp_double_s = Box::new(VosFpDoubleS::new(1.313));
        let mut vos_fp_double_s2 = Box::new(VosFpDoubleS::new(0.0));
        assert_eq!(vos_soft_fp_d_ceil(&mut vos_fp_double_s, &mut vos_fp_double_s2), vos_ok!());
        assert_eq!((vos_fp_double_s2.d_value - 2.0).abs() < 0.0000001, true);
    }

    #[test]
    fn test_vos_soft_fp_d_exp() {
        let mut vos_fp_double_s = Box::new(VosFpDoubleS::new(2.0));
        let mut vos_fp_double_s2 = Box::new(VosFpDoubleS::new(0.0));
        assert_eq!(vos_soft_fp_d_exp(&mut vos_fp_double_s, &mut vos_fp_double_s2), vos_ok!());
        assert_eq!((vos_fp_double_s2.d_value - 7.3890560989306495).abs() < 0.0000001, true);
    }

    #[test]
    fn test_vos_soft_fp_buffer_2_float() {
        let mut buffer = [0u8; 4];
        buffer.copy_from_slice(&1.313f32.to_le_bytes());
        let mut vos_fp_float_s = Box::new(VosFpFloatS::new(0.0));
        assert_eq!(vos_soft_fp_buffer_2_float(&mut buffer, &mut vos_fp_float_s), vos_ok!());
        assert_eq!((vos_fp_float_s.f_value - 1.313).abs() < 0.0000001, true);
    }

    #[test]
    fn test_vos_soft_fp_buffer_2_double() {
        let mut buffer = [0u8; 8];
        buffer.copy_from_slice(&1.313f64.to_le_bytes());
        let mut vos_fp_double_s = Box::new(VosFpDoubleS::new(0.0));
        assert_eq!(vos_soft_fp_buffer_2_double(&mut buffer, &mut vos_fp_double_s), vos_ok!());
        assert_eq!((vos_fp_double_s.d_value - 1.313).abs() < 0.0000001, true);
    }

    #[test]
    fn test_vos_soft_fp_float_2_buffer() {
        let mut vos_fp_float_s = Box::new(VosFpFloatS::new(1.313));
        let mut buffer = [0u8; 4];
        assert_eq!(vos_soft_fp_float_2_buffer(&mut vos_fp_float_s, &mut buffer), vos_ok!());
        assert_eq!(buffer, 1.313f32.to_le_bytes());
    }

    #[test]
    fn test_vos_soft_fp_double_2_buffer() {
        let mut vos_fp_double_s = Box::new(VosFpDoubleS::new(1.313));
        let mut buffer = [0u8; 8];
        assert_eq!(vos_soft_fp_double_2_buffer(&mut vos_fp_double_s, &mut buffer), vos_ok!());
        assert_eq!(buffer, 1.313f64.to_le_bytes());
    }

    #[test]
    fn test_vos_soft_fp_float_2_string() {
        let mut vos_fp_float_s = Box::new(VosFpFloatS::new(1.313));
        let mut s = String::new();
        assert_eq!(vos_soft_fp_float_2_string(&mut vos_fp_float_s, &mut s), vos_ok!());
        assert_eq!(s, "1.313");
    }

    #[test]
    fn test_vos_soft_fp_double_2_string() {
        let mut vos_fp_double_s = Box::new(VosFpDoubleS::new(1.313));
        let mut s = String::new();
        assert_eq!(vos_soft_fp_double_2_string(&mut vos_fp_double_s, &mut s), vos_ok!());
        assert_eq!(s, "1.31300000000000");
    }

    #[test]
    fn test_vos_soft_fp_int_div() {
        let mut vos_fp_double_s = Box::new(VosFpDoubleS::new(0.0));
        assert_eq!(vos_soft_fp_int_div(2, 2, &mut vos_fp_double_s), vos_ok!());
        assert_eq!((vos_fp_double_s.d_value - 1.0).abs() < 0.0000001, true);

        let mut vos_fp_double_s = Box::new(VosFpDoubleS::new(0.0));
        assert_eq!(vos_soft_fp_int_div(2, 0, &mut vos_fp_double_s), vos_errno_inval!());
    }

    #[test]
    fn test_vos_soft_fp_add_pow_for_dopra() {
        let mut vos_fp_double_s1 = Box::new(VosFpDoubleS::new(2.0));
        let mut vos_fp_double_s2 = Box::new(VosFpDoubleS::new(2.0));
        let mut vos_fp_double_s3 = Box::new(VosFpDoubleS::new(0.0));
        assert_eq!(vos_soft_fp_add_pow_for_dopra(&mut vos_fp_double_s1, &mut vos_fp_double_s2, &mut vos_fp_double_s3), vos_ok!());
        assert_eq!((vos_fp_double_s3.d_value - 4.0).abs() < 0.0000001, true);
    }
}