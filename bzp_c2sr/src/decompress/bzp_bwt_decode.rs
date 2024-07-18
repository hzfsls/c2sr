use crate::public::bzp_utils::*;

pub struct BzpBwtDecodeInfo {
    pub sorted: Vec<i32>, // 把mtf解码结果排序
    pub block: Vec<u8>,  // mtf解码的结果
    pub de_code: Vec<u8>, // 存放bwt解码结果
    pub n_block: i32,  // 一个块中的字符数量
    pub ori_ptr: i32,  // bwt解码的起始位置
}

impl BzpBwtDecodeInfo {
    pub fn new() -> BzpBwtDecodeInfo {
        BzpBwtDecodeInfo {
            sorted: vec![],
            block: vec![],
            de_code: vec![],
            n_block: 0,
            ori_ptr: 0,
        }
    }
}

pub fn bzp_bwt_decode_init(block_size: i32) -> Option<Box<BzpBwtDecodeInfo>> {
    if bzp_invalid_block_size!(block_size) {
        return None;
    }
    let space_size = block_size * bzp_base_block_size!();
    let mut bwt = Box::new(BzpBwtDecodeInfo::new());
    bwt.block = vec![0; space_size as usize];
    bwt.de_code = vec![0; space_size as usize];
    bwt.sorted = vec![0; space_size as usize];
    bwt.n_block = 0;
    bwt.ori_ptr = 0;
    Some(bwt)
}

pub fn bzp_bwt_decode(bwt: &mut Box<BzpBwtDecodeInfo>) {
    let mut ftab = [0; bzp_ascii_size!() + 1];
    for i in 0..bzp_ascii_size!() + 1 {
        ftab[i as usize] = 0;
    }
    for i in 0..bwt.n_block {
        ftab[(bwt.block[i as usize] + 1) as usize] += 1;
    }
    for i in 1..bzp_ascii_size!() + 1 {
        ftab[i as usize] += ftab[i - 1];
    }
    for i in 0..bwt.n_block {
        let ch = bwt.block[i as usize];
        bwt.sorted[ftab[ch as usize] as usize] = i;
        ftab[ch as usize] += 1;
    }
    let mut cnt = 0;
    let mut pos = bwt.ori_ptr;
    while cnt < bwt.n_block {
        pos = bwt.sorted[pos as usize];
        let ch = bwt.block[pos as usize];
        bwt.de_code[cnt as usize] = ch;
        cnt += 1;
    }
}


pub fn bzp_bwt_decode_finish(bwt: Box<BzpBwtDecodeInfo>) {
}
