use crate::rapidlz_log::*;
use crate::rapidlz_inner::*;

use std::sync::Mutex;

macro_rules! sixteen_byte { () => { 16 }; }
pub(crate) use sixteen_byte;

macro_rules! eight_byte { () => { 8 }; }
pub(crate) use eight_byte;

macro_rules! copy_protect_size { () => { 16 }; }
pub(crate) use copy_protect_size;

macro_rules! max_4bit_match { () => { 19 }; }
pub(crate) use max_4bit_match;

static OVERLAP_OFF_ADD_VAL: Mutex<[u8; 8]> = Mutex::new([0, 1, 2, 2, 4, 3, 2, 1]);

macro_rules! read_optional_length {
    ($len:expr, $src_curr:expr, $src_end:expr, $temp:expr) => {
        unsafe {
            if likely!(($src_curr as usize) < ($src_end as usize)) {
                $temp = *$src_curr;
                $src_curr = $src_curr.add(1);
                $len += $temp as u32;
            }
            while $temp == max_byte_value!() && ($src_curr as usize) < ($src_end as usize) {
                $temp = *$src_curr;
                $src_curr = $src_curr.add(1);
                $len += $temp as u32;
            }
        }
    };
}

macro_rules! safe_copy_match {
    ($dst_curr:expr, $match_src:expr, $match_length:expr) => {
        unsafe {
            while $match_length > 0 {
                $match_length -= 1;
                *$dst_curr = *$match_src;
                $dst_curr = $dst_curr.add(1);
                $match_src = $match_src.add(1);
            }
        }
    };
}

fn rapidlz_copy_literals_fast(src: &mut [u8], dst: &mut [u8], length: u32) {
    if likely!(length <= sixteen_byte!()) {
        copy_16byte(dst, src);
        return;
    }
    let dst_end = unsafe { dst.as_mut_ptr().add(length as usize) };
    rapidlz_wild_copy16(src, dst, dst_end);
}

fn rapidlz_copy_match_fast(dst: &mut [u8], match_src: &mut [u8], offset: u16, length: u32) {
    let mut dst_curr = dst;
    let mut match_ptr = match_src;

    if offset >= sixteen_byte!() {
        rapidlz_copy_literals_fast(match_ptr, dst_curr, length);
        return;
    }

    for i in 0..eight_byte!() {
        dst_curr[i] = match_ptr[i];
    }

    if length <= eight_byte!() {
        return;
    }

    let dst_end = unsafe { dst_curr.as_mut_ptr().add(length as usize) };
    if offset < eight_byte!() {
        match_ptr = &mut match_ptr[offset as usize..];
        dst_curr = &mut dst_curr[eight_byte!()..];
    }

    loop {
        copy_8byte(dst_curr, match_ptr);
        dst_curr = &mut dst_curr[eight_byte!()..];
        match_ptr = &mut match_ptr[eight_byte!()..];
        if (dst_curr.as_ptr() as usize) >= (dst_end as usize) {
            break;
        }
    }
}

pub fn rapidlz_decompress(src: &mut [u8], dst: &mut [u8], src_size: usize, dst_size: usize) -> usize {
    if src.is_empty() || dst.is_empty() || src_size == 0 || dst_size == 0 {
        rapidlz_log!(rapidlz_input_invalid!(), "input invalid\n");
        return 0;
    }

    let mut token: u8;
    let mut temp = 0;
    let mut offset: u16;
    let mut lit_len: u32;
    let mut match_len: u32;
    let mut match_src;
    let mut src_end = unsafe { src.as_mut_ptr().add(src_size) };
    let mut src_curr = src.as_mut_ptr();
    let mut src_end_fast = unsafe { src_end.sub(copy_protect_size!()) };
    let mut dst_end = unsafe { dst.as_mut_ptr().add(dst_size) };
    let mut dst_curr = dst.as_mut_ptr();
    let mut dst_end_fast = unsafe { dst_end.sub(copy_protect_size!()) };

    while src_curr < src_end {
    'read_match: {
        token = unsafe { *src_curr };
        src_curr = unsafe { src_curr.add(1) };
        lit_len = (token >> 4) as u32;

        if likely!(lit_len < max_4bit_match!()) {
            if likely!(src_curr as usize + lit_len as usize <= src_end_fast as usize 
                    && dst_curr as usize + lit_len as usize <= dst_end_fast as usize) {
                copy_16byte(wrap_ptr!(src_curr, src_end), wrap_ptr!(dst_curr, dst_end));
                dst_curr = unsafe { dst_curr.add(lit_len as usize) };
                src_curr = unsafe { src_curr.add(lit_len as usize) };
                break 'read_match;
            }
        } else {
            read_optional_length!(lit_len, src_curr, src_end, temp);
            if likely!(src_curr as usize + lit_len as usize <= src_end_fast as usize 
                && dst_curr as usize + lit_len as usize <= dst_end_fast as usize) {
                rapidlz_wild_copy16(wrap_ptr!(src_curr, src_end), wrap_ptr!(dst_curr, dst_end), unsafe { dst_curr.add(lit_len as usize) });
                dst_curr = unsafe { dst_curr.add(lit_len as usize) };
                src_curr = unsafe { src_curr.add(lit_len as usize) };
                break 'read_match;
            }
        }

        let left_src_size = src_end as usize - src_curr as usize;
        if unlikely!(lit_len as usize > left_src_size) {
            rapidlz_log!(rapidlz_dst_size_small!(), "lit_len:{} dst_end - dst:{}\n", lit_len, left_src_size);
            return 0;
        }
            
        dst_curr = unsafe { dst_curr.add(lit_len as usize) };
        src_curr = unsafe { src_curr.add(lit_len as usize) };

        if left_src_size == lit_len as usize {
            return dst_curr as usize - dst.as_mut_ptr() as usize;
        }
    } // 'read_match

        offset = read_le16bit(wrap_ptr!(src_curr, src_end));
        println!("decompress offset: {}", offset);
        src_curr = unsafe { src_curr.add(2) };
        println!("decompress src_curr: {:x}", src_curr as usize);
        match_src = unsafe { dst_curr.sub(offset as usize) };
        println!("decompress match_src: {:x}", match_src as usize);
        println!("decompress dst: {:x}", dst.as_mut_ptr() as usize);
        if unlikely!(match_src < dst.as_mut_ptr()) {
            rapidlz_log!(rapidlz_format_invalid!(), "rapidlz format invalid\n");
            return 0;
        }
        match_len = ((token & max_4bit_match!()) + min_match!()) as u32;
        if match_len == max_4bit_match!() {
            read_optional_length!(match_len, src_curr, src_end, temp);
        }
        if likely!(dst_curr as usize + match_len as usize <= dst_end_fast as usize) {
            rapidlz_copy_match_fast(wrap_ptr!(dst_curr, dst_end), wrap_ptr!(match_src, dst_end), offset, match_len);
            dst_curr = unsafe { dst_curr.add(match_len as usize) };
        } else {
            if dst_curr as usize + match_len as usize > dst_end as usize {
                rapidlz_log!(rapidlz_dst_size_small!(), "dst_end - dst_curr:{} match_len:{}\n", dst_end as usize - dst_curr as usize, match_len);
                return 0;
            }
            safe_copy_match!(dst_curr, match_src, match_len);
        }
    }
    dst_curr as usize - dst.as_mut_ptr() as usize
}