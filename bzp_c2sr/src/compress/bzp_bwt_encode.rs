use crate::public::bzp_utils::*;

pub struct BzpBwtInfo {
    pub sort_block: Vec<i32>,
    pub idx: Vec<i32>,
    pub is_start_pos: Vec<i32>,
    pub block: Vec<u8>,
    pub block_crc: u32,
    pub combined_crc: u32,
    pub n_block_max: i32,
    pub block_id: i32,
    pub n_block: i32,
    pub ori_ptr: i32,
    pub in_use: [bool; bzp_ascii_size!()], 
}

impl BzpBwtInfo {
    pub fn new() -> BzpBwtInfo {
        BzpBwtInfo {
            sort_block: vec![],
            idx: vec![],
            is_start_pos: vec![],
            block: vec![],
            block_crc: 0,
            combined_crc: 0,
            n_block_max: 0,
            block_id: 0,
            n_block: 0,
            ori_ptr: 0,
            in_use: [false; bzp_ascii_size!()],
        }
    }
}

pub struct BzpQSortInfo {
    pub stack_l: [i32; bzp_max_stack_size!()],
    pub stack_r: [i32; bzp_max_stack_size!()],
    pub cnt: i32,
    pub tl: i32,
    pub tr: i32,
}

impl BzpQSortInfo {
    pub fn new() -> BzpQSortInfo {
        BzpQSortInfo {
            stack_l: [0; bzp_max_stack_size!()],
            stack_r: [0; bzp_max_stack_size!()],
            cnt: 0,
            tl: 0,
            tr: 0,
        }
    }
}

pub fn bzp_block_sort_init(block_size: i32) -> Option<Box<BzpBwtInfo>> {
    if bzp_invalid_block_size!(block_size) {
        return None;
    }
    let mut bwt = Box::new(BzpBwtInfo::new());
    let space_size = block_size * bzp_block_reserved_space_size!();
    bwt.n_block_max = space_size - bzp_block_reserved_space_size!();
    bwt.block = vec![0; space_size as usize];
    bwt.sort_block = vec![0; space_size as usize];
    bwt.idx = vec![0; space_size as usize];
    bwt.is_start_pos = vec![0; space_size as usize];
    bwt.block_crc = bzp_init_block_crc!();
    Some(bwt)
}

fn bzp_shell_sort(sort_block: &mut [i32], idx: &mut [i32], l: i32, r: i32) {
    let increments = [bzp_shell_sort_increment1!(), bzp_shell_sort_increment0!()];
    if l >= r {
        return;
    }
    for id in 0..bzp_shell_sort_increment_nums!() {
        let h = increments[id];
        if r - l + 1 <= h {
            continue;
        }
        for i in (l + h)..=r {
            let tmp_idx = sort_block[i as usize];
            let tmp_val = idx[tmp_idx as usize];
            let mut j = i - h;
            while j >= l && idx[sort_block[j as usize] as usize] > tmp_val {
                sort_block[(j + h) as usize] = sort_block[j as usize];
                j -= h;
            }
            sort_block[(j + h) as usize] = tmp_idx;
        }
    }
}

fn bzp_swap_2_elem(sort_block: &mut [i32], l_pos: i32, r_pos: i32) {
    let value = sort_block[l_pos as usize];
    sort_block[l_pos as usize] = sort_block[r_pos as usize];
    sort_block[r_pos as usize] = value;
}

fn bzp_swap_3_elem(sort_block: &mut [i32], l_pos: i32, e_pos: i32, r_pos: i32) {
    let value = sort_block[l_pos as usize];
    sort_block[l_pos as usize] = sort_block[r_pos as usize];
    sort_block[r_pos as usize] = sort_block[e_pos as usize];
    sort_block[e_pos as usize] = value;
}

fn bzp_select_mid_val(sort_block: &[i32], idx: &[i32], mut l: i32, mut r: i32) -> i32 {
    let mid = (l + r) >> 1;
    let vl = idx[sort_block[l as usize] as usize];
    let vmid = idx[sort_block[mid as usize] as usize];
    let vr = idx[sort_block[r as usize] as usize];
    if vl > vr {
        let tmp = l;
        l = r;
        r = tmp;
        let vl = idx[sort_block[l as usize] as usize];
        let vr = idx[sort_block[r as usize] as usize];
    }
    if vmid <= vl {
        vl
    } else if vmid <= vr {
        vmid
    } else {
        vr
    }
}

fn bzp_q_sort_single(sort_block: &mut [i32], idx: &mut [i32], stack: &mut Box<BzpQSortInfo>) {
    let mut tl = stack.tl;
    let mut tr = stack.tr;
    let mut value = bzp_select_mid_val(sort_block, idx, tl, tr);
    let mut l_pos = tl;
    let mut r_pos = tr;
    let mut e_pos = tl;

    while e_pos <= r_pos {
        if idx[sort_block[e_pos as usize] as usize] < value {
            bzp_swap_2_elem(sort_block, e_pos, l_pos);
            e_pos += 1;
            l_pos += 1;
        } else if idx[sort_block[e_pos as usize] as usize] == value {
            e_pos += 1;
        } else {
            while r_pos >= e_pos && idx[sort_block[r_pos as usize] as usize] > value {
                r_pos -= 1;
            }
            if r_pos < e_pos {
                break;
            }
            if idx[sort_block[r_pos as usize] as usize] == value {
                bzp_swap_2_elem(sort_block, e_pos, r_pos);
            } else if l_pos == e_pos {
                bzp_swap_2_elem(sort_block, e_pos, r_pos);
                l_pos += 1;
            } else {
                bzp_swap_3_elem(sort_block, l_pos, e_pos, r_pos);
                l_pos += 1;
            }
            e_pos += 1;
            r_pos -= 1;
        }
    }

    if l_pos - tl > tr - r_pos {
        stack.stack_l[stack.cnt as usize] = tl;
        stack.stack_r[stack.cnt as usize] = l_pos - 1;
        stack.cnt += 1;
        stack.stack_l[stack.cnt as usize] = r_pos + 1;
        stack.stack_r[stack.cnt as usize] = tr;
        stack.cnt += 1;
    } else {
        stack.stack_l[stack.cnt as usize] = r_pos + 1;
        stack.stack_r[stack.cnt as usize] = tr;
        stack.cnt += 1;
        stack.stack_l[stack.cnt as usize] = tl;
        stack.stack_r[stack.cnt as usize] = l_pos - 1;
        stack.cnt += 1;
    }
}

fn bzp_q_sort(sort_block: &mut [i32], idx: &mut [i32], l: i32, r: i32) {
    let mut stack = Box::new(BzpQSortInfo::new());
    stack.cnt = 0;
    stack.stack_l[stack.cnt as usize] = l;
    stack.stack_r[stack.cnt as usize] = r;
    stack.cnt += 1;
    while stack.cnt > 0 {
        stack.cnt -= 1;
        let tl = stack.stack_l[stack.cnt as usize];
        let tr = stack.stack_r[stack.cnt as usize];
        if tl >= tr {
            continue;
        }
        if tr - tl < bzp_threshold_shell_sort!() {
            bzp_shell_sort(sort_block, idx, tl, tr);
            continue;
        }
        stack.tl = tl;
        stack.tr = tr;
        bzp_q_sort_single(sort_block, idx, &mut stack);
    }
}

fn bzp_update_flag(bwt: &mut Box<BzpBwtInfo>, l: i32, r: i32) {
    let mut tmpst = -1;
    for i in l..=r {
        let tmpnow = bwt.idx[bwt.sort_block[i as usize] as usize];
        if tmpst != tmpnow {
            bwt.is_start_pos[i as usize] = 1;
            tmpst = tmpnow;
        }
    }
}

fn bzp_binary_lifting_sort(bwt: &mut Box<BzpBwtInfo>) {
    let mut ftab = [0; bzp_ascii_size!()];
    for i in 0..bwt.n_block {
        ftab[bwt.block[i as usize] as usize] += 1;
    }
    for i in 1..bzp_ascii_size!() {
        ftab[i] += ftab[i - 1];
    }
    for i in 0..bwt.n_block {
        let ch = bwt.block[i as usize] as usize;
        ftab[ch] -= 1;
        bwt.sort_block[ftab[ch] as usize] = i;
    }
    for i in 0..bzp_ascii_size!() {
        bwt.is_start_pos[ftab[i] as usize] = 1;
    }
    let mut m = 1;
    let mut sortflag = true;
    while m < bwt.n_block && sortflag {
        let mut st = 0;
        sortflag = false;
        for i in 0..bwt.n_block {
            if bwt.is_start_pos[i as usize] == 1 {
                st = i;
            }
            let mut pos = bwt.sort_block[i as usize] - m;
            if pos < 0 {
                pos += bwt.n_block;
            }
            bwt.idx[pos as usize] = st;
        }
        let mut l = 0;
        let mut r = 1;
        while l < bwt.n_block {
            while r < bwt.n_block && bwt.is_start_pos[r as usize] != 1 {
                r += 1;
            }
            r -= 1;
            if l < r {
                sortflag = true;
                bzp_q_sort(&mut bwt.sort_block, &mut bwt.idx, l, r);
                bzp_update_flag(bwt, l, r);
            }
            l = r + 1;
            r = l + 1;
        }
        m <<= 1;
    }
}


pub fn bzp_block_sort_main(bwt: &mut Box<BzpBwtInfo>) {
    bzp_binary_lifting_sort(bwt);
    for i in 0..bwt.n_block {
        if bwt.sort_block[i as usize] == 0 {
            bwt.ori_ptr = i;
            break;
        }
    }
}

fn bzp_bwt_finish(bwt: Box<BzpBwtInfo>) {
}