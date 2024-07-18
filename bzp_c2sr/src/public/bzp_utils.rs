
macro_rules! bzp_base_block_size { () => { 100000 }; }
pub(crate) use bzp_base_block_size;
macro_rules! bzp_block_size_level_upper_limit { () => { 9 }; }
pub(crate) use bzp_block_size_level_upper_limit;
macro_rules! bzp_block_size_level_lower_limit { () => { 1 }; }
pub(crate) use bzp_block_size_level_lower_limit;
macro_rules! bzp_invalid_block_size {
    ($blockSize:expr) => {
        ($blockSize) < bzp_block_size_level_lower_limit!() || ($blockSize) > bzp_block_size_level_upper_limit!()
    };
}
pub(crate) use bzp_invalid_block_size;
macro_rules! bzp_invalid_alpha_size {
    ($alphaSize:expr) => {
        ($alphaSize) > bzp_max_alpha_size!() || ($alphaSize) < 1
    };
}
pub(crate) use bzp_invalid_alpha_size;
macro_rules! bzp_block_reserved_space_size { () => { 19 }; }
pub(crate) use bzp_block_reserved_space_size;
macro_rules! bzp_threshold_shell_sort { () => { 10 }; }
pub(crate) use bzp_threshold_shell_sort;
macro_rules! bzp_max_stack_size { () => { 100 }; }
pub(crate) use bzp_max_stack_size;
macro_rules! bzp_ascii_size { () => { 256 }; }
pub(crate) use bzp_ascii_size;
macro_rules! bzp_shell_sort_increment_nums { () => { 2 }; }
pub(crate) use bzp_shell_sort_increment_nums;
macro_rules! bzp_shell_sort_increment0 { () => { 1 }; }
pub(crate) use bzp_shell_sort_increment0;
macro_rules! bzp_shell_sort_increment1 { () => { 4 }; }
pub(crate) use bzp_shell_sort_increment1;
macro_rules! bzp_mtf_encode0 { () => { 0 }; }
pub(crate) use bzp_mtf_encode0;
macro_rules! bzp_mtf_encode1 { () => { 1 }; }
pub(crate) use bzp_mtf_encode1;
macro_rules! bzp_mtf_encode_base { () => { 2 }; }
pub(crate) use bzp_mtf_encode_base;
macro_rules! bzp_init_block_crc { () => { 0xffffffff }; }
pub(crate) use bzp_init_block_crc;
macro_rules! bzp_max_alpha_size { () => { 258 }; }
pub(crate) use bzp_max_alpha_size;
macro_rules! bzp_max_groups_num { () => { 6 }; }
pub(crate) use bzp_max_groups_num;
macro_rules! bzp_max_iter_num { () => { 4 }; }
pub(crate) use bzp_max_iter_num;
macro_rules! bzp_max_tree_height_encode { () => { 17 }; }
pub(crate) use bzp_max_tree_height_encode;
macro_rules! bzp_ngroups_block_num_limit0 { () => { 200 }; }
pub(crate) use bzp_ngroups_block_num_limit0;
macro_rules! bzp_ngroups_block_num_limit1 { () => { 600 }; }
pub(crate) use bzp_ngroups_block_num_limit1;
macro_rules! bzp_ngroups_block_num_limit2 { () => { 1200 }; }
pub(crate) use bzp_ngroups_block_num_limit2;
macro_rules! bzp_ngroups_block_num_limit3 { () => { 2400 }; }
pub(crate) use bzp_ngroups_block_num_limit3;
macro_rules! bzp_ngroups_num_0 { () => { 2 }; }
pub(crate) use bzp_ngroups_num_0;
macro_rules! bzp_ngroups_num_1 { () => { 3 }; }
pub(crate) use bzp_ngroups_num_1;
macro_rules! bzp_ngroups_num_2 { () => { 4 }; }
pub(crate) use bzp_ngroups_num_2;
macro_rules! bzp_ngroups_num_3 { () => { 5 }; }
pub(crate) use bzp_ngroups_num_3;
macro_rules! bzp_ngroups_num_4 { () => { 6 }; }
pub(crate) use bzp_ngroups_num_4;

macro_rules! bzp_elems_num_in_one_group { () => { 50 }; }
pub(crate) use bzp_elems_num_in_one_group;
macro_rules! bzp_huffman_height_weight_bits { () => { 8 }; }
pub(crate) use bzp_huffman_height_weight_bits;
macro_rules! bzp_huffman_len_max_cost { () => { 15 }; }
pub(crate) use bzp_huffman_len_max_cost;
macro_rules! bzp_huffman_len_upper_limit { () => { 20 }; }
pub(crate) use bzp_huffman_len_upper_limit;
macro_rules! bzp_huffman_max_size_select {
    () => {
        bzp_block_size_level_upper_limit!() * bzp_base_block_size!() / bzp_elems_num_in_one_group!()
    };
}
pub(crate) use bzp_huffman_max_size_select;
macro_rules! bzp_max_fun {
    ($a:expr, $b:expr) => {
        if $a > $b {
            $a
        } else {
            $b
        }
    };
}
pub(crate) use bzp_max_fun;
macro_rules! bzp_min_fun {
    ($a:expr, $b:expr) => {
        if $a < $b {
            $a
        } else {
            $b
        }
    };
}
pub(crate) use bzp_min_fun;
