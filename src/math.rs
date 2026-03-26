#[cfg(target_os = "psp")]
pub fn sin32(n: f32) -> f32 {
    unsafe { psp::math::sinf(n) }
}
