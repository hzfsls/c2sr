macro_rules! vos_ok { () => { 0 }; }

macro_rules! vos_error { () => { !0 }; }

macro_rules! vos_null_byte { () => { 0xFF }; }

macro_rules! vos_errno_lib_bcd_num_error { () => { 0x5FFFFFFF }; }

type _BCD = u8;

macro_rules! bcd_index_used_num { () => { 2 }; }

pub fn bcd_init(p_bcd: &mut [_BCD], uc_size: u8) {
    if p_bcd.is_empty() {
        return;
    }

    for i in 0..uc_size {
        if i % bcd_index_used_num!() == 0 {
            p_bcd[i as usize / bcd_index_used_num!()] |= 0xF0;
        } else {
            p_bcd[i as usize / bcd_index_used_num!()] |= 0x0F;
        }
    }
}

pub fn bcd_cat(p_to_bcd: &mut [_BCD], p_from_bcd: &mut [_BCD], uc_size: u8) -> u8 {
    let mut i: u8;
    let mut j: u8;
    let mut uc_char: u8;
    let mut uc_len: u8;

    if p_to_bcd.is_empty() {
        return 0;
    }

    i = bcd_len(p_to_bcd, uc_size);

    if p_from_bcd.is_empty() {
        return i;
    }

    if i >= uc_size {
        return i;
    }

    for j in 0..uc_size {
        if i >= uc_size {
            break;
        }

        if j % bcd_index_used_num!() != 0 {
            uc_char = p_from_bcd[j as usize / bcd_index_used_num!()] & 0x0F;
        } else {
            uc_char = (p_from_bcd[j as usize / bcd_index_used_num!()] & 0xF0) >> 4;
        }

        if uc_char == 0x0F {
            break;
        }

        if i % bcd_index_used_num!() != 0 {
            p_to_bcd[i as usize / bcd_index_used_num!()] &= 0xF0;
            p_to_bcd[i as usize / bcd_index_used_num!()] |= uc_char;
        } else {
            p_to_bcd[i as usize / bcd_index_used_num!()] &= 0x0F;
            p_to_bcd[i as usize / bcd_index_used_num!()] |= uc_char << 4;
        }

        i += 1;
    }

    uc_len = i;

    for i in uc_len..uc_size {
        if i % bcd_index_used_num!() == 0 {
            p_to_bcd[i as usize / bcd_index_used_num!()] |= 0xF0;
        } else {
            p_to_bcd[i as usize / bcd_index_used_num!()] |= 0x0F;
        }
    }

    uc_len
}

pub fn bcd_value(p_bcd: &[_BCD], uc_pos: u8) -> u8 {
    let uc_char: u8;

    if p_bcd.is_empty() {
        return 0xFF;
    }

    uc_char = p_bcd[uc_pos as usize / bcd_index_used_num!()];

    if uc_pos % bcd_index_used_num!() == 0 {
        return uc_char >> 4;
    }

    uc_char & 0x0F
}

pub fn bcd_len(p_bcd: &[_BCD], uc_max_len: u8) -> u8 {
    let mut i: u8;

    if p_bcd.is_empty() {
        return 0;
    }

    for i in 0..uc_max_len {
        if i % bcd_index_used_num!() == 0 {
            if p_bcd[i as usize / bcd_index_used_num!()] & 0xF0 == 0xF0 {
                return i;
            }
        } else {
            if p_bcd[i as usize / bcd_index_used_num!()] & 0x0F == 0x0F {
                return i;
            }
        }
    }

    uc_max_len
}

pub fn bcd_compare(p_bcd1: &[_BCD], p_bcd2: &mut [_BCD], uc_size: u8) -> i32 {
    let mut i: u8;
    let mut uc_len1: u8;
    let mut uc_len2: u8;

    if p_bcd1.is_empty() || p_bcd2.is_empty() {
        return 0;
    }

    uc_len1 = bcd_len(p_bcd1, uc_size);
    uc_len2 = bcd_len(p_bcd2, uc_size);

    if uc_len1 < uc_len2 {
        return -1;
    }

    if uc_len1 > uc_len2 {
        return 1;
    }

    for i in 0..uc_len1 {
        if i % bcd_index_used_num!() == 0 {
            if p_bcd1[i as usize / bcd_index_used_num!()] & 0xF0 == p_bcd2[i as usize / bcd_index_used_num!()] & 0xF0 {
                continue;
            }

            if p_bcd1[i as usize / bcd_index_used_num!()] & 0xF0 == 0xF0 {
                return -1;
            }

            if p_bcd2[i as usize / bcd_index_used_num!()] & 0xF0 == 0xF0 {
                return 1;
            }

            if p_bcd1[i as usize / bcd_index_used_num!()] & 0xF0 > p_bcd2[i as usize / bcd_index_used_num!()] & 0xF0 {
                return 1;
            }

            return -1;
        }

        if p_bcd1[i as usize / bcd_index_used_num!()] & 0x0F == p_bcd2[i as usize / bcd_index_used_num!()] & 0x0F {
            continue;
        }

        if p_bcd1[i as usize / bcd_index_used_num!()] & 0x0F == 0x0F {
            return -1;
        }

        if p_bcd2[i as usize / bcd_index_used_num!()] & 0x0F == 0x0F {
            return 1;
        }

        if p_bcd1[i as usize / bcd_index_used_num!()] & 0x0F > p_bcd2[i as usize / bcd_index_used_num!()] & 0x0F {
            return 1;
        }
        return -1;
    }
    0
}

pub fn bcd_copy(p_to_bcd: &mut [_BCD], p_from_bcd: &[_BCD], uc_size: u8) {
    let mut i: u8;

    if p_to_bcd.is_empty() || p_from_bcd.is_empty() {
        return;
    }

    if p_to_bcd.as_ptr() == p_from_bcd.as_ptr() {
        return;
    }

    if uc_size == 0 {
        p_to_bcd[0] = 0xFF;
        return;
    }

    if p_to_bcd.as_ptr() > p_from_bcd.as_ptr() {
        for i in (uc_size >> 1) + (uc_size & 1)..0 {
            p_to_bcd[i as usize - 1] = p_from_bcd[i as usize - 1];
        }
    } else {
        for i in 0..(uc_size >> 1) + (uc_size & 1) {
            p_to_bcd[i as usize] = p_from_bcd[i as usize];
        }
    }
}

pub fn bcd_minus(p_bcd1: &mut [_BCD], p_bcd2: &mut[_BCD], uc_size: u8, psi_result: &mut i32) -> u32 {
    let mut si_val1: i32;
    let mut si_val2: i32;
    let mut i: u8;
    let mut bcd_temp: _BCD;

    if p_bcd1.is_empty() || p_bcd2.is_empty() {
        return vos_error!();
    }

    *psi_result = 0;
    si_val1 = 0;
    for i in 0..uc_size {
        if i % bcd_index_used_num!() == 0 {
            bcd_temp = (p_bcd1[i as usize / bcd_index_used_num!()] & 0xF0) >> 4;
        } else {
            bcd_temp = p_bcd1[i as usize / bcd_index_used_num!()] & 0x0F;
        }

        if bcd_temp == 0x0F {
            break;
        }

        if bcd_temp > 0x09 {
            return vos_error!();
        }

        si_val1 = si_val1 * 10 + bcd_temp as i32;
    }

    si_val2 = 0;
    for i in 0..uc_size {
        if i % bcd_index_used_num!() == 0 {
            bcd_temp = (p_bcd2[i as usize / bcd_index_used_num!()] & 0xF0) >> 4;
        } else {
            bcd_temp = p_bcd2[i as usize / bcd_index_used_num!()] & 0x0F;
        }

        if bcd_temp == 0x0F {
            break;
        }

        if bcd_temp > 0x09 {
            return vos_error!();
        }

        si_val2 = si_val2 * 10 + bcd_temp as i32;
    }

    *psi_result = si_val1 - si_val2;
    vos_ok!()
}

pub fn bcd_clip(p_to_bcd: &mut [_BCD], p_from_bcd: &mut [_BCD], uc_size: u8, uc_pos: u8, uc_len: u8) {
    let mut i: u8;
    let mut j: u8;
    let mut uc_char: u8;

    if p_to_bcd.is_empty() || p_from_bcd.is_empty() {
        return;
    }

    i = 0;
    j = uc_pos;
    while i < uc_len && i < uc_size && j < uc_size {
        if j % bcd_index_used_num!() == 0 {
            uc_char = (p_from_bcd[j as usize / bcd_index_used_num!()] & 0xF0) >> 4;
        } else {
            uc_char = p_from_bcd[j as usize / bcd_index_used_num!()] & 0x0F;
        }

        if uc_char == 0x0F {
            break;
        }

        if i % bcd_index_used_num!() == 0 {
            p_to_bcd[i as usize / bcd_index_used_num!()] &= 0x0F;
            p_to_bcd[i as usize / bcd_index_used_num!()] |= uc_char << 4;
        } else {
            p_to_bcd[i as usize / bcd_index_used_num!()] &= 0xF0;
            p_to_bcd[i as usize / bcd_index_used_num!()] |= uc_char;
        }

        i += 1;
        j += 1;
    }

    while i < uc_size {
        if i % bcd_index_used_num!() == 0 {
            p_to_bcd[i as usize / bcd_index_used_num!()] |= 0xF0;
        } else {
            p_to_bcd[i as usize / bcd_index_used_num!()] |= 0x0F;
        }

        i += 1;
    }
}

pub fn bcd_to_string(p_bcd: &mut [u8], p_str: &mut [u8], us_max_len: u16) -> u16 {
    let mut i: u16;

    if p_bcd.is_empty() || p_str.is_empty() {
        return 0;
    }

    for i in 0..us_max_len {
        if i % bcd_index_used_num!() == 0 {
            if p_bcd[i as usize / bcd_index_used_num!()] & 0xF0 == 0xF0 {
                p_str[i as usize] = vos_null_byte!();
                return i;
            }
            p_str[i as usize] = (p_bcd[i as usize / bcd_index_used_num!()] >> 4) & 0x0F;
        } else {
            if p_bcd[i as usize / bcd_index_used_num!()] & 0x0F == 0x0F {
                p_str[i as usize] = vos_null_byte!();
                return i;
            }
            p_str[i as usize] = p_bcd[i as usize / bcd_index_used_num!()] & 0x0F;
        }
    }

    us_max_len
}

pub fn string_to_bcd(p_str: &mut [u8], p_bcd: &mut [u8], us_max_len: u16) -> u16 {
    let mut i: u16;

    if p_bcd.is_empty() || p_str.is_empty() {
        return 0;
    }
    
    i = 0;
    while i < us_max_len {
        if p_str[i as usize] == vos_null_byte!() {
            p_bcd[i as usize / bcd_index_used_num!()] = 0xFF;
            return i;
        }

        p_bcd[i as usize / bcd_index_used_num!()] = (p_str[i as usize] << 4) | 0x0F;

        i += 1;

        if p_str[i as usize] == vos_null_byte!() {
            return i;
        }

        p_bcd[i as usize / bcd_index_used_num!()] = (p_bcd[i as usize / bcd_index_used_num!()] & 0xF0) | (p_str[i as usize] & 0x0F);
    
        i += 1;
    }

    us_max_len
}

pub fn bcd_prefix(p_bcd_prefix: &mut [_BCD], p_bcd_num: &mut [_BCD], uc_size: u8) -> bool {
    let mut i: u8;
    let mut uc_len1: u8;
    let mut uc_len2: u8;

    if p_bcd_prefix.is_empty() || p_bcd_num.is_empty() {
        return false;
    }

    uc_len1 = bcd_len(p_bcd_prefix, uc_size);
    uc_len2 = bcd_len(p_bcd_num, uc_size);

    if uc_len1 > uc_len2 {
        return false;
    }

    for i in 0..uc_len1 {
        if i % bcd_index_used_num!() == 0 {
            if p_bcd_prefix[i as usize / bcd_index_used_num!()] & 0xF0 != p_bcd_num[i as usize / bcd_index_used_num!()] & 0xF0 {
                return false;
            }
        } else {
            if p_bcd_prefix[i as usize / bcd_index_used_num!()] & 0x0F != p_bcd_num[i as usize / bcd_index_used_num!()] & 0x0F {
                return false;
            }
        }
    }

    true
}

pub fn dec_to_bcd(ui_num: u32, uc_digits: u8, puc_str: &mut [u8]) -> u8 {
    let mut ul_max: u32 = 1;
    let mut i: u8;
    let mut uc_left_flag: u8 = 1;
    let mut uc_temp: u8 = 0;
    let mut puc_ptr: &mut [u8] = puc_str;
    let mut ui_num_tmp: u32 = ui_num;

    if uc_digits > 5 {
        return 0;
    }

    if puc_ptr.is_empty() {
        return 0;
    }

    for i in 0..uc_digits {
        ul_max *= 10;
    }

    if ui_num_tmp > ul_max {
        return 0;
    }

    ul_max /= 10;

    for i in 0..uc_digits {
        if ul_max != 0 {
            uc_temp = (ui_num_tmp / ul_max) as u8;
            ui_num_tmp %= ul_max;
            ul_max /= 10;
        }

        if uc_temp > 9 {
            return 0;
        }

        if uc_left_flag != 0 {
            puc_ptr[i as usize] = (uc_temp & 0x0F) << 4;
            uc_left_flag = 0;
        } else {
            puc_ptr[i as usize] |= uc_temp & 0x0F;
            uc_left_flag = 1;
        }
    }

    if uc_left_flag == 0 {
        puc_ptr[uc_digits as usize] |= 0x0F;
    }

    uc_digits
}

pub fn bcd_cat_fixed_point(p_to_bcd: &mut [_BCD], uc_begin_point: u8, p_from_bcd: &mut [_BCD], uc_copy_len: u8, uc_max_size: u8) -> u8 {
    let mut i: u8;
    let mut j: u8;
    let mut uc_char: u8;
    let mut uc_len: u8;

    if p_to_bcd.is_empty() || p_from_bcd.is_empty() {
        return uc_begin_point;
    }

    i = uc_begin_point;

    if i >= uc_max_size {
        return i;
    }

    j = 0;
    while i < uc_max_size && j < uc_copy_len {
        if j % bcd_index_used_num!() == 0 {
            uc_char = (p_from_bcd[j as usize / bcd_index_used_num!()] & 0xF0) >> 4;
        } else {
            uc_char = p_from_bcd[j as usize / bcd_index_used_num!()] & 0x0F;
        }

        if uc_char == 0x0F {
            break;
        }

        if i % bcd_index_used_num!() == 0 {
            p_to_bcd[i as usize / bcd_index_used_num!()] &= 0x0F;
            p_to_bcd[i as usize / bcd_index_used_num!()] |= uc_char << 4;
        } else {
            p_to_bcd[i as usize / bcd_index_used_num!()] &= 0xF0;
            p_to_bcd[i as usize / bcd_index_used_num!()] |= uc_char;
        }

        i += 1;
        j += 1;
    }

    uc_len = i;

    while i < uc_max_size {
        if i % bcd_index_used_num!() == 0 {
            p_to_bcd[i as usize / bcd_index_used_num!()] |= 0xF0;
        } else {
            p_to_bcd[i as usize / bcd_index_used_num!()] |= 0x0F;
        }

        i += 1;
    }

    uc_len
}

pub fn new_bcd_clip(p_to_bcd: &mut [_BCD], p_from_bcd: &mut [_BCD], uc_dest_bcd_size: u8, uc_src_bcd_size: u8, uc_pos: u8, uc_len: u8) {
    let mut i: u8;
    let mut j: u8;
    let mut uc_char: u8;

    if p_to_bcd.is_empty() || p_from_bcd.is_empty() {
        return;
    }

    if uc_pos > uc_src_bcd_size {
        return;
    }

    i = 0;
    j = uc_pos;

    while i < uc_len && i < uc_dest_bcd_size && j < uc_src_bcd_size {
        if j % 2 == 0 {
            uc_char = (p_from_bcd[j as usize / 2] & 0xF0) >> 4;
        } else {
            uc_char = p_from_bcd[j as usize / 2] & 0x0F;
        }

        if uc_char == 0x0F {
            break;
        }

        if i % 2 == 0 {
            p_to_bcd[i as usize / 2] &= 0x0F;
            p_to_bcd[i as usize / 2] |= uc_char << 4;
        } else {
            p_to_bcd[i as usize / 2] &= 0xF0;
            p_to_bcd[i as usize / 2] |= uc_char;
        }

        j += 1;
    }

    while i < uc_dest_bcd_size {
        if i % 2 == 0 {
            p_to_bcd[i as usize / 2] |= 0xF0;
        } else {
            p_to_bcd[i as usize / 2] |= 0x0F;
        }

        i += 1;
    }
}