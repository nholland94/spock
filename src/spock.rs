use vk::*;
use std::option::Option;
use std::result::Result as StdResult;
use std::ptr;

macro_rules! pointer_of_option {
    ($opt:expr) => {{
        match $opt {
            None            => ptr::null(),
            Some(value) => &value
        }
    }}
}

pub fn create_instance(create_info: InstanceCreateInfo, allocator_opt: Option<AllocationCallbacks>) -> StdResult<Instance, Result> {
    unsafe {
        let mut instance: Instance = ptr::null_mut();

        // println!("{:?} {:?}", &create_info, &mut instance);
        let result = vkCreateInstance(&create_info, pointer_of_option!(allocator_opt), &mut instance);

        // let create_info_ptr: *const InstanceCreateInfo = &create_info;
        // let instance_ptr: *mut Instance = &mut instance;
        // let allocator_ptr: *const AllocationCallbacks = pointer_of_option!(allocator_opt);
        // // let allocatorPtr = pointer_of_option!(allocatorOpt);
        // println!("{:?} {:?}",  instance_ptr, create_info_ptr);
        // let result = vkCreateInstance(&*create_info_ptr, &*allocator_ptr, &mut *instance_ptr);
        // println!("created instance");

        match result {
            Result::Success => Ok(instance),
            _               => Err(result)
        }
    }
}

pub fn destroy_instance(instance: Instance, allocator_opt: Option<AllocationCallbacks>) {
    unsafe {
        let allocator_ptr: *const AllocationCallbacks = pointer_of_option!(allocator_opt);

        vkDestroyInstance(instance, &*allocator_ptr);
    }
}

pub trait SpockInstance {
    fn enumerate_first_physical_device(self) -> StdResult<PhysicalDevice, Result>;
    // fn enumerate_physical_devices(self, u32) -> StdResult<[PhysicalDevice], Result>;
}

impl SpockInstance for Instance {
    fn enumerate_first_physical_device(self) -> StdResult<PhysicalDevice, Result> {
        unsafe {
            let mut count: u32 = 1;
            let mut physical_device: PhysicalDevice = ptr::null_mut();
            let result = vkEnumeratePhysicalDevices(self, &mut count, &mut physical_device);

            match result {
                Result::Success => Ok(physical_device),
                _               => Err(result)
            }
        }
    }


    // TODO: need to redo PhysicalDevice type so it can be packed into an array
    // pub fn enumerate_physical_devices(self, count: u32) -> StdResult<[PhysicalDevice], Result> {
    //     unsafe {
    //         let mut physical_devices: [PhysicalDevice; count];
    //         let result = vkEnumeratePhysicalDevices(self, &count, &mut physical_devices);

    //         match result {
    //             Result::Success => Ok(physical_devices),
    //             _               => Err(result)
    //         }
    //     }
    // }
}

pub trait SpockPhysicalDevice {
    fn get_queue_count(self) -> u32;
    fn get_all_queue_family_properties(self) -> Vec<QueueFamilyProperties>;
    fn get_memory_properties(self) -> PhysicalDeviceMemoryProperties;
    fn create_device(self, DeviceCreateInfo, Option<AllocationCallbacks>) -> StdResult<Device, Result>;
}

impl SpockPhysicalDevice for PhysicalDevice {
    fn get_queue_count(self) -> u32 {
        unsafe {
            let mut count: u32 = 0;
            vkGetPhysicalDeviceQueueFamilyProperties(self, &mut count, ptr::null_mut());
            count
        }
    }

    fn get_all_queue_family_properties(self) -> Vec<QueueFamilyProperties> {
        unsafe {
            let mut queue_count = self.get_queue_count();
            let mut properties = Vec::with_capacity(queue_count as usize);
            properties.resize(queue_count as usize, QueueFamilyProperties{..Default::default()});

            vkGetPhysicalDeviceQueueFamilyProperties(self, &mut queue_count, properties.as_mut_ptr());
            properties
        }
    }

    fn get_memory_properties(self) -> PhysicalDeviceMemoryProperties {
        unsafe {
            let mut memory_properties: PhysicalDeviceMemoryProperties = PhysicalDeviceMemoryProperties { ..Default::default() };
            vkGetPhysicalDeviceMemoryProperties(self, &mut memory_properties);
            memory_properties
        }
    }

    fn create_device(self, create_info: DeviceCreateInfo, allocator_opt: Option<AllocationCallbacks>) -> StdResult<Device, Result> {
        unsafe {
            let mut device: Device = ptr::null_mut();
            let result = vkCreateDevice(self, &create_info, pointer_of_option!(allocator_opt), &mut device);

            match result {
                Result::Success => Ok(device),
                _               => Err(result)
            }
        }
    }
}
