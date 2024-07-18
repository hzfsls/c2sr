use crate::public::bzp_utils::*;

pub struct BzpHuffmanDecode {
    pub select: Vec<i32>,
    pub len: [[i32; bzp_max_alpha_size!()]; bzp_max_groups_num!()],
    pub perm: [[i32; bzp_max_alpha_size!()]; bzp_max_groups_num!()],
    pub limit: [[i32; bzp_max_alpha_size!()]; bzp_max_groups_num!()],
    pub base: [[i32; bzp_max_alpha_size!()]; bzp_max_groups_num!()],
    pub min_lens: [i32; bzp_max_groups_num!()],
    pub n_groups: i32,
    pub n_select: i32,
    pub alpha_size: i32,
    pub de_code_num: i32,
    pub select_cnt: i32,
    pub n_block: i32,
}

impl BzpHuffmanDecode {
    pub fn new() -> BzpHuffmanDecode {
        BzpHuffmanDecode {
            select: vec![],
            len: [[0; bzp_max_alpha_size!()]; bzp_max_groups_num!()],
            perm: [[0; bzp_max_alpha_size!()]; bzp_max_groups_num!()],
            limit: [[0; bzp_max_alpha_size!()]; bzp_max_groups_num!()],
            base: [[0; bzp_max_alpha_size!()]; bzp_max_groups_num!()],
            min_lens: [0; bzp_max_groups_num!()],
            n_groups: 0,
            n_select: 0,
            alpha_size: 0,
            de_code_num: 0,
            select_cnt: 0,
            n_block: 0,
        }
    }
}

pub fn bzp_huffman_decode_init(block_size: i32) -> Option<Box<BzpHuffmanDecode>> {
    if bzp_invalid_block_size!(block_size) {
        return None;
    }
    let mut huffman = Box::new(BzpHuffmanDecode::new());
    let space_size = bzp_base_block_size!() * block_size / bzp_elems_num_in_one_group!();
    huffman.select = vec![0; space_size as usize];
    for i in 0..bzp_max_groups_num!() {
        for j in 0..bzp_max_alpha_size!() {
            huffman.base[i as usize][j as usize] = 0;
            huffman.perm[i as usize][j as usize] = 0;
            huffman.limit[i as usize][j as usize] = 0;
        }
    }
    huffman.select_cnt = 0;
    huffman.de_code_num = 0;
    Some(huffman)
}

pub fn bzp_huffman_decode_reset(huffman: &mut Box<BzpHuffmanDecode>) {
    for i in 0..huffman.base.len() {
        for j in 0..huffman.base[i].len() {
            huffman.base[i][j] = 0;
        }
    }
    for i in 0..huffman.perm.len() {
        for j in 0..huffman.perm[i].len() {
            huffman.perm[i][j] = 0;
        }
    }
    for i in 0..huffman.limit.len() {
        for j in 0..huffman.limit[i].len() {
            huffman.limit[i][j] = 0;
        }
    }
    huffman.select_cnt = 0;
    huffman.de_code_num = 0;
}

fn bzp_get_one_table(huffman: &mut Box<BzpHuffmanDecode>, t: i32) {
    let mut vec = 0;
    let mut cnt = 0;
    let mut mi = huffman.len[t as usize][0];
    let mut mx = huffman.len[t as usize][0];
    for i in 0..huffman.alpha_size {
        mi = bzp_min_fun!(mi, huffman.len[t as usize][i as usize]);
        mx = bzp_max_fun!(mx, huffman.len[t as usize][i as usize]);
    }
    huffman.min_lens[t as usize] = mi;
    for i in mi..=mx {
        for j in 0..huffman.alpha_size {
            if huffman.len[t as usize][j as usize] == i {
                huffman.perm[t as usize][cnt as usize] = j;
                cnt += 1;
            }
        }
    }
    for i in 0..huffman.alpha_size {
        huffman.base[t as usize][huffman.len[t as usize][i as usize] as usize + 1] += 1;
    }
    for i in 1..=mx + 1 {
        huffman.base[t as usize][i as usize] += huffman.base[t as usize][(i - 1) as usize];
    }
    for i in mi..=mx {
        vec += huffman.base[t as usize][(i + 1) as usize] - huffman.base[t as usize][i as usize];
        huffman.limit[t as usize][i as usize] = vec - 1;
        vec <<= 1;
    }
    for i in (mi + 1)..=mx {
        huffman.base[t as usize][i as usize] = ((huffman.limit[t as usize][(i - 1) as usize] + 1) << 1) - huffman.base[t as usize][i as usize];
    }
}

pub fn bzp_generate_decode_table(huffman: &mut Box<BzpHuffmanDecode>) {
    for t in 0..huffman.n_groups {
        bzp_get_one_table(huffman, t);
    }
}

pub fn bzp_huffman_decode_finish(huffman: Box<BzpHuffmanDecode>) {
}


