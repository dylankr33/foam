use foam_proc_macro::cfg_modern;

#[cfg(target_os = "psp")]
pub fn sin32(n: f32) -> f32 {
    unsafe { psp::math::sinf(n) }
}

#[cfg_modern]
pub fn sin32(n: f32) -> f32 {
    n.sin()
}
