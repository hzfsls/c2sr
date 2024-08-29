pub type VosOffT = u64;
pub type VosInt16 = i16;
pub type VosInt8 = i8;
pub type VosInt32 = i32;
pub type VosUchar = u8;
pub type VosWchar = i16;
pub type VosInt = i32;
pub type VosUint = u32;
pub type VosInt64 = i64;
pub type VosUint8 = u8;
pub type VosBool = u32;
pub type VosUint16 = u16;
pub type VosUint32 = u32;

pub type VosUint64 = u64;
pub type VosUintptr = usize;
pub type VosSizeT = usize;
pub type VosChar = i8;
pub type VosFloat = f32;
pub type VosDouble = f64;
pub type VosVoid = ();
pub type SizeT = usize;

macro_rules! vos_null_byte { () => { 0xFF }; }
macro_rules! vos_null_word { () => { 0xFFFF }; }
macro_rules! vos_null_dword { () => { 0xFFFFFFFF }; }
macro_rules! vos_null_long { () => { vos_null_dword!() }; }
macro_rules! vos_null_size_t { () => { !0 }; }
macro_rules! vos_false { () => { 0 }; }
macro_rules! vos_true { () => { 1 }; }
macro_rules! vos_null { () => { () }; }

macro_rules! vos_null_ptr { () => { None }; }

// pub type VosOptTypeE = u32;

// macro_rules! vos_opt_type_show { () => { 0 }; }
// macro_rules! vos_opt_type_get { () => { 1 }; }
// macro_rules! vos_opt_type_mem_dmg { () => { 2 }; }
// macro_rules! vos_opt_type_butt { () => { 3 }; }

// pub type VosOptAbortCtlFunc = Option<fn(enType: VosOptTypeE) -> VosUint32>;