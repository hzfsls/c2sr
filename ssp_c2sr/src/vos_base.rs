use crate::vos_typedef::*;

pub mod vos_barrier;

macro_rules! vos_millsecond_per_second { () => { 1000 }; }
macro_rules! vos_microsecond_per_millsecond { () => { 1000 }; }
macro_rules! vos_nanosecond_per_second { () => { 1000000000 }; }
macro_rules! vos_nanosecond_per_millsecond { () => { 1000000 }; }
macro_rules! vos_nanosecond_per_microsecond { () => { 1000 }; }

macro_rules! vos_base_likely { ($x:expr) => { $x }; }
macro_rules! vos_base_unlikely { ($x:expr) => { $x }; }

macro_rules! vos_ntohl { 
    ($x:expr) => {
        (($x & 0x000000ffu) << 24) | 
        (($x & 0x0000ff00u) << 8) | 
        (($x & 0x00ff0000u) >> 8) | 
        (($x & 0xff000000u) >> 24)
    };
}

macro_rules! vos_htonl { 
    ($x:expr) => {
        (($x & 0x000000ffu) << 24) | 
        (($x & 0x0000ff00u) << 8) | 
        (($x & 0x00ff0000u) >> 8) | 
        (($x & 0xff000000u) >> 24)
    };
}

macro_rules! vos_ntohs { 
    ($x:expr) => {
        (($x & 0x00ffu) << 8) | 
        (($x & 0xff00u) >> 8)
    };
}

macro_rules! vos_htons { 
    ($x:expr) => {
        (($x & 0x00ffu) << 8) | 
        (($x & 0xff00u) >> 8)
    };
}

macro_rules! vos_ntohll { 
    ($x:expr) => {
        vos_ntohl!($x as u32) as u64 << 32 | vos_ntohl!($x as u32 >> 32) as u64
    };
}

macro_rules! vos_htonll { 
    ($x:expr) => {
        vos_ntohl!($x as u32) as u64 << 32 | vos_ntohl!($x as u32 >> 32) as u64
    };
}

pub struct VosSystmS {
    pub us_year: VosUint16,
    pub uc_month: VosUint8,
    pub uc_date: VosUint8,
    pub uc_hour: VosUint8,
    pub uc_minute: VosUint8,
    pub uc_second: VosUint8,
    pub uc_week: VosUint8,
    pub ui_mill_sec: VosUint32,
}

pub struct VosPosixTimeS {
    pub siSecond: VosInt32,
    pub siMinute: VosInt32,
    pub siHour: VosInt32,
    pub siDay: VosInt32,
    pub siMonth: VosInt32,
    pub siYear: VosInt32,
    pub siWeekday: VosInt32,
    pub siYearDay: VosInt32,
    pub siIsDST: VosInt32,
}

macro_rules! vos_systime_zero {
    ($st_time:expr) => {
        $st_time.us_year = 0;
        $st_time.uc_month = 0;
        $st_time.uc_date = 0;
        $st_time.uc_hour = 0;
        $st_time.uc_minute = 0;
        $st_time.uc_second = 0;
        $st_time.uc_week = 0;
        $st_time.ui_mill_sec = 0;
    };
}

pub struct CpuTick {
    pub ulLow: VosUint32,
    pub ulHigh: VosUint32,
}

pub struct VosCpuTickS {
    pub uiLow: VosUint32,
    pub uiHigh: VosUint32,
}

pub struct VosCpuTickProtectS {
    pub uiCpuTickMagic: VosUint32,
    pub stCpuTick: VosCpuTickS,
}

pub struct VosCpuTickBaseProtectS {
    pub uiCpuTickBaseMagic: VosUint32,
    pub uiCpuTickBase: VosUint32,
}

macro_rules! vos_text_region_add { () => { 0 }; }
macro_rules! vos_text_region_del { () => { 0x1 }; }
type VosTextRegionOptionE = u8;

macro_rules! vos_cpu_affinity_comp { () => { 0 }; }
macro_rules! vos_cpu_affinity_task { () => { 0x1 }; }
macro_rules! vos_cpu_affinity_clock_task { () => { 0x2 }; }
macro_rules! vos_cpu_affinity_hrltime { () => { 0x3 }; }
macro_rules! vos_cpu_affinity_cputick { () => { 0x4 }; }
macro_rules! vos_cpu_affinity_tick_task { () => { 0x5 }; }
macro_rules! vos_cpu_affinity_butt { () => { 0x6 }; }

type VosCpuAffinityOptionE = u8;

macro_rules! vos_sched_none { () => { 0 }; }
macro_rules! vos_sched_nopreempt { () => { 1 }; }
macro_rules! vos_sched_preempt { () => { 2 }; }
macro_rules! vos_sched_mutex_lock { () => { 3 }; }

macro_rules! VOS_PRIdPTR { () => { "ld" }; }
macro_rules! VOS_PRIiPTR { () => { "li" }; }
macro_rules! VOS_PRIuPTR { () => { "lu" }; }
macro_rules! VOS_PRIxPTR { () => { "lx" }; }
macro_rules! VOS_PRIXPTR { () => { "lX" }; }
macro_rules! VOS_PRId64 { () => { "ld" }; }
macro_rules! VOS_PRIi64 { () => { "li" }; }
macro_rules! VOS_PRIo64 { () => { "lo" }; }
macro_rules! VOS_PRIu64 { () => { "lu" }; }
macro_rules! VOS_PRIx64 { () => { "lx" }; }
macro_rules! VOS_PRIX64 { () => { "lX" }; }

macro_rules! max_cpu_setsize { () => { 256 }; }
macro_rules! max_cpu_setbits { () => { 8 * std::mem::size_of::<VosUintptr>() }; }

pub struct VosCpuSetS {
    ulCpuMask: [VosUintptr; max_cpu_setsize!() / max_cpu_setbits!()],
}

macro_rules! cpu_mask_set {
    ($cpu:expr, $cpusetp:expr) => {
        $cpusetp.ulCpuMask[$cpu / max_cpu_setbits!()] |= (1 as VosUintptr) << ($cpu % max_cpu_setbits!());
    };
}

macro_rules! cpu_mask_clr {
    ($cpu:expr, $cpusetp:expr) => {
        $cpusetp.ulCpuMask[$cpu / max_cpu_setbits!()] &= !((1 as VosUintptr) << ($cpu % max_cpu_setbits!()));
    };
}

macro_rules! cpu_coreid_is_in_mask {
    ($cpu:expr, $cpusetp:expr) => {
        $cpusetp.ulCpuMask[$cpu / max_cpu_setbits!()] & ((1 as VosUintptr) << ($cpu % max_cpu_setbits!()));
    };
}

macro_rules! cpu_mask_zero {
    ($cpusetp:expr) => {
        for i_ in 0..(max_cpu_setsize!() / max_cpu_setbits!()) {
            $cpusetp.ulCpuMask[i_] = 0;
        }
    };
}

pub type VosCpuAffinityRechooseFunc = Option<fn(VosCpuAffinityOptionE, &str, &mut Box<VosCpuSetS>, &mut Box<VosCpuSetS>) -> VosBool>;

macro_rules! vos_array_size { ($arr:expr) => { arr.len() }; }
macro_rules! vos_is_power_of_two { ($uiValue:expr) => { ($uiValue != 0) && (($uiValue & ($uiValue - 1)) == 0) }; }
