use crate::public::bzp_stream_utils::*;
use crate::public::bzp_utils::*;
use crate::bzp_type::*;
use crate::compress::bzp_bwt_encode::*;
use crate::compress::bzp_huffman_encode::*;
use crate::compress::bzp_mtf_encode::*;


use std::io::{Read, Write};
use std::fs::File;

macro_rules! bzp_input_compress { () => { 0 }; }
pub(crate) use bzp_input_compress;
macro_rules! bzp_output_compress { () => { 1 }; }
pub(crate) use bzp_output_compress;
macro_rules! bzp_retuen_compress { () => { 2 }; }
pub(crate) use bzp_retuen_compress;

pub struct BzpFile {
    pub input: Box<BzpStream>,
    pub output: Box<BzpStream>,
    pub state: i32,
    pub las_char: i32,
    pub num: i32,
    pub pad: i32,
}

impl BzpFile {
    pub fn new() -> BzpFile {
        BzpFile {
            input: Box::new(BzpStream::new()),
            output: Box::new(BzpStream::new()),
            state: 0,
            las_char: 0,
            num: 0,
            pad: 0,
        }
    }
}

pub struct BzpOutComdata {
    pub out: Vec<u8>,
    pub num: i32,
    pub buf: u32,
    pub n_buf: i32,
    pub block_size: i32,
}

impl BzpOutComdata {
    pub fn new() -> BzpOutComdata {
        BzpOutComdata {
            out: vec![],
            num: 0,
            buf: 0,
            n_buf: 0,
            block_size: 0,
        }
    }
}

pub struct BzpAlgorithmInfo {
    pub bwt: Box<BzpBwtInfo>,
    pub huffman: Box<BzpHuffmanGroups>,
    pub mtf: Box<BzpMtfInfo>,
    pub compress_file: Box<BzpFile>,
    pub out_data: Box<BzpOutComdata>,
}

impl BzpAlgorithmInfo {
    pub fn new() -> BzpAlgorithmInfo {
        BzpAlgorithmInfo {
            bwt: Box::new(BzpBwtInfo::new()),
            huffman: Box::new(BzpHuffmanGroups::new()),
            mtf: Box::new(BzpMtfInfo::new()),
            compress_file: Box::new(BzpFile::new()),
            out_data: Box::new(BzpOutComdata::new()),
        }
    }
}

pub fn bzp_algorithm_info_init(block_size: i32) -> Option<Box<BzpAlgorithmInfo>> {
    let mut bzp_info = Box::new(BzpAlgorithmInfo::new());
    bzp_info.bwt = bzp_block_sort_init(block_size)?;
    bzp_info.mtf = bzp_mtf_init(block_size)?;
    bzp_info.huffman = bzp_huffman_groups_init(block_size)?;
    bzp_info.out_data = bzp_out_com_data_init(block_size)?;
    bzp_info.compress_file = bzp_file_init()?;

    return Some(bzp_info);
}

fn bzp_open_file(bzp_info: &mut Box<BzpAlgorithmInfo>, in_name: &str, out_name: &str) -> i32 {
    bzp_info.compress_file.input.file_ptr = File::open(in_name).ok();
    bzp_info.compress_file.output.file_ptr = File::create(out_name).ok();
    if bzp_info.compress_file.input.file_ptr.is_none() || bzp_info.compress_file.output.file_ptr.is_none() {
        std::fs::remove_file(out_name);
        return bzp_error_io!();
    }
    return bzp_ok!();
}

fn bzp_algorithm_info_finish(bzp_info: Box<BzpAlgorithmInfo>) {
}

fn bzp_file_init() -> Option<Box<BzpFile>> {
    let mut compress_file = Box::new(BzpFile::new());
    let in_stream = bzp_stream_init();
    let out_stream = bzp_stream_init();
    if in_stream.is_none() || out_stream.is_none() {
        return None;
    }
    compress_file.input = in_stream?;
    compress_file.output = out_stream?;
    compress_file.input.pos = 0;
    compress_file.output.pos = 0;
    compress_file.num = 0;
    compress_file.las_char = bzp_ascii_size!();
    compress_file.state = bzp_input_compress!();
    Some(compress_file)
}


pub fn bzp_file_finish(bzp_f: Box<BzpFile>) {
}

fn bzp_out_com_data_init(block_size: i32) -> Option<Box<BzpOutComdata>> {
    let mut out_data = Box::new(BzpOutComdata::new());
    out_data.out = vec![0; block_size as usize * bzp_base_block_size!() as usize];
    out_data.block_size = block_size;
    Some(out_data)
}

fn bzp_out_com_data_finish(data: Box<BzpOutComdata>) {
}

fn bzp_write_to_array(val: i32, n: i32, data: &mut Box<BzpOutComdata>) {
    while data.n_buf >= bzp_bits8!() {
        data.out[data.num as usize] = (data.buf >> bzp_bits24!()) as u8;
        data.num += 1;
        data.n_buf -= bzp_bits8!();
        data.buf <<= bzp_bits8!();
    }
    data.buf |= (val << (bzp_bits32!() - n - data.n_buf)) as u32;
    data.n_buf += n;
}

use std::io::{Seek, SeekFrom};

fn bzp_write_int32(val: i32, data: &mut Box<BzpOutComdata>) {
    bzp_write_to_array((val >> bzp_bits24!()) & 0xff, bzp_bits8!(), data);
    bzp_write_to_array((val >> bzp_bits16!()) & 0xff, bzp_bits8!(), data);
    bzp_write_to_array((val >> bzp_bits8!()) & 0xff, bzp_bits8!(), data);
    bzp_write_to_array(val & 0xff, bzp_bits8!(), data);
}

fn bzp_file_eof(f: &mut File) -> bool {
    let mut buf: [u8; 1] = [0; 1];
    let c = f.read(&mut buf);
    if c.is_ok() && c.unwrap() == 0 {
        return true;
    }
    f.seek(SeekFrom::Current(-1));
    return false;
}

fn bzp_write_file_head(out_data: &mut Box<BzpOutComdata>, block_id: i32) {
    if block_id == 0 {
        bzp_write_to_array(bzp_hdr_b!(), bzp_bits8!(), out_data);
        bzp_write_to_array(bzp_hdr_z!(), bzp_bits8!(), out_data);
        bzp_write_to_array(bzp_hdr_h!(), bzp_bits8!(), out_data);
        bzp_write_to_array(bzp_hdr_0!() + out_data.block_size, bzp_bits8!(), out_data);
    }
}

fn bzp_calculate_crc(bwt: &mut Box<BzpBwtInfo>) {
    bwt.block_crc = !(bwt.block_crc);
    bwt.combined_crc = (bwt.combined_crc << 1) | (bwt.combined_crc >> bzp_crc_move_right_val!());
    bwt.combined_crc ^= bwt.block_crc;
}

fn bzp_write_block_head(out_data: &mut Box<BzpOutComdata>, bwt: &mut Box<BzpBwtInfo>) {
    bzp_write_to_array(bzp_block_head_0!(), bzp_bits8!(), out_data);
    bzp_write_to_array(bzp_block_head_1!(), bzp_bits8!(), out_data);
    bzp_write_to_array(bzp_block_head_2!(), bzp_bits8!(), out_data);
    bzp_write_to_array(bzp_block_head_3!(), bzp_bits8!(), out_data);
    bzp_write_to_array(bzp_block_head_4!(), bzp_bits8!(), out_data);
    bzp_write_to_array(bzp_block_head_5!(), bzp_bits8!(), out_data);
    println!("compress crc: {:x}", bwt.block_crc);
    bzp_write_int32(bwt.block_crc as i32, out_data);
    bzp_write_to_array(0, bzp_bit!(), out_data);
    bzp_write_to_array(bwt.ori_ptr, bzp_bits24!(), out_data);
    println!("compress ori_ptr: {:x}", bwt.ori_ptr);
}

fn bzp_write_valid_ascii(out_data: &mut Box<BzpOutComdata>, bwt: &mut Box<BzpBwtInfo>) {
    let mut valid_gid = [0; bzp_ascii_size!()];
    let mut cnt = 0;
    let mut use16 = [false; bzp_ascii_size!()];
    for i in 0..bzp_ascii_size!() {
        let gid = i / bzp_chars_per_group_ascii!();
        use16[gid] |= bwt.in_use[i as usize];
    }
    for i in 0..bzp_groups_ascii!() {
        bzp_write_to_array(use16[i as usize] as i32, bzp_bit!(), out_data);
        if use16[i as usize] {
            valid_gid[cnt as usize] = i;
            cnt += 1;
        }
    }
    for i in 0..cnt {
        for j in 0..bzp_chars_per_group_ascii!() {
            let valid = valid_gid[i as usize] * bzp_chars_per_group_ascii!() + j;
            bzp_write_to_array(bwt.in_use[valid as usize] as i32, bzp_bit!(), out_data);
        }
    }
}


fn bzp_write_select(out_data: &mut Box<BzpOutComdata>, huffman: &mut Box<BzpHuffmanGroups>) {
    bzp_write_to_array(huffman.n_select, bzp_bits15!(), out_data);
    println!("compress n_select: {}", huffman.n_select);
    for i in 0..huffman.n_select {
        for j in 0..huffman.select_mtf[i as usize] {
            bzp_write_to_array(1, bzp_bit!(), out_data);
        }
        bzp_write_to_array(0, bzp_bit!(), out_data);
    }
}

fn bzp_write_len(out_data: &mut Box<BzpOutComdata>, huffman: &mut Box<BzpHuffmanGroups>) {
    for i in 0..huffman.n_groups {
        let mut val = huffman.huffman_groups[i as usize].len[0];
        bzp_write_to_array(val, bzp_bits5!(), out_data);
        for j in 0..huffman.alpha_size {
            let tar = huffman.huffman_groups[i as usize].len[j as usize];
            let mut deta = 0;
            let mut save_val = 0;
            if val < tar {
                save_val = bzp_huffman_len_increase!();
                deta = 1;
            } else if val > tar {
                save_val = bzp_huffman_len_reduced!();
                deta = -1;
            }
            while val != tar {
                bzp_write_to_array(save_val, bzp_bits2!(), out_data);
                val += deta;
            }
            bzp_write_to_array(0, bzp_bit!(), out_data);
        }
    }
}

fn bzp_write_input_encode(out_data: &mut Box<BzpOutComdata>, mtf: &mut Box<BzpMtfInfo>, huffman: &mut Box<BzpHuffmanGroups>) {
    for i in 0..mtf.n_mtf {
        let val = mtf.mtf_v[i as usize];
        let gid = huffman.select[i as usize / bzp_elems_num_in_one_group!()];
        let code = huffman.huffman_groups[gid as usize].table[val as usize];
        let len = huffman.huffman_groups[gid as usize].len[val as usize];
        bzp_write_to_array(code, len, out_data);
    }
}

fn bzp_write_file_end(out_data: &mut Box<BzpOutComdata>, combined_crc: i32) {
    bzp_write_to_array(bzp_file_end_0!(), bzp_bits8!(), out_data);
    bzp_write_to_array(bzp_file_end_1!(), bzp_bits8!(), out_data);
    bzp_write_to_array(bzp_file_end_2!(), bzp_bits8!(), out_data);
    bzp_write_to_array(bzp_file_end_3!(), bzp_bits8!(), out_data);
    bzp_write_to_array(bzp_file_end_4!(), bzp_bits8!(), out_data);
    bzp_write_to_array(bzp_file_end_5!(), bzp_bits8!(), out_data);
    println!("compress combined_crc: {:x}", combined_crc);
    bzp_write_int32(combined_crc, out_data);
}

fn bzp_flushbuf(out_data: &mut Box<BzpOutComdata>) {
    while out_data.n_buf > 0 {
        out_data.out[out_data.num as usize] = (out_data.buf >> bzp_bits24!()) as u8;
        out_data.num += 1;
        out_data.n_buf -= bzp_bits8!();
        out_data.buf <<= bzp_bits8!();
    }
}

fn bzp_compress_one_block(bwt: &mut Box<BzpBwtInfo>, mtf: &mut Box<BzpMtfInfo>, huffman: &mut Box<BzpHuffmanGroups>, out_data: &mut Box<BzpOutComdata>) -> i32 {
    let mut ret = bzp_ok!();
    if bwt.n_block == 0 {
        return bzp_ok!();
    }
    bzp_write_file_head(out_data, bwt.block_id);
    if bwt.n_block > 0 {
        bzp_calculate_crc(bwt);
        bzp_block_sort_main(bwt);
        println!("compress ori_ptr at here: {:x}", bwt.ori_ptr);
        bzp_mtf_reset(mtf);
        mtf.block = bwt.block.to_vec();
        mtf.map = bwt.sort_block.to_vec();
        mtf.in_use = bwt.in_use.to_vec();
        mtf.n_block = bwt.n_block;
        bzp_mtf_main(mtf);
        ret = bzp_huffman_groups_reset(huffman, mtf.n_use + bzp_extra_chars_num!());
        if ret != bzp_ok!() {
            return ret;
        }
        huffman.block = mtf.mtf_v.to_vec();
        huffman.mtf_freq = mtf.mtf_freq.to_vec();
        huffman.n_block = mtf.n_mtf;
        bzp_huffman_main(huffman);
        bzp_write_block_head(out_data, bwt);
        bzp_write_valid_ascii(out_data, bwt);
        println!("compress n_groups: {}", huffman.n_groups);
        bzp_write_to_array(huffman.n_groups, bzp_bits3!(), out_data);
        bzp_write_select(out_data, huffman);
        bzp_write_len(out_data, huffman);
        bzp_write_input_encode(out_data, mtf, huffman);
    }
    return bzp_ok!();
}

fn bzp_buff_to_stream(bzpf: &mut Box<BzpFile>, out_data: &mut Box<BzpOutComdata>) -> i32 {
    bzpf.output.pos = 0;
    let mut pos = 0;
    while pos < out_data.num {
        bzpf.output.n_buf = 0;
        while pos < out_data.num && bzpf.output.n_buf < bzp_buf_size!() {
            bzpf.output.buf[bzpf.output.n_buf as usize] = out_data.out[pos as usize];
            bzpf.output.n_buf += 1;
            pos += 1;         
        }
        let n2 = bzpf.output.file_ptr.as_mut().unwrap().write(&bzpf.output.buf[0..bzpf.output.n_buf as usize]);
        if n2.is_err() || n2.unwrap() != bzpf.output.n_buf as usize {
            return bzp_error_io!();
        }
    }
    return bzp_ok!();
}

fn bzp_add_char_to_block(lasch: i32, num: i32, bwt: &mut Box<BzpBwtInfo>) {
    if num < bzp_rlc_num_lower_limit!() || num > bzp_rlc_num_upper_limit!() {
        return;
    }
    for _ in 0..num {
        bzp_update_crc!(bwt.block_crc, lasch);
    }
    let val = bzp_min_fun!(num, bzp_rlc_num_4!() as i32);
    match val {
        bzp_rlc_num_4!() => {
            bwt.block[bwt.n_block as usize] = lasch as u8;
            bwt.n_block += 1;
            bwt.block[bwt.n_block as usize] = lasch as u8;
            bwt.n_block += 1;
            bwt.block[bwt.n_block as usize] = lasch as u8;
            bwt.n_block += 1;
            bwt.block[bwt.n_block as usize] = lasch as u8;
            bwt.n_block += 1;
        }
        bzp_rlc_num_3!() => {
            bwt.block[bwt.n_block as usize] = lasch as u8;
            bwt.n_block += 1;
            bwt.block[bwt.n_block as usize] = lasch as u8;
            bwt.n_block += 1;
            bwt.block[bwt.n_block as usize] = lasch as u8;
            bwt.n_block += 1;
        }
        bzp_rlc_num_2!() => {
            bwt.block[bwt.n_block as usize] = lasch as u8;
            bwt.n_block += 1;
            bwt.block[bwt.n_block as usize] = lasch as u8;
            bwt.n_block += 1;
        }
        bzp_rlc_num_1!() => {
            bwt.block[bwt.n_block as usize] = lasch as u8;
            bwt.n_block += 1;
        }
        _ => {}
    }
    if num >= bzp_rlc_num_4!() {
        bwt.block[bwt.n_block as usize] = (num - bzp_rlc_num_4!()) as u8;
        bwt.n_block += 1;
        bwt.in_use[(num - bzp_rlc_num_4!()) as usize] = true;
    }
    bwt.in_use[lasch as usize] = true;
}

fn bzp_buff_to_block_rlc(bzpf: &mut Box<BzpFile>, bwt: &mut Box<BzpBwtInfo>, is_last_data: bool) {
    while !bzp_block_full!(bwt) && !bzp_buff_read_empty!(bzpf) {
        let pos = bzpf.input.pos;
        let ch = bzpf.input.buf[pos as usize];
        let lasch = bzpf.las_char as u8;
        if ch != lasch || bzpf.num == bzp_rlc_num_upper_limit!() {
            bzp_add_char_to_block(lasch as i32, bzpf.num, bwt);
            bzpf.las_char = ch as i32;
            bzpf.num = 1;
        } else {
            bzpf.num += 1;
        }
        bzpf.input.pos += 1;
    }

    if is_last_data && bzp_buff_read_empty!(bzpf) {
        bzp_add_char_to_block(bzpf.las_char, bzpf.num, bwt);
        bzpf.las_char = bzp_ascii_size!();
        bzpf.num = 0;
    }
}

fn bzp_reset_compress(bwt: &mut Box<BzpBwtInfo>, out_data: &mut Box<BzpOutComdata>) {
    out_data.num = 0;
    bwt.n_block = 0;
    bwt.block_crc = bzp_init_block_crc!();
    for i in 0..bzp_ascii_size!() {
        bwt.in_use[i as usize] = false;
    }
    let n = out_data.block_size * bzp_base_block_size!();
    for i in 0..n {
        bwt.is_start_pos[i as usize] = 0;
    }
    bwt.block_id += 1;
}


fn bzp_process_data(bzp_info: &mut Box<BzpAlgorithmInfo>, is_last_data: bool) -> i32 {
    let bzpf = &mut bzp_info.compress_file;
    let mtf = &mut bzp_info.mtf;
    let huffman = &mut bzp_info.huffman;
    let out_data = &mut bzp_info.out_data;
    let bwt = &mut bzp_info.bwt;

    println!("bwt.n_block = {}", bwt.n_block);

    bzpf.state = bzp_input_compress!();
    let mut ret = bzp_ok!();
    while bzpf.state != bzp_retuen_compress!() {
        if bzpf.state == bzp_output_compress!() {
            ret = bzp_buff_to_stream(bzpf, out_data);
            bzp_reset_compress(bwt, out_data);
            bzpf.state = bzp_input_compress!();
            if is_last_data && bzp_buff_read_empty!(bzpf) {
                bzpf.state = bzp_retuen_compress!();
            }
        }
        if bzpf.state == bzp_input_compress!() {
            bzp_buff_to_block_rlc(bzpf, bwt, is_last_data);
            if is_last_data && bzp_buff_read_empty!(bzpf) {
                ret = bzp_compress_one_block(bwt, mtf, huffman, out_data);
                bzp_write_file_end(out_data, bwt.combined_crc as i32);
                bzp_flushbuf(out_data);
                bzpf.state = bzp_output_compress!();
            } else if bzp_block_full!(bwt) {
                ret = bzp_compress_one_block(bwt, mtf, huffman, out_data);
                bzpf.state = bzp_output_compress!();
            } else {
                bzpf.state = bzp_retuen_compress!();
            }
        }
        if ret != bzp_ok!() {
            return ret;
        }
    }
    return ret;
}

fn bzp_compress_end(bzp_info: Box<BzpAlgorithmInfo>) {
    // if bzp_info.compress_file.as_ref().unwrap().input.as_ref().unwrap().file_ptr.is_some() {
    //     bzp_info.compress_file.as_ref().unwrap().input.as_ref().unwrap().file_ptr.as_ref().unwrap().close();
    // }
    // if bzp_info.compress_file.as_ref().unwrap().output.as_ref().unwrap().file_ptr.is_some() {
    //     bzp_info.compress_file.as_ref().unwrap().output.as_ref().unwrap().file_ptr.as_ref().unwrap().close();
    // }
    bzp_algorithm_info_finish(bzp_info);
}

pub fn bzp_compress_stream(in_name: &str, out_name: &str, block_size: i32) -> i32 {
    let mut ret = bzp_ok!();
    let mut is_last_data = false;
    if in_name.is_empty() || out_name.is_empty() || bzp_invalid_block_size!(block_size) {
        return bzp_error_param!();
    }
    let Some(mut bzp_info) = bzp_algorithm_info_init(block_size) else {
        return bzp_error_memory_open_failure!();
    };
    ret = bzp_open_file(&mut bzp_info, in_name, out_name);
    if ret != bzp_ok!() {
        return ret;
    }
    
    while !is_last_data {
        let in_stream = &mut bzp_info.compress_file.input;
        in_stream.n_buf = in_stream.file_ptr.as_mut().unwrap().read(&mut in_stream.buf).ok().unwrap() as i32;
        in_stream.pos = 0;
        is_last_data = bzp_file_eof(in_stream.file_ptr.as_mut().unwrap());
        ret = bzp_process_data(&mut bzp_info, is_last_data);
        if ret != bzp_ok!() {
            break;
        }
    }
    bzp_compress_end(bzp_info);
    if ret != bzp_ok!() {
        std::fs::remove_file(out_name);
    }
    return ret;
}


