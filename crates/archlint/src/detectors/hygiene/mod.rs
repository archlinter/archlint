pub mod dead_code;
pub mod dead_symbols;
pub mod side_effect_import;
pub mod test_leakage;

pub fn init() {
    dead_code::init();
    dead_symbols::init();
    side_effect_import::init();
    test_leakage::init();
}
