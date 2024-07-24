pub(crate) mod sha256;

use crate::sha256::*;

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! sdv_text_less_56 { () => { b"abcdefghijklmnopqrstuvwxyzabcdefghijklmnopqrstuvwxyzabc"
.to_vec() } }
    macro_rules! sdv_text_more_64 { () => { b"a123bcd\"ef896\xe8\x99\xbe7345&^$##6\xe8\x82\x894^&5*&^$##%678o12`~..3pq\xe9\xb1\xbc&^$#rst#%".to_vec() } }

    fn test_digest_cmp(auc_digest_a: &[u8], auc_digest_b: &[u8], ui_digest_len: usize) -> u32 {
        for i in 0..ui_digest_len {
            if auc_digest_a[i] != auc_digest_b[i] {
                return 1;
            }
        }
        return 0;
    }

    #[test]
    fn sdv_vos_sha256_calc_001() {
        let mut auc_output: [u8; sha256_digest_size!()] = [0; sha256_digest_size!()];
        let mut auc_input: Vec<u8> = b"abcdefg".to_vec();
        let ui_inlen: u32 = auc_input.len() as u32;
        let mut st_ctx: Box<VosSha256Ctx> = Box::new(VosSha256Ctx::new());
        let digest_less_64: [u8; sha256_digest_size!()] = [
            0x90, 0x0e, 0xac, 0x36, 0xee, 0x15, 0xdb, 0x25, 0xd5, 0x2b, 0x80, 0x66, 0xfe, 0x82, 0x78, 0xff,
            0x2e, 0xcf, 0xc5, 0x73, 0x53, 0x0f, 0x1d, 0x2c, 0x9b, 0x72, 0x4b, 0x30, 0x77, 0x90, 0x0c, 0x3e,
        ];

        vos_sha256_begin(&mut st_ctx);
        vos_sha256_hash(&mut auc_input, ui_inlen, &mut st_ctx);
        vos_sha256_hash(&mut auc_input, ui_inlen, &mut st_ctx);
        vos_sha256_end(&mut auc_output, sha256_digest_size!(), &mut st_ctx);

        assert_eq!(test_digest_cmp(&auc_output, &digest_less_64, sha256_digest_size!()), 0);
    }

    #[test]
    fn sdv_vos_sha256_calc_002() {
        let mut auc_output: [u8; sha256_digest_size!()] = [0; sha256_digest_size!()];
        let mut auc_input1: Vec<u8> = sdv_text_less_56!();
        let mut auc_input2: Vec<u8> = b"abcdefghi".to_vec();
        let ui_inlen1: u32 = auc_input1.len() as u32;
        let ui_inlen2: u32 = auc_input2.len() as u32;
        let mut st_ctx: Box<VosSha256Ctx> = Box::new(VosSha256Ctx::new());
        let digest_equal_64: [u8; sha256_digest_size!()] = [
            0x3c, 0x6e, 0xdb, 0xa7, 0x94, 0x67, 0x0b, 0x44, 0x41, 0xd1, 0xf3, 0xa8, 0x22, 0x5f, 0xef, 0x18,
            0x35, 0xcb, 0x84, 0x26, 0xf0, 0xc1, 0x80, 0xa0, 0xb9, 0x56, 0x23, 0x25, 0x36, 0x4d, 0xac, 0xcc,
        ];

        vos_sha256_begin(&mut st_ctx);
        vos_sha256_hash(&mut auc_input1, ui_inlen1, &mut st_ctx);
        vos_sha256_hash(&mut auc_input2, ui_inlen2, &mut st_ctx);
        vos_sha256_end(&mut auc_output, sha256_digest_size!(), &mut st_ctx);

        assert_eq!(test_digest_cmp(&auc_output, &digest_equal_64, sha256_digest_size!()), 0);
    }

    #[test]
    fn sdv_vos_sha256_calc_003() {
        let mut auc_output: [u8; sha256_digest_size!()] = [0; sha256_digest_size!()];
        let mut auc_input1: Vec<u8> = sdv_text_less_56!();
        let mut auc_input2: Vec<u8> = sdv_text_less_56!();
        let ui_inlen1: u32 = auc_input1.len() as u32;
        let ui_inlen2: u32 = auc_input2.len() as u32;
        let mut st_ctx: Box<VosSha256Ctx> = Box::new(VosSha256Ctx::new());
        let digest_more_64: [u8; sha256_digest_size!()] = [
            0x68, 0x1d, 0x5b, 0x8f, 0xd1, 0xe7, 0x9b, 0x10, 0x17, 0x61, 0x52, 0x51, 0x35, 0x66, 0xd9, 0xd2,
            0xbc, 0x71, 0x31, 0xe0, 0x38, 0x70, 0xff, 0xb8, 0x4a, 0xe9, 0x63, 0xc6, 0x3b, 0x1b, 0xde, 0x4f,
        ];

        vos_sha256_begin(&mut st_ctx);
        vos_sha256_hash(&mut auc_input1, ui_inlen1, &mut st_ctx);
        vos_sha256_hash(&mut auc_input2, ui_inlen2, &mut st_ctx);
        vos_sha256_end(&mut auc_output, sha256_digest_size!(), &mut st_ctx);

        assert_eq!(test_digest_cmp(&auc_output, &digest_more_64, sha256_digest_size!()), 0);
    }

    #[test]
    fn sdv_vos_sha256_calc_004() {
        let mut auc_output: [u8; sha256_digest_size!()] = [0; sha256_digest_size!()];
        let mut auc_input1: Vec<u8> = sdv_text_more_64!();
        let mut auc_input2: Vec<u8> = sdv_text_more_64!();
        let ui_inlen1: u32 = auc_input1.len() as u32;
        let ui_inlen2: u32 = auc_input2.len() as u32;
        let mut st_ctx: Box<VosSha256Ctx> = Box::new(VosSha256Ctx::new());
        let digest_more_128: [u8; sha256_digest_size!()] = [
            0x70, 0xde, 0x62, 0x9f, 0x46, 0xa1, 0xfc, 0x73, 0xf5, 0x25, 0x14, 0x72, 0x96, 0x06, 0xb1, 0x4b,
            0xf4, 0x1d, 0x65, 0x35, 0xc4, 0x1f, 0xe6, 0x93, 0x94, 0x4f, 0x67, 0x3d, 0x0f, 0xfb, 0x78, 0x0e,
        ];

        vos_sha256_begin(&mut st_ctx);
        vos_sha256_hash(&mut auc_input1, ui_inlen1, &mut st_ctx);
        vos_sha256_hash(&mut auc_input2, ui_inlen2, &mut st_ctx);
        vos_sha256_end(&mut auc_output, sha256_digest_size!(), &mut st_ctx);

        assert_eq!(test_digest_cmp(&auc_output, &digest_more_128, sha256_digest_size!()), 0);
    }

    #[test]
    fn sdv_vos_sha256_calc_005() {
        let mut auc_output: [u8; sha256_digest_size!()] = [0; sha256_digest_size!()];
        let mut auc_input1: Vec<u8> = vec![b'a'; 0x40000000];
        let ui_inlen1: u32 = auc_input1.len() as u32;
        let mut st_ctx: Box<VosSha256Ctx> = Box::new(VosSha256Ctx::new());
        let digest_0x40000000: [u8; sha256_digest_size!()] = [
            0x95, 0xdf, 0x3e, 0xa6, 0x1d, 0xb5, 0x57, 0xb2, 0x2c, 0x1a, 0xbf, 0x60, 0x96, 0x45, 0xc3, 0x42,
            0x3b, 0xf8, 0x37, 0x74, 0xc2, 0x2c, 0x75, 0xe3, 0xc6, 0x37, 0xf8, 0xcb, 0x7f, 0xc3, 0x3f, 0xd8,
        ];

        vos_sha256_begin(&mut st_ctx);
        vos_sha256_hash(&mut auc_input1, ui_inlen1, &mut st_ctx);
        vos_sha256_hash(&mut auc_input1, ui_inlen1, &mut st_ctx);
        vos_sha256_end(&mut auc_output, sha256_digest_size!(), &mut st_ctx);

        assert_eq!(test_digest_cmp(&auc_output, &digest_0x40000000, sha256_digest_size!()), 0);
    }

    #[test]
    fn sdv_vos_sha256_calc_006() {
        let mut auc_output: [u8; sha256_digest_size!()] = [0; sha256_digest_size!()];
        let mut auc_input: Vec<u8> = sdv_text_more_64!();
        let ui_inlen: u32 = auc_input.len() as u32;
        let mut st_ctx: Box<VosSha256Ctx> = Box::new(VosSha256Ctx::new());
        let mut ui_ret: u32 = 0;

        vos_sha256_begin(&mut st_ctx);
        vos_sha256_hash(&mut auc_input, ui_inlen, &mut st_ctx);
        vos_sha256_end(&mut auc_output, 16, &mut st_ctx);

        for i in 16..sha256_digest_size!() {
            if auc_output[i] != 0 {
                ui_ret = 1;
                break;
            }
        }

        assert_eq!(ui_ret, 0);
    }

}