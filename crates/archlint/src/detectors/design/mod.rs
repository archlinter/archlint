pub mod abstractness;
pub mod barrel_abuse;
pub mod feature_envy;
pub mod god_module;
pub mod orphan_types;
pub mod primitive_obsession;
pub mod scattered_config;
pub mod scattered_module;
pub mod sdp_violation;
pub mod shared_mutable_state;
pub mod shotgun_surgery;
pub mod unstable_interface;

pub fn init() {
    abstractness::init();
    barrel_abuse::init();
    feature_envy::init();
    god_module::init();
    orphan_types::init();
    primitive_obsession::init();
    scattered_config::init();
    scattered_module::init();
    sdp_violation::init();
    shared_mutable_state::init();
    shotgun_surgery::init();
    unstable_interface::init();
}
