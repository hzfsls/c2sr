
macro_rules! one_short_byte_num { () => { 2 }; }

macro_rules! vos_cycle_byte32_num { () => { 32 }; }

macro_rules! vos_inet_cksum_reduce {
    ($uiSum: expr) => {
        {
            $uiSum = ($uiSum >> 16) + ($uiSum & 0xffff);
            $uiSum = ($uiSum >> 16) + ($uiSum & 0xffff);
        }
    };
}

macro_rules! vos_md5_op1 {
    ($m: expr, $n: expr, $o: expr, $p: expr, $x: expr, $y: expr, $z: expr) => {
        {
            $m += ((!$n & $p) | ($n & $o)) + $x + $z;
            $m = ($m << $y) | (($m >> (32 - $y)) & 0xffffffff);
            $m += $n;
        }
    };
}

macro_rules! vos_md5_op2 {
    ($m: expr, $n: expr, $o: expr, $p: expr, $x: expr, $y: expr, $z: expr) => {
        {
            $m += ((!$p & $o) | ($p & $n)) + $x + $z;
            $m = ($m << $y) | (($m >> (32 - $y)) & 0xffffffff);
            $m += $n;
        }
    };
}

macro_rules! vos_md5_op3 {
    ($m: expr, $n: expr, $o: expr, $p: expr, $x: expr, $y: expr, $z: expr) => {
        {
            $m += ($n ^ $o ^ $p) + $x + $z;
            $m = ($m << $y) | (($m >> (32 - $y)) & 0xffffffff);
            $m += $n;
        }
    };
}

macro_rules! vos_md5_op4 {
    ($m: expr, $n: expr, $o: expr, $p: expr, $x: expr, $y: expr, $z: expr) => {
        {
            $m += ($o ^ ($n | !$p)) + $x + $z;
            $m = ($m << $y) | (($m >> (32 - $y)) & 0xffffffff);
            $m += $n;
        }
    };
}

macro_rules! vos_md5_a_init { () => { 0x67452301 }; }
macro_rules! vos_md5_b_init { () => { 0xefcdab89 }; }
macro_rules! vos_md5_c_init { () => { 0x98badcfe }; }
macro_rules! vos_md5_d_init { () => { 0x10325476 }; }

macro_rules! vos_md5_get {
    ($m: expr, $p: expr) => {
        {
            let mut ul_xtmp: u32;
            ul_xtmp = $p[0] as u32; $p = &mut $p[1..];
            ul_xtmp |= ($p[0] as u32) << 8; $p = &mut $p[1..];
            ul_xtmp |= ($p[0] as u32) << 16; $p = &mut $p[1..];
            ul_xtmp |= ($p[0] as u32) << 24; $p = &mut $p[1..];
            $m = ul_xtmp;
        }
    };
}

macro_rules! vos_md5_put {
    ($m: expr, $p: expr) => {
        {
            let mut ul_xtmp: u32 = $m;
            $p[0] = ul_xtmp as u8; $p = &mut $p[1..];
            $p[0] = (ul_xtmp >> 8) as u8; $p = &mut $p[1..];
            $p[0] = (ul_xtmp >> 16) as u8; $p = &mut $p[1..];
            $p[0] = (ul_xtmp >> 24) as u8; $p = &mut $p[1..];
        }
    };
}

macro_rules! fletcher_add {
    ($c0: expr, $c1: expr, $p_buf: expr) => {
        {
            $c0 += $p_buf[0] as i32;
            $c1 += $c0;
            $p_buf = &mut $p_buf[1..];
        }
    };
}

macro_rules! fletcher_add_16 {
    ($c0: expr, $c1: expr, $p_buf: expr) => {
        {
            fletcher_add!($c0, $c1, $p_buf);
            fletcher_add!($c0, $c1, $p_buf);
            fletcher_add!($c0, $c1, $p_buf);
            fletcher_add!($c0, $c1, $p_buf);
            fletcher_add!($c0, $c1, $p_buf);
            fletcher_add!($c0, $c1, $p_buf);
            fletcher_add!($c0, $c1, $p_buf);
            fletcher_add!($c0, $c1, $p_buf);
            fletcher_add!($c0, $c1, $p_buf);
            fletcher_add!($c0, $c1, $p_buf);
            fletcher_add!($c0, $c1, $p_buf);
            fletcher_add!($c0, $c1, $p_buf);
            fletcher_add!($c0, $c1, $p_buf);
            fletcher_add!($c0, $c1, $p_buf);
            fletcher_add!($c0, $c1, $p_buf);
            fletcher_add!($c0, $c1, $p_buf);
        }
    }
}

macro_rules! vos_md5_get_16 {
    ($pul_xp: expr, $puc_dp: expr) => {
        {
            vos_md5_get!($pul_xp[0], $puc_dp);
            $pul_xp = &mut $pul_xp[1..];
            vos_md5_get!($pul_xp[0], $puc_dp);
            $pul_xp = &mut $pul_xp[1..];
            vos_md5_get!($pul_xp[0], $puc_dp);
            $pul_xp = &mut $pul_xp[1..];
            vos_md5_get!($pul_xp[0], $puc_dp);
            $pul_xp = &mut $pul_xp[1..];
            vos_md5_get!($pul_xp[0], $puc_dp);
            $pul_xp = &mut $pul_xp[1..];
            vos_md5_get!($pul_xp[0], $puc_dp);
            $pul_xp = &mut $pul_xp[1..];
            vos_md5_get!($pul_xp[0], $puc_dp);
            $pul_xp = &mut $pul_xp[1..];
            vos_md5_get!($pul_xp[0], $puc_dp);
            $pul_xp = &mut $pul_xp[1..];
            vos_md5_get!($pul_xp[0], $puc_dp);
            $pul_xp = &mut $pul_xp[1..];
            vos_md5_get!($pul_xp[0], $puc_dp);
            $pul_xp = &mut $pul_xp[1..];
            vos_md5_get!($pul_xp[0], $puc_dp);
            $pul_xp = &mut $pul_xp[1..];
            vos_md5_get!($pul_xp[0], $puc_dp);
            $pul_xp = &mut $pul_xp[1..];
            vos_md5_get!($pul_xp[0], $puc_dp);
            $pul_xp = &mut $pul_xp[1..];
            vos_md5_get!($pul_xp[0], $puc_dp);
            $pul_xp = &mut $pul_xp[1..];
            vos_md5_get!($pul_xp[0], $puc_dp);
            $pul_xp = &mut $pul_xp[1..];
            vos_md5_get!($pul_xp[0], $puc_dp);
        }
    };
}

pub fn vos_inet_cksum(p_addr: &mut [u8], ui_len: u32) -> u16 {
    let mut ui_sum: u32 = 0;
    let mut i_left_len: i32 = ui_len as i32;
    let mut b_one_byte_left: bool = false;
    let mut auc_bytes_left: [u8; one_short_byte_num!()] = [0, 0];
    let mut pus_word: &mut [u16];
    let mut p_addr_tmp = p_addr;

    if p_addr_tmp.is_empty() {
        return (!ui_sum as u16) & 0xffff;
    }

    if p_addr_tmp.as_ptr() as usize & 1 != 0 {
        auc_bytes_left[0] = p_addr_tmp[0];
        p_addr_tmp = &mut p_addr_tmp[1..];
        i_left_len -= 1;
        b_one_byte_left = true;
    }

    pus_word = unsafe { std::slice::from_raw_parts_mut(p_addr_tmp.as_mut_ptr() as *mut u16, (p_addr_tmp.len() + 1) / 2) };
    while i_left_len >= vos_cycle_byte32_num!() {
        ui_sum += pus_word[0] as u32 + pus_word[1] as u32 + pus_word[2] as u32 + pus_word[3] as u32 +
            pus_word[4] as u32 + pus_word[5] as u32 + pus_word[6] as u32 + pus_word[7] as u32 +
            pus_word[8] as u32 + pus_word[9] as u32 + pus_word[10] as u32 + pus_word[11] as u32 +
            pus_word[12] as u32 + pus_word[13] as u32 + pus_word[14] as u32 + pus_word[15] as u32;
        i_left_len -= vos_cycle_byte32_num!();
        pus_word = &mut pus_word[16..];
    }

    while i_left_len >= 8 {
        ui_sum += pus_word[0] as u32 + pus_word[1] as u32 + pus_word[2] as u32 + pus_word[3] as u32;
        i_left_len -= 8;
        pus_word = &mut pus_word[4..];
    }

    if !b_one_byte_left && i_left_len == 0 {
        vos_inet_cksum_reduce!(ui_sum);
        return (!ui_sum as u16) & 0xffff;
    }

    vos_inet_cksum_reduce!(ui_sum);

    i_left_len -= 2;
    while i_left_len >= 0 {
        ui_sum += pus_word[0] as u32;
        pus_word = &mut pus_word[1..];
        i_left_len -= 2;
    }

    if b_one_byte_left {
        vos_inet_cksum_reduce!(ui_sum);
        ui_sum <<= 8;
        if i_left_len == -1 {
            auc_bytes_left[1] = pus_word[0] as u8;
        }
        ui_sum += u16::from_ne_bytes(auc_bytes_left) as u32;
    } else if i_left_len == -1 {
        auc_bytes_left[0] = pus_word[0] as u8;
        auc_bytes_left[1] = 0;
        ui_sum += u16::from_ne_bytes(auc_bytes_left) as u32;
    }

    vos_inet_cksum_reduce!(ui_sum);

    (!ui_sum as u16) & 0xffff
}

pub fn vos_iso_check_sum(p_pkt: &mut [u8], ui_len: u32, puc_cksum: &mut [u8], ui_cksum_len: u32) -> u32 {
    let mut puc_tmp = unsafe { std::slice::from_raw_parts_mut(p_pkt.as_ptr() as *mut u8, p_pkt.len()) };
    let mut ui_index1: u32;
    let mut ui_index2: u32;
    let mut ui_length: u32 = ui_len;
    let mut si_c0: i32 = 0;
    let mut si_c1: i32 = 0;
    let mut si_cnt: i32;

    if p_pkt.is_empty() && ui_len != 0 {
        return 0;
    }

    if !puc_cksum.is_empty() && ui_cksum_len >= std::mem::size_of::<u16>() as u32 {
        puc_cksum[0] = 0;
        puc_cksum[1] = 0;
    }

    si_cnt = (ui_length >> 12) as i32;
    for ui_index1 in 0..si_cnt {
        for ui_index2 in 0..256 {
            fletcher_add_16!(si_c0, si_c1, puc_tmp);
        }

        si_c0 %= 255;
        si_c1 %= 255;
    }

    si_cnt = ((ui_length & 4095) >> 4) as i32;

    for ui_index1 in 0..si_cnt {
        fletcher_add_16!(si_c0, si_c1, puc_tmp);
    }

    ui_length &= 15;

    while ui_length > 0 {
        fletcher_add!(si_c0, si_c1, puc_tmp);
        ui_length -= 1;
    }

    si_c0 %= 255;
    si_c1 %= 255;

    if !puc_cksum.is_empty() && ui_cksum_len >= std::mem::size_of::<u16>() as u32 {
        si_cnt = (((p_pkt.as_ptr() as usize + ui_len as usize - 1 - puc_cksum.as_ptr() as usize) as i32 * si_c0) - si_c1) % 255;
        if si_cnt <= 0 {
            si_cnt += 255;
        }
        si_c1 = 510 - si_cnt - si_c0;
        if si_c1 > 255 {
            si_c1 -= 255;
        }
        si_c0 = si_cnt;
        puc_cksum[0] = si_c0 as u8;
        puc_cksum[1] = si_c1 as u8;
    }

    ((si_c0 as u32) << 8) | (si_c1 as u32)
}

pub fn vos_iso_cksum(p_pkt: &mut [u8], ui_len: u32, puc_cksum: &mut [u8]) -> u32 {
    vos_iso_check_sum(p_pkt, ui_len, puc_cksum, std::mem::size_of::<u16>() as u32)
}

pub fn vos_cks_sum_rest_data_cal(pp_x: &mut &mut [u32], puc_dp: &mut [u8], ui_data_len: u32, ui_total_len: u32) -> i32 {
    let mut si_all_done: i32 = 0;
    let mut p_x: &mut [u32] = unsafe { std::slice::from_raw_parts_mut(pp_x.as_ptr() as *mut u32, pp_x.len()) };
    let mut pul_xp: &mut [u32] = pp_x;
    let mut ui_data_len_mod: u32;
    let mut puc_dp_tmp = puc_dp;

    ui_data_len_mod = ui_data_len >> 2;
    while ui_data_len_mod >= 1 {
        vos_md5_get!(pul_xp[0], puc_dp_tmp);
        pul_xp = &mut pul_xp[1..];
        ui_data_len_mod -= 1;
    }

    match ui_data_len & 0x3 {
        3 => {
            pul_xp[0] = 0x80000000 | puc_dp_tmp[0] as u32;
            puc_dp_tmp = &mut puc_dp_tmp[1..];
            pul_xp[0] |= (puc_dp_tmp[0] as u32) << 8;
            puc_dp_tmp = &mut puc_dp_tmp[1..];
            pul_xp[0] |= (puc_dp_tmp[0] as u32) << 16;
            puc_dp_tmp = &mut puc_dp_tmp[1..];
            pul_xp = &mut pul_xp[1..];
        }
        2 => {
            pul_xp[0] = 0x800000 | puc_dp_tmp[0] as u32;
            puc_dp_tmp = &mut puc_dp_tmp[1..];
            pul_xp[0] |= (puc_dp_tmp[0] as u32) << 8;
            puc_dp_tmp = &mut puc_dp_tmp[1..];
            pul_xp = &mut pul_xp[1..];
        }
        1 => {
            pul_xp[0] = 0x8000 | puc_dp_tmp[0] as u32;
            puc_dp_tmp = &mut puc_dp_tmp[1..];
            pul_xp = &mut pul_xp[1..];
        }
        _ => {
            pul_xp[0] = 0x80;
            pul_xp = &mut pul_xp[1..];
        }
    }

    if pul_xp.as_ptr() as usize >= p_x[15..].as_ptr() as usize {
        if pul_xp.as_ptr() as usize == p_x[15..].as_ptr() as usize {
            pul_xp[0] = 0;
        }
    } else {
        while (pul_xp.as_ptr() as usize) < (p_x[14..].as_ptr() as usize) {
            pul_xp[0] = 0;
            pul_xp = &mut pul_xp[1..];
        }
        pul_xp[0] = ((ui_total_len << 3) & 0xffffffff) as u32;
        pul_xp = &mut pul_xp[1..];
        pul_xp[0] = ((ui_total_len >> 29) & 0xffffffff) as u32;
        si_all_done = 1;
    }

    si_all_done
}

pub fn vos_cks_sum_res_cal_by_var(pui_results: &mut [u32], ui_var1: u32, ui_var2: u32, ui_var3: u32, ui_var4: u32) {
    pui_results[0] += ui_var1;
    pui_results[1] += ui_var2;
    pui_results[2] += ui_var3;
    pui_results[3] += ui_var4;
}

pub fn cks_md5_cksum_block(pv_data: &mut [u8], ul_data_len: u32,  ul_total_len: u32, si_incomplete: i32, pul_results: &mut [u32]) {
    let mut ul_var1: u32;
    let mut ul_var2: u32;
    let mut ul_var3: u32;
    let mut ul_var4: u32;

    let mut puc_dp = pv_data;
    let mut ul_d_len = ul_data_len;
    let mut pul_xp: &mut [u32];
    let mut aui_assist_arr: [u32; 64] = [0; 64];
    let mut si_all_done: i32 = 0;
    
    let mut pui_tmp = unsafe {
        std::slice::from_raw_parts_mut(pul_results.as_ptr() as *mut u32, pul_results.len())
    };

    ul_var1 = pui_tmp[0];
    pui_tmp = &mut pui_tmp[1..];
    ul_var2 = pui_tmp[0];
    pui_tmp = &mut pui_tmp[1..];
    ul_var3 = pui_tmp[0];
    pui_tmp = &mut pui_tmp[1..];
    ul_var4 = pui_tmp[0];
    
    while ul_d_len != 0 {
        pul_xp = aui_assist_arr.as_mut();
        if ul_d_len >= 64 {
            vos_md5_get_16!(pul_xp, puc_dp);
            ul_d_len -= 64;
        } else {
            si_all_done = vos_cks_sum_rest_data_cal(&mut pul_xp, puc_dp, ul_d_len, ul_total_len);
            ul_d_len = 0;
        }

        pul_xp = aui_assist_arr.as_mut();

        vos_md5_op1!(ul_var1, ul_var2, ul_var3, ul_var4, pul_xp[ 0],  7, 0xd76aa478);
        vos_md5_op1!(ul_var4, ul_var1, ul_var2, ul_var3, pul_xp[ 1], 12, 0xe8c7b756);
        vos_md5_op1!(ul_var3, ul_var4, ul_var1, ul_var2, pul_xp[ 2], 17, 0x242070db);
        vos_md5_op1!(ul_var2, ul_var3, ul_var4, ul_var1, pul_xp[ 3], 22, 0xc1bdceee);
        vos_md5_op1!(ul_var1, ul_var2, ul_var3, ul_var4, pul_xp[ 4],  7, 0xf57c0faf);
        vos_md5_op1!(ul_var4, ul_var1, ul_var2, ul_var3, pul_xp[ 5], 12, 0x4787c62a);
        vos_md5_op1!(ul_var3, ul_var4, ul_var1, ul_var2, pul_xp[ 6], 17, 0xa8304613);
        vos_md5_op1!(ul_var2, ul_var3, ul_var4, ul_var1, pul_xp[ 7], 22, 0xfd469501);
        vos_md5_op1!(ul_var1, ul_var2, ul_var3, ul_var4, pul_xp[ 8],  7, 0x698098d8);
        vos_md5_op1!(ul_var4, ul_var1, ul_var2, ul_var3, pul_xp[ 9], 12, 0x8b44f7af);
        vos_md5_op1!(ul_var3, ul_var4, ul_var1, ul_var2, pul_xp[10], 17, 0xffff5bb1);
        vos_md5_op1!(ul_var2, ul_var3, ul_var4, ul_var1, pul_xp[11], 22, 0x895cd7be);
        vos_md5_op1!(ul_var1, ul_var2, ul_var3, ul_var4, pul_xp[12],  7, 0x6b901122);
        vos_md5_op1!(ul_var4, ul_var1, ul_var2, ul_var3, pul_xp[13], 12, 0xfd987193);
        vos_md5_op1!(ul_var3, ul_var4, ul_var1, ul_var2, pul_xp[14], 17, 0xa679438e);
        vos_md5_op1!(ul_var2, ul_var3, ul_var4, ul_var1, pul_xp[15], 22, 0x49b40821);

        vos_md5_op2!(ul_var1, ul_var2, ul_var3, ul_var4, pul_xp[ 1],  5, 0xf61e2562);
        vos_md5_op2!(ul_var4, ul_var1, ul_var2, ul_var3, pul_xp[ 6],  9, 0xc040b340);
        vos_md5_op2!(ul_var3, ul_var4, ul_var1, ul_var2, pul_xp[11], 14, 0x265e5a51);
        vos_md5_op2!(ul_var2, ul_var3, ul_var4, ul_var1, pul_xp[ 0], 20, 0xe9b6c7aa);
        vos_md5_op2!(ul_var1, ul_var2, ul_var3, ul_var4, pul_xp[ 5],  5, 0xd62f105d);
        vos_md5_op2!(ul_var4, ul_var1, ul_var2, ul_var3, pul_xp[10],  9, 0x02441453);
        vos_md5_op2!(ul_var3, ul_var4, ul_var1, ul_var2, pul_xp[15], 14, 0xd8a1e681);
        vos_md5_op2!(ul_var2, ul_var3, ul_var4, ul_var1, pul_xp[ 4], 20, 0xe7d3fbc8);
        vos_md5_op2!(ul_var1, ul_var2, ul_var3, ul_var4, pul_xp[ 9],  5, 0x21e1cde6);
        vos_md5_op2!(ul_var4, ul_var1, ul_var2, ul_var3, pul_xp[14],  9, 0xc33707d6);
        vos_md5_op2!(ul_var3, ul_var4, ul_var1, ul_var2, pul_xp[ 3], 14, 0xf4d50d87);
        vos_md5_op2!(ul_var2, ul_var3, ul_var4, ul_var1, pul_xp[ 8], 20, 0x455a14ed);
        vos_md5_op2!(ul_var1, ul_var2, ul_var3, ul_var4, pul_xp[13],  5, 0xa9e3e905);
        vos_md5_op2!(ul_var4, ul_var1, ul_var2, ul_var3, pul_xp[ 2],  9, 0xfcefa3f8);
        vos_md5_op2!(ul_var3, ul_var4, ul_var1, ul_var2, pul_xp[ 7], 14, 0x676f02d9);
        vos_md5_op2!(ul_var2, ul_var3, ul_var4, ul_var1, pul_xp[12], 20, 0x8d2a4c8a);

        vos_md5_op3!(ul_var1, ul_var2, ul_var3, ul_var4, pul_xp[ 5],  4, 0xfffa3942);
        vos_md5_op3!(ul_var4, ul_var1, ul_var2, ul_var3, pul_xp[ 8], 11, 0x8771f681);
        vos_md5_op3!(ul_var3, ul_var4, ul_var1, ul_var2, pul_xp[11], 16, 0x6d9d6122);
        vos_md5_op3!(ul_var2, ul_var3, ul_var4, ul_var1, pul_xp[14], 23, 0xfde5380c);
        vos_md5_op3!(ul_var1, ul_var2, ul_var3, ul_var4, pul_xp[ 1],  4, 0xa4beea44);
        vos_md5_op3!(ul_var4, ul_var1, ul_var2, ul_var3, pul_xp[ 4], 11, 0x4bdecfa9);
        vos_md5_op3!(ul_var3, ul_var4, ul_var1, ul_var2, pul_xp[ 7], 16, 0xf6bb4b60);
        vos_md5_op3!(ul_var2, ul_var3, ul_var4, ul_var1, pul_xp[10], 23, 0xbebfbc70);
        vos_md5_op3!(ul_var1, ul_var2, ul_var3, ul_var4, pul_xp[13],  4, 0x289b7ec6);
        vos_md5_op3!(ul_var4, ul_var1, ul_var2, ul_var3, pul_xp[ 0], 11, 0xeaa127fa);
        vos_md5_op3!(ul_var3, ul_var4, ul_var1, ul_var2, pul_xp[ 3], 16, 0xd4ef3085);
        vos_md5_op3!(ul_var2, ul_var3, ul_var4, ul_var1, pul_xp[ 6], 23, 0x04881d05);
        vos_md5_op3!(ul_var1, ul_var2, ul_var3, ul_var4, pul_xp[ 9],  4, 0xd9d4d039);
        vos_md5_op3!(ul_var4, ul_var1, ul_var2, ul_var3, pul_xp[12], 11, 0xe6db99e5);
        vos_md5_op3!(ul_var3, ul_var4, ul_var1, ul_var2, pul_xp[15], 16, 0x1fa27cf8);
        vos_md5_op3!(ul_var2, ul_var3, ul_var4, ul_var1, pul_xp[ 2], 23, 0xc4ac5665);

        vos_md5_op4!(ul_var1, ul_var2, ul_var3, ul_var4, pul_xp[ 0],  6, 0xf4292244);
        vos_md5_op4!(ul_var4, ul_var1, ul_var2, ul_var3, pul_xp[ 7], 10, 0x432aff97);
        vos_md5_op4!(ul_var3, ul_var4, ul_var1, ul_var2, pul_xp[14], 15, 0xab9423a7);
        vos_md5_op4!(ul_var2, ul_var3, ul_var4, ul_var1, pul_xp[ 5], 21, 0xfc93a039);
        vos_md5_op4!(ul_var1, ul_var2, ul_var3, ul_var4, pul_xp[12],  6, 0x655b59c3);
        vos_md5_op4!(ul_var4, ul_var1, ul_var2, ul_var3, pul_xp[ 3], 10, 0x8f0ccc92);
        vos_md5_op4!(ul_var3, ul_var4, ul_var1, ul_var2, pul_xp[10], 15, 0xffeff47d);
        vos_md5_op4!(ul_var2, ul_var3, ul_var4, ul_var1, pul_xp[ 1], 21, 0x85845dd1);
        vos_md5_op4!(ul_var1, ul_var2, ul_var3, ul_var4, pul_xp[ 8],  6, 0x6fa87e4f);
        vos_md5_op4!(ul_var4, ul_var1, ul_var2, ul_var3, pul_xp[15], 10, 0xfe2ce6e0);
        vos_md5_op4!(ul_var3, ul_var4, ul_var1, ul_var2, pul_xp[ 6], 15, 0xa3014314);
        vos_md5_op4!(ul_var2, ul_var3, ul_var4, ul_var1, pul_xp[13], 21, 0x4e0811a1);
        vos_md5_op4!(ul_var1, ul_var2, ul_var3, ul_var4, pul_xp[ 4],  6, 0xf7537e82);
        vos_md5_op4!(ul_var4, ul_var1, ul_var2, ul_var3, pul_xp[11], 10, 0xbd3af235);
        vos_md5_op4!(ul_var3, ul_var4, ul_var1, ul_var2, pul_xp[ 2], 15, 0x2ad7d2bb);
        vos_md5_op4!(ul_var2, ul_var3, ul_var4, ul_var1, pul_xp[ 9], 21, 0xeb86d391);

        ul_var1 += pul_results[0];
        pul_results[0] = ul_var1;

        ul_var2 += pul_results[1];
        pul_results[1] = ul_var2;

        ul_var3 += pul_results[2];
        pul_results[2] = ul_var3;

        ul_var4 += pul_results[3];
        pul_results[3] = ul_var4;
    }

    if (si_incomplete == 0) && (si_all_done == 0) {
        let ul_x0 = if (ul_data_len & 0x3f) != 0 { 0 } else { 0x80 };
        let ul_x14 = ((ul_total_len << 3) & 0xffffffff) as u32;
        let ul_x15 = ((ul_total_len >> 29) & 0xffffffff) as u32;

        vos_md5_op1!(ul_var1, ul_var2, ul_var3, ul_var4, ul_x0,  7, 0xd76aa478);
        vos_md5_op1!(ul_var4, ul_var1, ul_var2, ul_var3, 0, 12, 0xe8c7b756);
        vos_md5_op1!(ul_var3, ul_var4, ul_var1, ul_var2, 0, 17, 0x242070db);
        vos_md5_op1!(ul_var2, ul_var3, ul_var4, ul_var1, 0, 22, 0xc1bdceee);
        vos_md5_op1!(ul_var1, ul_var2, ul_var3, ul_var4, 0,  7, 0xf57c0faf);
        vos_md5_op1!(ul_var4, ul_var1, ul_var2, ul_var3, 0, 12, 0x4787c62a);
        vos_md5_op1!(ul_var3, ul_var4, ul_var1, ul_var2, 0, 17, 0xa8304613);
        vos_md5_op1!(ul_var2, ul_var3, ul_var4, ul_var1, 0, 22, 0xfd469501);
        vos_md5_op1!(ul_var1, ul_var2, ul_var3, ul_var4, 0,  7, 0x698098d8);
        vos_md5_op1!(ul_var4, ul_var1, ul_var2, ul_var3, 0, 12, 0x8b44f7af);
        vos_md5_op1!(ul_var3, ul_var4, ul_var1, ul_var2, 0, 17, 0xffff5bb1);
        vos_md5_op1!(ul_var2, ul_var3, ul_var4, ul_var1, 0, 22, 0x895cd7be);
        vos_md5_op1!(ul_var1, ul_var2, ul_var3, ul_var4, 0,  7, 0x6b901122);
        vos_md5_op1!(ul_var4, ul_var1, ul_var2, ul_var3, 0, 12, 0xfd987193);
        vos_md5_op1!(ul_var3, ul_var4, ul_var1, ul_var2, ul_x14, 17, 0xa679438e);
        vos_md5_op1!(ul_var2, ul_var3, ul_var4, ul_var1, ul_x15, 22, 0x49b40821);

        vos_md5_op2!(ul_var1, ul_var2, ul_var3, ul_var4, 0,  5, 0xf61e2562);
        vos_md5_op2!(ul_var4, ul_var1, ul_var2, ul_var3, 0,  9, 0xc040b340);
        vos_md5_op2!(ul_var3, ul_var4, ul_var1, ul_var2, 0, 14, 0x265e5a51);
        vos_md5_op2!(ul_var2, ul_var3, ul_var4, ul_var1, ul_x0, 20, 0xe9b6c7aa);
        vos_md5_op2!(ul_var1, ul_var2, ul_var3, ul_var4, 0,  5, 0xd62f105d);
        vos_md5_op2!(ul_var4, ul_var1, ul_var2, ul_var3, 0,  9, 0x02441453);
        vos_md5_op2!(ul_var3, ul_var4, ul_var1, ul_var2, ul_x15, 14, 0xd8a1e681);
        vos_md5_op2!(ul_var2, ul_var3, ul_var4, ul_var1, 0, 20, 0xe7d3fbc8);
        vos_md5_op2!(ul_var1, ul_var2, ul_var3, ul_var4, 0,  5, 0x21e1cde6);
        vos_md5_op2!(ul_var4, ul_var1, ul_var2, ul_var3, ul_x14,  9, 0xc33707d6);
        vos_md5_op2!(ul_var3, ul_var4, ul_var1, ul_var2, 0, 14, 0xf4d50d87);
        vos_md5_op2!(ul_var2, ul_var3, ul_var4, ul_var1, 0, 20, 0x455a14ed);
        vos_md5_op2!(ul_var1, ul_var2, ul_var3, ul_var4, 0,  5, 0xa9e3e905);
        vos_md5_op2!(ul_var4, ul_var1, ul_var2, ul_var3, 0,  9, 0xfcefa3f8);
        vos_md5_op2!(ul_var3, ul_var4, ul_var1, ul_var2, 0, 14, 0x676f02d9);
        vos_md5_op2!(ul_var2, ul_var3, ul_var4, ul_var1, 0, 20, 0x8d2a4c8a);

        vos_md5_op3!(ul_var1, ul_var2, ul_var3, ul_var4, 0,  4, 0xfffa3942);
        vos_md5_op3!(ul_var4, ul_var1, ul_var2, ul_var3, 0, 11, 0x8771f681);
        vos_md5_op3!(ul_var3, ul_var4, ul_var1, ul_var2, 0, 16, 0x6d9d6122);
        vos_md5_op3!(ul_var2, ul_var3, ul_var4, ul_var1, ul_x14, 23, 0xfde5380c);
        vos_md5_op3!(ul_var1, ul_var2, ul_var3, ul_var4, 0,  4, 0xa4beea44);
        vos_md5_op3!(ul_var4, ul_var1, ul_var2, ul_var3, 0, 11, 0x4bdecfa9);
        vos_md5_op3!(ul_var3, ul_var4, ul_var1, ul_var2, 0, 16, 0xf6bb4b60);
        vos_md5_op3!(ul_var2, ul_var3, ul_var4, ul_var1, 0, 23, 0xbebfbc70);
        vos_md5_op3!(ul_var1, ul_var2, ul_var3, ul_var4, 0,  4, 0x289b7ec6);
        vos_md5_op3!(ul_var4, ul_var1, ul_var2, ul_var3, ul_x0, 11, 0xeaa127fa);
        vos_md5_op3!(ul_var3, ul_var4, ul_var1, ul_var2, 0, 16, 0xd4ef3085);
        vos_md5_op3!(ul_var2, ul_var3, ul_var4, ul_var1, 0, 23, 0x04881d05);
        vos_md5_op3!(ul_var1, ul_var2, ul_var3, ul_var4, 0,  4, 0xd9d4d039);
        vos_md5_op3!(ul_var4, ul_var1, ul_var2, ul_var3, 0, 11, 0xe6db99e5);
        vos_md5_op3!(ul_var3, ul_var4, ul_var1, ul_var2, ul_x15, 16, 0x1fa27cf8);
        vos_md5_op3!(ul_var2, ul_var3, ul_var4, ul_var1, 0, 23, 0xc4ac5665);

        vos_md5_op4!(ul_var1, ul_var2, ul_var3, ul_var4, ul_x0, 6, 0xf4292244);
        vos_md5_op4!(ul_var4, ul_var1, ul_var2, ul_var3, 0, 10, 0x432aff97);
        vos_md5_op4!(ul_var3, ul_var4, ul_var1, ul_var2, ul_x14, 15, 0xab9423a7);
        vos_md5_op4!(ul_var2, ul_var3, ul_var4, ul_var1, 0, 21, 0xfc93a039);
        vos_md5_op4!(ul_var1, ul_var2, ul_var3, ul_var4, 0, 6, 0x655b59c3);
        vos_md5_op4!(ul_var4, ul_var1, ul_var2, ul_var3, 0, 10, 0x8f0ccc92);
        vos_md5_op4!(ul_var3, ul_var4, ul_var1, ul_var2, 0, 15, 0xffeff47d);
        vos_md5_op4!(ul_var2, ul_var3, ul_var4, ul_var1, 0, 21, 0x85845dd1);
        vos_md5_op4!(ul_var1, ul_var2, ul_var3, ul_var4, 0, 6, 0x6fa87e4f);
        vos_md5_op4!(ul_var4, ul_var1, ul_var2, ul_var3, ul_x15, 10, 0xfe2ce6e0);
        vos_md5_op4!(ul_var3, ul_var4, ul_var1, ul_var2, 0, 15, 0xa3014314);
        vos_md5_op4!(ul_var2, ul_var3, ul_var4, ul_var1, 0, 21, 0x4e0811a1);
        vos_md5_op4!(ul_var1, ul_var2, ul_var3, ul_var4, 0, 6, 0xf7537e82);
        vos_md5_op4!(ul_var4, ul_var1, ul_var2, ul_var3, 0, 10, 0xbd3af235);
        vos_md5_op4!(ul_var3, ul_var4, ul_var1, ul_var2, 0, 15, 0x2ad7d2bb);
        vos_md5_op4!(ul_var2, ul_var3, ul_var4, ul_var1, 0, 21, 0xeb86d391);

        vos_cks_sum_res_cal_by_var(pul_results, ul_var1, ul_var2, ul_var3, ul_var4);
    }
}

pub fn cks_md5_cksum_partial(pv_data: &mut [u8], pv_upto: &mut [u8], si_newdata: i32, pul_results: &mut [u32]) -> u32 {
    let mut ul_do_len: u32;

    if si_newdata != 0 {
        pul_results[0] = vos_md5_a_init!();
        pul_results[1] = vos_md5_b_init!();
        pul_results[2] = vos_md5_c_init!();
        pul_results[3] = vos_md5_d_init!();
    }

    ul_do_len = ((pv_upto.as_ptr() as usize - pv_data.as_ptr() as usize) & (!0x3f)) as u32;

    if ul_do_len != 0 {
        cks_md5_cksum_block(pv_data, ul_do_len, 0, 1, pul_results);
    }

    ul_do_len
}

pub fn vos_md5_cksum(pv_data: &mut [u8], ul_data_len: u32, ul_total_len: u32, pv_digest: &mut [u8], pul_init: &mut [u32]) {
    let mut puc_dp: &mut [u8];
    let mut pul_tp: &mut [u32];
    let mut aul_temp: [u32; 4] = [0; 4];

    if pv_data.is_empty() || pv_digest.is_empty() {
        return;
    }

    pul_tp = &mut aul_temp;

    if pul_init.is_empty() {
        pul_tp[0] = vos_md5_a_init!();
        pul_tp[1] = vos_md5_b_init!();
        pul_tp[2] = vos_md5_c_init!();
        pul_tp[3] = vos_md5_d_init!();
    } else {
        pul_tp[0] = pul_init[0];
        pul_tp[1] = pul_init[1];
        pul_tp[2] = pul_init[2];
        pul_tp[3] = pul_init[3];
    }

    cks_md5_cksum_block(pv_data, ul_data_len, ul_total_len, 0, &mut pul_tp);

    puc_dp = pv_digest;
    vos_md5_put!(pul_tp[0], puc_dp);
    pul_tp = &mut pul_tp[1..];
    vos_md5_put!(pul_tp[0], puc_dp);
    pul_tp = &mut pul_tp[1..];
    vos_md5_put!(pul_tp[0], puc_dp);
    pul_tp = &mut pul_tp[1..];
    vos_md5_put!(pul_tp[0], puc_dp);
}