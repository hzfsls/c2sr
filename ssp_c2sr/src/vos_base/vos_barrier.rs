use std::sync::atomic::Ordering;
use std::sync::atomic::fence;

macro_rules! vos_mem_barrier_r {
    () => {
        fence(Ordering::Acquire);
    };
}

pub(crate) use vos_mem_barrier_r;

macro_rules! vos_mem_barrier_w {
    () => {
        fence(Ordering::Release);
    };
}

pub(crate) use vos_mem_barrier_w;

macro_rules! vos_mem_barrier_rw {
    () => {
        fence(Ordering::AcqRel);
    };
}

pub(crate) use vos_mem_barrier_rw;

