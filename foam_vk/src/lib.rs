use std::{
    borrow::Cow,
    error::Error,
    ffi::{self, CString},
    os::raw::c_char,
};

use foam_common::{Event, FoamRenderer};
use kazan::{
    LoadInstanceFn,
    vk::{self, vk1_0},
};
use winit::{raw_window_handle::HasDisplayHandle, window::Window};

unsafe extern "system" fn vulkan_debug_callback(
    message_severity: vk::DebugUtilsMessageSeverityFlagBitsEXT,
    message_type: vk::DebugUtilsMessageTypeFlagsEXT,
    p_callback_data: *const vk::DebugUtilsMessengerCallbackDataEXT<'_>,
    _user_data: *mut std::os::raw::c_void,
) -> vk::Bool32 {
    let callback_data = unsafe { *p_callback_data };
    let message_id_number = callback_data.message_id_number;

    let message_id_name = if callback_data.p_message_id_name.is_null() {
        Cow::from("")
    } else {
        unsafe { ffi::CStr::from_ptr(callback_data.p_message_id_name).to_string_lossy() }
    };

    let message = if callback_data.p_message.is_null() {
        Cow::from("")
    } else {
        unsafe { ffi::CStr::from_ptr(callback_data.p_message).to_string_lossy() }
    };

    println!(
        "{message_severity:?}:\n{message_type:?} [{message_id_name} ({message_id_number})] : {message}\n",
    );

    vk::FALSE
}

pub struct VkApp {
    pub entry: kazan::Entry,
    pub instance: vk::Instance,
    pub instance_fn: vk1_0::InstanceFn,
    pub debug_callback: vk::DebugUtilsMessengerEXT, //pub device: vk::Device,
}

impl VkApp {
    pub fn new(window: &Window) -> Result<Self, Box<dyn Error>> {
        let entry = kazan::Entry::linked()?;
        let app_name = c"Foam App";
        let layer_names = [c"VK_LAYER_KHRONOS_validation"];
        let layer_names_raw: Vec<*const c_char> = layer_names.iter().map(|f| f.as_ptr()).collect();
        let display_handle = window.display_handle()?.as_raw();
        let required = kazan::window::required_extensions(display_handle)?;
        #[allow(unused_mut)]
        let mut extension_names: Vec<*const c_char> =
            required.names().map(|n| n.as_ptr()).collect();
        let app_info = vk::ApplicationInfo::default()
            .application_name(app_name)
            .application_version(0)
            .engine_name(c"foam_vk")
            .engine_version(0)
            .api_version(vk1_0::API_VERSION);
        let create_info = vk::InstanceCreateInfo::default()
            .application_info(&app_info)
            .enabled_extension_names(&extension_names)
            .enabled_layer_names(&layer_names_raw);
        let instance = unsafe { entry.vk1_0.create_instance(&create_info, None)? };
        let instance_fn = unsafe { vk1_0::InstanceFn::load(&entry, instance)? };
        let debug_info = vk::DebugUtilsMessengerCreateInfoEXT::default()
            .message_severity(
                vk::DebugUtilsMessageSeverityFlagBitsEXT::ERROR_EXT
                    | vk::DebugUtilsMessageSeverityFlagBitsEXT::WARNING_EXT
                    | vk::DebugUtilsMessageSeverityFlagBitsEXT::INFO_EXT,
            )
            .message_type(
                vk::DebugUtilsMessageTypeFlagBitsEXT::GENERAL_EXT
                    | vk::DebugUtilsMessageTypeFlagBitsEXT::VALIDATION_EXT
                    | vk::DebugUtilsMessageTypeFlagBitsEXT::PERFORMANCE_EXT,
            )
            .pfn_user_callback(vulkan_debug_callback);
        let debug_utils_fn = unsafe { vk::ext::debug_utils::InstanceFn::load(&entry, instance)? };
        let debug_callback =
            unsafe { debug_utils_fn.create_debug_utils_messenger(instance, &debug_info, None)? };
        Ok(Self {
            entry,
            instance,
            instance_fn,
            debug_callback,
        })
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
        #[allow(unused_unsafe)]
        unsafe {}
    }
}
