use crate::utils::write_value;

pub fn patch_string_pool_size(addr_1: usize, addr_2: usize, size: u32) {
    write_value(addr_1, size);
    write_value(addr_2, size);
}