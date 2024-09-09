mod rapidlz_inner;
mod rapidlz_compress;
mod rapidlz_decompress;
mod rapidlz_log;

use crate::rapidlz_inner::*;
use crate::rapidlz_compress::*;
use crate::rapidlz_decompress::*;
use crate::rapidlz_log::*;

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! rapidlz_max_input_size { () => { 0x7E000000 }; }

    fn test_log_func(message: &str, size: usize) {
        println!("LogError : {}, size: {}", message, size);
    }

    #[test]
    fn rapidlz_compress_api_002() {
        assert_eq!(2122219150, rapidlz_compress_bound(rapidlz_max_input_size!()));
        assert_eq!(0, rapidlz_compress_bound(rapidlz_max_input_size!() + 1));
    }

    #[test]
    fn rapidlz_compress_api_003() {
        let src = b"wieruoweiuro12lsdf123mkam078mcbs";
        let src_size = src.len();
        let dst_size = rapidlz_compress_bound(src_size);
        assert!(dst_size < 1024 * 10);
        let mut dst = vec![0; dst_size];
        assert_eq!(0, rapidlz_compress(&mut vec![], &mut dst, 0, 0, 0));
        assert_eq!(0, rapidlz_compress(&mut src.to_vec(), &mut vec![], src_size, dst_size, 0));
        assert_eq!(0, rapidlz_compress(&mut vec![], &mut dst, 0, dst_size, 0));
        assert_eq!(0, rapidlz_compress(&mut src.to_vec(), &mut dst, src_size, 0, 0));
        assert_eq!(0, rapidlz_compress(&mut src.to_vec(), &mut dst, src_size, dst_size, 11));
        for compress_level in 1..=10 {
            assert_eq!(0, rapidlz_compress(&mut vec![], &mut vec![], 0, 0, compress_level));
            assert_eq!(0, rapidlz_compress(&mut vec![], &mut dst, src_size, dst_size, compress_level));
            assert_eq!(0, rapidlz_compress(&mut src.to_vec(), &mut vec![], src_size, dst_size, compress_level));
            assert_eq!(0, rapidlz_compress(&mut src.to_vec(), &mut dst, 0, dst_size, compress_level));
            assert_eq!(0, rapidlz_compress(&mut src.to_vec(), &mut dst, src_size, 0, compress_level));
            assert_ne!(0, rapidlz_compress(&mut src.to_vec(), &mut dst, src_size, dst_size - 1, compress_level));
        }
    }


    // #[test]
    // fn rapidlz_compress_api_004() {
    //     let mut src = b"wieruoweiuro12lfasdert46546snbn_?sd'+ert&/gfdsdf123mkam078mcbs".to_vec();
    //     let src_size = src.len();
    //     let dst_size = rapidlz_compress_bound(src_size);
    //     assert!(dst_size < 1024 * 10);
    //     let mut dst = vec![0; dst_size];
    //     let c_size = rapidlz_compress(&mut src, &mut dst, src_size, dst_size, 10);
    //     assert_ne!(0, c_size);
    //     let mut compare_src = vec![0; src_size];
    //     assert_eq!(src_size, rapidlz_decompress(&mut dst, &mut compare_src, c_size, src_size));
    //     assert_eq!(src, compare_src.as_slice());
    // }

    #[test]
    fn rapidlz_compress_api_005() {
        let src = b"wieruoweiuro12lsdf123mkam078mcbs";
        let src_size = src.len();
        let dst_size = rapidlz_compress_bound(src_size);
        assert!(dst_size < 1024 * 10);
        let mut dst = vec![0; dst_size];
        assert_eq!(0, rapidlz_compress_default(&mut vec![], &mut dst, 0, 0));
        assert_eq!(0, rapidlz_compress_default(&mut src.to_vec(), &mut vec![], src_size, dst_size));
        assert_eq!(0, rapidlz_compress_default(&mut src.to_vec(), &mut dst, 0, dst_size));
        assert_eq!(0, rapidlz_compress_default(&mut src.to_vec(), &mut dst, src_size, 0));
        assert_ne!(0, rapidlz_compress_default(&mut src.to_vec(), &mut dst, src_size, dst_size - 1));
    }


    #[test]
    fn rapidlz_compress_api_006() {
        rapidlz_log_register(test_log_func);
        let mut src = b"wieruoweiuro12lfasdert46546snbn_?sd'+ert&/gfdsdf123mkam078mcbs".to_vec();
        let src_size = src.len();
        let dst_size = rapidlz_compress_bound(src_size);
        assert!(dst_size < 1024 * 10);
        let mut dst = vec![0; dst_size];
        let c_size = rapidlz_compress_default(&mut src, &mut dst, src_size, dst_size);
        println!("compress dst: {:?}", dst);
        println!("c_size: {}", c_size);
        assert_ne!(0, c_size);
        let mut compare_src = vec![0; src_size];
        assert_eq!(src_size, rapidlz_decompress(&mut dst, &mut compare_src, c_size, src_size));
        assert_eq!(src.as_slice(), compare_src.as_slice());
    }

    #[test]
    fn rapidlz_compress_api_006_1() {
        rapidlz_log_register(test_log_func);
        let mut src = b"wieruoweiuro12lfasdert46546snbn_?sd'+ert&/gfdsdf123mkam078mcbssfgaugifhunawj fhauifbwuojiafwau".to_vec();
        let src_size = src.len();
        let dst_size = rapidlz_compress_bound(src_size);
        assert!(dst_size < 1024 * 10);
        let mut dst = vec![0; dst_size];
        let c_size = rapidlz_compress_default(&mut src, &mut dst, src_size, dst_size);
        println!("compress dst: {:?}", dst);
        println!("c_size: {}", c_size);
        assert_ne!(0, c_size);
        let mut compare_src = vec![0; src_size];
        assert_eq!(src_size, rapidlz_decompress(&mut dst, &mut compare_src, c_size, src_size));
        assert_eq!(src.as_slice(), compare_src.as_slice());
    }

    #[test]
    fn rapidlz_compress_api_007() {
        assert_ne!(rapidlz_version_get(), "");
    }
    

    #[test]
    fn rapidlz_compress_api_008() {
        rapidlz_log_register(test_log_func);
        rapidlz_log!(0, "{}", "error");
    }

    #[test]
    fn rapidlz_compress_api_009() {
        let val: u64 = 0x100000000;
        assert_eq!(32, high_bit64(val));

        let val: u64 = 0xfffffffff;
        assert_eq!(35, high_bit64(val));

        let val: u64 = 0x1000000000000;
        assert_eq!(48, high_bit64(val));

        let val: u64 = 0x8000000000000000;
        assert_eq!(63, high_bit64(val));
    }


    #[test]
    fn rapidlz_compress_api_010() {
        let val: u64 = 0x100000000;
        assert_eq!(32, count_tail_zero64(val));

        let val: u64 = 0xffffff000;
        assert_eq!(12, count_tail_zero64(val));

        let val: u64 = 0x1000000000001;
        assert_eq!(0, count_tail_zero64(val));

        let val: u64 = 0x8000000000000000;
        assert_eq!(63, count_tail_zero64(val));
    }
}