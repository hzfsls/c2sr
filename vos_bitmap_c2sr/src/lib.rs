pub(crate) mod vos_bitmap;

use crate::vos_bitmap::*;


// tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bitmap_vos_uint32_num() {
        assert_eq!(bitmap_vos_uint32_num!(0), 0);
        assert_eq!(bitmap_vos_uint32_num!(1), 1);
        assert_eq!(bitmap_vos_uint32_num!(31), 1);
        assert_eq!(bitmap_vos_uint32_num!(32), 1);
        assert_eq!(bitmap_vos_uint32_num!(33), 2);
        assert_eq!(bitmap_vos_uint32_num!(63), 2);
        assert_eq!(bitmap_vos_uint32_num!(64), 2);
        assert_eq!(bitmap_vos_uint32_num!(65), 3);
    }

    #[test]
    fn test_bitmap_idx_which_uint32() {
        assert_eq!(bitmap_idx_which_uint32!(0), 0);
        assert_eq!(bitmap_idx_which_uint32!(1), 0);
        assert_eq!(bitmap_idx_which_uint32!(31), 0);
        assert_eq!(bitmap_idx_which_uint32!(32), 1);
        assert_eq!(bitmap_idx_which_uint32!(33), 1);
        assert_eq!(bitmap_idx_which_uint32!(63), 1);
        assert_eq!(bitmap_idx_which_uint32!(64), 2);
        assert_eq!(bitmap_idx_which_uint32!(65), 2);
    }

    #[test]
    fn test_bitmap_idx_mod_uint32() {
        assert_eq!(bitmap_idx_mod_uint32!(0), 0);
        assert_eq!(bitmap_idx_mod_uint32!(1), 1);
        assert_eq!(bitmap_idx_mod_uint32!(31), 31);
        assert_eq!(bitmap_idx_mod_uint32!(32), 0);
        assert_eq!(bitmap_idx_mod_uint32!(33), 1);
        assert_eq!(bitmap_idx_mod_uint32!(63), 31);
        assert_eq!(bitmap_idx_mod_uint32!(64), 0);
        assert_eq!(bitmap_idx_mod_uint32!(65), 1);
    }
    
    #[test]
    fn test_bitmap_idx_mask_uint32() {
        assert_eq!(bitmap_idx_mask_uint32!(0), 1);
        assert_eq!(bitmap_idx_mask_uint32!(1), 2);
        assert_eq!(bitmap_idx_mask_uint32!(31), 1 << 31);
        assert_eq!(bitmap_idx_mask_uint32!(32), 1);
        assert_eq!(bitmap_idx_mask_uint32!(33), 2);
        assert_eq!(bitmap_idx_mask_uint32!(63), 1 << 31);
        assert_eq!(bitmap_idx_mask_uint32!(64), 1);
        assert_eq!(bitmap_idx_mask_uint32!(65), 2);
    }

    #[test]
    fn test_vos_lw_bit_mask() {
        assert_eq!(vos_lw_bit_mask!(0), 1);
        assert_eq!(vos_lw_bit_mask!(1), 2);
        assert_eq!(vos_lw_bit_mask!(31), 1 << 31);
        assert_eq!(vos_lw_bit_mask!(35), 1 << 35);
    }

    #[test]
    fn test_vos_lw_bit_mask_low() {
        assert_eq!(vos_lw_bit_mask_low!(0), 0);
        assert_eq!(vos_lw_bit_mask_low!(1), 1);
        assert_eq!(vos_lw_bit_mask_low!(31), 0x7fffffff);
        assert_eq!(vos_lw_bit_mask_low!(35), 0xffffffff);
    }

    #[test]
    fn test_vos_lw_bit_set() {
        let mut f = 0;
        vos_lw_bit_set!(f, 1);
        assert_eq!(f, 1);
        vos_lw_bit_set!(f, 2);
        assert_eq!(f, 3);
        vos_lw_bit_set!(f, 4);
        assert_eq!(f, 7);
    }

    #[test]
    fn test_vos_lw_bit_unset() {
        let mut f = 7;
        vos_lw_bit_unset!(f, 4);
        assert_eq!(f, 3);
        vos_lw_bit_unset!(f, 2);
        assert_eq!(f, 1);
        vos_lw_bit_unset!(f, 1);
        assert_eq!(f, 0);
    }

    #[test]
    fn test_vos_lw_bit_test() {
        let f = 0b101;
        assert_eq!(vos_lw_bit_test!(f, 1), true);
        assert_eq!(vos_lw_bit_test!(f, 2), false);
        assert_eq!(vos_lw_bit_test!(f, 4), true);
    }

    #[test]
    fn test_vos_lw_bitval_set() {
        let mut f = 0;
        vos_lw_bitval_set!(f, 0);
        assert_eq!(f, 1);
        vos_lw_bitval_set!(f, 1);
        assert_eq!(f, 3);
        vos_lw_bitval_set!(f, 2);
        assert_eq!(f, 7);
    }

    #[test]
    fn test_vos_lw_bitval_unset() {
        let mut f = 7;
        vos_lw_bitval_unset!(f, 2);
        assert_eq!(f, 3);
        vos_lw_bitval_unset!(f, 1);
        assert_eq!(f, 1);
        vos_lw_bitval_unset!(f, 0);
        assert_eq!(f, 0);
    }

    #[test]
    fn test_vos_lw_bitval_test() {
        let f = 0b101;
        assert_eq!(vos_lw_bitval_test!(f, 0), true);
        assert_eq!(vos_lw_bitval_test!(f, 1), false);
        assert_eq!(vos_lw_bitval_test!(f, 2), true);
    }

    #[test]
    fn test_vos_lw_bit_range_get() {
        let f = 0b101;
        assert_eq!(vos_lw_bit_range_get!(f, 0, 1), 1);
        assert_eq!(vos_lw_bit_range_get!(f, 1, 1), 0);
        assert_eq!(vos_lw_bit_range_get!(f, 2, 1), 1);
    }

    #[test]
    fn test_vos_lw_bit_range_clr() {
        let mut f = 0b101;
        vos_lw_bit_range_clr!(f, 0, 1);
        assert_eq!(f, 0b100);
        vos_lw_bit_range_clr!(f, 1, 1);
        assert_eq!(f, 0b100);
        vos_lw_bit_range_clr!(f, 2, 1);
        assert_eq!(f, 0);
        let mut f = 0b11100011;
        vos_lw_bit_range_clr!(f, 0, 1);
        assert_eq!(f, 0b11100010);
        vos_lw_bit_range_clr!(f, 1, 1);
        assert_eq!(f, 0b11100000);
        vos_lw_bit_range_clr!(f, 5, 3);
        assert_eq!(f, 0);
    }

    #[test]
    fn test_vos_lw_bit_range_set() {
        let mut f = 0b101;
        vos_lw_bit_range_set!(f, 0, 1, 0);
        assert_eq!(f, 0b100);
        vos_lw_bit_range_set!(f, 1, 1, 0);
        assert_eq!(f, 0b100);
        vos_lw_bit_range_set!(f, 2, 1, 0);
        assert_eq!(f, 0);
        let mut f = 0b11100011;
        vos_lw_bit_range_set!(f, 0, 1, 0);
        assert_eq!(f, 0b11100010);
        vos_lw_bit_range_set!(f, 1, 1, 0);
        assert_eq!(f, 0b11100000);
        vos_lw_bit_range_set!(f, 5, 3, 0);
        assert_eq!(f, 0);
        let mut f = 0b101;
        vos_lw_bit_range_set!(f, 0, 1, 1);
        assert_eq!(f, 0b101);
        vos_lw_bit_range_set!(f, 1, 1, 1);
        assert_eq!(f, 0b111);
        vos_lw_bit_range_set!(f, 2, 1, 1);
        assert_eq!(f, 0b111);
        let mut f = 0b11100011;
        vos_lw_bit_range_set!(f, 0, 1, 1);
        assert_eq!(f, 0b11100011);
        vos_lw_bit_range_set!(f, 1, 1, 1);
        assert_eq!(f, 0b11100011);
        vos_lw_bit_range_set!(f, 5, 3, 3);
        assert_eq!(f, 0b01100011);
    }

    #[test]
    fn test_vos_lw_bitmap_define() {
        vos_lw_bitmap_define!(aui_bitmap, 0);
        assert_eq!(aui_bitmap, []);
        vos_lw_bitmap_define!(aui_bitmap, 1);
        assert_eq!(aui_bitmap, [0]);
        vos_lw_bitmap_define!(aui_bitmap, 31);
        assert_eq!(aui_bitmap, [0]);
        vos_lw_bitmap_define!(aui_bitmap, 32);
        assert_eq!(aui_bitmap, [0]);
        vos_lw_bitmap_define!(aui_bitmap, 33);
        assert_eq!(aui_bitmap, [0, 0]);
        vos_lw_bitmap_define!(aui_bitmap, 63);
        assert_eq!(aui_bitmap, [0, 0]);
        vos_lw_bitmap_define!(aui_bitmap, 64);
        assert_eq!(aui_bitmap, [0, 0]);
        vos_lw_bitmap_define!(aui_bitmap, 65);
        assert_eq!(aui_bitmap, [0, 0, 0]);
    }

    #[test]
    fn test_vos_lw_bitmap_set() {
        let mut pui_bmp: [u32; 2] = [0, 0];
        vos_lw_bitmap_set!(&mut pui_bmp, 2);
        assert_eq!(pui_bmp, [4, 0]);
        vos_lw_bitmap_set!(&mut pui_bmp, 0);
        assert_eq!(pui_bmp, [5, 0]);
        vos_lw_bitmap_set!(&mut pui_bmp, 7);
        assert_eq!(pui_bmp, [133, 0]);
        vos_lw_bitmap_set!(&mut pui_bmp, 33);
        assert_eq!(pui_bmp, [133, 2]);
    }

    #[test]
    fn test_vos_lw_bitmap_unset() {
        let mut pui_bmp: [u32; 2] = [133, 2];
        vos_lw_bitmap_unset!(&mut pui_bmp, 33);
        assert_eq!(pui_bmp, [133, 0]);
        vos_lw_bitmap_unset!(&mut pui_bmp, 7);
        assert_eq!(pui_bmp, [5, 0]);
        vos_lw_bitmap_unset!(&mut pui_bmp, 0);
        assert_eq!(pui_bmp, [4, 0]);
        vos_lw_bitmap_unset!(&mut pui_bmp, 2);
        assert_eq!(pui_bmp, [0, 0]);
    }
    
    #[test]
    fn test_vos_lw_bitmap_test() {
        let pui_bmp: [u32; 2] = [133, 2];
        assert_eq!(vos_lw_bitmap_test!(&pui_bmp, 33), true);
        assert_eq!(vos_lw_bitmap_test!(&pui_bmp, 7), true);
        assert_eq!(vos_lw_bitmap_test!(&pui_bmp, 0), true);
        assert_eq!(vos_lw_bitmap_test!(&pui_bmp, 2), true);
        assert_eq!(vos_lw_bitmap_test!(&pui_bmp, 1), false);
    }

    #[test]
    fn test_vos_bitmapffs() {
        assert_eq!(vos_bitmapffs(0), 31);
    }

    #[test]
    fn test_vos_bitmapffb() {
        let mut pui_bmp = [1, 2];
        let mut pui_bmp1 = [0, 3];
        assert_eq!(vos_bitmapffb(&mut [], 1), 1);
        assert_eq!(vos_bitmapffb(&mut pui_bmp, 2), 0);
        assert_eq!(vos_bitmapffb(&mut pui_bmp, 0), 0);
        assert_eq!(vos_bitmapffb(&mut pui_bmp1, 1), 1);
    }

    #[test]
    fn test_vos_lw_bitmap_first1bitget() {
        let mut pui_bmp = [1, 2];
        let mut pui_bmp1 = [0, 3];
        assert_eq!(vos_lw_bitmap_first1bitget!(&mut [], 1), 1);
        assert_eq!(vos_lw_bitmap_first1bitget!(&mut pui_bmp, 2), 0);
        assert_eq!(vos_lw_bitmap_first1bitget!(&mut pui_bmp, 0), 0);
        assert_eq!(vos_lw_bitmap_first1bitget!(&mut pui_bmp1, 1), 1);
    }

    #[test]
    fn test_vos_bitmapff0b() {
        let mut pui_bmp = [0, 1];
        let mut pui_bmp1 = [1, 2];
        assert_eq!(vos_bitmapff0b(&mut [], 1), 1);
        assert_eq!(vos_bitmapff0b(&mut pui_bmp, 0), 0);
        assert_eq!(vos_bitmapff0b(&mut pui_bmp, 1), 0);
        assert_eq!(vos_bitmapff0b(&mut pui_bmp1, 1), 1);
    }

    #[test]
    fn test_vos_lw_bitmap_first0bitget() {
        let mut pui_bmp = [0, 1];
        let mut pui_bmp1 = [1, 2];
        assert_eq!(vos_lw_bitmap_first0bitget!(&mut [], 1), 1);
        assert_eq!(vos_lw_bitmap_first0bitget!(&mut pui_bmp, 0), 0);
        assert_eq!(vos_lw_bitmap_first0bitget!(&mut pui_bmp, 1), 0);
        assert_eq!(vos_lw_bitmap_first0bitget!(&mut pui_bmp1, 1), 1);
    }

    #[test]
    fn test_vos_reverse_byte_bits() {
        assert_eq!(vos_reverse_byte_bits(0b00000000), 0b00000000);
        assert_eq!(vos_reverse_byte_bits(0b00000001), 0b10000000);
        assert_eq!(vos_reverse_byte_bits(0b00000010), 0b01000000);
        assert_eq!(vos_reverse_byte_bits(0b00000011), 0b11000000);
        assert_eq!(vos_reverse_byte_bits(0b10110011), 0b11001101);
    }

    #[test]
    fn test_vos_bit_map_byte_get_low_free() {
        assert_eq!(vos_bit_map_byte_get_low_free(0b00000000), 8);
        assert_eq!(vos_bit_map_byte_get_low_free(0b00000001), 0);
        assert_eq!(vos_bit_map_byte_get_low_free(0b00000010), 1);
        assert_eq!(vos_bit_map_byte_get_low_free(0b00000011), 0);
        assert_eq!(vos_bit_map_byte_get_low_free(0b10110011), 0);
        assert_eq!(vos_bit_map_byte_get_low_free(0b10111000), 3);
    }

    #[test]
    fn test_vos_bit_map_get_free() {
        let mut puc_bitmap = [0, 0, 0, 0];
        assert_eq!(vos_bit_map_get_free(&mut [], 1), vos_bitmap_invalid_index!());
        assert_eq!(vos_bit_map_get_free(&mut [255, 255, 255, 255], 4), vos_bitmap_invalid_index!());
        assert_eq!(vos_bit_map_get_free(&mut puc_bitmap, 4), 8);
        puc_bitmap = [1, 0, 0, 0];
        assert_eq!(vos_bit_map_get_free(&mut puc_bitmap, 4), 7);
        puc_bitmap = [0, 0, 0, 1];
        assert_eq!(vos_bit_map_get_free(&mut puc_bitmap, 4), 8);
        puc_bitmap = [15, 0, 0, 0];
        assert_eq!(vos_bit_map_get_free(&mut puc_bitmap, 4), 4);
        puc_bitmap = [255, 0, 0, 0];
        assert_eq!(vos_bit_map_get_free(&mut puc_bitmap, 4), 16);
        puc_bitmap = [255, 127, 0, 0];
        assert_eq!(vos_bit_map_get_free(&mut puc_bitmap, 4), 9);
        assert_eq!(vos_bit_map_get_free(&mut [255, 255, 255, 255, 255, 255, 255, 255, 255, 0], 10), 80);
        assert_eq!(vos_bit_map_get_free(&mut [255, 255, 255, 255, 255, 255, 255, 255, 255, 1], 10), 79);
    }

    #[test]
    fn test_vos_bit_map_set() {
        let mut puc_bitmap = [0, 0, 0, 0];
        assert_eq!(vos_bit_map_set(&mut [], 1, 1), vos_error!());
        assert_eq!(vos_bit_map_set(&mut puc_bitmap, 1, 9), vos_error!());
        assert_eq!(vos_bit_map_set(&mut puc_bitmap, 4, 32), vos_error!());
        assert_eq!(vos_bit_map_set(&mut puc_bitmap, 4, 0), vos_ok!());
        assert_eq!(puc_bitmap, [0b010000000, 0, 0, 0]);
        assert_eq!(vos_bit_map_set(&mut puc_bitmap, 4, 1), vos_ok!());
        assert_eq!(puc_bitmap, [0b011000000, 0, 0, 0]);
        assert_eq!(vos_bit_map_set(&mut puc_bitmap, 16, 15), vos_ok!());
        assert_eq!(puc_bitmap, [0b011000000, 0b00000001, 0, 0]);
        assert_eq!(vos_bit_map_set(&mut puc_bitmap, 32, 31), vos_ok!());
        assert_eq!(puc_bitmap, [0b011000000, 0b00000001, 0, 0b00000001]);
    }

    // pub fn vos_bit_map_unset(puc_bitmap: &mut [u8], ui_bitmap_size: u32, ui_index: u32) -> u32 {
    //     let mut pc_bit_map_byte_set;
    //     let mut ui_bit_index = ui_index % vos_bitmap_byte_bits!();
    //     let mut ui_byte_index = ui_index / vos_bitmap_byte_bits!();
    //     let mut uc_bit_unset_flag;
    
    //     if puc_bitmap.is_empty() || ui_byte_index >= ui_bitmap_size {
    //         return vos_error!();
    //     }
    
    //     pc_bit_map_byte_set = &mut puc_bitmap[ui_byte_index as usize..];
    //     uc_bit_unset_flag = !(vos_bitmap_byte_bit_flag!() >> ui_bit_index);
    
    //     pc_bit_map_byte_set[0] &= uc_bit_unset_flag;
    
    //     vos_ok!()
    // }

    #[test]
    fn test_vos_bit_map_unset() {
        let mut puc_bitmap = [0b011000000, 0b00000001, 0, 0b00000001];
        assert_eq!(vos_bit_map_unset(&mut [], 1, 1), vos_error!());
        assert_eq!(vos_bit_map_unset(&mut puc_bitmap, 1, 9), vos_error!());
        assert_eq!(vos_bit_map_unset(&mut puc_bitmap, 4, 32), vos_error!());
        assert_eq!(vos_bit_map_unset(&mut puc_bitmap, 32, 31), vos_ok!());
        assert_eq!(puc_bitmap, [0b011000000, 0b00000001, 0, 0]);
        assert_eq!(vos_bit_map_unset(&mut puc_bitmap, 16, 15), vos_ok!());
        assert_eq!(puc_bitmap, [0b011000000, 0, 0, 0]);
        assert_eq!(vos_bit_map_unset(&mut puc_bitmap, 4, 1), vos_ok!());
        assert_eq!(puc_bitmap, [0b010000000, 0, 0, 0]);
        assert_eq!(vos_bit_map_unset(&mut puc_bitmap, 4, 0), vos_ok!());
        assert_eq!(puc_bitmap, [0, 0, 0, 0]);
    }
    
    #[test]
    fn test_vos_bit_map_byte_set() {
        let mut uc_byte = 0b00000000;
        vos_bit_map_byte_set(&mut uc_byte, 0, 0);
        assert_eq!(uc_byte, 0b10000000);
        vos_bit_map_byte_set(&mut uc_byte, 1, 3);
        assert_eq!(uc_byte, 0b11110000);
        vos_bit_map_byte_set(&mut uc_byte, 4, 6);
        assert_eq!(uc_byte, 0b11111110);
        vos_bit_map_byte_set(&mut uc_byte, 7, 7);
        assert_eq!(uc_byte, 0b11111111);
    }

    #[test]
    fn test_vos_bit_map_byte_unset() {
        let mut uc_byte = 0b11111111;
        vos_bit_map_byte_unset(&mut uc_byte, 0, 0);
        assert_eq!(uc_byte, 0b01111111);
        vos_bit_map_byte_unset(&mut uc_byte, 1, 3);
        assert_eq!(uc_byte, 0b00001111);
        vos_bit_map_byte_unset(&mut uc_byte, 4, 6);
        assert_eq!(uc_byte, 0b00000001);
        vos_bit_map_byte_unset(&mut uc_byte, 7, 7);
        assert_eq!(uc_byte, 0b00000000);
    }

    #[test]
    fn test_vos_bit_map_byte_test() {
        assert_eq!(vos_bit_map_byte_test(0b00000000, 0, 0), true);
        assert_eq!(vos_bit_map_byte_test(0b00000000, 1, 3), true);
        assert_eq!(vos_bit_map_byte_test(0b00000000, 4, 6), true);
        assert_eq!(vos_bit_map_byte_test(0b00000000, 7, 7), true);
        assert_eq!(vos_bit_map_byte_test(0b11111111, 0, 0), false);
        assert_eq!(vos_bit_map_byte_test(0b11111111, 1, 3), false);
        assert_eq!(vos_bit_map_byte_test(0b11111111, 4, 6), false);
        assert_eq!(vos_bit_map_byte_test(0b11111111, 7, 7), false);
        assert_eq!(vos_bit_map_byte_test(0b11111111, 0, 1), false);
        assert_eq!(vos_bit_map_byte_test(0b11111111, 1, 2), false);
        assert_eq!(vos_bit_map_byte_test(0b11111111, 2, 3), false);
        assert_eq!(vos_bit_map_byte_test(0b11100000, 0, 1), false);
        assert_eq!(vos_bit_map_byte_test(0b11100000, 1, 2), false);
        assert_eq!(vos_bit_map_byte_test(0b11100000, 2, 3), false);
        assert_eq!(vos_bit_map_byte_test(0b11100000, 3, 4), true);
        assert_eq!(vos_bit_map_byte_test(0b11100000, 1, 4), false);
    }

    #[test]
    fn test_vos_bit_map_byte_segment_test() {
        assert_eq!(vos_bit_map_byte_segment_test(&mut [0], 1), true);
        assert_eq!(vos_bit_map_byte_segment_test(&mut [1], 1), false);
        assert_eq!(vos_bit_map_byte_segment_test(&mut [0, 0], 2), true);
        assert_eq!(vos_bit_map_byte_segment_test(&mut [1, 0], 2), false);
        assert_eq!(vos_bit_map_byte_segment_test(&mut [0, 0, 0, 0], 4), true);
    }

    #[test]
    fn test_vos_bit_map_array_set() {
        let mut puc_bitmap = [0, 0, 0, 0];
        assert_eq!(vos_bit_map_array_set(&mut [], 1, 1, 1), vos_errno_inval!());
        assert_eq!(vos_bit_map_array_set(&mut puc_bitmap, 1, 1, 1), vos_errno_inval!());
        assert_eq!(vos_bit_map_array_set(&mut puc_bitmap, 4, 4, 1), vos_errno_inval!());
        assert_eq!(vos_bit_map_array_set(&mut puc_bitmap, 4, 0, 5), vos_errno_inval!());
        assert_eq!(vos_bit_map_array_set(&mut puc_bitmap, 4, 1, 3), vos_ok!());
        assert_eq!(puc_bitmap, [0b01110000, 0, 0, 0]);
        let mut puc_bitmap = [0, 0, 0, 0];
        assert_eq!(vos_bit_map_array_set(&mut puc_bitmap, 10, 2, 8), vos_ok!());
        assert_eq!(puc_bitmap, [0b00111111, 0b11000000, 0, 0]);
        let mut puc_bitmap = [0, 0, 0, 0];
        assert_eq!(vos_bit_map_array_set(&mut puc_bitmap, 17, 0, 17), vos_ok!());
        assert_eq!(puc_bitmap, [0b11111111, 0b11111111, 0b10000000, 0]);
        let mut puc_bitmap = [0, 0, 0, 0];
        assert_eq!(vos_bit_map_array_set(&mut puc_bitmap, 32, 0, 32), vos_ok!());
        assert_eq!(puc_bitmap, [0b11111111, 0b11111111, 0b11111111, 0b11111111]);
    }

    #[test]
    fn test_vos_bit_map_array_unset() {
        let mut puc_bitmap = [0b01110000, 0, 0, 0];
        assert_eq!(vos_bit_map_array_unset(&mut [], 1, 1, 1), vos_errno_inval!());
        assert_eq!(vos_bit_map_array_unset(&mut puc_bitmap, 1, 1, 1), vos_errno_inval!());
        assert_eq!(vos_bit_map_array_unset(&mut puc_bitmap, 4, 4, 1), vos_errno_inval!());
        assert_eq!(vos_bit_map_array_unset(&mut puc_bitmap, 4, 0, 5), vos_errno_inval!());
        assert_eq!(vos_bit_map_array_unset(&mut puc_bitmap, 4, 1, 3), vos_ok!());

        assert_eq!(puc_bitmap, [0, 0, 0, 0]);
        let mut puc_bitmap = [0b00111111, 0b11000000, 0, 0];
        assert_eq!(vos_bit_map_array_unset(&mut puc_bitmap, 10, 2, 8), vos_ok!());
        assert_eq!(puc_bitmap, [0, 0, 0, 0]);
        let mut puc_bitmap = [0b11111111, 0b11111111, 0b10000000, 0];
        assert_eq!(vos_bit_map_array_unset(&mut puc_bitmap, 17, 0, 17), vos_ok!());
        assert_eq!(puc_bitmap, [0, 0, 0, 0]);
        let mut puc_bitmap = [0b11111111, 0b11111111, 0b11111111, 0b11111111];
        assert_eq!(vos_bit_map_array_unset(&mut puc_bitmap, 32, 0, 32), vos_ok!());
        assert_eq!(puc_bitmap, [0, 0, 0, 0]);
    }

    #[test]
    fn test_vos_bit_map_array_test() {
        let mut puc_bitmap = [0b01110000, 0, 0, 0];
        assert_eq!(vos_bit_map_array_test(&mut puc_bitmap, 1, 1, 1), false);
        assert_eq!(vos_bit_map_array_test(&mut puc_bitmap, 4, 4, 1), false);
        assert_eq!(vos_bit_map_array_test(&mut puc_bitmap, 4, 0, 5), false);
        assert_eq!(vos_bit_map_array_test(&mut puc_bitmap, 4, 1, 3), false);
        let mut puc_bitmap = [0b00111111, 0b11000000, 0, 0];
        assert_eq!(vos_bit_map_array_test(&mut puc_bitmap, 10, 2, 8), false);
        assert_eq!(vos_bit_map_array_test(&mut puc_bitmap, 10, 0, 2), true);
        assert_eq!(vos_bit_map_array_test(&mut puc_bitmap, 20, 10, 2), true);
    }

    #[test]
    fn test_vos_check_enough_bits_in_one_byte() {
        let mut ui_start = 0;
        assert_eq!(vos_check_enough_bits_in_one_byte(0b00000000, 1, &mut ui_start), true);
        assert_eq!(ui_start, 0);
        assert_eq!(vos_check_enough_bits_in_one_byte(0b01010101, 2, &mut ui_start), false);
        assert_eq!(vos_check_enough_bits_in_one_byte(0b11001000, 3, &mut ui_start), true);
        assert_eq!(ui_start, 5);
        assert_eq!(vos_check_enough_bits_in_one_byte(0b11001100, 3, &mut ui_start), false);
    }

    #[test]
    fn test_vos_check_enough_bits_in_two_bytes() {
        let mut ui_start = 0;
        assert_eq!(vos_check_enough_bits_in_two_bytes(0b00000000, 0b00000000, 1, &mut ui_start), true);
        assert_eq!(ui_start, 0);
        assert_eq!(vos_check_enough_bits_in_two_bytes(0b01010101, 0b01010101, 2, &mut ui_start), false);
        assert_eq!(vos_check_enough_bits_in_two_bytes(0b11001001, 0b11001000, 3, &mut ui_start), false);
        assert_eq!(vos_check_enough_bits_in_two_bytes(0b11001100, 0b00000000, 10, &mut ui_start), true);
        assert_eq!(ui_start, 6);
    }

    #[test]
    fn test_vos_bit_map_get_piece_free_array() {
        let mut puc_bitmap = [0, 0, 0, 0];
        let mut ui_index = 0;
        assert_eq!(vos_bit_map_get_piece_free_array(&mut puc_bitmap, 4, 1, &mut ui_index), vos_ok!());
        assert_eq!(ui_index, 0);
        let mut puc_bitmap = [0b11110000, 0, 0, 0];
        let mut ui_index = 0;
        assert_eq!(vos_bit_map_get_piece_free_array(&mut puc_bitmap, 4, 1, &mut ui_index), vos_error_nodata!());
        assert_eq!(vos_bit_map_get_piece_free_array(&mut puc_bitmap, 8, 1, &mut ui_index), vos_ok!());
        assert_eq!(ui_index, (0 << 3) + 4);
        assert_eq!(vos_bit_map_get_piece_free_array(&mut puc_bitmap, 8, 4, &mut ui_index), vos_ok!());
        assert_eq!(ui_index, (0 << 3) + 4);
        assert_eq!(vos_bit_map_get_piece_free_array(&mut puc_bitmap, 8, 7, &mut ui_index), vos_error_nodata!());
        assert_eq!(vos_bit_map_get_piece_free_array(&mut puc_bitmap, 16, 7, &mut ui_index), vos_ok!());
        assert_eq!(ui_index, (0 << 3) + 4);

        let mut puc_bitmap = [0b11110000, 0b00011100, 0b00000011, 0];
        assert_eq!(vos_bit_map_get_piece_free_array(&mut puc_bitmap, 32, 8, &mut ui_index), vos_ok!());
        assert_eq!(ui_index, (1 << 3) + 6);

        let mut puc_bitmap = [0b11110000, 0b00011100, 0b00000111, 0];
        assert_eq!(vos_bit_map_get_piece_free_array(&mut puc_bitmap, 32, 8, &mut ui_index), vos_ok!());
        assert_eq!(ui_index, (3 << 3) + 0);
    }

    #[test]
    fn test_vos_bit_map_get_common_free_array() {
        let mut puc_bitmap = [0, 0, 0, 0];
        let mut ui_index = 0;
        assert_eq!(vos_bit_map_get_common_free_array(&mut puc_bitmap, 32, 32, &mut ui_index), vos_ok!());
        assert_eq!(ui_index, 0);
        let mut puc_bitmap = [0b11110000, 0, 0, 0];
        let mut ui_index = 0;
        assert_eq!(vos_bit_map_get_common_free_array(&mut puc_bitmap, 32, 29, &mut ui_index), vos_error_nodata!());
        assert_eq!(vos_bit_map_get_common_free_array(&mut puc_bitmap, 32, 28, &mut ui_index), vos_ok!());
        assert_eq!(ui_index, (0 << 3) + 4);
        let mut puc_bitmap = [0b11110000, 0b00000001, 0b00000011, 0];
        assert_eq!(vos_bit_map_get_common_free_array(&mut puc_bitmap, 32, 12, &mut ui_index), vos_error_nodata!());
        assert_eq!(vos_bit_map_get_common_free_array(&mut puc_bitmap, 32, 11, &mut ui_index), vos_ok!());
        assert_eq!(ui_index, (0 << 3) + 4);
    }

    #[test]
    fn test_vos_bit_map_get_free_array() {
        let mut puc_bitmap = [0b11110000, 0b00011100, 0b00000111, 0];
        let mut ui_index = 0;
        assert_eq!(vos_bit_map_get_free_array(&mut puc_bitmap, 32, 8, &mut ui_index), vos_ok!());
        assert_eq!(ui_index, (3 << 3) + 0);

        let mut puc_bitmap = [0b11110000, 0b00000001, 0b00000011, 0];
        let mut ui_index = 0;
        assert_eq!(vos_bit_map_get_free_array(&mut puc_bitmap, 32, 12, &mut ui_index), vos_error_nodata!());
        assert_eq!(vos_bit_map_get_free_array(&mut puc_bitmap, 32, 11, &mut ui_index), vos_ok!());
        assert_eq!(ui_index, (0 << 3) + 4);
    }
}