pub(crate) mod bzp_type;
pub(crate) mod public;
pub(crate) mod compress;
pub(crate) mod decompress;

use crate::bzp_type::*;
use crate::compress::bzp_bwt_encode::*;
use crate::compress::bzp_huffman_encode::*;
use crate::compress::bzp_mtf_encode::*;
use crate::decompress::bzp_bwt_decode::*;
use crate::decompress::bzp_huffman_decode::*;
use crate::public::bzp_compress_stream::*;
use crate::public::bzp_decompress_stream::*;
use crate::public::bzp_stream_utils::*;
use crate::public::bzp_utils::*;


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ut_bwt_init_func_001() {
        let mi_level = bzp_block_size_level_lower_limit!();
        let mx_level = bzp_block_size_level_upper_limit!();
        for i in mi_level..=mx_level {
            let bwt = bzp_block_sort_init(i);
            assert_ne!(bwt.is_none(), true);
            bzp_bwt_finish(bwt.unwrap());
        }
    }

    #[test]
    fn ut_bwt_main_func_001() {
        let level = 1;
        let mut bwt = bzp_block_sort_init(level).unwrap();
        let mut ss: Vec<u8> = b"vxevfdoqrscqyumzltnjdozcjzhexqvdqvlpkxauluhqwvzodicdcexmlloskrqswogiwdgnymgjznnmqdvafxjzwebjfpqsgfci".to_vec();
        let res: Vec<i32> = vec![
            83, 38, 90, 50, 52, 98, 23, 10, 51, 69, 48, 5,  20, 31, 81, 89, 2,  53, 27, 97, 4,  92, 84, 96, 66,
            74, 70, 26, 42, 49, 99, 67, 19, 91, 24, 75, 86, 60, 36, 56, 57, 34, 16, 40, 73, 55, 79, 14, 18, 78,
            77, 71, 47, 65, 6,  58, 21, 35, 93, 80, 7,  94, 62, 29, 32, 43, 11, 61, 8,  9,  95, 59, 63, 17, 41,
            39, 13, 82, 30, 3,  33, 0,  45, 68, 88, 64, 44, 37, 1,  85, 54, 28, 72, 12, 22, 25, 15, 76, 46, 87,
        ];
        bwt.n_block = 0;
        for i in 0..ss.len() {
            bwt.n_block += 1;
            bwt.block[i] = ss[i];
        }
        bzp_block_sort_main(&mut bwt);
        for i in 0..bwt.n_block {
            assert_eq!(bwt.sort_block[i as usize], res[i as usize]);
        }
        bzp_bwt_finish(bwt);
    }

    #[test]
    fn ut_bwt_qsort_func_001() {
        let ss: Vec<u8> = b"qwertyuioaspdgcfvxbhzjnkml".to_vec();
        let mut sort_block: Vec<i32> = vec![0; 30];
        let mut idx: Vec<i32> = vec![0; 30];
        let n = ss.len();
        for i in 0..n {
            sort_block[i] = i as i32;
            idx[i] = ss[i] as i32 - 'a' as i32;
        }
        bzp_quick_sort(&mut sort_block, &mut idx, 0, n as i32 - 1);
        for i in 0..n {
            assert_eq!(ss[sort_block[i as usize] as usize], (i as u8 + 'a' as u8));
        }
    }

    #[test]
    fn ut_bwt_qsort_func_002() {
        let ss: Vec<u8> = b"qwedqsewdfubasqsbwvb".to_vec();
        let mut sort_block: Vec<i32> = vec![0; 30];
        let mut idx: Vec<i32> = vec![0; 30];
        let n = ss.len();
        let res: Vec<i32> = vec![12, 16, 19, 11, 3, 8, 2, 6, 9, 0, 14, 4, 5, 13, 15, 10, 18, 1, 17, 7];
        for i in 0..n {
            sort_block[i] = i as i32;
            idx[i] = ss[i] as i32 - 'a' as i32;
        }
        bzp_quick_sort(&mut sort_block, &mut idx, 0, n as i32 - 1);
        assert_eq!(sort_block[..n], res);
        for i in 0..n {
            assert_eq!(sort_block[i as usize], res[i as usize]);
        }
    }

    #[test]
    fn ut_bzp_bwt_decode_init_func_001() {
        let mi_level = bzp_block_size_level_lower_limit!();
        let mx_level = bzp_block_size_level_upper_limit!();
        for i in mi_level..=mx_level {
            let bwt = bzp_bwt_decode_init(i);
            assert_ne!(bwt.is_none(), true);
            bzp_bwt_decode_finish(bwt.unwrap());
        }
    }

    #[test]
    fn ut_bzp_bwt_decode_main_func_001() {
        let ss: Vec<u8> = b"vxeidfzscwofjvqwxchgvjasomdzudcgnbcgxspmlvzuyxnutnzgzwdldlfmoprxdhckqrqoqllaydqeqiwizsqkvfeenqojmjvj".to_vec();
        let res: Vec<u8> = b"vxevfdoqrscqyumzltnjdozcjzhexqvdqvlpkxauluhqwvzodicdcexmlloskrqswogiwdgnymgjznnmqdvafxjzwebjfpqsgfci".to_vec();
        let mut debwt = bzp_bwt_decode_init(1);
        assert_ne!(debwt.is_none(), true);
        let mut debwt = debwt.unwrap();
        debwt.ori_ptr = 81;
        for i in 0..ss.len() {
            debwt.block[debwt.n_block as usize] = ss[i];
            debwt.n_block += 1;
        }
        bzp_bwt_decode(&mut debwt);
        for i in 0..ss.len() {
            assert_eq!(debwt.de_code[i as usize], res[i as usize]);
        }
        bzp_bwt_decode_finish(debwt);
    }

    #[test]
    fn ut_bzp_huffman_init_func_001() {
        let huffman = bzp_huffman_groups_init(9);
        assert_ne!(huffman.is_none(), true);
        bzp_huffman_groups_finish(huffman.unwrap());

        let huffman = bzp_huffman_groups_init(3);
        assert_ne!(huffman.is_none(), true);
        bzp_huffman_groups_finish(huffman.unwrap());

        let mut huffman = bzp_huffman_groups_init(5);
        assert_ne!(huffman.is_none(), true);
        let mut huffman = huffman.unwrap();
        bzp_huffman_groups_reset(&mut huffman, 7);
        assert_eq!(huffman.alpha_size, 7);
        assert_eq!(huffman.huffman_groups[0].alpha_size, 7);
        bzp_huffman_groups_reset(&mut huffman, 300);
        assert_eq!(huffman.alpha_size, 7);
        bzp_huffman_groups_finish(huffman);

        let huffman = bzp_huffman_groups_init(10);
        assert_eq!(huffman.is_none(), true);
        let huffman = bzp_huffman_groups_init(0);
        assert_eq!(huffman.is_none(), true);
    }

    #[test]
    fn ut_bzp_huffman_decode_init_func_001() {
        let huffman = bzp_huffman_decode_init(9);
        assert_ne!(huffman.is_none(), true);
        bzp_huffman_decode_finish(huffman.unwrap());
        let huffman = bzp_huffman_decode_init(1);
        assert_ne!(huffman.is_none(), true);
        bzp_huffman_decode_finish(huffman.unwrap());
        let huffman = bzp_huffman_decode_init(0);
        assert_eq!(huffman.is_none(), true);
        let huffman = bzp_huffman_decode_init(10);
        assert_eq!(huffman.is_none(), true);
    }

    #[test]
    fn ut_bzp_huffman_encode_func_001() {
        let mut huffman = bzp_get_huffman_groups(0);
        assert_eq!(huffman, 2);
        bzp_get_huffman_groups(1);
        assert_eq!(huffman, 2);
        huffman = bzp_get_huffman_groups(100);
        assert_eq!(huffman, 2);
        huffman = bzp_get_huffman_groups(200);
        assert_eq!(huffman, 3);
        huffman = bzp_get_huffman_groups(400);
        assert_eq!(huffman, 3);
        huffman = bzp_get_huffman_groups(600);
        assert_eq!(huffman, 4);
        huffman = bzp_get_huffman_groups(800);
        assert_eq!(huffman, 4);
        huffman = bzp_get_huffman_groups(1200);
        assert_eq!(huffman, 5);
        huffman = bzp_get_huffman_groups(1800);
        assert_eq!(huffman, 5);
        huffman = bzp_get_huffman_groups(2400);
        assert_eq!(huffman, 6);
        huffman = bzp_get_huffman_groups(3000);
        assert_eq!(huffman, 6);
    }
    
    #[test]
    fn ut_bzp_mtf_init_func_001() {
        let mi_level = bzp_block_size_level_lower_limit!();
        let mx_level = bzp_block_size_level_upper_limit!();
        for i in mi_level..=mx_level {
            let mtf = bzp_mtf_init(i);
            assert_ne!(mtf.is_none(), true);
            bzp_mtf_finish(mtf.unwrap());
        }
    }

    #[test]
    fn ut_bzp_mtf_main_func_001() {
        let mut mtf = bzp_mtf_init(1);
        assert_ne!(mtf.is_none(), true);
        let mut mtf = mtf.unwrap();
        let res: Vec<i32> = vec![3, 0, 3, 3, 1, 4];
        mtf.block = b"banana".to_vec();
        mtf.map = vec![5, 3, 1, 0, 4, 2];
        mtf.in_use = vec![false; 256];
        mtf.n_block = 0;
        for i in 0..mtf.block.len() {
            mtf.n_block += 1;
            mtf.in_use[mtf.block[i] as usize] = true;
        }
        bzp_mtf_main(&mut mtf);
        assert_eq!(mtf.mtf_v[..mtf.n_mtf as usize], res);
        for i in 0..mtf.n_mtf {
            assert_eq!(mtf.mtf_v[i as usize], res[i as usize]);
        }
        bzp_mtf_finish(mtf);
    }


    #[test]
    fn ut_bzp_mtf_num_encode_func_001() {
        let res: Vec<Vec<i32>> = vec![
            vec![],
            vec![0],
            vec![1],
            vec![0, 0],
            vec![1, 0],
            vec![0, 1],
            vec![1, 1],
            vec![0, 0, 0],
        ];
        let mut mtf = bzp_mtf_init(1);
        assert_ne!(mtf.is_none(), true);
        let mut mtf = mtf.unwrap();
        mtf.mtf_v = vec![0; 100];
        for num in 1..=7 {
            bzp_num_encode(&mut mtf, num);
            assert_eq!(mtf.n_mtf, res[num as usize].len() as i32);
            for i in 0..mtf.n_mtf {
                assert_eq!(mtf.mtf_v[i as usize], res[num as usize][i as usize]);
            }
            mtf.n_mtf = 0;
            mtf.mtf_freq = [0; 258];
            mtf.mtf_v = vec![0; 100];
        }
        bzp_mtf_finish(mtf);
    }

    #[test]
    fn ut_stream_run_ok_func(){
        let mut d = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let in_name = d.join("test_data/compress_samples/2.txt");
        let in_name = in_name.to_str().unwrap();
        let out_name = d.join("test_data/compress_out/2.txt.bz2");
        let out_name = out_name.to_str().unwrap();
        let block_size = 9;
        let ret = bzp_compress_stream(in_name, out_name, block_size);
        assert_eq!(ret, bzp_ok!());
    }

    #[test]
    fn ut_stream_run_param_error_func(){
        let mut d = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let in_name = d.join("test_data/decompress_samples/2.txt.bz2");
        let in_name = in_name.to_str().unwrap();
        let out_name = d.join("test_data/decompress_out/2.txt");
        let out_name = out_name.to_str().unwrap();
        let block_size = 9;
        let mut ret = bzp_compress_stream("", out_name, 9);
        assert_eq!(ret, bzp_error_param!());
        ret = bzp_compress_stream(in_name, "", 9);
        assert_eq!(ret, bzp_error_param!());
        ret = bzp_compress_stream(in_name, out_name, 10);
        assert_eq!(ret, bzp_error_param!());
        ret = bzp_compress_stream(in_name, out_name, 0);
        assert_eq!(ret, bzp_error_param!());
        ret = bzp_compress_stream(in_name, out_name, -5);
        assert_eq!(ret, bzp_error_param!());
    }

    #[test]
    fn ut_stream_io_error_func(){
        let mut d = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let in_name = d.join("test_data/compress_samples/nothing.txt");
        let in_name = in_name.to_str().unwrap();
        let out_name = d.join("test_data/compress_out/nothing.txt.bz2");
        let out_name = out_name.to_str().unwrap();
        let block_size = 9;
        let ret = bzp_compress_stream(in_name, out_name, block_size);
        assert_eq!(ret, bzp_error_io!());
    }

    #[test]
    fn ut_de_com_stream_run_ok_func() {
        let mut d = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let in_name = d.join("test_data/decompress_samples/2.txt.bz2");
        let in_name = in_name.to_str().unwrap();
        let out_name = d.join("test_data/decompress_out/2.txt");
        let out_name = out_name.to_str().unwrap();
        let block_size = 9;
        let ret = bzp_de_compress_stream(in_name, out_name);
        assert_eq!(ret, bzp_ok!());
    }

    #[test]
    fn ut_de_com_stream_param_error_func() {
        let mut d = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let in_name = d.join("test_data/decompress_samples/2.txt.bz2.ori");
        let in_name = in_name.to_str().unwrap();
        let out_name = d.join("test_data/decompress_out/2.txt");
        let out_name = out_name.to_str().unwrap();
        let block_size = 9;
        let mut ret = bzp_de_compress_stream("", out_name);
        assert_eq!(ret, bzp_error_param!());
        ret = bzp_de_compress_stream(in_name, "");
        assert_eq!(ret, bzp_error_param!());
    }

    #[test]
    fn ut_de_com_stream_io_error_func() {
        let mut d = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let in_name = d.join("test_data/decompress_samples/nothing.txt.bz2");
        let in_name = in_name.to_str().unwrap();
        let out_name = d.join("test_data/decompress_out/nothing.txt");
        let out_name = out_name.to_str().unwrap();
        let block_size = 9;
        let ret = bzp_de_compress_stream(in_name, out_name);
        assert_eq!(ret, bzp_error_io!());
    }
}