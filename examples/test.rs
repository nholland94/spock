extern crate vulkan;
use vulkan::vk;
use vulkan::spock::*;
use std::ffi::CString;

fn main() {
    let application_info = vk::ApplicationInfo {
        pApplicationName: CString::new("Testing").unwrap().as_ptr(),
        pEngineName:      CString::new("Test").unwrap().as_ptr(),
        ..Default::default()
    };

    let instance_create_info = vk::InstanceCreateInfo {
        pApplicationInfo: &application_info,
        ..Default::default()
    };

    let instance = create_instance(instance_create_info, None).unwrap();
    let physical_device = instance.enumerate_first_physical_device().unwrap();
    let queue_properties = physical_device.get_all_queue_family_properties();

    let graphics_queue_index_opt = queue_properties.iter().position(|&p| (p.queueFlags as u32 & vk::QueueFlags::Graphics as u32) > 0);

    if graphics_queue_index_opt.is_none() {
        panic!("Could not find graphics queue");
    }

    let queue_create_info = vk::DeviceQueueCreateInfo {
        queueFamilyIndex: graphics_queue_index_opt.unwrap() as u32,
        queueCount: 1,
        ..Default::default()
    };

    let device_create_info = vk::DeviceCreateInfo {
        queueCreateInfoCount: 1,
        pQueueCreateInfos: &queue_create_info,
        ..Default::default()
    };

    let device = physical_device.create_device(device_create_info, None).unwrap();

    println!("success!");

    destroy_instance(instance, None);
}
