extern crate libc;
use libc::{uint32_t, uint64_t, int32_t, size_t, wchar_t, c_void, c_float};
use types::*;

#[link(name = "vulkan-1")]
#[allow(non_snake_case)]
#[allow(dead_code)]
extern "C" {
    pub fn vkCreateInstance(pCreateInfo: *const InstanceCreateInfo, pAllocator: *const AllocationCallbacks, pInstance: *mut Instance) -> Error;
    pub fn vkDestroyInstance(instance: Instance, pAllocator: *const AllocationCallbacks) -> c_void;
    pub fn vkEnumeratePhysicalDevices(instance: Instance, pPhysicalDeviceCount: *mut uint32_t, pPhysicalDevices: *mut PhysicalDevice) -> Error;
    pub fn vkGetPhysicalDeviceFeatures(physicalDevice: PhysicalDevice, pFeatures: *mut PhysicalDeviceFeatures) -> c_void;
    pub fn vkGetPhysicalDeviceFormatProperties(physicalDevice: PhysicalDevice, format: Format, pFormatProperties: *mut FormatProperties) -> c_void;
    pub fn vkGetPhysicalDeviceImageFormatProperties(physicalDevice: PhysicalDevice, format: Format, imageType: ImageType, tiling: ImageTiling,
                                                usage: ImageUsageFlags, flags: ImageCreateFlags, pImageFormatProperties: *mut ImageFormatProperties) -> Error;
    pub fn vkGetPhysicalDeviceProperties(physicalDevice: PhysicalDevice, pProperties: *mut PhysicalDeviceProperties) -> c_void;
    pub fn vkGetPhysicalDeviceQueueFamilyProperties(physicalDevice: PhysicalDevice, pQueueFamilyProperyCount: *mut uint32_t, pQueueFamilyProperties: *mut QueueFamilyProperties) -> c_void;
    pub fn vkGetPhysicalDeviceMemoryProperties(physicalDevice: PhysicalDevice, pMemoryProperties: *mut PhysicalDeviceMemoryProperties) -> c_void;
    pub fn vkGetInstanceProcAddr(instance: Instance, pName: *const wchar_t) -> VoidFunction;
    pub fn vkGetDeviceProcAddr(device: Device, pName: *const wchar_t) -> VoidFunction;
    pub fn vkCreateDevice(physicalDevice: PhysicalDevice, pCreateInfo: *const DeviceCreateInfo, pAllocator: *const AllocationCallbacks, pDevice: *mut Device) -> Error;
    pub fn vkDestroyDevice(device: Device, pAllocator: *const AllocationCallbacks) -> c_void;
    pub fn vkEnumerateInstanceExtensionProperties(pLayerName: *const wchar_t, pPropertyCount: *mut uint32_t, pProperties: *mut ExtensionProperties) -> Error;
    pub fn vkEnumerateDeviceExtensionProperties(physicalDevice: PhysicalDevice, pLayerName: *const wchar_t, pPropertyCount: *mut uint32_t, pProperties: *mut ExtensionProperties) -> Error;
    pub fn vkEnumerateInstanceLayerProperties(pPropertyCount: *mut uint32_t, pProperties: *mut LayerProperties) -> Error;
    pub fn vkEnumerateDeviceLayerProperties(physicalDevice: PhysicalDevice, pPropertyCount: *mut uint32_t, pProperties: *mut LayerProperties) -> Error;
    pub fn vkGetDeviceQueue(device: Device, queueFamilyIndex: uint32_t, queueIndex: uint32_t, pQueue: *mut Queue) -> Error;
    pub fn vkQueueSubmit(queue: Queue, submitCount: uint32_t, pSubmits: *const SubmitInfo) -> Error;
    pub fn vkQueueWaitIdle(queue: Queue) -> Error;
    pub fn vkDeviceWaitIdle(device: Device) -> Error;
    pub fn vkAllocateMemory(device: Device, pAllocateInfo: *const MemoryAllocateInfo, pAllocator: *const AllocationCallbacks, pMemory: *mut DeviceMemory) -> Error;
    pub fn vkFreeMemory(device: Device, memory: DeviceMemory, pAllocator: *const AllocationCallbacks) -> c_void;
    pub fn vkMapMemory(device: Device, memory: DeviceMemory, offset: DeviceSize, size: DeviceSize, flags: MemoryMapFlags, ppData: *mut *mut c_void) -> Error;
    pub fn vkUnmapMemory(device: Device, memory: DeviceMemory) -> c_void;
    pub fn vkFlushMappedMemoryRanges(device: Device, memoryRangeCount: uint32_t, pMemoryRanges: *const MappedMemoryRange) -> Error;
    pub fn vkInvalidateMappedMemoryRanges(device: Device, memoryRangeCount: uint32_t, pMemoryRanges: *const MappedMemoryRange) -> Error;
    pub fn vkGetDeviceMemoryCommitment(device: Device, memory: DeviceMemory, pCommittedMemoryInBytes: *mut DeviceSize) -> c_void;
    pub fn vkBindBufferMemory(device: Device, buffer: Buffer, memory: DeviceMemory, memoryOffset: DeviceSize) -> Error;
    pub fn vkBindImageMemory(device: Device, image: Image, memory: DeviceMemory, memoryOffset: DeviceSize) -> Error;
    pub fn vkGetBufferMemoryRequirements(device: Device, buffer: Buffer, pMemoryRequirements: *mut MemoryRequirements) -> c_void;
    pub fn vkGetImageMemoryRequirements(device: Device, image: Image, pMemoryRequirements: *mut MemoryRequirements) -> c_void;
    pub fn vkGetImageSparseMemoryRequirements(device: Device, image: Image, pSparseMemoryRequirementCount: *mut uint32_t, pSparseMemoryRequirements: *mut SparseImageMemoryRequirements) -> c_void;
    pub fn vkGetPhysicalDeviceSparseImageFormatProperties(physicalDevice: PhysicalDevice, format: Format, imageType: ImageType, samples: SampleCountFlags,
                                                      usage: ImageUsageFlags, tiling: ImageTiling, pPropertyCount: *mut uint32_t, pProperties: *mut SparseImageFormatProperties) -> c_void;
    pub fn vkQueueBindSparse(queue: Queue, bindInfoCount: uint32_t, pBindInfo: *const BindSparseInfo, fence: Fence) -> Error;
    pub fn vkCreateFence(device: Device, pCreateInfo: *const FenceCreateInfo, pAllocator: *const AllocationCallbacks, pFence: *mut Fence) -> Error;
    pub fn vkDestroyFence(device: Device, fence: Fence, pAllocator: *const AllocationCallbacks) -> c_void;
    pub fn vkResetFences(device: Device, fenceCount: uint32_t, pFences: *const Fence) -> Error;
    pub fn vkGetFenceStatus(device: Device, fence: Fence) -> Error;
    pub fn vkWaitForFences(device: Device, fenceCount: uint32_t, pFences: *const Fence, waitAll: Bool32, timeout: uint64_t) -> Error;
    pub fn vkCreateSemaphore(device: Device, pCreateInfo: *const SemaphoreCreateInfo, pAllocator: *const AllocationCallbacks, pSemaphore: *mut Semaphore) -> Error;
    pub fn vkDestroySemaphore(device: Device, semaphore: Semaphore, pAllocator: *const AllocationCallbacks) -> c_void;
    pub fn vkCreateEvent(device: Device, pCreateInfo: *const EventCreateInfo, pAllocator: *const AllocationCallbacks, pEvent: *mut Event) -> Error;
    pub fn vkDestroyEvent(device: Device, event: Event, pAllocator: *const AllocationCallbacks) -> c_void;
    pub fn vkGetEventStatus(device: Device, event: Event) -> Error;
    pub fn vkSetEvent(device: Device, event: Event) -> Error;
    pub fn vkResetEvent(device: Device, event: Event) -> Error;
    pub fn vkCreateQueryPool(device: Device, pCreateInfo: *const QueryPoolCreateInfo, pAllocator: *const AllocationCallbacks, pQueryPool: *mut QueryPool) -> Error;
    pub fn vkDestroyQueryPool(device: Device, queryPool: QueryPool, pAllocator: *const AllocationCallbacks) -> c_void;
    pub fn vkGetQueryPoolErrors(device: Device, queryPool: QueryPool, firstQuery: uint32_t, queryCount: uint32_t, dataSize: size_t, pData: *mut c_void,
                             stride: DeviceSize, flags: QueryErrorFlags) -> Error;
    pub fn vkCreateBuffer(device: Device, pCreateInfo: *const BufferCreateInfo, pAllocator: *const AllocationCallbacks, pBuffer: *mut Buffer) -> Error;
    pub fn vkDestroyBuffer(device: Device, buffer: Buffer, pAllocator: *const AllocationCallbacks) -> c_void;
    pub fn vkCreateBufferView(device: Device, pCreateInfo: *const BufferViewCreateInfo, pAllocator: *const AllocationCallbacks, pView: *mut BufferView) -> Error;
    pub fn vkDestroyBufferView(device: Device, bufferView: BufferView, pAllocator: *const AllocationCallbacks) -> c_void;
    pub fn vkCreateImage(device: Device, pCreateInfo: *const ImageCreateInfo, pAllocator: *const AllocationCallbacks, pImage: *mut Image) -> Error;
    pub fn vkDestroyImage(device: Device, image: Image, pAllocator: *const AllocationCallbacks) -> c_void;
    pub fn vkGetImageSubresourceLayout(device: Device, image: Image, pSubresource: *const ImageSubresource, pLayout: *mut SubresourceLayout) -> c_void;
    pub fn vkCreateImageView(device: Device, pCreateInfo: *const ImageViewCreateInfo, pAllocator: *const AllocationCallbacks, pView: *mut ImageView) -> Error;
    pub fn vkDestroyImageView(device: Device, imageView: ImageView, pAllocator: *const AllocationCallbacks) -> c_void;
    pub fn vkCreateShaderModule(device: Device, pCreateInfo: *const ShaderModuleCreateInfo, pAllocator: *const AllocationCallbacks, pShaderModule: *mut ShaderModule) -> Error;
    pub fn vkDestroyShaderModule(device: Device, shaderModule: ShaderModule, pAllocator: *const AllocationCallbacks) -> c_void;
    pub fn vkCreatePipelineCache(device: Device, pCreateInfo: *const PipelineCacheCreateInfo, pAllocator: *const AllocationCallbacks, pPipelineCache: *mut PipelineCache) -> Error;
    pub fn vkDestroyPipelineCache(device: Device, pipelineCache: PipelineCache, pAllocator: *const AllocationCallbacks) -> c_void;
    pub fn vkGetPipelineCacheData(device: Device, pipelineCache: PipelineCache, pDataSize: *mut size_t, pData: *mut c_void) -> Error;
    pub fn vkMergePipelineCaches(device: Device, dstCache: PipelineCache, srcCacheCount: uint32_t, pSrcCaches: *const PipelineCache) -> Error;
    pub fn vkCreateGraphicsPipelines(device: Device, pipelineCache: PipelineCache, createInfoCount: uint32_t, pCreateInfos: *const GraphicsPipelineCreateInfo,
                                 pAllocator: *const AllocationCallbacks, pPipelines: *mut Pipeline) -> Error;
    pub fn vkCreateComputePipelines(device: Device, pipelineCache: PipelineCache, createInfoCount: uint32_t, pCreateInfos: *const ComputePipelineCreateInfo,
                                pAllocator: *const AllocationCallbacks, pPipelines: *mut Pipeline) -> Error;
    pub fn vkDestroyPipeline(device: Device, pipeline: Pipeline, pAllocator: *const AllocationCallbacks) -> c_void;
    pub fn vkCreatePipelineLayout(device: Device, pCreateInfo: *const PipelineLayoutCreateInfo, pAllocator: *const AllocationCallbacks, pPipelineLayout: *mut PipelineLayout) -> Error;
    pub fn vkDestroyPipelineLayout(device: Device, pipelineLayout: PipelineLayout, pAllocator: *const AllocationCallbacks) -> c_void;
    pub fn vkCreateSampler(device: Device, pCreateInfo: *const SamplerCreateInfo, pAllocator: *const AllocationCallbacks) -> Error;
    pub fn vkDestroySampler(device: Device, sampler: Sampler, pAllocator: *const AllocationCallbacks) -> c_void;
    pub fn vkCreateDescriptorSetLayout(device: Device, pCreateInfo: *const DescriptorSetLayoutCreateInfo, pAllocator: *const AllocationCallbacks, pSetLayout: *mut DescriptorSetLayout) -> Error;
    pub fn vkDestroyDescriptorSetLayout(device: Device, descriptorSetLayout: DescriptorSetLayout, pAllocator: *const AllocationCallbacks) -> c_void;
    pub fn vkCreateDescriptorPool(device: Device, pCreateInfo: *const DescriptorPoolCreateInfo, pAllocator: *const AllocationCallbacks, pDescriptorPool: *mut DescriptorPool) -> Error;
    pub fn vkDestroyDescriptorPool(device: Device, descriptorPool: DescriptorPool, pAllocator: *const AllocationCallbacks) -> c_void;
    pub fn vkResetDescriptorPool(device: Device, descriptorPool: DescriptorPool, flags: DescriptorPoolResetFlags) -> Error;
    pub fn vkAllocateDescriptorSets(device: Device, pAllocateInfo: *const DescriptorSetAllocateInfo, pDescriptorSets: *mut DescriptorSet) -> Error;
    pub fn vkFreeDescriptorSets(device: Device, descriptorPool: DescriptorPool, descriptorSetCount: uint32_t, pDescriptorSets: *const DescriptorSet) -> Error;
    pub fn vkUpdateDescriptorSets(device: Device, descriptorWriteCount: uint32_t, pDescriptorWrites: *const WriteDescriptorSet, descriptorCopyCount: uint32_t,
                              pDescriptorCopies: *const CopyDescriptorSet) -> c_void;
    pub fn vkCreateFramebuffer(device: Device, pCreateInfo: *const FramebufferCreateInfo, pAllocator: *const AllocationCallbacks, pFramebuffer: *mut Framebuffer) -> Error;
    pub fn vkDestroyFramebuffer(device: Device, framebuffer: Framebuffer, pAllocator: *const AllocationCallbacks) -> c_void;
    pub fn vkCreateRenderPass(device: Device, pCreateInfo: *const RenderPassCreateInfo, pAllocator: *const AllocationCallbacks, pRenderPass: *mut RenderPass) -> Error;
    pub fn vkDestroyRenderPass(device: Device, renderPass: RenderPass, pAllactore: *const AllocationCallbacks) -> c_void;
    pub fn vkGetRenderAreaGranularity(device: Device, renderPass: RenderPass, pGranularity: *mut Extent2D) -> c_void;
    pub fn vkCreateCommandPool(device: Device, pCreateInfo: *const CommandPoolCreateInfo, pAllocator: *const AllocationCallbacks, pCommandPool: *mut CommandPool) -> Error;
    pub fn vkDestroyCommandPool(device: Device, commandPool: CommandPool, pAllocator: *const AllocationCallbacks) -> c_void;
    pub fn vkResetCommandPool(device: Device, commandPool: CommandPool, flags: CommandPoolResetFlags) -> Error;
    pub fn vkAllocateCommandBuffers(device: Device, pAllocateInfo: *const CommandBufferAllocateInfo, pCommandBuffers: *mut CommandBuffer) -> Error;
    pub fn vkFreeCommandBuffers(device: Device, commandPool: CommandPool, commandBufferCount: uint32_t, pCommandBuffers: *const CommandBuffer) -> c_void;
    pub fn vkBeginCommandBuffer(commandBuffer: CommandBuffer, pBeginInfo: *const CommandBufferBeginInfo) -> Error;
    pub fn vkEndCommandBuffer(commandBuffer: CommandBuffer) -> Error;
    pub fn vkResetCommandBuffer(commandBuffer: CommandBuffer, flags: CommandBufferResetFlags) -> Error;
    pub fn vkCmdBindPipeline(commandBuffer: CommandBuffer, pipelineBindPoint: PipelineBindPoint, pipeline: Pipeline) -> c_void;
    pub fn vkCmdSetViewport(commandBuffer: CommandBuffer, firstViewport: uint32_t, viewportCount: uint32_t, pViewports: *const Viewport) -> c_void;
    pub fn vkCmdSetScissor(commandBuffer: CommandBuffer, firstScissor: uint32_t, scissorCount: uint32_t, pScissors: *const Rect2D) -> c_void;
    pub fn vkCmdSetLineWidth(commandBuffer: CommandBuffer, lineWidth: c_float) -> c_void;
    pub fn vkCmdSetDepthBias(commandBuffer: CommandBuffer, depthBiasConstantFactor: c_float, depthBiasClamp: c_float, depthBiasSlopeFactor: c_float) -> c_void;
    pub fn vkCmdSetBlendConstants(commandBuffer: CommandBuffer, blendConstants: [c_float; 4]) -> c_void;
    pub fn vkCmdSetDepthBounds(commandBuffer: CommandBuffer, minDepthBounds: c_float, maxDepthBounds: c_float) -> c_void;
    pub fn vkCmdSetStencilCompareMask(commandBuffer: CommandBuffer, faceMask: StencilFaceFlags, compareMask: uint32_t) -> c_void;
    pub fn vkCmdSetStencilWriteMask(commandBuffer: CommandBuffer, faceMask: StencilFaceFlags, writeMask: uint32_t) -> c_void;
    pub fn vkCmdSetStencilReference(commandBuffer: CommandBuffer, faceMask: StencilFaceFlags, reference: uint32_t) -> c_void;
    pub fn vkCmdBindDescriptorSets(commandBuffer: CommandBuffer, pipelineBindPoint: PipelineBindPoint, layout: PipelineLayout, firstSet: uint32_t,
                               descriptorSetCount: uint32_t, pDescriptorSets: *const DescriptorSet, dynamicOffsetCount: uint32_t, pDynamicOffsets: *const uint32_t) -> c_void;
    pub fn vkCmdBindIndexBuffer(commandBuffer: CommandBuffer, buffer: Buffer, offset: DeviceSize, indexType: IndexType) -> c_void;
    pub fn vkCmdBindVertexBuffer(commandBuffer: CommandBuffer, firstBinding: uint32_t, bindingCount: uint32_t, pBuffers: *const Buffer, pOffsets: *const DeviceSize) -> c_void;
    pub fn vkCmdDraw(commandBuffer: CommandBuffer, vertexCount: uint32_t, instanceCount: uint32_t, firstVertext: uint32_t, firstInstance: uint32_t) -> c_void;
    pub fn vkCmdDrawIndexed(commandBuffer: CommandBuffer, indexCount: uint32_t, instanceCount: uint32_t, firstIndex: uint32_t, vertexOffset: int32_t, firstInstance: uint32_t) -> c_void;
    pub fn vkCmdDrawIndirect(commandBuffer: CommandBuffer, buffer: Buffer, offset: DeviceSize, drawCount: uint32_t, stride: uint32_t) -> c_void;
    pub fn vkCmdDrawIndexedIndirect(commandBuffer: CommandBuffer, buffer: Buffer, offset: DeviceSize, drawCount: uint32_t, stride: uint32_t) -> c_void;
    pub fn vkCmdDispatch(commandBuffer: CommandBuffer, x: uint32_t, y: uint32_t, z: uint32_t) -> c_void;
    pub fn vkCmdDispatchIndirect(commandBuffer: CommandBuffer, buffer: Buffer, offset: DeviceSize) -> c_void;
    pub fn vkCmdCopyBuffer(commandBuffer: CommandBuffer, srcBuffer: Buffer, dstBuffer: Buffer, regionCount: uint32_t, pRegions: *const BufferCopy) -> c_void;
    pub fn vkCmdCopyImage(commandBuffer: CommandBuffer, srcImage: Image, srcImageLayout: ImageLayout, dstImage: Image, dstImageLayout: ImageLayout,
                      regionCount: uint32_t, pRegions: *const ImageCopy) -> c_void;
    pub fn vkCmdBlitImage(commandBuffer: CommandBuffer, srcImage: Image, srcImageLayout: ImageLayout, dstImage: Image, dstImageLayout: ImageLayout,
                      regionCount: uint32_t, pRegions: *const ImageBlit, filter: Filter) -> c_void;
    pub fn vkCmdCopyBufferToImage(commandBuffer: CommandBuffer, srcBuffer: Buffer, dstImage: Image, dstImageLayout: ImageLayout, regionCount: uint32_t,
                              pRegions: *const BufferImageCopy) -> c_void;
    pub fn vkCmdCopyImageToBuffer(commandBuffer: CommandBuffer, srcImage: Image, srcImageLayout: ImageLayout, dstBuffer: Buffer, regionCount: uint32_t,
                              pRegions: *const BufferImageCopy) -> c_void;
    pub fn vkCmdUpdateBuffer(commandBuffer: CommandBuffer, dstBuffer: Buffer, dstOffset: DeviceSize, dataSize: DeviceSize, pData: *const uint32_t) -> c_void;
    pub fn vkCmdFillBuffer(commandBuffer: CommandBuffer, dstBuffer: Buffer, dstOffset: DeviceSize, size: DeviceSize, data: uint32_t) -> c_void;
    pub fn vkCmdClearColorImage(commandBuffer: CommandBuffer, image: Image, imageLayout: ImageLayout, pColor: *const ClearColorValue, rangeCount: uint32_t,
                            pRanges: *const ImageSubresourceRange) -> c_void;
    pub fn vkCmdClearDepthStencilImage(commandBuffer: CommandBuffer, image: Image, imageLayout: ImageLayout, pDepthStencil: *const ClearDepthStencilValue,
                                   rangeCount: uint32_t, pRanges: *const ImageSubresourceRange) -> c_void;
    pub fn vkCmdClearAttachments(commandBuffer: CommandBuffer, attachmentCount: uint32_t, pAttachments: *const ClearAttachment, rectCount: uint32_t, pRects: *const ClearRect) -> c_void;
    pub fn vkCmdResolveImage(commandBuffer: CommandBuffer, srcImage: Image, srcImageLayout: ImageLayout, dstImage: Image, dstImageLayout: ImageLayout,
                         regionCount: uint32_t, pRegions: *const ImageResolve) -> c_void;
    pub fn vkCmdSetEvent(commandBuffer: CommandBuffer, event: Event, stageMask: PipelineStageFlags) -> c_void;
    pub fn vkCmdResetEvent(commandBuffer: CommandBuffer, event: Event, stageMask: PipelineStageFlags) -> c_void;
    pub fn vkCmdWaitEvents(commandBuffer: CommandBuffer, eventCount: uint32_t, pEvents: *const Event, srcStageMask: PipelineStageFlags, dstStageMask: PipelineStageFlags,
                       memoryBarrierCount: uint32_t, pMemoryBarriers: *const MemoryBarrier, bufferMemoryBarrierCount: uint32_t, pMemoryBarriers: *const BufferMemoryBarrier,
                       imageMemoryBarrierCount: uint32_t, pImageMemoryBarriers: *const ImageMemoryBarrier) -> c_void;
    pub fn vkCmdPipelineBarrier(commandBuffer: CommandBuffer, srcStageMask: PipelineStageFlags, dstStageMask: PipelineStageFlags, dependencyFlags: DependencyFlags,
                            memoryBarrierCount: uint32_t, pMemoryBarriers: *const MemoryBarrier, bufferMemoryBarrierCount: uint32_t,
                            pBufferMemoryBarriers: *const BufferMemoryBarrier, imageMemoryBarrierCount: uint32_t, pImageMemoryBarriers: *const ImageMemoryBarrier) -> c_void;
    pub fn vkCmdBeginQuery(commandBuffer: CommandBuffer, queryPool: QueryPool, query: uint32_t, flags: QueryControlFlags) -> c_void;
    pub fn vkCmdEndQuery(commandBuffer: CommandBuffer, queryPool: QueryPool, query: uint32_t) -> c_void;
    pub fn vkCmdResetQueryPool(commandBuffer: CommandBuffer, queryPool: QueryPool, firstQuery: uint32_t, queryCount: uint32_t) -> c_void;
    pub fn vkCmdWriteTimestamp(commandBuffer: CommandBuffer, pipelineStage: PipelineStageFlags, queryPool: QueryPool, query: uint32_t) -> c_void;
    pub fn vkCmdCopyQueryPoolErrors(commandBuffer: CommandBuffer, queryPool: QueryPool, firstQuery: uint32_t, queryCount: uint32_t, dstBuffer: Buffer,
                                 dstOffset: DeviceSize, stride: DeviceSize, flags: QueryErrorFlags) -> c_void;
    pub fn vkCmdPushConstants(commandBuffer: CommandBuffer, layout: PipelineLayout, stageFlags: ShaderStageFlags, offset: uint32_t, size: uint32_t, pValues: *const c_void) -> c_void;
    pub fn vkCmdBeginRenderPass(commandBuffer: CommandBuffer, pRenderPassBegin: *const RenderPassBeginInfo, contents: SubpassContents) -> c_void;
    pub fn vkCmdNextSubpass(commandBuffer: CommandBuffer, contents: SubpassContents) -> c_void;
    pub fn vkCmdEndRenderPass(commandBuffer: CommandBuffer) -> c_void;
    pub fn vkCmdExecuteCommands(commandBuffer: CommandBuffer, commandBufferCount: uint32_t, pCommandBuffers: *const CommandBuffer) -> c_void;
}
