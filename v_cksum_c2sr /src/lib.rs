pub(crate) mod v_cksum;

use crate::v_cksum::*;

#[cfg(test)]
mod tests {
    use super::*;

    fn itest_vos_cksum(ui_count: u32) {
        let mut pch_buf_iso_vec = vec![9; ui_count as usize];
        let pch_buf_iso = &mut pch_buf_iso_vec;
        let pch_buf_iso_copy = unsafe { std::slice::from_raw_parts_mut(pch_buf_iso.as_mut_ptr(), ui_count as usize) };
        vos_iso_cksum(pch_buf_iso, ui_count, pch_buf_iso_copy); 
        let ui_ret = vos_iso_cksum(pch_buf_iso, ui_count, &mut []);
        assert_eq!(ui_ret, 0);
    }
    #[test]
    fn itest_vos_cksum_sdv_api_001() {
        itest_vos_cksum(2);
    }

    #[test]
    fn itest_vos_cksum_sdv_api_002() {
        itest_vos_cksum(3);
    }

    #[test]
    fn itest_vos_cksum_sdv_api_003() {
        itest_vos_cksum(15);
    }

    #[test]
    fn itest_vos_cksum_sdv_api_004() {
        itest_vos_cksum(16);
    }

    #[test]
    fn itest_vos_cksum_sdv_api_005() {
        itest_vos_cksum(17);
    }

    #[test]
    fn itest_vos_cksum_sdv_api_006() {
        itest_vos_cksum(0xFF);
    }

    #[test]
    fn itest_vos_cksum_sdv_api_007() {
        itest_vos_cksum(0x100);
    }

    #[test]
    fn itest_vos_cksum_sdv_api_008() {
        itest_vos_cksum(0xFFF);
    }

    #[test]
    fn itest_vos_cksum_sdv_api_009() {
        itest_vos_cksum(0x1000);
    }

    #[test]
    fn itest_vos_cksum_sdv_api_010() {
        itest_vos_cksum(0x1001);
    }

    #[test]
    fn itest_vos_cksum_sdv_api_011() {
        itest_vos_cksum(0xFFFF);
    }

    #[test]
    fn itest_vos_cksum_sdv_api_012() {
        itest_vos_cksum(0x10001);
    }

    #[test]
    fn itest_vos_cksum_sdv_api_013() {
        itest_vos_cksum(0xFFFFF);
    }

    #[test]
    fn itest_vos_cksum_sdv_fun_001() {
        let ui_ret = vos_iso_cksum(&mut [], 2, &mut []);
        assert_eq!(ui_ret, 0);
        assert_eq!(vos_iso_check_sum(&mut [], 2, &mut [], 2), 0);
        let mut pch_buf_iso_vec = vec![0; 2];
        let pch_buf_iso = &mut pch_buf_iso_vec;
        let pch_buf_iso_copy = unsafe { std::slice::from_raw_parts_mut(pch_buf_iso.as_mut_ptr(), 2) };
        vos_iso_cksum(pch_buf_iso, 2, pch_buf_iso_copy);
        let ui_ret = vos_iso_cksum(pch_buf_iso, 2, &mut []);
        assert_eq!(ui_ret, 0);
    }

    #[test]
    fn itest_vos_cksum_sdv_fun_002() {
        let mut pch_buf_iso_vec = vec![0, 1];
        let pch_buf_iso = &mut pch_buf_iso_vec;
        let pch_buf_iso_copy = unsafe { std::slice::from_raw_parts_mut(pch_buf_iso.as_mut_ptr(), 2) };
        vos_iso_cksum(pch_buf_iso, 2, pch_buf_iso_copy);
        let ui_ret = vos_iso_cksum(pch_buf_iso, 2, &mut []);
        assert_eq!(ui_ret, 0);
    }

    #[test]
    fn itest_vos_cksum_sdv_fun_003() {
        let mut pch_buf_iso_vec = vec![0, 1];
        let pch_buf_iso = &mut pch_buf_iso_vec;
        let pch_buf_iso_copy = unsafe { std::slice::from_raw_parts_mut(pch_buf_iso.as_mut_ptr(), 2) };
        vos_iso_cksum(pch_buf_iso, 2, pch_buf_iso_copy);
        let ui_ret = vos_iso_cksum(pch_buf_iso, 2, &mut []);
        assert_eq!(ui_ret, 0);
    }

    #[test]
    fn itest_vos_cksum_sdv_fun_004() {
        let mut pch_buf_iso_vec = vec![0; 16];
        for i in 0..16 {
            pch_buf_iso_vec[i] = ('a' as u8) + i as u8;
        }
        let pch_buf_iso = &mut pch_buf_iso_vec;
        let pch_buf_iso_copy = unsafe { std::slice::from_raw_parts_mut(pch_buf_iso.as_mut_ptr(), 16) };
        vos_iso_cksum(pch_buf_iso, 16, pch_buf_iso_copy);
        let ui_ret = vos_iso_cksum(pch_buf_iso, 16, &mut []);
        assert_eq!(ui_ret, 0);
    }
    
    #[test]
    fn itest_vos_cksum_sdv_fun_005() {
        let mut pch_buf_iso_vec = vec![0; 16];
        pch_buf_iso_vec[0]  = '~' as u8;
        pch_buf_iso_vec[1]  = '#' as u8;
        pch_buf_iso_vec[2]  = '@' as u8;
        pch_buf_iso_vec[3]  = '#' as u8;
        pch_buf_iso_vec[4]  = '&' as u8;
        pch_buf_iso_vec[5]  = '#' as u8;
        pch_buf_iso_vec[6]  = '^' as u8;
        pch_buf_iso_vec[7]  = ')' as u8;
        pch_buf_iso_vec[8]  = '*' as u8;
        pch_buf_iso_vec[9]  = '(' as u8;
        pch_buf_iso_vec[10] = '@' as u8;
        pch_buf_iso_vec[11] = '!' as u8;
        pch_buf_iso_vec[12] = '+' as u8;
        pch_buf_iso_vec[13] = '_' as u8;
        pch_buf_iso_vec[14] = '&' as u8;
        pch_buf_iso_vec[15] = '-' as u8;
        let pch_buf_iso = &mut pch_buf_iso_vec;
        let pch_buf_iso_copy = unsafe { std::slice::from_raw_parts_mut(pch_buf_iso[8..].as_mut_ptr(), 8) };
        vos_iso_cksum(pch_buf_iso, 16, pch_buf_iso_copy);
        let ui_ret = vos_iso_cksum(pch_buf_iso, 16, &mut []);
        assert_eq!(ui_ret, 0);
    }

    #[test]
    fn itest_vos_cksum_sdv_fun_006() {
        let mut pch_buf_iso_vec = vec![0; 16];
        pch_buf_iso_vec[0]  = 'A' as u8;
        pch_buf_iso_vec[1]  = 'B' as u8;
        pch_buf_iso_vec[2]  = 'C' as u8;
        pch_buf_iso_vec[3]  = '1' as u8;
        pch_buf_iso_vec[4]  = '2' as u8;
        pch_buf_iso_vec[5]  = '3' as u8;
        pch_buf_iso_vec[6]  = 'a' as u8;
        pch_buf_iso_vec[7]  = 'b' as u8;
        pch_buf_iso_vec[8]  = 'c' as u8;
        pch_buf_iso_vec[9]  = '2' as u8;
        pch_buf_iso_vec[10] = '3' as u8;
        pch_buf_iso_vec[11] = '1' as u8;
        pch_buf_iso_vec[12] = '-' as u8;
        pch_buf_iso_vec[13] = '%' as u8;
        pch_buf_iso_vec[14] = '%' as u8;
        pch_buf_iso_vec[15] = '!' as u8;
        let pch_buf_iso = &mut pch_buf_iso_vec;
        let pch_buf_iso_copy = unsafe { std::slice::from_raw_parts_mut(pch_buf_iso[14..].as_mut_ptr(), 2) };
        vos_iso_cksum(pch_buf_iso, 16, pch_buf_iso_copy);
        let ui_ret = vos_iso_cksum(pch_buf_iso, 16, &mut []);
        assert_eq!(ui_ret, 0);
    }

    #[test]
    fn itest_vos_cksum_sdv_fun_007() {
        let mut pch_buf_iso_vec = vec![0; 4096];
        for i in 0..4096 {
            pch_buf_iso_vec[i] = (i % 2) as u8;
        }
        let pch_buf_iso = &mut pch_buf_iso_vec;
        let pch_buf_iso_copy = unsafe { std::slice::from_raw_parts_mut(pch_buf_iso.as_mut_ptr(), 4096) };
        vos_iso_cksum(pch_buf_iso, 4096, pch_buf_iso_copy);
        let ui_ret = vos_iso_cksum(pch_buf_iso, 4096, &mut []);
        assert_eq!(ui_ret, 0);
    }

    #[test]
    fn itest_vos_cksum_sdv_fun_008() {
        let mut pch_buf_iso_vec = vec![0; 4096];
        for i in 0..4096 {
            pch_buf_iso_vec[i] = (i % 2) as u8;
        }
        let pch_buf_iso = &mut pch_buf_iso_vec;
        let pch_buf_iso_copy = unsafe { std::slice::from_raw_parts_mut(pch_buf_iso[2047..].as_mut_ptr(), 4096) };
        vos_iso_cksum(pch_buf_iso, 4096, pch_buf_iso_copy);
        let ui_ret = vos_iso_cksum(pch_buf_iso, 4096, &mut []);
        assert_eq!(ui_ret, 0);
    }

    #[test]
    fn itest_vos_cksum_sdv_fun_009() {
        let mut pch_buf_iso_vec = vec![0; 4096];
        pch_buf_iso_vec[0]  = 'A' as u8;
        pch_buf_iso_vec[1]  = 'B' as u8;
        pch_buf_iso_vec[2]  = 'C' as u8;
        pch_buf_iso_vec[3]  = '1' as u8;
        pch_buf_iso_vec[4]  = '2' as u8;
        pch_buf_iso_vec[5]  = '3' as u8;
        pch_buf_iso_vec[6]  = 'a' as u8;
        pch_buf_iso_vec[7]  = 'b' as u8;
        pch_buf_iso_vec[8]  = 'c' as u8;
        pch_buf_iso_vec[9]  = '2' as u8;
        pch_buf_iso_vec[10] = '3' as u8;
        pch_buf_iso_vec[11] = '1' as u8;
        pch_buf_iso_vec[12] = '-' as u8;
        pch_buf_iso_vec[13] = '%' as u8;
        pch_buf_iso_vec[14] = '%' as u8;
        pch_buf_iso_vec[15] = '!' as u8;
        for i in 16..4096 {
            pch_buf_iso_vec[i] = (i % 15) as u8;
        }
        let pch_buf_iso = &mut pch_buf_iso_vec;
        let pch_buf_iso_copy = unsafe { std::slice::from_raw_parts_mut(pch_buf_iso[4094..].as_mut_ptr(), 2) };
        vos_iso_cksum(pch_buf_iso, 4096, pch_buf_iso_copy);
        let ui_ret = vos_iso_cksum(pch_buf_iso, 4096, &mut []);
        assert_eq!(ui_ret, 0);
    }

    #[test]
    fn itest_vos_cksum_sdv_fun_010() {
        let mut pch_buf_iso_vec = vec![0; 2];
        pch_buf_iso_vec[0] = '0' as u8;
        pch_buf_iso_vec[1] = '1' as u8;
        let pch_buf_iso = &mut pch_buf_iso_vec;
        let pch_buf_iso_copy = unsafe { std::slice::from_raw_parts_mut(pch_buf_iso.as_mut_ptr(), 2) };
        vos_iso_cksum(pch_buf_iso, 2, pch_buf_iso_copy);
        pch_buf_iso[0] = 2;
        let ui_ret = vos_iso_cksum(pch_buf_iso, 2, &mut []);
        assert_ne!(ui_ret, 0);
    }

    #[test]
    fn itest_vos_cksum_sdv_fun_011() {
        let mut pch_buf_iso_vec = vec![0; 16];
        pch_buf_iso_vec[0]  = 'A' as u8;
        pch_buf_iso_vec[1]  = 'B' as u8;
        pch_buf_iso_vec[2]  = 'C' as u8;
        pch_buf_iso_vec[3]  = '1' as u8;
        pch_buf_iso_vec[4]  = '2' as u8;
        pch_buf_iso_vec[5]  = '3' as u8;
        pch_buf_iso_vec[6]  = 'a' as u8;
        pch_buf_iso_vec[7]  = 'b' as u8;
        pch_buf_iso_vec[8]  = 'c' as u8;
        pch_buf_iso_vec[9]  = '2' as u8;
        pch_buf_iso_vec[10] = '3' as u8;
        pch_buf_iso_vec[11] = '1' as u8;
        pch_buf_iso_vec[12] = '-' as u8;
        pch_buf_iso_vec[13] = '%' as u8;
        pch_buf_iso_vec[14] = '%' as u8;
        pch_buf_iso_vec[15] = '!' as u8;
        let pch_buf_iso = &mut pch_buf_iso_vec;
        let pch_buf_iso_copy = unsafe { std::slice::from_raw_parts_mut(pch_buf_iso[7..].as_mut_ptr(), 9) };
        vos_iso_cksum(pch_buf_iso, 16, pch_buf_iso_copy);
        pch_buf_iso[0] = 2;
        let ui_ret = vos_iso_cksum(pch_buf_iso, 16, &mut []);
        assert_ne!(ui_ret, 0);
    }

    #[test]
    fn itest_vos_cksum_sdv_fun_012() {
        let mut pch_buf_iso_vec = vec![0; 16];
        pch_buf_iso_vec[0]  = 'A' as u8;
        pch_buf_iso_vec[1]  = 'B' as u8;
        pch_buf_iso_vec[2]  = 'C' as u8;
        pch_buf_iso_vec[3]  = '1' as u8;
        pch_buf_iso_vec[4]  = '2' as u8;
        pch_buf_iso_vec[5]  = '3' as u8;
        pch_buf_iso_vec[6]  = 'a' as u8;
        pch_buf_iso_vec[7]  = 'b' as u8;
        pch_buf_iso_vec[8]  = 'c' as u8;
        pch_buf_iso_vec[9]  = '2' as u8;
        pch_buf_iso_vec[10] = '3' as u8;
        pch_buf_iso_vec[11] = '1' as u8;
        pch_buf_iso_vec[12] = '-' as u8;
        pch_buf_iso_vec[13] = '%' as u8;
        pch_buf_iso_vec[14] = '%' as u8;
        pch_buf_iso_vec[15] = '!' as u8;
        let pch_buf_iso = &mut pch_buf_iso_vec;
        let pch_buf_iso_copy = unsafe { std::slice::from_raw_parts_mut(pch_buf_iso[14..].as_mut_ptr(), 2) };
        vos_iso_cksum(pch_buf_iso, 16, pch_buf_iso_copy);
        pch_buf_iso[0] = 2;
        let ui_ret = vos_iso_cksum(pch_buf_iso, 16, &mut []);
        assert_ne!(ui_ret, 0);
    }

    #[test]
    fn itest_vos_cksum_sdv_fun_013() {
        let mut pch_buf_iso_vec = vec![0; 4096];
        for i in 0..4096 {
            pch_buf_iso_vec[i] = (i % 2) as u8;
        }
        let pch_buf_iso = &mut pch_buf_iso_vec;
        let mut cksum_temp = 0;
        let mut count = 0;
        for _ in 0..1000 {
            let cksum = vos_iso_cksum(pch_buf_iso, 4096, &mut []);
            if cksum_temp != cksum {
                cksum_temp = cksum;
                count += 1;
            }
        }
        if count > 1 {
            assert_eq!(0, 1);
        } else {
            assert_eq!(1, 1);
        }
    }

    #[test]
    fn itest_vos_cksum_sdv_fun_014() {
        let mut pch_buf_iso_vec = vec![0; 4096];
        for i in 0..4096 {
            pch_buf_iso_vec[i] = (i % 2) as u8;
        }
        let pch_buf_iso = &mut pch_buf_iso_vec;
        let pch_buf_iso_copy = unsafe { std::slice::from_raw_parts_mut(pch_buf_iso[10..].as_mut_ptr(), 4096) };
        let mut cksum_temp = 0;
        let mut count = 0;
        for _ in 0..1000 {
            let cksum = vos_iso_cksum(pch_buf_iso, 4096, pch_buf_iso_copy);
            if cksum_temp != cksum {
                cksum_temp = cksum;
                count += 1;
            }
        }
        if count > 1 {
            assert_eq!(0, 1);
        } else {
            assert_eq!(1, 1);
        }
    }

    // void TestAll(VOS_UINT32 uiCount, VOS_UINT32 uiVal)
    // {
    //     VOS_UINT32 uiRet;
    //     VOS_CHAR *pchBufISO;
    //     VOS_UINT32 i;
    //     pchBufISO = malloc(uiCount); 
    //     if(NULL == pchBufISO)
    //     {
    //         CU_ASSERT_EQUAL(0, 1);
    //         return;
    //     }
    //     if(1 == uiVal)
    //     {
    //         for(i = 0;i < uiCount; i++)
    //             pchBufISO[i] = i % 255;
    //     }
    //     else if(2 == uiVal)
    //     {
    //         for(i = 0;i < uiCount; i++)
    //             pchBufISO[i] = i % 2;
    //     }
    //     else
    //     {
    //     }
        
    //     VOS_ISO_Cksum(pchBufISO, uiCount, (VOS_UINT8 *)&pchBufISO[0]);
    //     uiRet = VOS_ISO_Cksum(pchBufISO, uiCount, VOS_NULL_PTR);
    //     CU_ASSERT_EQUAL(uiRet, 0);
    //     free(pchBufISO);    
    // }

    fn test_all(ui_count: u32, ui_val: u32) {
        let mut pch_buf_iso_vec = vec![0; ui_count as usize];
        let pch_buf_iso = &mut pch_buf_iso_vec;
        if ui_val == 1 {
            for i in 0..ui_count {
                pch_buf_iso[i as usize] = (i % 255) as u8;
            }
        } else if ui_val == 2 {
            for i in 0..ui_count {
                pch_buf_iso[i as usize] = (i % 2) as u8;
            }
        }
        let pch_buf_iso_copy = unsafe { std::slice::from_raw_parts_mut(pch_buf_iso.as_mut_ptr(), ui_count as usize) };
        vos_iso_cksum(pch_buf_iso, ui_count, pch_buf_iso_copy);
        let ui_ret = vos_iso_cksum(pch_buf_iso, ui_count, &mut []);
        assert_eq!(ui_ret, 0);
    }

    #[test]
    fn itest_vos_cksum_sdv_fun_015() {
        test_all(2, 1);
        test_all(3, 1);
        test_all(15, 1);
        test_all(16, 1);
        test_all(17, 1);
        test_all(0xFF, 1);
        test_all(0x100, 1);
        test_all(0xFFF, 1);
        test_all(0x1000, 1);
        test_all(0x1001, 1);
        test_all(0xFFFF, 1);
        test_all(0x10000, 1);

        test_all(2, 2);
        test_all(3, 2);
        test_all(15, 2);
        test_all(16, 2);
        test_all(17, 2);
        test_all(0xFF, 2);
        test_all(0x100, 2);
        test_all(0xFFF, 2);
        test_all(0x1000, 2);
        test_all(0x1001, 2);
        test_all(0xFFFF, 2);
        test_all(0x10000, 2);
    }

    macro_rules! test_md5_a_init { () => { 0x67452301 } }
    macro_rules! text_less_56 { () => { b"abcdefghijklmnopqrstuvwxyzabcdefghijklmnopqrstuvwxyzabc".to_vec() } }
    macro_rules! text_equal_56 { () => { b"12345678901234567890123456789012345678901234567890123456".to_vec() } }
    macro_rules! text_less_64 { () => { b"\xe5\x95\x8a\xe6\xb3\xa2\xe6\xac\xa1\xe7\x9a\x84\xe9\xa2\x9d\xe4\xbd\x9b\xe6\xad\x8c\xe9\xa2\x9d\xe5\xb7\xb2\xe7\xba\xa7\xe5\x8f\xaf\xe4\xba\x86\xe8\x8e\xab\xe5\x91\xa2\xe5\x93\xa6\xe7\xa0\xb4\xe5\x99\xa8\xe9\x88\xa4\xe6\x96\xaf\xe7\x89\xb9\xe5\x94\x94".to_vec() } }
    macro_rules! text_equal_64 { () => { b"./,!@#$%^&*(){}[\"``~..\"/.;':<>?';:,./:>?:*&^$##%&^&^$!@;.!@#$%^&".to_vec() } }
    macro_rules! text_more_64 { () => { b"a123bcd\"ef896\xe8\x99\xbe7345&^$##6\xe8\x82\x894^&5*&^$##%678o12`~..3pq\xe9\xb1\xbc&^$#rst#%".to_vec() } }
    macro_rules! text_equal_128 { () => { b"a123b\"lmnop\xe8\x9b\x8b7345&^$##6\xe9\xa5\xad4^&5*&^$##\xe7\xb3\x96%678o12`~..3pq&^$#rst#%\"ef&5*&^$#896\xe7\x89\x9b7345&^$##6\xe7\xbe\x8a4^&5*&^$##%678o\xe7\x8c\xaa12`~..3pq&^$#rst#%".to_vec() } }

    fn vos_strlen(pauc_in_text: &mut [u8]) -> usize {
        let mut ul_len = 0;
        for i in 0..pauc_in_text.len() {
            if pauc_in_text[i] == 0 {
                break;
            }
            ul_len += 1;
        }
        ul_len
    }

    #[test]
    fn itest_vos_md5_cksum_001() {
        let mut pauc_digest = vec![0; 20];
        let mut pauc_in_text = text_less_56!();
        let mut ul_text_len = vos_strlen(&mut pauc_in_text);
        let mut aui_init = [0x67452301, 0xefcdab89, 0x98badcfe, 0x10325476];
        vos_md5_cksum(&mut pauc_in_text, ul_text_len as u32, ul_text_len as u32, &mut pauc_digest, &mut []);
        assert_ne!(vos_strlen(&mut pauc_digest), 0);
        vos_md5_cksum(&mut pauc_in_text, ul_text_len as u32, ul_text_len as u32, &mut pauc_digest, &mut aui_init);
        assert_ne!(vos_strlen(&mut pauc_digest), 0);
    }

    #[test]
    fn itest_vos_md5_cksum_002() {
        let mut pauc_digest = vec![0; 20];
        let mut pauc_in_text = text_less_64!();
        let mut ul_text_len = vos_strlen(&mut pauc_in_text);
        vos_md5_cksum(&mut pauc_in_text, ul_text_len as u32, ul_text_len as u32, &mut pauc_digest, &mut []);
        if vos_strlen(&mut pauc_digest) <= 0 {
            assert_eq!(0, 1);
        }
    }

    #[test]
    fn itest_vos_md5_cksum_003() {
        let mut pauc_digest = vec![0; 20];
        let mut pauc_in_text = text_equal_64!();
        let mut ul_text_len = vos_strlen(&mut pauc_in_text);
        vos_md5_cksum(&mut pauc_in_text, ul_text_len as u32, ul_text_len as u32, &mut pauc_digest, &mut []);
        if vos_strlen(&mut pauc_digest) <= 0 {
            assert_eq!(0, 1);
        }
    }

    #[test]
    fn itest_vos_md5_cksum_004() {
        let mut pauc_digest = vec![0; 20];
        let mut pauc_in_text = text_more_64!();
        let mut ul_text_len = vos_strlen(&mut pauc_in_text);
        vos_md5_cksum(&mut pauc_in_text, ul_text_len as u32, ul_text_len as u32, &mut pauc_digest, &mut []);
        if vos_strlen(&mut pauc_digest) <= 0 {
            assert_eq!(0, 1);
        }
    }

    #[test]
    fn itest_vos_md5_cksum_005() {
        let mut pauc_digest = vec![0; 20];
        let mut pauc_in_text = text_equal_128!();
        let mut ul_text_len = vos_strlen(&mut pauc_in_text);
        vos_md5_cksum(&mut pauc_in_text, ul_text_len as u32, ul_text_len as u32, &mut pauc_digest, &mut []);
        if vos_strlen(&mut pauc_digest) <= 0 {
            assert_eq!(0, 1);
        }
    }

    // VOS_VOID ITest_VOS_INET_Cksum_001()
    // {
    //     VOS_INT32 ulRet;
    //     VOS_CHAR paucInText[] = TEXT_LESS_56;
    //     VOS_UINT32 ulTextLen = VOS_StrLen(TEXT_LESS_56);
    //     VOS_UINT16 expected;
    //     VOS_CHECK_SUM_BUF_S astBufArray[1] = {0};

    //     ulRet = VOS_INET_Cksum(paucInText, ulTextLen);
    //     if(ulRet < 0)
    //     {
    //         CU_ASSERT_TRUE(VOS_FALSE);
    //     }

    //     astBufArray[0].pBuf = (VOS_VOID *)paucInText;
    //     astBufArray[0].uiBufLen = ulTextLen;
    //     expected = test_in_cksum(astBufArray, 1, ulTextLen);
    //     CU_ASSERT_EQUAL(ulRet, expected);
    // }

    #[test]
    fn itest_vos_inet_cksum_001() {
        let mut pauc_in_text = text_less_56!();
        let ul_text_len = vos_strlen(&mut pauc_in_text);
        let ul_ret = vos_inet_cksum(&mut pauc_in_text, ul_text_len as u32);
        assert_eq!(ul_ret, 26142);
    }

    #[test]
    fn itest_vos_inet_cksum_002() {
        let mut pauc_in_text = text_less_64!();
        let ul_text_len = vos_strlen(&mut pauc_in_text);
        let ul_ret = vos_inet_cksum(&mut pauc_in_text, ul_text_len as u32);
        assert_eq!(ul_ret, 61365);
    }

    #[test]
    fn itest_vos_inet_cksum_003() {
        let mut pauc_in_text = text_equal_64!();
        let ul_text_len = vos_strlen(&mut pauc_in_text);
        let ul_ret = vos_inet_cksum(&mut pauc_in_text, ul_text_len as u32);
        assert_eq!(ul_ret, 53327);
    }

    #[test]
    fn itest_vos_inet_cksum_004() {
        let mut pauc_in_text = text_more_64!();
        let ul_text_len = vos_strlen(&mut pauc_in_text);
        let ul_ret = vos_inet_cksum(&mut pauc_in_text, ul_text_len as u32);
        assert_eq!(ul_ret, 19728);
    }

    #[test]
    fn itest_vos_inet_cksum_005() {
        let mut pauc_in_text = text_equal_128!();
        let ul_text_len = vos_strlen(&mut pauc_in_text);
        let ul_ret = vos_inet_cksum(&mut pauc_in_text, ul_text_len as u32);
        assert_eq!(ul_ret, 65502);
    }

    #[test]
    fn itest_vos_inet_cksum_006() {
        let mut pauc_in_text = text_equal_128!();
        let ul_text_len = vos_strlen(&mut pauc_in_text);
        let ul_ret = vos_inet_cksum(&mut [], ul_text_len as u32);
        assert_eq!(ul_ret, 65535);
    }

    #[test]
    fn itest_vos_inet_cksum_007() {
        let mut pauc_in_text = text_equal_128!();
        let ul_text_len = 0;
        let ul_ret = vos_inet_cksum(&mut pauc_in_text, ul_text_len as u32);
        assert_eq!(ul_ret, 65535);
    }

    #[test]
    fn itest_vos_inet_cksum_008() {
        let mut pauc_in_text = text_equal_128!();
        let ul_text_len = vos_strlen(&mut pauc_in_text);
        let ul_ret = vos_inet_cksum(&mut pauc_in_text[1..], ul_text_len as u32 - 2);
        assert_eq!(ul_ret, 16421);
    }

}
