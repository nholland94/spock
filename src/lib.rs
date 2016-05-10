#![crate_type = "rlib"]
#![crate_name = "spock"]

extern crate libc;

pub mod types;
pub mod vk;

use types::*;
use vk::*;

use std::option::Option;
use std::result::Result;
use std::ptr;

macro_rules! pointer_of_option {
    ($opt:expr) => {{
        match $opt {
            None        => ptr::null(),
            Some(value) => &value
        }
    }}
}

macro_rules! vulkan_result {
    ($err:expr, $ret:expr) => {{
        match $err {
            Error::Success => Ok($ret),
            _              => Err($err)
        }
    }}
}

pub fn create_instance(create_info: InstanceCreateInfo, allocator_opt: Option<AllocationCallbacks>) -> Result<Instance, Error> {
    unsafe {
        let mut instance: Instance = ptr::null_mut();
        let result = vkCreateInstance(&create_info, pointer_of_option!(allocator_opt), &mut instance);
        vulkan_result!(result, instance)
    }
}

// pub fn count_instance_extension_properties(layer_name: String) -> Result<u32, Error> {
//     unsafe {
//         let mut count = 0;
//         let result = vkEnumerateInstanceExtensionProperties(layer_name, &mut count, ptr::null_mut());
//         vulkan_result!(result, count)
//     }
// }

// pub fn enumerate_instance_extension_properties(layer_name: String, property_count: u32) -> Result<Vec<ExtensionProperty>, Error> {
// }



// pub fn enumerate_instance_layer_properties(layer_name: String, property_count: u32) -> Result<Vec<LayerProperty>, Error> {
// }

pub trait SpockInstance {
    fn destroy(self, allocator_opt: Option<AllocationCallbacks>);
    fn count_physical_devices(self) -> Result<u32, Error>;
    fn enumerate_physical_devices(self, u32) -> Result<Vec<PhysicalDevice>, Error>;
    fn enumerate_all_physical_devices(self) -> Result<Vec<PhysicalDevice>, Error>;
    // fn get_proc_addr(self, name: String) -> ???;
}

impl SpockInstance for Instance {
    fn destroy(self, allocator_opt: Option<AllocationCallbacks>) {
        unsafe { vkDestroyInstance(self, pointer_of_option!(allocator_opt)); }
    }

    fn count_physical_devices(self) -> Result<u32, Error> {
        unsafe {
            let mut count = 0;
            let result = vkEnumeratePhysicalDevices(self, &mut count, ptr::null_mut());
            vulkan_result!(result, count)
        }
    }

    fn enumerate_physical_devices(self, count: u32) -> Result<Vec<PhysicalDevice>, Error> {
        unsafe {
            let mut count_mut = count;
            let mut physical_devices = Vec::with_capacity(count as usize);
            physical_devices.resize(count as usize, ptr::null_mut());

            let result = vkEnumeratePhysicalDevices(self, &mut count_mut, physical_devices.as_mut_ptr()); 
            vulkan_result!(result, physical_devices)
        }
    }

    fn enumerate_all_physical_devices(self) -> Result<Vec<PhysicalDevice>, Error> {
        self.enumerate_physical_devices(try!(self.count_physical_devices()))
    }
}

pub trait SpockPhysicalDevice {
    fn create_device(self, DeviceCreateInfo, Option<AllocationCallbacks>) -> Result<Device, Error>;
    fn get_features(self) -> PhysicalDeviceFeatures;
    fn get_format_properties(self, Format) -> FormatProperties;
    fn get_image_format_properties(self, Format, ImageType, ImageTiling, ImageUsageFlags, ImageCreateFlags) -> Result<ImageFormatProperties, Error>;
    fn get_properties(self) -> PhysicalDeviceProperties;
    fn get_queue_count(self) -> u32;
    fn get_queue_family_properties(self, u32) -> Vec<QueueFamilyProperties>;
    fn get_all_queue_family_properties(self) -> Vec<QueueFamilyProperties>;
    fn get_memory_properties(self) -> PhysicalDeviceMemoryProperties;
    fn get_sparse_image_format_properties_count(self, Format, ImageType, SampleCountFlags, ImageUsageFlags, ImageTiling) -> u32;
    fn get_sparse_image_format_properties(self, Format, ImageType, SampleCountFlags, ImageUsageFlags, ImageTiling, u32) -> Vec<SparseImageFormatProperties>;
    fn get_all_sparse_image_format_properties(self, Format, ImageType, SampleCountFlags, ImageUsageFlags, ImageTiling) -> Vec<SparseImageFormatProperties>;
    // fn enumerate_extension_properties(self, layer_name: String, property_count: u32) -> Result<Vec<ExtensionProperty>, Error>
    // fn enumerate_layer_properties(self, layer_name: String, property_count: u32) -> Result<Vec<LayerPropery>, Error>
}

impl SpockPhysicalDevice for PhysicalDevice {
    fn get_features(self) -> PhysicalDeviceFeatures {
        unsafe {
            let mut features= PhysicalDeviceFeatures{..Default::default()};
            vkGetPhysicalDeviceFeatures(self, &mut features);
            features
        }
    }

    fn get_format_properties(self, format: Format) -> FormatProperties {
        unsafe {
            let mut properties = FormatProperties{..Default::default()};
            vkGetPhysicalDeviceFormatProperties(self, format, &mut properties);
            properties
        }
    }

    fn get_image_format_properties(self, format: Format, image_type: ImageType, tiling: ImageTiling, usage_flags: ImageUsageFlags, create_flags: ImageCreateFlags) -> Result<ImageFormatProperties, Error> {
        unsafe {
            let mut properties = ImageFormatProperties{..Default::default()};
            let result = vkGetPhysicalDeviceImageFormatProperties(self, format, image_type, tiling, usage_flags, create_flags, &mut properties);
            vulkan_result!(result, properties)
        }
    }

    fn get_properties(self) -> PhysicalDeviceProperties {
        unsafe {
            let mut properties = PhysicalDeviceProperties{..Default::default()};
            vkGetPhysicalDeviceProperties(self, &mut properties);
            properties
        }
    }

    fn get_queue_count(self) -> u32 {
        unsafe {
            let mut count: u32 = 0;
            vkGetPhysicalDeviceQueueFamilyProperties(self, &mut count, ptr::null_mut());
            count
        }
    }

    fn get_queue_family_properties(self, queue_count: u32) -> Vec<QueueFamilyProperties> {
        unsafe {
            let mut mut_queue_count = queue_count;
            let mut properties = Vec::with_capacity(queue_count as usize);
            properties.resize(queue_count as usize, QueueFamilyProperties{..Default::default()});

            vkGetPhysicalDeviceQueueFamilyProperties(self, &mut mut_queue_count, properties.as_mut_ptr());
            properties
        }
    }

    fn get_all_queue_family_properties(self) -> Vec<QueueFamilyProperties> {
        self.get_queue_family_properties(self.get_queue_count())
    }

    fn get_memory_properties(self) -> PhysicalDeviceMemoryProperties {
        unsafe {
            let mut memory_properties: PhysicalDeviceMemoryProperties = PhysicalDeviceMemoryProperties { ..Default::default() };
            vkGetPhysicalDeviceMemoryProperties(self, &mut memory_properties);
            memory_properties
        }
    }

    fn create_device(self, create_info: DeviceCreateInfo, allocator_opt: Option<AllocationCallbacks>) -> Result<Device, Error> {
        unsafe {
            let mut device: Device = ptr::null_mut();
            let result = vkCreateDevice(self, &create_info, pointer_of_option!(allocator_opt), &mut device);
            vulkan_result!(result, device)
        }
    }

    fn get_sparse_image_format_properties_count(self, format: Format, image_type: ImageType, sample_count: SampleCountFlags, usage: ImageUsageFlags, tiling: ImageTiling) -> u32 {
        unsafe {
            let mut count: u32 = 0;
            vkGetPhysicalDeviceSparseImageFormatProperties(self, format, image_type, sample_count, usage, tiling, &mut count, ptr::null_mut());
            count
        }
    }

    fn get_sparse_image_format_properties(self, format: Format, image_type: ImageType, sample_count: SampleCountFlags, usage: ImageUsageFlags, tiling: ImageTiling, count: u32) -> Vec<SparseImageFormatProperties> {
        unsafe {
            let mut mut_count = count;
            let mut properties = Vec::with_capacity(count as usize);
            properties.resize(count as usize, SparseImageFormatProperties{..Default::default()});

            vkGetPhysicalDeviceSparseImageFormatProperties(self, format, image_type, sample_count, usage, tiling, &mut mut_count, properties.as_mut_ptr());
            properties
        }
    }

    fn get_all_sparse_image_format_properties(self, format: Format, image_type: ImageType, sample_count: SampleCountFlags, usage: ImageUsageFlags, tiling: ImageTiling) -> Vec<SparseImageFormatProperties> {
        let count = self.get_sparse_image_format_properties_count(format, image_type, sample_count, usage, tiling);
        self.get_sparse_image_format_properties(format, image_type, sample_count, usage, tiling, count)
    }

}

pub trait SpockDevice {
    fn destroy(self, Option<AllocationCallbacks>);
    fn get_queue(self, u32, u32) -> Result<Queue, Error>;
    fn wait_idle(self) -> Error;
    fn allocate_memory(self, MemoryAllocateInfo, Option<AllocationCallbacks>) -> Result<DeviceMemory, Error>;
    fn free_memory(self, DeviceMemory, Option<AllocationCallbacks>);
    // fn map_memory(self, memory: DeviceMemory, offset: DeviceSize, size: DeviceSize, flags: MemoryMapFlags, pointer: *mut *mut c_void???) -> Error;
    fn flush_mapped_memory_ranges(self, Vec<MappedMemoryRange>) -> Error;
    fn invalidate_mapped_memory_ranges(self, Vec<MappedMemoryRange>) -> Error;
    fn get_memory_commitment(self, DeviceMemory) -> DeviceSize;
    fn bind_buffer_memory(self, Buffer, DeviceMemory, DeviceSize) -> Error;
    fn bind_image_memory(self, Image, DeviceMemory, DeviceSize) -> Error;
    fn get_buffer_memory_requirements(self, Buffer) -> MemoryRequirements;
    fn get_image_memory_requirements(self, Image) -> MemoryRequirements;
    fn get_image_sparse_memory_requirements_count(self, Image) -> u32;
    fn get_image_sparse_memory_requirements(self, Image, count: u32) -> SparseImageMemoryRequirements;
    fn get_all_image_sparse_memory_requirements(self, Image) -> SparseImageMemoryRequirements;
    fn create_fence(self, FenceCreateInfo, Option<AllocationCallbacks>) -> Result<Fence, Error>;
    fn destroy_fence(self, Fence, Option<AllocationCallbacks>);
    fn reset_fences(self, Vec<Fence>) -> Error;
    fn get_fence_status(self, Fence) -> Error;
    fn wait_for_fences(self, Vec<Fence>, bool, u64) -> Error;
    fn create_semaphore(self, SemaphoreCreateInfo, Option<AllocationCallbacks>) -> Result<Semaphore, Error>;
    fn destroy_semaphore(self, Semaphore, Option<AllocationCallbacks>);
    fn create_event(self, EventCreateInfo, Option<AllocationCallbacks>) -> Result<Event, Error>;
    fn destroy_event(self, Event, Option<AllocationCallbacks>);
    fn get_event_status(self, Event) -> Error;
    fn set_event(self, Event) -> Error;
    fn reset_event(self, Event) -> Error;
    fn create_query_pool(self, QueryPoolCreateInfo, Option<AllocationCallbacks>) -> Result<QueryPool, Error>;
    fn destroy_query_pool(self, QueryPool, Option<AllocationCallbacks>);
    // fn get_query_pool_errors(self, QueryPool, u32, u32, ???...);
    fn create_buffer(self, BufferCreateInfo, Option<AllocationCallbacks>) -> Result<Buffer, Error>;
    fn destroy_buffer(self, Buffer, Option<AllocationCallbacks>);
    fn create_buffer_view(self, BufferViewCreateInfo, Option<AllocationCallbacks>) -> Result<BufferView, Error>;
    fn destroy_buffer_view(self, BufferView, Option<AllocationCallbacks>);
    fn create_image(self, ImageCreateInfo, Option<AllocationCallbacks>) -> Result<Image, Error>;
    fn destroy_image(self, Image, Option<AllocationCallbacks>);
    fn get_image_subresource_layout(self, Image, ImageSubresource) -> SubresourceLayout;
    fn create_image_view(self, ImageViewCreateInfo, Option<AllocationCallbacks>) -> Result<ImageView, Error>;
    fn destroy_image_view(self, ImageView, Option<AllocationCallbacks>);
    fn create_shader_module(self, ShaderModuleCreateInfo, Option<AllocationCallbacks>) -> Result<ShaderModule, Error>;
    fn destroy_shader_module(self, ShaderModule, Option<AllocationCallbacks>);
    fn create_pipeline_cache(self, PipelineCacheCreateInfo, Option<AllocationCallbacks>) -> Result<PipelineCache, Error>;
    fn destroy_pipeline_cache(self, PipelineCache, Option<AllocationCallbacks>);
    // fn get_pipeline_cache_data(self, PipelineCache, ???...);
    fn merge_pipeline_caches(self, PipelineCache, Vec<PipelineCache>) -> Error;
    fn create_graphics_pipelines(self, PipelineCache, Vec<GraphicsPipelineCreateInfo>, Option<AllocationCallbacks>) -> Result<Vec<Pipeline>, Error>;
    fn create_compute_pipelines(self, PipelineCache, Vec<ComputePipelineCreateInfo>, Option<AllocationCallbacks>) -> Result<Vec<Pipeline>, Error>;
    fn destroy_pipeline(self, Pipeline, Option<AllocationCallbacks>);
    fn create_pipeline_layout(self, PipelineLayoutCreateInfo, Option<AllocationCallbacks>) -> Result<PipelineLayout, Error>;
    fn destroy_pipeline_layout(self, Pipeline, Option<AllocationCallbacks>);
    fn create_sampler(self, SamplerCreateInfo, Option<AllocationCallbacks>) -> Error;
    fn destroy_sampler(self, Sampler, Option<AllocationCallbacks>);
    fn create_descriptor_set_layout(self, DescriptorSetLayoutCreateInfo, Option<AllocationCallbacks>) -> Result<DescriptorSetLayout, Error>;
    fn destroy_descriptor_set_layout(self, DescriptorSetLayout, Option<AllocationCallbacks>);
    fn create_descriptor_pool(self, DescriptorPoolCreateInfo, Option<AllocationCallbacks>) -> Result<DescriptorPool, Error>;
    fn destroy_descriptor_pool(self, DescriptorPool, Option<AllocationCallbacks>);
    fn reset_descriptor_pool(self, DescriptorPool, DescriptorPoolResetFlags) -> Error;
    fn allocate_descriptor_set(self, DescriptorSetAllocateInfo) -> Result<Vec<DescriptorSet>, Error>;
    fn free_descriptor_sets(self, DescriptorPool, Vec<DescriptorSet>) -> Error;
    fn update_descriptor_sets(self, Vec<WriteDescriptorSet>, Vec<CopyDescriptorSet>);
    fn create_framebuffer(self, FramebufferCreateInfo, Option<AllocationCallbacks>) -> Result<Framebuffer, Error>;
    fn destroy_framebuffer(self, Framebuffer, Option<AllocationCallbacks>);
    fn create_render_pass(self, RenderPassCreateInfo, Option<AllocationCallbacks>) -> Result<RenderPass, Error>;
    fn destroy_render_pass(self, RenderPass, Option<AllocationCallbacks>);
    fn get_render_area_granularity(self, RenderPass) -> Extent2D;
    fn create_command_pool(self, CommandPoolCreateInfo, Option<AllocationCallbacks>) -> Result<CommandPool, Error>;
    fn destroy_command_pool(self, CommandPool, Option<AllocationCallbacks>);
    fn reset_command_pool(self, CommandPool, CommandPoolResetFlags) -> Error;
    fn allocate_command_buffers(self, CommandBufferAllocateInfo) -> Result<Vec<CommandBuffer>, Error>;
    fn free_command_buffers(self, CommandPool, Vec<CommandBuffer>);
    // fn get_proc_addr(self, name: String) -> ???;
}

impl SpockDevice for Device {
    fn destroy(self, allocator_opt: Option<AllocationCallbacks>) {
        unsafe { vkDestroyDevice(self, pointer_of_option!(allocator_opt)); }
    }

    fn get_queue(self, queue_family_index: u32, queue_index: u32) -> Result<Queue, Error> {
        unsafe {
            let mut queue: Queue = ptr::null_mut();
            let result = vkGetDeviceQueue(self, queue_family_index, queue_index, &mut queue);
            vulkan_result!(result, queue)
        }
    }

    fn wait_idle(self) -> Error {
        unsafe { vkDeviceWaitIdle(self) }
    }

    fn allocate_memory(self, allocate_info: MemoryAllocateInfo, allocator_opt: Option<AllocationCallbacks>) -> Result<DeviceMemory, Error> {
        unsafe {
            let mut memory: DeviceMemory = ptr::null_mut();
            let result = vkAllocateMemory(self, &allocate_info, pointer_of_option!(allocator_opt), &mut memory);
            vulkan_result!(result, memory)
        }
    }

    fn free_memory(self, memory: DeviceMemory, allocator_opt: Option<AllocationCallbacks>) {
        unsafe {
            vkFreeMemory(self, memory, pointer_of_option!(allocator_opt));
        }
    }

    // fn map_memory(...) {
    // }

    fn flush_mapped_memory_ranges(self, ranges: Vec<MappedMemoryRange>) -> Error {
        unsafe { vkFlushMappedMemoryRanges(self, ranges.len() as u32, ranges.as_ptr()) }
    }

    fn invalidate_mapped_memory_ranges(self, ranges: Vec<MappedMemoryRange>) -> Error {
        unsafe { vkInvalidateMappedMemoryRanges(self, ranges.len() as u32, ranges.as_ptr()) }
    }

    fn get_memory_commitment(self, memory: DeviceMemory) -> DeviceSize {
        unsafe {
            let mut commitment: DeviceSize = 0;
            vkGetDeviceMemoryCommitment(self, memory, &mut commitment);
            commitment
        }
    }

    fn bind_buffer_memory(self, buffer: Buffer, memory: DeviceMemory, size: DeviceSize) -> Error {
        unsafe { vkBindBufferMemory(self, buffer, memory, size) }
    }

    fn bind_image_memory(self, image: Image, memory: DeviceMemory, size: DeviceSize) -> Error {
        unsafe { vkBindImageMemory(self, image, memory, size) }
    }

    fn get_buffer_memory_requirements(self, buffer: Buffer) -> MemoryRequirements {
        unsafe {
            let mut requirements = MemoryRequirements{..Default::default()};
            vkGetBufferMemoryRequirements(self, buffer, &mut requirements);
            requirements
        }
    }

    fn get_image_memory_requirements(self, image: Image) -> MemoryRequirements {
        unsafe {
            let mut requirements = MemoryRequirements{..Default::default()};
            vkGetImageMemoryRequirements(self, image, &mut requirements);
            requirements
        }
    }

    fn get_image_sparse_memory_requirements_count(self, image: Image) -> u32 {
        unsafe {
            let mut count: u32 = 0;
            vkGetImageSparseMemoryRequirements(self, image, &mut count, ptr::null_mut());
            count
        }
    }

    fn get_image_sparse_memory_requirements(self, image: Image, count: u32) -> SparseImageMemoryRequirements {
        unsafe {
            let mut mut_count = count;
            let mut requirements = SparseImageMemoryRequirements{..Default::default()};
            vkGetImageSparseMemoryRequirements(self, image, &mut mut_count, &mut requirements);
            requirements
        }
    }

    fn get_all_image_sparse_memory_requirements(self, image: Image) -> SparseImageMemoryRequirements {
        self.get_image_sparse_memory_requirements(image, self.get_image_sparse_memory_requirements_count(image))
    }

    fn create_fence(self, create_info: FenceCreateInfo, allocator_opt: Option<AllocationCallbacks>) -> Result<Fence, Error> {
        unsafe {
            let mut fence: Fence = ptr::null_mut();
            let result = vkCreateFence(self, &create_info, pointer_of_option!(allocator_opt), &mut fence);
            vulkan_result!(result, fence)
        }
    }

    fn destroy_fence(self, fence: Fence, allocator_opt: Option<AllocationCallbacks>) {
        unsafe { vkDestroyFence(self, fence, pointer_of_option!(allocator_opt)); }
    }

    fn reset_fences(self, fences: Vec<Fence>) -> Error {
        unsafe { vkResetFences(self, fences.len() as u32, fences.as_ptr()) }
    }

    fn get_fence_status(self, fence: Fence) -> Error {
        unsafe { vkGetFenceStatus(self, fence) }
    }

    fn wait_for_fences(self, fences: Vec<Fence>, wait_for_all: bool, timeout: u64) -> Error {
        unsafe { vkWaitForFences(self, fences.len() as u32, fences.as_ptr(), wait_for_all as u32, timeout) }
    }

    fn create_semaphore(self, create_info: SemaphoreCreateInfo, allocator_opt: Option<AllocationCallbacks>) -> Result<Semaphore, Error> {
        unsafe {
            let mut semaphore: Semaphore = ptr::null_mut();
            let result = vkCreateSemaphore(self, &create_info, pointer_of_option!(allocator_opt), &mut semaphore);
            vulkan_result!(result, semaphore)
        }
    }

    fn destroy_semaphore(self, semaphore: Semaphore, allocator_opt: Option<AllocationCallbacks>) {
        unsafe { vkDestroySemaphore(self, semaphore, pointer_of_option!(allocator_opt)); }
    }

    fn create_event(self, create_info: EventCreateInfo, allocator_opt: Option<AllocationCallbacks>) -> Result<Event, Error> {
        unsafe {
            let mut event: Event = ptr::null_mut();
            let result = vkCreateEvent(self, &create_info, pointer_of_option!(allocator_opt), &mut event);
            vulkan_result!(result, event)
        }
    }

    fn destroy_event(self, event: Event, allocator_opt: Option<AllocationCallbacks>) {
        unsafe { vkDestroyEvent(self, event, pointer_of_option!(allocator_opt)); }
    }

    fn get_event_status(self, event: Event) -> Error {
        unsafe { vkGetEventStatus(self, event) }
    }

    fn set_event(self, event: Event) -> Error {
        unsafe { vkSetEvent(self, event) }
    }

    fn reset_event(self, event: Event) -> Error {
        unsafe { vkResetEvent(self, event) }
    }

    fn create_query_pool(self, create_info: QueryPoolCreateInfo, allocator_opt: Option<AllocationCallbacks>) -> Result<QueryPool, Error> {
        unsafe {
            let mut query_pool: QueryPool = ptr::null_mut();
            let result = vkCreateQueryPool(self, &create_info, pointer_of_option!(allocator_opt), &mut query_pool);
            vulkan_result!(result, query_pool)
        }
    }

    fn destroy_query_pool(self, query_pool: QueryPool, allocator_opt: Option<AllocationCallbacks>) {
        unsafe { vkDestroyQueryPool(self, query_pool, pointer_of_option!(allocator_opt)); }
    }

    fn create_buffer(self, create_info: BufferCreateInfo, allocator_opt: Option<AllocationCallbacks>) -> Result<Buffer, Error> {
        unsafe {
            let mut buffer: Buffer = ptr::null_mut();
            let result = vkCreateBuffer(self, &create_info, pointer_of_option!(allocator_opt), &mut buffer);
            vulkan_result!(result, buffer)
        }
    }

    fn destroy_buffer(self, buffer: Buffer, allocator_opt: Option<AllocationCallbacks>) {
        unsafe { vkDestroyBuffer(self, buffer, pointer_of_option!(allocator_opt)); }
    }

    fn create_buffer_view(self, create_info: BufferViewCreateInfo, allocator_opt: Option<AllocationCallbacks>) -> Result<BufferView, Error> {
        unsafe {
            let mut buffer_view: BufferView = ptr::null_mut();
            let result = vkCreateBufferView(self, &create_info, pointer_of_option!(allocator_opt), &mut buffer_view);
            vulkan_result!(result, buffer_view)
        }
    }

    fn destroy_buffer_view(self, buffer_view: BufferView, allocator_opt: Option<AllocationCallbacks>) {
        unsafe { vkDestroyBufferView(self, buffer_view, pointer_of_option!(allocator_opt)); }
    }

    fn create_image(self, create_info: ImageCreateInfo, allocator_opt: Option<AllocationCallbacks>) -> Result<Image, Error> {
        unsafe {
            let mut image: Image = ptr::null_mut();
            let result = vkCreateImage(self, &create_info, pointer_of_option!(allocator_opt), &mut image);
            vulkan_result!(result, image)
        }
    }

    fn destroy_image(self, image: Image, allocator_opt: Option<AllocationCallbacks>) {
        unsafe { vkDestroyImage(self, image, pointer_of_option!(allocator_opt)); }
    }

    fn get_image_subresource_layout(self, image: Image, subresource: ImageSubresource) -> SubresourceLayout {
        unsafe {
            let mut layout = SubresourceLayout{..Default::default()};
            vkGetImageSubresourceLayout(self, image, &subresource, &mut layout);
            layout
        }
    }

    fn create_image_view(self, create_info: ImageViewCreateInfo, allocator_opt: Option<AllocationCallbacks>) -> Result<ImageView, Error> {
        unsafe {
            let mut image_view: ImageView = ptr::null_mut();
            let result = vkCreateImageView(self, &create_info, pointer_of_option!(allocator_opt), &mut image_view);
            vulkan_result!(result, image_view)
        }
    }

    fn destroy_image_view(self, image_view: ImageView, allocator_opt: Option<AllocationCallbacks>) {
        unsafe { vkDestroyImageView(self, image_view, pointer_of_option!(allocator_opt)); }
    }

    fn create_shader_module(self, create_info: ShaderModuleCreateInfo, allocator_opt: Option<AllocationCallbacks>) -> Result<ShaderModule, Error> {
        unsafe {
            let mut shader_module: ShaderModule = ptr::null_mut();
            let result = vkCreateShaderModule(self, &create_info, pointer_of_option!(allocator_opt), &mut shader_module);
            vulkan_result!(result, shader_module)
        }
    }

    fn destroy_shader_module(self, shader_module: ShaderModule, allocator_opt: Option<AllocationCallbacks>) {
        unsafe { vkDestroyShaderModule(self, shader_module, pointer_of_option!(allocator_opt)); }
    }

    fn create_pipeline_cache(self, create_info: PipelineCacheCreateInfo, allocator_opt: Option<AllocationCallbacks>) -> Result<PipelineCache, Error> {
        unsafe {
            let mut pipeline_cache: PipelineCache = ptr::null_mut();
            let result = vkCreatePipelineCache(self, &create_info, pointer_of_option!(allocator_opt), &mut pipeline_cache);
            vulkan_result!(result, pipeline_cache)
        }
    }

    fn destroy_pipeline_cache(self, pipeline_cache: PipelineCache, allocator_opt: Option<AllocationCallbacks>) {
        unsafe { vkDestroyPipelineCache(self, pipeline_cache, pointer_of_option!(allocator_opt)); }
    }

    fn merge_pipeline_caches(self, destination_cache: PipelineCache, source_caches: Vec<PipelineCache>) -> Error {
        unsafe {
            vkMergePipelineCaches(self, destination_cache, source_caches.len() as u32, source_caches.as_ptr())
        }
    }

    fn create_graphics_pipelines(self, pipeline_cache: PipelineCache, create_infos: Vec<GraphicsPipelineCreateInfo>, allocator_opt: Option<AllocationCallbacks>) -> Result<Vec<Pipeline>, Error> {
        unsafe {
            let mut pipelines = Vec::with_capacity(create_infos.len());
            pipelines.resize(create_infos.len(), ptr::null_mut() as Pipeline);

            let result = vkCreateGraphicsPipelines(self, pipeline_cache, create_infos.len() as u32, create_infos.as_ptr(), pointer_of_option!(allocator_opt), pipelines.as_mut_ptr());
            vulkan_result!(result, pipelines)
        }
    }

    fn create_compute_pipelines(self, pipeline_cache: PipelineCache, create_infos: Vec<ComputePipelineCreateInfo>, allocator_opt: Option<AllocationCallbacks>) -> Result<Vec<Pipeline>, Error> {
        unsafe {
            let mut pipelines = Vec::with_capacity(create_infos.len());
            pipelines.resize(create_infos.len(), ptr::null_mut() as Pipeline);

            let result = vkCreateComputePipelines(self, pipeline_cache, create_infos.len() as u32, create_infos.as_ptr(), pointer_of_option!(allocator_opt), pipelines.as_mut_ptr());
            vulkan_result!(result, pipelines)
        }
    }

    fn destroy_pipeline(self, pipeline: Pipeline, allocator_opt: Option<AllocationCallbacks>) {
        unsafe { vkDestroyPipeline(self, pipeline, pointer_of_option!(allocator_opt)); }
    }

    fn create_pipeline_layout(self, create_info: PipelineLayoutCreateInfo, allocator_opt: Option<AllocationCallbacks>) -> Result<PipelineLayout, Error> {
        unsafe {
            let mut layout: PipelineLayout = ptr::null_mut();
            let result = vkCreatePipelineLayout(self, &create_info, pointer_of_option!(allocator_opt), &mut layout);
            vulkan_result!(result, layout)
        }
    }

    fn destroy_pipeline_layout(self, pipeline: Pipeline, allocator_opt: Option<AllocationCallbacks>) {
        unsafe { vkDestroyPipelineLayout(self, pipeline, pointer_of_option!(allocator_opt)); }
    }

    fn create_sampler(self, create_info: SamplerCreateInfo, allocator_opt: Option<AllocationCallbacks>) -> Error {
        unsafe { vkCreateSampler(self, &create_info, pointer_of_option!(allocator_opt)) }
    }

    fn destroy_sampler(self, sampler: Sampler, allocator_opt: Option<AllocationCallbacks>) {
        unsafe { vkDestroySampler(self, sampler, pointer_of_option!(allocator_opt)); }
    }

    fn create_descriptor_set_layout(self, create_info: DescriptorSetLayoutCreateInfo, allocator_opt: Option<AllocationCallbacks>) -> Result<DescriptorSetLayout, Error> {
        unsafe {
            let mut layout: DescriptorSetLayout = ptr::null_mut();
            let result = vkCreateDescriptorSetLayout(self, &create_info, pointer_of_option!(allocator_opt), &mut layout);
            vulkan_result!(result, layout)
        }
    }

    fn destroy_descriptor_set_layout(self, layout: DescriptorSetLayout, allocator_opt: Option<AllocationCallbacks>) {
        unsafe { vkDestroyDescriptorSetLayout(self, layout, pointer_of_option!(allocator_opt)); }
    }

    fn create_descriptor_pool(self, create_info: DescriptorPoolCreateInfo, allocator_opt: Option<AllocationCallbacks>) -> Result<DescriptorPool, Error> {
        unsafe {
            let mut pool: DescriptorPool = ptr::null_mut();
            let result = vkCreateDescriptorPool(self, &create_info, pointer_of_option!(allocator_opt), &mut pool);
            vulkan_result!(result, pool)
        }
    }

    fn destroy_descriptor_pool(self, pool: DescriptorPool, allocator_opt: Option<AllocationCallbacks>) {
        unsafe { vkDestroyDescriptorPool(self, pool, pointer_of_option!(allocator_opt)); }
    }

    fn reset_descriptor_pool(self, pool: DescriptorPool, reset_flags: DescriptorPoolResetFlags) -> Error {
        unsafe { vkResetDescriptorPool(self, pool, reset_flags) }
    }

    fn allocate_descriptor_set(self, allocate_info: DescriptorSetAllocateInfo) -> Result<Vec<DescriptorSet>, Error> {
        unsafe {
            let mut descriptor_sets = Vec::with_capacity(allocate_info.descriptorSetCount as usize);
            descriptor_sets.resize(allocate_info.descriptorSetCount as usize, ptr::null_mut());

            let result = vkAllocateDescriptorSets(self, &allocate_info, descriptor_sets.as_mut_ptr());
            vulkan_result!(result, descriptor_sets)
        }
    }

    fn free_descriptor_sets(self, pool: DescriptorPool, descriptor_sets: Vec<DescriptorSet>) -> Error {
        unsafe { vkFreeDescriptorSets(self, pool, descriptor_sets.len() as u32, descriptor_sets.as_ptr()) }
    }

    fn update_descriptor_sets(self, writes: Vec<WriteDescriptorSet>, copies: Vec<CopyDescriptorSet>) {
        unsafe { vkUpdateDescriptorSets(self, writes.len() as u32, writes.as_ptr(), copies.len() as u32, copies.as_ptr()); }
    }

    fn create_framebuffer(self, create_info: FramebufferCreateInfo, allocator_opt: Option<AllocationCallbacks>) -> Result<Framebuffer, Error> {
        unsafe {
            let mut framebuffer: Framebuffer = ptr::null_mut();
            let result = vkCreateFramebuffer(self, &create_info, pointer_of_option!(allocator_opt), &mut framebuffer);
            vulkan_result!(result, framebuffer)
        }
    }

    fn destroy_framebuffer(self, framebuffer: Framebuffer, allocator_opt: Option<AllocationCallbacks>) {
        unsafe { vkDestroyFramebuffer(self, framebuffer, pointer_of_option!(allocator_opt)); }
    }

    fn create_render_pass(self, create_info: RenderPassCreateInfo, allocator_opt: Option<AllocationCallbacks>) -> Result<RenderPass, Error> {
        unsafe {
            let mut render_pass: RenderPass = ptr::null_mut();
            let result = vkCreateRenderPass(self, &create_info, pointer_of_option!(allocator_opt), &mut render_pass);
            vulkan_result!(result, render_pass)
        }
    }

    fn destroy_render_pass(self, render_pass: RenderPass, allocator_opt: Option<AllocationCallbacks>) {
        unsafe { vkDestroyRenderPass(self, render_pass, pointer_of_option!(allocator_opt)); }
    }

    fn get_render_area_granularity(self, render_pass: RenderPass) -> Extent2D {
        unsafe {
            let mut extent: Extent2D = Extent2D{..Default::default()};
            vkGetRenderAreaGranularity(self, render_pass, &mut extent);
            extent
        }
    }

    fn create_command_pool(self, create_info: CommandPoolCreateInfo, allocator_opt: Option<AllocationCallbacks>) -> Result<CommandPool, Error> {
        unsafe {
            let mut pool: CommandPool = ptr::null_mut();
            let result = vkCreateCommandPool(self, &create_info, pointer_of_option!(allocator_opt), &mut pool);
            vulkan_result!(result, pool)
        }
    }

    fn destroy_command_pool(self, pool: CommandPool, allocator_opt: Option<AllocationCallbacks>) {
        unsafe { vkDestroyCommandPool(self, pool, pointer_of_option!(allocator_opt)); }
    }

    fn reset_command_pool(self, pool: CommandPool, reset_flags: CommandPoolResetFlags) -> Error {
        unsafe { vkResetCommandPool(self, pool, reset_flags) }
    }

    fn allocate_command_buffers(self, allocate_info: CommandBufferAllocateInfo) -> Result<Vec<CommandBuffer>, Error> {
        unsafe {
            let mut buffers = Vec::with_capacity(allocate_info.commandBufferCount as usize);
            buffers.resize(allocate_info.commandBufferCount as usize, ptr::null_mut());

            let result = vkAllocateCommandBuffers(self, &allocate_info, buffers.as_mut_ptr());
            vulkan_result!(result, buffers)
        }
    }

    fn free_command_buffers(self, pool: CommandPool, buffers: Vec<CommandBuffer>) {
        unsafe { vkFreeCommandBuffers(self, pool, buffers.len() as u32, buffers.as_ptr()); }
    }
}

pub trait SpockQueue {
    fn submit(self, u32, Vec<SubmitInfo>) -> Error;
    fn wait_idle(self) -> Error;
    fn bind_sparse(self, Vec<BindSparseInfo>, Fence) -> Error;
}

impl SpockQueue for Queue {
    fn submit(self, count: u32, info: Vec<SubmitInfo>) -> Error {
        unsafe { vkQueueSubmit(self, count, info.as_ptr()) }
    }

    fn wait_idle(self) -> Error {
        unsafe { vkQueueWaitIdle(self) }
    }

    fn bind_sparse(self, bind_info: Vec<BindSparseInfo>, fence: Fence) -> Error {
        unsafe { vkQueueBindSparse(self, bind_info.len() as u32, bind_info.as_ptr(), fence) }
    }
}

pub trait SpockCommandBuffer {
    fn begin(self, CommandBufferBeginInfo) -> Error;
    fn end(self) -> Error;
    fn reset(self, CommandBufferResetFlags) -> Error;
    fn cmd_bind_pipeline(self, PipelineBindPoint, Pipeline);
    fn cmd_set_viewport(self, u32, Vec<Viewport>);
    fn cmd_set_scissor(self, u32, Vec<Rect2D>);
    fn cmd_set_line_width(self, f32);
    fn cmd_set_depth_bias(self, f32, f32, f32);
    fn cmd_set_blend_constants(self, [f32; 4]);
    fn cmd_set_depth_bounds(self, f32, f32);
    fn cmd_set_stencil_compare_mask(self, StencilFaceFlags, u32);
    fn cmd_set_stencil_write_mask(self, StencilFaceFlags, u32);
    fn cmd_set_stencil_reference(self, StencilFaceFlags, u32);
    fn cmd_bind_descriptor_sets(self, PipelineBindPoint, PipelineLayout, u32, Vec<DescriptorSet>, Vec<u32>);
    fn cmd_bind_index_buffer(self, Buffer, DeviceSize, IndexType);
    fn cmd_bind_vertex_buffer(self, u32, Vec<Buffer>, Vec<DeviceSize>);
    fn cmd_draw(self, u32, u32, u32, u32);
    fn cmd_draw_indexed(self, u32, u32, u32, i32, u32);
    fn cmd_draw_indirect(self, Buffer, DeviceSize, u32, u32);
    fn cmd_draw_indexed_indirect(self, Buffer, DeviceSize, u32, u32);
    fn cmd_dispatch(self, u32, u32, u32);
    fn cmd_dispatch_indirect(self, Buffer, DeviceSize);
    fn cmd_copy_buffer(self, Buffer, Buffer, Vec<BufferCopy>);
    fn cmd_copy_image(self, Image, ImageLayout, Image, ImageLayout, Vec<ImageCopy>);
    fn cmd_blit_image(self, Image, ImageLayout, Image, ImageLayout, Vec<ImageBlit>, Filter);
    fn cmd_copy_buffer_to_image(self, Buffer, Image, ImageLayout, Vec<BufferImageCopy>);
    fn cmd_copy_image_to_buffer(self, Image, ImageLayout, Buffer, Vec<BufferImageCopy>);
    // cmd_fn update_buffer(self, Buffer, DeviceSize, DeviceSize, ???);
    fn cmd_fill_buffer(self, Buffer, DeviceSize, DeviceSize, u32);
    fn cmd_clear_color_image(self, Image, ImageLayout, ClearColorValue, Vec<ImageSubresourceRange>);
    fn cmd_clear_depth_stencil_image(self, Image, ImageLayout, ClearDepthStencilValue, Vec<ImageSubresourceRange>);
    fn cmd_clear_attachments(self, Vec<ClearAttachment>, Vec<ClearRect>);
    fn cmd_resolve_image(self, Image, ImageLayout, Image, ImageLayout, Vec<ImageResolve>);
    fn cmd_set_event(self, Event, PipelineStageFlags);
    fn cmd_reset_event(self, Event, PipelineStageFlags);
    fn cmd_wait_events(self, Vec<Event>, PipelineStageFlags, PipelineStageFlags, Vec<MemoryBarrier>, Vec<BufferMemoryBarrier>, Vec<ImageMemoryBarrier>);
    fn cmd_pipeline_barrier(self, PipelineStageFlags, PipelineStageFlags, DependencyFlags, Vec<MemoryBarrier>, Vec<BufferMemoryBarrier>, Vec<ImageMemoryBarrier>);
    fn cmd_begin_query(self, QueryPool, u32, QueryControlFlags);
    fn cmd_end_query(self, QueryPool, u32);
    fn cmd_reset_query_pool(self, QueryPool, u32, u32);
    fn cmd_write_timestamp(self, PipelineStageFlags, QueryPool, u32);
    fn cmd_copy_query_pool_errors(self, QueryPool, u32, u32, Buffer, DeviceSize, DeviceSize, QueryErrorFlags);
    // cmd_fn push_constants(self, PipelineLayout, ShaderStageFlags, u32, u32, ???);
    fn cmd_begin_render_pass(self, RenderPassBeginInfo, SubpassContents);
    fn cmd_next_subpass(self, SubpassContents);
    fn cmd_end_render_pass(self);
    fn cmd_execute_commands(self, Vec<CommandBuffer>);
}

impl SpockCommandBuffer for CommandBuffer {
    fn begin(self, info: CommandBufferBeginInfo) -> Error {
        unsafe { vkBeginCommandBuffer(self, &info) }
    }

    fn end(self) -> Error {
        unsafe { vkEndCommandBuffer(self) }
    }

    fn reset(self, flags: CommandBufferResetFlags) -> Error {
        unsafe { vkResetCommandBuffer(self, flags) }
    }

    fn cmd_bind_pipeline(self, bind_point: PipelineBindPoint, pipeline: Pipeline) {
        unsafe { vkCmdBindPipeline(self, bind_point, pipeline); }
    }

    fn cmd_set_viewport(self, first_viewport: u32, viewports: Vec<Viewport>) {
        unsafe { vkCmdSetViewport(self, first_viewport, viewports.len() as u32, viewports.as_ptr()); }
    }

    fn cmd_set_scissor(self, first_scissor: u32, scissors: Vec<Rect2D>) {
        unsafe { vkCmdSetScissor(self, first_scissor, scissors.len() as u32, scissors.as_ptr()); }
    }

    fn cmd_set_line_width(self, width: f32) {
        unsafe { vkCmdSetLineWidth(self, width); }
    }

    fn cmd_set_depth_bias(self, constant_factor: f32, clamp: f32, slope_factor: f32) {
        unsafe { vkCmdSetDepthBias(self, constant_factor, clamp, slope_factor); }
    }

    fn cmd_set_blend_constants(self, constants: [f32; 4]) {
        unsafe { vkCmdSetBlendConstants(self, constants); }
    }

    fn cmd_set_depth_bounds(self, min: f32, max: f32) {
        unsafe { vkCmdSetDepthBounds(self, min, max); }
    }

    fn cmd_set_stencil_compare_mask(self, flags: StencilFaceFlags, compare_mask: u32) {
        unsafe { vkCmdSetStencilCompareMask(self, flags, compare_mask); }
    }

    fn cmd_set_stencil_write_mask(self, flags: StencilFaceFlags, write_mask: u32) {
        unsafe { vkCmdSetStencilWriteMask(self, flags, write_mask); }
    }

    fn cmd_set_stencil_reference(self, flags: StencilFaceFlags, reference: u32) {
        unsafe { vkCmdSetStencilReference(self, flags, reference); }
    }

    fn cmd_bind_descriptor_sets(self, bind_point: PipelineBindPoint, layout: PipelineLayout, first_set: u32, descriptor_sets: Vec<DescriptorSet>, dynamic_offsets: Vec<u32>) {
        unsafe { vkCmdBindDescriptorSets(self, bind_point, layout, first_set, descriptor_sets.len() as u32, descriptor_sets.as_ptr(), dynamic_offsets.len() as u32, dynamic_offsets.as_ptr()); }
    }

    fn cmd_bind_index_buffer(self, buffer: Buffer, offset: DeviceSize, index_type: IndexType) {
        unsafe { vkCmdBindIndexBuffer(self, buffer, offset, index_type); }
    }

    fn cmd_bind_vertex_buffer(self, first_binding: u32, bindings: Vec<Buffer>, offsets: Vec<DeviceSize>) {
        unsafe {
            if bindings.len() != offsets.len() {
                panic!("CommandBuffer.cmd_bind_vertex_buffer must be called with equal length bindings and offsets");
            }

            vkCmdBindVertexBuffer(self, first_binding, bindings.len() as u32, bindings.as_ptr(), offsets.as_ptr());
        }
    }

    fn cmd_draw(self, vertex_count: u32, instance_count: u32, first_vertex: u32, first_instance: u32) {
        unsafe { vkCmdDraw(self, vertex_count, instance_count, first_vertex, first_instance); }
    }

    fn cmd_draw_indexed(self, index_count: u32, instance_count: u32, first_index: u32, vertex_offset: i32, first_instance: u32) {
        unsafe { vkCmdDrawIndexed(self, index_count, instance_count, first_index, vertex_offset, first_instance); }
    }

    fn cmd_draw_indirect(self, buffer: Buffer, offset: DeviceSize, draw_count: u32, stride: u32) {
        unsafe { vkCmdDrawIndirect(self, buffer, offset, draw_count, stride); }
    }

    fn cmd_draw_indexed_indirect(self, buffer: Buffer, offset: DeviceSize, draw_count: u32, stride: u32) {
        unsafe { vkCmdDrawIndexedIndirect(self, buffer, offset, draw_count, stride); }
    }

    fn cmd_dispatch(self, x: u32, y: u32, z: u32) {
        unsafe { vkCmdDispatch(self, x, y, z); }
    }

    fn cmd_dispatch_indirect(self, buffer: Buffer, offset: DeviceSize) {
        unsafe { vkCmdDispatchIndirect(self, buffer, offset); }
    }

    fn cmd_copy_buffer(self, source: Buffer, destination: Buffer, regions: Vec<BufferCopy>) {
        unsafe { vkCmdCopyBuffer(self, source, destination, regions.len() as u32, regions.as_ptr()); }
    }

    fn cmd_copy_image(self, source: Image, source_layout: ImageLayout, destination: Image, destination_layout: ImageLayout, regions: Vec<ImageCopy>) {
        unsafe { vkCmdCopyImage(self, source, source_layout, destination, destination_layout, regions.len() as u32, regions.as_ptr()); }
    }

    fn cmd_blit_image(self, source: Image, source_layout: ImageLayout, destination: Image, destination_layout: ImageLayout, regions: Vec<ImageBlit>, filter: Filter) {
        unsafe { vkCmdBlitImage(self, source, source_layout, destination, destination_layout, regions.len() as u32, regions.as_ptr(), filter); }
    }

    fn cmd_copy_buffer_to_image(self, buffer: Buffer, image: Image, layout: ImageLayout, regions: Vec<BufferImageCopy>) {
        unsafe { vkCmdCopyBufferToImage(self, buffer, image, layout, regions.len() as u32, regions.as_ptr()); }
    }

    fn cmd_copy_image_to_buffer(self, image: Image, layout: ImageLayout, buffer: Buffer, regions: Vec<BufferImageCopy>) {
        unsafe { vkCmdCopyImageToBuffer(self, image, layout, buffer, regions.len() as u32, regions.as_ptr()); }
    }

    fn cmd_fill_buffer(self, buffer: Buffer, offset: DeviceSize, size: DeviceSize, data: u32) {
        unsafe { vkCmdFillBuffer(self, buffer, offset, size, data); }
    }

    fn cmd_clear_color_image(self, image: Image, layout: ImageLayout, color: ClearColorValue, ranges: Vec<ImageSubresourceRange>) {
        unsafe { vkCmdClearColorImage(self, image, layout, &color, ranges.len() as u32, ranges.as_ptr()); }
    }

    fn cmd_clear_depth_stencil_image(self, image: Image, layout: ImageLayout, depth_stencil: ClearDepthStencilValue, ranges: Vec<ImageSubresourceRange>) {
        unsafe { vkCmdClearDepthStencilImage(self, image, layout, &depth_stencil, ranges.len() as u32, ranges.as_ptr()); }
    }

    fn cmd_clear_attachments(self, attachments: Vec<ClearAttachment>, rects: Vec<ClearRect>) {
        unsafe { vkCmdClearAttachments(self, attachments.len() as u32, attachments.as_ptr(), rects.len() as u32, rects.as_ptr()); }
    }

    fn cmd_resolve_image(self, source: Image, source_layout: ImageLayout, destination: Image, destination_layout: ImageLayout, regions: Vec<ImageResolve>) {
        unsafe { vkCmdResolveImage(self, source, source_layout, destination, destination_layout, regions.len() as u32, regions.as_ptr()); }
    }

    fn cmd_set_event(self, event: Event, flags: PipelineStageFlags) {
        unsafe { vkCmdSetEvent(self, event, flags); }
    }

    fn cmd_reset_event(self, event: Event, flags: PipelineStageFlags) {
        unsafe { vkCmdResetEvent(self, event, flags); }
    }

    fn cmd_wait_events(self, events: Vec<Event>, source_mask: PipelineStageFlags, destination_mask: PipelineStageFlags, memory_barriers: Vec<MemoryBarrier>, buffer_memory_barriers: Vec<BufferMemoryBarrier>, image_memory_barriers: Vec<ImageMemoryBarrier>) {
        unsafe { vkCmdWaitEvents(self, events.len() as u32, events.as_ptr(), source_mask, destination_mask, memory_barriers.len() as u32, memory_barriers.as_ptr(), buffer_memory_barriers.len() as u32, buffer_memory_barriers.as_ptr(), image_memory_barriers.len() as u32, image_memory_barriers.as_ptr()); }
    }

    fn cmd_pipeline_barrier(self, source_mask: PipelineStageFlags, destination_mask: PipelineStageFlags, dependency_flags: DependencyFlags, memory_barrier: Vec<MemoryBarrier>, buffer_memory_barrier: Vec<BufferMemoryBarrier>, image_memory_barrier: Vec<ImageMemoryBarrier>) {
        unsafe { vkCmdPipelineBarrier(self, source_mask, destination_mask, dependency_flags, memory_barrier.len() as u32, memory_barrier.as_ptr(), buffer_memory_barrier.len() as u32, buffer_memory_barrier.as_ptr(), image_memory_barrier.len() as u32, image_memory_barrier.as_ptr()); }
    }

    fn cmd_begin_query(self, pool: QueryPool, query: u32, flags: QueryControlFlags) {
        unsafe { vkCmdBeginQuery(self, pool, query, flags); }
    }

    fn cmd_end_query(self, pool: QueryPool, query: u32) {
        unsafe { vkCmdEndQuery(self, pool, query); }
    }

    fn cmd_reset_query_pool(self, pool: QueryPool, first_query: u32, query_count: u32) {
        unsafe { vkCmdResetQueryPool(self, pool, first_query, query_count); }
    }

    fn cmd_write_timestamp(self, stage: PipelineStageFlags, pool: QueryPool, query: u32) {
        unsafe { vkCmdWriteTimestamp(self, stage, pool, query); }
    }

    fn cmd_copy_query_pool_errors(self, pool: QueryPool, first_query: u32, query_offset: u32, buffer: Buffer, offset: DeviceSize, stride: DeviceSize, flags: QueryErrorFlags) {
        unsafe { vkCmdCopyQueryPoolErrors(self, pool, first_query, query_offset, buffer, offset, stride, flags); }
    }

    fn cmd_begin_render_pass(self, info: RenderPassBeginInfo, contents: SubpassContents) {
        unsafe { vkCmdBeginRenderPass(self, &info, contents); }
    }

    fn cmd_next_subpass(self, contents: SubpassContents) {
        unsafe { vkCmdNextSubpass(self, contents); }
    }

    fn cmd_end_render_pass(self) {
        unsafe { vkCmdEndRenderPass(self); }
    }

    fn cmd_execute_commands(self, buffers: Vec<CommandBuffer>) {
        unsafe { vkCmdExecuteCommands(self, buffers.len() as u32, buffers.as_ptr()); }
    }
}

