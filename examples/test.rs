extern crate spock;

use spock::*;
use spock::types::*;
use std::ffi::CString;

fn main() {
    let application_info = ApplicationInfo {
        pApplicationName: CString::new("Testing").unwrap().as_ptr(),
        pEngineName:      CString::new("Test").unwrap().as_ptr(),
        ..Default::default()
    };

    let instance_create_info = InstanceCreateInfo {
        pApplicationInfo: &application_info,
        ..Default::default()
    };

    let instance: Instance = create_instance(instance_create_info, None).unwrap();
    let physical_device: PhyiscalDevice = instance.enumerate_all_physical_devices().unwrap()[0];
    let queue_properties: Vec<QueueFamilyProperties> = physical_device.get_all_queue_family_properties();

    let graphics_queue_index = queue_properties.iter().position(|&p| (p.queueFlags as u32 & QueueFlags::Graphics as u32) > 0).unwrap();

    let queue_create_info = DeviceQueueCreateInfo {
        queueFamilyIndex: graphics_queue_index as u32,
        queueCount: 1,
        ..Default::default()
    };

    let device_create_info = DeviceCreateInfo {
        queueCreateInfoCount: 1,
        pQueueCreateInfos: &queue_create_info,
        ..Default::default()
    };

    let device: Device = physical_device.create_device(device_create_info, None).unwrap();

    let create_command_pool_info = CommandPoolCreateInfo {
        flags: CommandPoolCreateFlags::ResetCommandBuffer,
        queueFamilyIndex: swapchain.queue_node_index,
        ..Default::default()
    };

    let command_pool = device.create_command_pool(create_command_pool_info, None).unwrap();

    let single_command_buffer_allocate_info = CommandBufferAllocateInfo {
        commandPool: command_pool,
        level: CommandBufferLevel::Primary,
        commandBufferCount: 1,
        ..Default::default()
    };

    let image_command_buffer_allocate_info = CommandBufferAllocateInfo {
        commandPool: command_pool,
        level: CommandBufferLevel::Primary,
        commandBufferCount: swapchain.image_count,
        ..Default::default()
    };

    let setup_command_buffer: CommandBuffer = device.allocate_command_buffers(single_command_buffer_allocate_info).unwrap()[0];
    let begin_command_buffer_info = CommandBufferBeginInfo{..Default::default()};
    setup_command_buffer.begin(begin_command_buffer_info);

    let draw_command_buffers = device.allocate_command_buffers(image_command_buffer_allocate_info).unwrap();
    let post_present_command_buffer = device.allocate_command_buffers(single_command_buffer_allocate_info).unwrap()[0];

    let create_depth_stencil_info = ImageCreateInfo {
        imageType: ImageType::Type2D,
        format: depth_format,
        extent: Extend3D {
            width: width,
            height: heightSampleCountFlags,
            depth: 1
        },
        mipLevels: 1,
        arrayLayers: 1,
        samples: SampleCountFlags::Count1,
        tiling: ImageTiling::Optimal,
        usage: ImageUsageFlags::DepthStencilAttachment | ImageUsageFlags::TransferSrc,
        sharingMode: SharingMode::Exclusive,
        ..Default::default()
    };

    let depth_stencil_image = device.create_image(create_depth_stencil_info, None).unwrap();

    let attachment_descriptions = vec!(
        AttachmentDescription {
            format: Format::B8G8R8A8Unorm,
            samples: SampleCountFlags::Count1,
            loadOp: AttachmentLoadOp::Clear,
            storeOp: AttachmentStoreOp::Store,
            stencilLoadOp: AttachmentLoadOp::DontCare,
            stencilStoreOp: AttachmentStoreOp::DontCare,
            initialLayout: ImageLayout::ColorAttachmentOptimal,
            finalLayout: ImageLayout::ColorAttachmentOptimal,
            ..Default::default()
        },
        AttachmentDescription {
            format: depth_format,
            samples: SampleCountFlags::Count1,
            loadOp: AttachmentLoadOp::Clear,
            storeOp: AttachmentStoreOp::Store,
            stencilLoadOp: AttachmentLoadOp::DontCare,
            stencilStoreOp: AttachmentStoreOp::DontCare,
            initialLayout: ImageLayout::ColorAttachmentOptimal,
            finalLayout: ImageLayout::ColorAttachmentOptimal,
            ..Default::default()
        }
    );

    let subpass_color_attachment_reference = AttachmentReference {
        attachment: 0,
        layout: ImageLayout::ColorAttachmentOptimal
    };

    let subpass_depth_stencil_attachment_reference = AttachmentReference {
        attachment: 1,
        layout: ImageLayout::DepthStencilAttachmentOptimal
    };

    let subpass_description = SubpassDescription {
        pipelineBindPoint: PipelineBindPoint::Graphics,
        colorAttachmentCount: 1,
        pColorAttachments: &subpass_color_attachment_reference,
        pDepthStencilAttachment: &subpass_depth_stencil_attachment_reference,
        ..Default::default()
    };

    let render_pass_create_info = RenderPassCreateInfo {
        attachmentCount: 2,
        pAttachments: attachment_descriptions.as_ptr(),
        subpassCount: 1,
        pSubpasses: &subpass_description,
        ..Default::default()
    };

    let render_pass = device.create_render_pass(render_pass_create_info, None).unwrap();

    const triangle_vertices: [[[f32; 3]; 2]; 3] = [
          [ [  1.0,  1.0,  0.0 ], [  1.0,  0.0,  0.0 ] ],
          [ [ -1.0,  1.0,  0.0 ], [  0.0,  1.0,  0.0 ] ],
          [ [  0.0, -1.0,  0.0 ], [  0.0,  0.0,  1.0 ] ]
    ];

    const triangle_indices: [u32; 3] = [ 0, 1, 2 ];

    println!("success!");

    instance.destroy(None);
}
