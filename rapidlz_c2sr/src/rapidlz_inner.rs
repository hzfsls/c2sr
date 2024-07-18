macro_rules! max_byte_value { () => { 255 }; }
pub(crate) use max_byte_value;

macro_rules! max_4bit_value { () => { 15 }; }
pub(crate) use max_4bit_value;

macro_rules! min_match { () => { 4 }; }
pub(crate) use min_match;

macro_rules! rapidlz_input_invalid { () => { -100isize as usize }; }
pub(crate) use rapidlz_input_invalid;

macro_rules! rapidlz_malloc_failed { () => { -99isize as usize }; }
pub(crate) use rapidlz_malloc_failed;

macro_rules! rapidlz_dst_size_small { () => { -98isize as usize }; }
pub(crate) use rapidlz_dst_size_small;

macro_rules! rapidlz_securec_error { () => { -97isize as usize }; }
pub(crate) use rapidlz_securec_error;

macro_rules! rapidlz_format_invalid { () => { -96isize as usize }; }
pub(crate) use rapidlz_format_invalid;

macro_rules! likely { ($x:expr) => { $x } }
pub(crate) use likely;

macro_rules! unlikely { ($x:expr) => { $x } }
pub(crate) use unlikely;

pub(crate) struct UnalignU16 {
    pub v: u16,
}

pub(crate) struct UnalignU32 {
    pub v: u32,
}

pub(crate) struct UnalignU64 {
    pub v: u64,
}

macro_rules! read16bit {
    ($ptr:expr) => {
        {
            unsafe {
                (*(($ptr) as *const UnalignU16)).v as u16
            } 
        }
    };
}
pub(crate) use read16bit;

macro_rules! read32bit {
    ($ptr:expr) => {
        {
            unsafe {
                (*(($ptr) as *const UnalignU32)).v as u32
            }
        }
    };
}
pub(crate) use read32bit;


macro_rules! read64bit {
    ($ptr:expr) => {
        {
            unsafe {
                (*(($ptr) as *const UnalignU64)).v as u64
            }
        }
    };
}
pub(crate) use read64bit;

macro_rules! write64bit {
    ($ptr:expr, $val:expr) => {
        {
            unsafe {
                (*(($ptr) as *mut UnalignU64)).v = $val;
            }
        }
    };
}
pub(crate) use write64bit;

macro_rules! rapidlz_assert {
    ($x:expr) => {
        assert!($x)
    };
}
pub(crate) use rapidlz_assert;

pub(crate) fn is_le() -> bool {
    cfg!(target_endian = "little")
}

pub(crate) fn read_le16bit(addr: &mut [u8]) -> u16 {
    if is_le() {
        unsafe {
            *(addr.as_ptr() as *const u16)
        }
    } else {
        let tmp1 = addr[0];
        let tmp2 = addr[1];
        (tmp1 as u16) + ((tmp2 as u16) << 8)
    }
}

pub(crate) fn count_tail_zero64(x: u64) -> u8 {
    if x == 0 {
        return 0;
    }
    let mut val = x;
    let mut num = 0;
    while val & 1 == 0 {
        val >>= 1;
        num += 1;
    }
    num
}

pub(crate) fn count_lead_zero64(x: u64) -> u8 {
    if x == 0 {
        return 0;
    }
    let mut val = x;
    let mut num = 0;
    while val & 0x8000000000000000 == 0 {
        val <<= 1;
        num += 1;
    }
    num
}

pub(crate) fn high_bit64(x: u64) -> u8 {
    rapidlz_assert!(x != 0);

    let mut pos = 64;
    let mut val = x;

    if val == 0 {
        return 0;
    }
    if (val & 0xFFFFFFFF00000000) == 0 {
        val <<= 32;
        pos -= 32;
    }
    if (val & 0xFFFF000000000000) == 0 {
        val <<= 16;
        pos -= 16;
    }
    if (val & 0xFF00000000000000) == 0 {
        val <<= 8;
        pos -= 8;
    }
    if (val & 0xF000000000000000) == 0 {
        val <<= 4;
        pos -= 4;
    }
    if (val & 0xC000000000000000) == 0 {
        val <<= 2;
        pos -= 2;
    }
    if (val & 0x8000000000000000) == 0 {
        val <<= 1;
        pos -= 1;
    }

    pos - 1
}

pub(crate) fn write_le16(addr: &mut [u8], val: u16) {
    if is_le() {
        unsafe {
            *(addr.as_mut_ptr() as *mut u16) = val;
        }
    } else {
        let tmp = addr;
        tmp[0] = val as u8;
        tmp[1] = (val >> 8) as u8;
    }
}

pub(crate) fn copy_32byte(dst: &mut [u8], src: &mut [u8]) {
    write64bit!(dst.as_mut_ptr(), read64bit!(src.as_mut_ptr()));
    write64bit!(dst[8..].as_mut_ptr(), read64bit!(src[8..].as_mut_ptr()));
    write64bit!(dst[16..].as_mut_ptr(), read64bit!(src[16..].as_mut_ptr()));
    write64bit!(dst[24..].as_mut_ptr(), read64bit!(src[24..].as_mut_ptr()));
}

pub(crate) fn copy_16byte(dst: &mut [u8], src: &mut [u8]) {
    write64bit!(dst.as_mut_ptr(), read64bit!(src.as_mut_ptr()));
    write64bit!(dst[8..].as_mut_ptr(), read64bit!(src[8..].as_mut_ptr()));
}

pub(crate) fn copy_8byte(dst: &mut [u8], src: &mut [u8]) {
    write64bit!(dst.as_mut_ptr(), read64bit!(src.as_mut_ptr()));
}

pub(crate) fn rapidlz_wild_copy8(src_ptr: &mut [u8], dst_ptr: &mut [u8], dst_end: *mut u8) {
    let mut tmp_dst_ptr = dst_ptr;
    let mut tmp_src_ptr = src_ptr;
    loop {
        copy_8byte(tmp_dst_ptr, tmp_src_ptr);
        tmp_dst_ptr = &mut tmp_dst_ptr[8..];
        tmp_src_ptr = &mut tmp_src_ptr[8..];
        if (tmp_dst_ptr.as_ptr() as usize) >= (dst_end as usize) {
            break;
        }
    }
}

pub(crate) fn rapidlz_wild_copy16(src_ptr: &mut [u8], dst_ptr: &mut [u8], dst_end: *mut u8) {
    let mut tmp_dst_ptr = dst_ptr;
    let mut tmp_src_ptr = src_ptr;
    loop {
        copy_16byte(tmp_dst_ptr, tmp_src_ptr);
        tmp_dst_ptr = &mut tmp_dst_ptr[16..];
        tmp_src_ptr = &mut tmp_src_ptr[16..];
        if (tmp_dst_ptr.as_ptr() as usize) >= (dst_end as usize) {
            break;
        }
    }
}

pub(crate) fn rapidlz_wild_copy32(src_ptr: &mut [u8], dst_ptr: &mut [u8], dst_end: *mut u8) {
    let mut tmp_dst_ptr = dst_ptr;
    let mut tmp_src_ptr = src_ptr;
    loop {
        copy_32byte(tmp_dst_ptr, tmp_src_ptr);
        tmp_dst_ptr = &mut tmp_dst_ptr[32..];
        tmp_src_ptr = &mut tmp_src_ptr[32..];
        if (tmp_dst_ptr.as_ptr() as usize) >= (dst_end as usize) {
            break;
        }
    }
}

macro_rules! wrap_ptr { 
    ($ptr:expr) => { unsafe { core::slice::from_raw_parts_mut($ptr, 0) } }; 
    ($begin:expr, $end:expr) => { unsafe { core::slice::from_raw_parts_mut($begin, $end as usize - $begin as usize) } };
}
pub(crate) use wrap_ptr;