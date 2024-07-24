use crate::public::bzp_utils::*;
use crate::bzp_type::*;

pub struct BzpMtfInfo {
    pub block: Vec<u8>,
    pub map: Vec<i32>,
    pub mtf_v: Vec<i32>,
    pub in_use: Vec<bool>,
    pub mtf_freq: [i32; bzp_max_alpha_size!()],
    pub n_block: i32,
    pub n_mtf: i32, 
    pub n_use: i32,
    pub pad: i32,
}

impl BzpMtfInfo {
    pub fn new() -> BzpMtfInfo {
        BzpMtfInfo {
            block: vec![],
            map: vec![],
            mtf_v: vec![],
            in_use: vec![],
            mtf_freq: [0; bzp_max_alpha_size!()],
            n_block: 0,
            n_mtf: 0,
            n_use: 0,
            pad: 0,
        }
    }
}

pub fn bzp_mtf_init(block_size: i32) -> Option<Box<BzpMtfInfo>> {
    if bzp_invalid_block_size!(block_size) {
        return None;
    }
    let mut mtf = Box::new(BzpMtfInfo::new());
    mtf.mtf_v = vec![0; block_size as usize * bzp_base_block_size!() as usize];
    mtf.n_use = 0;
    mtf.n_mtf = 0;
    Some(mtf)
}

pub fn bzp_mtf_reset(mtf: &mut Box<BzpMtfInfo>) {
    mtf.n_use = 0;
    mtf.n_mtf = 0;
    mtf.block = vec![];
    mtf.map = vec![];
    mtf.in_use = vec![];
}

fn bzp_map_input_char(mtf: &mut Box<BzpMtfInfo>, list: &mut [u8], len_list: i32) {
    if bzp_ascii_size!() > len_list {
        return;
    }
    for i in 0..bzp_ascii_size!() {
        if mtf.in_use[i as usize] {
            list[mtf.n_use as usize] = i as u8;
            mtf.n_use += 1;
        }
    }
}


pub fn bzp_num_encode(mtf: &mut Box<BzpMtfInfo>, num: i32) {
    let mut num = num << 1;
    loop {
        num >>= 1;
        num -= 1;
        if num & 1 != 0 {
            mtf.mtf_v[mtf.n_mtf as usize] = bzp_mtf_encode1!();
            mtf.n_mtf += 1;
            mtf.mtf_freq[bzp_mtf_encode1!() as usize] += 1;
        } else {
            mtf.mtf_v[mtf.n_mtf as usize] = bzp_mtf_encode0!();
            mtf.n_mtf += 1;
            mtf.mtf_freq[bzp_mtf_encode0!() as usize] += 1;
        }
        if num < bzp_mtf_encode_base!() {
            break;
        }
    }
}

pub fn bzp_mtf_main(mtf: &mut Box<BzpMtfInfo>) {
    let mut list = [0; bzp_max_alpha_size!() as usize];
    let eob: i32;
    let mut num = 0;
    bzp_map_input_char(mtf, &mut list, bzp_max_alpha_size!());
    eob = mtf.n_use + 1;
    for i in 0..=eob {
        mtf.mtf_freq[i as usize] = 0;
    }
    for i in 0..mtf.n_block {
        let mut pos = mtf.map[i as usize] - 1;
        if pos < 0 {
            pos += mtf.n_block;
        }
        let ch = mtf.block[pos as usize];
        if ch == list[0] {
            num += 1;
        } else {
            if num > 0 {
                bzp_num_encode(mtf, num);
                num = 0;
            }
            let mut pos_ = 1;
            while ch != list[pos_ as usize] && pos_ < mtf.n_use {
                pos_ += 1;
            }
            for j in (1..=pos_).rev() {
                list[j as usize] = list[(j - 1) as usize];
            }
            list[0] = ch;
            mtf.mtf_v[mtf.n_mtf as usize] = pos_ + 1;
            mtf.mtf_freq[(pos_ + 1) as usize] += 1;
            mtf.n_mtf += 1;
        }
    }
    if num > 0 {
        bzp_num_encode(mtf, num);
    }
    mtf.mtf_v[mtf.n_mtf as usize] = eob;
    mtf.mtf_freq[eob as usize] += 1;
    mtf.n_mtf += 1;
}

pub fn bzp_mtf_finish(mtf: Box<BzpMtfInfo>) {
}