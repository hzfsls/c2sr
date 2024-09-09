pub(crate) mod v_bcd;

use crate::v_bcd::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn itest_util_bcd_001() {
        let mut puc_bcd = [0; 8];
        bcd_init(&mut puc_bcd, 8);
        assert_eq!(puc_bcd, [0xFF, 0xFF, 0xFF, 0xFF, 0, 0, 0, 0]);
    }

    #[test]
    fn itest_util_bcd_002() {
        let mut p_to_bcd = [0; 8];
        let mut p_from_bcd = [0; 4];
        let mut i = 0;
        let mut uc_count = 0;

        uc_count = bcd_cat(&mut p_to_bcd, &mut p_from_bcd, 16);

        for i in 0..3 {
            p_from_bcd[i] = i as u8;
        }

        p_from_bcd[3] = 0xFF;

        for i in 4..7 {
            p_to_bcd[i - 4] = i as u8;
        }

        p_to_bcd[3] = 0xFF;

        uc_count = bcd_cat(&mut p_to_bcd, &mut p_from_bcd, 16);
        assert_eq!(uc_count, 12);

        uc_count = bcd_cat(&mut p_to_bcd, &mut p_from_bcd, 8);
        assert_eq!(uc_count, 8);

        p_to_bcd[4] = 0xFF;

        uc_count = bcd_cat(&mut p_to_bcd, &mut p_from_bcd, 16);
        assert_eq!(uc_count, 14);

        assert_eq!(bcd_value(&mut p_to_bcd, 1), 4);
        assert_eq!(bcd_value(&mut p_to_bcd, 4), 0);
    }
    
    #[test]
    fn itest_util_bcd_007() {
        let mut a_to_bcd = [0; 11];
        let mut i = 0;
        let mut uc_len = 0;

        for i in 0..10 {
            a_to_bcd[i] = i as u8;
        }

        a_to_bcd[10] = 0xFF;

        uc_len = bcd_len(&mut a_to_bcd, 1);
        assert_eq!(uc_len, 1);

        uc_len = bcd_len(&mut a_to_bcd, 10);
        assert_eq!(uc_len, 10);
    }

    #[test]
    fn itest_util_bcd_009() {
        let mut auc_bcd1 = [0; 10];
        let mut auc_bcd2 = [0; 10];
        let mut i = 0;

        for i in 0..9 {
            auc_bcd1[i] = i as u8;
            auc_bcd2[i] = i as u8;
        }

        auc_bcd1[9] = 0xFF;
        auc_bcd2[9] = 0xFF;

        auc_bcd1[8] = 0x45;
        auc_bcd2[8] = 0x44;
        assert_eq!(bcd_compare(&mut auc_bcd1, &mut auc_bcd2, 20), 1);

        auc_bcd1[8] = 0x44;
        auc_bcd2[8] = 0x45;
        assert_eq!(bcd_compare(&mut auc_bcd1, &mut auc_bcd2, 20), -1);

        auc_bcd1[8] = 0x44;
        auc_bcd2[8] = 0x44;
        assert_eq!(bcd_compare(&mut auc_bcd1, &mut auc_bcd2, 20), 0);
    }

    #[test]
    fn itest_util_bcd_012() {
        let mut auc_bcd1 = [0; 11];
        let mut auc_bcd2 = [0; 10];
        let mut i = 0;

        for i in 0..10 {
            auc_bcd1[i] = i as u8;
            auc_bcd2[i] = i as u8;
        }

        auc_bcd1[10] = 0xFF;
        auc_bcd2[9] = 0xFF;

        assert_eq!(bcd_compare(&mut auc_bcd1, &mut auc_bcd2, 20), 1);
    }

    #[test]
    fn itest_util_bcd_013() {
        let mut auc_bcd1 = [0; 10];
        let mut auc_bcd2 = [0; 11];
        let mut i = 0;

        for i in 0..9 {
            auc_bcd1[i] = i as u8;
            auc_bcd2[i] = i as u8;
        }

        auc_bcd1[9] = 0xFF;
        auc_bcd2[10] = 0xFF;

        assert_eq!(bcd_compare(&mut auc_bcd1, &mut auc_bcd2, 20), -1);
    }

    // #[test]
    // fn itest_util_bcd_014() {
    //     let mut auc_bcd = [0; 10];
    //     let mut i = 0;

    //     for i in 0..9 {
    //         auc_bcd[i] = i as u8;
    //     }

    //     auc_bcd[9] = 0xFF;

    //     bcd_copy(&mut auc_bcd, &mut auc_bcd, 20);
    // }

    #[test]
    fn itest_util_bcd_015() {
        let mut auc_source = [0; 10];
        let mut auc_dest = [0; 20];
        let mut i = 0;

        for i in 0..9 {
            auc_source[i] = i as u8;
            auc_dest[i] = i as u8;
        }

        auc_source[9] = 0xFF;
        auc_dest[9] = 0xFF;

        bcd_copy(&mut auc_dest, &mut auc_source, 0);
        bcd_copy(&mut auc_dest, &mut auc_source, 20);
    }
    
    #[test]
    fn itest_util_bcd_017() {
        let mut res = 0;
        assert_eq!(bcd_minus(&mut [], &mut [], 0, &mut res), !0);
    }

    #[test]
    fn itest_util_bcd_018() {
        let mut auc_bcd1 = [0x00, 0x01, 0x02, 0xFF];
        let mut auc_bcd2 = [0x00, 0x01, 0x01, 0xFF];
        let mut res = 0;

        assert_eq!(bcd_minus(&mut auc_bcd1, &mut auc_bcd2, 6, &mut res), 0);
        assert_eq!(res, 1);
    }

    #[test]
    fn itest_util_bcd_019() {
        let mut auc_bcd1 = [0x00, 0x01, 0x01, 0xFF];
        let mut auc_bcd2 = [0x00, 0x01, 0x02, 0xFF];
        let mut res = 0;

        assert_eq!(bcd_minus(&mut auc_bcd1, &mut auc_bcd2, 6, &mut res), 0);
        assert_eq!(res, -1);
    }

    #[test]
    fn itest_util_bcd_020() {
        let mut auc_bcd1 = [0x00, 0x01, 0x0a, 0xFF];
        let mut slice1 = &mut auc_bcd1[..];
        let mut slice2 = unsafe { std::slice::from_raw_parts_mut(slice1.as_mut_ptr(), slice1.len()) };
        let mut res = 0;

        assert_eq!(bcd_minus(slice1, slice2, 6, &mut res), !0);
    }

    #[test]
    fn itest_util_bcd_021() {
        let mut auc_from_bcd = [0x00, 0x01, 0x02, 0xFF];
        let mut auc_result = [0; 10];
        let mut i = 0;
        let mut uc_len = 0;

        bcd_clip(&mut auc_result, &mut auc_from_bcd, 8, 2, 4);
        uc_len = bcd_len(&mut auc_result, 100);
        for i in 0..uc_len {
            print!("{} ", auc_result[i as usize]);
        }

        bcd_clip(&mut auc_result, &mut auc_from_bcd, 8, 9, 4);
        uc_len = bcd_len(&mut auc_result, 100);
        for i in 0..uc_len {
            print!("{} ", auc_result[i as usize]);
        }
    }

    #[test]
    fn itest_util_bcd_023() {
        let mut auc_bcd = [0x09, 0xFF];
        let mut auc_result = [0; 10];
        let mut i = 0;

        bcd_to_string(&mut auc_bcd, &mut auc_result, 10);
        print!("Contents of the string are : ");

        for i in 0..2 {
            print!("{}", auc_result[i as usize] + 0x30);
        }

        print!("\n");
    }

    #[test]
    fn itest_util_bcd_024() {
        let mut auc_str = [1, 2, 3, 4, 0xFF];
        let mut auc_bcd = [0; 10];

        string_to_bcd(&mut auc_str, &mut auc_bcd, 10);
        print!("BCD Number is : {:x}{:x}{:x}{:x}{:x}{:x}\n", (auc_bcd[0] & 0xF0) >> 4,
                auc_bcd[0] & 0x0F, (auc_bcd[1] & 0xF0) >> 4, auc_bcd[1] & 0x0F, (auc_bcd[2] & 0xF0) >> 4,
                auc_bcd[2] & 0x0F);
    }

    #[test]
    fn itest_util_bcd_025() {
        let mut auc_bcd = [1, 2, 3, 0x0F];
        let mut auc_prefix = [1, 2];
        let mut auc_prefix1 = [3, 5];
        let mut auc_prefix2 = [3, 5, 4, 5, 7, 8, 9];

        assert_eq!(bcd_prefix(&mut auc_prefix, &mut auc_bcd, 4), true);
        assert_eq!(bcd_prefix(&mut auc_prefix1, &mut auc_bcd, 10), false);
        assert_eq!(bcd_prefix(&mut auc_prefix2, &mut auc_bcd, 10), false);
    }

    #[test]
    fn itest_util_bcd_028() {
        let i_num = 4;
        let mut aul_bcd = [0; 4];

        assert_eq!(dec_to_bcd(i_num, 1, &mut aul_bcd), 1);

        let i_num = 789000;
        assert_eq!(dec_to_bcd(i_num, 6, &mut aul_bcd), 0);
    }

    #[test]
    fn itest_util_bcd_030() {
        let mut auc_to_bcd = [0; 4];
        let mut auc_from_bcd = [5, 6, 7, 8];

        assert_eq!(bcd_cat_fixed_point(&mut auc_to_bcd, 1, &mut auc_from_bcd, 2, 2), 2);
        assert_eq!(bcd_cat_fixed_point(&mut auc_to_bcd, 9, &mut auc_from_bcd, 2, 2), 9);
    }

    #[test]
    fn itest_util_bcd_032() {
        let mut p_to_bcd = [0; 5];
        let mut p_from_bcd = [1, 2, 3, 4];

        new_bcd_clip(&mut p_to_bcd, &mut p_from_bcd, 5, 4, 0, 4);
        new_bcd_clip(&mut p_to_bcd, &mut p_from_bcd, 5, 4, 5, 4);
    }

    #[test]
    fn itest_util_bcd_test_bcd_value() {
        let mut p_bcd = [1, 2, 3, 0xF];

        assert_eq!(bcd_value(&mut p_bcd, 5), 3);
        assert_eq!(bcd_value(&mut [], 5), 0xFF);
    }
    
    #[test]
    fn itest_util_bcd_test_bcd_len() {
        let mut p_bcd = [1, 2, 3, 0xFF];
        assert_eq!(bcd_len(&mut p_bcd, 8), 6);
        assert_eq!(bcd_len(&mut [], 8), 0);
    }

    #[test]
    fn itest_util_bcd_test_bcd_compare() {
        let mut p_bcd1 = [1, 2, 3, 0xFF];
        let mut p_bcd2 = [1, 2, 3, 0xFF];

        assert_eq!(bcd_compare(&mut p_bcd1, &mut p_bcd2, 6), 0);
        assert_eq!(bcd_compare(&mut p_bcd1, &mut [], 6), 0);
        assert_eq!(bcd_compare(&mut [], &mut p_bcd2, 6), 0);

        p_bcd1[2] = 0x2;
        p_bcd2[2] = 0x3;
        assert_eq!(bcd_compare(&mut p_bcd1, &mut p_bcd2, 6), -1);

        p_bcd1[2] = 0x3;
        p_bcd2[2] = 0x2;
        assert_eq!(bcd_compare(&mut p_bcd1, &mut p_bcd2, 6), 1);
    }

    #[test]
    fn itest_util_bcd_test_bcd_copy() {
        let mut p_from_bcd = [1, 2, 3, 0xFF];
        let mut p_to_bcd = [0; 5];

        bcd_copy(&mut p_to_bcd, &mut p_from_bcd, 5);
        bcd_copy(&mut [], &mut p_from_bcd, 5);
        bcd_copy(&mut p_to_bcd, &mut [], 5);
        bcd_copy(&mut p_from_bcd, &mut p_to_bcd, 5);
    }

    #[test]
    fn itest_util_bcd_test_bcd_minus() {
        let mut auc_bcd1 = [1, 2, 4, 0xFF];
        let mut auc_bcd2 = [1, 2, 3, 0xFF];
        let mut res = 0;

        assert_eq!(bcd_minus(&mut auc_bcd1, &mut auc_bcd2, 6, &mut res), 0);
        assert_eq!(res, 1);
    }

    #[test]
    fn itest_util_bcd_test_bcd_clip() {
        let mut auc_to_bcd = [0; 3];
        let mut auc_from_bcd = [1, 2, 3, 4, 5, 0xFF];

        bcd_clip(&mut auc_to_bcd, &mut auc_from_bcd, 6, 2, 4);
        assert_eq!(auc_to_bcd, [2, 3, 0xFF]);
        bcd_clip(&mut [], &mut auc_from_bcd, 6, 2, 4);
        bcd_clip(&mut auc_to_bcd, &mut [], 6, 2, 4);
    }

    #[test]
    fn itest_util_bcd_test_bcd_to_string() {
        let mut auc_bcd = [1, 2, 3, 0xFF];
        let mut auc_str = [0; 7];
        let mut res = 0;

        res = bcd_to_string(&mut [], &mut auc_str, 6);
        assert_eq!(res, 0);
        res = bcd_to_string(&mut auc_bcd, &mut [], 6);
        assert_eq!(res, 0);
        res = bcd_to_string(&mut auc_bcd, &mut auc_str, 6);
        assert_eq!(res, 6);
        assert_eq!(auc_str[..6], [0, 1, 0, 2, 0, 3]);
    }

    #[test]
    fn itest_util_bcd_test_string_to_bcd() {
        let mut auc_str = [0, 1, 0, 2, 0, 3, 0xFF];
        let mut auc_bcd = [0; 4];
        let mut res = 0;

        res = string_to_bcd(&mut [], &mut auc_bcd, 8);
        assert_eq!(res, 0);
        res = string_to_bcd(&mut auc_str, &mut [], 8);
        assert_eq!(res, 0);
        res = string_to_bcd(&mut auc_str, &mut auc_bcd, 8);
        assert_eq!(res, 6);
        assert_eq!(auc_bcd[0] & 0x0F, 1);
        assert_eq!(auc_bcd[0] & 0xF0, 0);
        assert_eq!(auc_bcd[1] & 0x0F, 2);
        assert_eq!(auc_bcd[1] & 0xF0, 0);
        assert_eq!(auc_bcd[2] & 0x0F, 3);
        assert_eq!(auc_bcd[2] & 0xF0, 0);
    }

    #[test]
    fn itest_util_bcd_test_bcd_prefix() {
        let mut auc_prefix_present = [1, 2, 0xFF];
        let mut auc_prefix_not_present = [1, 4, 0xFF];
        let mut auc_bc_num = [1, 2, 3, 0xFF];

        assert_eq!(bcd_prefix(&mut auc_prefix_present, &mut auc_bc_num, 4), true);
        assert_eq!(bcd_prefix(&mut [], &mut auc_bc_num, 4), false);
        assert_eq!(bcd_prefix(&mut auc_prefix_present, &mut [], 4), false);
        assert_eq!(bcd_prefix(&mut auc_prefix_not_present, &mut auc_bc_num, 4), false);
    }
    
    #[test]
    fn itest_util_bcd_test_dec_to_bcd() {
        let i_num = 19876;
        let mut aul_bcd = [0; 3];

        assert_eq!(dec_to_bcd(i_num, 5, &mut aul_bcd), 5);

        assert_eq!((aul_bcd[0] & 0xF0) >> 4, 1);
        assert_eq!(aul_bcd[0] & 0x0F, 9);
        assert_eq!((aul_bcd[1] & 0xF0) >> 4, 8);
        assert_eq!(aul_bcd[1] & 0x0F, 7);
        assert_eq!((aul_bcd[2] & 0xF0) >> 4, 6);
        assert_eq!(aul_bcd[2] & 0x0F, 0xF);
    }

    #[test]
    fn itest_util_bcd_test_bcd_cat_fixed_point() {
        let mut auc_from_bcd = [1, 2, 0xFF];
        let mut puc_to_bcd = [];

        assert_eq!(bcd_cat_fixed_point(&mut [], 4, &mut auc_from_bcd, 4, 10), 4);
        assert_eq!(bcd_cat_fixed_point(&mut puc_to_bcd, 4, &mut [], 4, 10), 4);

        let mut puc_to_bcd = [0; 10];
        puc_to_bcd[0] = 4;
        puc_to_bcd[1] = 5;
        puc_to_bcd[2] = 0xFF;

        assert_eq!(bcd_cat_fixed_point(&mut puc_to_bcd, 4, &mut auc_from_bcd, 4, 10), 8);
        assert_eq!(puc_to_bcd[2], 1);
        assert_eq!(puc_to_bcd[3], 2);
    }

    #[test]
    fn itest_util_bcd_new_bcd_clip() {
        let mut auc_from_bcd = [1, 2, 3, 0xFF];
        let mut auc_to_bcd = [0xFF, 0xFF, 0xFF, 0xFF];

        new_bcd_clip(&mut [], &mut auc_from_bcd, 6, 6, 2, 4);
        new_bcd_clip(&mut auc_to_bcd, &mut [], 6, 6, 2, 4);
        new_bcd_clip(&mut auc_to_bcd, &mut auc_from_bcd, 6, 6, 2, 4);
        assert_eq!(auc_to_bcd[0], 2);
        assert_eq!(auc_to_bcd[1], 3);
    }

    #[test]
    fn itest_vos_bcd_clip_001() {
        let mut auc_to_bcd = [0; 3];
        let mut auc_from_bcd = [1, 2, 3, 4, 5, 0xFF];

        bcd_clip(&mut auc_to_bcd, &mut auc_from_bcd, 6, 2, 4);
        assert_eq!(auc_to_bcd[0], 2);
        assert_eq!(auc_to_bcd[1], 3);
        assert_eq!(auc_to_bcd[2], 0xFF);
    }
}