use crate::public::bzp_utils::*;
use crate::bzp_type::*;

#[derive(Copy)]
#[derive(Clone)]
pub struct BzpHuffmanInfo {
    pub heap: [i32; bzp_max_alpha_size!() + 1],
    pub weight: [i32; bzp_max_alpha_size!() * 2],
    pub parent: [i32; bzp_max_alpha_size!() * 2],
    pub len: [i32; bzp_max_alpha_size!()],
    pub table: [i32; bzp_max_alpha_size!()],
    pub n_heap: i32,
    pub n_weight: i32,
    pub alpha_size: i32,
}

impl BzpHuffmanInfo {
    pub fn new() -> BzpHuffmanInfo {
        BzpHuffmanInfo {
            heap: [0; bzp_max_alpha_size!() + 1],
            weight: [0; bzp_max_alpha_size!() * 2],
            parent: [0; bzp_max_alpha_size!() * 2],
            len: [0; bzp_max_alpha_size!()],
            table: [0; bzp_max_alpha_size!()],
            n_heap: 0,
            n_weight: 0,
            alpha_size: 0,
        }
    }
}

pub struct BzpHuffmanGroups {
    pub block: Vec<i32>,
    pub mtf_freq: Vec<i32>,
    pub select: Vec<i32>,
    pub select_mtf: Vec<i32>,
    pub huffman_groups: [Box<BzpHuffmanInfo>; bzp_max_groups_num!()],
    pub cost: [i32; bzp_max_groups_num!()],
    pub n_groups: i32,
    pub n_block: i32,
    pub n_select: i32,
    pub alpha_size: i32,
}

impl BzpHuffmanGroups {
    pub fn new() -> BzpHuffmanGroups {
        BzpHuffmanGroups {
            block: Vec::new(),
            mtf_freq: Vec::new(),
            select: Vec::new(),
            select_mtf: Vec::new(),
            huffman_groups: core::array::from_fn(|_| Box::new(BzpHuffmanInfo::new())),
            cost: [0; bzp_max_groups_num!()],
            n_groups: 0,
            n_block: 0,
            n_select: 0,
            alpha_size: 0,
        }
    }
}

fn bzp_huffman_init(alpha_size: i32, huffman: &mut Box<BzpHuffmanInfo>) {
    *huffman = Box::new(BzpHuffmanInfo::new());
    huffman.n_heap = 0;
    huffman.n_weight = 0;
    huffman.alpha_size = alpha_size;
}

fn bzp_huffman_init_array(huffman: &mut Box<BzpHuffmanInfo>) {
    huffman.n_heap = 0;
    huffman.n_weight = huffman.alpha_size;

    for i in 0..huffman.alpha_size {
        huffman.parent[i as usize] = -1;
    }
}

fn bzp_heap_adjust_up(heap: &mut [i32], weight: &mut [i32], pos: i32) {
    let mut tmpw = weight[heap[pos as usize] as usize];
    let mut tmpv = heap[pos as usize];
    let mut pos = pos;
    while pos > 1 {
        if tmpw < weight[heap[(pos >> 1) as usize] as usize] {
            heap[pos as usize] = heap[(pos >> 1) as usize];
            pos >>= 1;
        } else {
            break;
        }
    }
    heap[pos as usize] = tmpv;
}

fn bzp_heap_adjust_down(heap: &mut [i32], weight: &mut [i32], n_heap: i32) {
    let mut pos = 1;
    let mut chpos = pos << 1;
    let mut tmpid = heap[pos as usize];
    let mut tmpv = weight[tmpid as usize];
    while chpos <= n_heap {
        if (chpos | 1) <= n_heap && weight[heap[chpos as usize] as usize] > weight[heap[(chpos | 1) as usize] as usize] {
            chpos |= 1;
        }
        if tmpv < weight[heap[chpos as usize] as usize] {
            break;
        }
        heap[pos as usize] = heap[chpos as usize];
        pos = chpos;
        chpos = pos << 1;
    }
    heap[pos as usize] = tmpid;
}

fn bzp_heap_init(huffman: &mut Box<BzpHuffmanInfo>) {
    for i in 0..huffman.alpha_size {
        huffman.n_heap += 1;
        huffman.heap[huffman.n_heap as usize] = i;
        bzp_heap_adjust_up(&mut huffman.heap, &mut huffman.weight, huffman.n_heap);
    }
}

fn bzp_huffman_weight_add(w1: i32, w2: i32) -> i32 {
    ((((w1 as u32) & 0xffffff00) + ((w2 as u32)  & 0xffffff00)) | (bzp_max_fun!((w1 as u32) & 0x000000ff, (w2 as u32)  & 0x000000ff)) + 1) as i32
}

fn bzp_build_huffman_tree(huffman: &mut Box<BzpHuffmanInfo>) {
    bzp_huffman_init_array(huffman);
    bzp_heap_init(huffman);
    let mut idx1;
    let mut idx2;
    while huffman.n_heap > 1 {
        idx1 = huffman.heap[1];
        huffman.heap[1] = huffman.heap[huffman.n_heap as usize];
        huffman.n_heap -= 1;
        bzp_heap_adjust_down(&mut huffman.heap, &mut huffman.weight, huffman.n_heap);
        idx2 = huffman.heap[1];
        huffman.heap[1] = huffman.heap[huffman.n_heap as usize];
        huffman.n_heap -= 1;
        bzp_heap_adjust_down(&mut huffman.heap, &mut huffman.weight, huffman.n_heap);
        huffman.weight[huffman.n_weight as usize] = bzp_huffman_weight_add(huffman.weight[idx1 as usize], huffman.weight[idx2 as usize]);
        huffman.parent[idx1 as usize] = huffman.n_weight;
        huffman.parent[idx2 as usize] = huffman.n_weight;
        huffman.parent[huffman.n_weight as usize] = -1;
        huffman.n_heap += 1;
        huffman.heap[huffman.n_heap as usize] = huffman.n_weight;
        huffman.n_weight += 1;
        bzp_heap_adjust_up(&mut huffman.heap, &mut huffman.weight, huffman.n_heap);
    }
}

fn bzp_get_code_len(huffman: &mut Box<BzpHuffmanInfo>) -> i32 {
    bzp_build_huffman_tree(huffman);
    let mut maxlen = 0;
    for i in 0..huffman.alpha_size {
        let mut x = i;
        let mut tlen = 0;
        while huffman.parent[x as usize] >= 0 {
            x = huffman.parent[x as usize];
            tlen += 1;
        }
        huffman.len[i as usize] = tlen;
        maxlen = bzp_max_fun!(maxlen, tlen);
    }
    maxlen
}

fn bzp_build_tree_balance_height(huffman: &mut Box<BzpHuffmanInfo>) {
    let mut maxlen = 0;
    for i in 0..huffman.alpha_size {
        if huffman.weight[i as usize] == 0 {
            huffman.weight[i as usize] = 1 << bzp_huffman_height_weight_bits!();
        } else {
            huffman.weight[i as usize] <<= bzp_huffman_height_weight_bits!();
        }
    }

    loop {
        maxlen = bzp_get_code_len(huffman);
        if maxlen > bzp_max_tree_height_encode!() {
            for i in 0..huffman.alpha_size {
                let mut w = huffman.weight[i as usize] >> bzp_huffman_height_weight_bits!();
                w = (w >> 1) + 1;
                huffman.weight[i as usize] = w << bzp_huffman_height_weight_bits!();
            }
        }
        if maxlen <= bzp_max_tree_height_encode!() {
            break;
        }
    }
}

fn bzp_get_huffman_table(huffman: &mut Box<BzpHuffmanInfo>) {
    let mut vec = 0;
    let mut mi = huffman.len[0];
    let mut mx = huffman.len[0];
    for i in 0..huffman.alpha_size {
        mi = bzp_min_fun!(mi, huffman.len[i as usize]);
        mx = bzp_max_fun!(mx, huffman.len[i as usize]);
    }
    for i in mi..=mx {
        for j in 0..huffman.alpha_size {
            if huffman.len[j as usize] == i {
                huffman.table[j as usize] = vec;
                vec += 1;
            }
        }
        vec <<= 1;
    }
}

pub fn bzp_huffman_groups_reset(huffman: &mut Box<BzpHuffmanGroups>, alpha_size: i32) -> i32 {
    if bzp_invalid_alpha_size!(alpha_size) {
        return bzp_error_param!();
    }

    huffman.alpha_size = alpha_size;
    huffman.block = Vec::new();
    huffman.mtf_freq = Vec::new();
    huffman.n_select = 0;
    huffman.n_groups = 0;

    for i in 0..bzp_max_groups_num!() {
        bzp_huffman_init(alpha_size, &mut huffman.huffman_groups[i as usize]);
    }

    bzp_ok!()
}

pub fn bzp_huffman_groups_init(block_size: i32) -> Option<Box<BzpHuffmanGroups>> {
    if bzp_invalid_block_size!(block_size) {
        return None;
    }
    let mut huffman_groups = Box::new(BzpHuffmanGroups::new());
    huffman_groups.select = Vec::new();
    huffman_groups.select_mtf = Vec::new();
    let space_size = block_size * bzp_base_block_size!() / bzp_elems_num_in_one_group!();
    huffman_groups.select = vec![0; space_size as usize];
    huffman_groups.select_mtf = vec![0; space_size as usize];
    huffman_groups.alpha_size = 0;
    huffman_groups.block = Vec::new();
    huffman_groups.mtf_freq = Vec::new();
    huffman_groups.n_select = 0;
    huffman_groups.n_groups = 0;

    for i in 0..bzp_max_groups_num!() {
        bzp_huffman_init(0, &mut huffman_groups.huffman_groups[i as usize]);
    }

    Some(huffman_groups)
}

pub fn bzp_huffman_groups_finish(huffman: Box<BzpHuffmanGroups>) {
}

pub fn bzp_get_huffman_groups(n_block: i32) -> i32 {
    let mut n_groups = 1;
    if n_block < bzp_ngroups_block_num_limit0!() {
        n_groups = bzp_ngroups_num_0!();
    } else if n_block < bzp_ngroups_block_num_limit1!() {
        n_groups = bzp_ngroups_num_1!();
    } else if n_block < bzp_ngroups_block_num_limit2!() {
        n_groups = bzp_ngroups_num_2!();
    } else if n_block < bzp_ngroups_block_num_limit3!() {
        n_groups = bzp_ngroups_num_3!();
    } else {
        n_groups = bzp_ngroups_num_4!();
    }
    n_groups
}

fn bzp_generate_select_mtf(huffman: &mut Box<BzpHuffmanGroups>) {
    let n_groups = huffman.n_groups;
    let mut list = vec![0; n_groups as usize];
    for i in 0..n_groups {
        list[i as usize] = i;
    }
    for i in 0..huffman.n_select {
        let mut pos = 0;
        for j in 0..n_groups {
            if huffman.select[i as usize] == list[j as usize] {
                pos = j;
                break;
            }
        }
        for j in (1..=pos).rev() {
            list[j as usize] = list[(j - 1) as usize];
        }
        list[0] = huffman.select[i as usize];
        huffman.select_mtf[i as usize] = pos;
    }
}

fn bzp_init_len_array(huffman: &mut Box<BzpHuffmanGroups>) {
    let n_groups = huffman.n_groups;
    let mut npart = n_groups;
    let mut all_freq_num = huffman.n_block;
    let mut st = 0;
    let mut ed;
    while npart > 0 {
        let mut now_freq_num = 0;
        let freq_num_limit = all_freq_num / npart;

        ed = st - 1;
        while ed < huffman.alpha_size - 1 && now_freq_num < freq_num_limit {
            ed += 1;
            now_freq_num += huffman.mtf_freq[ed as usize];
        }

        if ed > st && npart != n_groups && npart != 1 && ((n_groups - npart) & 1) != 0 {
            now_freq_num -= huffman.mtf_freq[ed as usize];
            ed -= 1;
        }
        for i in 0..huffman.alpha_size {
            if i >= st && i <= ed {
                huffman.huffman_groups[npart as usize - 1].len[i as usize] = 0;
            } else {
                huffman.huffman_groups[npart as usize - 1].len[i as usize] = bzp_huffman_len_max_cost!();
            }
        }
        npart -= 1;
        st = ed + 1;
        all_freq_num -= now_freq_num;
    }
}

fn bzp_calculate_cost(huffman: &mut Box<BzpHuffmanGroups>, st: i32, ed: i32) {
    for i in 0..bzp_max_groups_num!() {
        huffman.cost[i as usize] = 0;
    }
    let n_groups = huffman.n_groups;
    for k in st..=ed {
        for t in 0..n_groups {
            huffman.cost[t as usize] += huffman.huffman_groups[t as usize].len[huffman.block[k as usize] as usize];
        }
    }
}

fn bzp_select_tree(huffman: &mut Box<BzpHuffmanGroups>) -> i32 {
    let mut id = 0;
    let n_groups = huffman.n_groups;
    for k in 0..n_groups {
        if huffman.cost[k as usize] < huffman.cost[id as usize] {
            id = k;
        }
    }
    huffman.select[huffman.n_select as usize] = id;
    huffman.n_select += 1;
    id
}

pub fn bzp_huffman_main(huffman: &mut Box<BzpHuffmanGroups>) {
    let n_groups = bzp_get_huffman_groups(huffman.n_block);
    huffman.n_groups = n_groups;
    bzp_init_len_array(huffman);
    let mut st = 0;
    for _ in 0..bzp_max_iter_num!() {
        for j in 0..n_groups {
            bzp_huffman_init_array(&mut huffman.huffman_groups[j as usize]);
        }
        st = 0;
        huffman.n_select = 0;
        while st < huffman.n_block {
            let ed = bzp_min_fun!(huffman.n_block, st + bzp_elems_num_in_one_group!()) - 1;
            bzp_calculate_cost(huffman, st, ed);
            let id = bzp_select_tree(huffman);
            for k in st..=ed {
                huffman.huffman_groups[id as usize].weight[huffman.block[k as usize] as usize] += 1;
            }
            st = ed + 1;
        }
        for j in 0..n_groups {
            bzp_build_tree_balance_height(&mut huffman.huffman_groups[j as usize]);
        }
    }
    bzp_generate_select_mtf(huffman);
    for i in 0..n_groups {
        bzp_get_huffman_table(&mut huffman.huffman_groups[i as usize]);
    }
}

