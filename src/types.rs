extern crate libc;
use libc::{uint8_t, uint32_t, uint64_t, int32_t, size_t, c_void, c_float, c_char};
use std::default::Default;
use std::fmt;
use std::ptr;

// TODO: figure out some way to link these values
const VK_MAX_PHYSICAL_DEVICE_NAME_SIZE: usize = 256;
const VK_UUID_SIZE: usize = 16;
const VK_MAX_MEMORY_TYPES: usize = 32;
const VK_MAX_MEMORY_HEAPS: usize = 16;
const VK_MAX_EXTENSION_NAME_SIZE: usize = 256;
const VK_MAX_DESCRIPTION_SIZE: usize = 256;


pub type DeviceSize = uint64_t;
pub type SampleMask = uint32_t;

pub type Bool32 = uint32_t;

pub type Instance = *mut c_void;
pub type PhysicalDevice = *mut c_void;
pub type Device = *mut c_void;
pub type Queue = *mut c_void;
pub type Semaphore = *mut c_void;
pub type CommandBuffer = *mut c_void;
pub type Fence = *mut c_void;
pub type DeviceMemory = *mut c_void;
pub type Buffer = *mut c_void;
pub type Image = *mut c_void;
pub type Event = *mut c_void;
pub type QueryPool = *mut c_void;
pub type BufferView = *mut c_void;
pub type ImageView = *mut c_void;
pub type ShaderModule = *mut c_void;
pub type PipelineCache = *mut c_void;
pub type PipelineLayout = *mut c_void;
pub type RenderPass = *mut c_void;
pub type Pipeline = *mut c_void;
pub type DescriptorSetLayout = *mut c_void;
pub type Sampler = *mut c_void;
pub type DescriptorPool = *mut c_void;
pub type DescriptorSet = *mut c_void;
pub type Framebuffer = *mut c_void;
pub type CommandPool = *mut c_void;
pub type AllocationFunction = extern fn(*mut c_void, size_t, size_t, SystemAllocationScope) -> *mut c_void;
pub type ReallocationFunction = extern fn(*mut c_void, *mut c_void, size_t, size_t, SystemAllocationScope) -> *mut c_void;
pub type FreeFunction = extern fn(*mut c_void, *mut c_void) -> c_void;
pub type InternalAllocationNotificationFunction = extern fn(*mut c_void, size_t, InternalAllocationType, SystemAllocationScope) -> c_void;
pub type InternalFreeNotificationFunction = extern fn(*mut c_void, size_t, InternalAllocationType, SystemAllocationScope) -> c_void;
pub type VoidFunction = extern fn() -> c_void;


#[repr(C)]
#[derive(Copy, Clone)]
pub enum Error {
    Success = 0,
    NotReady = 1,
    Timeout = 2,
    EventSet = 3,
    EventReset = 4,
    Incomplete = 5,
    SuboptimalKHR = 1_000_001_003,
    ErrorOutOfHostMemory = -1,
    ErrorOutOfDeviceMemory = -2,
    ErrorInitializationFailed = -3,
    ErrorDeviceLost = -4,
    ErrorMemoryMapFailed = -5,
    ErrorLayerNotPresent = -6,
    ErrorExtensionNotPresent = -7,
    ErrorFeatureNotPresent = -8,
    ErrorIncompatibleDriver = -9,
    ErrorTooManyObjects = -10,
    ErrorFormatNotSupported = -11,
    ErrorSurfaceLostKHR = -1_000_000_000,
    ErrorNativeWindowInUseKHR = -1_000_000_001,
    ErrorOutOfDateKHR = -1_000_001_004,
    ErrorIncompatibleDisplayKHR = -1_000_003_001,
    ErrorValidationFailedExt = -1_000_011_001
}

impl ToString for Error {
    fn to_string(&self) -> String {
        match *self {
            Error::Success                     => "Success".to_string(),
            Error::NotReady                    => "Not Ready".to_string(),
            Error::Timeout                     => "Timeout".to_string(),
            Error::EventSet                    => "Event Set".to_string(),
            Error::EventReset                  => "Event Reset".to_string(),
            Error::Incomplete                  => "Incomplete".to_string(),
            Error::SuboptimalKHR               => "Suboptimal KHR".to_string(),
            Error::ErrorOutOfHostMemory        => "Error (Out of Host Memory".to_string(),
            Error::ErrorOutOfDeviceMemory      => "Error (Out of Device Memory)".to_string(),
            Error::ErrorInitializationFailed   => "Error (Intialization Failed)".to_string(),
            Error::ErrorDeviceLost             => "Error (Device Lost)".to_string(),
            Error::ErrorMemoryMapFailed        => "Error (Memory Map Failed)".to_string(),
            Error::ErrorLayerNotPresent        => "Error (Layer Not Present)".to_string(),
            Error::ErrorExtensionNotPresent    => "Error (Extension Not Present)".to_string(),
            Error::ErrorFeatureNotPresent      => "Error (Feature Not Present)".to_string(),
            Error::ErrorIncompatibleDriver     => "Error (Incompatible Driver)".to_string(),
            Error::ErrorTooManyObjects         => "Error (Too Many Objects)".to_string(),
            Error::ErrorFormatNotSupported     => "Error (Format Not Supported)".to_string(),
            Error::ErrorSurfaceLostKHR         => "Error (Surface Lost KHR)".to_string(),
            Error::ErrorNativeWindowInUseKHR   => "Error (Native Window in Use KHR)".to_string(),
            Error::ErrorOutOfDateKHR           => "Error (Out of Date KHR)".to_string(),
            Error::ErrorIncompatibleDisplayKHR => "Error (Incompaitble Display KHR)".to_string(),
            Error::ErrorValidationFailedExt    => "Error (Validation Vailed Ext)".to_string()
        }
    }
}

impl fmt::Debug for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub enum StructureType {
    ApplicationInfo = 0,
    InstanceCreateInfo = 1,
    DeviceQueueCreateInfo = 2,
    DeviceCreateInfo = 3,
    SubmitInfo = 4,
    MemoryAllocateInfo = 5,
    MappedMemoryRange = 6,
    BindSparseInfo = 7,
    FenceCreateInfo = 8,
    SemaphoreCreateInfo = 9,
    EventCreateInfo = 10,
    QueryPoolCreateInfo = 11,
    BufferCreateInfo = 12,
    BufferViewCreateInfo = 13,
    ImageCreateInfo = 14,
    ImageViewCreateInfo = 15,
    ShaderModuleCreateInfo = 16,
    PipelineCacheCreateInfo = 17,
    PipelineShaderStageCreateInfo = 18,
    PipelineVertexInputStateCreateInfo = 19,
    PipelineInputAssemblyStateCreateInfo = 20,
    PipelineTessellationStateCreateInfo = 21,
    PipelineViewportStateCreateInfo = 22,
    PipelineRasterizationStateCreateInfo = 23,
    PipelineMultisampleStateCreateInfo = 24,
    PipelineDepthStencilStateCreateInfo = 25,
    PipelineColorBlendStateCreateInfo = 26,
    PipelineDynamicStateCreateInfo = 27,
    GraphicsPipelineCreateInfo = 28,
    ComputePipelineCreateInfo = 29,
    PipelineLayoutCreateInfo = 30,
    SamplerCreateInfo = 31,
    DescriptorSetLayoutCreateInfo = 32,
    DescriptorPoolCreateInfo = 33,
    DescriptorSetAllocateInfo = 34,
    WriteDescriptorSet = 35,
    CopyDescriptorSet = 36,
    FramebufferCreateInfo = 37,
    RenderPassCreateInfo = 38,
    CommandPoolCreateInfo = 39,
    CommandBufferAllocateInfo = 40,
    CommandBufferInheritanceInfo = 41,
    CommandBufferBeginInfo = 42,
    RenderPassBeginInfo = 43,
    BufferMemoryBarrier = 44,
    ImageMemoryBarrier = 45,
    MemoryBarrier = 46,
    LoaderInstanceCreateInfo = 47,
    LoaderDeviceCreateInfo = 48,
    SwapchainCreateInfoKHR = 1_000_001_000,
    PresentInfoKHR = 1_000_001_001,
    DisplayModeCreateInfoKHR = 1_000_002_000,
    DisplaySurfaceCreateInfoKHR = 1_000_002_001,
    DisplayPresentInfoKHR = 1_000_003_000,
    XlibSurfaceCreateInfoKHR = 1_000_004_000,
    XcbSurfaceCreateInfoKHR = 1_000_005_000,
    WaylandSurfaceCreateInfoKHR = 1_000_006_000,
    MirSurfaceCreateInfoKHR = 1_000_007_000,
    AndroidSurfaceCreateInfoKHR = 1_000_008_000,
    Win32SurfaceCreateInfoKHR = 1_000_009_000,
    DebugReportCreateInfoExt = 1_000_011_000,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub enum SystemAllocationScope {
    Command = 0,
    Object = 1,
    Cache = 2,
    Device = 3,
    Instance = 4
}

#[repr(C)]
#[derive(Copy, Clone)]
pub enum InternalAllocationType {
    Executable = 0,
    Dummy = 1 // Dummy workaround for E0083
}

#[repr(C)]
#[derive(Copy, Clone)]
pub enum Format {
    Undefined = 0,
    R4G4UnormPacks = 1,
    R4G4B4A4UnormPack16 = 2,
    B4G4R4A4UnormPack16 = 3,
    R5G6B5UnormPack16 = 4,
    B5G6R5UnormPack16 = 5,
    R5G5B5A1UnormPack16 = 6,
    B5G5R5A1UnormPack16 = 7,
    A1R5G5B5UnormPack16 = 8,
    R8Unorm = 9,
    R8Snorm = 10,
    R8UscalED = 11,
    R8SscalED = 12,
    R8Uint = 13,
    R8Sint = 14,
    R8Srgb = 15,
    R8G8Unorm = 16,
    R8G8Snorm = 17,
    R8G8UscalED = 18,
    R8G8SSCALED = 19,
    R8G8Uint = 20,
    R8G8Sint = 21,
    R8G8SRGB = 22,
    R8G8B8Unorm = 23,
    R8G8B8Snorm = 24,
    R8G8B8Uscaled = 25,
    R8G8B8Sscaled = 26,
    R8G8B8Uint = 27,
    R8G8B8Sint = 28,
    R8G8B8SRGB = 29,
    B8G8R8Unorm = 30,
    B8G8R8Snorm = 31,
    B8G8R8Uscaled = 32,
    B8G8R8Sscaled = 33,
    B8G8R8Uint = 34,
    B8G8R8Sint = 35,
    B8G8R8SRGB = 36,
    R8G8B8A8Unorm = 37,
    R8G8B8A8Snorm = 38,
    R8G8B8A8Uscaled = 39,
    R8G8B8A8Sscaled = 40,
    R8G8B8A8Uint = 41,
    R8G8B8A8Sint = 42,
    R8G8B8A8SRGB = 43,
    B8G8R8A8Unorm = 44,
    B8G8R8A8Snorm = 45,
    B8G8R8A8Uscaled = 46,
    B8G8R8A8Sscaled = 47,
    B8G8R8A8Uint = 48,
    B8G8R8A8Sint = 49,
    B8G8R8A8SRGB = 50,
    A8B8G8R8UnormPack32 = 51,
    A8B8G8R8SnormPack32 = 52,
    A8B8G8R8UscaledPacK32 = 53,
    A8B8G8R8SscaledPacK32 = 54,
    A8B8G8R8UintPack32 = 55,
    A8B8G8R8SintPack32 = 56,
    A8B8G8R8SRGBPACK32 = 57,
    A2R10G10B10UnormPack32 = 58,
    A2R10G10B10SnormPack32 = 59,
    A2R10G10B10UscaledPack32 = 60,
    A2R10G10B10SscaledPack32 = 61,
    A2R10G10B10UintPack32 = 62,
    A2R10G10B10SintPack32 = 63,
    A2B10G10R10UnormPack32 = 64,
    A2B10G10R10SnormPack32 = 65,
    A2B10G10R10UscaledPacK32 = 66,
    A2B10G10R10SscaledPacK32 = 67,
    A2B10G10R10UintPack32 = 68,
    A2B10G10R10SintPack32 = 69,
    R16Unorm = 70,
    R16Snorm = 71,
    R16Uscaled = 72,
    R16Sscaled = 73,
    R16Uint = 74,
    R16Sint = 75,
    R16Sfloat = 76,
    R16G16Unorm = 77,
    R16G16Snorm = 78,
    R16G16Uscaled = 79,
    R16G16Sscaled = 80,
    R16G16Uint = 81,
    R16G16Sint = 82,
    R16G16Sfloat = 83,
    R16G16B16Unorm = 84,
    R16G16B16Snorm = 85,
    R16G16B16Uscaled = 86,
    R16G16B16Sscaled = 87,
    R16G16B16Uint = 88,
    R16G16B16Sint = 89,
    R16G16B16Sfloat = 90,
    R16G16B16A16Unorm = 91,
    R16G16B16A16Snorm = 92,
    R16G16B16A16Uscaled = 93,
    R16G16B16A16Sscaled = 94,
    R16G16B16A16Uint = 95,
    R16G16B16A16Sint = 96,
    R16G16B16A16Sfloat = 97,
    R32Uint = 98,
    R32Sint = 99,
    R32Sfloat = 100,
    R32G32Uint = 101,
    R32G32Sint = 102,
    R32G32Sfloat = 103,
    R32G32B32Uint = 104,
    R32G32B32Sint = 105,
    R32G32B32Sfloat = 106,
    R32G32B32A32Uint = 107,
    R32G32B32A32Sint = 108,
    R32G32B32A32Sfloat = 109,
    R64Uint = 110,
    R64Sint = 111,
    R64Sfloat = 112,
    R64G64Uint = 113,
    R64G64Sint = 114,
    R64G64Sfloat = 115,
    R64G64B64Uint = 116,
    R64G64B64Sint = 117,
    R64G64B64Sfloat = 118,
    R64G64B64A64Uint = 119,
    R64G64B64A64Sint = 120,
    R64G64B64A64Sfloat = 121,
    B10G11R11UfloatPack32 = 122,
    E5B9G9R9UfloatPack32 = 123,
    D16Unorm = 124,
    X8D24UnormPack32 = 125,
    D32Sfloat = 126,
    S8Uint = 127,
    D16UnormS8Uint = 128,
    D24UnormS8Uint = 129,
    D32SfloatS8Uint = 130,
    BC1RGBUnormBlock = 131,
    BC1RGBSRGBBlock = 132,
    BC1RGBAUnormBlock = 133,
    BC1RGBASRGBBlock = 134,
    BC2UnormBlock = 135,
    BC2SRGBBlock = 136,
    BC3UnormBlock = 137,
    BC3SRGBBlock = 138,
    BC4UnormBlock = 139,
    BC4SnormBlock = 140,
    BC5UnormBlock = 141,
    BC5SnormBlock = 142,
    BC6HUfloatBlock = 143,
    BC6HSfloatBlock = 144,
    BC7UnormBlock = 145,
    BC7RGBBlock = 146,
    ETC2R8G8B8UnormBlock = 147,
    ETC2R8G8B8SRGBBlock = 148,
    ETC2R8G8B8A1UnormBlock = 149,
    ETC2R8G8B8A1SRGBBlock = 150,
    ETC2R8G8B8A8UnormBlock = 151,
    ETC2R8G8B8A8SRGBBLock = 152,
    EACR11UnormBlock = 153,
    EACR11SnormBlock = 154,
    EACR11G11UnormBlock = 155,
    EACR11G11SnormBlock = 156,
    ASTC4x4UnormBlock = 157,
    ASTC4x4SRGBblock = 158,
    ASTC5x4UnormBlock = 159,
    ASTC5x4SRGBBlock = 160,
    ASTC5x5UnormBlock = 161,
    ASTC5x5SRGBBlock = 162,
    ASTC6x5UnormBlock = 163,
    ASTC6x5SRGBBlock = 164,
    ASTC6x6UnormBlock = 165,
    ASTC6x6SRGBBlock = 166,
    ASTC8x5UnormBlock = 167,
    ASTC8x5SRGBBlock = 168,
    ASTC8x6UnormBlock = 169,
    ASTC8x6SRGBBlock = 170,
    ASTC8x8UnormBlock = 171,
    ASTC8x8SRGBBlock = 172,
    ASTC10x5UnormBlock = 173,
    ASTC10x5SRGBBlock = 174,
    ASTC10x6UnormBlock = 175,
    ASTC10x6SRGBBlock = 176,
    ASTC10x8UnormBlock = 177,
    ASTC10x8SRGBBlock = 178,
    ASTC10x10UnormBlock = 179,
    ASTC10x10SRGBBlock = 180,
    ASTC12x10UnormBlock = 181,
    ASTC12x10SRGBBlock = 182,
    ASTC12x12UnormBlock = 183,
    ASTC12x12SRGBBlock = 184
}

#[repr(C)]
#[derive(Copy, Clone)]
pub enum ImageType {
    Type1D = 0,
    Type2D = 1,
    Type3D = 2
}

#[repr(C)]
#[derive(Copy, Clone)]
pub enum ImageTiling {
    Optimal = 0,
    Linear = 1
}

#[repr(C)]
#[derive(Copy, Clone)]
pub enum PhysicalDeviceType {
    Other = 0,
    IntegratedGPU = 1,
    DiscreteGPU = 2,
    VirtualGPU = 3,
    CPU = 4,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub enum QueryType {
    Occlusion = 0,
    PipelineStatistics = 1,
    Timestamp = 2
}

#[repr(C)]
#[derive(Copy, Clone)]
pub enum SharingMode {
    Exclusive = 0,
    Concurrent = 1
}

#[repr(C)]
#[derive(Copy, Clone)]
pub enum ImageLayout {
    Undefined = 0,
    General = 1,
    ColorAttachmentOptimal = 2,
    DepthStencilAttachmentOptimal = 3,
    DepthStencilReadOnlyOptimal = 4,
    ShaderReadOnly = 5,
    TransferSrcOptimal = 6,
    TransferDstOptimal = 7,
    Preinitialized = 8,
    PresentSrcKHR = 1_000_001_002
}

#[repr(C)]
#[derive(Copy, Clone)]
pub enum ImageViewType {
    Type1D = 0,
    Type2D = 1,
    Type3D = 2,
    TypeCube = 3,
    Type1DArray = 4,
    Type2DArray = 5,
    TypeCubeArray = 6
}

#[repr(C)]
#[derive(Copy, Clone)]
pub enum ComponentSwizzle {
    Identity = 0,
    Zero = 1,
    One = 2,
    R = 3,
    G = 4,
    B = 5,
    A = 6
}

#[repr(C)]
#[derive(Copy, Clone)]
pub enum VertexInputRate {
    Vertex = 0,
    Instance = 1
}

#[repr(C)]
#[derive(Copy, Clone)]
pub enum PrimitiveTopology {
    PointList = 0,
    LineList = 1,
    LineStrip = 2,
    TriangleList = 3,
    TriangleStrip = 4,
    TriangleFan = 5,
    LineListWithAdjacency = 6,
    LineStripWithAdjacency = 7,
    TriangleListWithAdjacency = 8,
    TriangleStripWithAdjacency = 9,
    PatchList = 10
}

#[repr(C)]
#[derive(Copy, Clone)]
pub enum PolygonMode {
    Fill = 0,
    Line = 1,
    Point = 2
}

#[repr(C)]
#[derive(Copy, Clone)]
pub enum FrontFace {
    CounterClockwise = 0,
    Clockwise = 1
}

#[repr(C)]
#[derive(Copy, Clone)]
pub enum CompareOp {
    Never = 0,
    Less = 1,
    Equal = 2,
    LessOrEqual = 3,
    Greater = 4,
    NotEqual = 5,
    GreaterOrEqual = 6,
    Always = 7
}

#[repr(C)]
#[derive(Copy, Clone)]
pub enum StencilOp {
    Keep = 0,
    Zero = 1,
    Replace = 2,
    IncrementAndClamp = 3,
    DecrementAndClamp = 4,
    Invert = 5,
    IncrementAndWrap = 6,
    DecrementAndWrap = 7
}

#[repr(C)]
#[derive(Copy, Clone)]
pub enum LogicOp {
    Clear = 0,
    And = 1,
    AndReverse = 2,
    Copy = 3,
    AndInverted = 4,
    NoOp = 5,
    Xor = 6,
    Or = 7,
    Nor = 8,
    Equivalent = 9,
    Invert = 10,
    OrReverse = 11,
    CopyInverted = 12,
    Inverted = 13,
    Nand = 14,
    Set = 15
}

#[repr(C)]
#[derive(Copy, Clone)]
pub enum BlendFactor {
    Zero = 0,
    One = 1,
    SrcColor = 2,
    OneMinusSrcColor = 3,
    DstColor = 4,
    OneMinusDstColor = 5,
    SrcAlpha = 6,
    OneMinusSrcAlpha = 7,
    DstAlpha = 8,
    OneMinusDstAlpha = 9,
    ConstantColor = 10,
    OneMinusConnstantColor = 11,
    ConstantAlpha = 12,
    OneMinusConstantAlpha = 13,
    SrcAlphaSaturate = 14,
    Src1Color = 15,
    OneMinusSrc1Color = 16,
    Src1Alpha = 17,
    OneMinusSrc1Alpha = 18
}

#[repr(C)]
#[derive(Copy, Clone)]
pub enum BlendOp {
    Add = 0,
    Subtract = 1,
    ReverseSubtract = 2,
    Min = 3,
    Max = 4
}

#[repr(C)]
#[derive(Copy, Clone)]
pub enum DynamicState {
    Viewport = 0,
    Scissor = 1,
    LineWidth = 2,
    DepthBias = 3,
    BlendConstants = 4,
    DepthBounds = 5,
    StencilCompareMask = 6,
    StencilWriteMask = 7,
    StencilReference = 8
}

#[repr(C)]
#[derive(Copy, Clone)]
pub enum Filter {
    Nearest = 0,
    Linear = 1
}

#[repr(C)]
#[derive(Copy, Clone)]
pub enum SamplerMipmapMode {
    Nearest = 0,
    Linear = 1
}

#[repr(C)]
#[derive(Copy, Clone)]
pub enum SamplerAddressMode {
    Repeat = 0,
    MirroredRepeat = 1,
    ClampToEdge = 2,
    ClampToBorder = 3,
    MirrorClampToEdge = 4
}

#[repr(C)]
#[derive(Copy, Clone)]
pub enum BorderColor {
    FloatTransparentBlack = 0,
    IntTransparentBlack = 1,
    FloatOpaqueBlack = 2,
    IntOpaqueBlack = 3,
    FloatOpaqueWhite = 4,
    IntOpaqueWhite = 5
}

#[repr(C)]
#[derive(Copy, Clone)]
pub enum DescriptorType {
    Sampler = 0,
    CombinedImageSampler = 1,
    SampledImage = 2,
    StorageImage = 3,
    UniformTexelBuffer = 4,
    StorageTexelBuffer = 5,
    UniformBuffer = 6,
    StorageBuffer = 7,
    UniformBufferDynamic = 8,
    StorageBufferDynamic = 9,
    InputAttachment = 10
}

#[repr(C)]
#[derive(Copy, Clone)]
pub enum AttachmentLoadOp {
    Load = 0,
    Clear = 1,
    DontCare = 2
}

#[repr(C)]
#[derive(Copy, Clone)]
pub enum AttachmentStoreOp {
    Store = 0,
    DontCare = 1
}

#[repr(C)]
#[derive(Copy, Clone)]
pub enum PipelineBindPoint {
    Graphics = 0,
    Compute = 1
}

#[repr(C)]
#[derive(Copy, Clone)]
pub enum CommandBufferLevel {
    Primary = 0,
    Secondary = 1
}

#[repr(C)]
#[derive(Copy, Clone)]
pub enum IndexType {
    Uint16 = 0,
    Uint32 = 1
}

#[repr(C)]
#[derive(Copy, Clone)]
pub enum SubpassContents {
    Inline = 0,
    SecondaryCommandBuffers = 1
}

#[repr(C)]
#[derive(Copy, Clone)]
pub enum InstanceCreateFlags {
    None = 0x00,
    Unused = 0xffffffff
}

#[repr(C)]
#[derive(Copy, Clone)]
pub enum FormatFeatureFlags {
    None = 0x00000000,
    SampledImage = 0x00000001,
    StorageImage = 0x00000002,
    StorageImageAtomic = 0x00000004,
    UniformTexelBuffer = 0x00000008,
    StorageTexelBuffer = 0x00000010,
    StorageTexelBufferAtomic = 0x00000020,
    VetexBuffer = 0x00000040,
    ColorAttachment = 0x00000080,
    ColorAttachmentBlend = 0x00000100,
    DepthStencilAttachment = 0x00000200,
    BlitSrc = 0x00000400,
    BlitDst = 0x00000800,
    SampledImageFilterLinear = 0x00001000
}

#[repr(C)]
#[derive(Copy, Clone)]
pub enum ImageCreateFlags {
    None = 0x00,
    SparseBinding = 0x01,
    SparseResidency = 0x02,
    SparseAliased = 0x04,
    MutableFormat = 0x08,
    CubeCompatible = 0x10
}

#[repr(C)]
#[derive(Copy, Clone)]
pub enum ImageUsageFlags {
    None = 0x00,
    TransferSrc = 0x01,
    TransferDst = 0x02,
    Sampled = 0x04,
    Storage = 0x08,
    ColorAttachment = 0x10,
    DepthStencilAttachment = 0x20,
    TransientAttachment = 0x40,
    InputAttachment = 0x80
}

#[repr(C)]
#[derive(Copy, Clone)]
pub enum SampleCountFlags {
    Count0 = 0x00,
    Count1 = 0x01,
    Count2 = 0x02,
    Count4 = 0x04,
    Count8 = 0x08,
    Count16 = 0x10,
    Count32 = 0x20,
    Count64 = 0x40
}

#[repr(C)]
#[derive(Copy, Clone)]
pub enum QueueFlags {
    None = 0x00,
    Graphics = 0x01,
    Compute = 0x02,
    Transfer = 0x04,
    SparseBinding = 0x08
}

#[repr(C)]
#[derive(Copy, Clone)]
pub enum MemoryPropertyFlags {
    None = 0x00,
    DeviceLocal = 0x01,
    HostVisible = 0x02,
    HostCoherent = 0x04,
    HostCached = 0x08,
    LazilyAllocated = 0x10
}

#[repr(C)]
#[derive(Copy, Clone)]
pub enum MemoryHeapFlags {
    None = 0x00,
    DeviceLocal = 0x01
}

#[repr(C)]
#[derive(Copy, Clone)]
pub enum PipelineStageFlags {
    None = 0x00000000,
    TopOfPipe = 0x00000001,
    DrawIndirect = 0x00000002,
    VertexInput = 0x00000004,
    VertexShader = 0x00000008,
    TessellationControlShader = 0x00000010,
    TessellationEvaluationShader = 0x00000020,
    GeometryShader = 0x00000040,
    FragmentShader = 0x00000080,
    EarlyFragmentTests = 0x00000100,
    LateFragmentTests = 0x00000200,
    ColorAttachmentOutput = 0x00000400,
    ComputeShader = 0x00000800,
    Transfer = 0x00001000,
    BottomOfPipe = 0x2000,
    Host = 0x00004000,
    AllGraphics = 0x00008000,
    AllCommands = 0x00010000
}

#[repr(C)]
#[derive(Copy, Clone)]
pub enum ImageAspectFlags {
    None = 0x00,
    Color = 0x01,
    Depth = 0x02,
    Stencil = 0x04,
    Metadata = 0x08
}

#[repr(C)]
#[derive(Copy, Clone)]
pub enum SparseImageFormatFlags {
    None = 0x00,
    SingleMiptail = 0x01,
    AlignedMipSize = 0x02,
    NonstandardBlockSize = 0x04
}

#[repr(C)]
#[derive(Copy, Clone)]
pub enum SparseMemoryBindFlags {
    None = 0x00,
    BindMetadata = 0x01
}

#[repr(C)]
#[derive(Copy, Clone)]
pub enum FenceCreateFlags {
    None = 0x00,
    Signaled = 0x01
}

#[repr(C)]
#[derive(Copy, Clone)]
pub enum QueryPipelineStatisticFlags {
    None = 0x0000,
    InputAssemblyVertices = 0x0001,
    InputAssemblyPrimitives = 0x0002,
    VertexShaderInvocations = 0x0004,
    GeometryShaderInvocations = 0x0008,
    GeometryShaderPrimitives = 0x0010,
    ClippingInvocations = 0x0020,
    ClippingPrimitives = 0x0040,
    FragmentShaderInvocations = 0x0080,
    TessellationControlShaderPatches = 0x0100,
    TessellationEvaluationShaderInvocations = 0x0200,
    ComputeShaderInvocations = 0x0400
}

#[repr(C)]
pub enum QueryErrorFlags {
    Error64 = 0x01,
    ErrorWait = 0x02,
    ErrorWithAvailability = 0x04,
    ErrorPartial = 0x08
}

#[repr(C)]
#[derive(Copy, Clone)]
pub enum BufferCreateFlags {
    None = 0x00,
    SparseBinding = 0x01,
    SparseResidency = 0x02,
    SparseAliased = 0x04
}

#[repr(C)]
#[derive(Copy, Clone)]
pub enum BufferUsageFlags {
    None = 0x0000,
    TransferSrc = 0x0001,
    TransferDst = 0x0002,
    UniformTexelBuffer = 0x0004,
    StorageTexelBuffer = 0x0008,
    UnifromBuffer = 0x0010,
    StorageBuffer = 0x0020,
    IndexBuffer = 0x0040,
    VertexBuffer = 0x0080,
    IndirectBuffer = 0x0100
}

#[repr(C)]
#[derive(Copy, Clone)]
pub enum PipelineCreateFlags {
    None = 0x00,
    DisableOptimization = 0x01,
    AllowDerivatives = 0x02,
    Derivative = 0x04
}

#[repr(C)]
#[derive(Copy, Clone)]
pub enum ShaderStageFlags {
    None = 0x00,
    Vertex = 0x01,
    TessellationControl = 0x02,
    TessellationEvaluation = 0x04,
    Geometry = 0x08,
    Fragment = 0x10,
    Compute = 0x20,
    AllGraphics = 0x1F,
    All = 0x7FFFFFFF
}

#[repr(C)]
#[derive(Copy, Clone)]
pub enum CullModeFlags {
    None = 0x00,
    Front = 0x01,
    Back = 0x02,
    FrontAndBack = 0x03
}

#[repr(C)]
#[derive(Copy, Clone)]
pub enum ColorComponentFlags {
    None = 0x00,
    R = 0x01,
    G = 0x02,
    B = 0x04,
    A = 0x08
}

#[repr(C)]
#[derive(Copy, Clone)]
pub enum DescriptorPoolCreateFlags {
    None = 0x00,
    FreeDescriptorSet = 0x01
}

#[repr(C)]
#[derive(Copy, Clone)]
pub enum AttachmentDescriptionFlags {
    None = 0x00,
    MayAlias = 0x01
}

#[repr(C)]
#[derive(Copy, Clone)]
pub enum AccessFlags {
    None = 0x00000000,
    IndirectCommandRead = 0x00000001,
    IndexRead = 0x00000002,
    VertexAttributeRead = 0x00000004,
    UniformRead = 0x00000008,
    InputAttachmentread = 0x00000010,
    ShaderRead = 0x00000020,
    ShaderWrite = 0x00000040,
    ColorAttachmentRead = 0x00000080,
    ColorAttachmentWrite = 0x00000100,
    DepthStencilAttachmentRead = 0x00000200,
    DepthSetncilAttachmentWrite = 0x00000400,
    TransferRead = 0x00000800,
    TransferWrite = 0x00001000,
    HostRead = 0x00002000,
    HostWrite = 0x00004000,
    MemoryRead = 0x00008000,
    MemoryWrite = 0x00010000
}

#[repr(C)]
#[derive(Copy, Clone)]
pub enum DependencyFlags {
    None = 0x00,
    ByRegion = 0x01
}

#[repr(C)]
#[derive(Copy, Clone)]
pub enum CommandPoolCreateFlags {
    None = 0x00,
    Transient = 0x01,
    ResetCommandBuffer = 0x02
}

#[repr(C)]
#[derive(Copy, Clone)]
pub enum CommandPoolResetFlags {
    None = 0x00,
    ResetReleaseResources = 0x01
}

#[repr(C)]
#[derive(Copy, Clone)]
pub enum CommandBufferUsageFlags {
    None = 0x00,
    OneTimeSubmit = 0x01,
    RenderPassContinue = 0x02,
    SimultaneousUse = 0x04
}

#[repr(C)]
#[derive(Copy, Clone)]
pub enum QueryControlFlags {
    None = 0x00,
    Precise = 0x01
}

#[repr(C)]
#[derive(Copy, Clone)]
pub enum CommandBufferResetFlags {
    None = 0x00,
    ResetReleaseResources = 0x01
}

#[repr(C)]
#[derive(Copy, Clone)]
pub enum StencilFaceFlags {
    Front = 0x01,
    Back = 0x02,
    FrontAndBack = 0x03
}

#[repr(C)]
#[derive(Copy, Clone)]
pub enum DescriptorPoolResetFlags {
    None = 0x00,
    Unused = 0xffffffff
}

#[repr(C)]
#[derive(Copy, Clone)]
pub enum RenderPassCreateFlags {
    None = 0x00,
    Unused = 0xffffffff
}

#[repr(C)]
#[derive(Copy, Clone)]
pub enum SubpassDescriptionFlags {
    None = 0x00,
    Unused = 0xffffffff
}

#[repr(C)]
#[derive(Copy, Clone)]
pub enum FramebufferCreateFlags {
    None = 0x00,
    Unused = 0xffffffff
}

#[repr(C)]
#[derive(Copy, Clone)]
pub enum DeviceCreateFlags {
    None = 0x00,
    Unused = 0xffffffff
}

#[repr(C)]
#[derive(Copy, Clone)]
pub enum DeviceQueueCreateFlags {
    None = 0x00,
    Unused = 0xffffffff
}

#[repr(C)]
#[derive(Copy, Clone)]
pub enum MemoryMapFlags {
    None = 0x00,
    Unused = 0xffffffff
}

#[repr(C)]
#[derive(Copy, Clone)]
pub enum SemaphoreCreateFlags {
    None = 0x00,
    Unused = 0xffffffff
}

#[repr(C)]
#[derive(Copy, Clone)]
pub enum EventCreateFlags {
    None = 0x00,
    Unused = 0xffffffff
}

#[repr(C)]
#[derive(Copy, Clone)]
pub enum QueryPoolCreateFlags {
    None = 0x00,
    Unused = 0xffffffff
}

#[repr(C)]
#[derive(Copy, Clone)]
pub enum BufferViewCreateFlags {
    None = 0x00,
    Unused = 0xffffffff
}

#[repr(C)]
#[derive(Copy, Clone)]
pub enum ImageViewCreateFlags {
    None = 0x00,
    Unused = 0xffffffff
}

#[repr(C)]
#[derive(Copy, Clone)]
pub enum ShaderModuleCreateFlags {
    None = 0x00,
    Unused = 0xffffffff
}

#[repr(C)]
#[derive(Copy, Clone)]
pub enum PipelineCacheCreateFlags {
    None = 0x00,
    Unused = 0xffffffff
}

#[repr(C)]
#[derive(Copy, Clone)]
pub enum PipelineShaderStageCreateFlags {
    None = 0x00,
    Unused = 0xffffffff
}

#[repr(C)]
#[derive(Copy, Clone)]
pub enum PipelineVertexInputStateCreateFlags {
    None = 0x00,
    Unused = 0xffffffff
}

#[repr(C)]
#[derive(Copy, Clone)]
pub enum PipelineInputAssemblyStateCreateFlags {
    None = 0x00,
    Unused = 0xffffffff
}

#[repr(C)]
#[derive(Copy, Clone)]
pub enum PipelineTessellationStateCreateFlags {
    None = 0x00,
    Unused = 0xffffffff
}

#[repr(C)]
#[derive(Copy, Clone)]
pub enum PipelineViewportStateCreateFlags {
    None = 0x00,
    Unused = 0xffffffff
}

#[repr(C)]
#[derive(Copy, Clone)]
pub enum PipelineRasterizationStateCreateFlags {
    None = 0x00,
    Unused = 0xffffffff
}

#[repr(C)]
#[derive(Copy, Clone)]
pub enum PipelineMultisampleStateCreateFlags {
    None = 0x00,
    Unused = 0xffffffff
}

#[repr(C)]
#[derive(Copy, Clone)]
pub enum PipelineDepthStencilStateCreateFlags {
    None = 0x00,
    Unused = 0xffffffff
}

#[repr(C)]
#[derive(Copy, Clone)]
pub enum PipelineColorBlendStateCreateFlags {
    None = 0x00,
    Unused = 0xffffffff
}

#[repr(C)]
#[derive(Copy, Clone)]
pub enum PipelineDynamicStateCreateFlags {
    None = 0x00,
    Unused = 0xffffffff
}

#[repr(C)]
#[derive(Copy, Clone)]
pub enum PipelineLayoutCreateFlags {
    None = 0x00,
    Unused = 0xffffffff
}

#[repr(C)]
#[derive(Copy, Clone)]
pub enum SamplerCreateFlags {
    None = 0x00,
    Unused = 0xffffffff
}

#[repr(C)]
#[derive(Copy, Clone)]
pub enum DescriptorSetLayoutCreateFlags {
    None = 0x00,
    Unused = 0xffffffff
}


#[repr(C)]
#[allow(non_snake_case)]
#[derive(Copy, Clone)]
pub struct ApplicationInfo {
    pub sType: StructureType,
    pub pNext: *const c_void,
    pub pApplicationName: *const c_char,
    pub applicationVersion: uint32_t,
    pub pEngineName: *const c_char,
    pub engineVersion: uint32_t,
    pub apiVersion: uint32_t
}

impl Default for ApplicationInfo {
    fn default() -> ApplicationInfo {
        ApplicationInfo {
            sType: StructureType::ApplicationInfo,
            pNext: ptr::null(),
            pApplicationName: ptr::null(),
            applicationVersion: 0,
            pEngineName: ptr::null(),
            engineVersion: 0,
            apiVersion: 0
        }
    }
}

// impl fmt::Debug for ApplicationInfo {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Error {
//         write!(
//             f,
//             r#"
//             ApplicationInfo (
//               pApplicationName: {},
//               applicationVersion: {},
//               pEngineName: {},
//               engineVersion: {},
//               apiVersion: {}
//             )
//             "# ,
//             // try!(CString::from_raw(self.pApplicationName).into_string()),
//             pApplicationName
//             self.applicationVersion,
//             // try!(CString::from_raw(self.pEngineName).into_string()),
//             pEngineName,
//             self.engineVersion, self.apiVersion
//         )
//     }
// }

#[repr(C)]
#[allow(non_snake_case)]
#[derive(Copy, Clone)]
pub struct InstanceCreateInfo {
    pub sType: StructureType,
    pub pNext: *const c_void,
    pub flags: InstanceCreateFlags,
    pub pApplicationInfo: *const ApplicationInfo,
    pub enabledLayerCount: uint32_t,
    pub ppEnabledLayerNames: *const *const libc::wchar_t,
    pub enabledExtensionCount: uint32_t,
    pub ppEnabledExtensionNames: *const *const libc::wchar_t
}

impl Default for InstanceCreateInfo {
    fn default() -> InstanceCreateInfo {
        InstanceCreateInfo {
            sType: StructureType::InstanceCreateInfo,
            pNext: ptr::null(),
            flags: InstanceCreateFlags::None,
            pApplicationInfo: ptr::null(),
            enabledLayerCount: 0,
            ppEnabledLayerNames: ptr::null(),
            enabledExtensionCount: 0,
            ppEnabledExtensionNames: ptr::null()
        }
    }
}

// TODO: code generate this.
// impl fmt::Debug for InstanceCreateInfo {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Error {
//         write!(
//             f,
//             r#"
//             InstanceCreateInfo (
//               pApplicationInfo: {},
//               enabledLayerCount: {},
//               ppEnabledLayerNames: {},
//               enabledExtensionCount: {},
//               ppEnabledExtensionNames: {}
//             )
//             "#,
//             self.pApplicationInfo,
//             self.enabledLayerCount, self.ppEnabledLayerCount,
//             self.enabledExtensionCount, self.ppEnabledExtensionNames
//         )
//     }
// }

#[repr(C)]
#[allow(non_snake_case)]
#[derive(Copy, Clone)]
pub struct AllocationCallbacks {
    pub pUserData: *mut c_void,
    pub pfnAllocation: Option<AllocationFunction>,
    pub pfnReallocation: Option<ReallocationFunction>,
    pub pfnFree: Option<FreeFunction>,
    pub pfnInternalAllocation: Option<InternalAllocationNotificationFunction>,
    pub pfnInternalFree: Option<InternalFreeNotificationFunction>
}

impl Default for AllocationCallbacks {
    fn default() -> AllocationCallbacks {
        AllocationCallbacks {
            pUserData: ptr::null_mut(),
            pfnAllocation: None,
            pfnReallocation: None,
            pfnFree: None,
            pfnInternalAllocation: None,
            pfnInternalFree: None
        }
    }
}

#[repr(C)]
#[allow(non_snake_case)]
#[derive(Copy, Clone)]
pub struct PhysicalDeviceFeatures {
    pub robustBufferAccess: Bool32,
    pub fullDrawIndexUint32: Bool32,
    pub imageCubeArray: Bool32,
    pub independentBlend: Bool32,
    pub geometryShader: Bool32,
    pub tessellationShader: Bool32,
    pub sampleRateShading: Bool32,
    pub dualSrcBlend: Bool32,
    pub logicOp: Bool32,
    pub multiDrawIndirect: Bool32,
    pub drawIndirectFirstInstance: Bool32,
    pub depthClamp: Bool32,
    pub depthBiasClamp: Bool32,
    pub fillModeNonSolid: Bool32,
    pub depthBounds: Bool32,
    pub wideLines: Bool32,
    pub largePoints: Bool32,
    pub alphaToOne: Bool32,
    pub multiViewport: Bool32,
    pub samplerAnisotropy: Bool32,
    pub textureCompressionETC2: Bool32,
    pub textureCompressionASTC_LDR: Bool32,
    pub textureCompressionBC: Bool32,
    pub occlusionQueryPrecise: Bool32,
    pub pipelineStatisticsQuery: Bool32,
    pub vertexPipelineStoresAndAtomics: Bool32,
    pub fragmentStoresAndAtomics: Bool32,
    pub shaderTessellationAndGeometryPointSize: Bool32,
    pub shaderImageGatherExtended: Bool32,
    pub shaderStorageImageExtendedFormats: Bool32,
    pub shaderStorageImageMultisample: Bool32,
    pub shaderStorageImageReadWithoutFormat: Bool32,
    pub shaderStorageImageWriteWithoutFormat: Bool32,
    pub shaderUniformBufferArrayDynamicIndexing: Bool32,
    pub shaderSampledImageArrayDynamicIndexing: Bool32,
    pub shaderStorageBufferArrayDynamicIndexing: Bool32,
    pub shaderStorageImageArrayDynamicIndexing: Bool32,
    pub shaderClipDistance: Bool32,
    pub shaderCullDistance: Bool32,
    pub shaderFloat64: Bool32,
    pub shaderInt64: Bool32,
    pub shaderInt16: Bool32,
    pub shaderResourceResidency: Bool32,
    pub shaderResourceMinLod: Bool32,
    pub sparseBinding: Bool32,
    pub sparseResidencyBuffer: Bool32,
    pub sparseResidencyImage2D: Bool32,
    pub sparseResidencyImage3D: Bool32,
    pub sparseResidency2Samples: Bool32,
    pub sparseResidency4Samples: Bool32,
    pub sparseResidency8Samples: Bool32,
    pub sparseResidency16Samples: Bool32,
    pub sparseResidencyAliased: Bool32,
    pub variableMultisampleRate: Bool32,
    pub inheritedQueries: Bool32
}

impl Default for PhysicalDeviceFeatures {
    fn default() -> PhysicalDeviceFeatures {
        PhysicalDeviceFeatures {
            robustBufferAccess: 0,
            fullDrawIndexUint32: 0,
            imageCubeArray: 0,
            independentBlend: 0,
            geometryShader: 0,
            tessellationShader: 0,
            sampleRateShading: 0,
            dualSrcBlend: 0,
            logicOp: 0,
            multiDrawIndirect: 0,
            drawIndirectFirstInstance: 0,
            depthClamp: 0,
            depthBiasClamp: 0,
            fillModeNonSolid: 0,
            depthBounds: 0,
            wideLines: 0,
            largePoints: 0,
            alphaToOne: 0,
            multiViewport: 0,
            samplerAnisotropy: 0,
            textureCompressionETC2: 0,
            textureCompressionASTC_LDR: 0,
            textureCompressionBC: 0,
            occlusionQueryPrecise: 0,
            pipelineStatisticsQuery: 0,
            vertexPipelineStoresAndAtomics: 0,
            fragmentStoresAndAtomics: 0,
            shaderTessellationAndGeometryPointSize: 0,
            shaderImageGatherExtended: 0,
            shaderStorageImageExtendedFormats: 0,
            shaderStorageImageMultisample: 0,
            shaderStorageImageReadWithoutFormat: 0,
            shaderStorageImageWriteWithoutFormat: 0,
            shaderUniformBufferArrayDynamicIndexing: 0,
            shaderSampledImageArrayDynamicIndexing: 0,
            shaderStorageBufferArrayDynamicIndexing: 0,
            shaderStorageImageArrayDynamicIndexing: 0,
            shaderClipDistance: 0,
            shaderCullDistance: 0,
            shaderFloat64: 0,
            shaderInt64: 0,
            shaderInt16: 0,
            shaderResourceResidency: 0,
            shaderResourceMinLod: 0,
            sparseBinding: 0,
            sparseResidencyBuffer: 0,
            sparseResidencyImage2D: 0,
            sparseResidencyImage3D: 0,
            sparseResidency2Samples: 0,
            sparseResidency4Samples: 0,
            sparseResidency8Samples: 0,
            sparseResidency16Samples: 0,
            sparseResidencyAliased: 0,
            variableMultisampleRate: 0,
            inheritedQueries: 0
        }
    }
}

#[repr(C)]
#[allow(non_snake_case)]
#[derive(Copy, Clone)]
pub struct FormatProperties {
    pub linearTilingFeatures: FormatFeatureFlags,
    pub optimalTilingFeatures: FormatFeatureFlags,
    pub bufferFeatures: FormatFeatureFlags
}

impl Default for FormatProperties {
    fn default() -> FormatProperties {
        FormatProperties {
            linearTilingFeatures: FormatFeatureFlags::None,
            optimalTilingFeatures: FormatFeatureFlags::None,
            bufferFeatures: FormatFeatureFlags::None
        }
    }
}

#[repr(C)]
#[allow(non_snake_case)]
#[derive(Copy, Clone)]
pub struct Extent3D {
    pub width: uint32_t,
    pub height: uint32_t,
    pub depth: uint32_t
}

impl Default for Extent3D {
    fn default() -> Extent3D {
        Extent3D {
            width: 0,
            height: 0,
            depth: 0
        }
    }
}

#[repr(C)]
#[allow(non_snake_case)]
#[derive(Copy, Clone)]
pub struct ImageFormatProperties {
    pub maxExtent: Extent3D,
    pub maxMipLevels: uint32_t,
    pub maxArrayLayers: uint32_t,
    pub sampleCounts: SampleCountFlags,
    pub maxResourceSize: DeviceSize
}

impl Default for ImageFormatProperties {
    fn default() -> ImageFormatProperties {
        ImageFormatProperties {
            maxExtent: Extent3D{..Default::default()},
            maxMipLevels: 0,
            maxArrayLayers: 0,
            sampleCounts: SampleCountFlags::Count0,
            maxResourceSize: 0
        }
    }
}

#[repr(C)]
#[allow(non_snake_case)]
#[derive(Copy, Clone)]
pub struct PhysicalDeviceLimits {
    pub maxImageDimension1D: uint32_t,
    pub maxImageDimension2D: uint32_t,
    pub maxImageDimension3D: uint32_t,
    pub maxImageDimensionCube: uint32_t,
    pub maxImageArrayLayers: uint32_t,
    pub maxTexelBufferElements: uint32_t,
    pub maxUniformBufferRange: uint32_t,
    pub maxStorageBufferRange: uint32_t,
    pub maxPushConstantsSize: uint32_t,
    pub maxMemoryAllocationCount: uint32_t,
    pub maxSamplerAllocationCount: uint32_t,
    pub bufferImageGranularity: DeviceSize,
    pub sparseAddressSpaceSize: DeviceSize,
    pub maxBoundDescriptorSets: uint32_t,
    pub maxPerStageDescriptorSamplers: uint32_t,
    pub maxPerStageDescriptorUniformBuffers: uint32_t,
    pub maxPerStageDescriptorStorageBuffers: uint32_t,
    pub maxPerStageDescriptorSampledImages: uint32_t,
    pub maxPerStageDescriptorStorageImages: uint32_t,
    pub maxPerStageDescriptorInputAttachments: uint32_t,
    pub maxPerStageResources: uint32_t,
    pub maxDescriptorSetSamplers: uint32_t,
    pub maxDescriptorSetUniformBuffers: uint32_t,
    pub maxDescriptorSetUniformBuffersDynamic: uint32_t,
    pub maxDescriptorSetStorageBuffers: uint32_t,
    pub maxDescriptorSetStorageBuffersDynamic: uint32_t,
    pub maxDescriptorSetSampledImages: uint32_t,
    pub maxDescriptorSetStorageImages: uint32_t,
    pub maxDescriptorSetInputAttachments: uint32_t,
    pub maxVertexInputAttributes: uint32_t,
    pub maxVertexInputBindings: uint32_t,
    pub maxVertexInputAttributeOffset: uint32_t,
    pub maxVertexInputBindingStride: uint32_t,
    pub maxVertexOutputComponents: uint32_t,
    pub maxTessellationGenerationLevel: uint32_t,
    pub maxTessellationPatchSize: uint32_t,
    pub maxTessellationControlPerVertexInputComponents: uint32_t,
    pub maxTessellationControlPerVertexOutputComponents: uint32_t,
    pub maxTessellationControlPerPatchOutputComponents: uint32_t,
    pub maxTessellationControlTotalOutputComponents: uint32_t,
    pub maxTessellationEvaluationInputComponents: uint32_t,
    pub maxTessellationEvaluationOutputComponents: uint32_t,
    pub maxGeometryShaderInvocations: uint32_t,
    pub maxGeometryInputComponents: uint32_t,
    pub maxGeometryOutputComponents: uint32_t,
    pub maxGeometryOutputVertices: uint32_t,
    pub maxGeometryTotalOutputComponents: uint32_t,
    pub maxFragmentInputComponents: uint32_t,
    pub maxFragmentOutputAttachments: uint32_t,
    pub maxFragmentDualSrcAttachments: uint32_t,
    pub maxFragmentCombinedOutputResources: uint32_t,
    pub maxComputeSharedMemorySize: uint32_t,
    pub maxComputeWorkGroupCount: [uint32_t; 3],
    pub maxComputeWorkGroupInvocations: uint32_t,
    pub maxComputeWorkGroupSize: [uint32_t; 3],
    pub subPixelPrecisionBits: uint32_t,
    pub subTexelPrecisionBits: uint32_t,
    pub mipmapPrecisionBits: uint32_t,
    pub maxDrawIndexedIndexValue: uint32_t,
    pub maxDrawIndirectCount: uint32_t,
    pub maxSamplerLodBias: c_float,
    pub maxSamplerAnisotropy: c_float,
    pub maxViewports: uint32_t,
    pub maxViewportDimensions: [uint32_t; 2],
    pub viewportBoundsRange: [c_float; 2],
    pub viewportSubPixelBits: uint32_t,
    pub minMemoryMapAlignment: size_t,
    pub minTexelBufferOffsetAlignment: DeviceSize,
    pub minUniformBufferOffsetAlignment: DeviceSize,
    pub minStorageBufferOffsetAlignment: DeviceSize,
    pub minTexelOffset: int32_t,
    pub maxTexelOffset: uint32_t,
    pub minTexelGatherOffset: int32_t,
    pub maxTexelGatherOffset: uint32_t,
    pub minInterpolationOffset: c_float,
    pub maxInterpolationOffset: c_float,
    pub subPixelInterpolationOffsetBits: uint32_t,
    pub maxFramebufferWidth: uint32_t,
    pub maxFramebufferHeight: uint32_t,
    pub maxFramebufferLayers: uint32_t,
    pub framebufferColorSampleCounts: SampleCountFlags,
    pub framebufferDepthSampleCounts: SampleCountFlags,
    pub framebufferStencilSampleCounts: SampleCountFlags,
    pub framebufferNoAttachmentsSampleCounts: SampleCountFlags,
    pub maxColorAttachments: uint32_t,
    pub sampledImageColorSampleCounts: SampleCountFlags,
    pub sampledImageIntegerSampleCounts: SampleCountFlags,
    pub sampledImageDepthSampleCounts: SampleCountFlags,
    pub sampledImageStencilSampleCounts: SampleCountFlags,
    pub storageImageSampleCounts: SampleCountFlags,
    pub maxSampleMaskWords: uint32_t,
    pub timestampComputeAndGraphics: Bool32,
    pub timestampPeriod: c_float,
    pub maxClipDistances: uint32_t,
    pub maxCullDistances: uint32_t,
    pub maxCombinedClipAndCullDistances: uint32_t,
    pub discreteQueuePriorities: uint32_t,
    pub pointSizeRange: [c_float; 2],
    pub lineWidthRange: [c_float; 2],
    pub pointSizeGranularity: c_float,
    pub lineWidthGranularity: c_float,
    pub strictLines: Bool32,
    pub standardSampleLocations: Bool32,
    pub optimalBufferCopyOffsetAlignment: DeviceSize,
    pub optimalBufferCopyRowPitchAlignment: DeviceSize,
    pub nonCoherentAtomSize: DeviceSize
}

impl Default for PhysicalDeviceLimits {
    fn default() -> PhysicalDeviceLimits {
        PhysicalDeviceLimits {
            maxImageDimension1D: 0,
            maxImageDimension2D: 0,
            maxImageDimension3D: 0,
            maxImageDimensionCube: 0,
            maxImageArrayLayers: 0,
            maxTexelBufferElements: 0,
            maxUniformBufferRange: 0,
            maxStorageBufferRange: 0,
            maxPushConstantsSize: 0,
            maxMemoryAllocationCount: 0,
            maxSamplerAllocationCount: 0,
            bufferImageGranularity: 0,
            sparseAddressSpaceSize: 0,
            maxBoundDescriptorSets: 0,
            maxPerStageDescriptorSamplers: 0,
            maxPerStageDescriptorUniformBuffers: 0,
            maxPerStageDescriptorStorageBuffers: 0,
            maxPerStageDescriptorSampledImages: 0,
            maxPerStageDescriptorStorageImages: 0,
            maxPerStageDescriptorInputAttachments: 0,
            maxPerStageResources: 0,
            maxDescriptorSetSamplers: 0,
            maxDescriptorSetUniformBuffers: 0,
            maxDescriptorSetUniformBuffersDynamic: 0,
            maxDescriptorSetStorageBuffers: 0,
            maxDescriptorSetStorageBuffersDynamic: 0,
            maxDescriptorSetSampledImages: 0,
            maxDescriptorSetStorageImages: 0,
            maxDescriptorSetInputAttachments: 0,
            maxVertexInputAttributes: 0,
            maxVertexInputBindings: 0,
            maxVertexInputAttributeOffset: 0,
            maxVertexInputBindingStride: 0,
            maxVertexOutputComponents: 0,
            maxTessellationGenerationLevel: 0,
            maxTessellationPatchSize: 0,
            maxTessellationControlPerVertexInputComponents: 0,
            maxTessellationControlPerVertexOutputComponents: 0,
            maxTessellationControlPerPatchOutputComponents: 0,
            maxTessellationControlTotalOutputComponents: 0,
            maxTessellationEvaluationInputComponents: 0,
            maxTessellationEvaluationOutputComponents: 0,
            maxGeometryShaderInvocations: 0,
            maxGeometryInputComponents: 0,
            maxGeometryOutputComponents: 0,
            maxGeometryOutputVertices: 0,
            maxGeometryTotalOutputComponents: 0,
            maxFragmentInputComponents: 0,
            maxFragmentOutputAttachments: 0,
            maxFragmentDualSrcAttachments: 0,
            maxFragmentCombinedOutputResources: 0,
            maxComputeSharedMemorySize: 0,
            maxComputeWorkGroupCount: [0; 3],
            maxComputeWorkGroupInvocations: 0,
            maxComputeWorkGroupSize: [0; 3],
            subPixelPrecisionBits: 0,
            subTexelPrecisionBits: 0,
            mipmapPrecisionBits: 0,
            maxDrawIndexedIndexValue: 0,
            maxDrawIndirectCount: 0,
            maxSamplerLodBias: 0.0,
            maxSamplerAnisotropy: 0.0,
            maxViewports: 0,
            maxViewportDimensions: [0; 2],
            viewportBoundsRange: [0.0; 2],
            viewportSubPixelBits: 0,
            minMemoryMapAlignment: 0,
            minTexelBufferOffsetAlignment: 0,
            minUniformBufferOffsetAlignment: 0,
            minStorageBufferOffsetAlignment: 0,
            minTexelOffset: 0,
            maxTexelOffset: 0,
            minTexelGatherOffset: 0,
            maxTexelGatherOffset: 0,
            minInterpolationOffset: 0.0,
            maxInterpolationOffset: 0.0,
            subPixelInterpolationOffsetBits: 0,
            maxFramebufferWidth: 0,
            maxFramebufferHeight: 0,
            maxFramebufferLayers: 0,
            framebufferColorSampleCounts: SampleCountFlags::Count0,
            framebufferDepthSampleCounts: SampleCountFlags::Count0,
            framebufferStencilSampleCounts: SampleCountFlags::Count0,
            framebufferNoAttachmentsSampleCounts: SampleCountFlags::Count0,
            maxColorAttachments: 0,
            sampledImageColorSampleCounts: SampleCountFlags::Count0,
            sampledImageIntegerSampleCounts: SampleCountFlags::Count0,
            sampledImageDepthSampleCounts: SampleCountFlags::Count0,
            sampledImageStencilSampleCounts: SampleCountFlags::Count0,
            storageImageSampleCounts: SampleCountFlags::Count0,
            maxSampleMaskWords: 0,
            timestampComputeAndGraphics: 0,
            timestampPeriod: 0.0,
            maxClipDistances: 0,
            maxCullDistances: 0,
            maxCombinedClipAndCullDistances: 0,
            discreteQueuePriorities: 0,
            pointSizeRange: [0.0; 2],
            lineWidthRange: [0.0; 2],
            pointSizeGranularity: 0.0,
            lineWidthGranularity: 0.0,
            strictLines: 0,
            standardSampleLocations: 0,
            optimalBufferCopyOffsetAlignment: 0,
            optimalBufferCopyRowPitchAlignment: 0,
            nonCoherentAtomSize: 0
        }
    }
}

#[repr(C)]
#[allow(non_snake_case)]
#[derive(Copy, Clone)]
pub struct PhysicalDeviceSparseProperties {
    pub residencyStandard2DBlockShape: Bool32,
    pub residencyStandard2DMultisampleBlockShape: Bool32,
    pub residencyStandard3DBlockShape: Bool32,
    pub residencyAlignedMipSize: Bool32,
    pub residencyNonResidentStrict: Bool32
}

impl Default for PhysicalDeviceSparseProperties {
    fn default() -> PhysicalDeviceSparseProperties {
        PhysicalDeviceSparseProperties {
            residencyStandard2DBlockShape: 0,
            residencyStandard2DMultisampleBlockShape: 0,
            residencyStandard3DBlockShape: 0,
            residencyAlignedMipSize: 0,
            residencyNonResidentStrict: 0
        }
    }
}

#[repr(C)]
#[allow(non_snake_case)]
#[derive(Copy)]
pub struct PhysicalDeviceProperties {
    pub apiVersion: uint32_t,
    pub driverVersion: uint32_t,
    pub vendorID: uint32_t,
    pub deviceID: uint32_t,
    pub deviceType: PhysicalDeviceType,
    pub deviceName: [c_char; VK_MAX_PHYSICAL_DEVICE_NAME_SIZE],
    pub pipelineCacheUUID: [uint8_t; VK_UUID_SIZE],
    pub limits: PhysicalDeviceLimits,
    pub sparseProperties: PhysicalDeviceSparseProperties
}

impl Default for PhysicalDeviceProperties {
    fn default() -> PhysicalDeviceProperties {
        PhysicalDeviceProperties {
            apiVersion: 0,
            driverVersion: 0,
            vendorID: 0,
            deviceID: 0,
            deviceType: PhysicalDeviceType::Other,
            deviceName: [0; VK_MAX_PHYSICAL_DEVICE_NAME_SIZE],
            pipelineCacheUUID: [0; VK_UUID_SIZE],
            limits: PhysicalDeviceLimits{..Default::default()},
            sparseProperties: PhysicalDeviceSparseProperties{..Default::default()}
        }
    }
}

impl Clone for PhysicalDeviceProperties {
    fn clone(&self) -> PhysicalDeviceProperties {
        PhysicalDeviceProperties {
            apiVersion: self.apiVersion.clone(),
            driverVersion: self.driverVersion.clone(),
            vendorID: self.vendorID.clone(),
            deviceID: self.deviceID.clone(),
            deviceType: self.deviceType.clone(),
            deviceName: self.deviceName,
            pipelineCacheUUID: self.pipelineCacheUUID,
            limits: self.limits.clone(),
            sparseProperties: self.sparseProperties.clone()
        }
    }
}

#[repr(C)]
#[allow(non_snake_case)]
#[derive(Copy, Clone)]
pub struct QueueFamilyProperties {
    pub queueFlags: QueueFlags,
    pub queueCount: uint32_t,
    pub timestampValidBits: uint32_t,
    pub minImageTransferGranularity: Extent3D
}

impl Default for QueueFamilyProperties {
    fn default() -> QueueFamilyProperties {
        QueueFamilyProperties {
            queueFlags: QueueFlags::None,
            queueCount: 0,
            timestampValidBits: 0,
            minImageTransferGranularity: Extent3D { ..Default::default() }
        }
    }
}

#[repr(C)]
#[allow(non_snake_case)]
#[derive(Copy, Clone)]
pub struct MemoryType {
    pub propertyFlags: MemoryPropertyFlags,
    pub heapIndex: uint32_t
}

impl Default for MemoryType {
    fn default() -> MemoryType {
        MemoryType {
            propertyFlags: MemoryPropertyFlags::None,
            heapIndex: 0
        }
    }
}

#[repr(C)]
#[allow(non_snake_case)]
#[derive(Copy, Clone)]
pub struct MemoryHeap {
    pub size: DeviceSize,
    pub flags: MemoryHeapFlags
}

impl Default for MemoryHeap {
    fn default() -> MemoryHeap {
        MemoryHeap {
            size: 0,
            flags: MemoryHeapFlags::None
        }
    }
}

#[repr(C)]
#[allow(non_snake_case)]
#[derive(Copy, Clone)]
pub struct PhysicalDeviceMemoryProperties {
    pub memoryTypeCount: uint32_t,
    pub memoryTypes: [MemoryType; VK_MAX_MEMORY_TYPES],
    pub memoryHeapCount: uint32_t,
    pub memoryHeaps: [MemoryHeap; VK_MAX_MEMORY_HEAPS]
}

impl Default for PhysicalDeviceMemoryProperties {
    fn default() -> PhysicalDeviceMemoryProperties {
        PhysicalDeviceMemoryProperties {
            memoryTypeCount: 0,
            memoryTypes: [MemoryType { ..Default::default() }; VK_MAX_MEMORY_TYPES],
            memoryHeapCount: 0,
            memoryHeaps: [MemoryHeap { ..Default::default() }; VK_MAX_MEMORY_HEAPS],
        }
    }
}

#[repr(C)]
#[allow(non_snake_case)]
#[derive(Copy, Clone)]
pub struct DeviceQueueCreateInfo {
    pub sType: StructureType,
    pub pNext: *const c_void,
    pub flags: DeviceQueueCreateFlags,
    pub queueFamilyIndex: uint32_t,
    pub queueCount: uint32_t,
    pub pQueuePriorities: *const c_float
}

impl Default for DeviceQueueCreateInfo {
    fn default() -> DeviceQueueCreateInfo {
        DeviceQueueCreateInfo {
            sType: StructureType::DeviceQueueCreateInfo,
            pNext: ptr::null(),
            flags: DeviceQueueCreateFlags::None,
            queueFamilyIndex: 0,
            queueCount: 0,
            pQueuePriorities: ptr::null()
        }
    }
}

#[repr(C)]
#[allow(non_snake_case)]
#[derive(Copy, Clone)]
pub struct DeviceCreateInfo {
    pub sType: StructureType,
    pub pNext: *const c_void,
    pub flags: DeviceCreateFlags,
    pub queueCreateInfoCount: uint32_t,
    pub pQueueCreateInfos: *const DeviceQueueCreateInfo,
    pub enabledLayerCount: uint32_t,
    pub ppEnabledExtensionNames: *const *const c_char,
    pub pEnabledFeatures: *const PhysicalDeviceFeatures
}

impl Default for DeviceCreateInfo {
    fn default() -> DeviceCreateInfo {
        DeviceCreateInfo {
            sType: StructureType::DeviceCreateInfo,
            pNext: ptr::null(),
            flags: DeviceCreateFlags::None,
            queueCreateInfoCount: 0,
            pQueueCreateInfos: ptr::null(),
            enabledLayerCount: 0,
            ppEnabledExtensionNames: ptr::null(),
            pEnabledFeatures: ptr::null()
        }
    }
}

#[repr(C)]
#[allow(non_snake_case)]
#[derive(Copy)]
pub struct ExtensionProperties {
    pub extensionName: [c_char; VK_MAX_EXTENSION_NAME_SIZE],
    pub specVersion: uint32_t
}

impl Default for ExtensionProperties {
    fn default() -> ExtensionProperties {
        ExtensionProperties {
            extensionName: [0; VK_MAX_EXTENSION_NAME_SIZE],
            specVersion: 0
        }
    }
}

impl Clone for ExtensionProperties {
    fn clone(&self) -> ExtensionProperties {
        ExtensionProperties {
            extensionName: self.extensionName,
            specVersion: self.specVersion.clone()
        }
    }
}

#[repr(C)]
#[allow(non_snake_case)]
#[derive(Copy)]
pub struct LayerProperties {
    pub layerName: [c_char; VK_MAX_EXTENSION_NAME_SIZE],
    pub specVersion: uint32_t,
    pub implementationVersion: uint32_t,
    pub description: [c_char; VK_MAX_DESCRIPTION_SIZE]
}

impl Default for LayerProperties {
    fn default() -> LayerProperties {
        LayerProperties {
            layerName: [0; VK_MAX_EXTENSION_NAME_SIZE],
            specVersion: 0,
            implementationVersion: 0,
            description: [0; VK_MAX_DESCRIPTION_SIZE]
        }
    }
}

impl Clone for LayerProperties {
    fn clone(&self) -> LayerProperties {
        LayerProperties {
            layerName: self.layerName,
            specVersion: self.specVersion.clone(),
            implementationVersion: self.implementationVersion.clone(),
            description: self.description
        }
    }
}

#[repr(C)]
#[allow(non_snake_case)]
#[derive(Copy, Clone)]
pub struct SubmitInfo {
    pub sType: StructureType,
    pub pNext: *const c_void,
    pub waitSemaphoreCount: uint32_t,
    pub pWaitSemaphores: *const Semaphore,
    pub pWaitDstStageMask: *const PipelineStageFlags,
    pub commandBufferCount: uint32_t,
    pub pCommandBuffers: *const CommandBuffer,
    pub signalSemaphoreCount: uint32_t,
    pub pSignalSemaphores: *const Semaphore
}

impl Default for SubmitInfo {
    fn default() -> SubmitInfo {
        SubmitInfo {
            sType: StructureType::SubmitInfo,
            pNext: ptr::null(),
            waitSemaphoreCount: 0,
            pWaitSemaphores: ptr::null(),
            pWaitDstStageMask: ptr::null(),
            commandBufferCount: 0,
            pCommandBuffers: ptr::null(),
            signalSemaphoreCount: 0,
            pSignalSemaphores: ptr::null()
        }
    }
}

#[repr(C)]
#[allow(non_snake_case)]
#[derive(Copy, Clone)]
pub struct MemoryAllocateInfo {
    pub sType: StructureType,
    pub pNext: *const c_void,
    pub allocationSize: DeviceSize,
    pub memoryTypeIndex: uint32_t
}

impl Default for MemoryAllocateInfo {
    fn default() -> MemoryAllocateInfo {
        MemoryAllocateInfo {
            sType: StructureType::MemoryAllocateInfo,
            pNext: ptr::null(),
            allocationSize: 0,
            memoryTypeIndex: 0
        }
    }
}

#[repr(C)]
#[allow(non_snake_case)]
#[derive(Copy, Clone)]
pub struct MappedMemoryRange {
    pub sType: StructureType,
    pub pNext: *const c_void,
    pub memory: DeviceMemory,
    pub offset: DeviceSize,
    pub size: DeviceSize
}

impl Default for MappedMemoryRange {
    fn default() -> MappedMemoryRange {
        MappedMemoryRange {
            sType: StructureType::MappedMemoryRange,
            pNext: ptr::null(),
            memory: ptr::null_mut(),
            offset: 0,
            size: 0
        }
    }
}

#[repr(C)]
#[allow(non_snake_case)]
#[derive(Copy, Clone)]
pub struct MemoryRequirements {
    pub size: DeviceSize,
    pub alignment: DeviceSize,
    pub memoryTypeBits: uint32_t
}

impl Default for MemoryRequirements {
    fn default() -> MemoryRequirements {
        MemoryRequirements {
            size: 0,
            alignment: 0,
            memoryTypeBits: 0
        }
    }
}

#[repr(C)]
#[allow(non_snake_case)]
#[derive(Copy, Clone)]
pub struct SparseImageFormatProperties {
    pub aspectMask: ImageAspectFlags,
    pub imageGranularity: Extent3D,
    pub flags: SparseImageFormatFlags
}

impl Default for SparseImageFormatProperties {
    fn default() -> SparseImageFormatProperties {
        SparseImageFormatProperties {
            aspectMask: ImageAspectFlags::None,
            imageGranularity: Extent3D{..Default::default()},
            flags: SparseImageFormatFlags::None
        }
    }
}

#[repr(C)]
#[allow(non_snake_case)]
#[derive(Copy, Clone)]
pub struct SparseImageMemoryRequirements {
    pub formatProperties: SparseImageFormatProperties,
    pub imageMipTailFirstLod: uint32_t,
    pub imageMipTailSize: DeviceSize,
    pub imageMipTailOffset: DeviceSize,
    pub imageMipTailStride: DeviceSize
}

impl Default for SparseImageMemoryRequirements {
    fn default() -> SparseImageMemoryRequirements {
        SparseImageMemoryRequirements {
            formatProperties: SparseImageFormatProperties{..Default::default()},
            imageMipTailFirstLod: 0,
            imageMipTailSize: 0,
            imageMipTailOffset: 0,
            imageMipTailStride: 0
        }
    }
}

#[repr(C)]
#[allow(non_snake_case)]
#[derive(Copy, Clone)]
pub struct SparseMemoryBind {
    pub resourceOffset: DeviceSize,
    pub size: DeviceSize,
    pub memory: DeviceMemory,
    pub memoryOffset: DeviceSize,
    pub flags: SparseMemoryBindFlags
}

impl Default for SparseMemoryBind {
    fn default() -> SparseMemoryBind {
        SparseMemoryBind {
            resourceOffset: 0,
            size: 0,
            memory: ptr::null_mut(),
            memoryOffset: 0,
            flags: SparseMemoryBindFlags::None
        }
    }
}

#[repr(C)]
#[allow(non_snake_case)]
#[derive(Copy, Clone)]
pub struct SparseBufferMemoryBindInfo {
    pub buffer: Buffer,
    pub bindCount: uint32_t,
    pub pBinds: *const SparseMemoryBind
}

impl Default for SparseBufferMemoryBindInfo {
    fn default() -> SparseBufferMemoryBindInfo {
        SparseBufferMemoryBindInfo {
            buffer: ptr::null_mut(),
            bindCount: 0,
            pBinds: ptr::null()
        }
    }
}

#[repr(C)]
#[allow(non_snake_case)]
#[derive(Copy, Clone)]
pub struct SparseImageOpaqueMemoryBindInfo {
    pub image: Image,
    pub bindCount: uint32_t,
    pub pBinds: *const SparseMemoryBind
}

impl Default for SparseImageOpaqueMemoryBindInfo {
    fn default() -> SparseImageOpaqueMemoryBindInfo {
        SparseImageOpaqueMemoryBindInfo {
            image: ptr::null_mut(),
            bindCount: 0,
            pBinds: ptr::null()
        }
    }
}

#[repr(C)]
#[allow(non_snake_case)]
#[derive(Copy, Clone)]
pub struct ImageSubresource {
    pub aspectMask: ImageAspectFlags,
    pub mipLevel: uint32_t,
    pub arrayLayer: uint32_t
}

impl Default for ImageSubresource {
    fn default() -> ImageSubresource {
        ImageSubresource {
            aspectMask: ImageAspectFlags::None,
            mipLevel: 0,
            arrayLayer: 0
        }
    }
}

#[repr(C)]
#[allow(non_snake_case)]
#[derive(Copy, Clone)]
pub struct Offset3D {
    pub x: int32_t,
    pub y: int32_t,
    pub z: int32_t
}

impl Default for Offset3D {
    fn default() -> Offset3D {
        Offset3D {
            x: 0,
            y: 0,
            z: 0
        }
    }
}

#[repr(C)]
#[allow(non_snake_case)]
#[derive(Copy, Clone)]
pub struct SparseImageMemoryBind {
    pub subresource: ImageSubresource,
    pub offset: Offset3D,
    pub extent: Offset3D,
    pub memory: DeviceMemory,
    pub memoryOffset: DeviceSize,
    pub flags: SparseMemoryBindFlags
}

impl Default for SparseImageMemoryBind {
    fn default() -> SparseImageMemoryBind {
        SparseImageMemoryBind {
            subresource: ImageSubresource{..Default::default()},
            offset: Offset3D{..Default::default()},
            extent: Offset3D{..Default::default()},
            memory: ptr::null_mut(),
            memoryOffset: 0,
            flags: SparseMemoryBindFlags::None
        }
    }
}

#[repr(C)]
#[allow(non_snake_case)]
#[derive(Copy, Clone)]
pub struct SparseImageMemoryBindInfo {
    pub image: Image,
    pub bindCount: uint32_t,
    pub pBinds: *const SparseImageMemoryBind
}

impl Default for SparseImageMemoryBindInfo {
    fn default() -> SparseImageMemoryBindInfo {
        SparseImageMemoryBindInfo {
            image: ptr::null_mut(),
            bindCount: 0,
            pBinds: ptr::null()
        }
    }
}

#[repr(C)]
#[allow(non_snake_case)]
#[derive(Copy, Clone)]
pub struct BindSparseInfo {
    pub sType: StructureType,
    pub pNext: *const c_void,
    pub waitSemaphoreCount: uint32_t,
    pub pWaitSemaphores: *const Semaphore,
    pub bufferBindCount: uint32_t,
    pub pBufferBinds: *const SparseBufferMemoryBindInfo,
    pub imageOpaqueBindCount: uint32_t,
    pub pImageOpaqueBinds: *const SparseImageOpaqueMemoryBindInfo,
    pub imageBindCount: uint32_t,
    pub pImageBinds: *const SparseImageMemoryBindInfo,
    pub signalSemaphoreCount: uint32_t,
    pub pSignalSemaphores: *const Semaphore
}

impl Default for BindSparseInfo {
    fn default() -> BindSparseInfo {
        BindSparseInfo {
            sType: StructureType::BindSparseInfo,
            pNext: ptr::null(),
            waitSemaphoreCount: 0,
            pWaitSemaphores: ptr::null(),
            bufferBindCount: 0,
            pBufferBinds: ptr::null(),
            imageOpaqueBindCount: 0,
            pImageOpaqueBinds: ptr::null(),
            imageBindCount: 0,
            pImageBinds: ptr::null(),
            signalSemaphoreCount: 0,
            pSignalSemaphores: ptr::null()
        }
    }
}

#[repr(C)]
#[allow(non_snake_case)]
#[derive(Copy, Clone)]
pub struct FenceCreateInfo {
    pub sType: StructureType,
    pub pNext: *const c_void,
    pub flags: FenceCreateFlags
}

impl Default for FenceCreateInfo {
    fn default() -> FenceCreateInfo {
        FenceCreateInfo {
            sType: StructureType::FenceCreateInfo,
            pNext: ptr::null(),
            flags: FenceCreateFlags::None
        }
    }
}

#[repr(C)]
#[allow(non_snake_case)]
#[derive(Copy, Clone)]
pub struct SemaphoreCreateInfo {
    pub sType: StructureType,
    pub pNext: *const c_void,
    pub flags: SemaphoreCreateFlags
}

impl Default for SemaphoreCreateInfo {
    fn default() -> SemaphoreCreateInfo {
        SemaphoreCreateInfo {
            sType: StructureType::SemaphoreCreateInfo,
            pNext: ptr::null(),
            flags: SemaphoreCreateFlags::None
        }
    }
}

#[repr(C)]
#[allow(non_snake_case)]
#[derive(Copy, Clone)]
pub struct EventCreateInfo {
    pub sType: StructureType,
    pub pNext: *const c_void,
    pub flags: EventCreateFlags
}

impl Default for EventCreateInfo {
    fn default() -> EventCreateInfo {
        EventCreateInfo {
            sType: StructureType::EventCreateInfo,
            pNext: ptr::null(),
            flags: EventCreateFlags::None
        }
    }
}

#[repr(C)]
#[allow(non_snake_case)]
#[derive(Copy, Clone)]
pub struct QueryPoolCreateInfo {
    pub sType: StructureType,
    pub pNext: *const c_void,
    pub flags: QueryPoolCreateFlags,
    pub queryType: QueryType,
    pub queryCount: uint32_t,
    pub pipelineStatistics: QueryPipelineStatisticFlags
}

impl Default for QueryPoolCreateInfo {
    fn default() -> QueryPoolCreateInfo {
        QueryPoolCreateInfo {
            sType: StructureType::QueryPoolCreateInfo,
            pNext: ptr::null(),
            flags: QueryPoolCreateFlags::None,
            queryType: QueryType::Occlusion,
            queryCount: 0,
            pipelineStatistics: QueryPipelineStatisticFlags::None
        }
    }
}

#[repr(C)]
#[allow(non_snake_case)]
#[derive(Copy, Clone)]
pub struct BufferCreateInfo {
    pub sType: StructureType,
    pub pNext: *const c_void,
    pub flags: BufferCreateFlags,
    pub size: DeviceSize,
    pub usage: BufferUsageFlags,
    pub sharingMode: SharingMode,
    pub queueFamilyIndexCount: uint32_t,
    pub pQueueFamilyIndices: *const uint32_t
}

impl Default for BufferCreateInfo {
    fn default() -> BufferCreateInfo {
        BufferCreateInfo {
            sType: StructureType::BufferCreateInfo,
            pNext: ptr::null(),
            flags: BufferCreateFlags::None,
            size: 0,
            usage: BufferUsageFlags::None,
            sharingMode: SharingMode::Exclusive,
            queueFamilyIndexCount: 0,
            pQueueFamilyIndices: ptr::null()
        }
    }
}

#[repr(C)]
#[allow(non_snake_case)]
#[derive(Copy, Clone)]
pub struct BufferViewCreateInfo {
    pub sType: StructureType,
    pub pNext: *const c_void,
    pub flags: BufferViewCreateFlags,
    pub buffer: Buffer,
    pub format: Format,
    pub offset: DeviceSize,
    pub range: DeviceSize
}

impl Default for BufferViewCreateInfo {
    fn default() -> BufferViewCreateInfo {
        BufferViewCreateInfo {
            sType: StructureType::BufferViewCreateInfo,
            pNext: ptr::null(),
            flags: BufferViewCreateFlags::None,
            buffer: ptr::null_mut(),
            format: Format::Undefined,
            offset: 0,
            range: 0
        }
    }
}

#[repr(C)]
#[allow(non_snake_case)]
#[derive(Copy, Clone)]
pub struct ImageCreateInfo {
    pub sType: StructureType,
    pub pNext: *const c_void,
    pub flags: ImageCreateFlags,
    pub imageType: ImageType,
    pub format: Format,
    pub extent: Extent3D,
    pub mipLevels: uint32_t,
    pub arrayLayers: uint32_t,
    pub samples: SampleCountFlags,
    pub tiling: ImageTiling,
    pub usage: ImageUsageFlags,
    pub sharingMode: SharingMode,
    pub queueFamilyIndexCount: uint32_t,
    pub pQueueFamilyIndices: *const uint32_t,
    pub initialLayout: ImageLayout
}

impl Default for ImageCreateInfo {
    fn default() -> ImageCreateInfo {
        ImageCreateInfo {
            sType: StructureType::ImageCreateInfo,
            pNext: ptr::null(),
            flags: ImageCreateFlags::None,
            imageType: ImageType::Type1D,
            format: Format::Undefined,
            extent: Extent3D{..Default::default()},
            mipLevels: 0,
            arrayLayers: 0,
            samples: SampleCountFlags::Count0,
            tiling: ImageTiling::Optimal,
            usage: ImageUsageFlags::None,
            sharingMode: SharingMode::Exclusive,
            queueFamilyIndexCount: 0,
            pQueueFamilyIndices: ptr::null(),
            initialLayout: ImageLayout::Undefined
        }
    }
}

#[repr(C)]
#[allow(non_snake_case)]
#[derive(Copy, Clone)]
pub struct SubresourceLayout {
    pub offset: DeviceSize,
    pub size: DeviceSize,
    pub rowPitch: DeviceSize,
    pub arrayPitch: DeviceSize,
    pub depthPitch: DeviceSize
}

impl Default for SubresourceLayout {
    fn default() -> SubresourceLayout {
        SubresourceLayout {
            offset: 0,
            size: 0,
            rowPitch: 0,
            arrayPitch: 0,
            depthPitch: 0
        }
    }
}

#[repr(C)]
#[allow(non_snake_case)]
#[derive(Copy, Clone)]
pub struct ComponentMapping {
    pub r: ComponentSwizzle,
    pub g: ComponentSwizzle,
    pub b: ComponentSwizzle,
    pub a: ComponentSwizzle
}

impl Default for ComponentMapping {
    fn default() -> ComponentMapping {
        ComponentMapping {
            r: ComponentSwizzle::Identity,
            g: ComponentSwizzle::Identity,
            b: ComponentSwizzle::Identity,
            a: ComponentSwizzle::Identity
        }
    }
}

#[repr(C)]
#[allow(non_snake_case)]
#[derive(Copy, Clone)]
pub struct ImageSubresourceRange {
    pub aspectMask: ImageAspectFlags,
    pub baseMipLevel: uint32_t,
    pub levelCount: uint32_t,
    pub baseArrayLayer: uint32_t,
    pub layerCount: uint32_t
}

impl Default for ImageSubresourceRange {
    fn default() -> ImageSubresourceRange {
        ImageSubresourceRange {
            aspectMask: ImageAspectFlags::None,
            baseMipLevel: 0,
            levelCount: 0,
            baseArrayLayer: 0,
            layerCount: 0
        }
    }
}

#[repr(C)]
#[allow(non_snake_case)]
#[derive(Copy, Clone)]
pub struct ImageViewCreateInfo {
    pub sType: StructureType,
    pub pNext: *const c_void,
    pub flags: ImageViewCreateFlags,
    pub image: Image,
    pub viewType: ImageViewType,
    pub format: Format,
    pub components: ComponentMapping,
    pub subresourceRange: ImageSubresourceRange
}

impl Default for ImageViewCreateInfo {
    fn default() -> ImageViewCreateInfo {
        ImageViewCreateInfo {
            sType: StructureType::ImageViewCreateInfo,
            pNext: ptr::null(),
            flags: ImageViewCreateFlags::None,
            image: ptr::null_mut(),
            viewType: ImageViewType::Type1D,
            format: Format::Undefined,
            components: ComponentMapping{..Default::default()},
            subresourceRange: ImageSubresourceRange{..Default::default()}
        }
    }
}

#[repr(C)]
#[allow(non_snake_case)]
#[derive(Copy, Clone)]
pub struct ShaderModuleCreateInfo {
    pub sType: StructureType,
    pub pNext: *const c_void,
    pub flags: ShaderModuleCreateFlags,
    pub codeSize: size_t,
    pub pCode: *const uint32_t
}

impl Default for ShaderModuleCreateInfo {
    fn default() -> ShaderModuleCreateInfo {
        ShaderModuleCreateInfo {
            sType: StructureType::ShaderModuleCreateInfo,
            pNext: ptr::null(),
            flags: ShaderModuleCreateFlags::None,
            codeSize: 0,
            pCode: ptr::null()
        }
    }
}

#[repr(C)]
#[allow(non_snake_case)]
#[derive(Copy, Clone)]
pub struct PipelineCacheCreateInfo {
    pub sType: StructureType,
    pub pNext: *const c_void,
    pub flags: PipelineCacheCreateFlags,
    pub initialDataSize: size_t,
    pub pInitialData: *const c_void
}

impl Default for PipelineCacheCreateInfo {
    fn default() -> PipelineCacheCreateInfo {
        PipelineCacheCreateInfo {
            sType: StructureType::PipelineCacheCreateInfo,
            pNext: ptr::null(),
            flags: PipelineCacheCreateFlags::None,
            initialDataSize: 0,
            pInitialData: ptr::null()
        }
    }
}

#[repr(C)]
#[allow(non_snake_case)]
#[derive(Copy, Clone)]
pub struct SpecializationMapEntry {
    pub constantID: uint32_t,
    pub offset: uint32_t,
    pub size: size_t
}

impl Default for SpecializationMapEntry {
    fn default() -> SpecializationMapEntry {
        SpecializationMapEntry {
            constantID: 0,
            offset: 0,
            size: 0
        }
    }
}

#[repr(C)]
#[allow(non_snake_case)]
#[derive(Copy, Clone)]
pub struct SpecializationInfo {
    pub mapEntryCount: uint32_t,
    pub pMapEntries: *const SpecializationMapEntry,
    pub dataSize: size_t,
    pub pData: *const c_void
}

impl Default for SpecializationInfo {
    fn default() -> SpecializationInfo {
        SpecializationInfo {
            mapEntryCount: 0,
            pMapEntries: ptr::null(),
            dataSize: 0,
            pData: ptr::null()
        }
    }
}

#[repr(C)]
#[allow(non_snake_case)]
#[derive(Copy, Clone)]
pub struct PipelineShaderStageCreateInfo {
    pub sType: StructureType,
    pub pNext: *const c_void,
    pub flags: PipelineShaderStageCreateFlags,
    pub stage: ShaderStageFlags,
    pub module: ShaderModule,
    pub pName: *const c_char,
    pub pSpecializationInfo: *const SpecializationInfo
}

impl Default for PipelineShaderStageCreateInfo {
    fn default() -> PipelineShaderStageCreateInfo {
        PipelineShaderStageCreateInfo {
            sType: StructureType::PipelineShaderStageCreateInfo,
            pNext: ptr::null(),
            flags: PipelineShaderStageCreateFlags::None,
            stage: ShaderStageFlags::None,
            module: ptr::null_mut(),
            pName: ptr::null(),
            pSpecializationInfo: ptr::null()
        }
    }
}

#[repr(C)]
#[allow(non_snake_case)]
#[derive(Copy, Clone)]
pub struct VertexInputBindingDescription {
    pub binding: uint32_t,
    pub stride: uint32_t,
    pub inputRate: VertexInputRate
}

impl Default for VertexInputBindingDescription {
    fn default() -> VertexInputBindingDescription {
        VertexInputBindingDescription {
            binding: 0,
            stride: 0,
            inputRate: VertexInputRate::Vertex
        }
    }
}

#[repr(C)]
#[allow(non_snake_case)]
#[derive(Copy, Clone)]
pub struct VertexInputAttributeDescription {
    pub location: uint32_t,
    pub binding: uint32_t,
    pub format: Format,
    pub offset: uint32_t,
}

impl Default for VertexInputAttributeDescription {
    fn default() -> VertexInputAttributeDescription {
        VertexInputAttributeDescription {
            location: 0,
            binding: 0,
            format: Format::Undefined,
            offset: 0
        }
    }
}

#[repr(C)]
#[allow(non_snake_case)]
#[derive(Copy, Clone)]
pub struct PipelineVertexInputStateCreateInfo {
    pub sType: StructureType,
    pub pNext: *const c_void,
    pub flags: PipelineVertexInputStateCreateFlags,
    pub vertexBindingDescriptionCount: uint32_t,
    pub pVertexBindingDescriptions: *const VertexInputBindingDescription,
    pub vertexAttributeDescriptionCount: uint32_t,
    pub pVertexAttributeDescriptions: *const VertexInputAttributeDescription
}

impl Default for PipelineVertexInputStateCreateInfo {
    fn default() -> PipelineVertexInputStateCreateInfo {
        PipelineVertexInputStateCreateInfo {
            sType: StructureType::PipelineVertexInputStateCreateInfo,
            pNext: ptr::null(),
            flags: PipelineVertexInputStateCreateFlags::None,
            vertexBindingDescriptionCount: 0,
            pVertexBindingDescriptions: ptr::null(),
            vertexAttributeDescriptionCount: 0,
            pVertexAttributeDescriptions: ptr::null()
        }
    }
}

#[repr(C)]
#[allow(non_snake_case)]
#[derive(Copy, Clone)]
pub struct PipelineInputAssemblyStateCreateInfo {
    pub sType: StructureType,
    pub pNext: *const c_void,
    pub flags: PipelineInputAssemblyStateCreateFlags,
    pub topology: PrimitiveTopology,
    pub primitiveRestartEnable: Bool32
}

impl Default for PipelineInputAssemblyStateCreateInfo {
    fn default() -> PipelineInputAssemblyStateCreateInfo {
        PipelineInputAssemblyStateCreateInfo {
            sType: StructureType::PipelineInputAssemblyStateCreateInfo,
            pNext: ptr::null(),
            flags: PipelineInputAssemblyStateCreateFlags::None,
            topology: PrimitiveTopology::PointList,
            primitiveRestartEnable: 0
        }
    }
}

#[repr(C)]
#[allow(non_snake_case)]
#[derive(Copy, Clone)]
pub struct PipelineTessellationStateCreateInfo {
    pub sType: StructureType,
    pub pNext: *const c_void,
    pub flags: PipelineTessellationStateCreateFlags,
    pub patchControlPoints: uint32_t
}

impl Default for PipelineTessellationStateCreateInfo {
    fn default() -> PipelineTessellationStateCreateInfo {
        PipelineTessellationStateCreateInfo {
            sType: StructureType::PipelineTessellationStateCreateInfo,
            pNext: ptr::null(),
            flags: PipelineTessellationStateCreateFlags::None,
            patchControlPoints: 0
        }
    }
}

#[repr(C)]
#[allow(non_snake_case)]
#[derive(Copy, Clone)]
pub struct Viewport {
    pub x: c_float,
    pub y: c_float,
    pub width: c_float,
    pub height: c_float,
    pub minDepth: c_float,
    pub maxDepth: c_float
}

impl Default for Viewport {
    fn default() -> Viewport {
        Viewport {
            x: 0.0,
            y: 0.0,
            width: 0.0,
            height: 0.0,
            minDepth: 0.0,
            maxDepth: 0.0
        }
    }
}

#[repr(C)]
#[allow(non_snake_case)]
#[derive(Copy, Clone)]
pub struct Offset2D {
    pub x: int32_t,
    pub y: int32_t
}

impl Default for Offset2D {
    fn default() -> Offset2D {
        Offset2D {
            x: 0,
            y: 0
        }
    }
}

#[repr(C)]
#[allow(non_snake_case)]
#[derive(Copy, Clone)]
pub struct Extent2D {
    pub width: uint32_t,
    pub height: uint32_t
}

impl Default for Extent2D {
    fn default() -> Extent2D {
        Extent2D {
            width: 0,
            height: 0
        }
    }
}

#[repr(C)]
#[allow(non_snake_case)]
#[derive(Copy, Clone)]
pub struct Rect2D {
    pub offset: Offset2D,
    pub extent: Extent2D
}

impl Default for Rect2D {
    fn default() -> Rect2D {
        Rect2D {
            offset: Offset2D{..Default::default()},
            extent: Extent2D{..Default::default()}
        }
    }
}

#[repr(C)]
#[allow(non_snake_case)]
#[derive(Copy, Clone)]
pub struct PipelineViewportStateCreateInfo {
    pub sType: StructureType,
    pub pNext: *const c_void,
    pub flags: PipelineViewportStateCreateFlags,
    pub viewportCount: uint32_t,
    pub pViewports: *const Viewport,
    pub scissorCount: uint32_t,
    pub pScissors: *const Rect2D
}

impl Default for PipelineViewportStateCreateInfo {
    fn default() -> PipelineViewportStateCreateInfo {
        PipelineViewportStateCreateInfo {
            sType: StructureType::PipelineViewportStateCreateInfo,
            pNext: ptr::null(),
            flags: PipelineViewportStateCreateFlags::None,
            viewportCount: 0,
            pViewports: ptr::null(),
            scissorCount: 0,
            pScissors: ptr::null()
        }
    }
}

#[repr(C)]
#[allow(non_snake_case)]
#[derive(Copy, Clone)]
pub struct PipelineRasterizationStateCreateInfo {
    pub sType: StructureType,
    pub pNext: *const c_void,
    pub flags: PipelineRasterizationStateCreateFlags,
    pub depthClampEnable: Bool32,
    pub rasterizerDiscardEnable: Bool32,
    pub polygonMode: PolygonMode,
    pub cullMode: CullModeFlags,
    pub frontFace: FrontFace,
    pub depthBiasEnable: Bool32,
    pub depthBiasConstantFactor: c_float,
    pub depthBiasClamp: c_float,
    pub depthSlopeFactor: c_float,
    pub lineWidth: c_float
}

impl Default for PipelineRasterizationStateCreateInfo {
    fn default() -> PipelineRasterizationStateCreateInfo {
        PipelineRasterizationStateCreateInfo {
            sType: StructureType::PipelineRasterizationStateCreateInfo,
            pNext: ptr::null(),
            flags: PipelineRasterizationStateCreateFlags::None,
            depthClampEnable: 0,
            rasterizerDiscardEnable: 0,
            polygonMode: PolygonMode::Fill,
            cullMode: CullModeFlags::None,
            frontFace: FrontFace::CounterClockwise,
            depthBiasEnable: 0,
            depthBiasConstantFactor: 0.0,
            depthBiasClamp: 0.0,
            depthSlopeFactor: 0.0,
            lineWidth: 0.0
        }
    }
}

#[repr(C)]
#[allow(non_snake_case)]
#[derive(Copy, Clone)]
pub struct PipelineMultisampleStateCreateInfo {
    pub sType: StructureType,
    pub pNext: *const c_void,
    pub flags: PipelineMultisampleStateCreateFlags,
    pub rasterizationSamples: SampleCountFlags,
    pub sampleShadingEnable: Bool32,
    pub minSampleShading: c_float,
    pub pSampleMask: *const SampleMask,
    pub alphaToCoverageEnable: Bool32,
    pub alphaToOneEnable: Bool32
}

impl Default for PipelineMultisampleStateCreateInfo {
    fn default() -> PipelineMultisampleStateCreateInfo {
        PipelineMultisampleStateCreateInfo {
            sType: StructureType::PipelineMultisampleStateCreateInfo,
            pNext: ptr::null(),
            flags: PipelineMultisampleStateCreateFlags::None,
            rasterizationSamples: SampleCountFlags::Count0,
            sampleShadingEnable: 0,
            minSampleShading: 0.0,
            pSampleMask: ptr::null(),
            alphaToCoverageEnable: 0,
            alphaToOneEnable: 0
        }
    }
}

#[repr(C)]
#[allow(non_snake_case)]
#[derive(Copy, Clone)]
pub struct StencilOpState {
    pub failOp: StencilOp,
    pub passOp: StencilOp,
    pub depthFailOp: StencilOp,
    pub compareOp: CompareOp,
    pub compareMask: uint32_t,
    pub writeMask: uint32_t,
    pub reference: uint32_t
}

impl Default for StencilOpState {
    fn default() -> StencilOpState {
        StencilOpState {
            failOp: StencilOp::Keep,
            passOp: StencilOp::Keep,
            depthFailOp: StencilOp::Keep,
            compareOp: CompareOp::Never,
            compareMask: 0,
            writeMask: 0,
            reference: 0
        }
    }
}

#[repr(C)]
#[allow(non_snake_case)]
#[derive(Copy, Clone)]
pub struct PipelineDepthStencilStateCreateInfo {
    pub sType: StructureType,
    pub pNext: *const c_void,
    pub flags: PipelineDepthStencilStateCreateFlags,
    pub depthTestsEnable: Bool32,
    pub depthWriteEnable: Bool32,
    pub depthCompareOp: CompareOp,
    pub depthBoundsTestEnable: Bool32,
    pub stencilTestEnable: Bool32,
    pub front: StencilOpState,
    pub back: StencilOpState,
    pub minDepthBounds: c_float,
    pub maxDepthBounds: c_float
}

impl Default for PipelineDepthStencilStateCreateInfo {
    fn default() -> PipelineDepthStencilStateCreateInfo {
        PipelineDepthStencilStateCreateInfo {
            sType: StructureType::PipelineDepthStencilStateCreateInfo,
            pNext: ptr::null(),
            flags: PipelineDepthStencilStateCreateFlags::None,
            depthTestsEnable: 0,
            depthWriteEnable: 0,
            depthCompareOp: CompareOp::Never,
            depthBoundsTestEnable: 0,
            stencilTestEnable: 0,
            front: StencilOpState{..Default::default()},
            back: StencilOpState{..Default::default()},
            minDepthBounds: 0.0,
            maxDepthBounds: 0.0
        }
    }
}

#[repr(C)]
#[allow(non_snake_case)]
#[derive(Copy, Clone)]
pub struct PipelineColorBlendAttachmentState {
    pub blendEnable: Bool32,
    pub srcColorBlendFactor: BlendFactor,
    pub dstColorBlendFactor: BlendFactor,
    pub colorBlendOp: BlendOp,
    pub srcAlphaBlendFactor: BlendFactor,
    pub dstAlphaBlendFactor: BlendFactor,
    pub alphaBlendOp: BlendOp,
    pub colorWriteMask: ColorComponentFlags
}

impl Default for PipelineColorBlendAttachmentState {
    fn default() -> PipelineColorBlendAttachmentState {
        PipelineColorBlendAttachmentState {
            blendEnable: 0,
            srcColorBlendFactor: BlendFactor::Zero,
            dstColorBlendFactor: BlendFactor::Zero,
            colorBlendOp: BlendOp::Add,
            srcAlphaBlendFactor: BlendFactor::Zero,
            dstAlphaBlendFactor: BlendFactor::Zero,
            alphaBlendOp: BlendOp::Add,
            colorWriteMask: ColorComponentFlags::None
        }
    }
}

#[repr(C)]
#[allow(non_snake_case)]
#[derive(Copy, Clone)]
pub struct PipelineColorBlendStateCreateInfo {
    pub sType: StructureType,
    pub pNext: *const c_void,
    pub flags: PipelineColorBlendStateCreateFlags,
    pub logicOpEnable: Bool32,
    pub logicOp: LogicOp,
    pub attachmentCount: uint32_t,
    pub pAttachments: *const PipelineColorBlendAttachmentState,
    pub blendConstants: [c_float; 4]
}

impl Default for PipelineColorBlendStateCreateInfo {
    fn default() -> PipelineColorBlendStateCreateInfo {
        PipelineColorBlendStateCreateInfo {
            sType: StructureType::PipelineColorBlendStateCreateInfo,
            pNext: ptr::null(),
            flags: PipelineColorBlendStateCreateFlags::None,
            logicOpEnable: 0,
            logicOp: LogicOp::Clear,
            attachmentCount: 0,
            pAttachments: ptr::null(),
            blendConstants: [0.0; 4]
        }
    }
}

#[repr(C)]
#[allow(non_snake_case)]
#[derive(Copy, Clone)]
pub struct PipelineDynamicStateCreateInfo {
    pub sType: StructureType,
    pub pNext: *const c_void,
    pub flags: PipelineDynamicStateCreateFlags,
    pub dynamicStateCount: uint32_t,
    pub pDynamicStates: *const DynamicState
}

impl Default for PipelineDynamicStateCreateInfo {
    fn default() -> PipelineDynamicStateCreateInfo {
        PipelineDynamicStateCreateInfo {
            sType: StructureType::PipelineDynamicStateCreateInfo,
            pNext: ptr::null(),
            flags: PipelineDynamicStateCreateFlags::None,
            dynamicStateCount: 0,
            pDynamicStates: ptr::null()
        }
    }
}

#[repr(C)]
#[allow(non_snake_case)]
#[derive(Copy, Clone)]
pub struct GraphicsPipelineCreateInfo {
    pub sType: StructureType,
    pub pNext: *const c_void,
    pub flags: PipelineCreateFlags,
    pub stageCount: uint32_t,
    pub pStages: *const PipelineShaderStageCreateInfo,
    pub pVertexInputState: *const PipelineVertexInputStateCreateInfo,
    pub pInputAssemblyState: *const PipelineInputAssemblyStateCreateInfo,
    pub pTessellationState: *const PipelineTessellationStateCreateInfo,
    pub pViewportState: *const PipelineViewportStateCreateInfo,
    pub pRasterizationState: *const PipelineRasterizationStateCreateInfo,
    pub pMultisampleState: *const PipelineMultisampleStateCreateInfo,
    pub pDepthStencilState: *const PipelineDepthStencilStateCreateInfo,
    pub pColorBlendState: *const PipelineColorBlendStateCreateInfo,
    pub pDynamicState: *const PipelineDynamicStateCreateInfo,
    pub layout: PipelineLayout,
    pub renderPass: RenderPass,
    pub subpass: uint32_t,
    pub basePipelineHandle: Pipeline,
    pub basePipelineIndex: int32_t
}

impl Default for GraphicsPipelineCreateInfo {
    fn default() -> GraphicsPipelineCreateInfo {
        GraphicsPipelineCreateInfo {
            sType: StructureType::GraphicsPipelineCreateInfo,
            pNext: ptr::null(),
            flags: PipelineCreateFlags::None,
            stageCount: 0,
            pStages: ptr::null(),
            pVertexInputState: ptr::null(),
            pInputAssemblyState: ptr::null(),
            pTessellationState: ptr::null(),
            pViewportState: ptr::null(),
            pRasterizationState: ptr::null(),
            pMultisampleState: ptr::null(),
            pDepthStencilState: ptr::null(),
            pColorBlendState: ptr::null(),
            pDynamicState: ptr::null(),
            layout: ptr::null_mut(),
            renderPass: ptr::null_mut(),
            subpass: 0,
            basePipelineHandle: ptr::null_mut(),
            basePipelineIndex: 0
        }
    }
}

#[repr(C)]
#[allow(non_snake_case)]
#[derive(Copy, Clone)]
pub struct ComputePipelineCreateInfo {
    pub sType: StructureType,
    pub pNext: *const c_void,
    pub flags: PipelineCreateFlags,
    pub stage: PipelineShaderStageCreateInfo,
    pub layout: PipelineLayout,
    pub basePipelineHandle: Pipeline,
    pub basePipelineIndex: int32_t
}

impl Default for ComputePipelineCreateInfo {
    fn default() -> ComputePipelineCreateInfo {
        ComputePipelineCreateInfo {
            sType: StructureType::ComputePipelineCreateInfo,
            pNext: ptr::null(),
            flags: PipelineCreateFlags::None,
            stage: PipelineShaderStageCreateInfo{..Default::default()},
            layout: ptr::null_mut(),
            basePipelineHandle: ptr::null_mut(),
            basePipelineIndex: 0
        }
    }
}

#[repr(C)]
#[allow(non_snake_case)]
#[derive(Copy, Clone)]
pub struct PushConstantRange {
    pub stageFlags: ShaderStageFlags,
    pub offset: uint32_t,
    pub size: uint32_t
}

impl Default for PushConstantRange {
    fn default() -> PushConstantRange {
        PushConstantRange {
            stageFlags: ShaderStageFlags::None,
            offset: 0,
            size: 0
        }
    }
}

#[repr(C)]
#[allow(non_snake_case)]
#[derive(Copy, Clone)]
pub struct PipelineLayoutCreateInfo {
    pub sType: StructureType,
    pub pNext: *const c_void,
    pub flags: PipelineLayoutCreateFlags,
    pub setLayoutCount: uint32_t,
    pub pSetLayouts: *const DescriptorSetLayout,
    pub pushConstantRangeCount: uint32_t,
    pub pPushConstantRanges: *const PushConstantRange
}

impl Default for PipelineLayoutCreateInfo {
    fn default() -> PipelineLayoutCreateInfo {
        PipelineLayoutCreateInfo {
            sType: StructureType::PipelineLayoutCreateInfo,
            pNext: ptr::null(),
            flags: PipelineLayoutCreateFlags::None,
            setLayoutCount: 0,
            pSetLayouts: ptr::null(),
            pushConstantRangeCount: 0,
            pPushConstantRanges: ptr::null()
        }
    }
}

#[repr(C)]
#[allow(non_snake_case)]
#[derive(Copy, Clone)]
pub struct SamplerCreateInfo {
    pub sType: StructureType,
    pub pNext: *const c_void,
    pub flags: SamplerCreateFlags,
    pub maxFilter: Filter,
    pub minFilter: Filter,
    pub mipmapMode: SamplerMipmapMode,
    pub addressModeU: SamplerAddressMode,
    pub addressModeV: SamplerAddressMode,
    pub addressModeW: SamplerAddressMode,
    pub mipLodBias: c_float,
    pub anisotropyEnable: Bool32,
    pub maxAnisotropy: c_float,
    pub compareEnable: Bool32,
    pub compareOp: CompareOp,
    pub minLod: c_float,
    pub maxLod: c_float,
    pub borderColor: BorderColor,
    pub unnormalizedCoordinates: Bool32
}

impl Default for SamplerCreateInfo {
    fn default() -> SamplerCreateInfo {
        SamplerCreateInfo {
            sType: StructureType::SamplerCreateInfo,
            pNext: ptr::null(),
            flags: SamplerCreateFlags::None,
            maxFilter: Filter::Nearest,
            minFilter: Filter::Nearest,
            mipmapMode: SamplerMipmapMode::Nearest,
            addressModeU: SamplerAddressMode::Repeat,
            addressModeV: SamplerAddressMode::Repeat,
            addressModeW: SamplerAddressMode::Repeat,
            mipLodBias: 0.0,
            anisotropyEnable: 0,
            maxAnisotropy: 0.0,
            compareEnable: 0,
            compareOp: CompareOp::Never,
            minLod: 0.0,
            maxLod: 0.0,
            borderColor: BorderColor::FloatTransparentBlack,
            unnormalizedCoordinates: 0
        }
    }
}

#[repr(C)]
#[allow(non_snake_case)]
#[derive(Copy, Clone)]
pub struct DescriptorSetLayoutBinding {
    pub binding: uint32_t,
    pub descriptorType: DescriptorType,
    pub descriptorCount: uint32_t,
    pub stageFlags: ShaderStageFlags,
    pub pImmutableSamples: *const Sampler
}

impl Default for DescriptorSetLayoutBinding {
    fn default() -> DescriptorSetLayoutBinding {
        DescriptorSetLayoutBinding {
            binding: 0,
            descriptorType: DescriptorType::Sampler,
            descriptorCount: 0,
            stageFlags: ShaderStageFlags::None,
            pImmutableSamples: ptr::null()
        }
    }
}

#[repr(C)]
#[allow(non_snake_case)]
#[derive(Copy, Clone)]
pub struct DescriptorSetLayoutCreateInfo {
    pub sType: StructureType,
    pub pNext: *const c_void,
    pub flags: DescriptorSetLayoutCreateFlags,
    pub bindingCount: uint32_t,
    pub pBindings: *const DescriptorSetLayoutBinding
}

impl Default for DescriptorSetLayoutCreateInfo {
    fn default() -> DescriptorSetLayoutCreateInfo {
        DescriptorSetLayoutCreateInfo {
            sType: StructureType::DescriptorSetLayoutCreateInfo,
            pNext: ptr::null(),
            flags: DescriptorSetLayoutCreateFlags::None,
            bindingCount: 0,
            pBindings: ptr::null()
        }
    }
}

#[repr(C)]
#[allow(non_snake_case)]
#[derive(Copy, Clone)]
pub struct DescriptorPoolSize {
    pub descriptorType: DescriptorType,
    pub descriptorCount: uint32_t
}

impl Default for DescriptorPoolSize {
    fn default() -> DescriptorPoolSize {
        DescriptorPoolSize {
            descriptorType: DescriptorType::Sampler,
            descriptorCount: 0
        }
    }
}

#[repr(C)]
#[allow(non_snake_case)]
#[derive(Copy, Clone)]
pub struct DescriptorPoolCreateInfo {
    pub sType: StructureType,
    pub pNext: *const c_void,
    pub flags: DescriptorPoolCreateFlags,
    pub maxSets: uint32_t,
    pub poolSizeCount: uint32_t,
    pub pPoolSizes: *const DescriptorPoolSize
}

impl Default for DescriptorPoolCreateInfo {
    fn default() -> DescriptorPoolCreateInfo {
        DescriptorPoolCreateInfo {
            sType: StructureType::DescriptorPoolCreateInfo,
            pNext: ptr::null(),
            flags: DescriptorPoolCreateFlags::None,
            maxSets: 0,
            poolSizeCount: 0,
            pPoolSizes: ptr::null()
        }
    }
}

#[repr(C)]
#[allow(non_snake_case)]
#[derive(Copy, Clone)]
pub struct DescriptorSetAllocateInfo {
    pub sType: StructureType,
    pub pNext: *const c_void,
    pub descriptorPool: DescriptorPool,
    pub descriptorSetCount: uint32_t,
    pub pSetLayouts: *const DescriptorSetLayout
}

impl Default for DescriptorSetAllocateInfo {
    fn default() -> DescriptorSetAllocateInfo {
        DescriptorSetAllocateInfo {
            sType: StructureType::DescriptorSetAllocateInfo,
            pNext: ptr::null(),
            descriptorPool: ptr::null_mut(),
            descriptorSetCount: 0,
            pSetLayouts: ptr::null()
        }
    }
}

#[repr(C)]
#[allow(non_snake_case)]
#[derive(Copy, Clone)]
pub struct DescriptorImageInfo {
    pub sampler: Sampler,
    pub imageView: ImageView,
    pub imageLayout: ImageLayout
}

impl Default for DescriptorImageInfo {
    fn default() -> DescriptorImageInfo {
        DescriptorImageInfo {
            sampler: ptr::null_mut(),
            imageView: ptr::null_mut(),
            imageLayout: ImageLayout::Undefined
        }
    }
}

#[repr(C)]
#[allow(non_snake_case)]
#[derive(Copy, Clone)]
pub struct DescriptorBufferInfo {
    pub buffer: Buffer,
    pub offset: DeviceSize,
    pub range: DeviceSize
}

impl Default for DescriptorBufferInfo {
    fn default() -> DescriptorBufferInfo {
        DescriptorBufferInfo {
            buffer: ptr::null_mut(),
            offset: 0,
            range: 0
        }
    }
}

#[repr(C)]
#[allow(non_snake_case)]
#[derive(Copy, Clone)]
pub struct WriteDescriptorSet {
    pub sType: StructureType,
    pub pNext: *const c_void,
    pub dstSet: DescriptorSet,
    pub dstBinding: uint32_t,
    pub dstArrayElement: uint32_t,
    pub descriptorCount: uint32_t,
    pub descriptorType: DescriptorType,
    pub pImageInfo: *const DescriptorImageInfo,
    pub pBufferInfo: *const DescriptorBufferInfo,
    pub pTexelBufferView: *const BufferView
}

impl Default for WriteDescriptorSet {
    fn default() -> WriteDescriptorSet {
        WriteDescriptorSet {
            sType: StructureType::WriteDescriptorSet,
            pNext: ptr::null(),
            dstSet: ptr::null_mut(),
            dstBinding: 0,
            dstArrayElement: 0,
            descriptorCount: 0,
            descriptorType: DescriptorType::Sampler,
            pImageInfo: ptr::null(),
            pBufferInfo: ptr::null(),
            pTexelBufferView: ptr::null()
        }
    }
}

#[repr(C)]
#[allow(non_snake_case)]
#[derive(Copy, Clone)]
pub struct CopyDescriptorSet {
    pub sType: StructureType,
    pub pNext: *const c_void,
    pub srcSet: DescriptorSet,
    pub srcBinding: uint32_t,
    pub srcArrayElement: uint32_t,
    pub dstSet: DescriptorSet,
    pub dstBinding: uint32_t,
    pub dstArrayElement: uint32_t,
    pub descriptorCount: uint32_t
}

impl Default for CopyDescriptorSet {
    fn default() -> CopyDescriptorSet {
        CopyDescriptorSet {
            sType: StructureType::CopyDescriptorSet,
            pNext: ptr::null(),
            srcSet: ptr::null_mut(),
            srcBinding: 0,
            srcArrayElement: 0,
            dstSet: ptr::null_mut(),
            dstBinding: 0,
            dstArrayElement: 0,
            descriptorCount: 0
        }
    }
}

#[repr(C)]
#[allow(non_snake_case)]
#[derive(Copy, Clone)]
pub struct FramebufferCreateInfo {
    pub sType: StructureType,
    pub pNext: *const c_void,
    pub flags: FramebufferCreateFlags,
    pub renderPass: RenderPass,
    pub attachmentCount: uint32_t,
    pub pAttachments: *const ImageView,
    pub width: uint32_t,
    pub height: uint32_t,
    pub layers: uint32_t
}

impl Default for FramebufferCreateInfo {
    fn default() -> FramebufferCreateInfo {
        FramebufferCreateInfo {
            sType: StructureType::FramebufferCreateInfo,
            pNext: ptr::null(),
            flags: FramebufferCreateFlags::None,
            renderPass: ptr::null_mut(),
            attachmentCount: 0,
            pAttachments: ptr::null(),
            width: 0,
            height: 0,
            layers: 0
        }
    }
}

#[repr(C)]
#[allow(non_snake_case)]
#[derive(Copy, Clone)]
pub struct AttachmentDescription {
    pub flags: AttachmentDescriptionFlags,
    pub format: Format,
    pub samples: SampleCountFlags,
    pub loadOp: AttachmentLoadOp,
    pub storeOp: AttachmentStoreOp,
    pub stencilLoadOp: AttachmentLoadOp,
    pub stencilStoreOp: AttachmentStoreOp,
    pub initialLayout: ImageLayout,
    pub finalLayout: ImageLayout
}

impl Default for AttachmentDescription {
    fn default() -> AttachmentDescription {
        AttachmentDescription {
            flags: AttachmentDescriptionFlags::None,
            format: Format::Undefined,
            samples: SampleCountFlags::Count0,
            loadOp: AttachmentLoadOp::Load,
            storeOp: AttachmentStoreOp::Store,
            stencilLoadOp: AttachmentLoadOp::Load,
            stencilStoreOp: AttachmentStoreOp::Store,
            initialLayout: ImageLayout::Undefined,
            finalLayout: ImageLayout::Undefined
        }
    }
}

#[repr(C)]
#[allow(non_snake_case)]
#[derive(Copy, Clone)]
pub struct AttachmentReference {
    pub attachment: uint32_t,
    pub layout: ImageLayout
}

impl Default for AttachmentReference {
    fn default() -> AttachmentReference {
        AttachmentReference {
            attachment: 0,
            layout: ImageLayout::Undefined
        }
    }
}

#[repr(C)]
#[allow(non_snake_case)]
#[derive(Copy, Clone)]
pub struct SubpassDescription {
    pub flags: SubpassDescriptionFlags,
    pub pipelineBindPoint: PipelineBindPoint,
    pub inputAttachmentCount: uint32_t,
    pub pInputAttachments: *const AttachmentReference,
    pub colorAttachmentCount: uint32_t,
    pub pColorAttachments: *const AttachmentReference,
    pub pResolveAttachments: *const AttachmentReference,
    pub pDepthStencilAttachment: *const AttachmentReference,
    pub preserveAttachmentCount: uint32_t,
    pub pPreserveAttachments: *const uint32_t
}

impl Default for SubpassDescription {
    fn default() -> SubpassDescription {
        SubpassDescription {
            flags: SubpassDescriptionFlags::None,
            pipelineBindPoint: PipelineBindPoint::Graphics,
            inputAttachmentCount: 0,
            pInputAttachments: ptr::null(),
            colorAttachmentCount: 0,
            pColorAttachments: ptr::null(),
            pResolveAttachments: ptr::null(),
            pDepthStencilAttachment: ptr::null(),
            preserveAttachmentCount: 0,
            pPreserveAttachments: ptr::null()
        }
    }
}

#[repr(C)]
#[allow(non_snake_case)]
#[derive(Copy, Clone)]
pub struct SubpassDependency {
    pub srcSubpass: uint32_t,
    pub dstSubpass: uint32_t,
    pub srcStageMask: PipelineStageFlags,
    pub dstStageMask: PipelineStageFlags,
    pub srcAccessMask: AccessFlags,
    pub dstAccessMask: AccessFlags,
    pub dependencyFlags: DependencyFlags
}

impl Default for SubpassDependency {
    fn default() -> SubpassDependency {
        SubpassDependency {
            srcSubpass: 0,
            dstSubpass: 0,
            srcStageMask: PipelineStageFlags::None,
            dstStageMask: PipelineStageFlags::None,
            srcAccessMask: AccessFlags::None,
            dstAccessMask: AccessFlags::None,
            dependencyFlags: DependencyFlags::None
        }
    }
}

#[repr(C)]
#[allow(non_snake_case)]
#[derive(Copy, Clone)]
pub struct RenderPassCreateInfo {
    pub sType: StructureType,
    pub pNext: *const c_void,
    pub flags: RenderPassCreateFlags,
    pub attachmentCount: uint32_t,
    pub pAttachments: *const AttachmentDescription,
    pub subpassCount: uint32_t,
    pub pSubpasses: *const SubpassDescription,
    pub dependencyCount: uint32_t,
    pub pDependencies: *const SubpassDependency
}

impl Default for RenderPassCreateInfo {
    fn default() -> RenderPassCreateInfo {
        RenderPassCreateInfo {
            sType: StructureType::RenderPassCreateInfo,
            pNext: ptr::null(),
            flags: RenderPassCreateFlags::None,
            attachmentCount: 0,
            pAttachments: ptr::null(),
            subpassCount: 0,
            pSubpasses: ptr::null(),
            dependencyCount: 0,
            pDependencies: ptr::null()
        }
    }
}

#[repr(C)]
#[allow(non_snake_case)]
#[derive(Copy, Clone)]
pub struct CommandPoolCreateInfo {
    pub sType: StructureType,
    pub pNext: *const c_void,
    pub flags: CommandPoolCreateFlags,
    pub queueFamilyIndex: uint32_t
}

impl Default for CommandPoolCreateInfo {
    fn default() -> CommandPoolCreateInfo {
        CommandPoolCreateInfo {
            sType: StructureType::CommandPoolCreateInfo,
            pNext: ptr::null(),
            flags: CommandPoolCreateFlags::None,
            queueFamilyIndex: 0
        }
    }
}

#[repr(C)]
#[allow(non_snake_case)]
#[derive(Copy, Clone)]
pub struct CommandBufferAllocateInfo {
    pub sType: StructureType,
    pub pNext: *const c_void,
    pub commandPool: CommandPool,
    pub level: CommandBufferLevel,
    pub commandBufferCount: uint32_t
}

impl Default for CommandBufferAllocateInfo {
    fn default() -> CommandBufferAllocateInfo {
        CommandBufferAllocateInfo {
            sType: StructureType::CommandBufferAllocateInfo,
            pNext: ptr::null(),
            commandPool: ptr::null_mut(),
            level: CommandBufferLevel::Primary,
            commandBufferCount: 0
        }
    }
}

#[repr(C)]
#[allow(non_snake_case)]
#[derive(Copy, Clone)]
pub struct CommandBufferInheritanceInfo {
    pub sType: StructureType,
    pub pNext: *const c_void,
    pub renderPass: RenderPass,
    pub subpass: uint32_t,
    pub framebuffer: Framebuffer,
    pub occlusionQueryEnable: Bool32,
    pub queryFlags: QueryControlFlags,
    pub pipelineStatistics: QueryPipelineStatisticFlags
}

impl Default for CommandBufferInheritanceInfo {
    fn default() -> CommandBufferInheritanceInfo {
        CommandBufferInheritanceInfo {
            sType: StructureType::CommandBufferInheritanceInfo,
            pNext: ptr::null(),
            renderPass: ptr::null_mut(),
            subpass: 0,
            framebuffer: ptr::null_mut(),
            occlusionQueryEnable: 0,
            queryFlags: QueryControlFlags::None,
            pipelineStatistics: QueryPipelineStatisticFlags::None
        }
    }
}

#[repr(C)]
#[allow(non_snake_case)]
#[derive(Copy, Clone)]
pub struct CommandBufferBeginInfo {
    pub sType: StructureType,
    pub pNext: *const c_void,
    pub flags: CommandBufferUsageFlags,
    pub pInheritanceInfo: *const CommandBufferInheritanceInfo
}

impl Default for CommandBufferBeginInfo {
    fn default() -> CommandBufferBeginInfo {
        CommandBufferBeginInfo {
            sType: StructureType::CommandBufferBeginInfo,
            pNext: ptr::null(),
            flags: CommandBufferUsageFlags::None,
            pInheritanceInfo: ptr::null()
        }
    }
}

#[repr(C)]
#[allow(non_snake_case)]
#[derive(Copy, Clone)]
pub struct BufferCopy {
    pub srcOffset: DeviceSize,
    pub dstOffset: DeviceSize,
    pub size: DeviceSize
}

impl Default for BufferCopy {
    fn default() -> BufferCopy {
        BufferCopy {
            srcOffset: 0,
            dstOffset: 0,
            size: 0
        }
    }
}

#[repr(C)]
#[allow(non_snake_case)]
#[derive(Copy, Clone)]
pub struct ImageSubresourceLayers {
    pub aspectMask: ImageAspectFlags,
    pub mipLevel: uint32_t,
    pub baseArrayLayer: uint32_t,
    pub layerCount: uint32_t
}

impl Default for ImageSubresourceLayers {
    fn default() -> ImageSubresourceLayers {
        ImageSubresourceLayers {
            aspectMask: ImageAspectFlags::None,
            mipLevel: 0,
            baseArrayLayer: 0,
            layerCount: 0
        }
    }
}

#[repr(C)]
#[allow(non_snake_case)]
#[derive(Copy, Clone)]
pub struct ImageCopy {
    pub srcSubresource: ImageSubresourceLayers,
    pub srcOffset: Offset3D,
    pub dstSubresource: ImageSubresourceLayers,
    pub dstOffset: Offset3D,
    pub extent: Extent3D
}

impl Default for ImageCopy {
    fn default() -> ImageCopy {
        ImageCopy {
            srcSubresource: ImageSubresourceLayers{..Default::default()},
            srcOffset: Offset3D{..Default::default()},
            dstSubresource: ImageSubresourceLayers{..Default::default()},
            dstOffset: Offset3D{..Default::default()},
            extent: Extent3D{..Default::default()}
        }
    }
}

#[repr(C)]
#[allow(non_snake_case)]
#[derive(Copy, Clone)]
pub struct ImageBlit {
    pub srcSubresource: ImageSubresourceLayers,
    pub srcOffsets: [Offset3D; 2],
    pub dstSubresource: ImageSubresourceLayers,
    pub dstOffsets: [Offset3D; 2]
}

impl Default for ImageBlit {
    fn default() -> ImageBlit {
        ImageBlit {
            srcSubresource: ImageSubresourceLayers{..Default::default()},
            srcOffsets: [Offset3D{..Default::default()}; 2],
            dstSubresource: ImageSubresourceLayers{..Default::default()},
            dstOffsets: [Offset3D{..Default::default()}; 2]
        }
    }
}

#[repr(C)]
#[allow(non_snake_case)]
#[derive(Copy, Clone)]
pub struct BufferImageCopy {
    pub bufferOffset: DeviceSize,
    pub bufferRowLength: uint32_t,
    pub bufferImageHeight: uint32_t,
    pub imageSubresource: ImageSubresourceLayers,
    pub imageOffset: Offset3D,
    pub imageExtent: Extent3D
}

impl Default for BufferImageCopy {
    fn default() -> BufferImageCopy {
        BufferImageCopy {
            bufferOffset: 0,
            bufferRowLength: 0,
            bufferImageHeight: 0,
            imageSubresource: ImageSubresourceLayers{..Default::default()},
            imageOffset: Offset3D{..Default::default()},
            imageExtent: Extent3D{..Default::default()}
        }
    }
}

// NOTE: This represents a union. This code should be rewritten when rust adds union support.
#[allow(non_snake_case)]
// Can be array of c_float, int32_t, or uint32_t.
pub type ClearColorValue = [uint32_t; 4];

#[repr(C)]
#[allow(non_snake_case)]
#[derive(Copy, Clone)]
pub struct ClearDepthStencilValue {
    pub depth: c_float,
    pub stencil: uint32_t
}

impl Default for ClearDepthStencilValue {
    fn default() -> ClearDepthStencilValue {
        ClearDepthStencilValue {
            depth: 0.0,
            stencil: 0
        }
    }
}

#[repr(C)]
#[allow(non_snake_case)]
#[derive(Copy, Clone)]
pub struct ClearValue {
    pub color: ClearColorValue,
    pub depthStencil: ClearDepthStencilValue
}

impl Default for ClearValue {
    fn default() -> ClearValue {
        ClearValue {
            color: [0; 4],
            depthStencil: ClearDepthStencilValue{..Default::default()}
        }
    }
}

#[repr(C)]
#[allow(non_snake_case)]
#[derive(Copy, Clone)]
pub struct ClearAttachment {
    pub aspectMask: ImageAspectFlags,
    pub colorAttachment: uint32_t,
    pub clearValue: ClearValue
}

impl Default for ClearAttachment {
    fn default() -> ClearAttachment {
        ClearAttachment {
            aspectMask: ImageAspectFlags::None,
            colorAttachment: 0,
            clearValue: ClearValue{..Default::default()}
        }
    }
}

#[repr(C)]
#[allow(non_snake_case)]
#[derive(Copy, Clone)]
pub struct ClearRect {
    pub rect: Rect2D,
    pub baseArrayLayer: uint32_t,
    pub layerCount: uint32_t
}

impl Default for ClearRect {
    fn default() -> ClearRect {
        ClearRect {
            rect: Rect2D{..Default::default()},
            baseArrayLayer: 0,
            layerCount: 0
        }
    }
}

#[repr(C)]
#[allow(non_snake_case)]
#[derive(Copy, Clone)]
pub struct ImageResolve {
    pub srcSubresource: ImageSubresourceLayers,
    pub srcOffset: Offset3D,
    pub dstSubresource: ImageSubresourceLayers,
    pub dstOffset: Offset3D,
    pub extent: Extent3D
}

impl Default for ImageResolve {
    fn default() -> ImageResolve {
        ImageResolve {
            srcSubresource: ImageSubresourceLayers{..Default::default()},
            srcOffset: Offset3D{..Default::default()},
            dstSubresource: ImageSubresourceLayers{..Default::default()},
            dstOffset: Offset3D{..Default::default()},
            extent: Extent3D{..Default::default()}
        }
    }
}

#[repr(C)]
#[allow(non_snake_case)]
#[derive(Copy, Clone)]
pub struct MemoryBarrier {
    pub sType: StructureType,
    pub pNext: *const c_void,
    pub srcAccessMask: AccessFlags,
    pub dstAccessMask: AccessFlags,
}

impl Default for MemoryBarrier {
    fn default() -> MemoryBarrier {
        MemoryBarrier {
            sType: StructureType::MemoryBarrier,
            pNext: ptr::null(),
            srcAccessMask: AccessFlags::None,
            dstAccessMask: AccessFlags::None
        }
    }
}

#[repr(C)]
#[allow(non_snake_case)]
#[derive(Copy, Clone)]
pub struct BufferMemoryBarrier {
    pub sType: StructureType,
    pub pNext: *const c_void,
    pub srcAccessMask: AccessFlags,
    pub dstAccessMask: AccessFlags,
    pub srcQueueFamilyIndex: uint32_t,
    pub dstQueueFamilyIndex: uint32_t,
    pub buffer: Buffer,
    pub offset: DeviceSize,
    pub size: DeviceSize
}

impl Default for BufferMemoryBarrier {
    fn default() -> BufferMemoryBarrier {
        BufferMemoryBarrier {
            sType: StructureType::BufferMemoryBarrier,
            pNext: ptr::null(),
            srcAccessMask: AccessFlags::None,
            dstAccessMask: AccessFlags::None,
            srcQueueFamilyIndex: 0,
            dstQueueFamilyIndex: 0,
            buffer: ptr::null_mut(),
            offset: 0,
            size: 0
        }
    }
}

#[repr(C)]
#[allow(non_snake_case)]
#[derive(Copy, Clone)]
pub struct ImageMemoryBarrier {
    pub sType: StructureType,
    pub pNext: *const c_void,
    pub srcAccessMask: AccessFlags,
    pub dstAccessMask: AccessFlags,
    pub oldLayout: ImageLayout,
    pub newLayout: ImageLayout,
    pub srcQueueFamilyIndex: uint32_t,
    pub dstQueueFamilyIndex: uint32_t,
    pub image: Image,
    pub subresourceRange: ImageSubresourceRange
}

impl Default for ImageMemoryBarrier {
    fn default() -> ImageMemoryBarrier {
        ImageMemoryBarrier {
            sType: StructureType::ImageMemoryBarrier,
            pNext: ptr::null(),
            srcAccessMask: AccessFlags::None,
            dstAccessMask: AccessFlags::None,
            oldLayout: ImageLayout::Undefined,
            newLayout: ImageLayout::Undefined,
            srcQueueFamilyIndex: 0,
            dstQueueFamilyIndex: 0,
            image: ptr::null_mut(),
            subresourceRange: ImageSubresourceRange{..Default::default()}
        }
    }
}

#[repr(C)]
#[allow(non_snake_case)]
#[derive(Copy, Clone)]
pub struct RenderPassBeginInfo {
    pub sType: StructureType,
    pub pNext: *const c_void,
    pub renderPass: RenderPass,
    pub framebuffer: Framebuffer,
    pub renderArea: Rect2D,
    pub clearValueCount: uint32_t,
    pub pClearValues: *const ClearValue
}

impl Default for RenderPassBeginInfo {
    fn default() -> RenderPassBeginInfo {
        RenderPassBeginInfo {
            sType: StructureType::RenderPassBeginInfo,
            pNext: ptr::null(),
            renderPass: ptr::null_mut(),
            framebuffer: ptr::null_mut(),
            renderArea: Rect2D{..Default::default()},
            clearValueCount: 0,
            pClearValues: ptr::null()
        }
    }
}

#[repr(C)]
#[allow(non_snake_case)]
#[derive(Copy, Clone)]
pub struct DispatchIndirectCommand {
    pub x: uint32_t,
    pub y: uint32_t,
    pub z: uint32_t,
}

impl Default for DispatchIndirectCommand {
    fn default() -> DispatchIndirectCommand {
        DispatchIndirectCommand {
            x: 0,
            y: 0,
            z: 0
        }
    }
}

#[repr(C)]
#[allow(non_snake_case)]
#[derive(Copy, Clone)]
pub struct DrawIndexedIndirectCommand {
    pub indexCount: uint32_t,
    pub instanceCount: uint32_t,
    pub firstIndex: uint32_t,
    pub vertexOffset: int32_t,
    pub firstInstance: uint32_t
}

impl Default for DrawIndexedIndirectCommand {
    fn default() -> DrawIndexedIndirectCommand {
        DrawIndexedIndirectCommand {
            indexCount: 0,
            instanceCount: 0,
            firstIndex: 0,
            vertexOffset: 0,
            firstInstance: 0
        }
    }
}

#[repr(C)]
#[allow(non_snake_case)]
#[derive(Copy, Clone)]
pub struct DrawIndirectCommand {
    pub vertexCount: uint32_t,
    pub instanceCount: uint32_t,
    pub firstVertex: uint32_t,
    pub firstInstance: uint32_t
}

impl Default for DrawIndirectCommand {
    fn default() -> DrawIndirectCommand {
        DrawIndirectCommand {
            vertexCount: 0,
            instanceCount: 0,
            firstVertex: 0,
            firstInstance: 0
        }
    }
}
