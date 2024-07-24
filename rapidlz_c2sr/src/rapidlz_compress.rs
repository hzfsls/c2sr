use crate::rapidlz_log::*;
use crate::rapidlz_inner::*;

macro_rules! max_input_size { () => { 0x7E000000 }; }

macro_rules! rapidlz_compressbound {
    ($size:expr) => {
        if $size > max_input_size!() {
            0
        } else {
            $size + $size / 255 + 16
        }
    };
}

macro_rules! rapidlz_acceleration_max { () => { 10 }; }

macro_rules! src_size_threshold { () => { 65536 }; }

macro_rules! max_offset { () => { 65535 }; }

macro_rules! last_literals { () => { 6 }; }

macro_rules! min_compress_size { () => { 16 }; }

macro_rules! hash_type_4 { () => { 4 }; }

macro_rules! hash_type_5 { () => { 5 }; }

macro_rules! min_hash_bit { () => { 6 }; }

macro_rules! max_hash_bit { () => { 12 }; }

macro_rules! step_forward_base { () => { 6 }; }

macro_rules! expand_forward {
    ($srcBegin:expr, $matchBegin:expr, $srcCurr:expr, $srcAnchor:expr) => {
        unsafe {
            while $srcBegin < $matchBegin && $srcCurr > $srcAnchor 
                && unlikely!($matchBegin.sub(1).read() == $srcCurr.sub(1).read()) {
                $matchBegin = $matchBegin.sub(1);
                $srcCurr = $srcCurr.sub(1);
            }
        }
    }
}

struct RapidlzCCtx {
    hash_table: Vec<u8>,
    hash_type: u8,
    hash_bits: u8,
    step: u8,
    buffer_limit: u8,
}

impl RapidlzCCtx {
    fn new() -> Self {
        Self {
            hash_table: vec![],
            hash_type: 0,
            hash_bits: 0,
            step: 0,
            buffer_limit: 0,
        }
    }
}

const RAPIDLZ_VERSION: &str = "rapidlz 3.24.00.SPC010B100";

pub fn rapidlz_version_get() -> &'static str {
    RAPIDLZ_VERSION
}

pub fn rapidlz_compress_bound(src_size: usize) -> usize {
    rapidlz_compressbound!(src_size)
}

fn put_pos_on_table(pos: u32, hash_value: u32, hash_table: &mut [u8], hash_type: u8) {
    if hash_type == 4 {
        unsafe {
            *((hash_table.as_mut_ptr() as *mut u16).add(hash_value as usize)) = pos as u16;
        }
    } else if hash_type == 5 {
        unsafe {
            *((hash_table.as_mut_ptr() as *mut u32).add(hash_value as usize)) = pos;
        }
    }
}

fn get_pos_on_table(hash_value: u32, hash_table: &mut [u8], hash_type: u8) -> u32 {
    if hash_type == 4 {
        unsafe {
            *((hash_table.as_ptr() as *const u16).add(hash_value as usize)) as u32
        }
    } else if hash_type == 5 {
        unsafe {
            *((hash_table.as_ptr() as *const u32).add(hash_value as usize)) as u32
        }
    } else {
        0
    }
}

fn calc_hash_value(src_curr: &mut [u8], hash_type: u8, hash_bits: u8) -> u32 {
    if hash_type == 5 {
        ((read64bit!(src_curr.as_mut_ptr()) << 24) * 11400714819323198485 >> (64 - hash_bits)) as u32
    } else {
        (read32bit!(src_curr.as_mut_ptr()) * 2654435769 >> (32 - hash_bits)) as u32
    }
}

fn rapidlz_compress_expand_backward<'a>(match_limit: *mut u8, match_ptr: &mut [u8], src_curr: &'a mut [u8]) -> &'a mut [u8] {
    let mut xor_val: u64;
    let loop_end = unsafe { match_limit.sub(7) };
    let mut src_curr_match_end = src_curr;
    let mut match_begin = match_ptr;

    while (src_curr_match_end.as_ptr() as usize) < (loop_end as usize) {
        xor_val = read64bit!(match_begin.as_mut_ptr()) ^ read64bit!(src_curr_match_end.as_mut_ptr());
        if unlikely!(xor_val == 0) {
            src_curr_match_end = &mut src_curr_match_end[8..];
            match_begin = &mut match_begin[8..];
            continue;
        }
        src_curr_match_end = if is_le() {
            &mut src_curr_match_end[(count_tail_zero64(xor_val) >> 3) as usize..]
        } else {
            &mut src_curr_match_end[(count_lead_zero64(xor_val) >> 3) as usize..]
        };
        return src_curr_match_end;
    }

    if ((src_curr_match_end.as_ptr() as usize) + 3 < (match_limit as usize)) && (read32bit!(src_curr_match_end.as_mut_ptr()) == read32bit!(match_begin.as_mut_ptr())) {
        src_curr_match_end = &mut src_curr_match_end[4..];
        match_begin = &mut match_begin[4..];
    }

    if ((src_curr_match_end.as_ptr() as usize) + 1 < (match_limit as usize)) && (read16bit!(src_curr_match_end.as_mut_ptr()) == read16bit!(match_begin.as_mut_ptr())) {
        src_curr_match_end = &mut src_curr_match_end[2..];
        match_begin = &mut match_begin[2..];
    }

    src_curr_match_end
}

fn rapidlz_compress_store_optional_length(dst: &mut [u8], lit_length: u32) -> &mut [u8] {
    let mut dst_curr = dst;
    let mut length = lit_length;

    if length < 255 {
        dst_curr[0] = length as u8;
        dst_curr = &mut dst_curr[1..];
        return dst_curr;
    }

    loop {
        dst_curr[0] = 255;
        dst_curr = &mut dst_curr[1..];
        length -= 255;
        if length < 255 {
            break;
        }
    }

    dst_curr[0] = length as u8;
    dst_curr = &mut dst_curr[1..];
    dst_curr
}

fn rapidlz_store_last_literals<'a>(dst: &'a mut [u8], dst_end: *mut u8, src_curr: &mut [u8], lit_length: u32, buffer_limit: u8) -> Option<&'a mut [u8]> {
    let mut dst_curr = dst;
    let mut length = lit_length;

    if buffer_limit != 0 {
        let lit_tok_size = 1 + lit_length + (lit_length / 255);
        if (dst_curr.as_ptr() as usize) + lit_tok_size as usize > (dst_end as usize) {
            rapidlz_log!(rapidlz_dst_size_small!(), "dstEnd - dstCur:{} litTokSize:{}", dst_end as usize - dst_curr.as_ptr() as usize, lit_tok_size);
            return None;
        }
    }

    let token: u8 = if lit_length < max_4bit_value!() {
        lit_length as u8
    } else {
        max_4bit_value!() 
    };
    dst_curr[0] = token << 4;
    dst_curr = &mut dst_curr[1..];

    if lit_length >= max_4bit_value!() {
        dst_curr = rapidlz_compress_store_optional_length(dst_curr, lit_length - max_4bit_value!());
    }

    if (src_curr.as_ptr() as usize) + lit_length as usize > (dst_end as usize) {
        return None;
    }

    dst_curr[..lit_length as usize].copy_from_slice(&src_curr[..lit_length as usize]);

    Some(&mut dst_curr[lit_length as usize..])
}

fn rapidlz_store_off_match<'a>(dst: &'a mut [u8], token: &mut [u8], match_length: u32, offset: u16) -> &'a mut [u8] {
    let mut dst_curr = dst;

    write_le16(dst_curr, offset);
    dst_curr = &mut dst_curr[2..];

    if match_length >= max_4bit_value!() {
        let mut optional_len = match_length - max_4bit_value!();
        token[0] += max_4bit_value!();
        while optional_len >= max_byte_value!() {
            dst_curr[0] = max_byte_value!();
            dst_curr = &mut dst_curr[1..];
            optional_len -= max_byte_value!();
        }
        dst_curr[0] = optional_len as u8;
        dst_curr = &mut dst_curr[1..];
    } else {
        token[0] += match_length as u8;
    }

    dst_curr
}

fn rapidlz_store_sequence<'a>(dst: &'a mut [u8], src_anchor: &mut [u8], literal_length: u32, match_length: u32, offset: u16) -> &'a mut [u8] {
    let (mut token, mut dst_curr) = dst.split_at_mut(1);
    if literal_length >= max_4bit_value!() {
        token[0] = max_4bit_value!() << 4;
        let mut optional_len = literal_length - max_4bit_value!();
        while optional_len >= max_byte_value!() {
            dst_curr[0] = max_byte_value!();
            dst_curr = &mut dst_curr[1..];
            optional_len -= max_byte_value!();
        }
        dst_curr[0] = optional_len as u8;
        dst_curr = &mut dst_curr[1..];
        copy_16byte(dst_curr, src_anchor);
        if literal_length > 16 {
            let dst_end = unsafe {dst_curr.as_mut_ptr().add(literal_length as usize)};
            rapidlz_wild_copy16(&mut src_anchor[16..], dst_curr, dst_end);
        }
        dst_curr = &mut dst_curr[literal_length as usize..];
    } else if literal_length > 0 {
        token[0] = (literal_length as u8) << 4;
        let dst_end = unsafe {dst_curr.as_mut_ptr().add(literal_length as usize)};
        rapidlz_wild_copy16(src_anchor, dst_curr, dst_end);
        dst_curr = &mut dst_curr[literal_length as usize..];
    } else {
        token[0] = 0;
    }

    rapidlz_store_off_match(dst_curr, token, match_length, offset)
}

fn rapidlz_compress_process(dst: &mut [u8], dst_size: usize, src: &mut [u8], src_size: usize, c_ctx: &mut Box<RapidlzCCtx>) -> usize {
    let mut hash_value: u32;
    let mut match_length: u32;
    let mut literal_length: u32;
    let mut step: u32 = 1;
    let mut offset: u16;
    let mut hash_table = c_ctx.hash_table.as_mut_slice();
    let mut src_begin = src.as_mut_ptr();
    let mut src_end = unsafe { src.as_mut_ptr().add(src_size) };
    let mut src_curr = unsafe { src_begin.add(1) };
    let mut src_curr_match_end;
    let mut src_anchor = src_begin;
    let mut match_begin;
    let match_limit = unsafe { src_end.sub(last_literals!() as usize) };
    let src_limit = unsafe { src_end.sub(min_compress_size!() as usize) };
    let dst_begin = dst.as_mut_ptr();
    let dst_end = unsafe { dst.as_mut_ptr().add(dst_size) };
    let mut dst_curr = dst_begin;
    let hash_type = c_ctx.hash_type;
    let hash_bits = c_ctx.hash_bits;
    let mut search_match_nb = c_ctx.step << step_forward_base!();
    let mut search_match_nb_tmp = search_match_nb;
    let buffer_limit = c_ctx.buffer_limit;
    
    while likely!((src_curr as usize) <= (src_limit as usize)) {
        loop {
            hash_value = calc_hash_value(wrap_ptr!(src_curr, src_end), hash_type, hash_bits);
            match_begin = unsafe { src_begin.add(get_pos_on_table(hash_value, hash_table, hash_type) as usize) } ; 
            put_pos_on_table((src_curr as usize - src_begin as usize) as u32, hash_value, hash_table, hash_type);

            if (read32bit!(src_curr) == read32bit!(match_begin)) && likely!((src_curr as usize - match_begin as usize) <= max_offset!()) {
                break;
            }

            src_curr = unsafe { src_curr.add(step as usize) };
            step = (search_match_nb_tmp >> step_forward_base!()) as u32 + 1;

            if src_curr > src_limit {
                if let Some(dst_curr) = rapidlz_store_last_literals(wrap_ptr!(dst_curr, dst_end), dst_end, wrap_ptr!(src_anchor, src_end), (src_end as usize - src_anchor as usize) as u32, buffer_limit) {
                    return dst_curr.as_ptr() as usize - dst_begin as usize;
                } else {
                    return 0;
                }
            }
        }
    
        step = 1;
        search_match_nb_tmp = search_match_nb;

        src_curr_match_end = rapidlz_compress_expand_backward(match_limit, wrap_ptr!(match_begin, match_limit), wrap_ptr!(src_curr, src_end)).as_mut_ptr();
        expand_forward!(src_begin, match_begin, src_curr, src_anchor);
        match_length = (src_curr_match_end as usize - src_curr as usize) as u32 - min_match!();
        offset = (src_curr as usize - match_begin as usize) as u16;
        literal_length = (src_curr as usize - src_anchor as usize) as u32;
        if buffer_limit != 0 {
            let write_size = literal_length + 8 + (literal_length + match_length / max_byte_value!());
            if unlikely!(dst_curr as usize + write_size as usize > dst_end as usize) {
                rapidlz_log!(rapidlz_dst_size_small!(), "dstEnd - dstCur:{} writeSize:{}", dst_end as usize - dst_curr as usize, write_size);
                return 0;
            }
        }
        unsafe { *dst_curr = 0; }
        dst_curr = rapidlz_store_sequence(wrap_ptr!(dst_curr, dst_end), wrap_ptr!(src_anchor, src_end), literal_length, match_length, offset).as_mut_ptr();

        src_curr = src_curr_match_end;
        src_anchor = src_curr;

        hash_value = calc_hash_value(wrap_ptr!(src_curr.sub(2), src_end), hash_type, hash_bits);
        put_pos_on_table((src_curr as usize - 2 - src_begin as usize) as u32, hash_value, hash_table, hash_type);
    }

    if src_anchor < src_end {
        if let Some(dst_curr) = rapidlz_store_last_literals(wrap_ptr!(dst_curr, dst_end), dst_end, wrap_ptr!(src_anchor, src_end), (src_end as usize - src_anchor as usize) as u32, buffer_limit) {
            return dst_curr.as_ptr() as usize - dst_begin as usize;
        } else {
            return 0;
        }
    }
    dst_curr as usize - dst_begin as usize
}

fn rapidlz_c_ctx_free(c_ctx: Box<RapidlzCCtx>) {

}

pub fn rapidlz_compress(src: &mut [u8], dst: &mut [u8], src_size: usize, dst_size: usize, acceleration: i32) -> usize {
    if src.is_empty() || dst.is_empty() || src_size == 0 || dst_size == 0 {
        rapidlz_log!(rapidlz_input_invalid!(), "input invalid");
        return 0;
    }

    if acceleration < 1 || acceleration > rapidlz_acceleration_max!() {
        rapidlz_log!(rapidlz_input_invalid!(), "acceleration:{}", acceleration);
        return 0;
    }

    let mut c_ctx = Box::new(RapidlzCCtx::new());
    c_ctx.hash_bits = min_hash_bit!();
    let total_hash_size: usize;
    if src_size <= src_size_threshold!() {
        c_ctx.hash_type = hash_type_4!();
        if src_size >= 64 {
            c_ctx.hash_bits = if high_bit64(src_size as u64) > max_hash_bit!() {
                max_hash_bit!() + 1
            } else {
                high_bit64(src_size as u64)
            };
        }
        total_hash_size = core::mem::size_of::<u16>() * (1 << c_ctx.hash_bits);
    } else {
        c_ctx.hash_type = hash_type_5!();
        c_ctx.hash_bits = max_hash_bit!();
        total_hash_size = core::mem::size_of::<u32>() * (1 << c_ctx.hash_bits);
    }

    let mut table = vec![0; total_hash_size];
    c_ctx.hash_table = table;
    c_ctx.step = acceleration as u8;
    c_ctx.buffer_limit = if dst_size < rapidlz_compress_bound(src_size) {
        1
    } else {
        0
    };

    let c_size = rapidlz_compress_process(dst, dst_size, src, src_size, &mut c_ctx);
    rapidlz_c_ctx_free(c_ctx);
    c_size
}

pub fn rapidlz_compress_default(src: &mut [u8], dst: &mut [u8], src_size: usize, dst_size: usize) -> usize {
    rapidlz_compress(src, dst, src_size, dst_size, 1)
}