pub mod complexity;
pub mod deep_nesting;
pub mod large_file;
pub mod lcom;
pub mod long_params;

pub fn init() {
    complexity::init();
    deep_nesting::init();
    large_file::init();
    lcom::init();
    long_params::init();
}
