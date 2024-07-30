macro_rules! vos_sha256_ctx_hash_len { () => { 8 }; }
pub(crate) use vos_sha256_ctx_hash_len;

macro_rules! vos_sha256_ctx_buf_len { () => { 16 }; }
pub(crate) use vos_sha256_ctx_buf_len;

macro_rules! sha256_block_size { () => { 64 }; }
pub(crate) use sha256_block_size;

macro_rules! sha256_digest_size { () => { 32 }; }
pub(crate) use sha256_digest_size;

pub fn c2sr_memcpy<A, B> (dest: &mut [B], src: &[A], size: usize) {
    unsafe {
        std::ptr::copy_nonoverlapping(
            src.as_ptr() as *const B,
            dest.as_mut_ptr(),
            size
        );
    }
}


pub struct VosSha256Ctx {
    pub h: [u32; 8],
    pub n: [u32; 2],
    pub block: [u32; sha256_block_size!() / std::mem::size_of::<u32>()],
    pub block_len: u32,
    pub out_len: u32,
    pub computed: u32,
    pub corrupted: u32,
}

impl VosSha256Ctx {
    pub fn new() -> Self {
        Self {
            h: [0; 8],
            n: [0; 2],
            block: [0; sha256_block_size!() / std::mem::size_of::<u32>()],
            block_len: 0,
            out_len: sha256_digest_size!(),
            computed: 0,
            corrupted: 0,
        }
    }
}


macro_rules! sha256_ok { () => { 0 }; }
pub(crate) use sha256_ok;

macro_rules! sha256_error { () => { !0 }; }
pub(crate) use sha256_error;

macro_rules! bits_pre_byte { () => { 8 }; }
pub(crate) use bits_pre_byte;

macro_rules! shifts_per_byte { () => { 3 }; }
pub(crate) use shifts_per_byte;

macro_rules! bitsize { ($t:ty) => { std::mem::size_of::<$t>() * bits_pre_byte!() }; }
pub(crate) use bitsize;

macro_rules! put_uint32_be { 
    ($v:expr, $p:expr, $i:expr) => {
        {
            $p[$i as usize + 0] = ($v >> 24) as u8;
            $p[$i as usize  + 1] = ($v >> 16) as u8;
            $p[$i as usize  + 2] = ($v >>  8) as u8;
            $p[$i as usize  + 3] = ($v >>  0) as u8;
        }
    } 
}
pub(crate) use put_uint32_be;

macro_rules! get_uint32_be { 
    ($p:expr, $i:expr) => {
        {
            ((($p[$i as usize  + 0] as u32) << 24) | 
            (($p[$i as usize  + 1] as u32) << 16) | 
            (($p[$i as usize  + 2] as u32) <<  8) | 
            (($p[$i as usize  + 3] as u32) <<  0))
        }
    } 
}
pub(crate) use get_uint32_be;

pub fn vos_sha256_begin(pst_ctx: &mut Box<VosSha256Ctx>) {
    *pst_ctx =  Box::new(VosSha256Ctx::new());
    pst_ctx.h[0] = 0x6a09e667;
    pst_ctx.h[1] = 0xbb67ae85;
    pst_ctx.h[2] = 0x3c6ef372;
    pst_ctx.h[3] = 0xa54ff53a;
    pst_ctx.h[4] = 0x510e527f;
    pst_ctx.h[5] = 0x9b05688c;
    pst_ctx.h[6] = 0x1f83d9ab;
    pst_ctx.h[7] = 0x5be0cd19;
    pst_ctx.out_len = sha256_digest_size!();
}

pub fn vos_sha256_ctx_prepare(pst_ctx: &mut Box<VosSha256Ctx>, ui_len: u32) -> u32 {
    let mut ui_cnt_first: u32;
    let mut ui_cnt_sec: u32;

    ui_cnt_first = (pst_ctx.n[0] + (ui_len << shifts_per_byte!())) & 0xffffffff;
    if ui_cnt_first < pst_ctx.n[0] {
        pst_ctx.n[1] += 1;
        if pst_ctx.n[1] == 0 {
            pst_ctx.corrupted = 1;
            return sha256_error!();
        }
    }

    ui_cnt_sec = pst_ctx.n[1] + (ui_len >> (bitsize!(u32) - shifts_per_byte!()));
    if ui_cnt_sec < pst_ctx.n[1] {
        pst_ctx.corrupted = 1;
        return sha256_error!();
    }

    pst_ctx.n[1] = ui_cnt_sec;
    pst_ctx.n[0] = ui_cnt_first;
    return sha256_ok!();
}

pub fn vos_sha256_last_padding(puc_data: &mut [u8], ui_len: u32, pst_ctx: &mut Box<VosSha256Ctx>, pui_padding_len: &mut u32) -> u32 {
    let mut err: i32;
    let ui_blc_len = pst_ctx.block_len;
    let mut puc_block: &mut [u8] = unsafe {
        std::slice::from_raw_parts_mut(pst_ctx.block.as_ptr() as *mut u8, pst_ctx.block.len() * std::mem::size_of::<u32>() / std::mem::size_of::<u8>())
    };

    if ui_len >= sha256_block_size!() as u32 || ui_len + ui_blc_len as u32 >= sha256_block_size!() as u32 {
        unsafe {
            std::ptr::copy_nonoverlapping(
                puc_data.as_mut_ptr(),
                puc_block[ui_blc_len as usize..].as_ptr() as *mut u8,
                (sha256_block_size!() - ui_blc_len) as usize
            );
        }
        vos_sha256_compress_mul(pst_ctx, puc_block, 1);
        *pui_padding_len = (sha256_block_size!() - ui_blc_len) as u32;
        pst_ctx.block_len = 0;
        puc_block.fill(0);
    } else {
        unsafe {
            std::ptr::copy_nonoverlapping(
                puc_data.as_mut_ptr(),
                puc_block[ui_blc_len as usize..].as_ptr() as *mut u8,
                ui_len as usize
            );
        }
        pst_ctx.block_len += ui_len;
        return sha256_error!();
    }

    return sha256_ok!();
}

pub fn vos_sha256_hash_by_blc_multi(puc_data: &mut [u8], ui_len: u32, pst_ctx: &mut Box<VosSha256Ctx>) {
    let mut ui_blc_len: u32;
    let mut ui_len_tmp = ui_len;
    let mut puc_src = puc_data;

    ui_blc_len = ui_len_tmp / sha256_block_size!() as u32;
    if ui_blc_len > 0 {
        vos_sha256_compress_mul(pst_ctx, puc_src, ui_blc_len);
        ui_blc_len *= sha256_block_size!() as u32;
        puc_src = &mut puc_src[ui_blc_len as usize..];
        ui_len_tmp -= ui_blc_len;
    }

    if ui_len_tmp != 0 {
        pst_ctx.block_len = ui_len_tmp;
        unsafe {
            std::ptr::copy_nonoverlapping(
                puc_src.as_ptr(),
                pst_ctx.block.as_mut_ptr() as *mut u8,
                ui_len_tmp as usize
            );
        }
    }
    return;
}

pub fn vos_sha256_hash(puc_data: &mut [u8], ui_len: u32, pst_ctx: &mut Box<VosSha256Ctx>) {
    let mut ui_blc_len: u32 = 0;
    let mut ui_len_tmp = ui_len;
    let mut puc_src = puc_data;

    if puc_src.is_empty() || ui_len_tmp == 0 || pst_ctx.corrupted == 1 || pst_ctx.computed == 1 || vos_sha256_ctx_prepare(pst_ctx, ui_len_tmp) != sha256_ok!() {
        return;
    }

    if pst_ctx.block_len != 0 {
        if vos_sha256_last_padding(puc_src, ui_len_tmp, pst_ctx, &mut ui_blc_len) == sha256_ok!() {
            puc_src = &mut puc_src[ui_blc_len as usize..];
            ui_len_tmp -= ui_blc_len;
        } else {
            return;
        }
    }

    vos_sha256_hash_by_blc_multi(puc_src, ui_len_tmp, pst_ctx);
    return;
}

pub fn vos_sha256_end(puc_out: &mut [u8], ui_out_size: u32, pst_ctx: &mut Box<VosSha256Ctx>) {
    let mut ui_index: u32;
    let mut puc_block: &mut [u8] = unsafe {
        std::slice::from_raw_parts_mut(pst_ctx.block.as_ptr() as *mut u8, pst_ctx.block.len() * std::mem::size_of::<u32>() / std::mem::size_of::<u8>())
    };
    let mut ui_blc_len: u32 = pst_ctx.block_len;

    if pst_ctx.corrupted == 1 || ui_out_size < pst_ctx.out_len {
        *pst_ctx = Box::new(VosSha256Ctx::new());
        return;
    }

    if pst_ctx.computed == 0 {
        puc_block[ui_blc_len as usize] = 0x80;
        ui_blc_len += 1;
        if ui_blc_len > (sha256_block_size!() - 8) {
            puc_block[ui_blc_len as usize..sha256_block_size!()].fill(0);
            ui_blc_len = 0;
            vos_sha256_compress_mul(pst_ctx, puc_block, 1);
        }

        puc_block[ui_blc_len as usize..(sha256_block_size!() - 8) as usize].fill(0);
        puc_block = &mut puc_block[(sha256_block_size!() - 8) as usize..];
        put_uint32_be!(pst_ctx.n[1], puc_block, 0);
        puc_block = &mut puc_block[std::mem::size_of::<u32>()..];
        put_uint32_be!(pst_ctx.n[0], puc_block, 0);
        puc_block = &mut puc_block[std::mem::size_of::<u32>()..];
        puc_block = unsafe {
            std::slice::from_raw_parts_mut(puc_block.as_ptr().sub(sha256_block_size!()) as *mut u8, puc_block.len() + sha256_block_size!())
        };
        vos_sha256_compress_mul(pst_ctx, puc_block, 1);
        pst_ctx.block_len = 0;
        puc_block[..sha256_block_size!()].fill(0);
        pst_ctx.computed = 1;
    }

    ui_blc_len = if pst_ctx.out_len <= ui_out_size { pst_ctx.out_len } else { ui_out_size } / (std::mem::size_of::<u32>() as u32);
    if !puc_out.is_empty() {
        for ui_index in 0..ui_blc_len {
            put_uint32_be!(pst_ctx.h[ui_index as usize], puc_out, (std::mem::size_of::<u32>() as u32) * ui_index);
        }
    }

    return;
}

use std::sync::Mutex;

static K256: Mutex<[u32; 64]> = Mutex::new([
    0x428a2f98, 0x71374491, 0xb5c0fbcf, 0xe9b5dba5, 0x3956c25b, 0x59f111f1, 0x923f82a4, 0xab1c5ed5,
    0xd807aa98, 0x12835b01, 0x243185be, 0x550c7dc3, 0x72be5d74, 0x80deb1fe, 0x9bdc06a7, 0xc19bf174,
    0xe49b69c1, 0xefbe4786, 0x0fc19dc6, 0x240ca1cc, 0x2de92c6f, 0x4a7484aa, 0x5cb0a9dc, 0x76f988da,
    0x983e5152, 0xa831c66d, 0xb00327c8, 0xbf597fc7, 0xc6e00bf3, 0xd5a79147, 0x06ca6351, 0x14292967,
    0x27b70a85, 0x2e1b2138, 0x4d2c6dfc, 0x53380d13, 0x650a7354, 0x766a0abb, 0x81c2c92e, 0x92722c85,
    0xa2bfe8a1, 0xa81a664b, 0xc24b8b70, 0xc76c51a3, 0xd192e819, 0xd6990624, 0xf40e3585, 0x106aa070,
    0x19a4c116, 0x1e376c08, 0x2748774c, 0x34b0bcb5, 0x391c0cb3, 0x4ed8aa4a, 0x5b9cca4f, 0x682e6ff3,
    0x748f82ee, 0x78a5636f, 0x84c87814, 0x8cc70208, 0x90befffa, 0xa4506ceb, 0xbef9a3f7, 0xc67178f2,
]);

macro_rules! vos_rotr32 {
    ($x:expr, $ui_blc_len:expr) => {
        (($x << (32 - $ui_blc_len)) | ($x >> $ui_blc_len))
    }
}
pub(crate) use vos_rotr32;

macro_rules! vos_round {
    ($a:expr, $b:expr, $c:expr, $d:expr, $e:expr, $f:expr, $g:expr, $h:expr, $i:expr, $k:expr, $w:expr) => {
        {
            $h += (vos_rotr32!($e, 6) ^ vos_rotr32!($e, 11) ^ vos_rotr32!($e, 25)) + ($g ^ ($e & ($f ^ $g))) + $k + $w[$i as usize];
            $d += $h;
            $h += (vos_rotr32!($a, 2) ^ vos_rotr32!($a, 13) ^ vos_rotr32!($a, 22)) + (($a & ($b | $c)) | ($b & $c));
        }
    }
}
pub(crate) use vos_round;

pub fn vos_sha256_compress_block(state: &mut [u32], block: &mut [u8]) {
    let mut W = [0u32; 64];
    let mut i: u32;
    let mut j: u32;
    let mut a: u32;
    let mut b: u32;
    let mut c: u32;
    let mut d: u32;
    let mut e: u32;
    let mut f: u32;
    let mut g: u32;
    let mut h: u32;

    for i in 0..16 {
        W[i] = get_uint32_be!(block, 4 * i);
    }

    for i in 16..64 {
        W[i] = W[i - 16] + W[i - 7] + 
            (vos_rotr32!(W[i - 15], 7) ^ vos_rotr32!(W[i - 15], 18) ^ (W[i - 15] >> 3)) +
            (vos_rotr32!(W[i - 2], 17) ^ vos_rotr32!(W[i - 2], 19) ^ (W[i - 2] >> 10));
    }

    j = 0;
    a = state[j as usize]; j += 1;
    b = state[j as usize]; j += 1;
    c = state[j as usize]; j += 1;
    d = state[j as usize]; j += 1;
    e = state[j as usize]; j += 1;
    f = state[j as usize]; j += 1;
    g = state[j as usize]; j += 1;
    h = state[j as usize];

    for i in (0..64).step_by(8) {
        j = 0;
        vos_round!(a, b, c, d, e, f, g, h, i + j, K256.lock().unwrap()[i as usize + 0], W); j += 1;
        vos_round!(h, a, b, c, d, e, f, g, i + j, K256.lock().unwrap()[i as usize + 1], W); j += 1;
        vos_round!(g, h, a, b, c, d, e, f, i + j, K256.lock().unwrap()[i as usize + 2], W); j += 1;
        vos_round!(f, g, h, a, b, c, d, e, i + j, K256.lock().unwrap()[i as usize + 3], W); j += 1;
        vos_round!(e, f, g, h, a, b, c, d, i + j, K256.lock().unwrap()[i as usize + 4], W); j += 1;
        vos_round!(d, e, f, g, h, a, b, c, i + j, K256.lock().unwrap()[i as usize + 5], W); j += 1;
        vos_round!(c, d, e, f, g, h, a, b, i + j, K256.lock().unwrap()[i as usize + 6], W); j += 1;
        vos_round!(b, c, d, e, f, g, h, a, i + j, K256.lock().unwrap()[i as usize + 7], W);
    }

    j = 0;
    
    state[j as usize] += a; j += 1;
    state[j as usize] += b; j += 1;
    state[j as usize] += c; j += 1;
    state[j as usize] += d; j += 1;
    state[j as usize] += e; j += 1;
    state[j as usize] += f; j += 1;
    state[j as usize] += g; j += 1;
    state[j as usize] += h;
}

pub fn vos_sha256_compress_mul(pst_ctx: &mut Box<VosSha256Ctx>, puc_input: &mut [u8], ui_num: u32) {
    let mut ui_num_tmp = ui_num;
    let mut puc_block = puc_input;

    while ui_num_tmp != 0 {
        vos_sha256_compress_block(&mut pst_ctx.h, puc_block);
        puc_block = &mut puc_block[sha256_block_size!() as usize..];
        ui_num_tmp -= 1;
    }
}

pub fn vos_sha256_calc(puc_input: &mut [u8], ui_input_len: u32, puc_output: &mut [u8], ui_output_len: u32) {
    let mut st_ctx = Box::new(VosSha256Ctx::new());
    vos_sha256_begin(&mut st_ctx);
    vos_sha256_hash(puc_input, ui_input_len, &mut st_ctx);
    vos_sha256_end(puc_output, ui_output_len, &mut st_ctx);
}
