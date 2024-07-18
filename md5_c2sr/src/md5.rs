pub struct Md5Ctx {
    pub aul_state: [u32; 4],
    pub aul_count: [u32; 2],
    pub auc_buffer: [u8; 64],
    pub ui_pos: u32,
}

impl Md5Ctx {
    pub fn new() -> Self {
        Self {
            aul_state: [0; 4],
            aul_count: [0; 2],
            auc_buffer: [0; 64],
            ui_pos: 0,
        }
    }
}

macro_rules! md5_digest_len { () => { 16 } }
pub(crate) use md5_digest_len;

macro_rules! md5_input_len_max { () => { 0xffffffffffffffff } }
pub(crate) use md5_input_len_max;

macro_rules! md5_buffer_size { () => { 64 } }
pub(crate) use md5_buffer_size;

macro_rules! md5_text_in_buffer_max { () => { 56 } }
pub(crate) use md5_text_in_buffer_max;

macro_rules! md5_linear_func_f { ($B:expr, $C:expr, $D:expr) => { (($B & $C) | ((!$B) & $D)) } }
pub(crate) use md5_linear_func_f;

macro_rules! md5_linear_func_g { ($B:expr, $C:expr, $D:expr) => { (($B & $D) | ($C & (!$D))) } }
pub(crate) use md5_linear_func_g;

macro_rules! md5_linear_func_h { ($B:expr, $C:expr, $D:expr) => { ($B ^ $C ^ $D) } }
pub(crate) use md5_linear_func_h;

macro_rules! md5_linear_func_i { ($B:expr, $C:expr, $D:expr) => { ($C ^ ($B | (!$D))) } }
pub(crate) use md5_linear_func_i;

macro_rules! md5_record_message_len {
    ($context:expr) => {
        for i in 0..($context.aul_count.len()) {
            $context.auc_buffer[$context.ui_pos as usize] = ($context.aul_count[i] & 0xff) as u8;
            $context.ui_pos += 1;
            $context.auc_buffer[$context.ui_pos as usize] = (($context.aul_count[i] >> 8) & 0xff) as u8;
            $context.ui_pos += 1;
            $context.auc_buffer[$context.ui_pos as usize] = (($context.aul_count[i] >> 16) & 0xff) as u8;
            $context.ui_pos += 1;
            $context.auc_buffer[$context.ui_pos as usize] = (($context.aul_count[i] >> 24) & 0xff) as u8;
            $context.ui_pos += 1;
        }
    };
}
pub(crate) use md5_record_message_len;

macro_rules! md5_compose_digest {
    ($digest:expr, $md5_state:expr) => {
        let mut __i = 0;
        let mut __j = 0;
        while __i < $md5_state.len() {
            $digest[__j] = ($md5_state[__i] & 0xff) as u8;
            __j += 1;
            $digest[__j] = (($md5_state[__i] >> 8) & 0xff) as u8;
            __j += 1;
            $digest[__j] = (($md5_state[__i] >> 16) & 0xff) as u8;
            __j += 1;
            $digest[__j] = (($md5_state[__i] >> 24) & 0xff) as u8;
            __j += 1;
            __i += 1;
        }
    };
}
pub(crate) use md5_compose_digest;

macro_rules! md5_cycle_move {
    ($numMoved:expr, $moveBit:expr) => {
        let mut __tmpValue: u32;
        __tmpValue = ($numMoved) >> (32 - ($moveBit));
        $numMoved = ($numMoved) << ($moveBit);
        $numMoved += __tmpValue;
    };
}
pub(crate) use md5_cycle_move;

macro_rules! md5_change_state_in_turn {
    ($state:expr, $value:expr) => {
        $state[0] = $state[3];
        $state[3] = $state[2];
        $state[2] = $state[1];
        $state[1] = $state[1] + $value;
    };
}
pub(crate) use md5_change_state_in_turn;

macro_rules! md5_func_f {
    ($value:expr, $md5_state:expr, $text:expr, $add_end:expr, $move_bit:expr) => {
        $value = md5_linear_func_f!($md5_state[1], $md5_state[2], $md5_state[3]) + $md5_state[0] + $text + $add_end;
        md5_cycle_move!($value, $move_bit);
        md5_change_state_in_turn!($md5_state, $value);
    };
}   
pub(crate) use md5_func_f;

macro_rules! md5_func_g {
    ($value:expr, $md5_state:expr, $text:expr, $add_end:expr, $move_bit:expr) => {
        $value = md5_linear_func_g!($md5_state[1], $md5_state[2], $md5_state[3]) + $md5_state[0] + $text + $add_end;
        md5_cycle_move!($value, $move_bit);
        md5_change_state_in_turn!($md5_state, $value);
    };
}
pub(crate) use md5_func_g;

macro_rules! md5_func_h {
    ($value:expr, $md5_state:expr, $text:expr, $add_end:expr, $move_bit:expr) => {
        $value = md5_linear_func_h!($md5_state[1], $md5_state[2], $md5_state[3]) + $md5_state[0] + $text + $add_end;
        md5_cycle_move!($value, $move_bit);
        md5_change_state_in_turn!($md5_state, $value);
    };
}
pub(crate) use md5_func_h;

macro_rules! md5_func_i {
    ($value:expr, $md5_state:expr, $text:expr, $add_end:expr, $move_bit:expr) => {
        $value = md5_linear_func_i!($md5_state[1], $md5_state[2], $md5_state[3]) + $md5_state[0] + $text + $add_end;
        md5_cycle_move!($value, $move_bit);
        md5_change_state_in_turn!($md5_state, $value);
    };
}
pub(crate) use md5_func_i;

macro_rules! md5_f_proc {
    ($tmpValue:expr, $tmpState:expr, $textFragment:expr) => {
        md5_func_f!($tmpValue, $tmpState, $textFragment[0], 0xd76aa478, 7);
        md5_func_f!($tmpValue, $tmpState, $textFragment[1], 0xe8c7b756, 12);
        md5_func_f!($tmpValue, $tmpState, $textFragment[2], 0x242070db, 17);
        md5_func_f!($tmpValue, $tmpState, $textFragment[3], 0xc1bdceee, 22);
        md5_func_f!($tmpValue, $tmpState, $textFragment[4], 0xf57c0faf, 7);
        md5_func_f!($tmpValue, $tmpState, $textFragment[5], 0x4787c62a, 12);
        md5_func_f!($tmpValue, $tmpState, $textFragment[6], 0xa8304613, 17);
        md5_func_f!($tmpValue, $tmpState, $textFragment[7], 0xfd469501, 22);
        md5_func_f!($tmpValue, $tmpState, $textFragment[8], 0x698098d8, 7);
        md5_func_f!($tmpValue, $tmpState, $textFragment[9], 0x8b44f7af, 12);
        md5_func_f!($tmpValue, $tmpState, $textFragment[10], 0xffff5bb1, 17);
        md5_func_f!($tmpValue, $tmpState, $textFragment[11], 0x895cd7be, 22);
        md5_func_f!($tmpValue, $tmpState, $textFragment[12], 0x6b901122, 7);
        md5_func_f!($tmpValue, $tmpState, $textFragment[13], 0xfd987193, 12);
        md5_func_f!($tmpValue, $tmpState, $textFragment[14], 0xa679438e, 17);
        md5_func_f!($tmpValue, $tmpState, $textFragment[15], 0x49b40821, 22);
    } 
}
pub(crate) use md5_f_proc;

macro_rules! md5_g_proc {
    ($tmpValue:expr, $tmpState:expr, $textFragment:expr) => {
        md5_func_g!($tmpValue, $tmpState, $textFragment[1], 0xf61e2562, 5);
        md5_func_g!($tmpValue, $tmpState, $textFragment[6], 0xc040b340, 9);
        md5_func_g!($tmpValue, $tmpState, $textFragment[11], 0x265e5a51, 14);
        md5_func_g!($tmpValue, $tmpState, $textFragment[0], 0xe9b6c7aa, 20);
        md5_func_g!($tmpValue, $tmpState, $textFragment[5], 0xd62f105d, 5);
        md5_func_g!($tmpValue, $tmpState, $textFragment[10], 0x02441453, 9);
        md5_func_g!($tmpValue, $tmpState, $textFragment[15], 0xd8a1e681, 14);
        md5_func_g!($tmpValue, $tmpState, $textFragment[4], 0xe7d3fbc8, 20);
        md5_func_g!($tmpValue, $tmpState, $textFragment[9], 0x21e1cde6, 5);
        md5_func_g!($tmpValue, $tmpState, $textFragment[14], 0xc33707d6, 9);
        md5_func_g!($tmpValue, $tmpState, $textFragment[3], 0xf4d50d87, 14);
        md5_func_g!($tmpValue, $tmpState, $textFragment[8], 0x455a14ed, 20);
        md5_func_g!($tmpValue, $tmpState, $textFragment[13], 0xa9e3e905, 5);
        md5_func_g!($tmpValue, $tmpState, $textFragment[2], 0xfcefa3f8, 9);
        md5_func_g!($tmpValue, $tmpState, $textFragment[7], 0x676f02d9, 14);
        md5_func_g!($tmpValue, $tmpState, $textFragment[12], 0x8d2a4c8a, 20);
    };
}
pub(crate) use md5_g_proc;

macro_rules! md5_h_proc {
    ($tmpValue:expr, $tmpState:expr, $textFragment:expr) => {
        md5_func_h!($tmpValue, $tmpState, $textFragment[5], 0xfffa3942, 4);
        md5_func_h!($tmpValue, $tmpState, $textFragment[8], 0x8771f681, 11);
        md5_func_h!($tmpValue, $tmpState, $textFragment[11], 0x6d9d6122, 16);
        md5_func_h!($tmpValue, $tmpState, $textFragment[14], 0xfde5380c, 23);
        md5_func_h!($tmpValue, $tmpState, $textFragment[1], 0xa4beea44, 4);
        md5_func_h!($tmpValue, $tmpState, $textFragment[4], 0x4bdecfa9, 11);
        md5_func_h!($tmpValue, $tmpState, $textFragment[7], 0xf6bb4b60, 16);
        md5_func_h!($tmpValue, $tmpState, $textFragment[10], 0xbebfbc70, 23);
        md5_func_h!($tmpValue, $tmpState, $textFragment[13], 0x289b7ec6, 4);
        md5_func_h!($tmpValue, $tmpState, $textFragment[0], 0xeaa127fa, 11);
        md5_func_h!($tmpValue, $tmpState, $textFragment[3], 0xd4ef3085, 16);
        md5_func_h!($tmpValue, $tmpState, $textFragment[6], 0x04881d05, 23);
        md5_func_h!($tmpValue, $tmpState, $textFragment[9], 0xd9d4d039, 4);
        md5_func_h!($tmpValue, $tmpState, $textFragment[12], 0xe6db99e5, 11);
        md5_func_h!($tmpValue, $tmpState, $textFragment[15], 0x1fa27cf8, 16);
        md5_func_h!($tmpValue, $tmpState, $textFragment[2], 0xc4ac5665, 23);
    };
}
pub(crate) use md5_h_proc;

macro_rules! md5_i_proc {
    ($tmpValue:expr, $tmpState:expr, $textFragment:expr) => {
        md5_func_i!($tmpValue, $tmpState, $textFragment[0], 0xf4292244, 6);
        md5_func_i!($tmpValue, $tmpState, $textFragment[7], 0x432aff97, 10);
        md5_func_i!($tmpValue, $tmpState, $textFragment[14], 0xab9423a7, 15);
        md5_func_i!($tmpValue, $tmpState, $textFragment[5], 0xfc93a039, 21);
        md5_func_i!($tmpValue, $tmpState, $textFragment[12], 0x655b59c3, 6);
        md5_func_i!($tmpValue, $tmpState, $textFragment[3], 0x8f0ccc92, 10);
        md5_func_i!($tmpValue, $tmpState, $textFragment[10], 0xffeff47d, 15);
        md5_func_i!($tmpValue, $tmpState, $textFragment[1], 0x85845dd1, 21);
        md5_func_i!($tmpValue, $tmpState, $textFragment[8], 0x6fa87e4f, 6);
        md5_func_i!($tmpValue, $tmpState, $textFragment[15], 0xfe2ce6e0, 10);
        md5_func_i!($tmpValue, $tmpState, $textFragment[6], 0xa3014314, 15);
        md5_func_i!($tmpValue, $tmpState, $textFragment[13], 0x4e0811a1, 21);
        md5_func_i!($tmpValue, $tmpState, $textFragment[4], 0xf7537e82, 6);
        md5_func_i!($tmpValue, $tmpState, $textFragment[11], 0xbd3af235, 10);
        md5_func_i!($tmpValue, $tmpState, $textFragment[2], 0x2ad7d2bb, 15);
        md5_func_i!($tmpValue, $tmpState, $textFragment[9], 0xeb86d391, 21);
    }
}
pub(crate) use md5_i_proc;


pub fn md5_calc_digest_of_buff(context: &mut Box<Md5Ctx>) {
    let mut i: u32;
    let mut tmp_value: u32;
    let mut text_fragment: [u32; 16] = [0; 16];
    let mut tmp_state: [u32; 4] = [0; 4];
    let mut tmp_text = context.auc_buffer.as_mut_slice();

    tmp_state[0] = context.aul_state[0];
    tmp_state[1] = context.aul_state[1];
    tmp_state[2] = context.aul_state[2];
    tmp_state[3] = context.aul_state[3];

    for i in (0..16).step_by(4) {
        text_fragment[i] = (tmp_text[0] as u32) + ((tmp_text[1] as u32) << 8) +
            ((tmp_text[2] as u32) << 16) + ((tmp_text[3] as u32) << 24);
        text_fragment[i + 1] = (tmp_text[4] as u32) + ((tmp_text[5] as u32) << 8) +
            ((tmp_text[6] as u32) << 16) + ((tmp_text[7] as u32) << 24);
        text_fragment[i + 2] = (tmp_text[8] as u32) + ((tmp_text[9] as u32) << 8) +
            ((tmp_text[10] as u32) << 16) + ((tmp_text[11] as u32) << 24);
        text_fragment[i + 3] = (tmp_text[12] as u32) + ((tmp_text[13] as u32) << 8) +
            ((tmp_text[14] as u32) << 16) + ((tmp_text[15] as u32) << 24);
        tmp_text = &mut tmp_text[16..];
    }

    md5_f_proc!(tmp_value, tmp_state, text_fragment);
    md5_g_proc!(tmp_value, tmp_state, text_fragment);
    md5_h_proc!(tmp_value, tmp_state, text_fragment);
    md5_i_proc!(tmp_value, tmp_state, text_fragment);

    context.aul_state[0] += tmp_state[0];
    context.aul_state[1] += tmp_state[1];
    context.aul_state[2] += tmp_state[2];
    context.aul_state[3] += tmp_state[3];
}

pub fn md5_pad_buff(context: &mut Box<Md5Ctx>) -> bool {
    let need_another_buff = context.ui_pos >= md5_text_in_buffer_max!();

    context.auc_buffer[context.ui_pos as usize] = 0x80;
    context.ui_pos += 1;
    if need_another_buff {
        while context.ui_pos < md5_buffer_size!() {
            context.auc_buffer[context.ui_pos as usize] = 0;
            context.ui_pos += 1;
        }
    } else {
        while context.ui_pos < md5_text_in_buffer_max!() {
            context.auc_buffer[context.ui_pos as usize] = 0;
            context.ui_pos += 1;
        }
        md5_record_message_len!(context);
    }

    need_another_buff
}

pub fn md5_init(context: &mut Box<Md5Ctx>) {
    *context = Box::new(Md5Ctx::new());
    context.aul_state[0] = 0x67452301;
    context.aul_state[1] = 0xefcdab89;
    context.aul_state[2] = 0x98badcfe;
    context.aul_state[3] = 0x10325476;
}

pub fn md5_update(context: &mut Box<Md5Ctx>, input: &[u8], input_len: u32) {
    let mut total_input_bits: u64;
    let mut input_index: u32 = 0;
    let mut input_bit: u64;
    let mut tmp_pos: u32;
    let mut context_buffer: &mut [u8];

    if (input.is_empty() && input_len != 0) {
        return;
    }

    input_bit = (input_len as u64) << 3;
    total_input_bits = ((context.aul_count[1] as u64) << 32) + context.aul_count[0] as u64;
    if (md5_input_len_max!() - total_input_bits) < input_bit {
        return;
    }
    total_input_bits += input_bit;
    context.aul_count[0] = total_input_bits as u32;
    context.aul_count[1] = (total_input_bits >> 32) as u32;

    tmp_pos = context.ui_pos;
    while input_index < input_len {
        context_buffer = &mut context.auc_buffer;
        if tmp_pos < md5_buffer_size!() {
            context_buffer[tmp_pos as usize] = input[input_index as usize];
            input_index += 1;
            tmp_pos += 1;
            continue;
        }
        md5_calc_digest_of_buff(context);
        tmp_pos = 0;
    }
    
    if tmp_pos == md5_buffer_size!() {
        md5_calc_digest_of_buff(context);
        tmp_pos = 0;
    }
    context.ui_pos = tmp_pos;
    return;
}

pub fn md5_final_ex(digest: &mut [u8], buf_len: u32, context: &mut Box<Md5Ctx>) {
    let mut need_another_buff = false;

    if digest.is_empty() || buf_len < md5_digest_len!() as u32 {
        return;
    }

    need_another_buff = md5_pad_buff(context);
    md5_calc_digest_of_buff(context);

    if need_another_buff {
        context.ui_pos = 0;
        while context.ui_pos < md5_text_in_buffer_max!() {
            context.auc_buffer[context.ui_pos as usize] = 0;
            context.ui_pos += 1;
        }
        md5_record_message_len!(context);
        md5_calc_digest_of_buff(context);
    }

    md5_compose_digest!(digest, context.aul_state);
    *context = Box::new(Md5Ctx::new());
}

pub fn vos_md5_final(digest: &mut [u8], context: &mut Box<Md5Ctx>) {
    md5_final_ex(digest, md5_digest_len!(), context);
}

pub fn vos_md5_calc_ex(output: &mut [u8], output_len: u32, input: &[u8], input_len: u32) {
    let mut context = Box::new(Md5Ctx::new());

    if output_len < md5_digest_len!() as u32 {
        return;
    }

    md5_init(&mut context);
    md5_update(&mut context, input, input_len);
    md5_final_ex(output, output_len, &mut context);
}

pub fn vos_md5_calc(output: &mut [u8], input: &[u8], input_len: u32) {
    vos_md5_calc_ex(output, md5_digest_len!(), input, input_len);
}
