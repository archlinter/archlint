pub mod circular_type_deps;
pub mod cycles;
pub mod high_coupling;
pub mod hub_dependency;
pub mod hub_module;
pub mod layer_violation;
pub mod package_cycle;
pub mod vendor_coupling;

pub const fn init() {
    circular_type_deps::init();
    cycles::init();
    high_coupling::init();
    hub_dependency::init();
    hub_module::init();
    layer_violation::init();
    package_cycle::init();
    vendor_coupling::init();
}
