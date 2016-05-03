pub mod vk {
    extern crate libc;
    use libc::{uint8_t, uint32_t, uint64_t, int32_t, size_t, c_char, c_void, c_float};
    use std::default::Default;
    use std::fmt;
    use std::ptr;

    pub type Flags = uint32_t;

    pub type DeviceSize = uint64_t;
    pub type SampleMask = uint32_t;

    pub type Bool32 = uint32_t;

    #[repr(C)]
    pub enum Bool {
        False = 0,
        True = 1
    }

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

    // TODO: figure out some way to link these values
    const VK_MAX_PHYSICAL_DEVICE_NAME_SIZE: usize = 256;
    const VK_UUID_SIZE: usize = 16;
    const VK_MAX_MEMORY_TYPES: usize = 32;
    const VK_MAX_MEMORY_HEAPS: usize = 16;
    const VK_MAX_EXTENSION_NAME_SIZE: usize = 256;
    const VK_MAX_DESCRIPTION_SIZE: usize = 256;

    #[repr(C)]
    pub enum Result {
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

    impl ToString for Result {
        fn to_string(&self) -> String {
            match *self {
                Result::Success                     => "Success".to_string(),
                Result::NotReady                    => "Not Ready".to_string(),
                Result::Timeout                     => "Timeout".to_string(),
                Result::EventSet                    => "Event Set".to_string(),
                Result::EventReset                  => "Event Reset".to_string(),
                Result::Incomplete                  => "Incomplete".to_string(),
                Result::SuboptimalKHR               => "Suboptimal KHR".to_string(),
                Result::ErrorOutOfHostMemory        => "Error (Out of Host Memory".to_string(),
                Result::ErrorOutOfDeviceMemory      => "Error (Out of Device Memory)".to_string(),
                Result::ErrorInitializationFailed   => "Error (Intialization Failed)".to_string(),
                Result::ErrorDeviceLost             => "Error (Device Lost)".to_string(),
                Result::ErrorMemoryMapFailed        => "Error (Memory Map Failed)".to_string(),
                Result::ErrorLayerNotPresent        => "Error (Layer Not Present)".to_string(),
                Result::ErrorExtensionNotPresent    => "Error (Extension Not Present)".to_string(),
                Result::ErrorFeatureNotPresent      => "Error (Feature Not Present)".to_string(),
                Result::ErrorIncompatibleDriver     => "Error (Incompatible Driver)".to_string(),
                Result::ErrorTooManyObjects         => "Error (Too Many Objects)".to_string(),
                Result::ErrorFormatNotSupported     => "Error (Format Not Supported)".to_string(),
                Result::ErrorSurfaceLostKHR         => "Error (Surface Lost KHR)".to_string(),
                Result::ErrorNativeWindowInUseKHR   => "Error (Native Window in Use KHR)".to_string(),
                Result::ErrorOutOfDateKHR           => "Error (Out of Date KHR)".to_string(),
                Result::ErrorIncompatibleDisplayKHR => "Error (Incompaitble Display KHR)".to_string(),
                Result::ErrorValidationFailedExt    => "Error (Validation Vailed Ext)".to_string()
            }
        }
    }

    impl fmt::Debug for Result {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "{}", self.to_string())
        }
    }

    #[repr(C)]
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
        CommmandBufferInheritanceInfo = 41,
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
    pub enum SystemAllocationScope {
        Command = 0,
        Object = 1,
        Cache = 2,
        Device = 3,
        Instance = 4
    }

    #[repr(C)]
    pub enum InternalAllocationType {
        Executable = 0,
        Dummy = 1 // Dummy workaround for E0083
    }

    #[repr(C)]
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
    pub enum ImageType {
        Type1D = 0,
        Type2D = 1,
        Type3D = 2
    }

    #[repr(C)]
    pub enum ImageTiling {
        Optimal = 0,
        Linear = 1
    }

    #[repr(C)]
    pub enum PhysicalDeviceType {
        Other = 0,
        IntegratedGPU = 1,
        DiscreteGPU = 2,
        VirtualGPU = 3,
        CPU = 4,
    }

    #[repr(C)]
    pub enum QueryType {
        Occlusion = 0,
        PipelineStatistics = 1,
        Timestamp = 2
    }

    #[repr(C)]
    pub enum SharingMode {
        Exclusive = 0,
        Concurrent = 1
    }

    #[repr(C)]
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
    pub enum VertexInputRate {
        Vertex = 0,
        Instance = 1
    }

    #[repr(C)]
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
    pub enum PolygonMode {
        Fill = 0,
        Line = 1,
        Point = 2
    }

    #[repr(C)]
    pub enum FrontFace {
        CounterClockwise = 0,
        Clockwise = 1
    }

    #[repr(C)]
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
    pub enum BlendOp {
        Add = 0,
        Subtract = 1,
        ReverseSubtract = 2,
        Min = 3,
        Max = 4
    }

    #[repr(C)]
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
    pub enum Filter {
        Nearest = 0,
        Linear = 1
    }

    #[repr(C)]
    pub enum SamplerMipmapMode {
        Nearest = 0,
        Linear = 1
    }

    #[repr(C)]
    pub enum SamplerAddressMode {
        Repeat = 0,
        MirroredRepeat = 1,
        ClampToEdge = 2,
        ClampToBorder = 3,
        MirrorClampToEdge = 4
    }

    #[repr(C)]
    pub enum BorderColor {
        FloatTransparentBlack = 0,
        IntTransparentBlack = 1,
        FloatOpaqueBlack = 2,
        IntOpaqueBlack = 3,
        FloatOpaqueWhite = 4,
        IntOpaqueWhite = 5
    }

    #[repr(C)]
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
    pub enum AttachmentLoadOp {
        Load = 0,
        Clear = 1,
        DontCare = 2
    }

    #[repr(C)]
    pub enum AttachmentStoreOp {
        Store = 0,
        DontCare = 1
    }

    #[repr(C)]
    pub enum PipelineBindPoint {
        Graphics = 0,
        Compute = 1
    }

    #[repr(C)]
    pub enum CommandBufferLevel {
        Primary = 0,
        Secondary = 1
    }

    #[repr(C)]
    pub enum IndexType {
        Uint16 = 0,
        Uint32 = 1
    }

    #[repr(C)]
    pub enum SubpassContents {
        Inline = 0,
        SecondaryCommandBuffers = 1
    }

    #[repr(C)]
    pub enum FormatFeatureFlags {
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
    pub enum ImageCreateFlags {
        SparseBinding = 0x01,
        SparseResidency = 0x02,
        SparseAliased = 0x04,
        MutableFormat = 0x08,
        CubeCompatible = 0x10
    }

    #[repr(C)]
    pub enum ImageUsageFlags {
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
    pub enum SampleCountFlags {
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
    #[derive(Clone, Copy)]
    pub enum MemoryPropertyFlags {
        None = 0x00,
        DeviceLocal = 0x01,
        HostVisible = 0x02,
        HostCoherent = 0x04,
        HostCached = 0x08,
        LazilyAllocated = 0x10
    }

    #[repr(C)]
    #[derive(Clone, Copy)]
    pub enum MemoryHeapFlags {
        None = 0x00,
        DeviceLocal = 0x01
    }

    #[repr(C)]
    pub enum PipelineStageFlags {
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
    pub enum ImageAspectFlags {
        Color = 0x01,
        Depth = 0x02,
        Stencil = 0x04,
        Metadata = 0x08
    }

    #[repr(C)]
    pub enum SparseImageFormatFlags {
        SingleMiptail = 0x01,
        AlignedMipSize = 0x02,
        NonstandardBlockSize = 0x04
    }

    #[repr(C)]
    pub enum SparseMemoryBindFlags {
        None = 0x00,
        BindMetadata = 0x01
    }

    #[repr(C)]
    pub enum FenceCreateFlags {
        None = 0x00,
        Signaled = 0x01
    }

    #[repr(C)]
    pub enum QueryPipelineStatisticFlags {
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
    pub enum QueryResultFlags {
        Result64 = 0x01,
        ResultWait = 0x02,
        ResultWithAvailability = 0x04,
        ResultPartial = 0x08
    }

    #[repr(C)]
    pub enum BufferCreateFlags {
        SparseBinding = 0x01,
        SparseResidency = 0x02,
        SparseAliased = 0x04
    }

    #[repr(C)]
    pub enum BufferUsageFlags {
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
    pub enum PipelineCreateFlags {
        DisableOptimization = 0x01,
        AllowDerivatives = 0x02,
        Derivative = 0x04
    }

    #[repr(C)]
    pub enum ShaderStageFlags {
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
    pub enum CullModeFlags {
        None = 0x00,
        Front = 0x01,
        Back = 0x02,
        FrontAndBack = 0x03
    }

    #[repr(C)]
    pub enum ColorComponentFlags {
        R = 0x01,
        G = 0x02,
        B = 0x04,
        A = 0x08
    }

    #[repr(C)]
    pub enum DescriptorPoolCreateFlags {
        None = 0x00,
        FreeDescriptorSet = 0x01
    }

    #[repr(C)]
    pub enum AttachmentDescriptionFlags {
        None = 0x00,
        MayAlias = 0x01
    }

    #[repr(C)]
    pub enum AccessFlags {
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
    pub enum DependencyFlags {
        None = 0x00,
        ByRegion = 0x01
    }

    #[repr(C)]
    pub enum CommandPoolCreateFlags {
        Transient = 0x01,
        ResetCommandBuffer = 0x02
    }

    #[repr(C)]
    pub enum CommandPoolResetFlags {
        None = 0x00,
        ResetReleaseResources = 0x01
    }

    #[repr(C)]
    pub enum CommandBufferUsageFlags {
        OneTimeSubmit = 0x01,
        RenderPassContinue = 0x02,
        SimultaneousUse = 0x04
    }

    #[repr(C)]
    pub enum QueryControlFlags {
        None = 0x00,
        Precise = 0x01
    }

    #[repr(C)]
    pub enum CommandBufferResetFlags {
        None = 0x00,
        ResetReleaseResources = 0x01
    }

    #[repr(C)]
    pub enum StencilFaceFlags {
        Front = 0x01,
        Back = 0x02,
        FrontAndBack = 0x03
    }

    pub type DescriptorPoolResetFlags = Flags;
    pub type RenderPassCreateFlags = Flags;
    pub type SubpassDescriptionFlags = Flags;
    pub type FramebufferCreateFlags = Flags;
    pub type DeviceCreateFlags = Flags;
    pub type DeviceQueueCreateFlags = Flags;
    pub type MemoryMapFlags = Flags;
    pub type SemaphoreCreateFlags = Flags;
    pub type EventCreateFlags = Flags;
    pub type QueryPoolCreateFlags = Flags;
    pub type BufferViewCreateFlags = Flags;
    pub type ImageViewCreateFlags = Flags;
    pub type ShaderModuleCreateFlags = Flags;
    pub type PipelineCacheCreateFlags = Flags;
    pub type PipelineShaderStageCreateFlags = Flags;
    pub type PipelineVertexInputStateCreateFlags = Flags;
    pub type PipelineInputAssemblyStateCreateFlags = Flags;
    pub type PipelineTessellationStateCreateFlags = Flags;
    pub type PipelineViewportStateCreateFlags = Flags;
    pub type PipelineRasterizationStateCreateFlags = Flags;
    pub type PipelineMultisampleStateCreateFlags = Flags;
    pub type PipelineDepthStencilStateCreateFlags = Flags;
    pub type PipelineColorBlendStateCreateFlags = Flags;
    pub type PipelineDynamicStateCreateFlags = Flags;
    pub type PipelineLayoutCreateFlags = Flags;
    pub type SamplerCreateFlags = Flags;
    pub type DescriptorSetLayoutCreateFlags = Flags;

    // impl Iterator for *const c_char {
    //     type Item = c_char;
    // 
    //     fn next(&mut self) -> Option<Item> {
    //         unsafe {
    //             self += mem::sizeof::<c_char>;
    //             let c = *self;
    //             match c {
    //                 ptr::null_mut() => None,
    //                 _               => Some(c)
    //             }
    //         }
    //     }
    // }
    // 
    // impl fmt::Debug for *const c_char {
    //     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    //         let mut 
    //         let c: c_char
    // 
    //         for  {
    //             match 
    //         }
    // 
    //         write!(
    //             f,
    //         )
    //     }
    // }

    #[repr(C)]
    #[allow(non_snake_case)]
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
    //     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
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
    pub struct InstanceCreateInfo {
        pub sType: StructureType,
        pub pNext: *const c_void,
        pub flags: Flags,
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
                flags: 0,
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
    //     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
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
    pub struct AllocationCallbacks {
        pub pUserData: *mut c_void,
        pub pfnAllocation: AllocationFunction,
        pub pfnReallocation: ReallocationFunction,
        pub pfnFree: FreeFunction,
        pub pfnInternalAllocation: InternalAllocationNotificationFunction,
        pub pfnInternalFree: InternalFreeNotificationFunction
    }

    #[repr(C)]
    #[allow(non_snake_case)]
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

    #[repr(C)]
    #[allow(non_snake_case)]
    pub struct FormatProperties {
        pub linearTilingFeatures: FormatFeatureFlags,
        pub optimalTilintFeatures: FormatFeatureFlags,
        pub bufferFeatures: FormatFeatureFlags
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
    pub struct ImageFormatProperties {
        pub maxExtent: Extent3D,
        pub maxMipLevels: uint32_t,
        pub maxArrayLayers: uint32_t,
        pub sampleCounts: SampleCountFlags,
        pub maxResourceSize: DeviceSize
    }

    #[repr(C)]
    #[allow(non_snake_case)]
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

    #[repr(C)]
    #[allow(non_snake_case)]
    pub struct PhysicalDeviceSparseProperties {
        pub residencyStandard2DBlockShape: Bool32,
        pub residencyStandard2DMultisampleBlockShape: Bool32,
        pub residencyStandard3DBlockShape: Bool32,
        pub residencyAlignedMipSize: Bool32,
        pub residencyNonResidentStrict: Bool32
    }

    #[repr(C)]
    #[allow(non_snake_case)]
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
                flags: 0,
                queueFamilyIndex: 0,
                queueCount: 0,
                pQueuePriorities: ptr::null()
            }
        }
    }

    #[repr(C)]
    #[allow(non_snake_case)]
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
                flags: 0,
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
    pub struct ExtensionProperties {
        pub extensionName: [c_char; VK_MAX_EXTENSION_NAME_SIZE],
        pub specVersion: uint32_t
    }

    #[repr(C)]
    #[allow(non_snake_case)]
    pub struct LayerProperties {
        pub layerName: [c_char; VK_MAX_EXTENSION_NAME_SIZE],
        pub specVersion: uint32_t,
        pub implementationVersion: uint32_t,
        pub description: [c_char; VK_MAX_DESCRIPTION_SIZE]
    }

    #[repr(C)]
    #[allow(non_snake_case)]
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

    #[repr(C)]
    #[allow(non_snake_case)]
    pub struct MemoryAllocateInfo {
        pub sType: StructureType,
        pub pNext: *const c_void,
        pub allocationSize: DeviceSize,
        pub memoryTypeIndex: uint32_t
    }

    #[repr(C)]
    #[allow(non_snake_case)]
    pub struct MappedMemoryRange {
        pub sType: StructureType,
        pub pNext: *const c_void,
        pub memory: DeviceMemory,
        pub offset: DeviceSize,
        pub size: DeviceSize
    }

    #[repr(C)]
    #[allow(non_snake_case)]
    pub struct MemoryRequirements {
        pub size: DeviceSize,
        pub alignment: DeviceSize,
        pub memoryTypeBits: uint32_t
    }

    #[repr(C)]
    #[allow(non_snake_case)]
    pub struct SparseImageFormatProperties {
        pub aspectMask: ImageAspectFlags,
        pub imageGranularity: Extent3D,
        pub flags: SparseImageFormatFlags
    }

    #[repr(C)]
    #[allow(non_snake_case)]
    pub struct SparseImageMemoryRequirements {
        pub formatProperties: SparseImageFormatProperties,
        pub imageMipTailFirstLod: uint32_t,
        pub imageMipTailSize: DeviceSize,
        pub imageMipTailOffset: DeviceSize,
        pub imageMipTailStride: DeviceSize
    }

    #[repr(C)]
    #[allow(non_snake_case)]
    pub struct SparseMemoryBind {
        pub resourceOffset: DeviceSize,
        pub size: DeviceSize,
        pub memory: DeviceMemory,
        pub memoryOffset: DeviceSize,
        pub flags: SparseMemoryBindFlags
    }

    #[repr(C)]
    #[allow(non_snake_case)]
    pub struct SparseBufferMemoryBindInfo {
        pub buffer: Buffer,
        pub bindCount: uint32_t,
        pub pBinds: *const SparseMemoryBind
    }

    #[repr(C)]
    #[allow(non_snake_case)]
    pub struct SparseImageOpaqueMemoryBindInfo {
        pub image: Image,
        pub bindCount: uint32_t,
        pub pBinds: *const SparseMemoryBind
    }

    #[repr(C)]
    #[allow(non_snake_case)]
    pub struct ImageSubresource {
        pub aspectMask: ImageAspectFlags,
        pub mipLevel: uint32_t,
        pub arrayLayer: uint32_t
    }

    #[repr(C)]
    #[allow(non_snake_case)]
    pub struct Offset3D {
        pub x: int32_t,
        pub y: int32_t,
        pub z: int32_t
    }

    #[repr(C)]
    #[allow(non_snake_case)]
    pub struct SparseImageMemoryBind {
        pub subresource: ImageSubresource,
        pub offset: Offset3D,
        pub extent: Offset3D,
        pub memory: DeviceMemory,
        pub memoryOffset: DeviceSize,
        pub flags: SparseMemoryBindFlags
    }

    #[repr(C)]
    #[allow(non_snake_case)]
    pub struct SparseImageMemoryBindInfo {
        pub image: Image,
        pub bindCount: uint32_t,
        pub pBinds: *const SparseImageMemoryBind
    }

    #[repr(C)]
    #[allow(non_snake_case)]
    pub struct BindSparseInfo {
        pub sType: StructureType,
        pub pNext: *const c_void,
        pub watiSemaphoreCount: uint32_t,
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

    #[repr(C)]
    #[allow(non_snake_case)]
    pub struct FenceCreateInfo {
        pub sType: StructureType,
        pub pNext: *const c_void,
        pub flags: FenceCreateFlags
    }

    #[repr(C)]
    #[allow(non_snake_case)]
    pub struct SemaphoreCreateInfo {
        pub sType: StructureType,
        pub pNext: *const c_void,
        pub flags: SemaphoreCreateFlags
    }

    #[repr(C)]
    #[allow(non_snake_case)]
    pub struct EventCreateInfo {
        pub sType: StructureType,
        pub pNext: *const c_void,
        pub flags: EventCreateFlags
    }

    #[repr(C)]
    #[allow(non_snake_case)]
    pub struct QueryPoolCreateInfo {
        pub sType: StructureType,
        pub pNext: *const c_void,
        pub flags: QueryPoolCreateFlags,
        pub queryType: QueryType,
        pub queryCount: uint32_t,
        pub pipelineStatistics: QueryPipelineStatisticFlags
    }

    #[repr(C)]
    #[allow(non_snake_case)]
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

    #[repr(C)]
    #[allow(non_snake_case)]
    pub struct BufferViewCreateInfo {
        pub sType: StructureType,
        pub pNext: *const c_void,
        pub flags: BufferViewCreateFlags,
        pub buffer: Buffer,
        pub format: Format,
        pub offset: DeviceSize,
        pub range: DeviceSize
    }

    #[repr(C)]
    #[allow(non_snake_case)]
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

    #[repr(C)]
    #[allow(non_snake_case)]
    pub struct SubresourceLayout {
        pub offset: DeviceSize,
        pub size: DeviceSize,
        pub rowPitch: DeviceSize,
        pub arrayPitch: DeviceSize,
        pub depthPitch: DeviceSize
    }

    #[repr(C)]
    #[allow(non_snake_case)]
    pub struct ComponentMapping {
        pub r: ComponentSwizzle,
        pub g: ComponentSwizzle,
        pub b: ComponentSwizzle,
        pub a: ComponentSwizzle
    }

    #[repr(C)]
    #[allow(non_snake_case)]
    pub struct ImageSubresourceRange {
        pub aspectMask: ImageAspectFlags,
        pub baseMipLevel: uint32_t,
        pub levelCount: uint32_t,
        pub baseArrayLayer: uint32_t,
        pub layerCount: uint32_t
    }

    #[repr(C)]
    #[allow(non_snake_case)]
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

    #[repr(C)]
    #[allow(non_snake_case)]
    pub struct ShaderModuleCreateInfo {
        pub sType: StructureType,
        pub pNext: *const c_void,
        pub flags: ShaderModuleCreateFlags,
        pub codeSize: size_t,
        pub pCode: *const uint32_t
    }

    #[repr(C)]
    #[allow(non_snake_case)]
    pub struct PipelineCacheCreateInfo {
        pub sType: StructureType,
        pub pNext: *const c_void,
        pub flags: PipelineCacheCreateFlags,
        pub initialDataSize: size_t,
        pub pInitialData: *const c_void
    }

    #[repr(C)]
    #[allow(non_snake_case)]
    pub struct SpecializationMapEntry {
        pub constantID: uint32_t,
        pub offset: uint32_t,
        pub size: size_t
    }

    #[repr(C)]
    #[allow(non_snake_case)]
    pub struct SpecializationInfo {
        pub mapEntryCount: uint32_t,
        pub pMapEntries: *const SpecializationMapEntry,
        pub dataSize: size_t,
        pub pData: *const c_void
    }

    #[repr(C)]
    #[allow(non_snake_case)]
    pub struct PipelineShaderStageCreateInfo {
        pub sType: StructureType,
        pub pNext: *const c_void,
        pub flags: PipelineShaderStageCreateFlags,
        pub stage: ShaderStageFlags,
        pub module: ShaderModule,
        pub pName: *const c_char,
        pub pSpecializationInfo: *const SpecializationInfo
    }

    #[repr(C)]
    #[allow(non_snake_case)]
    pub struct VertexInputBindingDescription {
        pub binding: uint32_t,
        pub strid: uint32_t,
        pub inputRate: VertexInputRate
    }

    #[repr(C)]
    #[allow(non_snake_case)]
    pub struct VertexInputAttributeDescription {
        pub location: uint32_t,
        pub binding: uint32_t,
        pub format: Format,
        pub offset: uint32_t,
    }

    #[repr(C)]
    #[allow(non_snake_case)]
    pub struct PipelineVertexInputStateCreateInfo {
        pub sType: StructureType,
        pub pNext: *const c_void,
        pub flags: PipelineVertexInputStateCreateFlags,
        pub vertexBindingDescriptionCount: uint32_t,
        pub pVertexBindingDescriptions: *const VertexInputBindingDescription,
        pub vertexAttributeDescriptionCount: uint32_t,
        pub pVertexAttributeDescriptions: *const VertexInputAttributeDescription
    }

    #[repr(C)]
    #[allow(non_snake_case)]
    pub struct PipelineInputAssemblyStateCreateInfo {
        pub sType: StructureType,
        pub pNext: *const c_void,
        pub flags: PipelineInputAssemblyStateCreateFlags,
        pub topology: PrimitiveTopology,
        pub primitiveRestartEnable: Bool32
    }

    #[repr(C)]
    #[allow(non_snake_case)]
    pub struct PipelineTessellationStateCreateInfo {
        pub sType: StructureType,
        pub pNext: *const c_void,
        pub flags: PipelineTessellationStateCreateFlags,
        pub patchControlPoints: uint32_t
    }

    #[repr(C)]
    #[allow(non_snake_case)]
    pub struct Viewport {
        pub x: c_float,
        pub y: c_float,
        pub width: c_float,
        pub hiehgt: c_float,
        pub minDepth: c_float,
        pub maxDepth: c_float
    }

    #[repr(C)]
    #[allow(non_snake_case)]
    pub struct Offset2D {
        pub x: int32_t,
        pub y: int32_t
    }

    #[repr(C)]
    #[allow(non_snake_case)]
    pub struct Extent2D {
        pub width: uint32_t,
        pub height: uint32_t
    }

    #[repr(C)]
    #[allow(non_snake_case)]
    pub struct Rect2D {
        pub offset: Offset2D,
        pub extent: Extent2D
    }

    #[repr(C)]
    #[allow(non_snake_case)]
    pub struct PipelineViewportStateCreateInfo {
        pub sType: StructureType,
        pub pNext: *const c_void,
        pub flags: PipelineViewportStateCreateFlags,
        pub viewportCount: uint32_t,
        pub pViewports: *const Viewport,
        pub scissorCount: uint32_t,
        pub pScissors: *const Rect2D
    }

    #[repr(C)]
    #[allow(non_snake_case)]
    pub struct PipelineRasterizationStateCreateInfo {
        pub sType: StructureType,
        pub pNext: *const c_void,
        pub flags: PipelineRasterizationStateCreateFlags,
        pub depthClampEnable: Bool32,
        pub rasterizerDiscardEnable: Bool32,
        pub polygonMode: PolygonMode,
        pub cullMode: CullModeFlags,
        pub frontFace: FrontFace,
        pub depthViasEanbel: Bool32,
        pub depthBiasConstantFactor: c_float,
        pub depthBiasClamp: c_float,
        pub depthSlopeFactor: c_float,
        pub lineWidth: c_float
    }

    #[repr(C)]
    #[allow(non_snake_case)]
    pub struct PipelineMultisampleStateCreateInfo {
        pub sType: StructureType,
        pub pNext: *const c_void,
        pub flags: PipelineMultisampleStateCreateFlags,
        pub rasterizationSamples: SampleCountFlags,
        pub sampleShadingEanble: Bool32,
        pub minSampleShading: c_float,
        pub pSampleMask: *const SampleMask,
        pub alphaToCoverageEnable: Bool32,
        pub alphaToOneEnable: Bool32
    }

    #[repr(C)]
    #[allow(non_snake_case)]
    pub struct StencilOpState {
        pub failOp: StencilOp,
        pub passOp: StencilOp,
        pub depthFailOp: StencilOp,
        pub compareOp: CompareOp,
        pub compareMask: uint32_t,
        pub writeMask: uint32_t,
        pub reference: uint32_t
    }

    #[repr(C)]
    #[allow(non_snake_case)]
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

    #[repr(C)]
    #[allow(non_snake_case)]
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

    #[repr(C)]
    #[allow(non_snake_case)]
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

    #[repr(C)]
    #[allow(non_snake_case)]
    pub struct PipelineDynamicStateCreateInfo {
        pub sType: StructureType,
        pub pNext: *const c_void,
        pub flags: PipelineDynamicStateCreateFlags,
        pub dynamicStateCount: uint32_t,
        pub pDynamicStates: *const DynamicState
    }

    #[repr(C)]
    #[allow(non_snake_case)]
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

    #[repr(C)]
    #[allow(non_snake_case)]
    pub struct ComputePipelineCreateInfo {
        pub sType: StructureType,
        pub pNext: *const c_void,
        pub flags: PipelineCreateFlags,
        pub stage: PipelineShaderStageCreateInfo,
        pub layout: PipelineLayout,
        pub basePipelineHandle: Pipeline,
        pub basePipelineIndex: int32_t
    }

    #[repr(C)]
    #[allow(non_snake_case)]
    pub struct PushConstantRange {
        pub stageFlags: ShaderStageFlags,
        pub offset: uint32_t,
        pub size: uint32_t
    }

    #[repr(C)]
    #[allow(non_snake_case)]
    pub struct PipelineLayoutCreateInfo {
        pub sType: StructureType,
        pub pNext: *const c_void,
        pub flags: PipelineLayoutCreateFlags,
        pub setLayoutCount: uint32_t,
        pub pSetLayouts: *const DescriptorSetLayout,
        pub pushConstantRangeCount: uint32_t,
        pub pPushConstantRanges: *const PushConstantRange
    }

    #[repr(C)]
    #[allow(non_snake_case)]
    pub struct SamplerCreateInfo {
        pub sType: StructureType,
        pub pNext: *const c_void,
        pub flags: SamplerCreateFlags,
        pub magFilter: Filter,
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
        pub boarderColor: BorderColor,
        pub unnomralizedCoordinates: Bool32
    }

    #[repr(C)]
    #[allow(non_snake_case)]
    pub struct DescriptorSetLayoutBinding {
        pub binding: uint32_t,
        pub descriptorType: DescriptorType,
        pub descriptorCount: uint32_t,
        pub stageFlags: ShaderStageFlags,
        pub pImmutableSamples: *const Sampler
    }

    #[repr(C)]
    #[allow(non_snake_case)]
    pub struct DescriptorSetLayoutCreateInfo {
        pub sType: StructureType,
        pub pNext: *const c_void,
        pub flags: DescriptorSetLayoutCreateFlags,
        pub bindingCount: uint32_t,
        pub pBindings: *const DescriptorSetLayoutBinding
    }

    #[repr(C)]
    #[allow(non_snake_case)]
    pub struct DescriptorPoolSize {
        pub descriptorType: DescriptorType,
        pub descriptorCount: uint32_t
    }

    #[repr(C)]
    #[allow(non_snake_case)]
    pub struct DescriptorPoolCreateInfo {
        pub sType: StructureType,
        pub pNext: *const c_void,
        pub flags: DescriptorPoolCreateFlags,
        pub maxSets: uint32_t,
        pub poolSizeCount: uint32_t,
        pub pPoolSizes: *const DescriptorPoolSize
    }

    #[repr(C)]
    #[allow(non_snake_case)]
    pub struct DescriptorSetAllocateInfo {
        pub sType: StructureType,
        pub pNext: *const c_void,
        pub descriptorPool: DescriptorPool,
        pub descriptorSetCount: uint32_t,
        pub pSetLayouts: *const DescriptorSetLayout
    }

    #[repr(C)]
    #[allow(non_snake_case)]
    pub struct DescriptorImageInfo {
        pub sampler: Sampler,
        pub imageView: ImageView,
        pub imageLayout: ImageLayout
    }

    #[repr(C)]
    #[allow(non_snake_case)]
    pub struct DescriptorBufferInfo {
        pub buffer: Buffer,
        pub offset: DeviceSize,
        pub range: DeviceSize
    }

    #[repr(C)]
    #[allow(non_snake_case)]
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

    #[repr(C)]
    #[allow(non_snake_case)]
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

    #[repr(C)]
    #[allow(non_snake_case)]
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

    #[repr(C)]
    #[allow(non_snake_case)]
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

    #[repr(C)]
    #[allow(non_snake_case)]
    pub struct AttachmentReference {
        pub attachment: uint32_t,
        pub layout: ImageLayout
    }

    #[repr(C)]
    #[allow(non_snake_case)]
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

    #[repr(C)]
    #[allow(non_snake_case)]
    pub struct SubpassDependency {
        pub srcSubpass: uint32_t,
        pub dstSubpass: uint32_t,
        pub srcStageMask: PipelineStageFlags,
        pub dstStageMask: PipelineStageFlags,
        pub srcAccessMask: AccessFlags,
        pub dstAccessMask: AccessFlags,
        pub dependencyFlags: DependencyFlags
    }

    #[repr(C)]
    #[allow(non_snake_case)]
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

    #[repr(C)]
    #[allow(non_snake_case)]
    pub struct CommandPoolCreateInfo {
        pub sType: StructureType,
        pub pNext: *const c_void,
        pub flags: CommandPoolCreateFlags,
        pub queueFamilyIndex: uint32_t
    }

    #[repr(C)]
    #[allow(non_snake_case)]
    pub struct CommandBufferAllocateInfo {
        pub sType: StructureType,
        pub pNext: *const c_void,
        pub commandPool: CommandPool,
        pub level: CommandBufferLevel,
        pub commandBufferCount: uint32_t
    }

    #[repr(C)]
    #[allow(non_snake_case)]
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

    #[repr(C)]
    #[allow(non_snake_case)]
    pub struct CommandBufferBeginInfo {
        pub sType: StructureType,
        pub pNext: *const c_void,
        pub flags: CommandBufferUsageFlags,
        pub pInheritanceInfo: *const CommandBufferInheritanceInfo
    }

    #[repr(C)]
    #[allow(non_snake_case)]
    pub struct BufferCopy {
        pub srcOffset: DeviceSize,
        pub dstOffset: DeviceSize,
        pub size: DeviceSize
    }

    #[repr(C)]
    #[allow(non_snake_case)]
    pub struct ImageSubresourceLayers {
        pub aspectMask: ImageAspectFlags,
        pub mipLevel: uint32_t,
        pub baseArrayLayer: uint32_t,
        pub layerCount: uint32_t
    }

    #[repr(C)]
    #[allow(non_snake_case)]
    pub struct ImageCopy {
        pub srcSubresource: ImageSubresourceLayers,
        pub srcOffset: Offset3D,
        pub dstSubresource: ImageSubresourceLayers,
        pub dstOffset: Offset3D,
        pub extent: Extent3D
    }

    #[repr(C)]
    #[allow(non_snake_case)]
    pub struct ImageBlit {
        pub srcSubresource: ImageSubresourceLayers,
        pub srcOffsets: [Offset3D; 2],
        pub dstSubresource: ImageSubresourceLayers,
        pub dstOffsets: [Offset3D; 2]
    }

    #[repr(C)]
    #[allow(non_snake_case)]
    pub struct BufferImageCopy {
        pub bufferOffset: DeviceSize,
        pub bufferRowLength: uint32_t,
        pub bufferImageHeight: uint32_t,
        pub imageSubresource: ImageSubresourceLayers,
        pub imageOffset: Offset3D,
        pub imageExtent: Extent3D
    }

    // NOTE: This represents a union. This code should be rewritten when rust adds union support.
    #[allow(non_snake_case)]
    // Can be array of c_float, int32_t, or uint32_t.
    pub type ClearColorValue = [uint32_t; 4];

    #[repr(C)]
    #[allow(non_snake_case)]
    pub struct ClearDepthStencilValue {
        pub depth: c_float,
        pub stencil: uint32_t
    }

    #[repr(C)]
    #[allow(non_snake_case)]
    pub struct ClearValue {
        pub color: ClearColorValue,
        pub depthStencil: ClearDepthStencilValue
    }

    #[repr(C)]
    #[allow(non_snake_case)]
    pub struct ClearAttachment {
        pub aspectMask: ImageAspectFlags,
        pub colorAttachment: uint32_t,
        pub clearValue: ClearValue
    }

    #[repr(C)]
    #[allow(non_snake_case)]
    pub struct ClearRect {
        pub rect: Rect2D,
        pub baseArrayLayer: uint32_t,
        pub layerCount: uint32_t
    }

    #[repr(C)]
    #[allow(non_snake_case)]
    pub struct ImageResolve {
        pub srcSubresource: ImageSubresourceLayers,
        pub srcOffset: Offset3D,
        pub dstSubresource: ImageSubresourceLayers,
        pub dstOffset: Offset3D,
        pub extent: Extent3D
    }

    #[repr(C)]
    #[allow(non_snake_case)]
    pub struct MemoryBarrier {
        pub sType: StructureType,
        pub pNext: *const c_void,
        pub srcAccessMask: AccessFlags,
        pub dstAccessMask: AccessFlags,
    }

    #[repr(C)]
    #[allow(non_snake_case)]
    pub struct BufferMemoryBarrier {
        pub sType: StructureType,
        pub pNext: *const c_void,
        pub srcAccessMask: AccessFlags,
        pub dstAccessMask: AccessFlags,
        pub srcQueueFamilyIndex: uint32_t,
        pub dstQueueFamilyINdex: uint32_t,
        pub buffer: Buffer,
        pub offset: DeviceSize,
        pub size: DeviceSize
    }

    #[repr(C)]
    #[allow(non_snake_case)]
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

    #[repr(C)]
    #[allow(non_snake_case)]
    pub struct RenderPassBeginInfo {
        pub sType: StructureType,
        pub pNext: *const c_void,
        pub renderPass: RenderPass,
        pub framebuffer: Framebuffer,
        pub renderArea: Rect2D,
        pub clearValueCount: uint32_t,
        pub pClearValues: *const ClearValue
    }

    #[repr(C)]
    #[allow(non_snake_case)]
    pub struct DispatchIndirectCommand {
        pub x: uint32_t,
        pub y: uint32_t,
        pub z: uint32_t,
    }

    #[repr(C)]
    #[allow(non_snake_case)]
    pub struct DrawIndexedIndirectCommand {
        pub indexCount: uint32_t,
        pub instanceCount: uint32_t,
        pub firstIndex: uint32_t,
        pub vertexOffset: int32_t,
        pub firstInstance: uint32_t
    }

    #[repr(C)]
    #[allow(non_snake_case)]
    pub struct DrawIndirectCommand {
        pub vertexCount: uint32_t,
        pub instanceCount: uint32_t,
        pub firstVertext: uint32_t,
        pub firstInstance: uint32_t
    }

    #[link(name = "vulkan-1")]
    #[allow(non_snake_case)]
    #[allow(dead_code)]
    extern "C" {
        fn vkCreateInstance(pCreateInfo: *const InstanceCreateInfo, pAllocator: *const AllocationCallbacks, pInstance: *mut Instance) -> Result;
        fn vkDestroyInstance(instance: Instance, pAllocator: *const AllocationCallbacks) -> c_void;
        fn vkEnumeratePhysicalDevices(instance: Instance, pPhysicalDeviceCount: *mut uint32_t, pPhysicalDevices: *mut PhysicalDevice) -> Result;
        fn vkGetPhysicalDeviceFeatures(physicalDevice: PhysicalDevice, pFeatures: *mut PhysicalDeviceFeatures) -> c_void;
        fn vkGetPhysicalDeviceFormatProperties(physicalDevice: PhysicalDevice, format: Format, pFormatProperties: *mut FormatProperties) -> c_void;
        fn vkGetPhysicalDeviceImageFormatProperties(physicalDevice: PhysicalDevice, format: Format, imageType: ImageType, tiling: ImageTiling,
                                                    usage: ImageUsageFlags, flags: ImageCreateFlags, pImageFormatProperties: *mut ImageFormatProperties) -> Result;
        fn vkGetPhysicalDeviceProperties(physicalDevice: PhysicalDevice, pProperties: *mut PhysicalDeviceProperties) -> c_void;
        fn vkGetPhysicalDeviceQueueFamilyProperties(physicalDevice: PhysicalDevice, pQueueFamilyProperyCount: *mut uint32_t, pQueueFamilyProperties: *mut QueueFamilyProperties) -> c_void;
        fn vkGetPhysicalDeviceMemoryProperties(physicalDevice: PhysicalDevice, pMemoryProperties: *mut PhysicalDeviceMemoryProperties) -> c_void;
        fn vkGetInstanceProcAddr(instance: Instance, pName: *const libc::wchar_t) -> VoidFunction;
        fn vkGetDeviceProcAddr(device: Device, pName: *const libc::wchar_t) -> VoidFunction;
        fn vkCreateDevice(physicalDevice: PhysicalDevice, pCreateInfo: *const DeviceCreateInfo, pAllocator: *const AllocationCallbacks, pDevice: *mut Device) -> Result;
        fn vkDestroyDevice(device: Device, pAllocator: *const AllocationCallbacks) -> c_void;
        fn vkEnumerateInstanceExtensionProperties(pLayerName: *const libc::wchar_t, pPropertyCount: *mut uint32_t, pProperties: *mut ExtensionProperties) -> Result;
        fn vkEnumerateDeviceExtensionProperties(physicalDevice: PhysicalDevice, pLayerName: *const libc::wchar_t, pPropertyCount: *mut uint32_t, pProperties: *mut ExtensionProperties) -> Result;
        fn vkEnumerateInstanceLayerProperties(pPropertyCount: *mut uint32_t, pProperties: *mut LayerProperties) -> Result;
        fn vkEnumerateDeviceLayerProperties(physicalDevice: PhysicalDevice, pPropertyCount: *mut uint32_t, pProperties: *mut LayerProperties) -> Result;
        fn vkGetDeviceQueue(device: Device, queueFamilyIndex: uint32_t, queueIndex: uint32_t, pQueue: *mut Queue) -> Result;
        fn vkQueueSubmit(queue: Queue, submitCount: uint32_t, pSubmits: *const SubmitInfo) -> Result;
        fn vkQueueWaitIdle(queue: Queue) -> Result;
        fn vkDeviceWaitIdle(device: Device) -> Result;
        fn vkAllocateMemory(device: Device, pAllocateInfo: *const MemoryAllocateInfo, pAllocator: *const AllocationCallbacks, pMemory: *mut DeviceMemory) -> Result;
        fn vkFreeMemory(device: Device, memory: DeviceMemory, pAllocator: *const AllocationCallbacks) -> c_void;
        fn vkMapMemory(device: Device, memory: DeviceMemory, offset: DeviceSize, size: DeviceSize, flags: MemoryMapFlags, ppData: *mut *mut c_void) -> Result;
        fn vkUnmapMemory(device: Device, memory: DeviceMemory) -> c_void;
        fn vkFlushMappedMemoryRanges(device: Device, memoryRangeCount: uint32_t, pMemoryRanges: *const MappedMemoryRange) -> Result;
        fn vkInvalidateMappedMemoryRanges(device: Device, memoryRangeCount: uint32_t, pMemoryRanges: *const MappedMemoryRange) -> Result;
        fn vkGetDeviceMemoryCommmitment(device: Device, memory: DeviceMemory, pCommittedMemoryInBytes: *mut DeviceSize) -> c_void;
        fn vkBindBufferMemory(device: Device, buffer: Buffer, memory: DeviceMemory, memoryOffset: DeviceSize) -> Result;
        fn vkBindImageMemory(device: Device, image: Image, memory: DeviceMemory, memoryOffset: DeviceSize) -> Result;
        fn vkGetBufferMemoryRequirements(device: Device, buffer: Buffer, pMemoryRequirements: *mut MemoryRequirements) -> c_void;
        fn vkGetImageMemoryRequirements(device: Device, image: Image, pMemoryRequirements: *mut MemoryRequirements) -> c_void;
        fn vkGetImageSparseMemoryRequirements(device: Device, image: Image, pSparseMemoryRequirementCount: uint32_t, pSparseMemoryRequirements: *mut SparseImageMemoryRequirements) -> c_void;
        fn vkGetPhysicalDeviceSparseImageFormatProperties(physicalDevice: PhysicalDevice, format: Format, imageType: ImageType, samples: SampleCountFlags,
                                                          usage: ImageUsageFlags, tiling: ImageTiling, pPropertyCount: *mut uint32_t, pProperties: *mut SparseImageFormatProperties) -> c_void;
        fn vkQueueBindSparse(queue: Queue, bindInfoCount: uint32_t, pBindInfo: *const BindSparseInfo, fence: Fence) -> Result;
        fn vkCreateFence(device: Device, pCreateInfo: *const FenceCreateInfo, pAllocator: *const AllocationCallbacks, pFence: *mut Fence) -> Result;
        fn vkDestroyFence(device: Device, fence: Fence, pAllocator: *const AllocationCallbacks) -> c_void;
        fn vkResetFences(device: Device, fenceCount: uint32_t, pFences: *const Fence) -> Result;
        fn vkGetFenceStatus(device: Device, fence: Fence) -> Result;
        fn vkWaitForFences(device: Device, fenceCount: uint32_t, pFences: *const Fence, waitAll: Bool32, timeout: uint64_t) -> Result;
        fn vkCreateSemaphore(device: Device, pCreateInfo: *const SemaphoreCreateInfo, pAllocator: *const AllocationCallbacks, pSemaphore: *mut Semaphore) -> Result;
        fn vkDestroySemaphore(device: Device, semaphore: Semaphore, pAllocator: *const AllocationCallbacks) -> c_void;
        fn vkCreateEvent(device: Device, pCreateInfo: *const EventCreateInfo, pAllocator: *const AllocationCallbacks, pEvent: *mut Event) -> Result;
        fn vkDestroyEvent(device: Device, event: Event, pAllocator: *const AllocationCallbacks) -> c_void;
        fn vkGetEventStatus(device: Device, event: Event) -> Result;
        fn vkSetEvent(device: Device, event: Event) -> Result;
        fn vkResetEvent(device: Device, event: Event) -> Result;
        fn vkCreateQueryPool(device: Device, pCreateInfo: *const QueryPoolCreateInfo, pAllocator: *const AllocationCallbacks, pQueryPool: *mut QueryPool) -> Result;
        fn vkDestroyQueryPool(device: Device, queryPool: QueryPool, pAllocator: *const AllocationCallbacks, pQueryPool: *mut QueryPool) -> c_void;
        fn vkGetQueryPoolResults(device: Device, queryPool: QueryPool, firstQuery: uint32_t, queryCount: uint32_t, dataSize: size_t, pData: *mut c_void,
                                 strid: DeviceSize, flags: QueryResultFlags) -> Result;
        fn vkCreateBuffer(device: Device, pCreateInfo: *const BufferCreateInfo, pAllocator: *const AllocationCallbacks, pBuffer: *mut Buffer) -> Result;
        fn vkDestroyBuffer(device: Device, buffer: Buffer, pAllocator: *const AllocationCallbacks) -> c_void;
        fn vkCreateBufferView(device: Device, pCreateInfo: *const BufferViewCreateInfo, pAllocator: *const AllocationCallbacks, pView: *mut BufferView) -> Result;
        fn vkDestroyBufferView(device: Device, bufferView: BufferView, pAllocator: *const AllocationCallbacks) -> c_void;
        fn vkCreateImage(device: Device, bufferView: BufferView, pAllocator: *const AllocationCallbacks) -> Result;
        fn vkDestroyImage(device: Device, image: Image, pAllocator: *const AllocationCallbacks) -> c_void;
        fn vkGetImageSubresourceLayout(device: Device, image: Image, pSubresource: *const ImageSubresource, pLayout: *mut SubresourceLayout) -> c_void;
        fn vkCreateImageView(device: Device, pCreateInfo: *const ImageViewCreateInfo, pAllocator: *const AllocationCallbacks, pView: *mut ImageView) -> Result;
        fn vkDestroyImageView(device: Device, imageView: ImageView, pAllocator: *const AllocationCallbacks) -> c_void;
        fn vkCreateShaderModule(device: Device, shaderModule: ShaderModule, pAllocator: *const AllocationCallbacks) -> Result;
        fn vkDestroyShaderModule(device: Device, shaderModule: ShaderModule, pAllocator: *const AllocationCallbacks) -> c_void;
        fn vkCreatePipelineCache(device: Device, pCreateInfo: *const PipelineCacheCreateInfo, pAllocator: *const AllocationCallbacks, pPipelineCache: *mut PipelineCache) -> Result;
        fn vkDestroyPipelineCache(device: Device, pipelineCache: PipelineCache, pAllocator: *const AllocationCallbacks) -> c_void;
        fn vkGetPipelinecachedata(device: Device, pipelineCache: PipelineCache, pDataSize: *mut size_t, pData: *mut c_void) -> Result;
        fn vkMergePipelineCaches(device: Device, dstCache: PipelineCache, srcCacheCount: uint32_t, pSrcCaches: *const PipelineCache) -> Result;
        fn vkCreateGraphicsPipelines(device: Device, pipelineCache: PipelineCache, createInfoCount: uint32_t, pCreateInfos: *const GraphicsPipelineCreateInfo,
                                     pAlloator: *const AllocationCallbacks, pPipelines: *mut Pipeline) -> Result;
        fn vkCreateComputePipelines(device: Device, pipelineCache: PipelineCache, createInfoCount: uint32_t, pCreateInfos: *const ComputePipelineCreateInfo,
                                    pAllocator: *const AllocationCallbacks, pPipelines: *mut Pipeline) -> Result;
        fn vkDestroyPipeline(device: Device, pipeline: Pipeline, pAllocator: *const AllocationCallbacks) -> c_void;
        fn vkCreatePipelineLayout(device: Device, pCreateInfo: *const PipelineLayoutCreateInfo, pAllocator: *const AllocationCallbacks, pPipelineLayout: *mut PipelineLayout) -> Result;
        fn vkDestroyPipelineLayout(device: Device, pipelineLayout: PipelineLayout, pAllocator: *const AllocationCallbacks) -> c_void;
        fn vkCreateSampler(device: Device, pCreateInfo: *const SamplerCreateInfo, pAllocator: *const AllocationCallbacks) -> Result;
        fn vkDestroySampler(device: Device, sampler: Sampler, pAllocator: *const AllocationCallbacks) -> c_void;
        fn vkCreateDescriptorSetLayout(device: Device, pCreateInfo: *const DescriptorSetLayoutCreateInfo, pAllocator: *const AllocationCallbacks, pSetLayout: *mut DescriptorSetLayout) -> Result;
        fn vkDestroyDescriptorSetLayout(device: Device, descriptorSetLayout: DescriptorSetLayout, pAllocator: *const AllocationCallbacks) -> c_void;
        fn vkCreateDescriptorPool(device: Device, pCreateInfo: *const DescriptorPoolCreateInfo, pAllocator: *const AllocationCallbacks, pDescriptorPool: *mut DescriptorPool) -> Result;
        fn vkDestroyDescriptorPool(device: Device, descriptorPool: DescriptorPool, pAllocator: *const AllocationCallbacks) -> c_void;
        fn vkResetDescriptorPool(device: Device, descriptorPool: DescriptorPool, flags: DescriptorPoolResetFlags) -> Result;
        fn vkAllocateDescriptorSets(device: Device, pAllocateInfo: *const DescriptorSetAllocateInfo, pDescriptorSets: *mut DescriptorSet) -> Result;
        fn vkFreeDescriptorSets(device: Device, descriptorPool: DescriptorPool, descriptorSetCount: uint32_t, pDescriptorSets: *const DescriptorSet) -> Result;
        fn vkUpdateDescriptorSets(device: Device, descriptorWriteCount: uint32_t, pDescriptorWrites: *const WriteDescriptorSet, descriptorCopyCount: uint32_t,
                                  pDescriptorCopies: *const CopyDescriptorSet) -> c_void;
        fn vkCreateFramebuffer(device: Device, pCreateInfo: *const FramebufferCreateInfo, pAllocator: *const AllocationCallbacks, pFramebuffer: *mut Framebuffer) -> Result;
        fn vkDestroyFramebuffer(device: Device, framebuffer: Framebuffer, pAllocator: *const AllocationCallbacks) -> c_void;
        fn vkCreateRenderPass(device: Device, pCreateInfo: *const RenderPassCreateInfo, pAllocator: *const AllocationCallbacks, pRenderPass: *mut RenderPass) -> Result;
        fn vkDestroyRenderPass(device: Device, renderPass: RenderPass, pAllactore: *const AllocationCallbacks) -> c_void;
        fn vkGetRenderAreaGranularity(device: Device, renderPass: RenderPass, pGranularity: *mut Extent2D) -> c_void;
        fn vkCreateCommandPool(device: Device, pCreateInfo: *const CommandPoolCreateInfo, pAllocator: *const AllocationCallbacks, pCommandPool: *mut CommandPool) -> Result;
        fn vkDestroyCommandPool(device: Device, commandPool: CommandPool, pAllocator: *const AllocationCallbacks) -> c_void;
        fn vkResetCommandPool(device: Device, commandPool: CommandPool, flags: CommandPoolResetFlags) -> Result;
        fn vkAllocateCommandBuffers(device: Device, pAllocateInfo: *const CommandBufferAllocateInfo, pCommandBuffers: *mut CommandBuffer) -> Result;
        fn vkFreeCommandBuffers(device: Device, commandPool: CommandPool, commandBufferCount: uint32_t, pCommandBuffers: *const CommandBuffer) -> c_void;
        fn vkBeginCommandBuffer(commandBuffer: CommandBuffer, pBeginInfo: *const CommandBufferBeginInfo) -> Result;
        fn vkEndCommandBuffer(commandBuffer: CommandBuffer) -> Result;
        fn vkResetCommandBuffer(commandBuffer: CommandBuffer, flags: CommandBufferResetFlags) -> Result;
        fn vkCmdBindPipeline(commandBuffer: CommandBuffer, pipelineBindPoint: PipelineBindPoint, pipeline: Pipeline) -> c_void;
        fn vkCmdSetViewport(commandBuffer: CommandBuffer, firstViewport: uint32_t, viewportCount: uint32_t, pViewports: *const Viewport) -> c_void;
        fn vkCmdSetScissor(commandBuffer: CommandBuffer, firstScissor: uint32_t, scissorCount: uint32_t, pScissors: *const Rect2D) -> c_void;
        fn vkCmdSetLineWidth(commandBuffer: CommandBuffer, lineWidth: c_float) -> c_void;
        fn vkCmdSetDepthBias(commandBuffer: CommandBuffer, depthBiasConstantFactor: c_float, depthBiasClamp: c_float, depthBiasSlopeFactor: c_float) -> c_void;
        fn vkCmdSetBlendConstants(commandBuffer: CommandBuffer, blendConstants: [c_float; 4]) -> c_void;
        fn vkCmdSetDepthBounds(commandBuffer: CommandBuffer, minDepthBounds: c_float, maxDepthBounds: c_float) -> c_void;
        fn vkCmdSetStencilCompareMask(commandBuffer: CommandBuffer, faceMask: StencilFaceFlags, compareMask: uint32_t) -> c_void;
        fn vkCmdSetStencilWriteMask(commandBuffer: CommandBuffer, faceMask: StencilFaceFlags, writeMask: uint32_t) -> c_void;
        fn vkCmdSetStencilReference(commandBuffer: CommandBuffer, faceMask: StencilFaceFlags, reference: uint32_t) -> c_void;
        fn vkCmdBindDescriptorSets(commandBuffer: CommandBuffer, pipelineBindPoint: PipelineBindPoint, layout: PipelineLayout, firstSet: uint32_t,
                                   descriptorSetCount: uint32_t, pDescriptorSets: *const DescriptorSet, dynamicOffsetCount: uint32_t, pDynamicOffsets: *const uint32_t) -> c_void;
        fn vkCmdBindIndexBuffer(commandBuffer: CommandBuffer, buffer: Buffer, offset: DeviceSize, indexType: IndexType) -> c_void;
        fn vkCmdBindVertexBuffer(commandBuffer: CommandBuffer, firstBinding: uint32_t, bindingCount: uint32_t, pBuffers: *const Buffer, pOffsets: *const DeviceSize) -> c_void;
        fn vkCmdDraw(commandBuffer: CommandBuffer, vertexCount: uint32_t, instanceCount: uint32_t, firstVetext: uint32_t, firstInstance: uint32_t) -> c_void;
        fn vkCmdDrawIndexed(commandBuffer: CommandBuffer, indexCount: uint32_t, instanceCount: uint32_t, firstIndex: uint32_t, vertexOffset: int32_t, firstInstance: uint32_t) -> c_void;
        fn vkCmdDrawIndirect(commandBuffer: CommandBuffer, buffer: Buffer, offset: DeviceSize, drawCount: uint32_t, stride: uint32_t) -> c_void;
        fn vkCmdDrawIndexedIndirect(commandBuffer: CommandBuffer, buffer: Buffer, offset: DeviceSize, drawCount: uint32_t, stride: uint32_t) -> c_void;
        fn vkCmdDispatch(commandBuffer: CommandBuffer, x: uint32_t, y: uint32_t, z: uint32_t) -> c_void;
        fn vkCmdDispatchIndirect(commandBuffer: CommandBuffer, buffer: Buffer, offset: DeviceSize) -> c_void;
        fn vkCmdCopyBuffer(commandBuffer: CommandBuffer, srcBuffer: Buffer, dstBuffer: Buffer, regionCount: uint32_t, pRegions: *const BufferCopy) -> c_void;
        fn vkCmdCopyImage(commandBuffer: CommandBuffer, srcImage: Image, srcImageLayout: ImageLayout, dstImage: Image, dstImageLayout: ImageLayout,
                          regionCount: uint32_t, pRegions: *const ImageCopy) -> c_void;
        fn vkCmdBlitImage(commandBuffer: CommandBuffer, srcImage: Image, srcImageLayout: ImageLayout, dstImage: Image, dstImageLayout: ImageLayout,
                          regionCount: uint32_t, pRegions: *const ImageBlit, filter: Filter) -> c_void;
        fn vkCmdCopyBufferToImage(commandBuffer: CommandBuffer, srcBuffer: Buffer, dstImage: Image, dstImageLayout: ImageLayout, regionCount: uint32_t,
                                  pRegions: *const BufferImageCopy) -> c_void;
        fn vkCmdCopyImageToBuffer(commandBuffer: CommandBuffer, srcImage: Image, srcImageLayout: ImageLayout, dstBuffer: Buffer, regionCount: uint32_t,
                                  pRegions: *const BufferImageCopy) -> c_void;
        fn vkCmdUpdateBuffer(commandBuffer: CommandBuffer, dstBuffer: Buffer, dstOffset: DeviceSize, dataSize: DeviceSize, pData: *const uint32_t) -> c_void;
        fn vkCmdFillBuffer(commandBuffer: CommandBuffer, dstBuffer: Buffer, dstOffset: DeviceSize, size: DeviceSize, data: uint32_t) -> c_void;
        fn vkCmdClearColorImage(commandBuffer: CommandBuffer, image: Image, imageLayout: ImageLayout, pColor: *const ClearColorValue, rangeCount: uint32_t,
                                pRanges: *const ImageSubresourceRange) -> c_void;
        fn vkCmdClearDepthStencilImage(commandBuffer: CommandBuffer, image: Image, imageLayout: ImageLayout, pDepthStencil: *const ClearDepthStencilValue,
                                       rangeCount: uint32_t, pRanges: *const ImageSubresourceRange) -> c_void;
        fn vkCmdClearAttachments(commandBuffer: CommandBuffer, attachmentCount: uint32_t, pAttachments: *const ClearAttachment, rectCount: uint32_t, pRects: *const ClearRect) -> c_void;
        fn vkCmdResolveImage(commandBuffer: CommandBuffer, srcImage: Image, srcImageLayout: ImageLayout, dstImage: Image, dstImageLayout: ImageLayout,
                             regionCount: uint32_t, pRegions: *const ImageResolve) -> c_void;
        fn vkCmdSetEvent(commandBuffer: CommandBuffer, event: Event, stageMask: PipelineStageFlags) -> c_void;
        fn vkCmdResetEvent(commandBuffer: CommandBuffer, event: Event, stageMask: PipelineStageFlags) -> c_void;
        fn vkCmdWaitEvents(commandBuffer: CommandBuffer, eventCount: uint32_t, pEvents: *const Event, srcStageMask: PipelineStageFlags, dstStageMask: PipelineStageFlags,
                           memoryBarrierCount: uint32_t, pMemoryBarriers: *const MemoryBarrier, bufferMemoryBarrierCount: uint32_t, pMemoryBarriers: *const MemoryBarrier,
                           imageMemoryBarrierCount: uint32_t, pImageMemoryBarriers: *const ImageMemoryBarrier) -> c_void;
        fn vkCmdPipelineBarrier(commandBuffer: CommandBuffer, srcStageMask: PipelineStageFlags, dstStageMask: PipelineStageFlags, dependencyFlags: DependencyFlags,
                                memoryBarrierCount: uint32_t, pMemoryBarriers: *const MemoryBarrier, bufferMemoryBarrierCount: uint32_t,
                                pBufferMemoryBarriers: *const BufferMemoryBarrier, imageMemoryBarrierCount: uint32_t, pImageMemoryBarriers: *const ImageMemoryBarrier) -> c_void;
        fn vkCmdBeginQuery(commandBuffer: CommandBuffer, queryPool: QueryPool, query: uint32_t, flags: QueryControlFlags) -> c_void;
        fn vkCmdEndQuery(commandBuffer: CommandBuffer, queryPool: QueryPool, query: uint32_t) -> c_void;
        fn vkCmdResetQueryPool(commandBuffer: CommandBuffer, queryPool: QueryPool, firstQuery: uint32_t, queryCount: uint32_t) -> c_void;
        fn vkCmdWriteTimestamp(commandBuffer: CommandBuffer, pipelineStage: PipelineStageFlags, queryPool: QueryPool, query: uint32_t) -> c_void;
        fn vkCmdCopyQueryPoolResults(commandBuffer: CommandBuffer, queryPool: QueryPool, firstQuery: uint32_t, queryCount: uint32_t, dstBuffer: Buffer,
                                     dstOffset: DeviceSize, stride: DeviceSize, flags: QueryResultFlags) -> c_void;
        fn vkCmdPushConstants(commandBuffer: CommandBuffer, layout: PipelineLayout, stageFlags: ShaderStageFlags, offset: uint32_t, size: uint32_t, pValues: *const c_void) -> c_void;
        fn vkCmdBeginRenderPass(commandBuffer: CommandBuffer, pRenderPassBegin: *const RenderPassBeginInfo, contents: SubpassContents) -> c_void;
        fn vkCmdNextSubpass(commandBuffer: CommandBuffer, contents: SubpassContents) -> c_void;
        fn vkCmdEndRenderPass(commandBuffer: CommandBuffer) -> c_void;
        fn vkCmdExecuteCommands(commandBuffer: CommandBuffer, commandBufferCount: uint32_t, pCommandBuffers: *const CommandBuffer) -> c_void;
    }
}
