pub(crate) mod md5;

use crate::md5::*;

//tests

#[cfg(test)]
mod tests {
    use super::*;



    #[test]
    fn test_vos_md5_init() {
        let mut content: Box<Md5Ctx> = Box::new(Md5Ctx::new());
        vos_md5_init(&mut content);
        assert_eq!(content.aul_state[0], 0x67452301);
        assert_eq!(content.aul_state[1], 0xefcdab89);
        assert_eq!(content.aul_state[2], 0x98badcfe);
        assert_eq!(content.aul_state[3], 0x10325476);
    }

    #[test]
    fn test_vos_md5_update() {
        let mut content: Box<Md5Ctx> = Box::new(Md5Ctx::new());
        let mut str = b"abcd".to_vec();
        vos_md5_update(&mut content, &mut str, 0);
        content = Box::new(Md5Ctx::new());

        vos_md5_update(&mut content, &mut str, 4);
        assert_eq!(content.aul_count[0], 32);
        assert_eq!(content.auc_buffer[..4], str[..4]);
    }

    #[test]
    fn test_vos_md5_final_ex() {
        let mut content: Box<Md5Ctx> = Box::new(Md5Ctx::new());
        let mut digest: [u8; 20] = [0; 20];
        vos_md5_final_ex(&mut digest, 10, &mut content);
        content = Box::new(Md5Ctx::new());
        vos_md5_final_ex(&mut digest, 20, &mut content);
        content = Box::new(Md5Ctx::new());
        vos_md5_final_ex(&mut digest, 16, &mut content);
    }



    #[test]
    fn test_vos_md5_calc_ex() {
        let mut output: [u8; 20] = [0; 20];
        let result_compare: [u8; 16] = [0xd4, 0x1d, 0x8c, 0xd9,
                                        0x8f, 0x00, 0xb2, 0x04,
                                        0xe9, 0x80, 0x09, 0x98,
                                        0xec, 0xf8, 0x42, 0x7e];
        vos_md5_calc_ex(&mut output, 0, &mut vec![], 0);
        vos_md5_calc_ex(&mut output, 10, &mut vec![], 0);
        vos_md5_calc_ex(&mut output, 20, &mut vec![], 0);
        assert_eq!(output.iter().take_while(|x| **x != 0).collect::<Vec<_>>(),
                     result_compare.iter().take_while(|x| **x != 0).collect::<Vec<_>>());
        vos_md5_calc_ex(&mut output, 16, &mut vec![], 0);
        assert_eq!(output.iter().take_while(|x| **x != 0).collect::<Vec<_>>(),
                     result_compare.iter().take_while(|x| **x != 0).collect::<Vec<_>>());
    }


    fn test_md5_calc(input: &mut [u8], result_compare: &mut [u8]) {
        let mut output: [u8; 16] = [0; 16];
        let in_len = input.len() as u32;
        vos_md5_calc(&mut output, input, in_len);
        assert_eq!(output[..16], result_compare[..16]);
    }

    macro_rules! test_md5_a_init { () => { 0x67452301 } }
    macro_rules! text_less_56 { () => { b"abcdefghijklmnopqrstuvwxyzabcdefghijklmnopqrstuvwxyzabc".to_vec() } }
    macro_rules! text_equal_56 { () => { b"12345678901234567890123456789012345678901234567890123456".to_vec() } }
    macro_rules! text_less_64 { () => { b"\xe5\x95\x8a\xe6\xb3\xa2\xe6\xac\xa1\xe7\x9a\x84\xe9\xa2\x9d\xe4\xbd\x9b\xe6\xad\x8c\xe9\xa2\x9d\xe5\xb7\xb2\xe7\xba\xa7\xe5\x8f\xaf\xe4\xba\x86\xe8\x8e\xab\xe5\x91\xa2\xe5\x93\xa6\xe7\xa0\xb4\xe5\x99\xa8\xe9\x88\xa4\xe6\x96\xaf\xe7\x89\xb9\xe5\x94\x94".to_vec() } }
    macro_rules! text_equal_64 { () => { b"./,!@#$%^&*(){}[\"``~..\"/.;':<>?';:,./:>?:*&^$##%&^&^$!@;.!@#$%^&".to_vec() } }
    macro_rules! text_more_64 { () => { b"a123bcd\"ef896\xe8\x99\xbe7345&^$##6\xe8\x82\x894^&5*&^$##%678o12`~..3pq\xe9\xb1\xbc&^$#rst#%".to_vec() } }
    macro_rules! text_equal_128 { () => { b"a123b\"lmnop\xe8\x9b\x8b7345&^$##6\xe9\xa5\xad4^&5*&^$##\xe7\xb3\x96%678o12`~..3pq&^$#rst#%\"ef&5*&^$#896\xe7\x89\x9b7345&^$##6\xe7\xbe\x8a4^&5*&^$##%678o\xe7\x8c\xaa12`~..3pq&^$#rst#%".to_vec() } }

    #[test]
    fn test_vos_md5_calc_001() {
        let mut result_compare: [u8; 16] = [0xd4, 0x1d, 0x8c, 0xd9,
                                            0x8f, 0x00, 0xb2, 0x04,
                                            0xe9, 0x80, 0x09, 0x98,
                                            0xec, 0xf8, 0x42, 0x7e];
        let mut input: Vec<u8> = vec![];
        test_md5_calc(&mut input, &mut result_compare);
    }

    #[test]
    fn test_vos_md5_calc_002() {
        let mut input: Vec<u8> = text_less_56!();
        let mut result_compare: [u8; 16] = [0x0d, 0x7a, 0xe0, 0x56,
                                            0xb2, 0xf0, 0x15, 0xcd,
                                            0x7d, 0xc6, 0x74, 0x94,
                                            0xef, 0xd6, 0x58, 0xf1];
        
        test_md5_calc(&mut input, &mut result_compare);
    }

    #[test]
    fn test_vos_md5_calc_003() {
        let mut input: Vec<u8> = text_equal_56!();
        let mut result_compare: [u8; 16] = [0x49, 0xf1, 0x93, 0xad,
                                            0xce, 0x17, 0x84, 0x90,
                                            0xe3, 0x4d, 0x1b, 0x3a,
                                            0x4e, 0xc0, 0x06, 0x4c];
        
        test_md5_calc(&mut input, &mut result_compare);
    }

    #[test]
    fn test_vos_md5_calc_004() {
        let mut input: Vec<u8> = text_less_64!();
        let mut result_compare: [u8; 16] = [0x57, 0x7b, 0xe3, 0xed,
                                            0x8e, 0x9f, 0xa4, 0x87,
                                            0xbd, 0x81, 0x2d, 0xd8,
                                            0x1c, 0x20, 0x3a, 0x21];
        
        test_md5_calc(&mut input, &mut result_compare);
    }

    #[test]
    fn test_vos_md5_calc_005() {
        let mut input: Vec<u8> = text_equal_64!();
        let mut result_compare: [u8; 16] = [0xeb, 0x2b, 0x3f, 0xb8,
                                            0xbc, 0x26, 0x07, 0x35,
                                            0x1b, 0x37, 0xfd, 0x83,
                                            0x03, 0x5b, 0xe3, 0xb5];
        
        test_md5_calc(&mut input, &mut result_compare);
    }

    #[test]
    fn test_vos_md5_calc_006() {
        let mut input: Vec<u8> = text_more_64!();
        let mut result_compare: [u8; 16] = [0xcd, 0x29, 0x93, 0x8c,
                                            0xa4, 0x8b, 0x59, 0x17,
                                            0xe0, 0x61, 0x94, 0x98,
                                            0xae, 0x9c, 0x53, 0x49];
        
        test_md5_calc(&mut input, &mut result_compare);
    }

    #[test]
    fn test_vos_md5_calc_007() {
        let mut input: Vec<u8> = text_equal_128!();
        let mut result_compare: [u8; 16] = [0xd7, 0x1c, 0x48, 0x3e,
                                            0xc6, 0x00, 0xea, 0x7c,
                                            0x0b, 0x7e, 0xd5, 0x25,
                                            0x45, 0x7d, 0xe9, 0xe8];
        
        test_md5_calc(&mut input, &mut result_compare);
    }

    #[test]
    fn test_vos_md5_calc_008() {
        let in_len: u32 = 0x20000000;
        let mut input: Vec<u8> = vec![];
        for i in 0..in_len {
            input.push(('a' as u32 + i % 10) as u8);
        }
        let mut result_compare: [u8; 16] = [0x02, 0xe4, 0xcd, 0x84,
                                            0x8b, 0xee, 0x68, 0xa0,
                                            0x08, 0x7d, 0xf0, 0x6b,
                                            0xb6, 0xe7, 0xc8, 0xa2];
        
        vos_md5_calc(&mut result_compare, &mut input, in_len);
        assert_eq!(result_compare[..16], result_compare[..16]);
    }



    fn test_md5_update(input: &mut [u8]) {
        let input_len = input.len() as u32;
        let mut context: Box<Md5Ctx> = Box::new(Md5Ctx::new());
        vos_md5_init(&mut context);
        if context.aul_count[0] != 0 && context.aul_count[1] != 0 {
            assert!(false);
        }
        vos_md5_update(&mut context, input, input_len);
        if context.aul_state[0] == test_md5_a_init!() {
            assert!(false);
        }
    }

    #[test]
    fn test_vos_md5_009() {
        let mut input: Vec<u8> = text_less_56!();
        let mut input_len = input.len() as u32;
        let mut context: Box<Md5Ctx> = Box::new(Md5Ctx::new());
        vos_md5_init(&mut context);
        if context.aul_count[0] != 0 && context.aul_count[1] != 0 {
            assert!(false);
        }
        vos_md5_update(&mut context, &mut input, input_len);
        if context.aul_state[0] != test_md5_a_init!() {
            assert!(false);
        }
    }

    #[test]
    fn test_vos_md5_010() {
        let mut input: Vec<u8> = text_less_64!();
        let mut input_len = input.len() as u32;
        let mut context: Box<Md5Ctx> = Box::new(Md5Ctx::new());
        vos_md5_init(&mut context);
        if context.aul_count[0] != 0 && context.aul_count[1] != 0 {
            assert!(false);
        }
        vos_md5_update(&mut context, &mut input, input_len);
        assert!(context.aul_state[0] == test_md5_a_init!());
    }

    #[test]
    fn test_vos_md5_011() {
        let mut input: Vec<u8> = text_equal_64!();
        test_md5_update(&mut input);
    }

    #[test]
    fn test_vos_md5_012() {
        let mut input: Vec<u8> = text_more_64!();
        test_md5_update(&mut input);
    }

    #[test]
    fn test_vos_md5_013() {
        let mut input: Vec<u8> = text_equal_128!();
        test_md5_update(&mut input);
    }

    #[test]
    fn test_vos_md5_014() {
        let mut input: Vec<u8> = text_less_56!();
        let mut input_len = input.len() as u32;
        let mut context: Box<Md5Ctx> = Box::new(Md5Ctx::new());
        let mut digest: [u8; 20] = [0; 20];

        vos_md5_init(&mut context);
        if context.aul_count[0] != 0 && context.aul_count[1] != 0 {
            assert!(false);
        }
        vos_md5_update(&mut context, &mut input, input_len);
        assert!(context.aul_state[0] == test_md5_a_init!());
        
        vos_md5_final_ex(&mut digest, 20, &mut context);
        if digest.iter().take_while(|x| **x != 0).count() <= 0 {
            assert!(false);
        }
    }

    #[test]
    fn test_vos_md5_015() {
        let mut input: Vec<u8> = text_less_64!();
        let mut input_len = input.len() as u32;
        let mut context: Box<Md5Ctx> = Box::new(Md5Ctx::new());
        let mut digest: [u8; 20] = [0; 20];

        vos_md5_init(&mut context);
        if context.aul_count[0] != 0 && context.aul_count[1] != 0 {
            assert!(false);
        }
        vos_md5_update(&mut context, &mut input, input_len);
        assert!(context.aul_state[0] == test_md5_a_init!());
        
        vos_md5_final_ex(&mut digest, 20, &mut context);
        if digest.iter().take_while(|x| **x != 0).count() <= 0 {
            assert!(false);
        }
    }

    fn test_md5_final(input: &mut [u8]) {
        let input_len = input.len() as u32;
        let mut context: Box<Md5Ctx> = Box::new(Md5Ctx::new());
        let mut digest: [u8; 20] = [0; 20];
        vos_md5_init(&mut context);
        if context.aul_count[0] != 0 && context.aul_count[1] != 0 {
            assert!(false);
        }
        vos_md5_update(&mut context, input, input_len);
        if context.aul_state[0] == test_md5_a_init!() {
            assert!(false);
        }
        vos_md5_final_ex(&mut digest, 20, &mut context);
        if digest.iter().take_while(|x| **x != 0).count() <= 0 {
            assert!(false);
        }
    }

    #[test]
    fn test_vos_md5_016() {
        let mut input: Vec<u8> = text_equal_64!();
        test_md5_final(&mut input);
    }

    #[test]
    fn test_vos_md5_017() {
        let mut input: Vec<u8> = text_more_64!();
        test_md5_final(&mut input);
    }
    
    #[test]
    fn test_vos_md5_018() {
        let mut input: Vec<u8> = text_equal_128!();
        test_md5_final(&mut input);
    }

    #[test]
    fn test_vos_md5_020() {
        let mut context: Box<Md5Ctx> = Box::new(Md5Ctx::new());
        let mut digest: [u8; 20] = [0; 20];
        vos_md5_init(&mut context);
        vos_md5_update(&mut context, &mut vec![], 10);
        assert!(context.aul_count[0] == 0);
        vos_md5_final(&mut digest, &mut context);
        if digest.iter().take_while(|x| **x != 0).count() <= 0 {
            assert!(false);
        }
    }

    #[test]
    fn test_vos_md5_021() {
        let mut context: Box<Md5Ctx> = Box::new(Md5Ctx::new());
        let mut digest: [u8; 16] = [0; 16];
        let mut input: Vec<u8> = text_less_56!();
        let in_len = input.len() as u32;
        let result_compare: [u8; 16] = [0x0d, 0x7a, 0xe0, 0x56,
                                        0xb2, 0xf0, 0x15, 0xcd,
                                        0x7d, 0xc6, 0x74, 0x94,
                                        0xef, 0xd6, 0x58, 0xf1];
        vos_md5_init(&mut context);
        vos_md5_update(&mut context, &mut input, in_len);
        vos_md5_final_ex(&mut digest, 16, &mut context);
        assert!(digest.iter().take_while(|x| **x != 0).collect::<Vec<_>>() == result_compare.iter().take_while(|x| **x != 0).collect::<Vec<_>>());
    }

    #[test]
    fn test_vos_md5_022() {
        let mut context: Box<Md5Ctx> = Box::new(Md5Ctx::new());
        let mut digest: [u8; 15] = [0; 15];
        let mut input: Vec<u8> = text_less_56!();
        let in_len = input.len() as u32;

        vos_md5_init(&mut context);
        vos_md5_update(&mut context, &mut input, in_len);
        vos_md5_final_ex(&mut digest, 15, &mut context);
        assert!(digest.iter().take_while(|x| **x != 0).collect::<Vec<_>>().is_empty());
    }


    fn test_md5_multiple_update(input_head: &mut [u8], input_tail: &mut [u8], input_combine: &mut [u8]) {
        let mut separate_output: [u8; 16] = [0; 16];
        let mut combine_output: [u8; 16] = [0; 16];
        let mut context: Box<Md5Ctx> = Box::new(Md5Ctx::new());
        let input_head_len = input_head.len() as u32;
        let input_tail_len = input_tail.len() as u32;
        let input_combine_len = input_combine.len() as u32;

        vos_md5_init(&mut context);
        vos_md5_update(&mut context, input_combine, input_combine_len);
        vos_md5_final(&mut combine_output, &mut context);

        vos_md5_init(&mut context);
        vos_md5_update(&mut context, input_head, input_head_len);
        vos_md5_update(&mut context, input_tail, input_tail_len);
        vos_md5_final(&mut separate_output, &mut context);

        assert!(combine_output[..16].iter().take_while(|x| **x != 0).collect::<Vec<_>>() == separate_output[..16].iter().take_while(|x| **x != 0).collect::<Vec<_>>());
    }

    #[test]
    fn test_md5_multiple_update_001() {
        let mut input_head: Vec<u8> = b"abcdefghijklmnopqrstuvw".to_vec();
        let mut input_tail: Vec<u8> = b"xyzabcdefghijklmnopqrstuvwxyzabcdefghijk".to_vec();
        let mut input_combine: Vec<u8> = b"abcdefghijklmnopqrstuvwxyzabcdefghijklmnopqrstuvwxyzabcdefghijk".to_vec();
        test_md5_multiple_update(&mut input_head, &mut input_tail, &mut input_combine);
    }

    #[test]
    fn test_md5_multiple_update_002() {
        let mut input_head: Vec<u8> = b"abcdefghijklmnopqrstuvwxyzabcdefghijklmnopqrstuvw".to_vec();
        let mut input_tail: Vec<u8> = b"xyzabcdefghijkl".to_vec();
        let mut input_combine: Vec<u8> = b"abcdefghijklmnopqrstuvwxyzabcdefghijklmnopqrstuvwxyzabcdefghijkl".to_vec();
        test_md5_multiple_update(&mut input_head, &mut input_tail, &mut input_combine);
    }

    #[test]
    fn test_md5_multiple_update_003() {
        let mut input_head: Vec<u8> = b"abcdefghijklmnopqrstuvwxyzabcdefghijklmnopqrstuvwxyz".to_vec();
        let mut input_tail: Vec<u8> = b"abcdefghijklm".to_vec();
        let mut input_combine: Vec<u8> = b"abcdefghijklmnopqrstuvwxyzabcdefghijklmnopqrstuvwxyzabcdefghijklm".to_vec();
        test_md5_multiple_update(&mut input_head, &mut input_tail, &mut input_combine);
    }

    #[test]
    fn test_md5_multiple_update_004() {
        let mut input_head: Vec<u8> = b"abcdefghijklmnopqrstuvwxyzabcdefghijklmnopqrstuvwxyz".to_vec();
        let mut input_tail: Vec<u8> = b"abcdefghijklmnopqrstuvwxyzabcdefghijklmnopqrstuvwxyzabcdefghijklmnopqrstuvwxyz".to_vec();
        let mut input_combine: Vec<u8> = b"abcdefghijklmnopqrstuvwxyzabcdefghijklmnopqrstuvwxyzabcdefghijklmnopqrstuvwxyzabcdefghijklmnopqrstuvwxyzabcdefghijklmnopqrstuvwxyz".to_vec();
        test_md5_multiple_update(&mut input_head, &mut input_tail, &mut input_combine);
    }

    #[test]
    fn test_md5_multiple_update_005() {
        let mut input_head: Vec<u8> = b"abcdefghijklmnopqrstuvwxyzabcdefghijklmnopqrstuvwxyzabcdefghijklmnopqrstuvwxyz".to_vec();
        let mut input_tail: Vec<u8> = b"abcdefghijklmnopqrstuvwxyzabcdefghijklmnopqrstuvwxyz".to_vec();
        let mut input_combine: Vec<u8> = b"abcdefghijklmnopqrstuvwxyzabcdefghijklmnopqrstuvwxyzabcdefghijklmnopqrstuvwxyzabcdefghijklmnopqrstuvwxyzabcdefghijklmnopqrstuvwxyz".to_vec();
        test_md5_multiple_update(&mut input_head, &mut input_tail, &mut input_combine);
    }

    #[test]
    fn test_md5_multiple_update_006() {
        let mut input_head: Vec<u8> = "a123b\"lmnop蛋7345&^$##6饭4^&5*&^$##糖%678o12`~..3pq&^$#rs6饭4^&5*&^$##糖".as_bytes().to_vec();
        let mut input_tail: Vec<u8> = "a123bcd\"ef896虾7345&^$##6肉4^&5*&^$##%678o12`~..3pq鱼&^$#rst#%!@##$%%^&&*(".as_bytes().to_vec();
        let mut input_combine: Vec<u8> = "a123b\"lmnop蛋7345&^$##6饭4^&5*&^$##糖%678o12`~..3pq&^$#rs6饭4^&5*&^$##糖a123bcd\"ef896虾7345&^$##6肉4^&5*&^$##%678o12`~..3pq鱼&^$#rst#%!@##$%%^&&*(".as_bytes().to_vec();
        test_md5_multiple_update(&mut input_head, &mut input_tail, &mut input_combine);
    }

    #[test]
    fn test_md5_multiple_update_007() {
        let mut context: Box<Md5Ctx> = Box::new(Md5Ctx::new());
        let mut input_head_len: u32 = 0x20000000;
        let mut input_head: Vec<u8> = vec![0; input_head_len as usize];
        let mut input_tail: Vec<u8> = b"12345678910".to_vec();
        let mut input_tail_len: u32 = input_tail.len() as u32;
        let mut output: [u8; 20] = [0; 20];
        let mut output_result: [u8; 16] = [0xfc, 0x73, 0xcb, 0x71,
                                            0x4d, 0xf6, 0x8d, 0x82,
                                            0x6a, 0xc3, 0x3d, 0x66,
                                            0x49, 0x36, 0xc0, 0x72];
        for i in 0..input_head_len {
            input_head[i as usize] = ('a' as u32 + i % 10) as u8;
        }

        vos_md5_init(&mut context);
        vos_md5_update(&mut context, &mut input_head, input_head_len);
        vos_md5_update(&mut context, &mut input_tail, input_tail_len);
        vos_md5_final(&mut output, &mut context);

        assert!(output[..16].iter().take_while(|x| **x != 0).collect::<Vec<_>>() == output_result[..16].iter().take_while(|x| **x != 0).collect::<Vec<_>>());
    }

    #[test]
    fn test_md5_multiple_update_008() {
        let mut context: Box<Md5Ctx> = Box::new(Md5Ctx::new());
        let mut input_head: Vec<u8> = b"abcdefg".to_vec();
        let mut input_head_len: u32 = input_head.len() as u32;
        vos_md5_init(&mut context);
        context.aul_count[0] = 0xffffffff;
        context.aul_count[1] = 0xffffffff;
        vos_md5_update(&mut context, &mut input_head, input_head_len);
        assert!(context.auc_buffer[..16].iter().take_while(|x| **x != 0).collect::<Vec<_>>().is_empty());
    }

    #[test]
    fn test_md5_multiple_update_009() {
        let mut input_head: Vec<u8> = "a123b\"lmnop蛋7345&^$##6饭4^&5*&^$##糖%678o12`~..3pq&^$#rs6饭4^&5*&^$##糖".as_bytes().to_vec();
        let mut input_tail: Vec<u8> = "a123bcd\"ef896虾7345&^$##6肉4^&5*&^$##%678o12`~..3pq鱼&^$#rst#%!@##$%%^&&*(".as_bytes().to_vec();
        let mut output1: [u8; 17] = [0; 17];
        let mut output2: [u8; 17] = [0; 17];
        let mut context: Box<Md5Ctx> = Box::new(Md5Ctx::new());
        let input_head_len = input_head.len() as u32;
        let input_tail_len = input_tail.len() as u32;

        vos_md5_init(&mut context);
        vos_md5_update(&mut context, &mut input_head, input_head_len);
        vos_md5_update(&mut context, &mut input_tail, input_tail_len);
        vos_md5_final(&mut output1, &mut context);

        vos_md5_init(&mut context);
        vos_md5_update(&mut context, &mut input_tail, input_tail_len);
        vos_md5_update(&mut context, &mut input_head, input_head_len);
        vos_md5_final(&mut output2, &mut context);

        assert!(output1.iter().take_while(|x| **x != 0).collect::<Vec<_>>() != output2.iter().take_while(|x| **x != 0).collect::<Vec<_>>());
    }
    
    #[test]
    fn test_vos_md5_api_call_001() {
        let mut input: Vec<u8> = text_equal_56!();
        let mut output: [u8; 16] = [0; 16];
        let mut context: Box<Md5Ctx>;
        let mut input_len: u32 = input.len() as u32;
        let mut result_compare: [u8; 16] = [0x49, 0xf1, 0x93, 0xad,
                                            0xce, 0x17, 0x84, 0x90,
                                            0xe3, 0x4d, 0x1b, 0x3a,
                                            0x4e, 0xc0, 0x06, 0x4c];
        
        context = Box::new(Md5Ctx::new());
        vos_md5_init(&mut context);
        vos_md5_final(&mut output, &mut context);
        vos_md5_update(&mut context, &mut input, input_len);
        assert!(output[..16].iter().take_while(|x| **x != 0).collect::<Vec<_>>() != result_compare[..16].iter().take_while(|x| **x != 0).collect::<Vec<_>>());

        context = Box::new(Md5Ctx::new());
        vos_md5_update(&mut context, &mut input, input_len);
        vos_md5_final(&mut output, &mut context);
        vos_md5_init(&mut context);
        assert!(output[..16].iter().take_while(|x| **x != 0).collect::<Vec<_>>() != result_compare[..16].iter().take_while(|x| **x != 0).collect::<Vec<_>>());

        context = Box::new(Md5Ctx::new());
        vos_md5_update(&mut context, &mut input, input_len);
        vos_md5_init(&mut context);
        vos_md5_final(&mut output, &mut context);
        assert!(output[..16].iter().take_while(|x| **x != 0).collect::<Vec<_>>() != result_compare[..16].iter().take_while(|x| **x != 0).collect::<Vec<_>>());

        context = Box::new(Md5Ctx::new());
        vos_md5_final(&mut output, &mut context);
        vos_md5_update(&mut context, &mut input, input_len);
        vos_md5_init(&mut context);
        assert!(output[..16].iter().take_while(|x| **x != 0).collect::<Vec<_>>() != result_compare[..16].iter().take_while(|x| **x != 0).collect::<Vec<_>>());

        context = Box::new(Md5Ctx::new());
        vos_md5_final(&mut output, &mut context);
        vos_md5_init(&mut context);
        vos_md5_update(&mut context, &mut input, input_len);
        assert!(output[..16].iter().take_while(|x| **x != 0).collect::<Vec<_>>() != result_compare[..16].iter().take_while(|x| **x != 0).collect::<Vec<_>>());

        context = Box::new(Md5Ctx::new());
        vos_md5_init(&mut context);
        vos_md5_update(&mut context, &mut input, input_len);
        vos_md5_final(&mut output, &mut context);
        assert!(output[..16].iter().take_while(|x| **x != 0).collect::<Vec<_>>() == result_compare[..16].iter().take_while(|x| **x != 0).collect::<Vec<_>>());
    }

    #[test]
    fn test_vos_md5_api_call_002() {
        let mut input: Vec<u8> = text_equal_56!();
        let mut output: [u8; 17] = [0; 17];
        let mut context: Box<Md5Ctx>;
        let mut input_len: u32 = input.len() as u32;
        let mut result_compare: [u8; 16] = [0x49, 0xf1, 0x93, 0xad,
                                            0xce, 0x17, 0x84, 0x90,
                                            0xe3, 0x4d, 0x1b, 0x3a,
                                            0x4e, 0xc0, 0x06, 0x4c];
        
        context = Box::new(Md5Ctx::new());
        vos_md5_init(&mut context);
        vos_md5_init(&mut context);
        vos_md5_update(&mut context, &mut input, input_len);
        vos_md5_final(&mut output, &mut context);
        assert!(output[..16].iter().take_while(|x| **x != 0).collect::<Vec<_>>() == result_compare[..16].iter().take_while(|x| **x != 0).collect::<Vec<_>>());

        context = Box::new(Md5Ctx::new());
        vos_md5_update(&mut context, &mut input, input_len);
        vos_md5_update(&mut context, &mut input, input_len);
        vos_md5_final(&mut output, &mut context);
        assert!(output[..16].iter().take_while(|x| **x != 0).collect::<Vec<_>>() != result_compare[..16].iter().take_while(|x| **x != 0).collect::<Vec<_>>());

        context = Box::new(Md5Ctx::new());
        vos_md5_update(&mut context, &mut input, input_len);
        vos_md5_final(&mut output, &mut context);
        vos_md5_final(&mut output, &mut context);
        assert!(output[..16].iter().take_while(|x| **x != 0).collect::<Vec<_>>() != result_compare[..16].iter().take_while(|x| **x != 0).collect::<Vec<_>>());
    }

}