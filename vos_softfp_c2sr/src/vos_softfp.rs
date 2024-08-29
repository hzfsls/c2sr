macro_rules! vos_ok { () => { 0 }; }

macro_rules! vos_errno_fail { () => { 0xFFFFFFFF }; }

macro_rules! vos_errno_inval { () => { 22 }; }

macro_rules! vos_fp_value_eq { () => { 0 }; }

macro_rules! vos_fp_value_gt { () => { 1 }; }

macro_rules! vos_fp_value_lt { () => { 2 }; }

macro_rules! vos_errno_softfp_nan_or_inf { () => { 3 }; }

macro_rules! vos_fp_max_float_str_len { () => { 64 }; }

macro_rules! vos_fp_max_double_str_len { () => { 320 }; }

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VosFpFloatS {
    pub f_value: f32
}

impl VosFpFloatS {
    pub fn new(f_value: f32) -> Self {
        VosFpFloatS {
            f_value
        }
    }

    pub fn get_st_bit_ui_sign(&self) -> u32 {
        let mut st_bit = self.f_value.to_bits() & 0x80000000;
        st_bit >>= 31;
        st_bit
    }

    pub fn set_st_bit_ui_sign(&mut self, ui_sign: u32) {
        let mut f_value_bits = self.f_value.to_bits();
        f_value_bits &= 0x7FFFFFFF;
        f_value_bits |= ui_sign << 31;
        self.f_value = f32::from_bits(f_value_bits);
    }

    pub fn get_st_bit_ui_exp(&self) -> u32 {
        let mut st_bit = self.f_value.to_bits() & 0x7F800000;
        st_bit >>= 23;
        st_bit
    }

    pub fn set_st_bit_ui_exp(&mut self, ui_exp: u32) {
        let mut f_value_bits = self.f_value.to_bits();
        f_value_bits &= 0xFFFFFE01;
        f_value_bits |= ui_exp << 1;
        self.f_value = f32::from_bits(f_value_bits);
    }

    pub fn get_st_bit_ui_frac(&self) -> u32 {
        let mut st_bit = self.f_value.to_bits() & 0x007FFFFF;
        st_bit
    }

    pub fn set_st_bit_ui_frac(&mut self, ui_frac: u32) {
        let mut f_value_bits = self.f_value.to_bits();
        f_value_bits &= 0xFF800000;
        f_value_bits |= ui_frac;
        self.f_value = f32::from_bits(f_value_bits);
    }
}

// ```c
// typedef union tagFpDouble {
//     VOS_DOUBLE dValue;
//     struct {
// #if VOS_BYTE_ORDER == VOS_BIG_ENDIAN
//         VOS_UINT32 uiSign : 1;
//         VOS_UINT32 uiExp : 11;
//         VOS_UINT32 uiFrac1 : 20;
//         VOS_UINT32 uiFrac0 : 32;
// #else
//         VOS_UINT32 uiFrac0 : 32;
//         VOS_UINT32 uiFrac1 : 20;
//         VOS_UINT32 uiExp : 11;
//         VOS_UINT32 uiSign : 1;
// #endif
//     } stBit;
// } VOS_FP_DOUBLE_S;
// ```
// // Translate from C to Rust:
// ```rust

pub struct VosFpDoubleS {
    pub d_value: f64
}

impl VosFpDoubleS {
    pub fn new(d_value: f64) -> Self {
        VosFpDoubleS {
            d_value
        }
    }

    pub fn get_st_bit_ui_sign(&self) -> u32 {
        let mut st_bit = self.d_value.to_bits() & 0x8000000000000000;
        st_bit >>= 63;
        st_bit as u32
    }

    pub fn set_st_bit_ui_sign(&mut self, ui_sign: u32) {
        let mut d_value_bits = self.d_value.to_bits();
        d_value_bits &= 0x7FFFFFFFFFFFFFFF;
        d_value_bits |= (ui_sign as u64) << 63;
        self.d_value = f64::from_bits(d_value_bits);
    }

    pub fn get_st_bit_ui_exp(&self) -> u32 {
        let mut st_bit = self.d_value.to_bits() & 0x7FF0000000000000;
        st_bit >>= 52;
        st_bit as u32
    }

    pub fn set_st_bit_ui_exp(&mut self, ui_exp: u32) {
        let mut d_value_bits = self.d_value.to_bits();
        d_value_bits &= 0x800FFFFFFFFFFFFF;
        d_value_bits |= (ui_exp as u64) << 52;
        self.d_value = f64::from_bits(d_value_bits);
    }

    pub fn get_st_bit_ui_frac_0(&self) -> u32 {
        let mut st_bit = self.d_value.to_bits() & 0x00000000FFFFFFFF;
        st_bit as u32
    }

    pub fn set_st_bit_ui_frac_0(&mut self, ui_frac_0: u32) {
        let mut d_value_bits = self.d_value.to_bits();
        d_value_bits &= 0xFFFFFFFF00000000;
        d_value_bits |= ui_frac_0 as u64;
        self.d_value = f64::from_bits(d_value_bits);
    }

    pub fn get_st_bit_ui_frac_1(&self) -> u32 {
        let mut st_bit = self.d_value.to_bits() & 0x000FFFFF00000000;
        st_bit >>= 32;
        st_bit as u32
    }

    pub fn set_st_bit_ui_frac_1(&mut self, ui_frac_1: u32) {
        let mut d_value_bits = self.d_value.to_bits();
        d_value_bits &= 0xFFF00000FFFFFFFF;
        d_value_bits |= (ui_frac_1 as u64) << 32;
        self.d_value = f64::from_bits(d_value_bits);
    }
}

pub fn vos_soft_fp_invalid_check(pst_v1: Box<VosFpFloatS>) -> bool {
    if pst_v1.f_value.is_nan() || pst_v1.f_value.is_infinite() {
        return true;
    }
    false
}

macro_rules! soft_fp_check_of_num_range { ($pcPoint: expr) => { 
    $pcPoint.peek().is_some() &&
    ('0' <= *$pcPoint.peek().unwrap() && '9' >= *$pcPoint.peek().unwrap()) }; }

macro_rules! soft_fp_check_sign_sub_add { ($pcPoint: expr) => { 
    $pcPoint.peek().is_some() &&
    ('-' == *$pcPoint.peek().unwrap() || '+' == *$pcPoint.peek().unwrap()) }; }

macro_rules! soft_fp_get_str_val { 
    ($value: expr, $pcPoint: expr) => { 
        while ($pcPoint.peek().is_some() && (*$pcPoint.peek().unwrap() >= '0') && (*$pcPoint.peek().unwrap() <= '9')){
            $value = $value * 10 + (*$pcPoint.peek().unwrap() as u32 - '0' as u32);
            $pcPoint.next();
        }
    }; 
}

macro_rules! soft_fp_null_and_return_invalid { ($fR: expr) => { if $fR.is_none() { return vos_errno_inval!(); } }; }

macro_rules! vos_inner_check_nan_or_inf { ($x: expr) => { $x == std::num::FpCategory::Nan || $x == std::num::FpCategory::Infinite }; }

pub fn vos_soft_fp_str_2_float_ieee754(ppsc_point: &mut String, pdl_value: &mut f64) -> u32 {
    let mut psc_point_tmp = ppsc_point.chars().peekable();
    let mut value = *pdl_value;
    let mut factor = 10.0;
    let mut ui_ret = vos_ok!();
    let mut ui_expo: u32 = 0;
    match {psc_point_tmp.next(); psc_point_tmp.peek()} {
        Some('-') => {
            factor = 0.1;
            psc_point_tmp.next();
        },
        Some('+') => {
            psc_point_tmp.next();
        },
        Some('0'..='9') => {},
        _ => {
            value = 0.0;
            ui_ret = vos_errno_fail!();
        }
    }

    if ui_ret == vos_ok!() {
        soft_fp_get_str_val!(ui_expo, psc_point_tmp);
        println!("ui_expo: {}", ui_expo);
        println!("factor: {}", factor);

        loop {
            if (ui_expo & 0x1) != 0 {
                value *= factor;
            }
            if {ui_expo >>= 0x1; ui_expo} == 0 {
                break;
            }

            factor *= factor;
        }
    }
    
    *pdl_value = value;
    *ppsc_point = psc_point_tmp.collect::<String>();
    ui_ret
}

pub fn vos_soft_fp_str_2_float(psc_souse: &String, ppsc_endptr: Option<&mut String>) -> f64 {
    let mut psc_point = psc_souse.chars().peekable();
    let mut value: f64 = 0.0;
    let mut si_sign: i32 = 0;
    let mut factor: f64;
    let mut ui_ret: u32;

    while *psc_point.peek().unwrap() == ' ' {
        psc_point.next();
    }

    if soft_fp_check_sign_sub_add!(psc_point) {
        si_sign = *psc_point.peek().unwrap() as i32;
        psc_point.next();
    }

    let mut i_value: u32 = 0;
    soft_fp_get_str_val!(i_value, psc_point);
    value = i_value as f64;

    if *psc_point.peek().unwrap() == '.' {
        factor = 1.0;
        psc_point.next();
        while soft_fp_check_of_num_range!(psc_point) {
            factor *= 0.1;
            value += ((*psc_point.peek().unwrap() as u32 - '0' as u32) as f64) * factor;
            psc_point.next();
        }
    }

    if psc_point.peek().unwrap().to_ascii_lowercase() == 'e' {
        let mut psc_str = psc_point.clone().collect::<String>();
        ui_ret = vos_soft_fp_str_2_float_ieee754(&mut psc_str, &mut value);
        if ui_ret != vos_ok!() {
            psc_point = psc_souse.chars().peekable();
        }
    }

    if ppsc_endptr.is_some() {
        *ppsc_endptr.unwrap() = psc_point.collect();
    }

    return if si_sign == '-' as i32 { -value } else { value };
}

pub fn vos_soft_fp_s_eva(psc_v1: &String, pst_reasult: &mut Box<VosFpFloatS>) -> u32 {
    pst_reasult.f_value = vos_soft_fp_str_2_float(psc_v1, None) as f32;
    vos_ok!()
}

pub fn vos_soft_fp_s_cmp(pst_v1: &mut Box<VosFpFloatS>, pst_v2: &mut Box<VosFpFloatS>) -> u32 {
    if pst_v1.f_value > pst_v2.f_value {
        return vos_fp_value_gt!();
    } else if pst_v1.f_value < pst_v2.f_value {
        return vos_fp_value_lt!();
    }
    vos_fp_value_eq!()
}

pub fn vos_soft_fp_s_add(pst_v1: &mut Box<VosFpFloatS>, pst_v2: &mut Box<VosFpFloatS>, pst_reasult: &mut Box<VosFpFloatS>) -> u32 {
    pst_reasult.f_value = pst_v1.f_value + pst_v2.f_value;
    let si_ret = pst_reasult.f_value.classify();
    if vos_inner_check_nan_or_inf!(si_ret) {
        return vos_errno_softfp_nan_or_inf!();
    }
    vos_ok!()
}

pub fn vos_soft_fp_s_sub(pst_v1: &mut Box<VosFpFloatS>, pst_v2: &mut Box<VosFpFloatS>, pst_reasult: &mut Box<VosFpFloatS>) -> u32 {
    pst_reasult.f_value = pst_v1.f_value - pst_v2.f_value;
    let si_ret = pst_reasult.f_value.classify();
    if vos_inner_check_nan_or_inf!(si_ret) {
        return vos_errno_softfp_nan_or_inf!();
    }
    vos_ok!()
}

pub fn vos_soft_fp_s_mul(pst_v1: &mut Box<VosFpFloatS>, pst_v2: &mut Box<VosFpFloatS>, pst_reasult: &mut Box<VosFpFloatS>) -> u32 {
    pst_reasult.f_value = pst_v1.f_value * pst_v2.f_value;
    let si_ret = pst_reasult.f_value.classify();
    if vos_inner_check_nan_or_inf!(si_ret) {
        return vos_errno_softfp_nan_or_inf!();
    }
    vos_ok!()
}

pub fn vos_soft_fp_s_div(pst_v1: &mut Box<VosFpFloatS>, pst_v2: &mut Box<VosFpFloatS>, pst_reasult: &mut Box<VosFpFloatS>) -> u32 {
    if pst_v2.f_value > -std::f32::EPSILON && pst_v2.f_value < std::f32::EPSILON {
        return vos_errno_inval!();
    }
    pst_reasult.f_value = pst_v1.f_value / pst_v2.f_value;
    let si_ret = pst_reasult.f_value.classify();
    if vos_inner_check_nan_or_inf!(si_ret) {
        return vos_errno_softfp_nan_or_inf!();
    }
    vos_ok!()
}

pub fn vos_soft_fp_s_log(pst_v1: &mut Box<VosFpFloatS>, pst_reasult: &mut Box<VosFpFloatS>) -> u32 {
    if pst_v1.f_value <= 0.0 {
        return vos_errno_inval!();
    }
    pst_reasult.f_value = pst_v1.f_value.ln();
    let si_ret = pst_reasult.f_value.classify();
    if vos_inner_check_nan_or_inf!(si_ret) {
        return vos_errno_softfp_nan_or_inf!();
    }
    vos_ok!()
}

pub fn vos_soft_fp_s_log10(pst_v1: &mut Box<VosFpFloatS>, pst_reasult: &mut Box<VosFpFloatS>) -> u32 {
    if pst_v1.f_value <= 0.0 {
        return vos_errno_inval!();
    }
    pst_reasult.f_value = pst_v1.f_value.log10();
    let si_ret = pst_reasult.f_value.classify();
    if vos_inner_check_nan_or_inf!(si_ret) {
        return vos_errno_softfp_nan_or_inf!();
    }
    vos_ok!()
}

pub fn vos_soft_fp_s_sqrtf(pst_v1: &mut Box<VosFpFloatS>, pst_reasult: &mut Box<VosFpFloatS>) -> u32 {
    if pst_v1.f_value < 0.0 {
        return vos_errno_inval!();
    }
    pst_reasult.f_value = pst_v1.f_value.sqrt();
    let si_ret = pst_reasult.f_value.classify();
    if vos_inner_check_nan_or_inf!(si_ret) {
        return vos_errno_softfp_nan_or_inf!();
    }
    vos_ok!()
}

pub fn vos_soft_fp_d_cmp(pst_v1: &mut Box<VosFpDoubleS>, pst_v2: &mut Box<VosFpDoubleS>) -> u32 {
    if pst_v1.d_value > pst_v2.d_value {
        return vos_fp_value_gt!();
    } else if pst_v1.d_value < pst_v2.d_value {
        return vos_fp_value_lt!();
    }
    vos_fp_value_eq!()
}

pub fn  vos_soft_fp_d_eva(psc_v1: &String, pst_reasult: &mut Box<VosFpDoubleS>) -> u32 {
    pst_reasult.d_value = vos_soft_fp_str_2_float(psc_v1, None) as f64;
    vos_ok!()
}

pub fn vos_soft_fp_d_add(pst_v1: &mut Box<VosFpDoubleS>, pst_v2: &mut Box<VosFpDoubleS>, pst_reasult: &mut Box<VosFpDoubleS>) -> u32 {
    pst_reasult.d_value = pst_v1.d_value + pst_v2.d_value;
    let si_ret = pst_reasult.d_value.classify();
    if vos_inner_check_nan_or_inf!(si_ret) {
        return vos_errno_softfp_nan_or_inf!();
    }
    vos_ok!()
}

pub fn vos_soft_fp_d_sub(pst_v1: &mut Box<VosFpDoubleS>, pst_v2: &mut Box<VosFpDoubleS>, pst_reasult: &mut Box<VosFpDoubleS>) -> u32 {
    pst_reasult.d_value = pst_v1.d_value - pst_v2.d_value;
    let si_ret = pst_reasult.d_value.classify();
    if vos_inner_check_nan_or_inf!(si_ret) {
        return vos_errno_softfp_nan_or_inf!();
    }
    vos_ok!()
}

pub fn vos_soft_fp_d_mul(pst_v1: &mut Box<VosFpDoubleS>, pst_v2: &mut Box<VosFpDoubleS>, pst_reasult: &mut Box<VosFpDoubleS>) -> u32 {
    pst_reasult.d_value = pst_v1.d_value * pst_v2.d_value;
    let si_ret = pst_reasult.d_value.classify();
    if vos_inner_check_nan_or_inf!(si_ret) {
        return vos_errno_softfp_nan_or_inf!();
    }
    vos_ok!()
}

pub fn vos_soft_fp_d_div(pst_v1: &mut Box<VosFpDoubleS>, pst_v2: &mut Box<VosFpDoubleS>, pst_reasult: &mut Box<VosFpDoubleS>) -> u32 {
    if pst_v2.d_value > -std::f64::EPSILON && pst_v2.d_value < std::f64::EPSILON {
        return vos_errno_inval!();
    }
    pst_reasult.d_value = pst_v1.d_value / pst_v2.d_value;
    let si_ret = pst_reasult.d_value.classify();
    if vos_inner_check_nan_or_inf!(si_ret) {
        return vos_errno_softfp_nan_or_inf!();
    }
    vos_ok!()
}

pub fn vos_soft_fp_d_log(pst_v1: &mut Box<VosFpDoubleS>, pst_reasult: &mut Box<VosFpDoubleS>) -> u32 {
    if pst_v1.d_value <= 0.0 {
        return vos_errno_inval!();
    }
    pst_reasult.d_value = pst_v1.d_value.ln();
    let si_ret = pst_reasult.d_value.classify();
    if vos_inner_check_nan_or_inf!(si_ret) {
        return vos_errno_softfp_nan_or_inf!();
    }
    vos_ok!()
}

pub fn vos_soft_fp_d_log10(pst_v1: &mut Box<VosFpDoubleS>, pst_reasult: &mut Box<VosFpDoubleS>) -> u32 {
    if pst_v1.d_value <= 0.0 {
        return vos_errno_inval!();
    }
    pst_reasult.d_value = pst_v1.d_value.log10();
    let si_ret = pst_reasult.d_value.classify();
    if vos_inner_check_nan_or_inf!(si_ret) {
        return vos_errno_softfp_nan_or_inf!();
    }
    vos_ok!()
}

pub fn vos_soft_fp_d_sqrt(pst_v1: &mut Box<VosFpDoubleS>, pst_reasult: &mut Box<VosFpDoubleS>) -> u32 {
    if pst_v1.d_value < 0.0 {
        return vos_errno_inval!();
    }
    pst_reasult.d_value = pst_v1.d_value.sqrt();
    let si_ret = pst_reasult.d_value.classify();
    if vos_inner_check_nan_or_inf!(si_ret) {
        return vos_errno_softfp_nan_or_inf!();
    }
    vos_ok!()
}

pub fn vos_soft_fp_double_2_ushort(pst_v1: &mut Box<VosFpDoubleS>, pus_v2: &mut u16) -> u32 {
    *pus_v2 = pst_v1.d_value as u16;
    vos_ok!()
}

pub fn vos_soft_fp_double_2_int32(pst_v1: &mut Box<VosFpDoubleS>, psi_v2: &mut i32) -> u32 {
    *psi_v2 = pst_v1.d_value as i32;
    vos_ok!()
}

pub fn vos_soft_fp_int32_2_double(psi_v1: &mut i32, pst_v2: &mut Box<VosFpDoubleS>) -> u32 {
    pst_v2.d_value = *psi_v1 as f64;
    vos_ok!()
}

pub fn vos_soft_fp_uint32_2_double(pui_v1: &mut u32, pst_v2: &mut Box<VosFpDoubleS>) -> u32 {
    pst_v2.d_value = *pui_v1 as f64;
    vos_ok!()
}

pub fn vos_soft_fp_double_2_uint32(pst_v1: &mut Box<VosFpDoubleS>, pui_v2: &mut u32) -> u32 {
    *pui_v2 = pst_v1.d_value as u32;
    vos_ok!()
}

pub fn vos_soft_fp_uint32_2_float(pui_v1: &mut u32, pst_v2: &mut Box<VosFpFloatS>) -> u32 {
    pst_v2.f_value = *pui_v1 as f32;
    vos_ok!()
}

pub fn vos_soft_fp_float_2_uint32(pst_v1: &mut Box<VosFpFloatS>, pui_v2: &mut u32) -> u32 {
    *pui_v2 = pst_v1.f_value as u32;
    vos_ok!()
}

pub fn vos_soft_fp_int32_2_float(psi_v1: &mut i32, pst_v2: &mut Box<VosFpFloatS>) -> u32 {
    pst_v2.f_value = *psi_v1 as f32;
    vos_ok!()
}

pub fn vos_soft_fp_float_2_int32(pst_v1: &mut Box<VosFpFloatS>, psi_v2: &mut i32) -> u32 {
    *psi_v2 = pst_v1.f_value as i32;
    vos_ok!()
}

pub fn vos_soft_fp_ushort_2_double(pus_v1: &mut u16, pst_v2: &mut Box<VosFpDoubleS>) -> u32 {
    pst_v2.d_value = *pus_v1 as f64;
    vos_ok!()
}

pub fn vos_soft_fp_short_2_double(pss_v1: &mut i16, pst_v2: &mut Box<VosFpDoubleS>) -> u32 {
    pst_v2.d_value = *pss_v1 as f64;
    vos_ok!()
}

pub fn vos_soft_fp_double_2_short(pst_v1: &mut Box<VosFpDoubleS>, pss_v2: &mut i16) -> u32 {
    *pss_v2 = pst_v1.d_value as i16;
    vos_ok!()
}

pub fn vos_soft_fp_double_2_ascii(pst_v1: &mut Box<VosFpDoubleS>, psc_ascii: &mut String) -> u32 {
    *psc_ascii = pst_v1.d_value.to_string();
    vos_ok!()
}

pub fn vos_soft_fp_d_ceil(pst_v1: &mut Box<VosFpDoubleS>, pst_v2: &mut Box<VosFpDoubleS>) -> u32 {
    pst_v2.d_value = pst_v1.d_value.ceil();
    let si_ret = pst_v2.d_value.classify();
    if vos_inner_check_nan_or_inf!(si_ret) {
        return vos_errno_softfp_nan_or_inf!();
    }
    vos_ok!()
}

pub fn vos_soft_fp_d_exp(pst_v1: &mut Box<VosFpDoubleS>, pst_v2: &mut Box<VosFpDoubleS>) -> u32 {
    pst_v2.d_value = pst_v1.d_value.exp();
    let si_ret = pst_v2.d_value.classify();
    if vos_inner_check_nan_or_inf!(si_ret) {
        return vos_errno_softfp_nan_or_inf!();
    }
    vos_ok!()
}

pub fn vos_soft_fp_buffer_2_float(psc_v1: &mut [u8], pst_v2: &mut Box<VosFpFloatS>) -> u32 {
    pst_v2.f_value = unsafe { 
        std::mem::transmute::<[u8; 4], f32>(psc_v1[0..4].try_into().unwrap())
    };
    vos_ok!()
}

pub fn vos_soft_fp_buffer_2_double(psc_v1: &mut [u8], pst_v2: &mut Box<VosFpDoubleS>) -> u32 {
    pst_v2.d_value = unsafe { 
        std::mem::transmute::<[u8; 8], f64>(psc_v1[0..8].try_into().unwrap())
    };
    vos_ok!()
}

pub fn vos_soft_fp_float_2_buffer(pst_v1: &mut Box<VosFpFloatS>, psc_v2: &mut [u8]) -> u32 {
    psc_v2[0..4].copy_from_slice(&unsafe { std::mem::transmute::<f32, [u8; 4]>(pst_v1.f_value) });
    vos_ok!()
}

pub fn vos_soft_fp_double_2_buffer(pst_v1: &mut Box<VosFpDoubleS>, psc_v2: &mut [u8]) -> u32 {
    psc_v2[0..8].copy_from_slice(&unsafe { std::mem::transmute::<f64, [u8; 8]>(pst_v1.d_value) });
    vos_ok!()
}

pub fn vos_soft_fp_float_2_string(pst_v1: &mut Box<VosFpFloatS>, psc_v2: &mut String) -> u32 {
    *psc_v2 = format!("{}", pst_v1.f_value);
    vos_ok!()
}

pub fn vos_soft_fp_double_2_string(pst_v1: &mut Box<VosFpDoubleS>, psc_v2: &mut String) -> u32 {
    *psc_v2 = format!("{:.14}", pst_v1.d_value);
    vos_ok!()
}

pub fn vos_soft_fp_int_div(ui_v1: u32, ui_v2: u32, pst_v3: &mut Box<VosFpDoubleS>) -> u32 {
    if ui_v2 == 0 {
        return vos_errno_inval!();
    }

    pst_v3.d_value = ui_v1 as f64 / ui_v2 as f64;

    vos_ok!()
}

pub fn vos_soft_fp_add_pow_for_dopra(pst_v1: &mut Box<VosFpDoubleS>, pst_v2: &mut Box<VosFpDoubleS>, pst_result: &mut Box<VosFpDoubleS>) -> u32 {
    pst_result.d_value = pst_v1.d_value.powf(pst_v2.d_value);
    let si_ret = pst_result.d_value.classify();
    if vos_inner_check_nan_or_inf!(si_ret) {
        return vos_errno_softfp_nan_or_inf!();
    }
    vos_ok!()
}

pub fn vos_soft_fp_double_2_uint64(pst_v1: &mut Box<VosFpDoubleS>, pull_v2: &mut u64) -> u32 {
    *pull_v2 = pst_v1.d_value as u64;
    vos_ok!()
}

pub fn vos_soft_fp_uint64_2_double(pull_v1: &mut u64, pst_v2: &mut Box<VosFpDoubleS>) -> u32 {
    pst_v2.d_value = *pull_v1 as f64;
    vos_ok!()
}

pub fn vos_soft_fp_int64_2_double(psll_v1: &mut i64, pst_v2: &mut Box<VosFpDoubleS>) -> u32 {
    pst_v2.d_value = *psll_v1 as f64;
    vos_ok!()
}

pub fn vos_soft_fp_double_2_int64(pst_v1: &mut Box<VosFpDoubleS>, psll_v2: &mut i64) -> u32 {
    *psll_v2 = pst_v1.d_value as i64;
    vos_ok!()
}

pub fn vos_soft_fp_float_2_double(pst_v1: &mut Box<VosFpFloatS>, pst_v2: &mut Box<VosFpDoubleS>) -> u32 {
    pst_v2.d_value = pst_v1.f_value as f64;
    vos_ok!()
}








