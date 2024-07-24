use crate::public::bzp_stream_utils::*;
use crate::public::bzp_utils::*;
use crate::bzp_type::*;
use crate::decompress::bzp_bwt_decode::*;
use crate::decompress::bzp_huffman_decode::*;

use std::io::{Read, Write};
use std::fs::File;

pub struct InDeComdata {
    pub input: Box<BzpStream>,
    pub output: Box<BzpStream>,
    pub las_char: i32,
    pub num: i32,
    pub buf: u32,
    pub n_buf: i32,
    pub block_size: i32,
    pub block_crc: u32,
    pub list: [i32; bzp_ascii_size!()],
}

impl InDeComdata {
    pub fn new() -> InDeComdata {
        InDeComdata {
            input: Box::new(BzpStream::new()),
            output: Box::new(BzpStream::new()),
            las_char: 0,
            num: 0,
            buf: 0,
            n_buf: 0,
            block_size: 0,
            block_crc: 0,
            list: [0; bzp_ascii_size!()],
        }
    }
}

fn bzp_in_de_comdata_init() -> Option<Box<InDeComdata>> {
    let mut in_data = Box::new(InDeComdata::new());
    in_data.input = Box::new(BzpStream::new());
    in_data.output = Box::new(BzpStream::new());
    in_data.num = 0;
    in_data.las_char = bzp_ascii_size!();
    in_data.n_buf = 0;
    in_data.buf = 0;
    in_data.num = 0;
    in_data.block_crc = bzp_init_block_crc!();
    Some(in_data)
}

fn bzp_in_de_comdata_finish(in_data: Box<InDeComdata>) {
}

fn bzp_read_bits(n_bit: i32, in_data: &mut Box<InDeComdata>) -> u32 {
    let mut res = 0;
    while in_data.n_buf < n_bit {
        if in_data.input.n_buf == in_data.input.pos {
            let mut file_ptr = in_data.input.file_ptr.as_ref().unwrap();
            in_data.input.n_buf = file_ptr.read(&mut in_data.input.buf).unwrap() as i32;
            in_data.input.pos = 0;
        }
        let data = in_data.input.buf[in_data.input.pos as usize] as u32;
        in_data.buf = (in_data.buf << bzp_bits8!()) | data;
        in_data.input.pos += 1;
        in_data.n_buf += bzp_bits8!();
    }
    res = in_data.buf >> (in_data.n_buf - n_bit);
    res = res & ((1 << n_bit) - 1);
    in_data.n_buf -= n_bit;
    res
}

fn bzp_write_char(ch: u8, in_data: &mut Box<InDeComdata>) -> i32 {
    let mut ret = bzp_ok!();
    if in_data.output.n_buf >= bzp_buf_size!() {
        let mut file_ptr = in_data.output.file_ptr.as_ref().unwrap();
        let mut buf = in_data.output.buf;
        let n2 = file_ptr.write(&buf).unwrap() as i32;
        if n2 != in_data.output.n_buf {
            ret = bzp_error_io!();
        }
        in_data.output.n_buf = 0;
    }
    let n_buf = in_data.output.n_buf;
    in_data.output.buf[n_buf as usize] = ch;
    in_data.output.n_buf += 1;
    ret
}

fn bzp_huffman_decode_step(huffman: &mut Box<BzpHuffmanDecode>, in_data: &mut Box<InDeComdata>) -> i32 {
    if huffman.de_code_num == bzp_elems_num_in_one_group!() {
        huffman.de_code_num = 0;
        huffman.select_cnt += 1;
    }
    let gid = huffman.select[huffman.select_cnt as usize];
    let mut chlen = huffman.min_lens[gid as usize];
    let mut val = bzp_read_bits(chlen, in_data) as i32;
    while chlen < bzp_huffman_len_upper_limit!() && val > huffman.limit[gid as usize][chlen as usize] {
        chlen += 1;
        let nxtbit = bzp_read_bits(1, in_data);
        val = (val << 1) | nxtbit as i32;
    }
    if chlen > bzp_huffman_len_upper_limit!() {
        return -1;
    }
    val = val - huffman.base[gid as usize][chlen as usize];
    val = huffman.perm[gid as usize][val as usize];
    huffman.de_code_num += 1;
    val 
}

fn bzp_check_file_head(in_data: &mut Box<InDeComdata>) -> i32 {
    let ch = bzp_read_bits(bzp_bits8!(), in_data) as u8;
    if ch != bzp_block_head_1!() {
        return bzp_error_data!();
    }
    let ch = bzp_read_bits(bzp_bits8!(), in_data) as u8;
    if ch != bzp_block_head_2!() {
        return bzp_error_data!();
    }
    let ch = bzp_read_bits(bzp_bits8!(), in_data) as u8;
    if ch != bzp_block_head_3!() {
        return bzp_error_data!();
    }
    let ch = bzp_read_bits(bzp_bits8!(), in_data) as u8;
    if ch != bzp_block_head_4!() {
        return bzp_error_data!();
    }
    let ch = bzp_read_bits(bzp_bits8!(), in_data) as u8;
    if ch != bzp_block_head_5!() {
        return bzp_error_data!();
    }
    bzp_ok!()
}

fn bzp_read_uint24(in_data: &mut Box<InDeComdata>) -> u32 {
    let mut val = 0;
    let ch = bzp_read_bits(bzp_bits8!(), in_data) as u8;
    val = (val << bzp_bits8!()) | (ch as u32);
    let ch = bzp_read_bits(bzp_bits8!(), in_data) as u8;
    val = (val << bzp_bits8!()) | (ch as u32);
    let ch = bzp_read_bits(bzp_bits8!(), in_data) as u8;
    val = (val << bzp_bits8!()) | (ch as u32);
    val
}

fn bzp_read_uint32(in_data: &mut Box<InDeComdata>) -> u32 {
    let mut val = 0;
    let ch = bzp_read_bits(bzp_bits8!(), in_data) as u8;
    val = (val << bzp_bits8!()) | (ch as u32);
    let ch = bzp_read_bits(bzp_bits8!(), in_data) as u8;
    val = (val << bzp_bits8!()) | (ch as u32);
    let ch = bzp_read_bits(bzp_bits8!(), in_data) as u8;
    val = (val << bzp_bits8!()) | (ch as u32);
    let ch = bzp_read_bits(bzp_bits8!(), in_data) as u8;
    val = (val << bzp_bits8!()) | (ch as u32);
    val
}


fn bzp_de_huffman_select(in_data: &mut Box<InDeComdata>, huffman: &mut Box<BzpHuffmanDecode>) -> i32 {
    let mut ch: u8;
    let mut select_mtf = [0; bzp_huffman_max_size_select!()];
    for i in 0..huffman.n_select {
        let mut j = -1;
        loop {
            ch = bzp_read_bits(bzp_bit!(), in_data) as u8;
            j += 1;
            if ch == 0 {
                break;
            }
        }
        if j >= huffman.n_groups {
            return bzp_error_data!();
        }
        select_mtf[i as usize] = j;
    }
    let mut list_select = [0; bzp_max_groups_num!()];
    for i in 0..bzp_max_groups_num!() {
        list_select[i as usize] = i;
    }
    for i in 0..huffman.n_select {
        let pos = select_mtf[i as usize];
        let tmpv = list_select[pos as usize];
        for j in (1..=pos).rev() {
            list_select[j as usize] = list_select[(j - 1) as usize];
        }
        list_select[0] = tmpv;
        huffman.select[i as usize] = tmpv;
    }
    bzp_ok!()
}

fn bzp_de_huffman_len(in_data: &mut Box<InDeComdata>, huffman: &mut Box<BzpHuffmanDecode>) -> i32 {
    let mut ch: u8;
    for i in 0..huffman.n_groups {
        let mut val = bzp_read_bits(bzp_bits5!(), in_data) as i32;
        for j in 0..huffman.alpha_size {
            ch = bzp_read_bits(bzp_bit!(), in_data) as u8;
            while ch != 0 {
                ch = bzp_read_bits(bzp_bit!(), in_data) as u8;
                val += if ch == 0 { 1 } else { -1 };
                ch = bzp_read_bits(bzp_bit!(), in_data) as u8;
            }
            if val < 1 || val > bzp_huffman_len_upper_limit!() {
                return bzp_error_data!();
            }
            huffman.len[i as usize][j as usize] = val;
        }
    }
    bzp_ok!()
}

fn bzp_mtf_de_code(in_data: &mut Box<InDeComdata>, huffman: &mut Box<BzpHuffmanDecode>, debwt: &mut Box<BzpBwtDecodeInfo>) -> i32 {
    debwt.n_block = 0;
    let mut ch: u8;
    let nin_use = huffman.alpha_size - bzp_extra_chars_num!();
    let eob = nin_use + 1;
    let mut val = bzp_huffman_decode_step(huffman, in_data);
    while val != eob && val != -1 {
        if val == 0 || val == 1 {
            let mut res = 0;
            let mut base_num = 1;
            let mut cnt = 0;
            while val == 0 || val == 1 {
                res = res + (val + 1) * base_num;
                base_num <<= 1;
                val = bzp_huffman_decode_step(huffman, in_data);
            }
            for _ in 0..res {
                debwt.block[debwt.n_block as usize] = in_data.list[0] as u8;
                debwt.n_block += 1;
            }
        } else {
            let pos = val - 1;
            ch = in_data.list[pos as usize] as u8;
            debwt.block[debwt.n_block as usize] = ch;
            debwt.n_block += 1;
            for j in (1..=pos).rev() {
                in_data.list[j as usize] = in_data.list[(j - 1) as usize];
            }
            in_data.list[0] = ch as i32;
            val = bzp_huffman_decode_step(huffman, in_data);
        }
    }
    if val == -1 {
        return bzp_error_data!();
    }
    bzp_ok!()
}

fn bzp_de_code_to_stream(in_data: &mut Box<InDeComdata>, debwt: &mut Box<BzpBwtDecodeInfo>) -> i32 {
    let mut ret = bzp_ok!();
    for i in 0..debwt.n_block {
        let ch = debwt.de_code[i as usize];
        if in_data.num == bzp_rlc_num_4!() {
            for _ in 0..ch {
                bzp_update_crc!(in_data.block_crc, in_data.las_char);
                ret |= bzp_write_char(in_data.las_char as u8, in_data);
            }
            in_data.las_char = bzp_ascii_size!();
            in_data.num = 0;
        } else if ch == in_data.las_char as u8 {
            bzp_update_crc!(in_data.block_crc, ch);
            ret = bzp_write_char(ch as u8, in_data);
            in_data.num += 1;
        } else {
            bzp_update_crc!(in_data.block_crc, ch);
            ret = bzp_write_char(ch as u8, in_data);
            in_data.las_char = ch as i32;
            in_data.num = 1;
        }
        if ret != bzp_ok!() {
            break;
        }
    }
    ret
}

fn bzp_get_dictionary_list(in_data: &mut Box<InDeComdata>) -> i32 {
    let mut nin_use = 0;
    let mut use16 = [false; bzp_groups_ascii!()];
    let mut in_use = [false; bzp_ascii_size!()];
    for i in 0..bzp_groups_ascii!() {
        use16[i as usize] = bzp_read_bits(bzp_bit!(), in_data) != 0;
    }
    for i in 0..bzp_groups_ascii!() {
        if use16[i as usize] {
            for j in 0..bzp_chars_per_group_ascii!() {
                in_use[(i * bzp_groups_ascii!() + j) as usize] = bzp_read_bits(bzp_bit!(), in_data) != 0;
            }
        }
    }
    for i in 0..bzp_ascii_size!() {
        if in_use[i as usize] {
            in_data.list[nin_use as usize] = i as i32;
            nin_use += 1;
        }
    }
    nin_use
}

fn bzp_de_compress_one_block(in_data: &mut Box<InDeComdata>, huffman: &mut Box<BzpHuffmanDecode>, debwt: &mut Box<BzpBwtDecodeInfo>) -> i32 {
    let mut ret = bzp_ok!();
    bzp_check_file_head(in_data);
    let block_crc = bzp_read_uint32(in_data);
    println!("decompress crc: {:x}", block_crc);
    bzp_read_bits(bzp_bit!(), in_data);
    let ori_ptr = bzp_read_uint24(in_data);
    println!("decompress ori_ptr: {:x}", ori_ptr);
    if ori_ptr < 0 || ori_ptr > (bzp_base_block_size!() * in_data.block_size) as u32 {
        return bzp_error_data!();
    }
    let nin_use = bzp_get_dictionary_list(in_data);
    huffman.alpha_size = nin_use + bzp_extra_chars_num!();
    huffman.n_groups = bzp_read_bits(bzp_bits3!(), in_data) as i32;
    println!("decompress n_groups: {}", huffman.n_groups );
    if huffman.n_groups < bzp_ngroups_num_0!() || huffman.n_groups > bzp_ngroups_num_4!() {
        return bzp_error_data!();
    }
    huffman.n_select = bzp_read_bits(bzp_bits15!(), in_data) as i32;
    println!("decompress n_select: {}", huffman.n_select);
    let n_select_upper_limit = in_data.block_size * bzp_base_block_size!() / bzp_elems_num_in_one_group!() + 1;
    if huffman.n_select < 1 || huffman.n_select > n_select_upper_limit {
        return bzp_error_data!();
    }
    ret |= bzp_de_huffman_select(in_data, huffman);
    ret |= bzp_de_huffman_len(in_data, huffman);
    if ret != bzp_ok!() {
        return ret;
    }
    bzp_generate_decode_table(huffman);
    debwt.ori_ptr = ori_ptr as i32;
    bzp_mtf_de_code(in_data, huffman, debwt);
    if debwt.n_block >= bzp_base_block_size!() * in_data.block_size {
        return bzp_error_data!();
    }
    bzp_bwt_decode(debwt);
    bzp_de_code_to_stream(in_data, debwt);
    in_data.block_crc = !in_data.block_crc;
    println!("decompress in_data.block_crc: {:x}", in_data.block_crc);
    if block_crc != in_data.block_crc {
        return bzp_error_data!();
    }
    ret
}

fn bzp_read_file_end(in_data: &mut Box<InDeComdata>, cal_total_crc: u32) -> i32 {
    let ch = bzp_read_bits(bzp_bits8!(), in_data) as u8;
    if ch != bzp_file_end_1!() {
        return bzp_error_data!();
    }
    let ch = bzp_read_bits(bzp_bits8!(), in_data) as u8;
    if ch != bzp_file_end_2!() {
        return bzp_error_data!();
    }
    let ch = bzp_read_bits(bzp_bits8!(), in_data) as u8;
    if ch != bzp_file_end_3!() {
        return bzp_error_data!();
    }
    let ch = bzp_read_bits(bzp_bits8!(), in_data) as u8;
    if ch != bzp_file_end_4!() {
        return bzp_error_data!();
    }
    let ch = bzp_read_bits(bzp_bits8!(), in_data) as u8;
    if ch != bzp_file_end_5!() {
        return bzp_error_data!();
    }
    let stored_combined_crc = bzp_read_uint32(in_data);
    if cal_total_crc != stored_combined_crc {
        return bzp_error_data!();
    }
    bzp_ok!()
}

fn bzp_read_file_head(in_data: &mut Box<InDeComdata>) -> i32 {
    let ch = bzp_read_bits(bzp_bits8!(), in_data) as u8;
    if ch != bzp_hdr_b!() {
        return bzp_error_data_magic!();
    }
    let ch = bzp_read_bits(bzp_bits8!(), in_data) as u8;
    if ch != bzp_hdr_z!() {
        return bzp_error_data_magic!();
    }
    let ch = bzp_read_bits(bzp_bits8!(), in_data) as u8;
    if ch != bzp_hdr_h!() {
        return bzp_error_data_magic!();
    }
    let ch = bzp_read_bits(bzp_bits8!(), in_data) as u8;
    let block_size = ch - bzp_hdr_0!();
    if bzp_invalid_block_size!(block_size) {
        return bzp_error_data_magic!();
    }
    in_data.block_size = block_size as i32;
    bzp_ok!()
}

fn bzp_de_compress_data(in_data: &mut Box<InDeComdata>) -> i32 {
    let mut ret = bzp_ok!();
    let mut cal_total_crc = 0;
    let mut ch = 0;
    
    ret = bzp_read_file_head(in_data);
    if ret != bzp_ok!() {
        return ret;
    }
    let mut huffman = bzp_huffman_decode_init(in_data.block_size).unwrap();
    let mut debwt = bzp_bwt_decode_init(in_data.block_size).unwrap();
    while {ch = bzp_read_bits(bzp_bits8!(), in_data) as u8; 
            ch != bzp_file_end_0!() } {
        if ch != bzp_block_head_0!() {
            ret = bzp_error_data!();
            break;
        }
        bzp_huffman_decode_reset(&mut huffman);
        in_data.block_crc = bzp_init_block_crc!();
        ret = bzp_de_compress_one_block(in_data, &mut huffman, &mut debwt);
        if ret != bzp_ok!() {
            break;
        }
        cal_total_crc = (cal_total_crc << 1) | (cal_total_crc >> bzp_crc_move_right_val!());
        cal_total_crc ^= in_data.block_crc;
    }
    if ret == bzp_ok!() {
        ret = bzp_read_file_end(in_data, cal_total_crc);
    }
    bzp_huffman_decode_finish(huffman);
    bzp_bwt_decode_finish(debwt);
    ret
}

fn bzp_de_com_stream_finish(in_data: Box<InDeComdata>, in_stream: Box<BzpStream>, out_stream: Box<BzpStream>) {
    bzp_stream_finish(in_stream);
    bzp_stream_finish(out_stream);
    bzp_in_de_comdata_finish(in_data);
}


pub fn bzp_de_compress_stream(in_name: &str, out_name: &str) -> i32 {
    let mut ret = bzp_ok!();
    if in_name.is_empty() || out_name.is_empty() {
        return bzp_error_param!();
    }

    let (Some(mut in_stream), Some(mut out_stream)) = (bzp_stream_init(), bzp_stream_init()) else {
        return bzp_error_memory_open_failure!();
    };

    in_stream.file_ptr = File::open(in_name).ok();
    out_stream.file_ptr = File::create(out_name).ok();

    if in_stream.file_ptr.is_none() || out_stream.file_ptr.is_none() {
        std::fs::remove_file(out_name);
        return bzp_error_io!();
    }

    let Some(mut in_data) = bzp_in_de_comdata_init() else {
        std::fs::remove_file(out_name);
        return bzp_error_memory_open_failure!();
    };
    
    in_data.input = in_stream;
    in_data.output = out_stream;

    ret = bzp_de_compress_data(&mut in_data);
    if in_data.output.n_buf > 0 {
        let n2 = in_data.output.file_ptr.as_ref().unwrap().write(&in_data.output.buf[..in_data.output.n_buf as usize]).unwrap() as i32;
        if n2 != in_data.output.n_buf {
            ret = bzp_error_io!();
        }
        in_data.output.n_buf = 0;
    }

    // bzp_de_com_stream_finish(in_data, in_data.input.unwrap(), in_data.output.unwrap());
    if ret != bzp_ok!() {
        std::fs::remove_file(out_name);
    }
    ret
}