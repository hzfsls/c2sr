macro_rules! bitmap_bits_per_vos_uint32 { () => { 32 }; }
pub(crate) use bitmap_bits_per_vos_uint32;


// [begin] bitmap_m_001

macro_rules! bitmap_vos_uint32_num {
    ($max_value:expr) => {
        ($max_value / bitmap_bits_per_vos_uint32!()) + if $max_value % bitmap_bits_per_vos_uint32!() != 0 { 1 } else { 0 }
    };
}

// [end]

pub(crate) use bitmap_vos_uint32_num;

// [begin] bitmap_m_002

macro_rules! bitmap_idx_which_uint32 {
    ($value:expr) => {
        $value / bitmap_bits_per_vos_uint32!()
    };
}

// [end]

pub(crate) use bitmap_idx_which_uint32;

// [begin] bitmap_m_003

macro_rules! bitmap_idx_mod_uint32 {
    ($value:expr) => {
        $value % bitmap_bits_per_vos_uint32!()
    };
}

// [end]

pub(crate) use bitmap_idx_mod_uint32;

// [begin] bitmap_m_004

macro_rules! bitmap_idx_mask_uint32 {
    ($value:expr) => {
        1 << bitmap_idx_mod_uint32!($value)
    };
}

// [end]

pub(crate) use bitmap_idx_mask_uint32;

// [begin] bitmap_m_005

macro_rules! vos_lw_bit_mask {
    ($l:expr) => {
        1 << $l
    };
}

// [end]

pub(crate) use vos_lw_bit_mask;

// [begin] bitmap_m_006

macro_rules! vos_lw_bit_mask_low {
    ($l:expr) => {
        if $l < bitmap_bits_per_vos_uint32!() {
            (1 << $l) - 1
        } else {
            0xffffffff as u32
        }
    };
}

// [end]

pub(crate) use vos_lw_bit_mask_low;

// [begin] bitmap_m_007

macro_rules! vos_lw_bit_set {
    ($f:expr, $b:expr) => {
        $f |= $b
    };
}

// [end]

pub(crate) use vos_lw_bit_set;

// [begin] bitmap_m_008

macro_rules! vos_lw_bit_unset {
    ($f:expr, $b:expr) => {
        $f &= !$b
    };
}

// [end]

pub(crate) use vos_lw_bit_unset;

// [begin] bitmap_m_009

macro_rules! vos_lw_bit_test {
    ($f:expr, $b:expr) => {
        0 != ($f & $b)
    };
}

// [end]

pub(crate) use vos_lw_bit_test;

// [begin] bitmap_m_010

macro_rules! vos_lw_bitval_set {
    ($f:expr, $v:expr) => {
        $f |= vos_lw_bit_mask!($v)
    };
}

// [end]

pub(crate) use vos_lw_bitval_set;

// [begin] bitmap_m_011

macro_rules! vos_lw_bitval_unset {
    ($f:expr, $v:expr) => {
        $f &= !vos_lw_bit_mask!($v)
    };
}

// [end]

pub(crate) use vos_lw_bitval_unset;

// [begin] bitmap_m_012

macro_rules! vos_lw_bitval_test {
    ($f:expr, $v:expr) => {
        ($f & vos_lw_bit_mask!($v)) != 0
    };
}

// [end]

pub(crate) use vos_lw_bitval_test;

// [begin] bitmap_m_013

macro_rules! vos_lw_bit_range_get {
    ($zone:expr, $bit_begin:expr, $bit_len:expr) => {
        ($zone >> $bit_begin) & vos_lw_bit_mask_low!($bit_len)
    };
}

// [end]

pub(crate) use vos_lw_bit_range_get;

// [begin] bitmap_m_014

macro_rules! vos_lw_bit_range_clr {
    ($zone:expr, $bit_begin:expr, $bit_len:expr) => {
        $zone &= !(vos_lw_bit_mask_low!($bit_len) << $bit_begin)
    };
}

// [end]

pub(crate) use vos_lw_bit_range_clr;

// [begin] bitmap_m_015

macro_rules! vos_lw_bit_range_set {
    ($zone:expr, $bit_begin:expr, $bit_len:expr, $val:expr) => {
        vos_lw_bit_range_clr!($zone, $bit_begin, $bit_len);
        $zone |= $val << $bit_begin
    };
}

// [end]

pub(crate) use vos_lw_bit_range_set;

// [begin] bitmap_m_016

macro_rules! vos_lw_bitmap_define {
    ($aui_bitmap:ident, $max_value:expr) => {
        let mut $aui_bitmap = [0u32; bitmap_vos_uint32_num!($max_value)];
    };
}

// [end]

pub(crate) use vos_lw_bitmap_define;

// [begin] bitmap_m_017

macro_rules! vos_lw_bitmap_set {
    ($aui_bitmap:expr, $value:expr) => {
        $aui_bitmap[bitmap_idx_which_uint32!($value) as usize] |= bitmap_idx_mask_uint32!($value)
    };
}

// [end]

pub(crate) use vos_lw_bitmap_set;

// [begin] bitmap_m_018

macro_rules! vos_lw_bitmap_unset {
    ($aui_bitmap:expr, $value:expr) => {
        $aui_bitmap[bitmap_idx_which_uint32!($value) as usize] &= !bitmap_idx_mask_uint32!($value)
    };
}

// [end]

pub(crate) use vos_lw_bitmap_unset;

// [begin] bitmap_m_019

macro_rules! vos_lw_bitmap_test {
    ($aui_bitmap:expr, $value:expr) => {
        (1 & ($aui_bitmap[bitmap_idx_which_uint32!($value) as usize] >> bitmap_idx_mod_uint32!($value))) != 0
    }
}

// [end]

pub(crate) use vos_lw_bitmap_test;

// [begin] bitmap_m_020

macro_rules! vos_lw_bitmap_first1bitget {
    ($aui_bitmap:expr, $max_val:expr) => {
        vos_bitmapffb($aui_bitmap, $max_val)
    };
}

// [end]

pub(crate) use vos_lw_bitmap_first1bitget;

// [begin] bitmap_m_021

macro_rules! vos_lw_bitmap_first0bitget {
    ($aui_bitmap:expr, $max_val:expr) => {
        vos_bitmapff0b($aui_bitmap, $max_val)
    };
}

// [end]

pub(crate) use vos_lw_bitmap_first0bitget;

macro_rules! vos_bitmap_byte_bits { () => { 8 }; }
pub(crate) use vos_bitmap_byte_bits;

macro_rules! vos_bitmap_byte_shift_step { () => { 3 }; }
pub(crate) use vos_bitmap_byte_shift_step;

macro_rules! vos_bitmap_byte_bit_flag { () => { 0x80 }; }
pub(crate) use vos_bitmap_byte_bit_flag;

macro_rules! vos_bitmap_invalid_index { () => { 0xFFFFFFFF }; }
pub(crate) use vos_bitmap_invalid_index;

macro_rules! vos_bitmap_double_byte_bits { () => { 16 }; }
pub(crate) use vos_bitmap_double_byte_bits;

macro_rules! vos_bitmap_single_floating_point_fraction { () => { 23 }; }
pub(crate) use vos_bitmap_single_floating_point_fraction;

macro_rules! vos_bitmap_single_floating_point_bias { () => { 0x7F }; }
pub(crate) use vos_bitmap_single_floating_point_bias;

// [begin] bitmap_f_001

pub fn vos_bitmapffs(ui_val: u32) -> u32 {
    let mut ui_num = 0;
    let mut ui_val_tmp = ui_val;
    if (ui_val_tmp & 0xffff) == 0 {
        ui_num += 16;
        ui_val_tmp >>= 16;
    }
    if (ui_val_tmp & 0xff) == 0 {
        ui_num += 8;
        ui_val_tmp >>= 8;
    }
    if (ui_val_tmp & 0xf) == 0 {
        ui_num += 4;
        ui_val_tmp >>= 4;
    }
    if (ui_val_tmp & 0x3) == 0 {
        ui_num += 2;
        ui_val_tmp >>= 2;
    }
    if (ui_val_tmp & 0x1) == 0 {
        ui_num += 1;
    }
    ui_num
}

// [end]

// [begin] bitmap_f_002

pub fn vos_bitmapffb(pui_bmp: &mut [u32], ui_max_val: u32) -> u32 {
    let mut uix = 0;
    let mut ui_val;
    let mut pui_bmp_tmp = pui_bmp;

    if pui_bmp_tmp.is_empty() {
        return ui_max_val;
    }

    while uix < ui_max_val {
        ui_val = pui_bmp_tmp[0];
        pui_bmp_tmp = &mut pui_bmp_tmp[1..];
        if ui_val != 0 {
            return vos_bitmapffs(ui_val) + uix;
        }
        uix += (std::mem::size_of_val(&pui_bmp_tmp[0]) << 3) as u32;
    }
    if uix > ui_max_val {
        uix = ui_max_val;
    }

    uix
}

// [end]

// [begin] bitmap_f_003

pub fn vos_bitmapff0b(pui_bmp: &mut [u32], ui_max_val: u32) -> u32 {
    let mut uix = 0;
    let mut ui_val;
    let mut pui_bmp_tmp = pui_bmp;

    if pui_bmp_tmp.is_empty() {
        return ui_max_val;
    }

    while uix < ui_max_val {
        ui_val = pui_bmp_tmp[0];
        pui_bmp_tmp = &mut pui_bmp_tmp[1..];
        ui_val = !ui_val;
        if ui_val != 0 {
            return vos_bitmapffs(ui_val) + uix;
        }
        uix += (std::mem::size_of_val(&pui_bmp_tmp[0]) << 3) as u32;
    }
    if uix > ui_max_val {
        uix = ui_max_val;
    }

    uix
}

// [end]

macro_rules! vos_null_byte { () => { 0xFF }; }
pub(crate) use vos_null_byte;

// [begin] bitmap_f_004

pub fn vos_reverse_byte_bits(uc_byte: u8) -> u8 {
    ((((((uc_byte as u32 * 0x0802) & 0x22110) | ((uc_byte as u32 * 0x8020) & 0x88440)) * 0x10101) >>
        vos_bitmap_double_byte_bits!()) & vos_null_byte!()) as u8
}

// [end]

// [begin] bitmap_f_005

pub fn vos_bit_map_byte_get_low_free(uc_byte: u8) -> u32 {
    let mut ui_bit_free_cnt;
    let mut ui_byte_tmp = uc_byte;
    let f_byte;

    if uc_byte == 0 {
        return vos_bitmap_byte_bits!();
    }
    f_byte = (ui_byte_tmp & ((-(ui_byte_tmp as i8)) as u8)) as f32;
    ui_bit_free_cnt = (f_byte.to_bits() >> vos_bitmap_single_floating_point_fraction!()) - vos_bitmap_single_floating_point_bias!();
    return ui_bit_free_cnt;
}

// [end]

// [begin] bitmap_f_006

pub fn vos_bit_map_get_free(puc_bitmap: &mut [u8], ui_bitmap_size: u32) -> u32 {
    let mut ui_get_index = vos_bitmap_invalid_index!();
    let mut ui_byte_loop: u32;
    let mut ui_bit_idx;
    let mut uc_byte_test;

    if puc_bitmap.is_empty() {
        return vos_bitmap_invalid_index!();
    }

    for ui_byte_loop in 0..ui_bitmap_size {
        uc_byte_test = puc_bitmap[ui_byte_loop as usize];
        if uc_byte_test == vos_null_byte!() {
            continue;
        }

        uc_byte_test = vos_reverse_byte_bits(uc_byte_test);
        ui_bit_idx = vos_bit_map_byte_get_low_free(uc_byte_test);
        ui_get_index = (ui_byte_loop * vos_bitmap_byte_bits!()) + ui_bit_idx;
        break;
    }

    ui_get_index
}

// [end]

macro_rules! vos_ok { () => { 0 }; }
pub(crate) use vos_ok;

macro_rules! vos_error { () => { !0 }; }
pub(crate) use vos_error;

macro_rules! vos_error_nodata { () => { 60 }; }
pub(crate) use vos_error_nodata;

macro_rules! vos_errno_inval { () => { 22 }; }
pub(crate) use vos_errno_inval;

// [begin] bitmap_f_007

pub fn vos_bit_map_set(puc_bitmap: &mut [u8], ui_bitmap_size: u32, ui_index: u32) -> u32 {
    let mut pc_bit_map_byte_set;
    let mut ui_bit_index = ui_index % vos_bitmap_byte_bits!();
    let mut ui_byte_index = ui_index / vos_bitmap_byte_bits!();
    let mut uc_bit_set_flag;
    if puc_bitmap.is_empty() || ui_byte_index >= ui_bitmap_size {
        return vos_error!();
    }

    pc_bit_map_byte_set = &mut puc_bitmap[ui_byte_index as usize..];
    uc_bit_set_flag = vos_bitmap_byte_bit_flag!() >> ui_bit_index;

    pc_bit_map_byte_set[0] |= uc_bit_set_flag;

    vos_ok!()
}

// [end]

// [begin] bitmap_f_008

pub fn vos_bit_map_unset(puc_bitmap: &mut [u8], ui_bitmap_size: u32, ui_index: u32) -> u32 {
    let mut pc_bit_map_byte_set;
    let mut ui_bit_index = ui_index % vos_bitmap_byte_bits!();
    let mut ui_byte_index = ui_index / vos_bitmap_byte_bits!();
    let mut uc_bit_unset_flag;

    if puc_bitmap.is_empty() || ui_byte_index >= ui_bitmap_size {
        return vos_error!();
    }

    pc_bit_map_byte_set = &mut puc_bitmap[ui_byte_index as usize..];
    uc_bit_unset_flag = !(vos_bitmap_byte_bit_flag!() >> ui_bit_index);

    pc_bit_map_byte_set[0] &= uc_bit_unset_flag;

    vos_ok!()
}

// [end]

// [begin] bitmap_f_009

pub fn vos_bit_map_byte_set(puc_byte: &mut u8, uc_start: u8, uc_end: u8) {
    let uc_byte_set_mask = ((0xFF >> uc_start) & (0xFF << (vos_bitmap_byte_bits!() - 1 - uc_end))) as u8;
    *puc_byte |= uc_byte_set_mask;
}

// [end]

// [begin] bitmap_f_010

pub fn vos_bit_map_byte_unset(puc_byte: &mut u8, uc_start: u8, uc_end: u8) {
    let uc_byte_unset_mask = !(((0xFF >> uc_start) & (0xFF << (vos_bitmap_byte_bits!() - 1 - uc_end))) as u8);
    *puc_byte &= uc_byte_unset_mask;
}

// [end]

// [begin] bitmap_f_011

pub fn vos_bit_map_byte_test(puc_byte: u8, uc_start: u8, uc_end: u8) -> bool {
    let uc_byte_test_mask = ((0xFF >> uc_start) & (0xFF << (vos_bitmap_byte_bits!() - 1 - uc_end))) as u8;
    (uc_byte_test_mask & puc_byte) == 0
}

// [end]

// [begin] bitmap_f_012

pub fn vos_bit_map_byte_segment_test(puc_byte_seg: &mut [u8], ui_seg_len: u32) -> bool {
    (puc_byte_seg[0] == 0) && (puc_byte_seg[1..].iter().all(|&x| x == puc_byte_seg[0]))
}

// [end]

// [begin] bitmap_f_013

pub fn vos_bit_map_array_set(puc_bitmap: &mut [u8], ui_bit_map_bit_size: u32, ui_index: u32, ui_array_size: u32) -> u32 {
    let ui_byte_head = ui_index / vos_bitmap_byte_bits!();
    let ui_byte_head_next = ui_byte_head + 1;
    let ui_byte_end = (ui_index + ui_array_size - 1) / vos_bitmap_byte_bits!();
    let ui_byte_end_prev = ui_byte_end - 1;
    let ui_byte_len = ui_byte_end_prev - ui_byte_head_next + 1;
    let uc_bit_head_index = (ui_index % vos_bitmap_byte_bits!()) as u8;
    let uc_bit_end_index = ((ui_index + ui_array_size - 1) % vos_bitmap_byte_bits!()) as u8;

    if puc_bitmap.is_empty() || ui_array_size == 0 || ui_index + ui_array_size > ui_bit_map_bit_size {
        return vos_errno_inval!();
    }

    if ui_byte_head == ui_byte_end {
        vos_bit_map_byte_set(&mut puc_bitmap[ui_byte_head as usize], uc_bit_head_index, uc_bit_end_index);
        return vos_ok!();
    }

    vos_bit_map_byte_set(&mut puc_bitmap[ui_byte_head as usize], uc_bit_head_index, vos_bitmap_byte_bits!() - 1);
    vos_bit_map_byte_set(&mut puc_bitmap[ui_byte_end as usize], 0, uc_bit_end_index);

    if ui_byte_end_prev >= ui_byte_head_next {
        puc_bitmap[ui_byte_head_next as usize..(ui_byte_head_next + ui_byte_len) as usize].fill(0xff);
    }

    vos_ok!()
}

// [end]

// [begin] bitmap_f_014

pub fn vos_bit_map_array_unset(puc_bitmap: &mut [u8], ui_bit_map_bit_size: u32, ui_index: u32, ui_array_size: u32) -> u32 {
    let ui_byte_start = ui_index / vos_bitmap_byte_bits!();
    let ui_byte_end = (ui_index + ui_array_size - 1) / vos_bitmap_byte_bits!();
    let ui_byte_start_next = ui_byte_start + 1;
    let ui_byte_end_prev = ui_byte_end - 1;
    let ui_byte_seg_size = ui_byte_end_prev - ui_byte_start_next + 1;
    let uc_bit_start_index = (ui_index % vos_bitmap_byte_bits!()) as u8;
    let uc_bit_end_index = ((ui_index + ui_array_size - 1) % vos_bitmap_byte_bits!()) as u8;

    if puc_bitmap.is_empty() || ui_array_size == 0 || ui_bit_map_bit_size - ui_array_size > ui_bit_map_bit_size || ui_index > ui_bit_map_bit_size - ui_array_size {
        return vos_errno_inval!();
    }

    if ui_byte_start == ui_byte_end {
        vos_bit_map_byte_unset(&mut puc_bitmap[ui_byte_start as usize], uc_bit_start_index, uc_bit_end_index);
        return vos_ok!();
    }

    vos_bit_map_byte_unset(&mut puc_bitmap[ui_byte_start as usize], uc_bit_start_index, vos_bitmap_byte_bits!() - 1);
    vos_bit_map_byte_unset(&mut puc_bitmap[ui_byte_end as usize], 0, uc_bit_end_index);

    if ui_byte_end_prev >= ui_byte_start_next {
        puc_bitmap[ui_byte_start_next as usize..(ui_byte_start_next + ui_byte_seg_size) as usize].fill(0);
    }

    vos_ok!()
}

// [end]

// [begin] bitmap_f_015

pub fn vos_bit_map_array_test(puc_bitmap: &mut [u8], ui_bit_map_bit_size: u32, ui_index: u32, ui_array_size: u32) -> bool {
    let ui_byte_start = ui_index / vos_bitmap_byte_bits!();
    let ui_byte_start_next = ui_byte_start + 1;
    let ui_byte_end = (ui_index + ui_array_size - 1) / vos_bitmap_byte_bits!();
    let ui_byte_end_prev = ui_byte_end - 1;
    let ui_byte_len = ui_byte_end_prev - ui_byte_start_next + 1;
    let uc_bit_start_index = (ui_index % vos_bitmap_byte_bits!()) as u8;
    let uc_bit_end_index = ((ui_index + ui_array_size - 1) % vos_bitmap_byte_bits!()) as u8;

    if puc_bitmap.is_empty() || ui_array_size == 0 || ui_index + ui_array_size > ui_bit_map_bit_size {
        return false;
    }

    if ui_byte_start == ui_byte_end {
        return vos_bit_map_byte_test(puc_bitmap[ui_byte_start as usize], uc_bit_start_index, uc_bit_end_index);
    }

    if vos_bit_map_byte_test(puc_bitmap[ui_byte_start as usize], uc_bit_start_index, vos_bitmap_byte_bits!() - 1) == false ||
        vos_bit_map_byte_test(puc_bitmap[ui_byte_end as usize], 0, uc_bit_end_index) == false {
        return false;
    }

    if ui_byte_end_prev >= ui_byte_start_next {
        if vos_bit_map_byte_segment_test(&mut puc_bitmap[ui_byte_start_next as usize..(ui_byte_start_next + ui_byte_len) as usize], ui_byte_len) == false {
            return false;
        }
    }

    true
}

// [end]

// [begin] bitmap_f_016

pub fn vos_check_enough_bits_in_one_byte(uc_byte: u8, ui_bit_size_need: u32, pui_start_idx: &mut u32) -> bool {
    let mut ui_start = 0;
    let mut b_flag = true;

    while !vos_bit_map_byte_test(uc_byte, ui_start as u8, (ui_start + ui_bit_size_need - 1) as u8) {
        ui_start += 1;
        if ui_start + ui_bit_size_need > vos_bitmap_byte_bits!() {
            b_flag = false;
            break;
        }
    }
    if b_flag {
        *pui_start_idx = ui_start;
    }
    b_flag
}

// [end]

// [begin] bitmap_f_017

pub fn vos_check_enough_bits_in_two_bytes(uc_first_byte: u8, uc_second_byte: u8, ui_bit_size_need: u32, pui_start_idx: &mut u32) -> bool {
    let ui_consecutive_zero_bits = vos_bit_map_byte_get_low_free(uc_first_byte);
    if vos_bit_map_byte_test(uc_second_byte, 0, (ui_bit_size_need - ui_consecutive_zero_bits - 1) as u8) {
        *pui_start_idx = vos_bitmap_byte_bits!() - ui_consecutive_zero_bits;
        return true;
    }
    false
}

// [end]

// [begin] bitmap_f_018

pub fn vos_bit_map_get_piece_free_array(puc_bitmap: &mut [u8], ui_bit_map_bit_size: u32, ui_array_size: u32, pui_index: &mut u32) -> u32 {
    let ui_bit_map_byte_size = (ui_bit_map_bit_size + vos_bitmap_byte_bits!() - 1) >> vos_bitmap_byte_shift_step!();
    let ui_loop_butt = ui_bit_map_byte_size - 1;
    let mut ui_byte_loop: u32;
    let mut uc_byte_test;
    let mut uc_next_byte;
    let mut ui_start_idx = 0;
    let mut b_array_get = false;

    for ui_byte_loop in 0..ui_loop_butt {
        uc_byte_test = puc_bitmap[ui_byte_loop as usize];
        if vos_check_enough_bits_in_one_byte(uc_byte_test, ui_array_size, &mut ui_start_idx) {
            *pui_index = (ui_byte_loop << vos_bitmap_byte_shift_step!()) + ui_start_idx;
            b_array_get = true;
            break;
        } else {
            uc_next_byte = puc_bitmap[(ui_byte_loop + 1) as usize];
            if vos_check_enough_bits_in_two_bytes(uc_byte_test, uc_next_byte, ui_array_size, &mut ui_start_idx) {
                *pui_index = (ui_byte_loop << vos_bitmap_byte_shift_step!()) + ui_start_idx;
                b_array_get = true;
                break;
            }
        }
    }

    if b_array_get && *pui_index + ui_array_size <= ui_bit_map_bit_size {
        return vos_ok!();
    }

    if vos_check_enough_bits_in_one_byte(puc_bitmap[ui_bit_map_byte_size as usize - 1], ui_array_size, &mut ui_start_idx) {
        *pui_index = (ui_loop_butt << vos_bitmap_byte_shift_step!()) + ui_start_idx;
        if *pui_index + ui_array_size <= ui_bit_map_bit_size {
            return vos_ok!();
        }
    }

    vos_error_nodata!()
}

// [end]

// [begin] bitmap_f_019

pub fn vos_bit_map_get_common_free_array(puc_bitmap: &mut [u8], ui_bit_map_bit_size: u32, ui_array_size: u32, pui_index: &mut u32) -> u32 {
    let ui_bit_map_byte_size = (ui_bit_map_bit_size + vos_bitmap_byte_bits!() - 1) / vos_bitmap_byte_bits!();
    let mut ui_zero_bits_tail;
    let mut ui_bytes;
    let mut ui_consecutive_zero_bits;
    let mut uc_byte;
    let mut ui_byte_loop: u32;

    for ui_byte_loop in 0..ui_bit_map_byte_size {
        uc_byte = puc_bitmap[ui_byte_loop as usize];
        ui_consecutive_zero_bits = vos_bit_map_byte_get_low_free(uc_byte);
        if (ui_byte_loop + 1) * vos_bitmap_byte_bits!() - ui_consecutive_zero_bits + ui_array_size > ui_bit_map_bit_size {
            break;
        }
        if ui_consecutive_zero_bits == 0 {
            continue;
        }
        if ui_consecutive_zero_bits >= ui_array_size {
            *pui_index = ui_byte_loop * vos_bitmap_byte_bits!() + vos_bitmap_byte_bits!() - ui_consecutive_zero_bits;
            return vos_ok!();
        }

        ui_bytes = (ui_array_size - ui_consecutive_zero_bits) / vos_bitmap_byte_bits!();
        if ui_bytes > 0 && !vos_bit_map_byte_segment_test(&mut puc_bitmap[(ui_byte_loop + 1) as usize..(ui_byte_loop + 1 + ui_bytes) as usize], ui_bytes) {
            continue;
        }
        ui_zero_bits_tail = ui_array_size - ui_consecutive_zero_bits - ui_bytes * vos_bitmap_byte_bits!();
        if ui_zero_bits_tail == 0 || vos_bit_map_byte_test(puc_bitmap[(ui_byte_loop + 1 + ui_bytes) as usize], 0, (ui_zero_bits_tail - 1) as u8) {
            *pui_index = ui_byte_loop * vos_bitmap_byte_bits!() + vos_bitmap_byte_bits!() - ui_consecutive_zero_bits;
            return vos_ok!();
        }
    }

    vos_error_nodata!()
}

// [end]

// [begin] bitmap_f_020

pub fn vos_bit_map_get_free_array(puc_bitmap: &mut [u8], ui_bit_map_bit_size: u32, ui_array_size: u32, pui_index: &mut u32) -> u32 {
    if puc_bitmap.is_empty() || ui_array_size == 0 || ui_array_size > ui_bit_map_bit_size {
        return vos_error!();
    }

    if ui_array_size < vos_bitmap_byte_bits!() {
        return vos_bit_map_get_piece_free_array(puc_bitmap, ui_bit_map_bit_size, ui_array_size, pui_index);
    } else {
        return vos_bit_map_get_common_free_array(puc_bitmap, ui_bit_map_bit_size, ui_array_size, pui_index);
    }
}

// [end]