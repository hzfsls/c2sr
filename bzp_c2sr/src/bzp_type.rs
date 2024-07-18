macro_rules! bzp_ok { () => { 0 }; }
pub(crate) use bzp_ok;

macro_rules! bzp_error_memory_open_failure { () => { 1 }; }
pub(crate) use bzp_error_memory_open_failure;

macro_rules! bzp_error_param { () => { 2 }; }
pub(crate) use bzp_error_param;

macro_rules! bzp_error_io { () => { 3 }; }
pub(crate) use bzp_error_io;

macro_rules! bzp_error_data { () => { 4 }; }
pub(crate) use bzp_error_data;

macro_rules! bzp_error_data_magic { () => { 5 }; }
pub(crate) use bzp_error_data_magic;

macro_rules! bzp_error_stream_compress_failure { () => { 10 }; }
pub(crate) use bzp_error_stream_compress_failure;
