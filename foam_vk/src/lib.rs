use std::{error::Error, ffi::CString};

use ash::{self, vk};
use foam_common::{Event, FoamRenderer};

unsafe extern "system" fn vulkan_debug_utils_cb(
    severity: vk::DebugUtilsMessageSeverityFlagsEXT,
    message_type: vk::DebugUtilsMessageTypeFlagsEXT,
    cb_data: *const vk::DebugUtilsMessengerCallbackDataEXT,
    _data: *mut std::ffi::c_void,
) -> vk::Bool32 {
    let message = unsafe { std::ffi::CStr::from_ptr((*cb_data).p_message) };
    let severity = format!("{:?}", severity).to_lowercase();
    let ty = format!("{:?}", message_type).to_lowercase();
    println!("[Debug][{}][{}] {:?}", severity, ty, message);
    vk::FALSE
}

pub struct VkApp {
    instance: ash::Instance,
}

impl VkApp {
    pub fn new() -> Result<Self, Box<dyn Error>> {
        let entry = unsafe { ash::Entry::load()? };
        let engine_name = CString::new("foam_vk")?;
        let app_name = CString::new("Foam App")?;
        let app_info = vk::ApplicationInfo {
            p_application_name: app_name.as_ptr(),
            p_engine_name: engine_name.as_ptr(),
            api_version: vk::make_api_version(1, 0, 106, 0),
            ..Default::default()
        };
        let layer_names: Vec<CString> = vec![CString::new("VK_LAYER_KHRONOS_validation")?];
        let layer_names_ptrs: Vec<*const i8> = layer_names.iter().map(|i| i.as_ptr()).collect();
        let extension_name_ptrs: Vec<*const i8> = vec![ash::ext::debug_utils::NAME.as_ptr()];
        let instance_create_info = vk::InstanceCreateInfo {
            p_application_info: &app_info,
            pp_enabled_extension_names: extension_name_ptrs.as_ptr(),
            enabled_extension_count: extension_name_ptrs.len() as u32,
            pp_enabled_layer_names: layer_names_ptrs.as_ptr(),
            enabled_layer_count: layer_names_ptrs.len() as u32,

            ..Default::default()
        };
        let instance = unsafe { entry.create_instance(&instance_create_info, None)? };
        Ok(Self { instance })
    }
}

impl FoamRenderer for VkApp {
    fn clear(&mut self, color: u32) {}
    fn draw_square(&mut self, color: u32, w: u16, h: u16, x: i16, y: i16) {}
    fn end_drawing(&mut self) {}
    /// Unimplemented for `VkApp`, because winit polls everything itself
    fn poll_event(&mut self) -> Vec<Event> {
        unimplemented!()
    }
}

impl Drop for VkApp {
    fn drop(&mut self) {
        unsafe {
            self.instance.destroy_instance(None);
        }
    }
}
