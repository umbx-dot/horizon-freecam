fn freecam_movement(forward: &mut f32, strafe: &mut f32, jump: &mut bool, sneak: &mut bool) {
    crate::game::umbx_freecam_glue::movement(forward, strafe, jump, sneak);
}
