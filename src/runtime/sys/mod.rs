#![cfg_attr(feature = "no-std", no_std)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#[cfg(feature = "no-std")]
extern crate alloc;
#[cfg(feature = "no-std")]
extern crate no_std_compat as std;
#[cfg(any(
    feature = "cuda-12040",
    feature = "cuda-12050",
    feature = "cuda-12060",
    feature = "cuda-12080",
    feature = "cuda-12090",
    feature = "cuda-13000"
))]
pub use self::cudaAsyncNotificationType_enum as cudaAsyncNotificationType;
pub use self::cudaDataType_t as cudaDataType;
#[cfg(any(feature = "cuda-13000"))]
pub use self::cudaEmulationMantissaControl_t as cudaEmulationMantissaControl;
#[cfg(any(feature = "cuda-13000"))]
pub use self::cudaEmulationSpecialValuesSupport_t as cudaEmulationSpecialValuesSupport;
#[cfg(any(feature = "cuda-13000"))]
pub use self::cudaEmulationStrategy_t as cudaEmulationStrategy;
pub use self::cudaError as cudaError_t;
#[cfg(any(
    feature = "cuda-12030",
    feature = "cuda-12040",
    feature = "cuda-12050",
    feature = "cuda-12060",
    feature = "cuda-12080",
    feature = "cuda-12090",
    feature = "cuda-13000"
))]
pub use self::cudaGraphDependencyType_enum as cudaGraphDependencyType;
#[cfg(any(
    feature = "cuda-11040",
    feature = "cuda-11050",
    feature = "cuda-11060",
    feature = "cuda-11070",
    feature = "cuda-11080"
))]
pub use self::cudaOutputMode as cudaOutputMode_t;
#[cfg(any(feature = "cuda-13000"))]
pub use self::CUDAlogLevel_enum as cudaLogLevel;
#[cfg(any(feature = "cuda-11040"))]
pub const CUDART_VERSION: u32 = 11040;
#[cfg(any(feature = "cuda-11050"))]
pub const CUDART_VERSION: u32 = 11050;
#[cfg(any(feature = "cuda-11060"))]
pub const CUDART_VERSION: u32 = 11060;
#[cfg(any(feature = "cuda-11070"))]
pub const CUDART_VERSION: u32 = 11070;
#[cfg(any(feature = "cuda-11080"))]
pub const CUDART_VERSION: u32 = 11080;
#[cfg(any(feature = "cuda-12000"))]
pub const CUDART_VERSION: u32 = 12000;
#[cfg(any(feature = "cuda-12010"))]
pub const CUDART_VERSION: u32 = 12010;
#[cfg(any(feature = "cuda-12020"))]
pub const CUDART_VERSION: u32 = 12020;
#[cfg(any(feature = "cuda-12030"))]
pub const CUDART_VERSION: u32 = 12030;
#[cfg(any(feature = "cuda-12040"))]
pub const CUDART_VERSION: u32 = 12040;
#[cfg(any(feature = "cuda-12050"))]
pub const CUDART_VERSION: u32 = 12050;
#[cfg(any(feature = "cuda-12060"))]
pub const CUDART_VERSION: u32 = 12060;
#[cfg(any(feature = "cuda-12080"))]
pub const CUDART_VERSION: u32 = 12080;
#[cfg(any(feature = "cuda-12090"))]
pub const CUDART_VERSION: u32 = 12090;
#[cfg(any(feature = "cuda-13000"))]
pub const CUDART_VERSION: u32 = 13000;
pub const CUDA_IPC_HANDLE_SIZE: u32 = 64;
pub const cudaArrayColorAttachment: u32 = 32;
pub const cudaArrayCubemap: u32 = 4;
pub const cudaArrayDefault: u32 = 0;
#[cfg(any(
    feature = "cuda-11060",
    feature = "cuda-11070",
    feature = "cuda-11080",
    feature = "cuda-12000",
    feature = "cuda-12010",
    feature = "cuda-12020",
    feature = "cuda-12030",
    feature = "cuda-12040",
    feature = "cuda-12050",
    feature = "cuda-12060",
    feature = "cuda-12080",
    feature = "cuda-12090",
    feature = "cuda-13000"
))]
pub const cudaArrayDeferredMapping: u32 = 128;
pub const cudaArrayLayered: u32 = 1;
pub const cudaArraySparse: u32 = 64;
pub const cudaArraySparsePropertiesSingleMipTail: u32 = 1;
pub const cudaArraySurfaceLoadStore: u32 = 2;
pub const cudaArrayTextureGather: u32 = 8;
#[cfg(any(
    feature = "cuda-11040",
    feature = "cuda-11050",
    feature = "cuda-11060",
    feature = "cuda-11070",
    feature = "cuda-11080",
    feature = "cuda-12000",
    feature = "cuda-12010",
    feature = "cuda-12020",
    feature = "cuda-12030",
    feature = "cuda-12040",
    feature = "cuda-12050",
    feature = "cuda-12060",
    feature = "cuda-12080",
    feature = "cuda-12090"
))]
pub const cudaCooperativeLaunchMultiDeviceNoPostSync: u32 = 2;
#[cfg(any(
    feature = "cuda-11040",
    feature = "cuda-11050",
    feature = "cuda-11060",
    feature = "cuda-11070",
    feature = "cuda-11080",
    feature = "cuda-12000",
    feature = "cuda-12010",
    feature = "cuda-12020",
    feature = "cuda-12030",
    feature = "cuda-12040",
    feature = "cuda-12050",
    feature = "cuda-12060",
    feature = "cuda-12080",
    feature = "cuda-12090"
))]
pub const cudaCooperativeLaunchMultiDeviceNoPreSync: u32 = 1;
pub const cudaDeviceBlockingSync: u32 = 4;
pub const cudaDeviceLmemResizeToMax: u32 = 16;
pub const cudaDeviceMapHost: u32 = 8;
#[cfg(any(
    feature = "cuda-11040",
    feature = "cuda-11050",
    feature = "cuda-11060",
    feature = "cuda-11070",
    feature = "cuda-11080",
    feature = "cuda-12000"
))]
pub const cudaDeviceMask: u32 = 31;
#[cfg(any(
    feature = "cuda-12010",
    feature = "cuda-12020",
    feature = "cuda-12030",
    feature = "cuda-12040",
    feature = "cuda-12050",
    feature = "cuda-12060",
    feature = "cuda-12080",
    feature = "cuda-12090",
    feature = "cuda-13000"
))]
pub const cudaDeviceMask: u32 = 255;
pub const cudaDeviceScheduleAuto: u32 = 0;
pub const cudaDeviceScheduleBlockingSync: u32 = 4;
pub const cudaDeviceScheduleMask: u32 = 7;
pub const cudaDeviceScheduleSpin: u32 = 1;
pub const cudaDeviceScheduleYield: u32 = 2;
#[cfg(any(
    feature = "cuda-12010",
    feature = "cuda-12020",
    feature = "cuda-12030",
    feature = "cuda-12040",
    feature = "cuda-12050",
    feature = "cuda-12060",
    feature = "cuda-12080",
    feature = "cuda-12090",
    feature = "cuda-13000"
))]
pub const cudaDeviceSyncMemops: u32 = 128;
pub const cudaEventBlockingSync: u32 = 1;
pub const cudaEventDefault: u32 = 0;
pub const cudaEventDisableTiming: u32 = 2;
pub const cudaEventInterprocess: u32 = 4;
pub const cudaEventRecordDefault: u32 = 0;
pub const cudaEventRecordExternal: u32 = 1;
pub const cudaEventWaitDefault: u32 = 0;
pub const cudaEventWaitExternal: u32 = 1;
pub const cudaExternalMemoryDedicated: u32 = 1;
pub const cudaExternalSemaphoreSignalSkipNvSciBufMemSync: u32 = 1;
pub const cudaExternalSemaphoreWaitSkipNvSciBufMemSync: u32 = 2;
#[cfg(any(
    feature = "cuda-12030",
    feature = "cuda-12040",
    feature = "cuda-12050",
    feature = "cuda-12060",
    feature = "cuda-12080",
    feature = "cuda-12090",
    feature = "cuda-13000"
))]
pub const cudaGraphKernelNodePortDefault: u32 = 0;
#[cfg(any(
    feature = "cuda-12030",
    feature = "cuda-12040",
    feature = "cuda-12050",
    feature = "cuda-12060",
    feature = "cuda-12080",
    feature = "cuda-12090",
    feature = "cuda-13000"
))]
pub const cudaGraphKernelNodePortLaunchCompletion: u32 = 2;
#[cfg(any(
    feature = "cuda-12030",
    feature = "cuda-12040",
    feature = "cuda-12050",
    feature = "cuda-12060",
    feature = "cuda-12080",
    feature = "cuda-12090",
    feature = "cuda-13000"
))]
pub const cudaGraphKernelNodePortProgrammatic: u32 = 1;
pub const cudaHostAllocDefault: u32 = 0;
pub const cudaHostAllocMapped: u32 = 2;
pub const cudaHostAllocPortable: u32 = 1;
pub const cudaHostAllocWriteCombined: u32 = 4;
pub const cudaHostRegisterDefault: u32 = 0;
pub const cudaHostRegisterIoMemory: u32 = 4;
pub const cudaHostRegisterMapped: u32 = 2;
pub const cudaHostRegisterPortable: u32 = 1;
pub const cudaHostRegisterReadOnly: u32 = 8;
#[cfg(any(
    feature = "cuda-12000",
    feature = "cuda-12010",
    feature = "cuda-12020",
    feature = "cuda-12030",
    feature = "cuda-12040",
    feature = "cuda-12050",
    feature = "cuda-12060",
    feature = "cuda-12080",
    feature = "cuda-12090",
    feature = "cuda-13000"
))]
pub const cudaInitDeviceFlagsAreValid: u32 = 1;
pub const cudaIpcMemLazyEnablePeerAccess: u32 = 1;
pub const cudaMemAttachGlobal: u32 = 1;
pub const cudaMemAttachHost: u32 = 2;
pub const cudaMemAttachSingle: u32 = 4;
#[cfg(any(feature = "cuda-12080", feature = "cuda-12090", feature = "cuda-13000"))]
pub const cudaMemPoolCreateUsageHwDecompress: u32 = 2;
pub const cudaNvSciSyncAttrSignal: u32 = 1;
pub const cudaNvSciSyncAttrWait: u32 = 2;
pub const cudaOccupancyDefault: u32 = 0;
pub const cudaOccupancyDisableCachingOverride: u32 = 1;
pub const cudaPeerAccessDefault: u32 = 0;
pub const cudaStreamDefault: u32 = 0;
pub const cudaStreamNonBlocking: u32 = 1;
pub const cudaSurfaceType1D: u32 = 1;
pub const cudaSurfaceType1DLayered: u32 = 241;
pub const cudaSurfaceType2D: u32 = 2;
pub const cudaSurfaceType2DLayered: u32 = 242;
pub const cudaSurfaceType3D: u32 = 3;
pub const cudaSurfaceTypeCubemap: u32 = 12;
pub const cudaSurfaceTypeCubemapLayered: u32 = 252;
pub const cudaTextureType1D: u32 = 1;
pub const cudaTextureType1DLayered: u32 = 241;
pub const cudaTextureType2D: u32 = 2;
pub const cudaTextureType2DLayered: u32 = 242;
pub const cudaTextureType3D: u32 = 3;
pub const cudaTextureTypeCubemap: u32 = 12;
pub const cudaTextureTypeCubemapLayered: u32 = 252;
pub type cudaArray_const_t = *const cudaArray;
pub type cudaArray_t = *mut cudaArray;
#[cfg(any(
    feature = "cuda-12040",
    feature = "cuda-12050",
    feature = "cuda-12060",
    feature = "cuda-12080",
    feature = "cuda-12090",
    feature = "cuda-13000"
))]
pub type cudaAsyncCallback = ::core::option::Option<
    unsafe extern "C" fn(
        arg1: *mut cudaAsyncNotificationInfo_t,
        arg2: *mut ::core::ffi::c_void,
        arg3: cudaAsyncCallbackHandle_t,
    ),
>;
#[cfg(any(
    feature = "cuda-12040",
    feature = "cuda-12050",
    feature = "cuda-12060",
    feature = "cuda-12080",
    feature = "cuda-12090",
    feature = "cuda-13000"
))]
pub type cudaAsyncCallbackHandle_t = *mut cudaAsyncCallbackEntry;
#[cfg(any(
    feature = "cuda-12040",
    feature = "cuda-12050",
    feature = "cuda-12060",
    feature = "cuda-12080",
    feature = "cuda-12090",
    feature = "cuda-13000"
))]
pub type cudaAsyncNotificationInfo_t = cudaAsyncNotificationInfo;
pub type cudaEvent_t = *mut CUevent_st;
pub type cudaExternalMemory_t = *mut CUexternalMemory_st;
pub type cudaExternalSemaphore_t = *mut CUexternalSemaphore_st;
pub type cudaFunction_t = *mut CUfunc_st;
#[cfg(any(
    feature = "cuda-12030",
    feature = "cuda-12040",
    feature = "cuda-12050",
    feature = "cuda-12060",
    feature = "cuda-12080",
    feature = "cuda-12090",
    feature = "cuda-13000"
))]
pub type cudaGraphConditionalHandle = ::core::ffi::c_ulonglong;
#[cfg(any(
    feature = "cuda-12040",
    feature = "cuda-12050",
    feature = "cuda-12060",
    feature = "cuda-12080",
    feature = "cuda-12090",
    feature = "cuda-13000"
))]
pub type cudaGraphDeviceNode_t = *mut CUgraphDeviceUpdatableNode_st;
#[cfg(any(
    feature = "cuda-12030",
    feature = "cuda-12040",
    feature = "cuda-12050",
    feature = "cuda-12060",
    feature = "cuda-12080",
    feature = "cuda-12090",
    feature = "cuda-13000"
))]
pub type cudaGraphEdgeData = cudaGraphEdgeData_st;
#[cfg(any(
    feature = "cuda-12000",
    feature = "cuda-12010",
    feature = "cuda-12020",
    feature = "cuda-12030",
    feature = "cuda-12040",
    feature = "cuda-12050",
    feature = "cuda-12060",
    feature = "cuda-12080",
    feature = "cuda-12090",
    feature = "cuda-13000"
))]
pub type cudaGraphExecUpdateResultInfo = cudaGraphExecUpdateResultInfo_st;
pub type cudaGraphExec_t = *mut CUgraphExec_st;
#[cfg(any(
    feature = "cuda-12000",
    feature = "cuda-12010",
    feature = "cuda-12020",
    feature = "cuda-12030",
    feature = "cuda-12040",
    feature = "cuda-12050",
    feature = "cuda-12060",
    feature = "cuda-12080",
    feature = "cuda-12090",
    feature = "cuda-13000"
))]
pub type cudaGraphInstantiateParams = cudaGraphInstantiateParams_st;
pub type cudaGraphNode_t = *mut CUgraphNode_st;
pub type cudaGraph_t = *mut CUgraph_st;
pub type cudaGraphicsResource_t = *mut cudaGraphicsResource;
pub type cudaHostFn_t =
    ::core::option::Option<unsafe extern "C" fn(userData: *mut ::core::ffi::c_void)>;
pub type cudaIpcEventHandle_t = cudaIpcEventHandle_st;
pub type cudaIpcMemHandle_t = cudaIpcMemHandle_st;
#[cfg(any(
    feature = "cuda-12010",
    feature = "cuda-12020",
    feature = "cuda-12030",
    feature = "cuda-12040",
    feature = "cuda-12050",
    feature = "cuda-12060",
    feature = "cuda-12080",
    feature = "cuda-12090",
    feature = "cuda-13000"
))]
pub type cudaKernel_t = *mut CUkern_st;
#[cfg(any(
    feature = "cuda-11080",
    feature = "cuda-12000",
    feature = "cuda-12010",
    feature = "cuda-12020",
    feature = "cuda-12030",
    feature = "cuda-12040",
    feature = "cuda-12050",
    feature = "cuda-12060",
    feature = "cuda-12080",
    feature = "cuda-12090",
    feature = "cuda-13000"
))]
pub type cudaLaunchAttribute = cudaLaunchAttribute_st;
#[cfg(any(
    feature = "cuda-11080",
    feature = "cuda-12000",
    feature = "cuda-12010",
    feature = "cuda-12020",
    feature = "cuda-12030",
    feature = "cuda-12040",
    feature = "cuda-12050",
    feature = "cuda-12060",
    feature = "cuda-12080",
    feature = "cuda-12090",
    feature = "cuda-13000"
))]
pub type cudaLaunchConfig_t = cudaLaunchConfig_st;
#[cfg(any(
    feature = "cuda-12000",
    feature = "cuda-12010",
    feature = "cuda-12020",
    feature = "cuda-12030",
    feature = "cuda-12040",
    feature = "cuda-12050",
    feature = "cuda-12060",
    feature = "cuda-12080",
    feature = "cuda-12090",
    feature = "cuda-13000"
))]
pub type cudaLaunchMemSyncDomainMap = cudaLaunchMemSyncDomainMap_st;
#[cfg(any(feature = "cuda-12080", feature = "cuda-12090", feature = "cuda-13000"))]
pub type cudaLibrary_t = *mut CUlib_st;
#[cfg(any(feature = "cuda-13000"))]
pub type cudaLogIterator = ::core::ffi::c_uint;
#[cfg(any(feature = "cuda-13000"))]
pub type cudaLogsCallbackHandle = *mut CUlogsCallbackEntry_st;
#[cfg(any(feature = "cuda-13000"))]
pub type cudaLogsCallback_t = ::core::option::Option<
    unsafe extern "C" fn(
        data: *mut ::core::ffi::c_void,
        logLevel: cudaLogLevel,
        message: *mut ::core::ffi::c_char,
        length: usize,
    ),
>;
#[cfg(any(
    feature = "cuda-12030",
    feature = "cuda-12040",
    feature = "cuda-12050",
    feature = "cuda-12060",
    feature = "cuda-12080",
    feature = "cuda-12090",
    feature = "cuda-13000"
))]
pub type cudaMemFabricHandle_t = cudaMemFabricHandle_st;
pub type cudaMemPool_t = *mut CUmemPoolHandle_st;
pub type cudaMipmappedArray_const_t = *const cudaMipmappedArray;
pub type cudaMipmappedArray_t = *mut cudaMipmappedArray;
pub type cudaStreamCallback_t = ::core::option::Option<
    unsafe extern "C" fn(
        stream: cudaStream_t,
        status: cudaError_t,
        userData: *mut ::core::ffi::c_void,
    ),
>;
pub type cudaStream_t = *mut CUstream_st;
pub type cudaSurfaceObject_t = ::core::ffi::c_ulonglong;
pub type cudaTextureObject_t = ::core::ffi::c_ulonglong;
pub type cudaUUID_t = CUuuid_st;
pub type cudaUserObject_t = *mut CUuserObject_st;
#[cfg(any(feature = "cuda-13000"))]
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub enum CUDAlogLevel_enum {
    cudaLogLevelError = 0,
    cudaLogLevelWarning = 1,
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub enum cudaAccessProperty {
    cudaAccessPropertyNormal = 0,
    cudaAccessPropertyStreaming = 1,
    cudaAccessPropertyPersisting = 2,
}
#[cfg(any(
    feature = "cuda-12040",
    feature = "cuda-12050",
    feature = "cuda-12060",
    feature = "cuda-12080",
    feature = "cuda-12090",
    feature = "cuda-13000"
))]
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub enum cudaAsyncNotificationType_enum {
    cudaAsyncNotificationTypeOverBudget = 1,
}
#[cfg(any(feature = "cuda-13000"))]
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub enum cudaAtomicOperation {
    cudaAtomicOperationIntegerAdd = 0,
    cudaAtomicOperationIntegerMin = 1,
    cudaAtomicOperationIntegerMax = 2,
    cudaAtomicOperationIntegerIncrement = 3,
    cudaAtomicOperationIntegerDecrement = 4,
    cudaAtomicOperationAnd = 5,
    cudaAtomicOperationOr = 6,
    cudaAtomicOperationXOR = 7,
    cudaAtomicOperationExchange = 8,
    cudaAtomicOperationCAS = 9,
    cudaAtomicOperationFloatAdd = 10,
    cudaAtomicOperationFloatMin = 11,
    cudaAtomicOperationFloatMax = 12,
}
#[cfg(any(feature = "cuda-13000"))]
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub enum cudaAtomicOperationCapability {
    cudaAtomicCapabilitySigned = 1,
    cudaAtomicCapabilityUnsigned = 2,
    cudaAtomicCapabilityReduction = 4,
    cudaAtomicCapabilityScalar32 = 8,
    cudaAtomicCapabilityScalar64 = 16,
    cudaAtomicCapabilityScalar128 = 32,
    cudaAtomicCapabilityVector32x4 = 64,
}
#[cfg(any(
    feature = "cuda-11040",
    feature = "cuda-11050",
    feature = "cuda-11060",
    feature = "cuda-11070",
    feature = "cuda-11080",
    feature = "cuda-12000",
    feature = "cuda-12010",
    feature = "cuda-12020",
    feature = "cuda-12030",
    feature = "cuda-12040",
    feature = "cuda-12050",
    feature = "cuda-12060",
    feature = "cuda-12080",
    feature = "cuda-12090"
))]
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub enum cudaCGScope {
    cudaCGScopeInvalid = 0,
    cudaCGScopeGrid = 1,
    cudaCGScopeMultiGrid = 2,
}
#[cfg(any(feature = "cuda-13000"))]
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub enum cudaCGScope {
    cudaCGScopeInvalid = 0,
    cudaCGScopeGrid = 1,
    cudaCGScopeReserved = 2,
}
#[cfg(any(feature = "cuda-11040"))]
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub enum cudaChannelFormatKind {
    cudaChannelFormatKindSigned = 0,
    cudaChannelFormatKindUnsigned = 1,
    cudaChannelFormatKindFloat = 2,
    cudaChannelFormatKindNone = 3,
    cudaChannelFormatKindNV12 = 4,
}
#[cfg(any(
    feature = "cuda-11050",
    feature = "cuda-11060",
    feature = "cuda-11070",
    feature = "cuda-11080",
    feature = "cuda-12000",
    feature = "cuda-12010",
    feature = "cuda-12020",
    feature = "cuda-12030",
    feature = "cuda-12040",
    feature = "cuda-12050",
    feature = "cuda-12060"
))]
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub enum cudaChannelFormatKind {
    cudaChannelFormatKindSigned = 0,
    cudaChannelFormatKindUnsigned = 1,
    cudaChannelFormatKindFloat = 2,
    cudaChannelFormatKindNone = 3,
    cudaChannelFormatKindNV12 = 4,
    cudaChannelFormatKindUnsignedNormalized8X1 = 5,
    cudaChannelFormatKindUnsignedNormalized8X2 = 6,
    cudaChannelFormatKindUnsignedNormalized8X4 = 7,
    cudaChannelFormatKindUnsignedNormalized16X1 = 8,
    cudaChannelFormatKindUnsignedNormalized16X2 = 9,
    cudaChannelFormatKindUnsignedNormalized16X4 = 10,
    cudaChannelFormatKindSignedNormalized8X1 = 11,
    cudaChannelFormatKindSignedNormalized8X2 = 12,
    cudaChannelFormatKindSignedNormalized8X4 = 13,
    cudaChannelFormatKindSignedNormalized16X1 = 14,
    cudaChannelFormatKindSignedNormalized16X2 = 15,
    cudaChannelFormatKindSignedNormalized16X4 = 16,
    cudaChannelFormatKindUnsignedBlockCompressed1 = 17,
    cudaChannelFormatKindUnsignedBlockCompressed1SRGB = 18,
    cudaChannelFormatKindUnsignedBlockCompressed2 = 19,
    cudaChannelFormatKindUnsignedBlockCompressed2SRGB = 20,
    cudaChannelFormatKindUnsignedBlockCompressed3 = 21,
    cudaChannelFormatKindUnsignedBlockCompressed3SRGB = 22,
    cudaChannelFormatKindUnsignedBlockCompressed4 = 23,
    cudaChannelFormatKindSignedBlockCompressed4 = 24,
    cudaChannelFormatKindUnsignedBlockCompressed5 = 25,
    cudaChannelFormatKindSignedBlockCompressed5 = 26,
    cudaChannelFormatKindUnsignedBlockCompressed6H = 27,
    cudaChannelFormatKindSignedBlockCompressed6H = 28,
    cudaChannelFormatKindUnsignedBlockCompressed7 = 29,
    cudaChannelFormatKindUnsignedBlockCompressed7SRGB = 30,
}
#[cfg(any(feature = "cuda-12080", feature = "cuda-12090", feature = "cuda-13000"))]
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub enum cudaChannelFormatKind {
    cudaChannelFormatKindSigned = 0,
    cudaChannelFormatKindUnsigned = 1,
    cudaChannelFormatKindFloat = 2,
    cudaChannelFormatKindNone = 3,
    cudaChannelFormatKindNV12 = 4,
    cudaChannelFormatKindUnsignedNormalized8X1 = 5,
    cudaChannelFormatKindUnsignedNormalized8X2 = 6,
    cudaChannelFormatKindUnsignedNormalized8X4 = 7,
    cudaChannelFormatKindUnsignedNormalized16X1 = 8,
    cudaChannelFormatKindUnsignedNormalized16X2 = 9,
    cudaChannelFormatKindUnsignedNormalized16X4 = 10,
    cudaChannelFormatKindSignedNormalized8X1 = 11,
    cudaChannelFormatKindSignedNormalized8X2 = 12,
    cudaChannelFormatKindSignedNormalized8X4 = 13,
    cudaChannelFormatKindSignedNormalized16X1 = 14,
    cudaChannelFormatKindSignedNormalized16X2 = 15,
    cudaChannelFormatKindSignedNormalized16X4 = 16,
    cudaChannelFormatKindUnsignedBlockCompressed1 = 17,
    cudaChannelFormatKindUnsignedBlockCompressed1SRGB = 18,
    cudaChannelFormatKindUnsignedBlockCompressed2 = 19,
    cudaChannelFormatKindUnsignedBlockCompressed2SRGB = 20,
    cudaChannelFormatKindUnsignedBlockCompressed3 = 21,
    cudaChannelFormatKindUnsignedBlockCompressed3SRGB = 22,
    cudaChannelFormatKindUnsignedBlockCompressed4 = 23,
    cudaChannelFormatKindSignedBlockCompressed4 = 24,
    cudaChannelFormatKindUnsignedBlockCompressed5 = 25,
    cudaChannelFormatKindSignedBlockCompressed5 = 26,
    cudaChannelFormatKindUnsignedBlockCompressed6H = 27,
    cudaChannelFormatKindSignedBlockCompressed6H = 28,
    cudaChannelFormatKindUnsignedBlockCompressed7 = 29,
    cudaChannelFormatKindUnsignedBlockCompressed7SRGB = 30,
    cudaChannelFormatKindUnsignedNormalized1010102 = 31,
}
#[cfg(any(
    feature = "cuda-11080",
    feature = "cuda-12000",
    feature = "cuda-12010",
    feature = "cuda-12020",
    feature = "cuda-12030",
    feature = "cuda-12040",
    feature = "cuda-12050",
    feature = "cuda-12060",
    feature = "cuda-12080",
    feature = "cuda-12090",
    feature = "cuda-13000"
))]
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub enum cudaClusterSchedulingPolicy {
    cudaClusterSchedulingPolicyDefault = 0,
    cudaClusterSchedulingPolicySpread = 1,
    cudaClusterSchedulingPolicyLoadBalancing = 2,
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub enum cudaComputeMode {
    cudaComputeModeDefault = 0,
    cudaComputeModeExclusive = 1,
    cudaComputeModeProhibited = 2,
    cudaComputeModeExclusiveProcess = 3,
}
#[cfg(any(
    feature = "cuda-11040",
    feature = "cuda-11050",
    feature = "cuda-11060",
    feature = "cuda-11070"
))]
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub enum cudaDataType_t {
    CUDA_R_16F = 2,
    CUDA_C_16F = 6,
    CUDA_R_16BF = 14,
    CUDA_C_16BF = 15,
    CUDA_R_32F = 0,
    CUDA_C_32F = 4,
    CUDA_R_64F = 1,
    CUDA_C_64F = 5,
    CUDA_R_4I = 16,
    CUDA_C_4I = 17,
    CUDA_R_4U = 18,
    CUDA_C_4U = 19,
    CUDA_R_8I = 3,
    CUDA_C_8I = 7,
    CUDA_R_8U = 8,
    CUDA_C_8U = 9,
    CUDA_R_16I = 20,
    CUDA_C_16I = 21,
    CUDA_R_16U = 22,
    CUDA_C_16U = 23,
    CUDA_R_32I = 10,
    CUDA_C_32I = 11,
    CUDA_R_32U = 12,
    CUDA_C_32U = 13,
    CUDA_R_64I = 24,
    CUDA_C_64I = 25,
    CUDA_R_64U = 26,
    CUDA_C_64U = 27,
}
#[cfg(any(
    feature = "cuda-11080",
    feature = "cuda-12000",
    feature = "cuda-12010",
    feature = "cuda-12020",
    feature = "cuda-12030",
    feature = "cuda-12040",
    feature = "cuda-12050",
    feature = "cuda-12060"
))]
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub enum cudaDataType_t {
    CUDA_R_16F = 2,
    CUDA_C_16F = 6,
    CUDA_R_16BF = 14,
    CUDA_C_16BF = 15,
    CUDA_R_32F = 0,
    CUDA_C_32F = 4,
    CUDA_R_64F = 1,
    CUDA_C_64F = 5,
    CUDA_R_4I = 16,
    CUDA_C_4I = 17,
    CUDA_R_4U = 18,
    CUDA_C_4U = 19,
    CUDA_R_8I = 3,
    CUDA_C_8I = 7,
    CUDA_R_8U = 8,
    CUDA_C_8U = 9,
    CUDA_R_16I = 20,
    CUDA_C_16I = 21,
    CUDA_R_16U = 22,
    CUDA_C_16U = 23,
    CUDA_R_32I = 10,
    CUDA_C_32I = 11,
    CUDA_R_32U = 12,
    CUDA_C_32U = 13,
    CUDA_R_64I = 24,
    CUDA_C_64I = 25,
    CUDA_R_64U = 26,
    CUDA_C_64U = 27,
    CUDA_R_8F_E4M3 = 28,
    CUDA_R_8F_E5M2 = 29,
}
#[cfg(any(feature = "cuda-12080", feature = "cuda-12090", feature = "cuda-13000"))]
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub enum cudaDataType_t {
    CUDA_R_16F = 2,
    CUDA_C_16F = 6,
    CUDA_R_16BF = 14,
    CUDA_C_16BF = 15,
    CUDA_R_32F = 0,
    CUDA_C_32F = 4,
    CUDA_R_64F = 1,
    CUDA_C_64F = 5,
    CUDA_R_4I = 16,
    CUDA_C_4I = 17,
    CUDA_R_4U = 18,
    CUDA_C_4U = 19,
    CUDA_R_8I = 3,
    CUDA_C_8I = 7,
    CUDA_R_8U = 8,
    CUDA_C_8U = 9,
    CUDA_R_16I = 20,
    CUDA_C_16I = 21,
    CUDA_R_16U = 22,
    CUDA_C_16U = 23,
    CUDA_R_32I = 10,
    CUDA_C_32I = 11,
    CUDA_R_32U = 12,
    CUDA_C_32U = 13,
    CUDA_R_64I = 24,
    CUDA_C_64I = 25,
    CUDA_R_64U = 26,
    CUDA_C_64U = 27,
    CUDA_R_8F_E4M3 = 28,
    CUDA_R_8F_E5M2 = 29,
    CUDA_R_8F_UE8M0 = 30,
    CUDA_R_6F_E2M3 = 31,
    CUDA_R_6F_E3M2 = 32,
    CUDA_R_4F_E2M1 = 33,
}
#[cfg(any(feature = "cuda-11040"))]
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub enum cudaDeviceAttr {
    cudaDevAttrMaxThreadsPerBlock = 1,
    cudaDevAttrMaxBlockDimX = 2,
    cudaDevAttrMaxBlockDimY = 3,
    cudaDevAttrMaxBlockDimZ = 4,
    cudaDevAttrMaxGridDimX = 5,
    cudaDevAttrMaxGridDimY = 6,
    cudaDevAttrMaxGridDimZ = 7,
    cudaDevAttrMaxSharedMemoryPerBlock = 8,
    cudaDevAttrTotalConstantMemory = 9,
    cudaDevAttrWarpSize = 10,
    cudaDevAttrMaxPitch = 11,
    cudaDevAttrMaxRegistersPerBlock = 12,
    cudaDevAttrClockRate = 13,
    cudaDevAttrTextureAlignment = 14,
    cudaDevAttrGpuOverlap = 15,
    cudaDevAttrMultiProcessorCount = 16,
    cudaDevAttrKernelExecTimeout = 17,
    cudaDevAttrIntegrated = 18,
    cudaDevAttrCanMapHostMemory = 19,
    cudaDevAttrComputeMode = 20,
    cudaDevAttrMaxTexture1DWidth = 21,
    cudaDevAttrMaxTexture2DWidth = 22,
    cudaDevAttrMaxTexture2DHeight = 23,
    cudaDevAttrMaxTexture3DWidth = 24,
    cudaDevAttrMaxTexture3DHeight = 25,
    cudaDevAttrMaxTexture3DDepth = 26,
    cudaDevAttrMaxTexture2DLayeredWidth = 27,
    cudaDevAttrMaxTexture2DLayeredHeight = 28,
    cudaDevAttrMaxTexture2DLayeredLayers = 29,
    cudaDevAttrSurfaceAlignment = 30,
    cudaDevAttrConcurrentKernels = 31,
    cudaDevAttrEccEnabled = 32,
    cudaDevAttrPciBusId = 33,
    cudaDevAttrPciDeviceId = 34,
    cudaDevAttrTccDriver = 35,
    cudaDevAttrMemoryClockRate = 36,
    cudaDevAttrGlobalMemoryBusWidth = 37,
    cudaDevAttrL2CacheSize = 38,
    cudaDevAttrMaxThreadsPerMultiProcessor = 39,
    cudaDevAttrAsyncEngineCount = 40,
    cudaDevAttrUnifiedAddressing = 41,
    cudaDevAttrMaxTexture1DLayeredWidth = 42,
    cudaDevAttrMaxTexture1DLayeredLayers = 43,
    cudaDevAttrMaxTexture2DGatherWidth = 45,
    cudaDevAttrMaxTexture2DGatherHeight = 46,
    cudaDevAttrMaxTexture3DWidthAlt = 47,
    cudaDevAttrMaxTexture3DHeightAlt = 48,
    cudaDevAttrMaxTexture3DDepthAlt = 49,
    cudaDevAttrPciDomainId = 50,
    cudaDevAttrTexturePitchAlignment = 51,
    cudaDevAttrMaxTextureCubemapWidth = 52,
    cudaDevAttrMaxTextureCubemapLayeredWidth = 53,
    cudaDevAttrMaxTextureCubemapLayeredLayers = 54,
    cudaDevAttrMaxSurface1DWidth = 55,
    cudaDevAttrMaxSurface2DWidth = 56,
    cudaDevAttrMaxSurface2DHeight = 57,
    cudaDevAttrMaxSurface3DWidth = 58,
    cudaDevAttrMaxSurface3DHeight = 59,
    cudaDevAttrMaxSurface3DDepth = 60,
    cudaDevAttrMaxSurface1DLayeredWidth = 61,
    cudaDevAttrMaxSurface1DLayeredLayers = 62,
    cudaDevAttrMaxSurface2DLayeredWidth = 63,
    cudaDevAttrMaxSurface2DLayeredHeight = 64,
    cudaDevAttrMaxSurface2DLayeredLayers = 65,
    cudaDevAttrMaxSurfaceCubemapWidth = 66,
    cudaDevAttrMaxSurfaceCubemapLayeredWidth = 67,
    cudaDevAttrMaxSurfaceCubemapLayeredLayers = 68,
    cudaDevAttrMaxTexture1DLinearWidth = 69,
    cudaDevAttrMaxTexture2DLinearWidth = 70,
    cudaDevAttrMaxTexture2DLinearHeight = 71,
    cudaDevAttrMaxTexture2DLinearPitch = 72,
    cudaDevAttrMaxTexture2DMipmappedWidth = 73,
    cudaDevAttrMaxTexture2DMipmappedHeight = 74,
    cudaDevAttrComputeCapabilityMajor = 75,
    cudaDevAttrComputeCapabilityMinor = 76,
    cudaDevAttrMaxTexture1DMipmappedWidth = 77,
    cudaDevAttrStreamPrioritiesSupported = 78,
    cudaDevAttrGlobalL1CacheSupported = 79,
    cudaDevAttrLocalL1CacheSupported = 80,
    cudaDevAttrMaxSharedMemoryPerMultiprocessor = 81,
    cudaDevAttrMaxRegistersPerMultiprocessor = 82,
    cudaDevAttrManagedMemory = 83,
    cudaDevAttrIsMultiGpuBoard = 84,
    cudaDevAttrMultiGpuBoardGroupID = 85,
    cudaDevAttrHostNativeAtomicSupported = 86,
    cudaDevAttrSingleToDoublePrecisionPerfRatio = 87,
    cudaDevAttrPageableMemoryAccess = 88,
    cudaDevAttrConcurrentManagedAccess = 89,
    cudaDevAttrComputePreemptionSupported = 90,
    cudaDevAttrCanUseHostPointerForRegisteredMem = 91,
    cudaDevAttrReserved92 = 92,
    cudaDevAttrReserved93 = 93,
    cudaDevAttrReserved94 = 94,
    cudaDevAttrCooperativeLaunch = 95,
    cudaDevAttrCooperativeMultiDeviceLaunch = 96,
    cudaDevAttrMaxSharedMemoryPerBlockOptin = 97,
    cudaDevAttrCanFlushRemoteWrites = 98,
    cudaDevAttrHostRegisterSupported = 99,
    cudaDevAttrPageableMemoryAccessUsesHostPageTables = 100,
    cudaDevAttrDirectManagedMemAccessFromHost = 101,
    cudaDevAttrMaxBlocksPerMultiprocessor = 106,
    cudaDevAttrMaxPersistingL2CacheSize = 108,
    cudaDevAttrMaxAccessPolicyWindowSize = 109,
    cudaDevAttrReservedSharedMemoryPerBlock = 111,
    cudaDevAttrSparseCudaArraySupported = 112,
    cudaDevAttrHostRegisterReadOnlySupported = 113,
    cudaDevAttrMaxTimelineSemaphoreInteropSupported = 114,
    cudaDevAttrMemoryPoolsSupported = 115,
    cudaDevAttrGPUDirectRDMASupported = 116,
    cudaDevAttrGPUDirectRDMAFlushWritesOptions = 117,
    cudaDevAttrGPUDirectRDMAWritesOrdering = 118,
    cudaDevAttrMemoryPoolSupportedHandleTypes = 119,
    cudaDevAttrMax = 120,
}
#[cfg(any(feature = "cuda-11050"))]
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub enum cudaDeviceAttr {
    cudaDevAttrMaxThreadsPerBlock = 1,
    cudaDevAttrMaxBlockDimX = 2,
    cudaDevAttrMaxBlockDimY = 3,
    cudaDevAttrMaxBlockDimZ = 4,
    cudaDevAttrMaxGridDimX = 5,
    cudaDevAttrMaxGridDimY = 6,
    cudaDevAttrMaxGridDimZ = 7,
    cudaDevAttrMaxSharedMemoryPerBlock = 8,
    cudaDevAttrTotalConstantMemory = 9,
    cudaDevAttrWarpSize = 10,
    cudaDevAttrMaxPitch = 11,
    cudaDevAttrMaxRegistersPerBlock = 12,
    cudaDevAttrClockRate = 13,
    cudaDevAttrTextureAlignment = 14,
    cudaDevAttrGpuOverlap = 15,
    cudaDevAttrMultiProcessorCount = 16,
    cudaDevAttrKernelExecTimeout = 17,
    cudaDevAttrIntegrated = 18,
    cudaDevAttrCanMapHostMemory = 19,
    cudaDevAttrComputeMode = 20,
    cudaDevAttrMaxTexture1DWidth = 21,
    cudaDevAttrMaxTexture2DWidth = 22,
    cudaDevAttrMaxTexture2DHeight = 23,
    cudaDevAttrMaxTexture3DWidth = 24,
    cudaDevAttrMaxTexture3DHeight = 25,
    cudaDevAttrMaxTexture3DDepth = 26,
    cudaDevAttrMaxTexture2DLayeredWidth = 27,
    cudaDevAttrMaxTexture2DLayeredHeight = 28,
    cudaDevAttrMaxTexture2DLayeredLayers = 29,
    cudaDevAttrSurfaceAlignment = 30,
    cudaDevAttrConcurrentKernels = 31,
    cudaDevAttrEccEnabled = 32,
    cudaDevAttrPciBusId = 33,
    cudaDevAttrPciDeviceId = 34,
    cudaDevAttrTccDriver = 35,
    cudaDevAttrMemoryClockRate = 36,
    cudaDevAttrGlobalMemoryBusWidth = 37,
    cudaDevAttrL2CacheSize = 38,
    cudaDevAttrMaxThreadsPerMultiProcessor = 39,
    cudaDevAttrAsyncEngineCount = 40,
    cudaDevAttrUnifiedAddressing = 41,
    cudaDevAttrMaxTexture1DLayeredWidth = 42,
    cudaDevAttrMaxTexture1DLayeredLayers = 43,
    cudaDevAttrMaxTexture2DGatherWidth = 45,
    cudaDevAttrMaxTexture2DGatherHeight = 46,
    cudaDevAttrMaxTexture3DWidthAlt = 47,
    cudaDevAttrMaxTexture3DHeightAlt = 48,
    cudaDevAttrMaxTexture3DDepthAlt = 49,
    cudaDevAttrPciDomainId = 50,
    cudaDevAttrTexturePitchAlignment = 51,
    cudaDevAttrMaxTextureCubemapWidth = 52,
    cudaDevAttrMaxTextureCubemapLayeredWidth = 53,
    cudaDevAttrMaxTextureCubemapLayeredLayers = 54,
    cudaDevAttrMaxSurface1DWidth = 55,
    cudaDevAttrMaxSurface2DWidth = 56,
    cudaDevAttrMaxSurface2DHeight = 57,
    cudaDevAttrMaxSurface3DWidth = 58,
    cudaDevAttrMaxSurface3DHeight = 59,
    cudaDevAttrMaxSurface3DDepth = 60,
    cudaDevAttrMaxSurface1DLayeredWidth = 61,
    cudaDevAttrMaxSurface1DLayeredLayers = 62,
    cudaDevAttrMaxSurface2DLayeredWidth = 63,
    cudaDevAttrMaxSurface2DLayeredHeight = 64,
    cudaDevAttrMaxSurface2DLayeredLayers = 65,
    cudaDevAttrMaxSurfaceCubemapWidth = 66,
    cudaDevAttrMaxSurfaceCubemapLayeredWidth = 67,
    cudaDevAttrMaxSurfaceCubemapLayeredLayers = 68,
    cudaDevAttrMaxTexture1DLinearWidth = 69,
    cudaDevAttrMaxTexture2DLinearWidth = 70,
    cudaDevAttrMaxTexture2DLinearHeight = 71,
    cudaDevAttrMaxTexture2DLinearPitch = 72,
    cudaDevAttrMaxTexture2DMipmappedWidth = 73,
    cudaDevAttrMaxTexture2DMipmappedHeight = 74,
    cudaDevAttrComputeCapabilityMajor = 75,
    cudaDevAttrComputeCapabilityMinor = 76,
    cudaDevAttrMaxTexture1DMipmappedWidth = 77,
    cudaDevAttrStreamPrioritiesSupported = 78,
    cudaDevAttrGlobalL1CacheSupported = 79,
    cudaDevAttrLocalL1CacheSupported = 80,
    cudaDevAttrMaxSharedMemoryPerMultiprocessor = 81,
    cudaDevAttrMaxRegistersPerMultiprocessor = 82,
    cudaDevAttrManagedMemory = 83,
    cudaDevAttrIsMultiGpuBoard = 84,
    cudaDevAttrMultiGpuBoardGroupID = 85,
    cudaDevAttrHostNativeAtomicSupported = 86,
    cudaDevAttrSingleToDoublePrecisionPerfRatio = 87,
    cudaDevAttrPageableMemoryAccess = 88,
    cudaDevAttrConcurrentManagedAccess = 89,
    cudaDevAttrComputePreemptionSupported = 90,
    cudaDevAttrCanUseHostPointerForRegisteredMem = 91,
    cudaDevAttrReserved92 = 92,
    cudaDevAttrReserved93 = 93,
    cudaDevAttrReserved94 = 94,
    cudaDevAttrCooperativeLaunch = 95,
    cudaDevAttrCooperativeMultiDeviceLaunch = 96,
    cudaDevAttrMaxSharedMemoryPerBlockOptin = 97,
    cudaDevAttrCanFlushRemoteWrites = 98,
    cudaDevAttrHostRegisterSupported = 99,
    cudaDevAttrPageableMemoryAccessUsesHostPageTables = 100,
    cudaDevAttrDirectManagedMemAccessFromHost = 101,
    cudaDevAttrMaxBlocksPerMultiprocessor = 106,
    cudaDevAttrMaxPersistingL2CacheSize = 108,
    cudaDevAttrMaxAccessPolicyWindowSize = 109,
    cudaDevAttrReservedSharedMemoryPerBlock = 111,
    cudaDevAttrSparseCudaArraySupported = 112,
    cudaDevAttrHostRegisterReadOnlySupported = 113,
    cudaDevAttrTimelineSemaphoreInteropSupported = 114,
    cudaDevAttrMemoryPoolsSupported = 115,
    cudaDevAttrGPUDirectRDMASupported = 116,
    cudaDevAttrGPUDirectRDMAFlushWritesOptions = 117,
    cudaDevAttrGPUDirectRDMAWritesOrdering = 118,
    cudaDevAttrMemoryPoolSupportedHandleTypes = 119,
    cudaDevAttrMax = 120,
}
#[cfg(any(feature = "cuda-11060", feature = "cuda-11070"))]
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub enum cudaDeviceAttr {
    cudaDevAttrMaxThreadsPerBlock = 1,
    cudaDevAttrMaxBlockDimX = 2,
    cudaDevAttrMaxBlockDimY = 3,
    cudaDevAttrMaxBlockDimZ = 4,
    cudaDevAttrMaxGridDimX = 5,
    cudaDevAttrMaxGridDimY = 6,
    cudaDevAttrMaxGridDimZ = 7,
    cudaDevAttrMaxSharedMemoryPerBlock = 8,
    cudaDevAttrTotalConstantMemory = 9,
    cudaDevAttrWarpSize = 10,
    cudaDevAttrMaxPitch = 11,
    cudaDevAttrMaxRegistersPerBlock = 12,
    cudaDevAttrClockRate = 13,
    cudaDevAttrTextureAlignment = 14,
    cudaDevAttrGpuOverlap = 15,
    cudaDevAttrMultiProcessorCount = 16,
    cudaDevAttrKernelExecTimeout = 17,
    cudaDevAttrIntegrated = 18,
    cudaDevAttrCanMapHostMemory = 19,
    cudaDevAttrComputeMode = 20,
    cudaDevAttrMaxTexture1DWidth = 21,
    cudaDevAttrMaxTexture2DWidth = 22,
    cudaDevAttrMaxTexture2DHeight = 23,
    cudaDevAttrMaxTexture3DWidth = 24,
    cudaDevAttrMaxTexture3DHeight = 25,
    cudaDevAttrMaxTexture3DDepth = 26,
    cudaDevAttrMaxTexture2DLayeredWidth = 27,
    cudaDevAttrMaxTexture2DLayeredHeight = 28,
    cudaDevAttrMaxTexture2DLayeredLayers = 29,
    cudaDevAttrSurfaceAlignment = 30,
    cudaDevAttrConcurrentKernels = 31,
    cudaDevAttrEccEnabled = 32,
    cudaDevAttrPciBusId = 33,
    cudaDevAttrPciDeviceId = 34,
    cudaDevAttrTccDriver = 35,
    cudaDevAttrMemoryClockRate = 36,
    cudaDevAttrGlobalMemoryBusWidth = 37,
    cudaDevAttrL2CacheSize = 38,
    cudaDevAttrMaxThreadsPerMultiProcessor = 39,
    cudaDevAttrAsyncEngineCount = 40,
    cudaDevAttrUnifiedAddressing = 41,
    cudaDevAttrMaxTexture1DLayeredWidth = 42,
    cudaDevAttrMaxTexture1DLayeredLayers = 43,
    cudaDevAttrMaxTexture2DGatherWidth = 45,
    cudaDevAttrMaxTexture2DGatherHeight = 46,
    cudaDevAttrMaxTexture3DWidthAlt = 47,
    cudaDevAttrMaxTexture3DHeightAlt = 48,
    cudaDevAttrMaxTexture3DDepthAlt = 49,
    cudaDevAttrPciDomainId = 50,
    cudaDevAttrTexturePitchAlignment = 51,
    cudaDevAttrMaxTextureCubemapWidth = 52,
    cudaDevAttrMaxTextureCubemapLayeredWidth = 53,
    cudaDevAttrMaxTextureCubemapLayeredLayers = 54,
    cudaDevAttrMaxSurface1DWidth = 55,
    cudaDevAttrMaxSurface2DWidth = 56,
    cudaDevAttrMaxSurface2DHeight = 57,
    cudaDevAttrMaxSurface3DWidth = 58,
    cudaDevAttrMaxSurface3DHeight = 59,
    cudaDevAttrMaxSurface3DDepth = 60,
    cudaDevAttrMaxSurface1DLayeredWidth = 61,
    cudaDevAttrMaxSurface1DLayeredLayers = 62,
    cudaDevAttrMaxSurface2DLayeredWidth = 63,
    cudaDevAttrMaxSurface2DLayeredHeight = 64,
    cudaDevAttrMaxSurface2DLayeredLayers = 65,
    cudaDevAttrMaxSurfaceCubemapWidth = 66,
    cudaDevAttrMaxSurfaceCubemapLayeredWidth = 67,
    cudaDevAttrMaxSurfaceCubemapLayeredLayers = 68,
    cudaDevAttrMaxTexture1DLinearWidth = 69,
    cudaDevAttrMaxTexture2DLinearWidth = 70,
    cudaDevAttrMaxTexture2DLinearHeight = 71,
    cudaDevAttrMaxTexture2DLinearPitch = 72,
    cudaDevAttrMaxTexture2DMipmappedWidth = 73,
    cudaDevAttrMaxTexture2DMipmappedHeight = 74,
    cudaDevAttrComputeCapabilityMajor = 75,
    cudaDevAttrComputeCapabilityMinor = 76,
    cudaDevAttrMaxTexture1DMipmappedWidth = 77,
    cudaDevAttrStreamPrioritiesSupported = 78,
    cudaDevAttrGlobalL1CacheSupported = 79,
    cudaDevAttrLocalL1CacheSupported = 80,
    cudaDevAttrMaxSharedMemoryPerMultiprocessor = 81,
    cudaDevAttrMaxRegistersPerMultiprocessor = 82,
    cudaDevAttrManagedMemory = 83,
    cudaDevAttrIsMultiGpuBoard = 84,
    cudaDevAttrMultiGpuBoardGroupID = 85,
    cudaDevAttrHostNativeAtomicSupported = 86,
    cudaDevAttrSingleToDoublePrecisionPerfRatio = 87,
    cudaDevAttrPageableMemoryAccess = 88,
    cudaDevAttrConcurrentManagedAccess = 89,
    cudaDevAttrComputePreemptionSupported = 90,
    cudaDevAttrCanUseHostPointerForRegisteredMem = 91,
    cudaDevAttrReserved92 = 92,
    cudaDevAttrReserved93 = 93,
    cudaDevAttrReserved94 = 94,
    cudaDevAttrCooperativeLaunch = 95,
    cudaDevAttrCooperativeMultiDeviceLaunch = 96,
    cudaDevAttrMaxSharedMemoryPerBlockOptin = 97,
    cudaDevAttrCanFlushRemoteWrites = 98,
    cudaDevAttrHostRegisterSupported = 99,
    cudaDevAttrPageableMemoryAccessUsesHostPageTables = 100,
    cudaDevAttrDirectManagedMemAccessFromHost = 101,
    cudaDevAttrMaxBlocksPerMultiprocessor = 106,
    cudaDevAttrMaxPersistingL2CacheSize = 108,
    cudaDevAttrMaxAccessPolicyWindowSize = 109,
    cudaDevAttrReservedSharedMemoryPerBlock = 111,
    cudaDevAttrSparseCudaArraySupported = 112,
    cudaDevAttrHostRegisterReadOnlySupported = 113,
    cudaDevAttrTimelineSemaphoreInteropSupported = 114,
    cudaDevAttrMemoryPoolsSupported = 115,
    cudaDevAttrGPUDirectRDMASupported = 116,
    cudaDevAttrGPUDirectRDMAFlushWritesOptions = 117,
    cudaDevAttrGPUDirectRDMAWritesOrdering = 118,
    cudaDevAttrMemoryPoolSupportedHandleTypes = 119,
    cudaDevAttrDeferredMappingCudaArraySupported = 121,
    cudaDevAttrMax = 122,
}
#[cfg(any(feature = "cuda-11080"))]
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub enum cudaDeviceAttr {
    cudaDevAttrMaxThreadsPerBlock = 1,
    cudaDevAttrMaxBlockDimX = 2,
    cudaDevAttrMaxBlockDimY = 3,
    cudaDevAttrMaxBlockDimZ = 4,
    cudaDevAttrMaxGridDimX = 5,
    cudaDevAttrMaxGridDimY = 6,
    cudaDevAttrMaxGridDimZ = 7,
    cudaDevAttrMaxSharedMemoryPerBlock = 8,
    cudaDevAttrTotalConstantMemory = 9,
    cudaDevAttrWarpSize = 10,
    cudaDevAttrMaxPitch = 11,
    cudaDevAttrMaxRegistersPerBlock = 12,
    cudaDevAttrClockRate = 13,
    cudaDevAttrTextureAlignment = 14,
    cudaDevAttrGpuOverlap = 15,
    cudaDevAttrMultiProcessorCount = 16,
    cudaDevAttrKernelExecTimeout = 17,
    cudaDevAttrIntegrated = 18,
    cudaDevAttrCanMapHostMemory = 19,
    cudaDevAttrComputeMode = 20,
    cudaDevAttrMaxTexture1DWidth = 21,
    cudaDevAttrMaxTexture2DWidth = 22,
    cudaDevAttrMaxTexture2DHeight = 23,
    cudaDevAttrMaxTexture3DWidth = 24,
    cudaDevAttrMaxTexture3DHeight = 25,
    cudaDevAttrMaxTexture3DDepth = 26,
    cudaDevAttrMaxTexture2DLayeredWidth = 27,
    cudaDevAttrMaxTexture2DLayeredHeight = 28,
    cudaDevAttrMaxTexture2DLayeredLayers = 29,
    cudaDevAttrSurfaceAlignment = 30,
    cudaDevAttrConcurrentKernels = 31,
    cudaDevAttrEccEnabled = 32,
    cudaDevAttrPciBusId = 33,
    cudaDevAttrPciDeviceId = 34,
    cudaDevAttrTccDriver = 35,
    cudaDevAttrMemoryClockRate = 36,
    cudaDevAttrGlobalMemoryBusWidth = 37,
    cudaDevAttrL2CacheSize = 38,
    cudaDevAttrMaxThreadsPerMultiProcessor = 39,
    cudaDevAttrAsyncEngineCount = 40,
    cudaDevAttrUnifiedAddressing = 41,
    cudaDevAttrMaxTexture1DLayeredWidth = 42,
    cudaDevAttrMaxTexture1DLayeredLayers = 43,
    cudaDevAttrMaxTexture2DGatherWidth = 45,
    cudaDevAttrMaxTexture2DGatherHeight = 46,
    cudaDevAttrMaxTexture3DWidthAlt = 47,
    cudaDevAttrMaxTexture3DHeightAlt = 48,
    cudaDevAttrMaxTexture3DDepthAlt = 49,
    cudaDevAttrPciDomainId = 50,
    cudaDevAttrTexturePitchAlignment = 51,
    cudaDevAttrMaxTextureCubemapWidth = 52,
    cudaDevAttrMaxTextureCubemapLayeredWidth = 53,
    cudaDevAttrMaxTextureCubemapLayeredLayers = 54,
    cudaDevAttrMaxSurface1DWidth = 55,
    cudaDevAttrMaxSurface2DWidth = 56,
    cudaDevAttrMaxSurface2DHeight = 57,
    cudaDevAttrMaxSurface3DWidth = 58,
    cudaDevAttrMaxSurface3DHeight = 59,
    cudaDevAttrMaxSurface3DDepth = 60,
    cudaDevAttrMaxSurface1DLayeredWidth = 61,
    cudaDevAttrMaxSurface1DLayeredLayers = 62,
    cudaDevAttrMaxSurface2DLayeredWidth = 63,
    cudaDevAttrMaxSurface2DLayeredHeight = 64,
    cudaDevAttrMaxSurface2DLayeredLayers = 65,
    cudaDevAttrMaxSurfaceCubemapWidth = 66,
    cudaDevAttrMaxSurfaceCubemapLayeredWidth = 67,
    cudaDevAttrMaxSurfaceCubemapLayeredLayers = 68,
    cudaDevAttrMaxTexture1DLinearWidth = 69,
    cudaDevAttrMaxTexture2DLinearWidth = 70,
    cudaDevAttrMaxTexture2DLinearHeight = 71,
    cudaDevAttrMaxTexture2DLinearPitch = 72,
    cudaDevAttrMaxTexture2DMipmappedWidth = 73,
    cudaDevAttrMaxTexture2DMipmappedHeight = 74,
    cudaDevAttrComputeCapabilityMajor = 75,
    cudaDevAttrComputeCapabilityMinor = 76,
    cudaDevAttrMaxTexture1DMipmappedWidth = 77,
    cudaDevAttrStreamPrioritiesSupported = 78,
    cudaDevAttrGlobalL1CacheSupported = 79,
    cudaDevAttrLocalL1CacheSupported = 80,
    cudaDevAttrMaxSharedMemoryPerMultiprocessor = 81,
    cudaDevAttrMaxRegistersPerMultiprocessor = 82,
    cudaDevAttrManagedMemory = 83,
    cudaDevAttrIsMultiGpuBoard = 84,
    cudaDevAttrMultiGpuBoardGroupID = 85,
    cudaDevAttrHostNativeAtomicSupported = 86,
    cudaDevAttrSingleToDoublePrecisionPerfRatio = 87,
    cudaDevAttrPageableMemoryAccess = 88,
    cudaDevAttrConcurrentManagedAccess = 89,
    cudaDevAttrComputePreemptionSupported = 90,
    cudaDevAttrCanUseHostPointerForRegisteredMem = 91,
    cudaDevAttrReserved92 = 92,
    cudaDevAttrReserved93 = 93,
    cudaDevAttrReserved94 = 94,
    cudaDevAttrCooperativeLaunch = 95,
    cudaDevAttrCooperativeMultiDeviceLaunch = 96,
    cudaDevAttrMaxSharedMemoryPerBlockOptin = 97,
    cudaDevAttrCanFlushRemoteWrites = 98,
    cudaDevAttrHostRegisterSupported = 99,
    cudaDevAttrPageableMemoryAccessUsesHostPageTables = 100,
    cudaDevAttrDirectManagedMemAccessFromHost = 101,
    cudaDevAttrMaxBlocksPerMultiprocessor = 106,
    cudaDevAttrMaxPersistingL2CacheSize = 108,
    cudaDevAttrMaxAccessPolicyWindowSize = 109,
    cudaDevAttrReservedSharedMemoryPerBlock = 111,
    cudaDevAttrSparseCudaArraySupported = 112,
    cudaDevAttrHostRegisterReadOnlySupported = 113,
    cudaDevAttrTimelineSemaphoreInteropSupported = 114,
    cudaDevAttrMemoryPoolsSupported = 115,
    cudaDevAttrGPUDirectRDMASupported = 116,
    cudaDevAttrGPUDirectRDMAFlushWritesOptions = 117,
    cudaDevAttrGPUDirectRDMAWritesOrdering = 118,
    cudaDevAttrMemoryPoolSupportedHandleTypes = 119,
    cudaDevAttrClusterLaunch = 120,
    cudaDevAttrDeferredMappingCudaArraySupported = 121,
    cudaDevAttrMax = 122,
}
#[cfg(any(feature = "cuda-12000"))]
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub enum cudaDeviceAttr {
    cudaDevAttrMaxThreadsPerBlock = 1,
    cudaDevAttrMaxBlockDimX = 2,
    cudaDevAttrMaxBlockDimY = 3,
    cudaDevAttrMaxBlockDimZ = 4,
    cudaDevAttrMaxGridDimX = 5,
    cudaDevAttrMaxGridDimY = 6,
    cudaDevAttrMaxGridDimZ = 7,
    cudaDevAttrMaxSharedMemoryPerBlock = 8,
    cudaDevAttrTotalConstantMemory = 9,
    cudaDevAttrWarpSize = 10,
    cudaDevAttrMaxPitch = 11,
    cudaDevAttrMaxRegistersPerBlock = 12,
    cudaDevAttrClockRate = 13,
    cudaDevAttrTextureAlignment = 14,
    cudaDevAttrGpuOverlap = 15,
    cudaDevAttrMultiProcessorCount = 16,
    cudaDevAttrKernelExecTimeout = 17,
    cudaDevAttrIntegrated = 18,
    cudaDevAttrCanMapHostMemory = 19,
    cudaDevAttrComputeMode = 20,
    cudaDevAttrMaxTexture1DWidth = 21,
    cudaDevAttrMaxTexture2DWidth = 22,
    cudaDevAttrMaxTexture2DHeight = 23,
    cudaDevAttrMaxTexture3DWidth = 24,
    cudaDevAttrMaxTexture3DHeight = 25,
    cudaDevAttrMaxTexture3DDepth = 26,
    cudaDevAttrMaxTexture2DLayeredWidth = 27,
    cudaDevAttrMaxTexture2DLayeredHeight = 28,
    cudaDevAttrMaxTexture2DLayeredLayers = 29,
    cudaDevAttrSurfaceAlignment = 30,
    cudaDevAttrConcurrentKernels = 31,
    cudaDevAttrEccEnabled = 32,
    cudaDevAttrPciBusId = 33,
    cudaDevAttrPciDeviceId = 34,
    cudaDevAttrTccDriver = 35,
    cudaDevAttrMemoryClockRate = 36,
    cudaDevAttrGlobalMemoryBusWidth = 37,
    cudaDevAttrL2CacheSize = 38,
    cudaDevAttrMaxThreadsPerMultiProcessor = 39,
    cudaDevAttrAsyncEngineCount = 40,
    cudaDevAttrUnifiedAddressing = 41,
    cudaDevAttrMaxTexture1DLayeredWidth = 42,
    cudaDevAttrMaxTexture1DLayeredLayers = 43,
    cudaDevAttrMaxTexture2DGatherWidth = 45,
    cudaDevAttrMaxTexture2DGatherHeight = 46,
    cudaDevAttrMaxTexture3DWidthAlt = 47,
    cudaDevAttrMaxTexture3DHeightAlt = 48,
    cudaDevAttrMaxTexture3DDepthAlt = 49,
    cudaDevAttrPciDomainId = 50,
    cudaDevAttrTexturePitchAlignment = 51,
    cudaDevAttrMaxTextureCubemapWidth = 52,
    cudaDevAttrMaxTextureCubemapLayeredWidth = 53,
    cudaDevAttrMaxTextureCubemapLayeredLayers = 54,
    cudaDevAttrMaxSurface1DWidth = 55,
    cudaDevAttrMaxSurface2DWidth = 56,
    cudaDevAttrMaxSurface2DHeight = 57,
    cudaDevAttrMaxSurface3DWidth = 58,
    cudaDevAttrMaxSurface3DHeight = 59,
    cudaDevAttrMaxSurface3DDepth = 60,
    cudaDevAttrMaxSurface1DLayeredWidth = 61,
    cudaDevAttrMaxSurface1DLayeredLayers = 62,
    cudaDevAttrMaxSurface2DLayeredWidth = 63,
    cudaDevAttrMaxSurface2DLayeredHeight = 64,
    cudaDevAttrMaxSurface2DLayeredLayers = 65,
    cudaDevAttrMaxSurfaceCubemapWidth = 66,
    cudaDevAttrMaxSurfaceCubemapLayeredWidth = 67,
    cudaDevAttrMaxSurfaceCubemapLayeredLayers = 68,
    cudaDevAttrMaxTexture1DLinearWidth = 69,
    cudaDevAttrMaxTexture2DLinearWidth = 70,
    cudaDevAttrMaxTexture2DLinearHeight = 71,
    cudaDevAttrMaxTexture2DLinearPitch = 72,
    cudaDevAttrMaxTexture2DMipmappedWidth = 73,
    cudaDevAttrMaxTexture2DMipmappedHeight = 74,
    cudaDevAttrComputeCapabilityMajor = 75,
    cudaDevAttrComputeCapabilityMinor = 76,
    cudaDevAttrMaxTexture1DMipmappedWidth = 77,
    cudaDevAttrStreamPrioritiesSupported = 78,
    cudaDevAttrGlobalL1CacheSupported = 79,
    cudaDevAttrLocalL1CacheSupported = 80,
    cudaDevAttrMaxSharedMemoryPerMultiprocessor = 81,
    cudaDevAttrMaxRegistersPerMultiprocessor = 82,
    cudaDevAttrManagedMemory = 83,
    cudaDevAttrIsMultiGpuBoard = 84,
    cudaDevAttrMultiGpuBoardGroupID = 85,
    cudaDevAttrHostNativeAtomicSupported = 86,
    cudaDevAttrSingleToDoublePrecisionPerfRatio = 87,
    cudaDevAttrPageableMemoryAccess = 88,
    cudaDevAttrConcurrentManagedAccess = 89,
    cudaDevAttrComputePreemptionSupported = 90,
    cudaDevAttrCanUseHostPointerForRegisteredMem = 91,
    cudaDevAttrReserved92 = 92,
    cudaDevAttrReserved93 = 93,
    cudaDevAttrReserved94 = 94,
    cudaDevAttrCooperativeLaunch = 95,
    cudaDevAttrCooperativeMultiDeviceLaunch = 96,
    cudaDevAttrMaxSharedMemoryPerBlockOptin = 97,
    cudaDevAttrCanFlushRemoteWrites = 98,
    cudaDevAttrHostRegisterSupported = 99,
    cudaDevAttrPageableMemoryAccessUsesHostPageTables = 100,
    cudaDevAttrDirectManagedMemAccessFromHost = 101,
    cudaDevAttrMaxBlocksPerMultiprocessor = 106,
    cudaDevAttrMaxPersistingL2CacheSize = 108,
    cudaDevAttrMaxAccessPolicyWindowSize = 109,
    cudaDevAttrReservedSharedMemoryPerBlock = 111,
    cudaDevAttrSparseCudaArraySupported = 112,
    cudaDevAttrHostRegisterReadOnlySupported = 113,
    cudaDevAttrTimelineSemaphoreInteropSupported = 114,
    cudaDevAttrMemoryPoolsSupported = 115,
    cudaDevAttrGPUDirectRDMASupported = 116,
    cudaDevAttrGPUDirectRDMAFlushWritesOptions = 117,
    cudaDevAttrGPUDirectRDMAWritesOrdering = 118,
    cudaDevAttrMemoryPoolSupportedHandleTypes = 119,
    cudaDevAttrClusterLaunch = 120,
    cudaDevAttrDeferredMappingCudaArraySupported = 121,
    cudaDevAttrReserved122 = 122,
    cudaDevAttrReserved123 = 123,
    cudaDevAttrReserved124 = 124,
    cudaDevAttrIpcEventSupport = 125,
    cudaDevAttrMemSyncDomainCount = 126,
    cudaDevAttrMax = 127,
}
#[cfg(any(feature = "cuda-12010"))]
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub enum cudaDeviceAttr {
    cudaDevAttrMaxThreadsPerBlock = 1,
    cudaDevAttrMaxBlockDimX = 2,
    cudaDevAttrMaxBlockDimY = 3,
    cudaDevAttrMaxBlockDimZ = 4,
    cudaDevAttrMaxGridDimX = 5,
    cudaDevAttrMaxGridDimY = 6,
    cudaDevAttrMaxGridDimZ = 7,
    cudaDevAttrMaxSharedMemoryPerBlock = 8,
    cudaDevAttrTotalConstantMemory = 9,
    cudaDevAttrWarpSize = 10,
    cudaDevAttrMaxPitch = 11,
    cudaDevAttrMaxRegistersPerBlock = 12,
    cudaDevAttrClockRate = 13,
    cudaDevAttrTextureAlignment = 14,
    cudaDevAttrGpuOverlap = 15,
    cudaDevAttrMultiProcessorCount = 16,
    cudaDevAttrKernelExecTimeout = 17,
    cudaDevAttrIntegrated = 18,
    cudaDevAttrCanMapHostMemory = 19,
    cudaDevAttrComputeMode = 20,
    cudaDevAttrMaxTexture1DWidth = 21,
    cudaDevAttrMaxTexture2DWidth = 22,
    cudaDevAttrMaxTexture2DHeight = 23,
    cudaDevAttrMaxTexture3DWidth = 24,
    cudaDevAttrMaxTexture3DHeight = 25,
    cudaDevAttrMaxTexture3DDepth = 26,
    cudaDevAttrMaxTexture2DLayeredWidth = 27,
    cudaDevAttrMaxTexture2DLayeredHeight = 28,
    cudaDevAttrMaxTexture2DLayeredLayers = 29,
    cudaDevAttrSurfaceAlignment = 30,
    cudaDevAttrConcurrentKernels = 31,
    cudaDevAttrEccEnabled = 32,
    cudaDevAttrPciBusId = 33,
    cudaDevAttrPciDeviceId = 34,
    cudaDevAttrTccDriver = 35,
    cudaDevAttrMemoryClockRate = 36,
    cudaDevAttrGlobalMemoryBusWidth = 37,
    cudaDevAttrL2CacheSize = 38,
    cudaDevAttrMaxThreadsPerMultiProcessor = 39,
    cudaDevAttrAsyncEngineCount = 40,
    cudaDevAttrUnifiedAddressing = 41,
    cudaDevAttrMaxTexture1DLayeredWidth = 42,
    cudaDevAttrMaxTexture1DLayeredLayers = 43,
    cudaDevAttrMaxTexture2DGatherWidth = 45,
    cudaDevAttrMaxTexture2DGatherHeight = 46,
    cudaDevAttrMaxTexture3DWidthAlt = 47,
    cudaDevAttrMaxTexture3DHeightAlt = 48,
    cudaDevAttrMaxTexture3DDepthAlt = 49,
    cudaDevAttrPciDomainId = 50,
    cudaDevAttrTexturePitchAlignment = 51,
    cudaDevAttrMaxTextureCubemapWidth = 52,
    cudaDevAttrMaxTextureCubemapLayeredWidth = 53,
    cudaDevAttrMaxTextureCubemapLayeredLayers = 54,
    cudaDevAttrMaxSurface1DWidth = 55,
    cudaDevAttrMaxSurface2DWidth = 56,
    cudaDevAttrMaxSurface2DHeight = 57,
    cudaDevAttrMaxSurface3DWidth = 58,
    cudaDevAttrMaxSurface3DHeight = 59,
    cudaDevAttrMaxSurface3DDepth = 60,
    cudaDevAttrMaxSurface1DLayeredWidth = 61,
    cudaDevAttrMaxSurface1DLayeredLayers = 62,
    cudaDevAttrMaxSurface2DLayeredWidth = 63,
    cudaDevAttrMaxSurface2DLayeredHeight = 64,
    cudaDevAttrMaxSurface2DLayeredLayers = 65,
    cudaDevAttrMaxSurfaceCubemapWidth = 66,
    cudaDevAttrMaxSurfaceCubemapLayeredWidth = 67,
    cudaDevAttrMaxSurfaceCubemapLayeredLayers = 68,
    cudaDevAttrMaxTexture1DLinearWidth = 69,
    cudaDevAttrMaxTexture2DLinearWidth = 70,
    cudaDevAttrMaxTexture2DLinearHeight = 71,
    cudaDevAttrMaxTexture2DLinearPitch = 72,
    cudaDevAttrMaxTexture2DMipmappedWidth = 73,
    cudaDevAttrMaxTexture2DMipmappedHeight = 74,
    cudaDevAttrComputeCapabilityMajor = 75,
    cudaDevAttrComputeCapabilityMinor = 76,
    cudaDevAttrMaxTexture1DMipmappedWidth = 77,
    cudaDevAttrStreamPrioritiesSupported = 78,
    cudaDevAttrGlobalL1CacheSupported = 79,
    cudaDevAttrLocalL1CacheSupported = 80,
    cudaDevAttrMaxSharedMemoryPerMultiprocessor = 81,
    cudaDevAttrMaxRegistersPerMultiprocessor = 82,
    cudaDevAttrManagedMemory = 83,
    cudaDevAttrIsMultiGpuBoard = 84,
    cudaDevAttrMultiGpuBoardGroupID = 85,
    cudaDevAttrHostNativeAtomicSupported = 86,
    cudaDevAttrSingleToDoublePrecisionPerfRatio = 87,
    cudaDevAttrPageableMemoryAccess = 88,
    cudaDevAttrConcurrentManagedAccess = 89,
    cudaDevAttrComputePreemptionSupported = 90,
    cudaDevAttrCanUseHostPointerForRegisteredMem = 91,
    cudaDevAttrReserved92 = 92,
    cudaDevAttrReserved93 = 93,
    cudaDevAttrReserved94 = 94,
    cudaDevAttrCooperativeLaunch = 95,
    cudaDevAttrCooperativeMultiDeviceLaunch = 96,
    cudaDevAttrMaxSharedMemoryPerBlockOptin = 97,
    cudaDevAttrCanFlushRemoteWrites = 98,
    cudaDevAttrHostRegisterSupported = 99,
    cudaDevAttrPageableMemoryAccessUsesHostPageTables = 100,
    cudaDevAttrDirectManagedMemAccessFromHost = 101,
    cudaDevAttrMaxBlocksPerMultiprocessor = 106,
    cudaDevAttrMaxPersistingL2CacheSize = 108,
    cudaDevAttrMaxAccessPolicyWindowSize = 109,
    cudaDevAttrReservedSharedMemoryPerBlock = 111,
    cudaDevAttrSparseCudaArraySupported = 112,
    cudaDevAttrHostRegisterReadOnlySupported = 113,
    cudaDevAttrTimelineSemaphoreInteropSupported = 114,
    cudaDevAttrMemoryPoolsSupported = 115,
    cudaDevAttrGPUDirectRDMASupported = 116,
    cudaDevAttrGPUDirectRDMAFlushWritesOptions = 117,
    cudaDevAttrGPUDirectRDMAWritesOrdering = 118,
    cudaDevAttrMemoryPoolSupportedHandleTypes = 119,
    cudaDevAttrClusterLaunch = 120,
    cudaDevAttrDeferredMappingCudaArraySupported = 121,
    cudaDevAttrReserved122 = 122,
    cudaDevAttrReserved123 = 123,
    cudaDevAttrReserved124 = 124,
    cudaDevAttrIpcEventSupport = 125,
    cudaDevAttrMemSyncDomainCount = 126,
    cudaDevAttrReserved127 = 127,
    cudaDevAttrReserved128 = 128,
    cudaDevAttrReserved129 = 129,
    cudaDevAttrReserved132 = 132,
    cudaDevAttrMax = 133,
}
#[cfg(any(feature = "cuda-12020"))]
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub enum cudaDeviceAttr {
    cudaDevAttrMaxThreadsPerBlock = 1,
    cudaDevAttrMaxBlockDimX = 2,
    cudaDevAttrMaxBlockDimY = 3,
    cudaDevAttrMaxBlockDimZ = 4,
    cudaDevAttrMaxGridDimX = 5,
    cudaDevAttrMaxGridDimY = 6,
    cudaDevAttrMaxGridDimZ = 7,
    cudaDevAttrMaxSharedMemoryPerBlock = 8,
    cudaDevAttrTotalConstantMemory = 9,
    cudaDevAttrWarpSize = 10,
    cudaDevAttrMaxPitch = 11,
    cudaDevAttrMaxRegistersPerBlock = 12,
    cudaDevAttrClockRate = 13,
    cudaDevAttrTextureAlignment = 14,
    cudaDevAttrGpuOverlap = 15,
    cudaDevAttrMultiProcessorCount = 16,
    cudaDevAttrKernelExecTimeout = 17,
    cudaDevAttrIntegrated = 18,
    cudaDevAttrCanMapHostMemory = 19,
    cudaDevAttrComputeMode = 20,
    cudaDevAttrMaxTexture1DWidth = 21,
    cudaDevAttrMaxTexture2DWidth = 22,
    cudaDevAttrMaxTexture2DHeight = 23,
    cudaDevAttrMaxTexture3DWidth = 24,
    cudaDevAttrMaxTexture3DHeight = 25,
    cudaDevAttrMaxTexture3DDepth = 26,
    cudaDevAttrMaxTexture2DLayeredWidth = 27,
    cudaDevAttrMaxTexture2DLayeredHeight = 28,
    cudaDevAttrMaxTexture2DLayeredLayers = 29,
    cudaDevAttrSurfaceAlignment = 30,
    cudaDevAttrConcurrentKernels = 31,
    cudaDevAttrEccEnabled = 32,
    cudaDevAttrPciBusId = 33,
    cudaDevAttrPciDeviceId = 34,
    cudaDevAttrTccDriver = 35,
    cudaDevAttrMemoryClockRate = 36,
    cudaDevAttrGlobalMemoryBusWidth = 37,
    cudaDevAttrL2CacheSize = 38,
    cudaDevAttrMaxThreadsPerMultiProcessor = 39,
    cudaDevAttrAsyncEngineCount = 40,
    cudaDevAttrUnifiedAddressing = 41,
    cudaDevAttrMaxTexture1DLayeredWidth = 42,
    cudaDevAttrMaxTexture1DLayeredLayers = 43,
    cudaDevAttrMaxTexture2DGatherWidth = 45,
    cudaDevAttrMaxTexture2DGatherHeight = 46,
    cudaDevAttrMaxTexture3DWidthAlt = 47,
    cudaDevAttrMaxTexture3DHeightAlt = 48,
    cudaDevAttrMaxTexture3DDepthAlt = 49,
    cudaDevAttrPciDomainId = 50,
    cudaDevAttrTexturePitchAlignment = 51,
    cudaDevAttrMaxTextureCubemapWidth = 52,
    cudaDevAttrMaxTextureCubemapLayeredWidth = 53,
    cudaDevAttrMaxTextureCubemapLayeredLayers = 54,
    cudaDevAttrMaxSurface1DWidth = 55,
    cudaDevAttrMaxSurface2DWidth = 56,
    cudaDevAttrMaxSurface2DHeight = 57,
    cudaDevAttrMaxSurface3DWidth = 58,
    cudaDevAttrMaxSurface3DHeight = 59,
    cudaDevAttrMaxSurface3DDepth = 60,
    cudaDevAttrMaxSurface1DLayeredWidth = 61,
    cudaDevAttrMaxSurface1DLayeredLayers = 62,
    cudaDevAttrMaxSurface2DLayeredWidth = 63,
    cudaDevAttrMaxSurface2DLayeredHeight = 64,
    cudaDevAttrMaxSurface2DLayeredLayers = 65,
    cudaDevAttrMaxSurfaceCubemapWidth = 66,
    cudaDevAttrMaxSurfaceCubemapLayeredWidth = 67,
    cudaDevAttrMaxSurfaceCubemapLayeredLayers = 68,
    cudaDevAttrMaxTexture1DLinearWidth = 69,
    cudaDevAttrMaxTexture2DLinearWidth = 70,
    cudaDevAttrMaxTexture2DLinearHeight = 71,
    cudaDevAttrMaxTexture2DLinearPitch = 72,
    cudaDevAttrMaxTexture2DMipmappedWidth = 73,
    cudaDevAttrMaxTexture2DMipmappedHeight = 74,
    cudaDevAttrComputeCapabilityMajor = 75,
    cudaDevAttrComputeCapabilityMinor = 76,
    cudaDevAttrMaxTexture1DMipmappedWidth = 77,
    cudaDevAttrStreamPrioritiesSupported = 78,
    cudaDevAttrGlobalL1CacheSupported = 79,
    cudaDevAttrLocalL1CacheSupported = 80,
    cudaDevAttrMaxSharedMemoryPerMultiprocessor = 81,
    cudaDevAttrMaxRegistersPerMultiprocessor = 82,
    cudaDevAttrManagedMemory = 83,
    cudaDevAttrIsMultiGpuBoard = 84,
    cudaDevAttrMultiGpuBoardGroupID = 85,
    cudaDevAttrHostNativeAtomicSupported = 86,
    cudaDevAttrSingleToDoublePrecisionPerfRatio = 87,
    cudaDevAttrPageableMemoryAccess = 88,
    cudaDevAttrConcurrentManagedAccess = 89,
    cudaDevAttrComputePreemptionSupported = 90,
    cudaDevAttrCanUseHostPointerForRegisteredMem = 91,
    cudaDevAttrReserved92 = 92,
    cudaDevAttrReserved93 = 93,
    cudaDevAttrReserved94 = 94,
    cudaDevAttrCooperativeLaunch = 95,
    cudaDevAttrCooperativeMultiDeviceLaunch = 96,
    cudaDevAttrMaxSharedMemoryPerBlockOptin = 97,
    cudaDevAttrCanFlushRemoteWrites = 98,
    cudaDevAttrHostRegisterSupported = 99,
    cudaDevAttrPageableMemoryAccessUsesHostPageTables = 100,
    cudaDevAttrDirectManagedMemAccessFromHost = 101,
    cudaDevAttrMaxBlocksPerMultiprocessor = 106,
    cudaDevAttrMaxPersistingL2CacheSize = 108,
    cudaDevAttrMaxAccessPolicyWindowSize = 109,
    cudaDevAttrReservedSharedMemoryPerBlock = 111,
    cudaDevAttrSparseCudaArraySupported = 112,
    cudaDevAttrHostRegisterReadOnlySupported = 113,
    cudaDevAttrTimelineSemaphoreInteropSupported = 114,
    cudaDevAttrMemoryPoolsSupported = 115,
    cudaDevAttrGPUDirectRDMASupported = 116,
    cudaDevAttrGPUDirectRDMAFlushWritesOptions = 117,
    cudaDevAttrGPUDirectRDMAWritesOrdering = 118,
    cudaDevAttrMemoryPoolSupportedHandleTypes = 119,
    cudaDevAttrClusterLaunch = 120,
    cudaDevAttrDeferredMappingCudaArraySupported = 121,
    cudaDevAttrReserved122 = 122,
    cudaDevAttrReserved123 = 123,
    cudaDevAttrReserved124 = 124,
    cudaDevAttrIpcEventSupport = 125,
    cudaDevAttrMemSyncDomainCount = 126,
    cudaDevAttrReserved127 = 127,
    cudaDevAttrReserved128 = 128,
    cudaDevAttrReserved129 = 129,
    cudaDevAttrNumaConfig = 130,
    cudaDevAttrNumaId = 131,
    cudaDevAttrReserved132 = 132,
    cudaDevAttrHostNumaId = 134,
    cudaDevAttrMax = 135,
}
#[cfg(any(feature = "cuda-12030", feature = "cuda-12040"))]
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub enum cudaDeviceAttr {
    cudaDevAttrMaxThreadsPerBlock = 1,
    cudaDevAttrMaxBlockDimX = 2,
    cudaDevAttrMaxBlockDimY = 3,
    cudaDevAttrMaxBlockDimZ = 4,
    cudaDevAttrMaxGridDimX = 5,
    cudaDevAttrMaxGridDimY = 6,
    cudaDevAttrMaxGridDimZ = 7,
    cudaDevAttrMaxSharedMemoryPerBlock = 8,
    cudaDevAttrTotalConstantMemory = 9,
    cudaDevAttrWarpSize = 10,
    cudaDevAttrMaxPitch = 11,
    cudaDevAttrMaxRegistersPerBlock = 12,
    cudaDevAttrClockRate = 13,
    cudaDevAttrTextureAlignment = 14,
    cudaDevAttrGpuOverlap = 15,
    cudaDevAttrMultiProcessorCount = 16,
    cudaDevAttrKernelExecTimeout = 17,
    cudaDevAttrIntegrated = 18,
    cudaDevAttrCanMapHostMemory = 19,
    cudaDevAttrComputeMode = 20,
    cudaDevAttrMaxTexture1DWidth = 21,
    cudaDevAttrMaxTexture2DWidth = 22,
    cudaDevAttrMaxTexture2DHeight = 23,
    cudaDevAttrMaxTexture3DWidth = 24,
    cudaDevAttrMaxTexture3DHeight = 25,
    cudaDevAttrMaxTexture3DDepth = 26,
    cudaDevAttrMaxTexture2DLayeredWidth = 27,
    cudaDevAttrMaxTexture2DLayeredHeight = 28,
    cudaDevAttrMaxTexture2DLayeredLayers = 29,
    cudaDevAttrSurfaceAlignment = 30,
    cudaDevAttrConcurrentKernels = 31,
    cudaDevAttrEccEnabled = 32,
    cudaDevAttrPciBusId = 33,
    cudaDevAttrPciDeviceId = 34,
    cudaDevAttrTccDriver = 35,
    cudaDevAttrMemoryClockRate = 36,
    cudaDevAttrGlobalMemoryBusWidth = 37,
    cudaDevAttrL2CacheSize = 38,
    cudaDevAttrMaxThreadsPerMultiProcessor = 39,
    cudaDevAttrAsyncEngineCount = 40,
    cudaDevAttrUnifiedAddressing = 41,
    cudaDevAttrMaxTexture1DLayeredWidth = 42,
    cudaDevAttrMaxTexture1DLayeredLayers = 43,
    cudaDevAttrMaxTexture2DGatherWidth = 45,
    cudaDevAttrMaxTexture2DGatherHeight = 46,
    cudaDevAttrMaxTexture3DWidthAlt = 47,
    cudaDevAttrMaxTexture3DHeightAlt = 48,
    cudaDevAttrMaxTexture3DDepthAlt = 49,
    cudaDevAttrPciDomainId = 50,
    cudaDevAttrTexturePitchAlignment = 51,
    cudaDevAttrMaxTextureCubemapWidth = 52,
    cudaDevAttrMaxTextureCubemapLayeredWidth = 53,
    cudaDevAttrMaxTextureCubemapLayeredLayers = 54,
    cudaDevAttrMaxSurface1DWidth = 55,
    cudaDevAttrMaxSurface2DWidth = 56,
    cudaDevAttrMaxSurface2DHeight = 57,
    cudaDevAttrMaxSurface3DWidth = 58,
    cudaDevAttrMaxSurface3DHeight = 59,
    cudaDevAttrMaxSurface3DDepth = 60,
    cudaDevAttrMaxSurface1DLayeredWidth = 61,
    cudaDevAttrMaxSurface1DLayeredLayers = 62,
    cudaDevAttrMaxSurface2DLayeredWidth = 63,
    cudaDevAttrMaxSurface2DLayeredHeight = 64,
    cudaDevAttrMaxSurface2DLayeredLayers = 65,
    cudaDevAttrMaxSurfaceCubemapWidth = 66,
    cudaDevAttrMaxSurfaceCubemapLayeredWidth = 67,
    cudaDevAttrMaxSurfaceCubemapLayeredLayers = 68,
    cudaDevAttrMaxTexture1DLinearWidth = 69,
    cudaDevAttrMaxTexture2DLinearWidth = 70,
    cudaDevAttrMaxTexture2DLinearHeight = 71,
    cudaDevAttrMaxTexture2DLinearPitch = 72,
    cudaDevAttrMaxTexture2DMipmappedWidth = 73,
    cudaDevAttrMaxTexture2DMipmappedHeight = 74,
    cudaDevAttrComputeCapabilityMajor = 75,
    cudaDevAttrComputeCapabilityMinor = 76,
    cudaDevAttrMaxTexture1DMipmappedWidth = 77,
    cudaDevAttrStreamPrioritiesSupported = 78,
    cudaDevAttrGlobalL1CacheSupported = 79,
    cudaDevAttrLocalL1CacheSupported = 80,
    cudaDevAttrMaxSharedMemoryPerMultiprocessor = 81,
    cudaDevAttrMaxRegistersPerMultiprocessor = 82,
    cudaDevAttrManagedMemory = 83,
    cudaDevAttrIsMultiGpuBoard = 84,
    cudaDevAttrMultiGpuBoardGroupID = 85,
    cudaDevAttrHostNativeAtomicSupported = 86,
    cudaDevAttrSingleToDoublePrecisionPerfRatio = 87,
    cudaDevAttrPageableMemoryAccess = 88,
    cudaDevAttrConcurrentManagedAccess = 89,
    cudaDevAttrComputePreemptionSupported = 90,
    cudaDevAttrCanUseHostPointerForRegisteredMem = 91,
    cudaDevAttrReserved92 = 92,
    cudaDevAttrReserved93 = 93,
    cudaDevAttrReserved94 = 94,
    cudaDevAttrCooperativeLaunch = 95,
    cudaDevAttrCooperativeMultiDeviceLaunch = 96,
    cudaDevAttrMaxSharedMemoryPerBlockOptin = 97,
    cudaDevAttrCanFlushRemoteWrites = 98,
    cudaDevAttrHostRegisterSupported = 99,
    cudaDevAttrPageableMemoryAccessUsesHostPageTables = 100,
    cudaDevAttrDirectManagedMemAccessFromHost = 101,
    cudaDevAttrMaxBlocksPerMultiprocessor = 106,
    cudaDevAttrMaxPersistingL2CacheSize = 108,
    cudaDevAttrMaxAccessPolicyWindowSize = 109,
    cudaDevAttrReservedSharedMemoryPerBlock = 111,
    cudaDevAttrSparseCudaArraySupported = 112,
    cudaDevAttrHostRegisterReadOnlySupported = 113,
    cudaDevAttrTimelineSemaphoreInteropSupported = 114,
    cudaDevAttrMemoryPoolsSupported = 115,
    cudaDevAttrGPUDirectRDMASupported = 116,
    cudaDevAttrGPUDirectRDMAFlushWritesOptions = 117,
    cudaDevAttrGPUDirectRDMAWritesOrdering = 118,
    cudaDevAttrMemoryPoolSupportedHandleTypes = 119,
    cudaDevAttrClusterLaunch = 120,
    cudaDevAttrDeferredMappingCudaArraySupported = 121,
    cudaDevAttrReserved122 = 122,
    cudaDevAttrReserved123 = 123,
    cudaDevAttrReserved124 = 124,
    cudaDevAttrIpcEventSupport = 125,
    cudaDevAttrMemSyncDomainCount = 126,
    cudaDevAttrReserved127 = 127,
    cudaDevAttrReserved128 = 128,
    cudaDevAttrReserved129 = 129,
    cudaDevAttrNumaConfig = 130,
    cudaDevAttrNumaId = 131,
    cudaDevAttrReserved132 = 132,
    cudaDevAttrMpsEnabled = 133,
    cudaDevAttrHostNumaId = 134,
    cudaDevAttrMax = 135,
}
#[cfg(any(feature = "cuda-12050", feature = "cuda-12060"))]
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub enum cudaDeviceAttr {
    cudaDevAttrMaxThreadsPerBlock = 1,
    cudaDevAttrMaxBlockDimX = 2,
    cudaDevAttrMaxBlockDimY = 3,
    cudaDevAttrMaxBlockDimZ = 4,
    cudaDevAttrMaxGridDimX = 5,
    cudaDevAttrMaxGridDimY = 6,
    cudaDevAttrMaxGridDimZ = 7,
    cudaDevAttrMaxSharedMemoryPerBlock = 8,
    cudaDevAttrTotalConstantMemory = 9,
    cudaDevAttrWarpSize = 10,
    cudaDevAttrMaxPitch = 11,
    cudaDevAttrMaxRegistersPerBlock = 12,
    cudaDevAttrClockRate = 13,
    cudaDevAttrTextureAlignment = 14,
    cudaDevAttrGpuOverlap = 15,
    cudaDevAttrMultiProcessorCount = 16,
    cudaDevAttrKernelExecTimeout = 17,
    cudaDevAttrIntegrated = 18,
    cudaDevAttrCanMapHostMemory = 19,
    cudaDevAttrComputeMode = 20,
    cudaDevAttrMaxTexture1DWidth = 21,
    cudaDevAttrMaxTexture2DWidth = 22,
    cudaDevAttrMaxTexture2DHeight = 23,
    cudaDevAttrMaxTexture3DWidth = 24,
    cudaDevAttrMaxTexture3DHeight = 25,
    cudaDevAttrMaxTexture3DDepth = 26,
    cudaDevAttrMaxTexture2DLayeredWidth = 27,
    cudaDevAttrMaxTexture2DLayeredHeight = 28,
    cudaDevAttrMaxTexture2DLayeredLayers = 29,
    cudaDevAttrSurfaceAlignment = 30,
    cudaDevAttrConcurrentKernels = 31,
    cudaDevAttrEccEnabled = 32,
    cudaDevAttrPciBusId = 33,
    cudaDevAttrPciDeviceId = 34,
    cudaDevAttrTccDriver = 35,
    cudaDevAttrMemoryClockRate = 36,
    cudaDevAttrGlobalMemoryBusWidth = 37,
    cudaDevAttrL2CacheSize = 38,
    cudaDevAttrMaxThreadsPerMultiProcessor = 39,
    cudaDevAttrAsyncEngineCount = 40,
    cudaDevAttrUnifiedAddressing = 41,
    cudaDevAttrMaxTexture1DLayeredWidth = 42,
    cudaDevAttrMaxTexture1DLayeredLayers = 43,
    cudaDevAttrMaxTexture2DGatherWidth = 45,
    cudaDevAttrMaxTexture2DGatherHeight = 46,
    cudaDevAttrMaxTexture3DWidthAlt = 47,
    cudaDevAttrMaxTexture3DHeightAlt = 48,
    cudaDevAttrMaxTexture3DDepthAlt = 49,
    cudaDevAttrPciDomainId = 50,
    cudaDevAttrTexturePitchAlignment = 51,
    cudaDevAttrMaxTextureCubemapWidth = 52,
    cudaDevAttrMaxTextureCubemapLayeredWidth = 53,
    cudaDevAttrMaxTextureCubemapLayeredLayers = 54,
    cudaDevAttrMaxSurface1DWidth = 55,
    cudaDevAttrMaxSurface2DWidth = 56,
    cudaDevAttrMaxSurface2DHeight = 57,
    cudaDevAttrMaxSurface3DWidth = 58,
    cudaDevAttrMaxSurface3DHeight = 59,
    cudaDevAttrMaxSurface3DDepth = 60,
    cudaDevAttrMaxSurface1DLayeredWidth = 61,
    cudaDevAttrMaxSurface1DLayeredLayers = 62,
    cudaDevAttrMaxSurface2DLayeredWidth = 63,
    cudaDevAttrMaxSurface2DLayeredHeight = 64,
    cudaDevAttrMaxSurface2DLayeredLayers = 65,
    cudaDevAttrMaxSurfaceCubemapWidth = 66,
    cudaDevAttrMaxSurfaceCubemapLayeredWidth = 67,
    cudaDevAttrMaxSurfaceCubemapLayeredLayers = 68,
    cudaDevAttrMaxTexture1DLinearWidth = 69,
    cudaDevAttrMaxTexture2DLinearWidth = 70,
    cudaDevAttrMaxTexture2DLinearHeight = 71,
    cudaDevAttrMaxTexture2DLinearPitch = 72,
    cudaDevAttrMaxTexture2DMipmappedWidth = 73,
    cudaDevAttrMaxTexture2DMipmappedHeight = 74,
    cudaDevAttrComputeCapabilityMajor = 75,
    cudaDevAttrComputeCapabilityMinor = 76,
    cudaDevAttrMaxTexture1DMipmappedWidth = 77,
    cudaDevAttrStreamPrioritiesSupported = 78,
    cudaDevAttrGlobalL1CacheSupported = 79,
    cudaDevAttrLocalL1CacheSupported = 80,
    cudaDevAttrMaxSharedMemoryPerMultiprocessor = 81,
    cudaDevAttrMaxRegistersPerMultiprocessor = 82,
    cudaDevAttrManagedMemory = 83,
    cudaDevAttrIsMultiGpuBoard = 84,
    cudaDevAttrMultiGpuBoardGroupID = 85,
    cudaDevAttrHostNativeAtomicSupported = 86,
    cudaDevAttrSingleToDoublePrecisionPerfRatio = 87,
    cudaDevAttrPageableMemoryAccess = 88,
    cudaDevAttrConcurrentManagedAccess = 89,
    cudaDevAttrComputePreemptionSupported = 90,
    cudaDevAttrCanUseHostPointerForRegisteredMem = 91,
    cudaDevAttrReserved92 = 92,
    cudaDevAttrReserved93 = 93,
    cudaDevAttrReserved94 = 94,
    cudaDevAttrCooperativeLaunch = 95,
    cudaDevAttrCooperativeMultiDeviceLaunch = 96,
    cudaDevAttrMaxSharedMemoryPerBlockOptin = 97,
    cudaDevAttrCanFlushRemoteWrites = 98,
    cudaDevAttrHostRegisterSupported = 99,
    cudaDevAttrPageableMemoryAccessUsesHostPageTables = 100,
    cudaDevAttrDirectManagedMemAccessFromHost = 101,
    cudaDevAttrMaxBlocksPerMultiprocessor = 106,
    cudaDevAttrMaxPersistingL2CacheSize = 108,
    cudaDevAttrMaxAccessPolicyWindowSize = 109,
    cudaDevAttrReservedSharedMemoryPerBlock = 111,
    cudaDevAttrSparseCudaArraySupported = 112,
    cudaDevAttrHostRegisterReadOnlySupported = 113,
    cudaDevAttrTimelineSemaphoreInteropSupported = 114,
    cudaDevAttrMemoryPoolsSupported = 115,
    cudaDevAttrGPUDirectRDMASupported = 116,
    cudaDevAttrGPUDirectRDMAFlushWritesOptions = 117,
    cudaDevAttrGPUDirectRDMAWritesOrdering = 118,
    cudaDevAttrMemoryPoolSupportedHandleTypes = 119,
    cudaDevAttrClusterLaunch = 120,
    cudaDevAttrDeferredMappingCudaArraySupported = 121,
    cudaDevAttrReserved122 = 122,
    cudaDevAttrReserved123 = 123,
    cudaDevAttrReserved124 = 124,
    cudaDevAttrIpcEventSupport = 125,
    cudaDevAttrMemSyncDomainCount = 126,
    cudaDevAttrReserved127 = 127,
    cudaDevAttrReserved128 = 128,
    cudaDevAttrReserved129 = 129,
    cudaDevAttrNumaConfig = 130,
    cudaDevAttrNumaId = 131,
    cudaDevAttrReserved132 = 132,
    cudaDevAttrMpsEnabled = 133,
    cudaDevAttrHostNumaId = 134,
    cudaDevAttrD3D12CigSupported = 135,
    cudaDevAttrMax = 136,
}
#[cfg(any(feature = "cuda-12080"))]
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub enum cudaDeviceAttr {
    cudaDevAttrMaxThreadsPerBlock = 1,
    cudaDevAttrMaxBlockDimX = 2,
    cudaDevAttrMaxBlockDimY = 3,
    cudaDevAttrMaxBlockDimZ = 4,
    cudaDevAttrMaxGridDimX = 5,
    cudaDevAttrMaxGridDimY = 6,
    cudaDevAttrMaxGridDimZ = 7,
    cudaDevAttrMaxSharedMemoryPerBlock = 8,
    cudaDevAttrTotalConstantMemory = 9,
    cudaDevAttrWarpSize = 10,
    cudaDevAttrMaxPitch = 11,
    cudaDevAttrMaxRegistersPerBlock = 12,
    cudaDevAttrClockRate = 13,
    cudaDevAttrTextureAlignment = 14,
    cudaDevAttrGpuOverlap = 15,
    cudaDevAttrMultiProcessorCount = 16,
    cudaDevAttrKernelExecTimeout = 17,
    cudaDevAttrIntegrated = 18,
    cudaDevAttrCanMapHostMemory = 19,
    cudaDevAttrComputeMode = 20,
    cudaDevAttrMaxTexture1DWidth = 21,
    cudaDevAttrMaxTexture2DWidth = 22,
    cudaDevAttrMaxTexture2DHeight = 23,
    cudaDevAttrMaxTexture3DWidth = 24,
    cudaDevAttrMaxTexture3DHeight = 25,
    cudaDevAttrMaxTexture3DDepth = 26,
    cudaDevAttrMaxTexture2DLayeredWidth = 27,
    cudaDevAttrMaxTexture2DLayeredHeight = 28,
    cudaDevAttrMaxTexture2DLayeredLayers = 29,
    cudaDevAttrSurfaceAlignment = 30,
    cudaDevAttrConcurrentKernels = 31,
    cudaDevAttrEccEnabled = 32,
    cudaDevAttrPciBusId = 33,
    cudaDevAttrPciDeviceId = 34,
    cudaDevAttrTccDriver = 35,
    cudaDevAttrMemoryClockRate = 36,
    cudaDevAttrGlobalMemoryBusWidth = 37,
    cudaDevAttrL2CacheSize = 38,
    cudaDevAttrMaxThreadsPerMultiProcessor = 39,
    cudaDevAttrAsyncEngineCount = 40,
    cudaDevAttrUnifiedAddressing = 41,
    cudaDevAttrMaxTexture1DLayeredWidth = 42,
    cudaDevAttrMaxTexture1DLayeredLayers = 43,
    cudaDevAttrMaxTexture2DGatherWidth = 45,
    cudaDevAttrMaxTexture2DGatherHeight = 46,
    cudaDevAttrMaxTexture3DWidthAlt = 47,
    cudaDevAttrMaxTexture3DHeightAlt = 48,
    cudaDevAttrMaxTexture3DDepthAlt = 49,
    cudaDevAttrPciDomainId = 50,
    cudaDevAttrTexturePitchAlignment = 51,
    cudaDevAttrMaxTextureCubemapWidth = 52,
    cudaDevAttrMaxTextureCubemapLayeredWidth = 53,
    cudaDevAttrMaxTextureCubemapLayeredLayers = 54,
    cudaDevAttrMaxSurface1DWidth = 55,
    cudaDevAttrMaxSurface2DWidth = 56,
    cudaDevAttrMaxSurface2DHeight = 57,
    cudaDevAttrMaxSurface3DWidth = 58,
    cudaDevAttrMaxSurface3DHeight = 59,
    cudaDevAttrMaxSurface3DDepth = 60,
    cudaDevAttrMaxSurface1DLayeredWidth = 61,
    cudaDevAttrMaxSurface1DLayeredLayers = 62,
    cudaDevAttrMaxSurface2DLayeredWidth = 63,
    cudaDevAttrMaxSurface2DLayeredHeight = 64,
    cudaDevAttrMaxSurface2DLayeredLayers = 65,
    cudaDevAttrMaxSurfaceCubemapWidth = 66,
    cudaDevAttrMaxSurfaceCubemapLayeredWidth = 67,
    cudaDevAttrMaxSurfaceCubemapLayeredLayers = 68,
    cudaDevAttrMaxTexture1DLinearWidth = 69,
    cudaDevAttrMaxTexture2DLinearWidth = 70,
    cudaDevAttrMaxTexture2DLinearHeight = 71,
    cudaDevAttrMaxTexture2DLinearPitch = 72,
    cudaDevAttrMaxTexture2DMipmappedWidth = 73,
    cudaDevAttrMaxTexture2DMipmappedHeight = 74,
    cudaDevAttrComputeCapabilityMajor = 75,
    cudaDevAttrComputeCapabilityMinor = 76,
    cudaDevAttrMaxTexture1DMipmappedWidth = 77,
    cudaDevAttrStreamPrioritiesSupported = 78,
    cudaDevAttrGlobalL1CacheSupported = 79,
    cudaDevAttrLocalL1CacheSupported = 80,
    cudaDevAttrMaxSharedMemoryPerMultiprocessor = 81,
    cudaDevAttrMaxRegistersPerMultiprocessor = 82,
    cudaDevAttrManagedMemory = 83,
    cudaDevAttrIsMultiGpuBoard = 84,
    cudaDevAttrMultiGpuBoardGroupID = 85,
    cudaDevAttrHostNativeAtomicSupported = 86,
    cudaDevAttrSingleToDoublePrecisionPerfRatio = 87,
    cudaDevAttrPageableMemoryAccess = 88,
    cudaDevAttrConcurrentManagedAccess = 89,
    cudaDevAttrComputePreemptionSupported = 90,
    cudaDevAttrCanUseHostPointerForRegisteredMem = 91,
    cudaDevAttrReserved92 = 92,
    cudaDevAttrReserved93 = 93,
    cudaDevAttrReserved94 = 94,
    cudaDevAttrCooperativeLaunch = 95,
    cudaDevAttrCooperativeMultiDeviceLaunch = 96,
    cudaDevAttrMaxSharedMemoryPerBlockOptin = 97,
    cudaDevAttrCanFlushRemoteWrites = 98,
    cudaDevAttrHostRegisterSupported = 99,
    cudaDevAttrPageableMemoryAccessUsesHostPageTables = 100,
    cudaDevAttrDirectManagedMemAccessFromHost = 101,
    cudaDevAttrMaxBlocksPerMultiprocessor = 106,
    cudaDevAttrMaxPersistingL2CacheSize = 108,
    cudaDevAttrMaxAccessPolicyWindowSize = 109,
    cudaDevAttrReservedSharedMemoryPerBlock = 111,
    cudaDevAttrSparseCudaArraySupported = 112,
    cudaDevAttrHostRegisterReadOnlySupported = 113,
    cudaDevAttrTimelineSemaphoreInteropSupported = 114,
    cudaDevAttrMemoryPoolsSupported = 115,
    cudaDevAttrGPUDirectRDMASupported = 116,
    cudaDevAttrGPUDirectRDMAFlushWritesOptions = 117,
    cudaDevAttrGPUDirectRDMAWritesOrdering = 118,
    cudaDevAttrMemoryPoolSupportedHandleTypes = 119,
    cudaDevAttrClusterLaunch = 120,
    cudaDevAttrDeferredMappingCudaArraySupported = 121,
    cudaDevAttrReserved122 = 122,
    cudaDevAttrReserved123 = 123,
    cudaDevAttrReserved124 = 124,
    cudaDevAttrIpcEventSupport = 125,
    cudaDevAttrMemSyncDomainCount = 126,
    cudaDevAttrReserved127 = 127,
    cudaDevAttrReserved128 = 128,
    cudaDevAttrReserved129 = 129,
    cudaDevAttrNumaConfig = 130,
    cudaDevAttrNumaId = 131,
    cudaDevAttrReserved132 = 132,
    cudaDevAttrMpsEnabled = 133,
    cudaDevAttrHostNumaId = 134,
    cudaDevAttrD3D12CigSupported = 135,
    cudaDevAttrGpuPciDeviceId = 139,
    cudaDevAttrGpuPciSubsystemId = 140,
    cudaDevAttrHostNumaMultinodeIpcSupported = 143,
    cudaDevAttrMax = 144,
}
#[cfg(any(feature = "cuda-12090"))]
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub enum cudaDeviceAttr {
    cudaDevAttrMaxThreadsPerBlock = 1,
    cudaDevAttrMaxBlockDimX = 2,
    cudaDevAttrMaxBlockDimY = 3,
    cudaDevAttrMaxBlockDimZ = 4,
    cudaDevAttrMaxGridDimX = 5,
    cudaDevAttrMaxGridDimY = 6,
    cudaDevAttrMaxGridDimZ = 7,
    cudaDevAttrMaxSharedMemoryPerBlock = 8,
    cudaDevAttrTotalConstantMemory = 9,
    cudaDevAttrWarpSize = 10,
    cudaDevAttrMaxPitch = 11,
    cudaDevAttrMaxRegistersPerBlock = 12,
    cudaDevAttrClockRate = 13,
    cudaDevAttrTextureAlignment = 14,
    cudaDevAttrGpuOverlap = 15,
    cudaDevAttrMultiProcessorCount = 16,
    cudaDevAttrKernelExecTimeout = 17,
    cudaDevAttrIntegrated = 18,
    cudaDevAttrCanMapHostMemory = 19,
    cudaDevAttrComputeMode = 20,
    cudaDevAttrMaxTexture1DWidth = 21,
    cudaDevAttrMaxTexture2DWidth = 22,
    cudaDevAttrMaxTexture2DHeight = 23,
    cudaDevAttrMaxTexture3DWidth = 24,
    cudaDevAttrMaxTexture3DHeight = 25,
    cudaDevAttrMaxTexture3DDepth = 26,
    cudaDevAttrMaxTexture2DLayeredWidth = 27,
    cudaDevAttrMaxTexture2DLayeredHeight = 28,
    cudaDevAttrMaxTexture2DLayeredLayers = 29,
    cudaDevAttrSurfaceAlignment = 30,
    cudaDevAttrConcurrentKernels = 31,
    cudaDevAttrEccEnabled = 32,
    cudaDevAttrPciBusId = 33,
    cudaDevAttrPciDeviceId = 34,
    cudaDevAttrTccDriver = 35,
    cudaDevAttrMemoryClockRate = 36,
    cudaDevAttrGlobalMemoryBusWidth = 37,
    cudaDevAttrL2CacheSize = 38,
    cudaDevAttrMaxThreadsPerMultiProcessor = 39,
    cudaDevAttrAsyncEngineCount = 40,
    cudaDevAttrUnifiedAddressing = 41,
    cudaDevAttrMaxTexture1DLayeredWidth = 42,
    cudaDevAttrMaxTexture1DLayeredLayers = 43,
    cudaDevAttrMaxTexture2DGatherWidth = 45,
    cudaDevAttrMaxTexture2DGatherHeight = 46,
    cudaDevAttrMaxTexture3DWidthAlt = 47,
    cudaDevAttrMaxTexture3DHeightAlt = 48,
    cudaDevAttrMaxTexture3DDepthAlt = 49,
    cudaDevAttrPciDomainId = 50,
    cudaDevAttrTexturePitchAlignment = 51,
    cudaDevAttrMaxTextureCubemapWidth = 52,
    cudaDevAttrMaxTextureCubemapLayeredWidth = 53,
    cudaDevAttrMaxTextureCubemapLayeredLayers = 54,
    cudaDevAttrMaxSurface1DWidth = 55,
    cudaDevAttrMaxSurface2DWidth = 56,
    cudaDevAttrMaxSurface2DHeight = 57,
    cudaDevAttrMaxSurface3DWidth = 58,
    cudaDevAttrMaxSurface3DHeight = 59,
    cudaDevAttrMaxSurface3DDepth = 60,
    cudaDevAttrMaxSurface1DLayeredWidth = 61,
    cudaDevAttrMaxSurface1DLayeredLayers = 62,
    cudaDevAttrMaxSurface2DLayeredWidth = 63,
    cudaDevAttrMaxSurface2DLayeredHeight = 64,
    cudaDevAttrMaxSurface2DLayeredLayers = 65,
    cudaDevAttrMaxSurfaceCubemapWidth = 66,
    cudaDevAttrMaxSurfaceCubemapLayeredWidth = 67,
    cudaDevAttrMaxSurfaceCubemapLayeredLayers = 68,
    cudaDevAttrMaxTexture1DLinearWidth = 69,
    cudaDevAttrMaxTexture2DLinearWidth = 70,
    cudaDevAttrMaxTexture2DLinearHeight = 71,
    cudaDevAttrMaxTexture2DLinearPitch = 72,
    cudaDevAttrMaxTexture2DMipmappedWidth = 73,
    cudaDevAttrMaxTexture2DMipmappedHeight = 74,
    cudaDevAttrComputeCapabilityMajor = 75,
    cudaDevAttrComputeCapabilityMinor = 76,
    cudaDevAttrMaxTexture1DMipmappedWidth = 77,
    cudaDevAttrStreamPrioritiesSupported = 78,
    cudaDevAttrGlobalL1CacheSupported = 79,
    cudaDevAttrLocalL1CacheSupported = 80,
    cudaDevAttrMaxSharedMemoryPerMultiprocessor = 81,
    cudaDevAttrMaxRegistersPerMultiprocessor = 82,
    cudaDevAttrManagedMemory = 83,
    cudaDevAttrIsMultiGpuBoard = 84,
    cudaDevAttrMultiGpuBoardGroupID = 85,
    cudaDevAttrHostNativeAtomicSupported = 86,
    cudaDevAttrSingleToDoublePrecisionPerfRatio = 87,
    cudaDevAttrPageableMemoryAccess = 88,
    cudaDevAttrConcurrentManagedAccess = 89,
    cudaDevAttrComputePreemptionSupported = 90,
    cudaDevAttrCanUseHostPointerForRegisteredMem = 91,
    cudaDevAttrReserved92 = 92,
    cudaDevAttrReserved93 = 93,
    cudaDevAttrReserved94 = 94,
    cudaDevAttrCooperativeLaunch = 95,
    cudaDevAttrCooperativeMultiDeviceLaunch = 96,
    cudaDevAttrMaxSharedMemoryPerBlockOptin = 97,
    cudaDevAttrCanFlushRemoteWrites = 98,
    cudaDevAttrHostRegisterSupported = 99,
    cudaDevAttrPageableMemoryAccessUsesHostPageTables = 100,
    cudaDevAttrDirectManagedMemAccessFromHost = 101,
    cudaDevAttrMaxBlocksPerMultiprocessor = 106,
    cudaDevAttrMaxPersistingL2CacheSize = 108,
    cudaDevAttrMaxAccessPolicyWindowSize = 109,
    cudaDevAttrReservedSharedMemoryPerBlock = 111,
    cudaDevAttrSparseCudaArraySupported = 112,
    cudaDevAttrHostRegisterReadOnlySupported = 113,
    cudaDevAttrTimelineSemaphoreInteropSupported = 114,
    cudaDevAttrMemoryPoolsSupported = 115,
    cudaDevAttrGPUDirectRDMASupported = 116,
    cudaDevAttrGPUDirectRDMAFlushWritesOptions = 117,
    cudaDevAttrGPUDirectRDMAWritesOrdering = 118,
    cudaDevAttrMemoryPoolSupportedHandleTypes = 119,
    cudaDevAttrClusterLaunch = 120,
    cudaDevAttrDeferredMappingCudaArraySupported = 121,
    cudaDevAttrReserved122 = 122,
    cudaDevAttrReserved123 = 123,
    cudaDevAttrReserved124 = 124,
    cudaDevAttrIpcEventSupport = 125,
    cudaDevAttrMemSyncDomainCount = 126,
    cudaDevAttrReserved127 = 127,
    cudaDevAttrReserved128 = 128,
    cudaDevAttrReserved129 = 129,
    cudaDevAttrNumaConfig = 130,
    cudaDevAttrNumaId = 131,
    cudaDevAttrReserved132 = 132,
    cudaDevAttrMpsEnabled = 133,
    cudaDevAttrHostNumaId = 134,
    cudaDevAttrD3D12CigSupported = 135,
    cudaDevAttrVulkanCigSupported = 138,
    cudaDevAttrGpuPciDeviceId = 139,
    cudaDevAttrGpuPciSubsystemId = 140,
    cudaDevAttrReserved141 = 141,
    cudaDevAttrHostNumaMemoryPoolsSupported = 142,
    cudaDevAttrHostNumaMultinodeIpcSupported = 143,
    cudaDevAttrMax = 144,
}
#[cfg(any(feature = "cuda-13000"))]
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub enum cudaDeviceAttr {
    cudaDevAttrMaxThreadsPerBlock = 1,
    cudaDevAttrMaxBlockDimX = 2,
    cudaDevAttrMaxBlockDimY = 3,
    cudaDevAttrMaxBlockDimZ = 4,
    cudaDevAttrMaxGridDimX = 5,
    cudaDevAttrMaxGridDimY = 6,
    cudaDevAttrMaxGridDimZ = 7,
    cudaDevAttrMaxSharedMemoryPerBlock = 8,
    cudaDevAttrTotalConstantMemory = 9,
    cudaDevAttrWarpSize = 10,
    cudaDevAttrMaxPitch = 11,
    cudaDevAttrMaxRegistersPerBlock = 12,
    cudaDevAttrClockRate = 13,
    cudaDevAttrTextureAlignment = 14,
    cudaDevAttrGpuOverlap = 15,
    cudaDevAttrMultiProcessorCount = 16,
    cudaDevAttrKernelExecTimeout = 17,
    cudaDevAttrIntegrated = 18,
    cudaDevAttrCanMapHostMemory = 19,
    cudaDevAttrComputeMode = 20,
    cudaDevAttrMaxTexture1DWidth = 21,
    cudaDevAttrMaxTexture2DWidth = 22,
    cudaDevAttrMaxTexture2DHeight = 23,
    cudaDevAttrMaxTexture3DWidth = 24,
    cudaDevAttrMaxTexture3DHeight = 25,
    cudaDevAttrMaxTexture3DDepth = 26,
    cudaDevAttrMaxTexture2DLayeredWidth = 27,
    cudaDevAttrMaxTexture2DLayeredHeight = 28,
    cudaDevAttrMaxTexture2DLayeredLayers = 29,
    cudaDevAttrSurfaceAlignment = 30,
    cudaDevAttrConcurrentKernels = 31,
    cudaDevAttrEccEnabled = 32,
    cudaDevAttrPciBusId = 33,
    cudaDevAttrPciDeviceId = 34,
    cudaDevAttrTccDriver = 35,
    cudaDevAttrMemoryClockRate = 36,
    cudaDevAttrGlobalMemoryBusWidth = 37,
    cudaDevAttrL2CacheSize = 38,
    cudaDevAttrMaxThreadsPerMultiProcessor = 39,
    cudaDevAttrAsyncEngineCount = 40,
    cudaDevAttrUnifiedAddressing = 41,
    cudaDevAttrMaxTexture1DLayeredWidth = 42,
    cudaDevAttrMaxTexture1DLayeredLayers = 43,
    cudaDevAttrMaxTexture2DGatherWidth = 45,
    cudaDevAttrMaxTexture2DGatherHeight = 46,
    cudaDevAttrMaxTexture3DWidthAlt = 47,
    cudaDevAttrMaxTexture3DHeightAlt = 48,
    cudaDevAttrMaxTexture3DDepthAlt = 49,
    cudaDevAttrPciDomainId = 50,
    cudaDevAttrTexturePitchAlignment = 51,
    cudaDevAttrMaxTextureCubemapWidth = 52,
    cudaDevAttrMaxTextureCubemapLayeredWidth = 53,
    cudaDevAttrMaxTextureCubemapLayeredLayers = 54,
    cudaDevAttrMaxSurface1DWidth = 55,
    cudaDevAttrMaxSurface2DWidth = 56,
    cudaDevAttrMaxSurface2DHeight = 57,
    cudaDevAttrMaxSurface3DWidth = 58,
    cudaDevAttrMaxSurface3DHeight = 59,
    cudaDevAttrMaxSurface3DDepth = 60,
    cudaDevAttrMaxSurface1DLayeredWidth = 61,
    cudaDevAttrMaxSurface1DLayeredLayers = 62,
    cudaDevAttrMaxSurface2DLayeredWidth = 63,
    cudaDevAttrMaxSurface2DLayeredHeight = 64,
    cudaDevAttrMaxSurface2DLayeredLayers = 65,
    cudaDevAttrMaxSurfaceCubemapWidth = 66,
    cudaDevAttrMaxSurfaceCubemapLayeredWidth = 67,
    cudaDevAttrMaxSurfaceCubemapLayeredLayers = 68,
    cudaDevAttrMaxTexture1DLinearWidth = 69,
    cudaDevAttrMaxTexture2DLinearWidth = 70,
    cudaDevAttrMaxTexture2DLinearHeight = 71,
    cudaDevAttrMaxTexture2DLinearPitch = 72,
    cudaDevAttrMaxTexture2DMipmappedWidth = 73,
    cudaDevAttrMaxTexture2DMipmappedHeight = 74,
    cudaDevAttrComputeCapabilityMajor = 75,
    cudaDevAttrComputeCapabilityMinor = 76,
    cudaDevAttrMaxTexture1DMipmappedWidth = 77,
    cudaDevAttrStreamPrioritiesSupported = 78,
    cudaDevAttrGlobalL1CacheSupported = 79,
    cudaDevAttrLocalL1CacheSupported = 80,
    cudaDevAttrMaxSharedMemoryPerMultiprocessor = 81,
    cudaDevAttrMaxRegistersPerMultiprocessor = 82,
    cudaDevAttrManagedMemory = 83,
    cudaDevAttrIsMultiGpuBoard = 84,
    cudaDevAttrMultiGpuBoardGroupID = 85,
    cudaDevAttrHostNativeAtomicSupported = 86,
    cudaDevAttrSingleToDoublePrecisionPerfRatio = 87,
    cudaDevAttrPageableMemoryAccess = 88,
    cudaDevAttrConcurrentManagedAccess = 89,
    cudaDevAttrComputePreemptionSupported = 90,
    cudaDevAttrCanUseHostPointerForRegisteredMem = 91,
    cudaDevAttrReserved92 = 92,
    cudaDevAttrReserved93 = 93,
    cudaDevAttrReserved94 = 94,
    cudaDevAttrCooperativeLaunch = 95,
    cudaDevAttrReserved96 = 96,
    cudaDevAttrMaxSharedMemoryPerBlockOptin = 97,
    cudaDevAttrCanFlushRemoteWrites = 98,
    cudaDevAttrHostRegisterSupported = 99,
    cudaDevAttrPageableMemoryAccessUsesHostPageTables = 100,
    cudaDevAttrDirectManagedMemAccessFromHost = 101,
    cudaDevAttrMaxBlocksPerMultiprocessor = 106,
    cudaDevAttrMaxPersistingL2CacheSize = 108,
    cudaDevAttrMaxAccessPolicyWindowSize = 109,
    cudaDevAttrReservedSharedMemoryPerBlock = 111,
    cudaDevAttrSparseCudaArraySupported = 112,
    cudaDevAttrHostRegisterReadOnlySupported = 113,
    cudaDevAttrTimelineSemaphoreInteropSupported = 114,
    cudaDevAttrMemoryPoolsSupported = 115,
    cudaDevAttrGPUDirectRDMASupported = 116,
    cudaDevAttrGPUDirectRDMAFlushWritesOptions = 117,
    cudaDevAttrGPUDirectRDMAWritesOrdering = 118,
    cudaDevAttrMemoryPoolSupportedHandleTypes = 119,
    cudaDevAttrClusterLaunch = 120,
    cudaDevAttrDeferredMappingCudaArraySupported = 121,
    cudaDevAttrReserved122 = 122,
    cudaDevAttrReserved123 = 123,
    cudaDevAttrReserved124 = 124,
    cudaDevAttrIpcEventSupport = 125,
    cudaDevAttrMemSyncDomainCount = 126,
    cudaDevAttrReserved127 = 127,
    cudaDevAttrReserved128 = 128,
    cudaDevAttrReserved129 = 129,
    cudaDevAttrNumaConfig = 130,
    cudaDevAttrNumaId = 131,
    cudaDevAttrReserved132 = 132,
    cudaDevAttrMpsEnabled = 133,
    cudaDevAttrHostNumaId = 134,
    cudaDevAttrD3D12CigSupported = 135,
    cudaDevAttrVulkanCigSupported = 138,
    cudaDevAttrGpuPciDeviceId = 139,
    cudaDevAttrGpuPciSubsystemId = 140,
    cudaDevAttrReserved141 = 141,
    cudaDevAttrHostNumaMemoryPoolsSupported = 142,
    cudaDevAttrHostNumaMultinodeIpcSupported = 143,
    cudaDevAttrHostMemoryPoolsSupported = 144,
    cudaDevAttrReserved145 = 145,
    cudaDevAttrOnlyPartialHostNativeAtomicSupported = 147,
    cudaDevAttrMax = 148,
}
#[cfg(any(
    feature = "cuda-12020",
    feature = "cuda-12030",
    feature = "cuda-12040",
    feature = "cuda-12050",
    feature = "cuda-12060",
    feature = "cuda-12080",
    feature = "cuda-12090",
    feature = "cuda-13000"
))]
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub enum cudaDeviceNumaConfig {
    cudaDeviceNumaConfigNone = 0,
    cudaDeviceNumaConfigNumaNode = 1,
}
#[cfg(any(
    feature = "cuda-11040",
    feature = "cuda-11050",
    feature = "cuda-11060",
    feature = "cuda-11070",
    feature = "cuda-11080",
    feature = "cuda-12000",
    feature = "cuda-12010",
    feature = "cuda-12020",
    feature = "cuda-12030",
    feature = "cuda-12040",
    feature = "cuda-12050",
    feature = "cuda-12060",
    feature = "cuda-12080",
    feature = "cuda-12090"
))]
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub enum cudaDeviceP2PAttr {
    cudaDevP2PAttrPerformanceRank = 1,
    cudaDevP2PAttrAccessSupported = 2,
    cudaDevP2PAttrNativeAtomicSupported = 3,
    cudaDevP2PAttrCudaArrayAccessSupported = 4,
}
#[cfg(any(feature = "cuda-13000"))]
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub enum cudaDeviceP2PAttr {
    cudaDevP2PAttrPerformanceRank = 1,
    cudaDevP2PAttrAccessSupported = 2,
    cudaDevP2PAttrNativeAtomicSupported = 3,
    cudaDevP2PAttrCudaArrayAccessSupported = 4,
    cudaDevP2PAttrOnlyPartialNativeAtomicSupported = 5,
}
#[cfg(any(
    feature = "cuda-12000",
    feature = "cuda-12010",
    feature = "cuda-12020",
    feature = "cuda-12030",
    feature = "cuda-12040",
    feature = "cuda-12050",
    feature = "cuda-12060",
    feature = "cuda-12080",
    feature = "cuda-12090",
    feature = "cuda-13000"
))]
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub enum cudaDriverEntryPointQueryResult {
    cudaDriverEntryPointSuccess = 0,
    cudaDriverEntryPointSymbolNotFound = 1,
    cudaDriverEntryPointVersionNotSufficent = 2,
}
#[cfg(any(feature = "cuda-13000"))]
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub enum cudaEmulationMantissaControl_t {
    CUDA_EMULATION_MANTISSA_CONTROL_DYNAMIC = 0,
    CUDA_EMULATION_MANTISSA_CONTROL_FIXED = 1,
}
#[cfg(any(feature = "cuda-13000"))]
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub enum cudaEmulationSpecialValuesSupport_t {
    CUDA_EMULATION_SPECIAL_VALUES_SUPPORT_DEFAULT = 65535,
    CUDA_EMULATION_SPECIAL_VALUES_SUPPORT_NONE = 0,
    CUDA_EMULATION_SPECIAL_VALUES_SUPPORT_INFINITY = 1,
    CUDA_EMULATION_SPECIAL_VALUES_SUPPORT_NAN = 2,
}
#[cfg(any(feature = "cuda-13000"))]
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub enum cudaEmulationStrategy_t {
    CUDA_EMULATION_STRATEGY_DEFAULT = 0,
    CUDA_EMULATION_STRATEGY_PERFORMANT = 1,
    CUDA_EMULATION_STRATEGY_EAGER = 2,
}
#[cfg(any(
    feature = "cuda-11040",
    feature = "cuda-11050",
    feature = "cuda-11060",
    feature = "cuda-11070"
))]
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub enum cudaError {
    cudaSuccess = 0,
    cudaErrorInvalidValue = 1,
    cudaErrorMemoryAllocation = 2,
    cudaErrorInitializationError = 3,
    cudaErrorCudartUnloading = 4,
    cudaErrorProfilerDisabled = 5,
    cudaErrorProfilerNotInitialized = 6,
    cudaErrorProfilerAlreadyStarted = 7,
    cudaErrorProfilerAlreadyStopped = 8,
    cudaErrorInvalidConfiguration = 9,
    cudaErrorInvalidPitchValue = 12,
    cudaErrorInvalidSymbol = 13,
    cudaErrorInvalidHostPointer = 16,
    cudaErrorInvalidDevicePointer = 17,
    cudaErrorInvalidTexture = 18,
    cudaErrorInvalidTextureBinding = 19,
    cudaErrorInvalidChannelDescriptor = 20,
    cudaErrorInvalidMemcpyDirection = 21,
    cudaErrorAddressOfConstant = 22,
    cudaErrorTextureFetchFailed = 23,
    cudaErrorTextureNotBound = 24,
    cudaErrorSynchronizationError = 25,
    cudaErrorInvalidFilterSetting = 26,
    cudaErrorInvalidNormSetting = 27,
    cudaErrorMixedDeviceExecution = 28,
    cudaErrorNotYetImplemented = 31,
    cudaErrorMemoryValueTooLarge = 32,
    cudaErrorStubLibrary = 34,
    cudaErrorInsufficientDriver = 35,
    cudaErrorCallRequiresNewerDriver = 36,
    cudaErrorInvalidSurface = 37,
    cudaErrorDuplicateVariableName = 43,
    cudaErrorDuplicateTextureName = 44,
    cudaErrorDuplicateSurfaceName = 45,
    cudaErrorDevicesUnavailable = 46,
    cudaErrorIncompatibleDriverContext = 49,
    cudaErrorMissingConfiguration = 52,
    cudaErrorPriorLaunchFailure = 53,
    cudaErrorLaunchMaxDepthExceeded = 65,
    cudaErrorLaunchFileScopedTex = 66,
    cudaErrorLaunchFileScopedSurf = 67,
    cudaErrorSyncDepthExceeded = 68,
    cudaErrorLaunchPendingCountExceeded = 69,
    cudaErrorInvalidDeviceFunction = 98,
    cudaErrorNoDevice = 100,
    cudaErrorInvalidDevice = 101,
    cudaErrorDeviceNotLicensed = 102,
    cudaErrorSoftwareValidityNotEstablished = 103,
    cudaErrorStartupFailure = 127,
    cudaErrorInvalidKernelImage = 200,
    cudaErrorDeviceUninitialized = 201,
    cudaErrorMapBufferObjectFailed = 205,
    cudaErrorUnmapBufferObjectFailed = 206,
    cudaErrorArrayIsMapped = 207,
    cudaErrorAlreadyMapped = 208,
    cudaErrorNoKernelImageForDevice = 209,
    cudaErrorAlreadyAcquired = 210,
    cudaErrorNotMapped = 211,
    cudaErrorNotMappedAsArray = 212,
    cudaErrorNotMappedAsPointer = 213,
    cudaErrorECCUncorrectable = 214,
    cudaErrorUnsupportedLimit = 215,
    cudaErrorDeviceAlreadyInUse = 216,
    cudaErrorPeerAccessUnsupported = 217,
    cudaErrorInvalidPtx = 218,
    cudaErrorInvalidGraphicsContext = 219,
    cudaErrorNvlinkUncorrectable = 220,
    cudaErrorJitCompilerNotFound = 221,
    cudaErrorUnsupportedPtxVersion = 222,
    cudaErrorJitCompilationDisabled = 223,
    cudaErrorUnsupportedExecAffinity = 224,
    cudaErrorInvalidSource = 300,
    cudaErrorFileNotFound = 301,
    cudaErrorSharedObjectSymbolNotFound = 302,
    cudaErrorSharedObjectInitFailed = 303,
    cudaErrorOperatingSystem = 304,
    cudaErrorInvalidResourceHandle = 400,
    cudaErrorIllegalState = 401,
    cudaErrorSymbolNotFound = 500,
    cudaErrorNotReady = 600,
    cudaErrorIllegalAddress = 700,
    cudaErrorLaunchOutOfResources = 701,
    cudaErrorLaunchTimeout = 702,
    cudaErrorLaunchIncompatibleTexturing = 703,
    cudaErrorPeerAccessAlreadyEnabled = 704,
    cudaErrorPeerAccessNotEnabled = 705,
    cudaErrorSetOnActiveProcess = 708,
    cudaErrorContextIsDestroyed = 709,
    cudaErrorAssert = 710,
    cudaErrorTooManyPeers = 711,
    cudaErrorHostMemoryAlreadyRegistered = 712,
    cudaErrorHostMemoryNotRegistered = 713,
    cudaErrorHardwareStackError = 714,
    cudaErrorIllegalInstruction = 715,
    cudaErrorMisalignedAddress = 716,
    cudaErrorInvalidAddressSpace = 717,
    cudaErrorInvalidPc = 718,
    cudaErrorLaunchFailure = 719,
    cudaErrorCooperativeLaunchTooLarge = 720,
    cudaErrorNotPermitted = 800,
    cudaErrorNotSupported = 801,
    cudaErrorSystemNotReady = 802,
    cudaErrorSystemDriverMismatch = 803,
    cudaErrorCompatNotSupportedOnDevice = 804,
    cudaErrorMpsConnectionFailed = 805,
    cudaErrorMpsRpcFailure = 806,
    cudaErrorMpsServerNotReady = 807,
    cudaErrorMpsMaxClientsReached = 808,
    cudaErrorMpsMaxConnectionsReached = 809,
    cudaErrorStreamCaptureUnsupported = 900,
    cudaErrorStreamCaptureInvalidated = 901,
    cudaErrorStreamCaptureMerge = 902,
    cudaErrorStreamCaptureUnmatched = 903,
    cudaErrorStreamCaptureUnjoined = 904,
    cudaErrorStreamCaptureIsolation = 905,
    cudaErrorStreamCaptureImplicit = 906,
    cudaErrorCapturedEvent = 907,
    cudaErrorStreamCaptureWrongThread = 908,
    cudaErrorTimeout = 909,
    cudaErrorGraphExecUpdateFailure = 910,
    cudaErrorExternalDevice = 911,
    cudaErrorUnknown = 999,
    cudaErrorApiFailureBase = 10000,
}
#[cfg(any(feature = "cuda-11080"))]
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub enum cudaError {
    cudaSuccess = 0,
    cudaErrorInvalidValue = 1,
    cudaErrorMemoryAllocation = 2,
    cudaErrorInitializationError = 3,
    cudaErrorCudartUnloading = 4,
    cudaErrorProfilerDisabled = 5,
    cudaErrorProfilerNotInitialized = 6,
    cudaErrorProfilerAlreadyStarted = 7,
    cudaErrorProfilerAlreadyStopped = 8,
    cudaErrorInvalidConfiguration = 9,
    cudaErrorInvalidPitchValue = 12,
    cudaErrorInvalidSymbol = 13,
    cudaErrorInvalidHostPointer = 16,
    cudaErrorInvalidDevicePointer = 17,
    cudaErrorInvalidTexture = 18,
    cudaErrorInvalidTextureBinding = 19,
    cudaErrorInvalidChannelDescriptor = 20,
    cudaErrorInvalidMemcpyDirection = 21,
    cudaErrorAddressOfConstant = 22,
    cudaErrorTextureFetchFailed = 23,
    cudaErrorTextureNotBound = 24,
    cudaErrorSynchronizationError = 25,
    cudaErrorInvalidFilterSetting = 26,
    cudaErrorInvalidNormSetting = 27,
    cudaErrorMixedDeviceExecution = 28,
    cudaErrorNotYetImplemented = 31,
    cudaErrorMemoryValueTooLarge = 32,
    cudaErrorStubLibrary = 34,
    cudaErrorInsufficientDriver = 35,
    cudaErrorCallRequiresNewerDriver = 36,
    cudaErrorInvalidSurface = 37,
    cudaErrorDuplicateVariableName = 43,
    cudaErrorDuplicateTextureName = 44,
    cudaErrorDuplicateSurfaceName = 45,
    cudaErrorDevicesUnavailable = 46,
    cudaErrorIncompatibleDriverContext = 49,
    cudaErrorMissingConfiguration = 52,
    cudaErrorPriorLaunchFailure = 53,
    cudaErrorLaunchMaxDepthExceeded = 65,
    cudaErrorLaunchFileScopedTex = 66,
    cudaErrorLaunchFileScopedSurf = 67,
    cudaErrorSyncDepthExceeded = 68,
    cudaErrorLaunchPendingCountExceeded = 69,
    cudaErrorInvalidDeviceFunction = 98,
    cudaErrorNoDevice = 100,
    cudaErrorInvalidDevice = 101,
    cudaErrorDeviceNotLicensed = 102,
    cudaErrorSoftwareValidityNotEstablished = 103,
    cudaErrorStartupFailure = 127,
    cudaErrorInvalidKernelImage = 200,
    cudaErrorDeviceUninitialized = 201,
    cudaErrorMapBufferObjectFailed = 205,
    cudaErrorUnmapBufferObjectFailed = 206,
    cudaErrorArrayIsMapped = 207,
    cudaErrorAlreadyMapped = 208,
    cudaErrorNoKernelImageForDevice = 209,
    cudaErrorAlreadyAcquired = 210,
    cudaErrorNotMapped = 211,
    cudaErrorNotMappedAsArray = 212,
    cudaErrorNotMappedAsPointer = 213,
    cudaErrorECCUncorrectable = 214,
    cudaErrorUnsupportedLimit = 215,
    cudaErrorDeviceAlreadyInUse = 216,
    cudaErrorPeerAccessUnsupported = 217,
    cudaErrorInvalidPtx = 218,
    cudaErrorInvalidGraphicsContext = 219,
    cudaErrorNvlinkUncorrectable = 220,
    cudaErrorJitCompilerNotFound = 221,
    cudaErrorUnsupportedPtxVersion = 222,
    cudaErrorJitCompilationDisabled = 223,
    cudaErrorUnsupportedExecAffinity = 224,
    cudaErrorInvalidSource = 300,
    cudaErrorFileNotFound = 301,
    cudaErrorSharedObjectSymbolNotFound = 302,
    cudaErrorSharedObjectInitFailed = 303,
    cudaErrorOperatingSystem = 304,
    cudaErrorInvalidResourceHandle = 400,
    cudaErrorIllegalState = 401,
    cudaErrorSymbolNotFound = 500,
    cudaErrorNotReady = 600,
    cudaErrorIllegalAddress = 700,
    cudaErrorLaunchOutOfResources = 701,
    cudaErrorLaunchTimeout = 702,
    cudaErrorLaunchIncompatibleTexturing = 703,
    cudaErrorPeerAccessAlreadyEnabled = 704,
    cudaErrorPeerAccessNotEnabled = 705,
    cudaErrorSetOnActiveProcess = 708,
    cudaErrorContextIsDestroyed = 709,
    cudaErrorAssert = 710,
    cudaErrorTooManyPeers = 711,
    cudaErrorHostMemoryAlreadyRegistered = 712,
    cudaErrorHostMemoryNotRegistered = 713,
    cudaErrorHardwareStackError = 714,
    cudaErrorIllegalInstruction = 715,
    cudaErrorMisalignedAddress = 716,
    cudaErrorInvalidAddressSpace = 717,
    cudaErrorInvalidPc = 718,
    cudaErrorLaunchFailure = 719,
    cudaErrorCooperativeLaunchTooLarge = 720,
    cudaErrorNotPermitted = 800,
    cudaErrorNotSupported = 801,
    cudaErrorSystemNotReady = 802,
    cudaErrorSystemDriverMismatch = 803,
    cudaErrorCompatNotSupportedOnDevice = 804,
    cudaErrorMpsConnectionFailed = 805,
    cudaErrorMpsRpcFailure = 806,
    cudaErrorMpsServerNotReady = 807,
    cudaErrorMpsMaxClientsReached = 808,
    cudaErrorMpsMaxConnectionsReached = 809,
    cudaErrorMpsClientTerminated = 810,
    cudaErrorStreamCaptureUnsupported = 900,
    cudaErrorStreamCaptureInvalidated = 901,
    cudaErrorStreamCaptureMerge = 902,
    cudaErrorStreamCaptureUnmatched = 903,
    cudaErrorStreamCaptureUnjoined = 904,
    cudaErrorStreamCaptureIsolation = 905,
    cudaErrorStreamCaptureImplicit = 906,
    cudaErrorCapturedEvent = 907,
    cudaErrorStreamCaptureWrongThread = 908,
    cudaErrorTimeout = 909,
    cudaErrorGraphExecUpdateFailure = 910,
    cudaErrorExternalDevice = 911,
    cudaErrorInvalidClusterSize = 912,
    cudaErrorUnknown = 999,
    cudaErrorApiFailureBase = 10000,
}
#[cfg(any(feature = "cuda-12000"))]
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub enum cudaError {
    cudaSuccess = 0,
    cudaErrorInvalidValue = 1,
    cudaErrorMemoryAllocation = 2,
    cudaErrorInitializationError = 3,
    cudaErrorCudartUnloading = 4,
    cudaErrorProfilerDisabled = 5,
    cudaErrorProfilerNotInitialized = 6,
    cudaErrorProfilerAlreadyStarted = 7,
    cudaErrorProfilerAlreadyStopped = 8,
    cudaErrorInvalidConfiguration = 9,
    cudaErrorInvalidPitchValue = 12,
    cudaErrorInvalidSymbol = 13,
    cudaErrorInvalidHostPointer = 16,
    cudaErrorInvalidDevicePointer = 17,
    cudaErrorInvalidTexture = 18,
    cudaErrorInvalidTextureBinding = 19,
    cudaErrorInvalidChannelDescriptor = 20,
    cudaErrorInvalidMemcpyDirection = 21,
    cudaErrorAddressOfConstant = 22,
    cudaErrorTextureFetchFailed = 23,
    cudaErrorTextureNotBound = 24,
    cudaErrorSynchronizationError = 25,
    cudaErrorInvalidFilterSetting = 26,
    cudaErrorInvalidNormSetting = 27,
    cudaErrorMixedDeviceExecution = 28,
    cudaErrorNotYetImplemented = 31,
    cudaErrorMemoryValueTooLarge = 32,
    cudaErrorStubLibrary = 34,
    cudaErrorInsufficientDriver = 35,
    cudaErrorCallRequiresNewerDriver = 36,
    cudaErrorInvalidSurface = 37,
    cudaErrorDuplicateVariableName = 43,
    cudaErrorDuplicateTextureName = 44,
    cudaErrorDuplicateSurfaceName = 45,
    cudaErrorDevicesUnavailable = 46,
    cudaErrorIncompatibleDriverContext = 49,
    cudaErrorMissingConfiguration = 52,
    cudaErrorPriorLaunchFailure = 53,
    cudaErrorLaunchMaxDepthExceeded = 65,
    cudaErrorLaunchFileScopedTex = 66,
    cudaErrorLaunchFileScopedSurf = 67,
    cudaErrorSyncDepthExceeded = 68,
    cudaErrorLaunchPendingCountExceeded = 69,
    cudaErrorInvalidDeviceFunction = 98,
    cudaErrorNoDevice = 100,
    cudaErrorInvalidDevice = 101,
    cudaErrorDeviceNotLicensed = 102,
    cudaErrorSoftwareValidityNotEstablished = 103,
    cudaErrorStartupFailure = 127,
    cudaErrorInvalidKernelImage = 200,
    cudaErrorDeviceUninitialized = 201,
    cudaErrorMapBufferObjectFailed = 205,
    cudaErrorUnmapBufferObjectFailed = 206,
    cudaErrorArrayIsMapped = 207,
    cudaErrorAlreadyMapped = 208,
    cudaErrorNoKernelImageForDevice = 209,
    cudaErrorAlreadyAcquired = 210,
    cudaErrorNotMapped = 211,
    cudaErrorNotMappedAsArray = 212,
    cudaErrorNotMappedAsPointer = 213,
    cudaErrorECCUncorrectable = 214,
    cudaErrorUnsupportedLimit = 215,
    cudaErrorDeviceAlreadyInUse = 216,
    cudaErrorPeerAccessUnsupported = 217,
    cudaErrorInvalidPtx = 218,
    cudaErrorInvalidGraphicsContext = 219,
    cudaErrorNvlinkUncorrectable = 220,
    cudaErrorJitCompilerNotFound = 221,
    cudaErrorUnsupportedPtxVersion = 222,
    cudaErrorJitCompilationDisabled = 223,
    cudaErrorUnsupportedExecAffinity = 224,
    cudaErrorInvalidSource = 300,
    cudaErrorFileNotFound = 301,
    cudaErrorSharedObjectSymbolNotFound = 302,
    cudaErrorSharedObjectInitFailed = 303,
    cudaErrorOperatingSystem = 304,
    cudaErrorInvalidResourceHandle = 400,
    cudaErrorIllegalState = 401,
    cudaErrorSymbolNotFound = 500,
    cudaErrorNotReady = 600,
    cudaErrorIllegalAddress = 700,
    cudaErrorLaunchOutOfResources = 701,
    cudaErrorLaunchTimeout = 702,
    cudaErrorLaunchIncompatibleTexturing = 703,
    cudaErrorPeerAccessAlreadyEnabled = 704,
    cudaErrorPeerAccessNotEnabled = 705,
    cudaErrorSetOnActiveProcess = 708,
    cudaErrorContextIsDestroyed = 709,
    cudaErrorAssert = 710,
    cudaErrorTooManyPeers = 711,
    cudaErrorHostMemoryAlreadyRegistered = 712,
    cudaErrorHostMemoryNotRegistered = 713,
    cudaErrorHardwareStackError = 714,
    cudaErrorIllegalInstruction = 715,
    cudaErrorMisalignedAddress = 716,
    cudaErrorInvalidAddressSpace = 717,
    cudaErrorInvalidPc = 718,
    cudaErrorLaunchFailure = 719,
    cudaErrorCooperativeLaunchTooLarge = 720,
    cudaErrorNotPermitted = 800,
    cudaErrorNotSupported = 801,
    cudaErrorSystemNotReady = 802,
    cudaErrorSystemDriverMismatch = 803,
    cudaErrorCompatNotSupportedOnDevice = 804,
    cudaErrorMpsConnectionFailed = 805,
    cudaErrorMpsRpcFailure = 806,
    cudaErrorMpsServerNotReady = 807,
    cudaErrorMpsMaxClientsReached = 808,
    cudaErrorMpsMaxConnectionsReached = 809,
    cudaErrorMpsClientTerminated = 810,
    cudaErrorCdpNotSupported = 811,
    cudaErrorCdpVersionMismatch = 812,
    cudaErrorStreamCaptureUnsupported = 900,
    cudaErrorStreamCaptureInvalidated = 901,
    cudaErrorStreamCaptureMerge = 902,
    cudaErrorStreamCaptureUnmatched = 903,
    cudaErrorStreamCaptureUnjoined = 904,
    cudaErrorStreamCaptureIsolation = 905,
    cudaErrorStreamCaptureImplicit = 906,
    cudaErrorCapturedEvent = 907,
    cudaErrorStreamCaptureWrongThread = 908,
    cudaErrorTimeout = 909,
    cudaErrorGraphExecUpdateFailure = 910,
    cudaErrorExternalDevice = 911,
    cudaErrorInvalidClusterSize = 912,
    cudaErrorUnknown = 999,
    cudaErrorApiFailureBase = 10000,
}
#[cfg(any(feature = "cuda-12010", feature = "cuda-12020"))]
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub enum cudaError {
    cudaSuccess = 0,
    cudaErrorInvalidValue = 1,
    cudaErrorMemoryAllocation = 2,
    cudaErrorInitializationError = 3,
    cudaErrorCudartUnloading = 4,
    cudaErrorProfilerDisabled = 5,
    cudaErrorProfilerNotInitialized = 6,
    cudaErrorProfilerAlreadyStarted = 7,
    cudaErrorProfilerAlreadyStopped = 8,
    cudaErrorInvalidConfiguration = 9,
    cudaErrorInvalidPitchValue = 12,
    cudaErrorInvalidSymbol = 13,
    cudaErrorInvalidHostPointer = 16,
    cudaErrorInvalidDevicePointer = 17,
    cudaErrorInvalidTexture = 18,
    cudaErrorInvalidTextureBinding = 19,
    cudaErrorInvalidChannelDescriptor = 20,
    cudaErrorInvalidMemcpyDirection = 21,
    cudaErrorAddressOfConstant = 22,
    cudaErrorTextureFetchFailed = 23,
    cudaErrorTextureNotBound = 24,
    cudaErrorSynchronizationError = 25,
    cudaErrorInvalidFilterSetting = 26,
    cudaErrorInvalidNormSetting = 27,
    cudaErrorMixedDeviceExecution = 28,
    cudaErrorNotYetImplemented = 31,
    cudaErrorMemoryValueTooLarge = 32,
    cudaErrorStubLibrary = 34,
    cudaErrorInsufficientDriver = 35,
    cudaErrorCallRequiresNewerDriver = 36,
    cudaErrorInvalidSurface = 37,
    cudaErrorDuplicateVariableName = 43,
    cudaErrorDuplicateTextureName = 44,
    cudaErrorDuplicateSurfaceName = 45,
    cudaErrorDevicesUnavailable = 46,
    cudaErrorIncompatibleDriverContext = 49,
    cudaErrorMissingConfiguration = 52,
    cudaErrorPriorLaunchFailure = 53,
    cudaErrorLaunchMaxDepthExceeded = 65,
    cudaErrorLaunchFileScopedTex = 66,
    cudaErrorLaunchFileScopedSurf = 67,
    cudaErrorSyncDepthExceeded = 68,
    cudaErrorLaunchPendingCountExceeded = 69,
    cudaErrorInvalidDeviceFunction = 98,
    cudaErrorNoDevice = 100,
    cudaErrorInvalidDevice = 101,
    cudaErrorDeviceNotLicensed = 102,
    cudaErrorSoftwareValidityNotEstablished = 103,
    cudaErrorStartupFailure = 127,
    cudaErrorInvalidKernelImage = 200,
    cudaErrorDeviceUninitialized = 201,
    cudaErrorMapBufferObjectFailed = 205,
    cudaErrorUnmapBufferObjectFailed = 206,
    cudaErrorArrayIsMapped = 207,
    cudaErrorAlreadyMapped = 208,
    cudaErrorNoKernelImageForDevice = 209,
    cudaErrorAlreadyAcquired = 210,
    cudaErrorNotMapped = 211,
    cudaErrorNotMappedAsArray = 212,
    cudaErrorNotMappedAsPointer = 213,
    cudaErrorECCUncorrectable = 214,
    cudaErrorUnsupportedLimit = 215,
    cudaErrorDeviceAlreadyInUse = 216,
    cudaErrorPeerAccessUnsupported = 217,
    cudaErrorInvalidPtx = 218,
    cudaErrorInvalidGraphicsContext = 219,
    cudaErrorNvlinkUncorrectable = 220,
    cudaErrorJitCompilerNotFound = 221,
    cudaErrorUnsupportedPtxVersion = 222,
    cudaErrorJitCompilationDisabled = 223,
    cudaErrorUnsupportedExecAffinity = 224,
    cudaErrorUnsupportedDevSideSync = 225,
    cudaErrorInvalidSource = 300,
    cudaErrorFileNotFound = 301,
    cudaErrorSharedObjectSymbolNotFound = 302,
    cudaErrorSharedObjectInitFailed = 303,
    cudaErrorOperatingSystem = 304,
    cudaErrorInvalidResourceHandle = 400,
    cudaErrorIllegalState = 401,
    cudaErrorSymbolNotFound = 500,
    cudaErrorNotReady = 600,
    cudaErrorIllegalAddress = 700,
    cudaErrorLaunchOutOfResources = 701,
    cudaErrorLaunchTimeout = 702,
    cudaErrorLaunchIncompatibleTexturing = 703,
    cudaErrorPeerAccessAlreadyEnabled = 704,
    cudaErrorPeerAccessNotEnabled = 705,
    cudaErrorSetOnActiveProcess = 708,
    cudaErrorContextIsDestroyed = 709,
    cudaErrorAssert = 710,
    cudaErrorTooManyPeers = 711,
    cudaErrorHostMemoryAlreadyRegistered = 712,
    cudaErrorHostMemoryNotRegistered = 713,
    cudaErrorHardwareStackError = 714,
    cudaErrorIllegalInstruction = 715,
    cudaErrorMisalignedAddress = 716,
    cudaErrorInvalidAddressSpace = 717,
    cudaErrorInvalidPc = 718,
    cudaErrorLaunchFailure = 719,
    cudaErrorCooperativeLaunchTooLarge = 720,
    cudaErrorNotPermitted = 800,
    cudaErrorNotSupported = 801,
    cudaErrorSystemNotReady = 802,
    cudaErrorSystemDriverMismatch = 803,
    cudaErrorCompatNotSupportedOnDevice = 804,
    cudaErrorMpsConnectionFailed = 805,
    cudaErrorMpsRpcFailure = 806,
    cudaErrorMpsServerNotReady = 807,
    cudaErrorMpsMaxClientsReached = 808,
    cudaErrorMpsMaxConnectionsReached = 809,
    cudaErrorMpsClientTerminated = 810,
    cudaErrorCdpNotSupported = 811,
    cudaErrorCdpVersionMismatch = 812,
    cudaErrorStreamCaptureUnsupported = 900,
    cudaErrorStreamCaptureInvalidated = 901,
    cudaErrorStreamCaptureMerge = 902,
    cudaErrorStreamCaptureUnmatched = 903,
    cudaErrorStreamCaptureUnjoined = 904,
    cudaErrorStreamCaptureIsolation = 905,
    cudaErrorStreamCaptureImplicit = 906,
    cudaErrorCapturedEvent = 907,
    cudaErrorStreamCaptureWrongThread = 908,
    cudaErrorTimeout = 909,
    cudaErrorGraphExecUpdateFailure = 910,
    cudaErrorExternalDevice = 911,
    cudaErrorInvalidClusterSize = 912,
    cudaErrorUnknown = 999,
    cudaErrorApiFailureBase = 10000,
}
#[cfg(any(feature = "cuda-12030", feature = "cuda-12040", feature = "cuda-12050"))]
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub enum cudaError {
    cudaSuccess = 0,
    cudaErrorInvalidValue = 1,
    cudaErrorMemoryAllocation = 2,
    cudaErrorInitializationError = 3,
    cudaErrorCudartUnloading = 4,
    cudaErrorProfilerDisabled = 5,
    cudaErrorProfilerNotInitialized = 6,
    cudaErrorProfilerAlreadyStarted = 7,
    cudaErrorProfilerAlreadyStopped = 8,
    cudaErrorInvalidConfiguration = 9,
    cudaErrorInvalidPitchValue = 12,
    cudaErrorInvalidSymbol = 13,
    cudaErrorInvalidHostPointer = 16,
    cudaErrorInvalidDevicePointer = 17,
    cudaErrorInvalidTexture = 18,
    cudaErrorInvalidTextureBinding = 19,
    cudaErrorInvalidChannelDescriptor = 20,
    cudaErrorInvalidMemcpyDirection = 21,
    cudaErrorAddressOfConstant = 22,
    cudaErrorTextureFetchFailed = 23,
    cudaErrorTextureNotBound = 24,
    cudaErrorSynchronizationError = 25,
    cudaErrorInvalidFilterSetting = 26,
    cudaErrorInvalidNormSetting = 27,
    cudaErrorMixedDeviceExecution = 28,
    cudaErrorNotYetImplemented = 31,
    cudaErrorMemoryValueTooLarge = 32,
    cudaErrorStubLibrary = 34,
    cudaErrorInsufficientDriver = 35,
    cudaErrorCallRequiresNewerDriver = 36,
    cudaErrorInvalidSurface = 37,
    cudaErrorDuplicateVariableName = 43,
    cudaErrorDuplicateTextureName = 44,
    cudaErrorDuplicateSurfaceName = 45,
    cudaErrorDevicesUnavailable = 46,
    cudaErrorIncompatibleDriverContext = 49,
    cudaErrorMissingConfiguration = 52,
    cudaErrorPriorLaunchFailure = 53,
    cudaErrorLaunchMaxDepthExceeded = 65,
    cudaErrorLaunchFileScopedTex = 66,
    cudaErrorLaunchFileScopedSurf = 67,
    cudaErrorSyncDepthExceeded = 68,
    cudaErrorLaunchPendingCountExceeded = 69,
    cudaErrorInvalidDeviceFunction = 98,
    cudaErrorNoDevice = 100,
    cudaErrorInvalidDevice = 101,
    cudaErrorDeviceNotLicensed = 102,
    cudaErrorSoftwareValidityNotEstablished = 103,
    cudaErrorStartupFailure = 127,
    cudaErrorInvalidKernelImage = 200,
    cudaErrorDeviceUninitialized = 201,
    cudaErrorMapBufferObjectFailed = 205,
    cudaErrorUnmapBufferObjectFailed = 206,
    cudaErrorArrayIsMapped = 207,
    cudaErrorAlreadyMapped = 208,
    cudaErrorNoKernelImageForDevice = 209,
    cudaErrorAlreadyAcquired = 210,
    cudaErrorNotMapped = 211,
    cudaErrorNotMappedAsArray = 212,
    cudaErrorNotMappedAsPointer = 213,
    cudaErrorECCUncorrectable = 214,
    cudaErrorUnsupportedLimit = 215,
    cudaErrorDeviceAlreadyInUse = 216,
    cudaErrorPeerAccessUnsupported = 217,
    cudaErrorInvalidPtx = 218,
    cudaErrorInvalidGraphicsContext = 219,
    cudaErrorNvlinkUncorrectable = 220,
    cudaErrorJitCompilerNotFound = 221,
    cudaErrorUnsupportedPtxVersion = 222,
    cudaErrorJitCompilationDisabled = 223,
    cudaErrorUnsupportedExecAffinity = 224,
    cudaErrorUnsupportedDevSideSync = 225,
    cudaErrorInvalidSource = 300,
    cudaErrorFileNotFound = 301,
    cudaErrorSharedObjectSymbolNotFound = 302,
    cudaErrorSharedObjectInitFailed = 303,
    cudaErrorOperatingSystem = 304,
    cudaErrorInvalidResourceHandle = 400,
    cudaErrorIllegalState = 401,
    cudaErrorLossyQuery = 402,
    cudaErrorSymbolNotFound = 500,
    cudaErrorNotReady = 600,
    cudaErrorIllegalAddress = 700,
    cudaErrorLaunchOutOfResources = 701,
    cudaErrorLaunchTimeout = 702,
    cudaErrorLaunchIncompatibleTexturing = 703,
    cudaErrorPeerAccessAlreadyEnabled = 704,
    cudaErrorPeerAccessNotEnabled = 705,
    cudaErrorSetOnActiveProcess = 708,
    cudaErrorContextIsDestroyed = 709,
    cudaErrorAssert = 710,
    cudaErrorTooManyPeers = 711,
    cudaErrorHostMemoryAlreadyRegistered = 712,
    cudaErrorHostMemoryNotRegistered = 713,
    cudaErrorHardwareStackError = 714,
    cudaErrorIllegalInstruction = 715,
    cudaErrorMisalignedAddress = 716,
    cudaErrorInvalidAddressSpace = 717,
    cudaErrorInvalidPc = 718,
    cudaErrorLaunchFailure = 719,
    cudaErrorCooperativeLaunchTooLarge = 720,
    cudaErrorNotPermitted = 800,
    cudaErrorNotSupported = 801,
    cudaErrorSystemNotReady = 802,
    cudaErrorSystemDriverMismatch = 803,
    cudaErrorCompatNotSupportedOnDevice = 804,
    cudaErrorMpsConnectionFailed = 805,
    cudaErrorMpsRpcFailure = 806,
    cudaErrorMpsServerNotReady = 807,
    cudaErrorMpsMaxClientsReached = 808,
    cudaErrorMpsMaxConnectionsReached = 809,
    cudaErrorMpsClientTerminated = 810,
    cudaErrorCdpNotSupported = 811,
    cudaErrorCdpVersionMismatch = 812,
    cudaErrorStreamCaptureUnsupported = 900,
    cudaErrorStreamCaptureInvalidated = 901,
    cudaErrorStreamCaptureMerge = 902,
    cudaErrorStreamCaptureUnmatched = 903,
    cudaErrorStreamCaptureUnjoined = 904,
    cudaErrorStreamCaptureIsolation = 905,
    cudaErrorStreamCaptureImplicit = 906,
    cudaErrorCapturedEvent = 907,
    cudaErrorStreamCaptureWrongThread = 908,
    cudaErrorTimeout = 909,
    cudaErrorGraphExecUpdateFailure = 910,
    cudaErrorExternalDevice = 911,
    cudaErrorInvalidClusterSize = 912,
    cudaErrorUnknown = 999,
    cudaErrorApiFailureBase = 10000,
}
#[cfg(any(feature = "cuda-12060"))]
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub enum cudaError {
    cudaSuccess = 0,
    cudaErrorInvalidValue = 1,
    cudaErrorMemoryAllocation = 2,
    cudaErrorInitializationError = 3,
    cudaErrorCudartUnloading = 4,
    cudaErrorProfilerDisabled = 5,
    cudaErrorProfilerNotInitialized = 6,
    cudaErrorProfilerAlreadyStarted = 7,
    cudaErrorProfilerAlreadyStopped = 8,
    cudaErrorInvalidConfiguration = 9,
    cudaErrorInvalidPitchValue = 12,
    cudaErrorInvalidSymbol = 13,
    cudaErrorInvalidHostPointer = 16,
    cudaErrorInvalidDevicePointer = 17,
    cudaErrorInvalidTexture = 18,
    cudaErrorInvalidTextureBinding = 19,
    cudaErrorInvalidChannelDescriptor = 20,
    cudaErrorInvalidMemcpyDirection = 21,
    cudaErrorAddressOfConstant = 22,
    cudaErrorTextureFetchFailed = 23,
    cudaErrorTextureNotBound = 24,
    cudaErrorSynchronizationError = 25,
    cudaErrorInvalidFilterSetting = 26,
    cudaErrorInvalidNormSetting = 27,
    cudaErrorMixedDeviceExecution = 28,
    cudaErrorNotYetImplemented = 31,
    cudaErrorMemoryValueTooLarge = 32,
    cudaErrorStubLibrary = 34,
    cudaErrorInsufficientDriver = 35,
    cudaErrorCallRequiresNewerDriver = 36,
    cudaErrorInvalidSurface = 37,
    cudaErrorDuplicateVariableName = 43,
    cudaErrorDuplicateTextureName = 44,
    cudaErrorDuplicateSurfaceName = 45,
    cudaErrorDevicesUnavailable = 46,
    cudaErrorIncompatibleDriverContext = 49,
    cudaErrorMissingConfiguration = 52,
    cudaErrorPriorLaunchFailure = 53,
    cudaErrorLaunchMaxDepthExceeded = 65,
    cudaErrorLaunchFileScopedTex = 66,
    cudaErrorLaunchFileScopedSurf = 67,
    cudaErrorSyncDepthExceeded = 68,
    cudaErrorLaunchPendingCountExceeded = 69,
    cudaErrorInvalidDeviceFunction = 98,
    cudaErrorNoDevice = 100,
    cudaErrorInvalidDevice = 101,
    cudaErrorDeviceNotLicensed = 102,
    cudaErrorSoftwareValidityNotEstablished = 103,
    cudaErrorStartupFailure = 127,
    cudaErrorInvalidKernelImage = 200,
    cudaErrorDeviceUninitialized = 201,
    cudaErrorMapBufferObjectFailed = 205,
    cudaErrorUnmapBufferObjectFailed = 206,
    cudaErrorArrayIsMapped = 207,
    cudaErrorAlreadyMapped = 208,
    cudaErrorNoKernelImageForDevice = 209,
    cudaErrorAlreadyAcquired = 210,
    cudaErrorNotMapped = 211,
    cudaErrorNotMappedAsArray = 212,
    cudaErrorNotMappedAsPointer = 213,
    cudaErrorECCUncorrectable = 214,
    cudaErrorUnsupportedLimit = 215,
    cudaErrorDeviceAlreadyInUse = 216,
    cudaErrorPeerAccessUnsupported = 217,
    cudaErrorInvalidPtx = 218,
    cudaErrorInvalidGraphicsContext = 219,
    cudaErrorNvlinkUncorrectable = 220,
    cudaErrorJitCompilerNotFound = 221,
    cudaErrorUnsupportedPtxVersion = 222,
    cudaErrorJitCompilationDisabled = 223,
    cudaErrorUnsupportedExecAffinity = 224,
    cudaErrorUnsupportedDevSideSync = 225,
    cudaErrorInvalidSource = 300,
    cudaErrorFileNotFound = 301,
    cudaErrorSharedObjectSymbolNotFound = 302,
    cudaErrorSharedObjectInitFailed = 303,
    cudaErrorOperatingSystem = 304,
    cudaErrorInvalidResourceHandle = 400,
    cudaErrorIllegalState = 401,
    cudaErrorLossyQuery = 402,
    cudaErrorSymbolNotFound = 500,
    cudaErrorNotReady = 600,
    cudaErrorIllegalAddress = 700,
    cudaErrorLaunchOutOfResources = 701,
    cudaErrorLaunchTimeout = 702,
    cudaErrorLaunchIncompatibleTexturing = 703,
    cudaErrorPeerAccessAlreadyEnabled = 704,
    cudaErrorPeerAccessNotEnabled = 705,
    cudaErrorSetOnActiveProcess = 708,
    cudaErrorContextIsDestroyed = 709,
    cudaErrorAssert = 710,
    cudaErrorTooManyPeers = 711,
    cudaErrorHostMemoryAlreadyRegistered = 712,
    cudaErrorHostMemoryNotRegistered = 713,
    cudaErrorHardwareStackError = 714,
    cudaErrorIllegalInstruction = 715,
    cudaErrorMisalignedAddress = 716,
    cudaErrorInvalidAddressSpace = 717,
    cudaErrorInvalidPc = 718,
    cudaErrorLaunchFailure = 719,
    cudaErrorCooperativeLaunchTooLarge = 720,
    cudaErrorNotPermitted = 800,
    cudaErrorNotSupported = 801,
    cudaErrorSystemNotReady = 802,
    cudaErrorSystemDriverMismatch = 803,
    cudaErrorCompatNotSupportedOnDevice = 804,
    cudaErrorMpsConnectionFailed = 805,
    cudaErrorMpsRpcFailure = 806,
    cudaErrorMpsServerNotReady = 807,
    cudaErrorMpsMaxClientsReached = 808,
    cudaErrorMpsMaxConnectionsReached = 809,
    cudaErrorMpsClientTerminated = 810,
    cudaErrorCdpNotSupported = 811,
    cudaErrorCdpVersionMismatch = 812,
    cudaErrorStreamCaptureUnsupported = 900,
    cudaErrorStreamCaptureInvalidated = 901,
    cudaErrorStreamCaptureMerge = 902,
    cudaErrorStreamCaptureUnmatched = 903,
    cudaErrorStreamCaptureUnjoined = 904,
    cudaErrorStreamCaptureIsolation = 905,
    cudaErrorStreamCaptureImplicit = 906,
    cudaErrorCapturedEvent = 907,
    cudaErrorStreamCaptureWrongThread = 908,
    cudaErrorTimeout = 909,
    cudaErrorGraphExecUpdateFailure = 910,
    cudaErrorExternalDevice = 911,
    cudaErrorInvalidClusterSize = 912,
    cudaErrorFunctionNotLoaded = 913,
    cudaErrorInvalidResourceType = 914,
    cudaErrorInvalidResourceConfiguration = 915,
    cudaErrorUnknown = 999,
    cudaErrorApiFailureBase = 10000,
}
#[cfg(any(feature = "cuda-12080", feature = "cuda-12090", feature = "cuda-13000"))]
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub enum cudaError {
    cudaSuccess = 0,
    cudaErrorInvalidValue = 1,
    cudaErrorMemoryAllocation = 2,
    cudaErrorInitializationError = 3,
    cudaErrorCudartUnloading = 4,
    cudaErrorProfilerDisabled = 5,
    cudaErrorProfilerNotInitialized = 6,
    cudaErrorProfilerAlreadyStarted = 7,
    cudaErrorProfilerAlreadyStopped = 8,
    cudaErrorInvalidConfiguration = 9,
    cudaErrorInvalidPitchValue = 12,
    cudaErrorInvalidSymbol = 13,
    cudaErrorInvalidHostPointer = 16,
    cudaErrorInvalidDevicePointer = 17,
    cudaErrorInvalidTexture = 18,
    cudaErrorInvalidTextureBinding = 19,
    cudaErrorInvalidChannelDescriptor = 20,
    cudaErrorInvalidMemcpyDirection = 21,
    cudaErrorAddressOfConstant = 22,
    cudaErrorTextureFetchFailed = 23,
    cudaErrorTextureNotBound = 24,
    cudaErrorSynchronizationError = 25,
    cudaErrorInvalidFilterSetting = 26,
    cudaErrorInvalidNormSetting = 27,
    cudaErrorMixedDeviceExecution = 28,
    cudaErrorNotYetImplemented = 31,
    cudaErrorMemoryValueTooLarge = 32,
    cudaErrorStubLibrary = 34,
    cudaErrorInsufficientDriver = 35,
    cudaErrorCallRequiresNewerDriver = 36,
    cudaErrorInvalidSurface = 37,
    cudaErrorDuplicateVariableName = 43,
    cudaErrorDuplicateTextureName = 44,
    cudaErrorDuplicateSurfaceName = 45,
    cudaErrorDevicesUnavailable = 46,
    cudaErrorIncompatibleDriverContext = 49,
    cudaErrorMissingConfiguration = 52,
    cudaErrorPriorLaunchFailure = 53,
    cudaErrorLaunchMaxDepthExceeded = 65,
    cudaErrorLaunchFileScopedTex = 66,
    cudaErrorLaunchFileScopedSurf = 67,
    cudaErrorSyncDepthExceeded = 68,
    cudaErrorLaunchPendingCountExceeded = 69,
    cudaErrorInvalidDeviceFunction = 98,
    cudaErrorNoDevice = 100,
    cudaErrorInvalidDevice = 101,
    cudaErrorDeviceNotLicensed = 102,
    cudaErrorSoftwareValidityNotEstablished = 103,
    cudaErrorStartupFailure = 127,
    cudaErrorInvalidKernelImage = 200,
    cudaErrorDeviceUninitialized = 201,
    cudaErrorMapBufferObjectFailed = 205,
    cudaErrorUnmapBufferObjectFailed = 206,
    cudaErrorArrayIsMapped = 207,
    cudaErrorAlreadyMapped = 208,
    cudaErrorNoKernelImageForDevice = 209,
    cudaErrorAlreadyAcquired = 210,
    cudaErrorNotMapped = 211,
    cudaErrorNotMappedAsArray = 212,
    cudaErrorNotMappedAsPointer = 213,
    cudaErrorECCUncorrectable = 214,
    cudaErrorUnsupportedLimit = 215,
    cudaErrorDeviceAlreadyInUse = 216,
    cudaErrorPeerAccessUnsupported = 217,
    cudaErrorInvalidPtx = 218,
    cudaErrorInvalidGraphicsContext = 219,
    cudaErrorNvlinkUncorrectable = 220,
    cudaErrorJitCompilerNotFound = 221,
    cudaErrorUnsupportedPtxVersion = 222,
    cudaErrorJitCompilationDisabled = 223,
    cudaErrorUnsupportedExecAffinity = 224,
    cudaErrorUnsupportedDevSideSync = 225,
    cudaErrorContained = 226,
    cudaErrorInvalidSource = 300,
    cudaErrorFileNotFound = 301,
    cudaErrorSharedObjectSymbolNotFound = 302,
    cudaErrorSharedObjectInitFailed = 303,
    cudaErrorOperatingSystem = 304,
    cudaErrorInvalidResourceHandle = 400,
    cudaErrorIllegalState = 401,
    cudaErrorLossyQuery = 402,
    cudaErrorSymbolNotFound = 500,
    cudaErrorNotReady = 600,
    cudaErrorIllegalAddress = 700,
    cudaErrorLaunchOutOfResources = 701,
    cudaErrorLaunchTimeout = 702,
    cudaErrorLaunchIncompatibleTexturing = 703,
    cudaErrorPeerAccessAlreadyEnabled = 704,
    cudaErrorPeerAccessNotEnabled = 705,
    cudaErrorSetOnActiveProcess = 708,
    cudaErrorContextIsDestroyed = 709,
    cudaErrorAssert = 710,
    cudaErrorTooManyPeers = 711,
    cudaErrorHostMemoryAlreadyRegistered = 712,
    cudaErrorHostMemoryNotRegistered = 713,
    cudaErrorHardwareStackError = 714,
    cudaErrorIllegalInstruction = 715,
    cudaErrorMisalignedAddress = 716,
    cudaErrorInvalidAddressSpace = 717,
    cudaErrorInvalidPc = 718,
    cudaErrorLaunchFailure = 719,
    cudaErrorCooperativeLaunchTooLarge = 720,
    cudaErrorTensorMemoryLeak = 721,
    cudaErrorNotPermitted = 800,
    cudaErrorNotSupported = 801,
    cudaErrorSystemNotReady = 802,
    cudaErrorSystemDriverMismatch = 803,
    cudaErrorCompatNotSupportedOnDevice = 804,
    cudaErrorMpsConnectionFailed = 805,
    cudaErrorMpsRpcFailure = 806,
    cudaErrorMpsServerNotReady = 807,
    cudaErrorMpsMaxClientsReached = 808,
    cudaErrorMpsMaxConnectionsReached = 809,
    cudaErrorMpsClientTerminated = 810,
    cudaErrorCdpNotSupported = 811,
    cudaErrorCdpVersionMismatch = 812,
    cudaErrorStreamCaptureUnsupported = 900,
    cudaErrorStreamCaptureInvalidated = 901,
    cudaErrorStreamCaptureMerge = 902,
    cudaErrorStreamCaptureUnmatched = 903,
    cudaErrorStreamCaptureUnjoined = 904,
    cudaErrorStreamCaptureIsolation = 905,
    cudaErrorStreamCaptureImplicit = 906,
    cudaErrorCapturedEvent = 907,
    cudaErrorStreamCaptureWrongThread = 908,
    cudaErrorTimeout = 909,
    cudaErrorGraphExecUpdateFailure = 910,
    cudaErrorExternalDevice = 911,
    cudaErrorInvalidClusterSize = 912,
    cudaErrorFunctionNotLoaded = 913,
    cudaErrorInvalidResourceType = 914,
    cudaErrorInvalidResourceConfiguration = 915,
    cudaErrorUnknown = 999,
    cudaErrorApiFailureBase = 10000,
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub enum cudaExternalMemoryHandleType {
    cudaExternalMemoryHandleTypeOpaqueFd = 1,
    cudaExternalMemoryHandleTypeOpaqueWin32 = 2,
    cudaExternalMemoryHandleTypeOpaqueWin32Kmt = 3,
    cudaExternalMemoryHandleTypeD3D12Heap = 4,
    cudaExternalMemoryHandleTypeD3D12Resource = 5,
    cudaExternalMemoryHandleTypeD3D11Resource = 6,
    cudaExternalMemoryHandleTypeD3D11ResourceKmt = 7,
    cudaExternalMemoryHandleTypeNvSciBuf = 8,
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub enum cudaExternalSemaphoreHandleType {
    cudaExternalSemaphoreHandleTypeOpaqueFd = 1,
    cudaExternalSemaphoreHandleTypeOpaqueWin32 = 2,
    cudaExternalSemaphoreHandleTypeOpaqueWin32Kmt = 3,
    cudaExternalSemaphoreHandleTypeD3D12Fence = 4,
    cudaExternalSemaphoreHandleTypeD3D11Fence = 5,
    cudaExternalSemaphoreHandleTypeNvSciSync = 6,
    cudaExternalSemaphoreHandleTypeKeyedMutex = 7,
    cudaExternalSemaphoreHandleTypeKeyedMutexKmt = 8,
    cudaExternalSemaphoreHandleTypeTimelineSemaphoreFd = 9,
    cudaExternalSemaphoreHandleTypeTimelineSemaphoreWin32 = 10,
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub enum cudaFlushGPUDirectRDMAWritesOptions {
    cudaFlushGPUDirectRDMAWritesOptionHost = 1,
    cudaFlushGPUDirectRDMAWritesOptionMemOps = 2,
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub enum cudaFlushGPUDirectRDMAWritesScope {
    cudaFlushGPUDirectRDMAWritesToOwner = 100,
    cudaFlushGPUDirectRDMAWritesToAllDevices = 200,
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub enum cudaFlushGPUDirectRDMAWritesTarget {
    cudaFlushGPUDirectRDMAWritesTargetCurrentDevice = 0,
}
#[cfg(any(
    feature = "cuda-11040",
    feature = "cuda-11050",
    feature = "cuda-11060",
    feature = "cuda-11070"
))]
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub enum cudaFuncAttribute {
    cudaFuncAttributeMaxDynamicSharedMemorySize = 8,
    cudaFuncAttributePreferredSharedMemoryCarveout = 9,
    cudaFuncAttributeMax = 10,
}
#[cfg(any(
    feature = "cuda-11080",
    feature = "cuda-12000",
    feature = "cuda-12010",
    feature = "cuda-12020",
    feature = "cuda-12030",
    feature = "cuda-12040",
    feature = "cuda-12050",
    feature = "cuda-12060",
    feature = "cuda-12080",
    feature = "cuda-12090",
    feature = "cuda-13000"
))]
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub enum cudaFuncAttribute {
    cudaFuncAttributeMaxDynamicSharedMemorySize = 8,
    cudaFuncAttributePreferredSharedMemoryCarveout = 9,
    cudaFuncAttributeClusterDimMustBeSet = 10,
    cudaFuncAttributeRequiredClusterWidth = 11,
    cudaFuncAttributeRequiredClusterHeight = 12,
    cudaFuncAttributeRequiredClusterDepth = 13,
    cudaFuncAttributeNonPortableClusterSizeAllowed = 14,
    cudaFuncAttributeClusterSchedulingPolicyPreference = 15,
    cudaFuncAttributeMax = 16,
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub enum cudaFuncCache {
    cudaFuncCachePreferNone = 0,
    cudaFuncCachePreferShared = 1,
    cudaFuncCachePreferL1 = 2,
    cudaFuncCachePreferEqual = 3,
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub enum cudaGPUDirectRDMAWritesOrdering {
    cudaGPUDirectRDMAWritesOrderingNone = 0,
    cudaGPUDirectRDMAWritesOrderingOwner = 100,
    cudaGPUDirectRDMAWritesOrderingAllDevices = 200,
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub enum cudaGetDriverEntryPointFlags {
    cudaEnableDefault = 0,
    cudaEnableLegacyStream = 1,
    cudaEnablePerThreadDefaultStream = 2,
}
#[cfg(any(feature = "cuda-12090", feature = "cuda-13000"))]
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub enum cudaGraphChildGraphNodeOwnership {
    cudaGraphChildGraphOwnershipClone = 0,
    cudaGraphChildGraphOwnershipMove = 1,
}
#[cfg(any(
    feature = "cuda-12030",
    feature = "cuda-12040",
    feature = "cuda-12050",
    feature = "cuda-12060",
    feature = "cuda-12080",
    feature = "cuda-12090",
    feature = "cuda-13000"
))]
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub enum cudaGraphConditionalHandleFlags {
    cudaGraphCondAssignDefault = 1,
}
#[cfg(any(
    feature = "cuda-12030",
    feature = "cuda-12040",
    feature = "cuda-12050",
    feature = "cuda-12060"
))]
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub enum cudaGraphConditionalNodeType {
    cudaGraphCondTypeIf = 0,
    cudaGraphCondTypeWhile = 1,
}
#[cfg(any(feature = "cuda-12080", feature = "cuda-12090", feature = "cuda-13000"))]
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub enum cudaGraphConditionalNodeType {
    cudaGraphCondTypeIf = 0,
    cudaGraphCondTypeWhile = 1,
    cudaGraphCondTypeSwitch = 2,
}
#[cfg(any(
    feature = "cuda-11040",
    feature = "cuda-11050",
    feature = "cuda-11060",
    feature = "cuda-11070",
    feature = "cuda-11080",
    feature = "cuda-12000",
    feature = "cuda-12010",
    feature = "cuda-12020"
))]
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub enum cudaGraphDebugDotFlags {
    cudaGraphDebugDotFlagsVerbose = 1,
    cudaGraphDebugDotFlagsKernelNodeParams = 4,
    cudaGraphDebugDotFlagsMemcpyNodeParams = 8,
    cudaGraphDebugDotFlagsMemsetNodeParams = 16,
    cudaGraphDebugDotFlagsHostNodeParams = 32,
    cudaGraphDebugDotFlagsEventNodeParams = 64,
    cudaGraphDebugDotFlagsExtSemasSignalNodeParams = 128,
    cudaGraphDebugDotFlagsExtSemasWaitNodeParams = 256,
    cudaGraphDebugDotFlagsKernelNodeAttributes = 512,
    cudaGraphDebugDotFlagsHandles = 1024,
}
#[cfg(any(
    feature = "cuda-12030",
    feature = "cuda-12040",
    feature = "cuda-12050",
    feature = "cuda-12060",
    feature = "cuda-12080",
    feature = "cuda-12090",
    feature = "cuda-13000"
))]
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub enum cudaGraphDebugDotFlags {
    cudaGraphDebugDotFlagsVerbose = 1,
    cudaGraphDebugDotFlagsKernelNodeParams = 4,
    cudaGraphDebugDotFlagsMemcpyNodeParams = 8,
    cudaGraphDebugDotFlagsMemsetNodeParams = 16,
    cudaGraphDebugDotFlagsHostNodeParams = 32,
    cudaGraphDebugDotFlagsEventNodeParams = 64,
    cudaGraphDebugDotFlagsExtSemasSignalNodeParams = 128,
    cudaGraphDebugDotFlagsExtSemasWaitNodeParams = 256,
    cudaGraphDebugDotFlagsKernelNodeAttributes = 512,
    cudaGraphDebugDotFlagsHandles = 1024,
    cudaGraphDebugDotFlagsConditionalNodeParams = 32768,
}
#[cfg(any(
    feature = "cuda-12030",
    feature = "cuda-12040",
    feature = "cuda-12050",
    feature = "cuda-12060",
    feature = "cuda-12080",
    feature = "cuda-12090",
    feature = "cuda-13000"
))]
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub enum cudaGraphDependencyType_enum {
    cudaGraphDependencyTypeDefault = 0,
    cudaGraphDependencyTypeProgrammatic = 1,
}
#[cfg(any(feature = "cuda-11040", feature = "cuda-11050"))]
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub enum cudaGraphExecUpdateResult {
    cudaGraphExecUpdateSuccess = 0,
    cudaGraphExecUpdateError = 1,
    cudaGraphExecUpdateErrorTopologyChanged = 2,
    cudaGraphExecUpdateErrorNodeTypeChanged = 3,
    cudaGraphExecUpdateErrorFunctionChanged = 4,
    cudaGraphExecUpdateErrorParametersChanged = 5,
    cudaGraphExecUpdateErrorNotSupported = 6,
    cudaGraphExecUpdateErrorUnsupportedFunctionChange = 7,
}
#[cfg(any(
    feature = "cuda-11060",
    feature = "cuda-11070",
    feature = "cuda-11080",
    feature = "cuda-12000",
    feature = "cuda-12010",
    feature = "cuda-12020",
    feature = "cuda-12030",
    feature = "cuda-12040",
    feature = "cuda-12050",
    feature = "cuda-12060",
    feature = "cuda-12080",
    feature = "cuda-12090",
    feature = "cuda-13000"
))]
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub enum cudaGraphExecUpdateResult {
    cudaGraphExecUpdateSuccess = 0,
    cudaGraphExecUpdateError = 1,
    cudaGraphExecUpdateErrorTopologyChanged = 2,
    cudaGraphExecUpdateErrorNodeTypeChanged = 3,
    cudaGraphExecUpdateErrorFunctionChanged = 4,
    cudaGraphExecUpdateErrorParametersChanged = 5,
    cudaGraphExecUpdateErrorNotSupported = 6,
    cudaGraphExecUpdateErrorUnsupportedFunctionChange = 7,
    cudaGraphExecUpdateErrorAttributesChanged = 8,
}
#[cfg(any(feature = "cuda-11040", feature = "cuda-11050", feature = "cuda-11060"))]
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub enum cudaGraphInstantiateFlags {
    cudaGraphInstantiateFlagAutoFreeOnLaunch = 1,
}
#[cfg(any(feature = "cuda-11070", feature = "cuda-11080"))]
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub enum cudaGraphInstantiateFlags {
    cudaGraphInstantiateFlagAutoFreeOnLaunch = 1,
    cudaGraphInstantiateFlagUseNodePriority = 8,
}
#[cfg(any(
    feature = "cuda-12000",
    feature = "cuda-12010",
    feature = "cuda-12020",
    feature = "cuda-12030",
    feature = "cuda-12040",
    feature = "cuda-12050",
    feature = "cuda-12060",
    feature = "cuda-12080",
    feature = "cuda-12090",
    feature = "cuda-13000"
))]
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub enum cudaGraphInstantiateFlags {
    cudaGraphInstantiateFlagAutoFreeOnLaunch = 1,
    cudaGraphInstantiateFlagUpload = 2,
    cudaGraphInstantiateFlagDeviceLaunch = 4,
    cudaGraphInstantiateFlagUseNodePriority = 8,
}
#[cfg(any(
    feature = "cuda-12000",
    feature = "cuda-12010",
    feature = "cuda-12020",
    feature = "cuda-12030",
    feature = "cuda-12040",
    feature = "cuda-12050",
    feature = "cuda-12060"
))]
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub enum cudaGraphInstantiateResult {
    cudaGraphInstantiateSuccess = 0,
    cudaGraphInstantiateError = 1,
    cudaGraphInstantiateInvalidStructure = 2,
    cudaGraphInstantiateNodeOperationNotSupported = 3,
    cudaGraphInstantiateMultipleDevicesNotSupported = 4,
}
#[cfg(any(feature = "cuda-12080", feature = "cuda-12090", feature = "cuda-13000"))]
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub enum cudaGraphInstantiateResult {
    cudaGraphInstantiateSuccess = 0,
    cudaGraphInstantiateError = 1,
    cudaGraphInstantiateInvalidStructure = 2,
    cudaGraphInstantiateNodeOperationNotSupported = 3,
    cudaGraphInstantiateMultipleDevicesNotSupported = 4,
    cudaGraphInstantiateConditionalHandleUnused = 5,
}
#[cfg(any(
    feature = "cuda-12040",
    feature = "cuda-12050",
    feature = "cuda-12060",
    feature = "cuda-12080",
    feature = "cuda-12090",
    feature = "cuda-13000"
))]
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub enum cudaGraphKernelNodeField {
    cudaGraphKernelNodeFieldInvalid = 0,
    cudaGraphKernelNodeFieldGridDim = 1,
    cudaGraphKernelNodeFieldParam = 2,
    cudaGraphKernelNodeFieldEnabled = 3,
}
#[cfg(any(feature = "cuda-11040", feature = "cuda-11050"))]
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub enum cudaGraphMemAttributeType {
    cudaGraphMemAttrUsedMemCurrent = 1,
    cudaGraphMemAttrUsedMemHigh = 2,
    cudaGraphMemAttrReservedMemCurrent = 3,
    cudaGraphMemAttrReservedMemHigh = 4,
}
#[cfg(any(
    feature = "cuda-11060",
    feature = "cuda-11070",
    feature = "cuda-11080",
    feature = "cuda-12000",
    feature = "cuda-12010",
    feature = "cuda-12020",
    feature = "cuda-12030",
    feature = "cuda-12040",
    feature = "cuda-12050",
    feature = "cuda-12060",
    feature = "cuda-12080",
    feature = "cuda-12090",
    feature = "cuda-13000"
))]
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub enum cudaGraphMemAttributeType {
    cudaGraphMemAttrUsedMemCurrent = 0,
    cudaGraphMemAttrUsedMemHigh = 1,
    cudaGraphMemAttrReservedMemCurrent = 2,
    cudaGraphMemAttrReservedMemHigh = 3,
}
#[cfg(any(
    feature = "cuda-11040",
    feature = "cuda-11050",
    feature = "cuda-11060",
    feature = "cuda-11070",
    feature = "cuda-11080",
    feature = "cuda-12000",
    feature = "cuda-12010",
    feature = "cuda-12020"
))]
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub enum cudaGraphNodeType {
    cudaGraphNodeTypeKernel = 0,
    cudaGraphNodeTypeMemcpy = 1,
    cudaGraphNodeTypeMemset = 2,
    cudaGraphNodeTypeHost = 3,
    cudaGraphNodeTypeGraph = 4,
    cudaGraphNodeTypeEmpty = 5,
    cudaGraphNodeTypeWaitEvent = 6,
    cudaGraphNodeTypeEventRecord = 7,
    cudaGraphNodeTypeExtSemaphoreSignal = 8,
    cudaGraphNodeTypeExtSemaphoreWait = 9,
    cudaGraphNodeTypeMemAlloc = 10,
    cudaGraphNodeTypeMemFree = 11,
    cudaGraphNodeTypeCount = 12,
}
#[cfg(any(
    feature = "cuda-12030",
    feature = "cuda-12040",
    feature = "cuda-12050",
    feature = "cuda-12060",
    feature = "cuda-12080",
    feature = "cuda-12090",
    feature = "cuda-13000"
))]
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub enum cudaGraphNodeType {
    cudaGraphNodeTypeKernel = 0,
    cudaGraphNodeTypeMemcpy = 1,
    cudaGraphNodeTypeMemset = 2,
    cudaGraphNodeTypeHost = 3,
    cudaGraphNodeTypeGraph = 4,
    cudaGraphNodeTypeEmpty = 5,
    cudaGraphNodeTypeWaitEvent = 6,
    cudaGraphNodeTypeEventRecord = 7,
    cudaGraphNodeTypeExtSemaphoreSignal = 8,
    cudaGraphNodeTypeExtSemaphoreWait = 9,
    cudaGraphNodeTypeMemAlloc = 10,
    cudaGraphNodeTypeMemFree = 11,
    cudaGraphNodeTypeConditional = 13,
    cudaGraphNodeTypeCount = 14,
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub enum cudaGraphicsCubeFace {
    cudaGraphicsCubeFacePositiveX = 0,
    cudaGraphicsCubeFaceNegativeX = 1,
    cudaGraphicsCubeFacePositiveY = 2,
    cudaGraphicsCubeFaceNegativeY = 3,
    cudaGraphicsCubeFacePositiveZ = 4,
    cudaGraphicsCubeFaceNegativeZ = 5,
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub enum cudaGraphicsMapFlags {
    cudaGraphicsMapFlagsNone = 0,
    cudaGraphicsMapFlagsReadOnly = 1,
    cudaGraphicsMapFlagsWriteDiscard = 2,
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub enum cudaGraphicsRegisterFlags {
    cudaGraphicsRegisterFlagsNone = 0,
    cudaGraphicsRegisterFlagsReadOnly = 1,
    cudaGraphicsRegisterFlagsWriteDiscard = 2,
    cudaGraphicsRegisterFlagsSurfaceLoadStore = 4,
    cudaGraphicsRegisterFlagsTextureGather = 8,
}
#[cfg(any(feature = "cuda-12080", feature = "cuda-12090", feature = "cuda-13000"))]
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub enum cudaJitOption {
    cudaJitMaxRegisters = 0,
    cudaJitThreadsPerBlock = 1,
    cudaJitWallTime = 2,
    cudaJitInfoLogBuffer = 3,
    cudaJitInfoLogBufferSizeBytes = 4,
    cudaJitErrorLogBuffer = 5,
    cudaJitErrorLogBufferSizeBytes = 6,
    cudaJitOptimizationLevel = 7,
    cudaJitFallbackStrategy = 10,
    cudaJitGenerateDebugInfo = 11,
    cudaJitLogVerbose = 12,
    cudaJitGenerateLineInfo = 13,
    cudaJitCacheMode = 14,
    cudaJitPositionIndependentCode = 30,
    cudaJitMinCtaPerSm = 31,
    cudaJitMaxThreadsPerBlock = 32,
    cudaJitOverrideDirectiveValues = 33,
}
#[cfg(any(feature = "cuda-12080", feature = "cuda-12090", feature = "cuda-13000"))]
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub enum cudaJit_CacheMode {
    cudaJitCacheOptionNone = 0,
    cudaJitCacheOptionCG = 1,
    cudaJitCacheOptionCA = 2,
}
#[cfg(any(feature = "cuda-12080", feature = "cuda-12090", feature = "cuda-13000"))]
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub enum cudaJit_Fallback {
    cudaPreferPtx = 0,
    cudaPreferBinary = 1,
}
#[cfg(any(feature = "cuda-11040", feature = "cuda-11050", feature = "cuda-11060"))]
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub enum cudaKernelNodeAttrID {
    cudaKernelNodeAttributeAccessPolicyWindow = 1,
    cudaKernelNodeAttributeCooperative = 2,
}
#[cfg(any(feature = "cuda-11070"))]
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub enum cudaKernelNodeAttrID {
    cudaKernelNodeAttributeAccessPolicyWindow = 1,
    cudaKernelNodeAttributeCooperative = 2,
    cudaKernelNodeAttributePriority = 8,
}
#[cfg(any(feature = "cuda-11080"))]
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub enum cudaLaunchAttributeID {
    cudaLaunchAttributeIgnore = 0,
    cudaLaunchAttributeAccessPolicyWindow = 1,
    cudaLaunchAttributeCooperative = 2,
    cudaLaunchAttributeSynchronizationPolicy = 3,
    cudaLaunchAttributeClusterDimension = 4,
    cudaLaunchAttributeClusterSchedulingPolicyPreference = 5,
    cudaLaunchAttributeProgrammaticStreamSerialization = 6,
    cudaLaunchAttributeProgrammaticEvent = 7,
    cudaLaunchAttributePriority = 8,
}
#[cfg(any(feature = "cuda-12000", feature = "cuda-12010", feature = "cuda-12020"))]
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub enum cudaLaunchAttributeID {
    cudaLaunchAttributeIgnore = 0,
    cudaLaunchAttributeAccessPolicyWindow = 1,
    cudaLaunchAttributeCooperative = 2,
    cudaLaunchAttributeSynchronizationPolicy = 3,
    cudaLaunchAttributeClusterDimension = 4,
    cudaLaunchAttributeClusterSchedulingPolicyPreference = 5,
    cudaLaunchAttributeProgrammaticStreamSerialization = 6,
    cudaLaunchAttributeProgrammaticEvent = 7,
    cudaLaunchAttributePriority = 8,
    cudaLaunchAttributeMemSyncDomainMap = 9,
    cudaLaunchAttributeMemSyncDomain = 10,
}
#[cfg(any(feature = "cuda-12030"))]
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub enum cudaLaunchAttributeID {
    cudaLaunchAttributeIgnore = 0,
    cudaLaunchAttributeAccessPolicyWindow = 1,
    cudaLaunchAttributeCooperative = 2,
    cudaLaunchAttributeSynchronizationPolicy = 3,
    cudaLaunchAttributeClusterDimension = 4,
    cudaLaunchAttributeClusterSchedulingPolicyPreference = 5,
    cudaLaunchAttributeProgrammaticStreamSerialization = 6,
    cudaLaunchAttributeProgrammaticEvent = 7,
    cudaLaunchAttributePriority = 8,
    cudaLaunchAttributeMemSyncDomainMap = 9,
    cudaLaunchAttributeMemSyncDomain = 10,
    cudaLaunchAttributeLaunchCompletionEvent = 12,
}
#[cfg(any(feature = "cuda-12040"))]
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub enum cudaLaunchAttributeID {
    cudaLaunchAttributeIgnore = 0,
    cudaLaunchAttributeAccessPolicyWindow = 1,
    cudaLaunchAttributeCooperative = 2,
    cudaLaunchAttributeSynchronizationPolicy = 3,
    cudaLaunchAttributeClusterDimension = 4,
    cudaLaunchAttributeClusterSchedulingPolicyPreference = 5,
    cudaLaunchAttributeProgrammaticStreamSerialization = 6,
    cudaLaunchAttributeProgrammaticEvent = 7,
    cudaLaunchAttributePriority = 8,
    cudaLaunchAttributeMemSyncDomainMap = 9,
    cudaLaunchAttributeMemSyncDomain = 10,
    cudaLaunchAttributeLaunchCompletionEvent = 12,
    cudaLaunchAttributeDeviceUpdatableKernelNode = 13,
}
#[cfg(any(feature = "cuda-12050", feature = "cuda-12060"))]
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub enum cudaLaunchAttributeID {
    cudaLaunchAttributeIgnore = 0,
    cudaLaunchAttributeAccessPolicyWindow = 1,
    cudaLaunchAttributeCooperative = 2,
    cudaLaunchAttributeSynchronizationPolicy = 3,
    cudaLaunchAttributeClusterDimension = 4,
    cudaLaunchAttributeClusterSchedulingPolicyPreference = 5,
    cudaLaunchAttributeProgrammaticStreamSerialization = 6,
    cudaLaunchAttributeProgrammaticEvent = 7,
    cudaLaunchAttributePriority = 8,
    cudaLaunchAttributeMemSyncDomainMap = 9,
    cudaLaunchAttributeMemSyncDomain = 10,
    cudaLaunchAttributeLaunchCompletionEvent = 12,
    cudaLaunchAttributeDeviceUpdatableKernelNode = 13,
    cudaLaunchAttributePreferredSharedMemoryCarveout = 14,
}
#[cfg(any(feature = "cuda-12080", feature = "cuda-12090"))]
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub enum cudaLaunchAttributeID {
    cudaLaunchAttributeIgnore = 0,
    cudaLaunchAttributeAccessPolicyWindow = 1,
    cudaLaunchAttributeCooperative = 2,
    cudaLaunchAttributeSynchronizationPolicy = 3,
    cudaLaunchAttributeClusterDimension = 4,
    cudaLaunchAttributeClusterSchedulingPolicyPreference = 5,
    cudaLaunchAttributeProgrammaticStreamSerialization = 6,
    cudaLaunchAttributeProgrammaticEvent = 7,
    cudaLaunchAttributePriority = 8,
    cudaLaunchAttributeMemSyncDomainMap = 9,
    cudaLaunchAttributeMemSyncDomain = 10,
    cudaLaunchAttributePreferredClusterDimension = 11,
    cudaLaunchAttributeLaunchCompletionEvent = 12,
    cudaLaunchAttributeDeviceUpdatableKernelNode = 13,
    cudaLaunchAttributePreferredSharedMemoryCarveout = 14,
}
#[cfg(any(feature = "cuda-13000"))]
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub enum cudaLaunchAttributeID {
    cudaLaunchAttributeIgnore = 0,
    cudaLaunchAttributeAccessPolicyWindow = 1,
    cudaLaunchAttributeCooperative = 2,
    cudaLaunchAttributeSynchronizationPolicy = 3,
    cudaLaunchAttributeClusterDimension = 4,
    cudaLaunchAttributeClusterSchedulingPolicyPreference = 5,
    cudaLaunchAttributeProgrammaticStreamSerialization = 6,
    cudaLaunchAttributeProgrammaticEvent = 7,
    cudaLaunchAttributePriority = 8,
    cudaLaunchAttributeMemSyncDomainMap = 9,
    cudaLaunchAttributeMemSyncDomain = 10,
    cudaLaunchAttributePreferredClusterDimension = 11,
    cudaLaunchAttributeLaunchCompletionEvent = 12,
    cudaLaunchAttributeDeviceUpdatableKernelNode = 13,
    cudaLaunchAttributePreferredSharedMemoryCarveout = 14,
    cudaLaunchAttributeNvlinkUtilCentricScheduling = 16,
}
#[cfg(any(
    feature = "cuda-12000",
    feature = "cuda-12010",
    feature = "cuda-12020",
    feature = "cuda-12030",
    feature = "cuda-12040",
    feature = "cuda-12050",
    feature = "cuda-12060",
    feature = "cuda-12080",
    feature = "cuda-12090",
    feature = "cuda-13000"
))]
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub enum cudaLaunchMemSyncDomain {
    cudaLaunchMemSyncDomainDefault = 0,
    cudaLaunchMemSyncDomainRemote = 1,
}
#[cfg(any(feature = "cuda-12080", feature = "cuda-12090", feature = "cuda-13000"))]
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub enum cudaLibraryOption {
    cudaLibraryHostUniversalFunctionAndDataTable = 0,
    cudaLibraryBinaryIsPreserved = 1,
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub enum cudaLimit {
    cudaLimitStackSize = 0,
    cudaLimitPrintfFifoSize = 1,
    cudaLimitMallocHeapSize = 2,
    cudaLimitDevRuntimeSyncDepth = 3,
    cudaLimitDevRuntimePendingLaunchCount = 4,
    cudaLimitMaxL2FetchGranularity = 5,
    cudaLimitPersistingL2CacheSize = 6,
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub enum cudaMemAccessFlags {
    cudaMemAccessFlagsProtNone = 0,
    cudaMemAccessFlagsProtRead = 1,
    cudaMemAccessFlagsProtReadWrite = 3,
}
#[cfg(any(
    feature = "cuda-11040",
    feature = "cuda-11050",
    feature = "cuda-11060",
    feature = "cuda-11070",
    feature = "cuda-11080",
    feature = "cuda-12000",
    feature = "cuda-12010",
    feature = "cuda-12020",
    feature = "cuda-12030"
))]
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub enum cudaMemAllocationHandleType {
    cudaMemHandleTypeNone = 0,
    cudaMemHandleTypePosixFileDescriptor = 1,
    cudaMemHandleTypeWin32 = 2,
    cudaMemHandleTypeWin32Kmt = 4,
}
#[cfg(any(
    feature = "cuda-12040",
    feature = "cuda-12050",
    feature = "cuda-12060",
    feature = "cuda-12080",
    feature = "cuda-12090",
    feature = "cuda-13000"
))]
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub enum cudaMemAllocationHandleType {
    cudaMemHandleTypeNone = 0,
    cudaMemHandleTypePosixFileDescriptor = 1,
    cudaMemHandleTypeWin32 = 2,
    cudaMemHandleTypeWin32Kmt = 4,
    cudaMemHandleTypeFabric = 8,
}
#[cfg(any(
    feature = "cuda-11040",
    feature = "cuda-11050",
    feature = "cuda-11060",
    feature = "cuda-11070",
    feature = "cuda-11080",
    feature = "cuda-12000",
    feature = "cuda-12010",
    feature = "cuda-12020",
    feature = "cuda-12030",
    feature = "cuda-12040",
    feature = "cuda-12050",
    feature = "cuda-12060",
    feature = "cuda-12080",
    feature = "cuda-12090"
))]
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub enum cudaMemAllocationType {
    cudaMemAllocationTypeInvalid = 0,
    cudaMemAllocationTypePinned = 1,
    cudaMemAllocationTypeMax = 2147483647,
}
#[cfg(any(feature = "cuda-13000"))]
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub enum cudaMemAllocationType {
    cudaMemAllocationTypeInvalid = 0,
    cudaMemAllocationTypePinned = 1,
    cudaMemAllocationTypeManaged = 2,
    cudaMemAllocationTypeMax = 2147483647,
}
#[cfg(any(
    feature = "cuda-11040",
    feature = "cuda-11050",
    feature = "cuda-11060",
    feature = "cuda-11070",
    feature = "cuda-11080",
    feature = "cuda-12000",
    feature = "cuda-12010"
))]
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub enum cudaMemLocationType {
    cudaMemLocationTypeInvalid = 0,
    cudaMemLocationTypeDevice = 1,
}
#[cfg(any(
    feature = "cuda-12020",
    feature = "cuda-12030",
    feature = "cuda-12040",
    feature = "cuda-12050",
    feature = "cuda-12060",
    feature = "cuda-12080",
    feature = "cuda-12090",
    feature = "cuda-13000"
))]
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub enum cudaMemLocationType {
    cudaMemLocationTypeInvalid = 0,
    cudaMemLocationTypeDevice = 1,
    cudaMemLocationTypeHost = 2,
    cudaMemLocationTypeHostNuma = 3,
    cudaMemLocationTypeHostNumaCurrent = 4,
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub enum cudaMemPoolAttr {
    cudaMemPoolReuseFollowEventDependencies = 1,
    cudaMemPoolReuseAllowOpportunistic = 2,
    cudaMemPoolReuseAllowInternalDependencies = 3,
    cudaMemPoolAttrReleaseThreshold = 4,
    cudaMemPoolAttrReservedMemCurrent = 5,
    cudaMemPoolAttrReservedMemHigh = 6,
    cudaMemPoolAttrUsedMemCurrent = 7,
    cudaMemPoolAttrUsedMemHigh = 8,
}
#[cfg(any(
    feature = "cuda-11040",
    feature = "cuda-11050",
    feature = "cuda-11060",
    feature = "cuda-11070",
    feature = "cuda-11080",
    feature = "cuda-12000",
    feature = "cuda-12010"
))]
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub enum cudaMemRangeAttribute {
    cudaMemRangeAttributeReadMostly = 1,
    cudaMemRangeAttributePreferredLocation = 2,
    cudaMemRangeAttributeAccessedBy = 3,
    cudaMemRangeAttributeLastPrefetchLocation = 4,
}
#[cfg(any(
    feature = "cuda-12020",
    feature = "cuda-12030",
    feature = "cuda-12040",
    feature = "cuda-12050",
    feature = "cuda-12060",
    feature = "cuda-12080",
    feature = "cuda-12090",
    feature = "cuda-13000"
))]
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub enum cudaMemRangeAttribute {
    cudaMemRangeAttributeReadMostly = 1,
    cudaMemRangeAttributePreferredLocation = 2,
    cudaMemRangeAttributeAccessedBy = 3,
    cudaMemRangeAttributeLastPrefetchLocation = 4,
    cudaMemRangeAttributePreferredLocationType = 5,
    cudaMemRangeAttributePreferredLocationId = 6,
    cudaMemRangeAttributeLastPrefetchLocationType = 7,
    cudaMemRangeAttributeLastPrefetchLocationId = 8,
}
#[cfg(any(feature = "cuda-12080", feature = "cuda-12090", feature = "cuda-13000"))]
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub enum cudaMemcpy3DOperandType {
    cudaMemcpyOperandTypePointer = 1,
    cudaMemcpyOperandTypeArray = 2,
    cudaMemcpyOperandTypeMax = 2147483647,
}
#[cfg(any(feature = "cuda-12080", feature = "cuda-12090", feature = "cuda-13000"))]
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub enum cudaMemcpyFlags {
    cudaMemcpyFlagDefault = 0,
    cudaMemcpyFlagPreferOverlapWithCompute = 1,
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub enum cudaMemcpyKind {
    cudaMemcpyHostToHost = 0,
    cudaMemcpyHostToDevice = 1,
    cudaMemcpyDeviceToHost = 2,
    cudaMemcpyDeviceToDevice = 3,
    cudaMemcpyDefault = 4,
}
#[cfg(any(feature = "cuda-12080", feature = "cuda-12090", feature = "cuda-13000"))]
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub enum cudaMemcpySrcAccessOrder {
    cudaMemcpySrcAccessOrderInvalid = 0,
    cudaMemcpySrcAccessOrderStream = 1,
    cudaMemcpySrcAccessOrderDuringApiCall = 2,
    cudaMemcpySrcAccessOrderAny = 3,
    cudaMemcpySrcAccessOrderMax = 2147483647,
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub enum cudaMemoryAdvise {
    cudaMemAdviseSetReadMostly = 1,
    cudaMemAdviseUnsetReadMostly = 2,
    cudaMemAdviseSetPreferredLocation = 3,
    cudaMemAdviseUnsetPreferredLocation = 4,
    cudaMemAdviseSetAccessedBy = 5,
    cudaMemAdviseUnsetAccessedBy = 6,
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub enum cudaMemoryType {
    cudaMemoryTypeUnregistered = 0,
    cudaMemoryTypeHost = 1,
    cudaMemoryTypeDevice = 2,
    cudaMemoryTypeManaged = 3,
}
#[cfg(any(
    feature = "cuda-11040",
    feature = "cuda-11050",
    feature = "cuda-11060",
    feature = "cuda-11070",
    feature = "cuda-11080"
))]
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub enum cudaOutputMode {
    cudaKeyValuePair = 0,
    cudaCSV = 1,
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub enum cudaResourceType {
    cudaResourceTypeArray = 0,
    cudaResourceTypeMipmappedArray = 1,
    cudaResourceTypeLinear = 2,
    cudaResourceTypePitch2D = 3,
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub enum cudaResourceViewFormat {
    cudaResViewFormatNone = 0,
    cudaResViewFormatUnsignedChar1 = 1,
    cudaResViewFormatUnsignedChar2 = 2,
    cudaResViewFormatUnsignedChar4 = 3,
    cudaResViewFormatSignedChar1 = 4,
    cudaResViewFormatSignedChar2 = 5,
    cudaResViewFormatSignedChar4 = 6,
    cudaResViewFormatUnsignedShort1 = 7,
    cudaResViewFormatUnsignedShort2 = 8,
    cudaResViewFormatUnsignedShort4 = 9,
    cudaResViewFormatSignedShort1 = 10,
    cudaResViewFormatSignedShort2 = 11,
    cudaResViewFormatSignedShort4 = 12,
    cudaResViewFormatUnsignedInt1 = 13,
    cudaResViewFormatUnsignedInt2 = 14,
    cudaResViewFormatUnsignedInt4 = 15,
    cudaResViewFormatSignedInt1 = 16,
    cudaResViewFormatSignedInt2 = 17,
    cudaResViewFormatSignedInt4 = 18,
    cudaResViewFormatHalf1 = 19,
    cudaResViewFormatHalf2 = 20,
    cudaResViewFormatHalf4 = 21,
    cudaResViewFormatFloat1 = 22,
    cudaResViewFormatFloat2 = 23,
    cudaResViewFormatFloat4 = 24,
    cudaResViewFormatUnsignedBlockCompressed1 = 25,
    cudaResViewFormatUnsignedBlockCompressed2 = 26,
    cudaResViewFormatUnsignedBlockCompressed3 = 27,
    cudaResViewFormatUnsignedBlockCompressed4 = 28,
    cudaResViewFormatSignedBlockCompressed4 = 29,
    cudaResViewFormatUnsignedBlockCompressed5 = 30,
    cudaResViewFormatSignedBlockCompressed5 = 31,
    cudaResViewFormatUnsignedBlockCompressed6H = 32,
    cudaResViewFormatSignedBlockCompressed6H = 33,
    cudaResViewFormatUnsignedBlockCompressed7 = 34,
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub enum cudaRoundMode {
    cudaRoundNearest = 0,
    cudaRoundZero = 1,
    cudaRoundPosInf = 2,
    cudaRoundMinInf = 3,
}
#[repr(i32)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub enum cudaSharedCarveout {
    cudaSharedmemCarveoutDefault = -1,
    cudaSharedmemCarveoutMaxShared = 100,
    cudaSharedmemCarveoutMaxL1 = 0,
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub enum cudaSharedMemConfig {
    cudaSharedMemBankSizeDefault = 0,
    cudaSharedMemBankSizeFourByte = 1,
    cudaSharedMemBankSizeEightByte = 2,
}
#[cfg(any(
    feature = "cuda-11040",
    feature = "cuda-11050",
    feature = "cuda-11060",
    feature = "cuda-11070"
))]
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub enum cudaStreamAttrID {
    cudaStreamAttributeAccessPolicyWindow = 1,
    cudaStreamAttributeSynchronizationPolicy = 3,
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub enum cudaStreamCaptureMode {
    cudaStreamCaptureModeGlobal = 0,
    cudaStreamCaptureModeThreadLocal = 1,
    cudaStreamCaptureModeRelaxed = 2,
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub enum cudaStreamCaptureStatus {
    cudaStreamCaptureStatusNone = 0,
    cudaStreamCaptureStatusActive = 1,
    cudaStreamCaptureStatusInvalidated = 2,
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub enum cudaStreamUpdateCaptureDependenciesFlags {
    cudaStreamAddCaptureDependencies = 0,
    cudaStreamSetCaptureDependencies = 1,
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub enum cudaSurfaceBoundaryMode {
    cudaBoundaryModeZero = 0,
    cudaBoundaryModeClamp = 1,
    cudaBoundaryModeTrap = 2,
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub enum cudaSurfaceFormatMode {
    cudaFormatModeForced = 0,
    cudaFormatModeAuto = 1,
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub enum cudaSynchronizationPolicy {
    cudaSyncPolicyAuto = 1,
    cudaSyncPolicySpin = 2,
    cudaSyncPolicyYield = 3,
    cudaSyncPolicyBlockingSync = 4,
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub enum cudaTextureAddressMode {
    cudaAddressModeWrap = 0,
    cudaAddressModeClamp = 1,
    cudaAddressModeMirror = 2,
    cudaAddressModeBorder = 3,
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub enum cudaTextureFilterMode {
    cudaFilterModePoint = 0,
    cudaFilterModeLinear = 1,
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub enum cudaTextureReadMode {
    cudaReadModeElementType = 0,
    cudaReadModeNormalizedFloat = 1,
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub enum cudaUserObjectFlags {
    cudaUserObjectNoDestructorSync = 1,
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub enum cudaUserObjectRetainFlags {
    cudaGraphUserObjectMove = 1,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct CUevent_st {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct CUexternalMemory_st {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct CUexternalSemaphore_st {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct CUfunc_st {
    _unused: [u8; 0],
}
#[cfg(any(
    feature = "cuda-12040",
    feature = "cuda-12050",
    feature = "cuda-12060",
    feature = "cuda-12080",
    feature = "cuda-12090",
    feature = "cuda-13000"
))]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct CUgraphDeviceUpdatableNode_st {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct CUgraphExec_st {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct CUgraphNode_st {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct CUgraph_st {
    _unused: [u8; 0],
}
#[cfg(any(
    feature = "cuda-12010",
    feature = "cuda-12020",
    feature = "cuda-12030",
    feature = "cuda-12040",
    feature = "cuda-12050",
    feature = "cuda-12060",
    feature = "cuda-12080",
    feature = "cuda-12090",
    feature = "cuda-13000"
))]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct CUkern_st {
    _unused: [u8; 0],
}
#[cfg(any(feature = "cuda-12080", feature = "cuda-12090", feature = "cuda-13000"))]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct CUlib_st {
    _unused: [u8; 0],
}
#[cfg(any(feature = "cuda-13000"))]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct CUlogsCallbackEntry_st {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct CUmemPoolHandle_st {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct CUstream_st {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct CUuserObject_st {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct CUuuid_st {
    pub bytes: [::core::ffi::c_char; 16usize],
}
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialOrd, PartialEq)]
pub struct cudaAccessPolicyWindow {
    pub base_ptr: *mut ::core::ffi::c_void,
    pub num_bytes: usize,
    pub hitRatio: f32,
    pub hitProp: cudaAccessProperty,
    pub missProp: cudaAccessProperty,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct cudaArray {
    _unused: [u8; 0],
}
#[cfg(any(
    feature = "cuda-11060",
    feature = "cuda-11070",
    feature = "cuda-11080",
    feature = "cuda-12000",
    feature = "cuda-12010",
    feature = "cuda-12020",
    feature = "cuda-12030",
    feature = "cuda-12040",
    feature = "cuda-12050",
    feature = "cuda-12060",
    feature = "cuda-12080",
    feature = "cuda-12090",
    feature = "cuda-13000"
))]
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct cudaArrayMemoryRequirements {
    pub size: usize,
    pub alignment: usize,
    pub reserved: [::core::ffi::c_uint; 4usize],
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct cudaArraySparseProperties {
    pub tileExtent: cudaArraySparseProperties__bindgen_ty_1,
    pub miptailFirstLevel: ::core::ffi::c_uint,
    pub miptailSize: ::core::ffi::c_ulonglong,
    pub flags: ::core::ffi::c_uint,
    pub reserved: [::core::ffi::c_uint; 4usize],
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct cudaArraySparseProperties__bindgen_ty_1 {
    pub width: ::core::ffi::c_uint,
    pub height: ::core::ffi::c_uint,
    pub depth: ::core::ffi::c_uint,
}
#[cfg(any(
    feature = "cuda-12040",
    feature = "cuda-12050",
    feature = "cuda-12060",
    feature = "cuda-12080",
    feature = "cuda-12090",
    feature = "cuda-13000"
))]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct cudaAsyncCallbackEntry {
    _unused: [u8; 0],
}
#[cfg(any(
    feature = "cuda-12040",
    feature = "cuda-12050",
    feature = "cuda-12060",
    feature = "cuda-12080",
    feature = "cuda-12090",
    feature = "cuda-13000"
))]
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cudaAsyncNotificationInfo {
    pub type_: cudaAsyncNotificationType,
    pub info: cudaAsyncNotificationInfo__bindgen_ty_1,
}
#[cfg(any(
    feature = "cuda-12040",
    feature = "cuda-12050",
    feature = "cuda-12060",
    feature = "cuda-12080",
    feature = "cuda-12090",
    feature = "cuda-13000"
))]
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct cudaAsyncNotificationInfo__bindgen_ty_1__bindgen_ty_1 {
    pub bytesOverBudget: ::core::ffi::c_ulonglong,
}
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct cudaChannelFormatDesc {
    pub x: ::core::ffi::c_int,
    pub y: ::core::ffi::c_int,
    pub z: ::core::ffi::c_int,
    pub w: ::core::ffi::c_int,
    pub f: cudaChannelFormatKind,
}
#[cfg(any(
    feature = "cuda-12020",
    feature = "cuda-12030",
    feature = "cuda-12040",
    feature = "cuda-12050",
    feature = "cuda-12060",
    feature = "cuda-12080"
))]
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct cudaChildGraphNodeParams {
    pub graph: cudaGraph_t,
}
#[cfg(any(feature = "cuda-12090", feature = "cuda-13000"))]
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct cudaChildGraphNodeParams {
    pub graph: cudaGraph_t,
    pub ownership: cudaGraphChildGraphNodeOwnership,
}
#[cfg(any(
    feature = "cuda-12030",
    feature = "cuda-12040",
    feature = "cuda-12050",
    feature = "cuda-12060",
    feature = "cuda-12080",
    feature = "cuda-12090",
    feature = "cuda-13000"
))]
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct cudaConditionalNodeParams {
    pub handle: cudaGraphConditionalHandle,
    pub type_: cudaGraphConditionalNodeType,
    pub size: ::core::ffi::c_uint,
    pub phGraph_out: *mut cudaGraph_t,
}
#[cfg(any(
    feature = "cuda-11040",
    feature = "cuda-11050",
    feature = "cuda-11060",
    feature = "cuda-11070",
    feature = "cuda-11080"
))]
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct cudaDeviceProp {
    pub name: [::core::ffi::c_char; 256usize],
    pub uuid: cudaUUID_t,
    pub luid: [::core::ffi::c_char; 8usize],
    pub luidDeviceNodeMask: ::core::ffi::c_uint,
    pub totalGlobalMem: usize,
    pub sharedMemPerBlock: usize,
    pub regsPerBlock: ::core::ffi::c_int,
    pub warpSize: ::core::ffi::c_int,
    pub memPitch: usize,
    pub maxThreadsPerBlock: ::core::ffi::c_int,
    pub maxThreadsDim: [::core::ffi::c_int; 3usize],
    pub maxGridSize: [::core::ffi::c_int; 3usize],
    pub clockRate: ::core::ffi::c_int,
    pub totalConstMem: usize,
    pub major: ::core::ffi::c_int,
    pub minor: ::core::ffi::c_int,
    pub textureAlignment: usize,
    pub texturePitchAlignment: usize,
    pub deviceOverlap: ::core::ffi::c_int,
    pub multiProcessorCount: ::core::ffi::c_int,
    pub kernelExecTimeoutEnabled: ::core::ffi::c_int,
    pub integrated: ::core::ffi::c_int,
    pub canMapHostMemory: ::core::ffi::c_int,
    pub computeMode: ::core::ffi::c_int,
    pub maxTexture1D: ::core::ffi::c_int,
    pub maxTexture1DMipmap: ::core::ffi::c_int,
    pub maxTexture1DLinear: ::core::ffi::c_int,
    pub maxTexture2D: [::core::ffi::c_int; 2usize],
    pub maxTexture2DMipmap: [::core::ffi::c_int; 2usize],
    pub maxTexture2DLinear: [::core::ffi::c_int; 3usize],
    pub maxTexture2DGather: [::core::ffi::c_int; 2usize],
    pub maxTexture3D: [::core::ffi::c_int; 3usize],
    pub maxTexture3DAlt: [::core::ffi::c_int; 3usize],
    pub maxTextureCubemap: ::core::ffi::c_int,
    pub maxTexture1DLayered: [::core::ffi::c_int; 2usize],
    pub maxTexture2DLayered: [::core::ffi::c_int; 3usize],
    pub maxTextureCubemapLayered: [::core::ffi::c_int; 2usize],
    pub maxSurface1D: ::core::ffi::c_int,
    pub maxSurface2D: [::core::ffi::c_int; 2usize],
    pub maxSurface3D: [::core::ffi::c_int; 3usize],
    pub maxSurface1DLayered: [::core::ffi::c_int; 2usize],
    pub maxSurface2DLayered: [::core::ffi::c_int; 3usize],
    pub maxSurfaceCubemap: ::core::ffi::c_int,
    pub maxSurfaceCubemapLayered: [::core::ffi::c_int; 2usize],
    pub surfaceAlignment: usize,
    pub concurrentKernels: ::core::ffi::c_int,
    pub ECCEnabled: ::core::ffi::c_int,
    pub pciBusID: ::core::ffi::c_int,
    pub pciDeviceID: ::core::ffi::c_int,
    pub pciDomainID: ::core::ffi::c_int,
    pub tccDriver: ::core::ffi::c_int,
    pub asyncEngineCount: ::core::ffi::c_int,
    pub unifiedAddressing: ::core::ffi::c_int,
    pub memoryClockRate: ::core::ffi::c_int,
    pub memoryBusWidth: ::core::ffi::c_int,
    pub l2CacheSize: ::core::ffi::c_int,
    pub persistingL2CacheMaxSize: ::core::ffi::c_int,
    pub maxThreadsPerMultiProcessor: ::core::ffi::c_int,
    pub streamPrioritiesSupported: ::core::ffi::c_int,
    pub globalL1CacheSupported: ::core::ffi::c_int,
    pub localL1CacheSupported: ::core::ffi::c_int,
    pub sharedMemPerMultiprocessor: usize,
    pub regsPerMultiprocessor: ::core::ffi::c_int,
    pub managedMemory: ::core::ffi::c_int,
    pub isMultiGpuBoard: ::core::ffi::c_int,
    pub multiGpuBoardGroupID: ::core::ffi::c_int,
    pub hostNativeAtomicSupported: ::core::ffi::c_int,
    pub singleToDoublePrecisionPerfRatio: ::core::ffi::c_int,
    pub pageableMemoryAccess: ::core::ffi::c_int,
    pub concurrentManagedAccess: ::core::ffi::c_int,
    pub computePreemptionSupported: ::core::ffi::c_int,
    pub canUseHostPointerForRegisteredMem: ::core::ffi::c_int,
    pub cooperativeLaunch: ::core::ffi::c_int,
    pub cooperativeMultiDeviceLaunch: ::core::ffi::c_int,
    pub sharedMemPerBlockOptin: usize,
    pub pageableMemoryAccessUsesHostPageTables: ::core::ffi::c_int,
    pub directManagedMemAccessFromHost: ::core::ffi::c_int,
    pub maxBlocksPerMultiProcessor: ::core::ffi::c_int,
    pub accessPolicyMaxWindowSize: ::core::ffi::c_int,
    pub reservedSharedMemPerBlock: usize,
}
#[cfg(any(feature = "cuda-12000"))]
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct cudaDeviceProp {
    pub name: [::core::ffi::c_char; 256usize],
    pub uuid: cudaUUID_t,
    pub luid: [::core::ffi::c_char; 8usize],
    pub luidDeviceNodeMask: ::core::ffi::c_uint,
    pub totalGlobalMem: usize,
    pub sharedMemPerBlock: usize,
    pub regsPerBlock: ::core::ffi::c_int,
    pub warpSize: ::core::ffi::c_int,
    pub memPitch: usize,
    pub maxThreadsPerBlock: ::core::ffi::c_int,
    pub maxThreadsDim: [::core::ffi::c_int; 3usize],
    pub maxGridSize: [::core::ffi::c_int; 3usize],
    pub clockRate: ::core::ffi::c_int,
    pub totalConstMem: usize,
    pub major: ::core::ffi::c_int,
    pub minor: ::core::ffi::c_int,
    pub textureAlignment: usize,
    pub texturePitchAlignment: usize,
    pub deviceOverlap: ::core::ffi::c_int,
    pub multiProcessorCount: ::core::ffi::c_int,
    pub kernelExecTimeoutEnabled: ::core::ffi::c_int,
    pub integrated: ::core::ffi::c_int,
    pub canMapHostMemory: ::core::ffi::c_int,
    pub computeMode: ::core::ffi::c_int,
    pub maxTexture1D: ::core::ffi::c_int,
    pub maxTexture1DMipmap: ::core::ffi::c_int,
    pub maxTexture1DLinear: ::core::ffi::c_int,
    pub maxTexture2D: [::core::ffi::c_int; 2usize],
    pub maxTexture2DMipmap: [::core::ffi::c_int; 2usize],
    pub maxTexture2DLinear: [::core::ffi::c_int; 3usize],
    pub maxTexture2DGather: [::core::ffi::c_int; 2usize],
    pub maxTexture3D: [::core::ffi::c_int; 3usize],
    pub maxTexture3DAlt: [::core::ffi::c_int; 3usize],
    pub maxTextureCubemap: ::core::ffi::c_int,
    pub maxTexture1DLayered: [::core::ffi::c_int; 2usize],
    pub maxTexture2DLayered: [::core::ffi::c_int; 3usize],
    pub maxTextureCubemapLayered: [::core::ffi::c_int; 2usize],
    pub maxSurface1D: ::core::ffi::c_int,
    pub maxSurface2D: [::core::ffi::c_int; 2usize],
    pub maxSurface3D: [::core::ffi::c_int; 3usize],
    pub maxSurface1DLayered: [::core::ffi::c_int; 2usize],
    pub maxSurface2DLayered: [::core::ffi::c_int; 3usize],
    pub maxSurfaceCubemap: ::core::ffi::c_int,
    pub maxSurfaceCubemapLayered: [::core::ffi::c_int; 2usize],
    pub surfaceAlignment: usize,
    pub concurrentKernels: ::core::ffi::c_int,
    pub ECCEnabled: ::core::ffi::c_int,
    pub pciBusID: ::core::ffi::c_int,
    pub pciDeviceID: ::core::ffi::c_int,
    pub pciDomainID: ::core::ffi::c_int,
    pub tccDriver: ::core::ffi::c_int,
    pub asyncEngineCount: ::core::ffi::c_int,
    pub unifiedAddressing: ::core::ffi::c_int,
    pub memoryClockRate: ::core::ffi::c_int,
    pub memoryBusWidth: ::core::ffi::c_int,
    pub l2CacheSize: ::core::ffi::c_int,
    pub persistingL2CacheMaxSize: ::core::ffi::c_int,
    pub maxThreadsPerMultiProcessor: ::core::ffi::c_int,
    pub streamPrioritiesSupported: ::core::ffi::c_int,
    pub globalL1CacheSupported: ::core::ffi::c_int,
    pub localL1CacheSupported: ::core::ffi::c_int,
    pub sharedMemPerMultiprocessor: usize,
    pub regsPerMultiprocessor: ::core::ffi::c_int,
    pub managedMemory: ::core::ffi::c_int,
    pub isMultiGpuBoard: ::core::ffi::c_int,
    pub multiGpuBoardGroupID: ::core::ffi::c_int,
    pub hostNativeAtomicSupported: ::core::ffi::c_int,
    pub singleToDoublePrecisionPerfRatio: ::core::ffi::c_int,
    pub pageableMemoryAccess: ::core::ffi::c_int,
    pub concurrentManagedAccess: ::core::ffi::c_int,
    pub computePreemptionSupported: ::core::ffi::c_int,
    pub canUseHostPointerForRegisteredMem: ::core::ffi::c_int,
    pub cooperativeLaunch: ::core::ffi::c_int,
    pub cooperativeMultiDeviceLaunch: ::core::ffi::c_int,
    pub sharedMemPerBlockOptin: usize,
    pub pageableMemoryAccessUsesHostPageTables: ::core::ffi::c_int,
    pub directManagedMemAccessFromHost: ::core::ffi::c_int,
    pub maxBlocksPerMultiProcessor: ::core::ffi::c_int,
    pub accessPolicyMaxWindowSize: ::core::ffi::c_int,
    pub reservedSharedMemPerBlock: usize,
    pub hostRegisterSupported: ::core::ffi::c_int,
    pub sparseCudaArraySupported: ::core::ffi::c_int,
    pub hostRegisterReadOnlySupported: ::core::ffi::c_int,
    pub timelineSemaphoreInteropSupported: ::core::ffi::c_int,
    pub memoryPoolsSupported: ::core::ffi::c_int,
    pub gpuDirectRDMASupported: ::core::ffi::c_int,
    pub gpuDirectRDMAFlushWritesOptions: ::core::ffi::c_uint,
    pub gpuDirectRDMAWritesOrdering: ::core::ffi::c_int,
    pub memoryPoolSupportedHandleTypes: ::core::ffi::c_uint,
    pub deferredMappingCudaArraySupported: ::core::ffi::c_int,
    pub ipcEventSupported: ::core::ffi::c_int,
    pub clusterLaunch: ::core::ffi::c_int,
    pub unifiedFunctionPointers: ::core::ffi::c_int,
    pub reserved: [::core::ffi::c_int; 63usize],
}
#[cfg(any(feature = "cuda-12010", feature = "cuda-12020"))]
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct cudaDeviceProp {
    pub name: [::core::ffi::c_char; 256usize],
    pub uuid: cudaUUID_t,
    pub luid: [::core::ffi::c_char; 8usize],
    pub luidDeviceNodeMask: ::core::ffi::c_uint,
    pub totalGlobalMem: usize,
    pub sharedMemPerBlock: usize,
    pub regsPerBlock: ::core::ffi::c_int,
    pub warpSize: ::core::ffi::c_int,
    pub memPitch: usize,
    pub maxThreadsPerBlock: ::core::ffi::c_int,
    pub maxThreadsDim: [::core::ffi::c_int; 3usize],
    pub maxGridSize: [::core::ffi::c_int; 3usize],
    pub clockRate: ::core::ffi::c_int,
    pub totalConstMem: usize,
    pub major: ::core::ffi::c_int,
    pub minor: ::core::ffi::c_int,
    pub textureAlignment: usize,
    pub texturePitchAlignment: usize,
    pub deviceOverlap: ::core::ffi::c_int,
    pub multiProcessorCount: ::core::ffi::c_int,
    pub kernelExecTimeoutEnabled: ::core::ffi::c_int,
    pub integrated: ::core::ffi::c_int,
    pub canMapHostMemory: ::core::ffi::c_int,
    pub computeMode: ::core::ffi::c_int,
    pub maxTexture1D: ::core::ffi::c_int,
    pub maxTexture1DMipmap: ::core::ffi::c_int,
    pub maxTexture1DLinear: ::core::ffi::c_int,
    pub maxTexture2D: [::core::ffi::c_int; 2usize],
    pub maxTexture2DMipmap: [::core::ffi::c_int; 2usize],
    pub maxTexture2DLinear: [::core::ffi::c_int; 3usize],
    pub maxTexture2DGather: [::core::ffi::c_int; 2usize],
    pub maxTexture3D: [::core::ffi::c_int; 3usize],
    pub maxTexture3DAlt: [::core::ffi::c_int; 3usize],
    pub maxTextureCubemap: ::core::ffi::c_int,
    pub maxTexture1DLayered: [::core::ffi::c_int; 2usize],
    pub maxTexture2DLayered: [::core::ffi::c_int; 3usize],
    pub maxTextureCubemapLayered: [::core::ffi::c_int; 2usize],
    pub maxSurface1D: ::core::ffi::c_int,
    pub maxSurface2D: [::core::ffi::c_int; 2usize],
    pub maxSurface3D: [::core::ffi::c_int; 3usize],
    pub maxSurface1DLayered: [::core::ffi::c_int; 2usize],
    pub maxSurface2DLayered: [::core::ffi::c_int; 3usize],
    pub maxSurfaceCubemap: ::core::ffi::c_int,
    pub maxSurfaceCubemapLayered: [::core::ffi::c_int; 2usize],
    pub surfaceAlignment: usize,
    pub concurrentKernels: ::core::ffi::c_int,
    pub ECCEnabled: ::core::ffi::c_int,
    pub pciBusID: ::core::ffi::c_int,
    pub pciDeviceID: ::core::ffi::c_int,
    pub pciDomainID: ::core::ffi::c_int,
    pub tccDriver: ::core::ffi::c_int,
    pub asyncEngineCount: ::core::ffi::c_int,
    pub unifiedAddressing: ::core::ffi::c_int,
    pub memoryClockRate: ::core::ffi::c_int,
    pub memoryBusWidth: ::core::ffi::c_int,
    pub l2CacheSize: ::core::ffi::c_int,
    pub persistingL2CacheMaxSize: ::core::ffi::c_int,
    pub maxThreadsPerMultiProcessor: ::core::ffi::c_int,
    pub streamPrioritiesSupported: ::core::ffi::c_int,
    pub globalL1CacheSupported: ::core::ffi::c_int,
    pub localL1CacheSupported: ::core::ffi::c_int,
    pub sharedMemPerMultiprocessor: usize,
    pub regsPerMultiprocessor: ::core::ffi::c_int,
    pub managedMemory: ::core::ffi::c_int,
    pub isMultiGpuBoard: ::core::ffi::c_int,
    pub multiGpuBoardGroupID: ::core::ffi::c_int,
    pub hostNativeAtomicSupported: ::core::ffi::c_int,
    pub singleToDoublePrecisionPerfRatio: ::core::ffi::c_int,
    pub pageableMemoryAccess: ::core::ffi::c_int,
    pub concurrentManagedAccess: ::core::ffi::c_int,
    pub computePreemptionSupported: ::core::ffi::c_int,
    pub canUseHostPointerForRegisteredMem: ::core::ffi::c_int,
    pub cooperativeLaunch: ::core::ffi::c_int,
    pub cooperativeMultiDeviceLaunch: ::core::ffi::c_int,
    pub sharedMemPerBlockOptin: usize,
    pub pageableMemoryAccessUsesHostPageTables: ::core::ffi::c_int,
    pub directManagedMemAccessFromHost: ::core::ffi::c_int,
    pub maxBlocksPerMultiProcessor: ::core::ffi::c_int,
    pub accessPolicyMaxWindowSize: ::core::ffi::c_int,
    pub reservedSharedMemPerBlock: usize,
    pub hostRegisterSupported: ::core::ffi::c_int,
    pub sparseCudaArraySupported: ::core::ffi::c_int,
    pub hostRegisterReadOnlySupported: ::core::ffi::c_int,
    pub timelineSemaphoreInteropSupported: ::core::ffi::c_int,
    pub memoryPoolsSupported: ::core::ffi::c_int,
    pub gpuDirectRDMASupported: ::core::ffi::c_int,
    pub gpuDirectRDMAFlushWritesOptions: ::core::ffi::c_uint,
    pub gpuDirectRDMAWritesOrdering: ::core::ffi::c_int,
    pub memoryPoolSupportedHandleTypes: ::core::ffi::c_uint,
    pub deferredMappingCudaArraySupported: ::core::ffi::c_int,
    pub ipcEventSupported: ::core::ffi::c_int,
    pub clusterLaunch: ::core::ffi::c_int,
    pub unifiedFunctionPointers: ::core::ffi::c_int,
    pub reserved2: [::core::ffi::c_int; 2usize],
    pub reserved: [::core::ffi::c_int; 61usize],
}
#[cfg(any(
    feature = "cuda-12030",
    feature = "cuda-12040",
    feature = "cuda-12050",
    feature = "cuda-12060"
))]
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct cudaDeviceProp {
    pub name: [::core::ffi::c_char; 256usize],
    pub uuid: cudaUUID_t,
    pub luid: [::core::ffi::c_char; 8usize],
    pub luidDeviceNodeMask: ::core::ffi::c_uint,
    pub totalGlobalMem: usize,
    pub sharedMemPerBlock: usize,
    pub regsPerBlock: ::core::ffi::c_int,
    pub warpSize: ::core::ffi::c_int,
    pub memPitch: usize,
    pub maxThreadsPerBlock: ::core::ffi::c_int,
    pub maxThreadsDim: [::core::ffi::c_int; 3usize],
    pub maxGridSize: [::core::ffi::c_int; 3usize],
    pub clockRate: ::core::ffi::c_int,
    pub totalConstMem: usize,
    pub major: ::core::ffi::c_int,
    pub minor: ::core::ffi::c_int,
    pub textureAlignment: usize,
    pub texturePitchAlignment: usize,
    pub deviceOverlap: ::core::ffi::c_int,
    pub multiProcessorCount: ::core::ffi::c_int,
    pub kernelExecTimeoutEnabled: ::core::ffi::c_int,
    pub integrated: ::core::ffi::c_int,
    pub canMapHostMemory: ::core::ffi::c_int,
    pub computeMode: ::core::ffi::c_int,
    pub maxTexture1D: ::core::ffi::c_int,
    pub maxTexture1DMipmap: ::core::ffi::c_int,
    pub maxTexture1DLinear: ::core::ffi::c_int,
    pub maxTexture2D: [::core::ffi::c_int; 2usize],
    pub maxTexture2DMipmap: [::core::ffi::c_int; 2usize],
    pub maxTexture2DLinear: [::core::ffi::c_int; 3usize],
    pub maxTexture2DGather: [::core::ffi::c_int; 2usize],
    pub maxTexture3D: [::core::ffi::c_int; 3usize],
    pub maxTexture3DAlt: [::core::ffi::c_int; 3usize],
    pub maxTextureCubemap: ::core::ffi::c_int,
    pub maxTexture1DLayered: [::core::ffi::c_int; 2usize],
    pub maxTexture2DLayered: [::core::ffi::c_int; 3usize],
    pub maxTextureCubemapLayered: [::core::ffi::c_int; 2usize],
    pub maxSurface1D: ::core::ffi::c_int,
    pub maxSurface2D: [::core::ffi::c_int; 2usize],
    pub maxSurface3D: [::core::ffi::c_int; 3usize],
    pub maxSurface1DLayered: [::core::ffi::c_int; 2usize],
    pub maxSurface2DLayered: [::core::ffi::c_int; 3usize],
    pub maxSurfaceCubemap: ::core::ffi::c_int,
    pub maxSurfaceCubemapLayered: [::core::ffi::c_int; 2usize],
    pub surfaceAlignment: usize,
    pub concurrentKernels: ::core::ffi::c_int,
    pub ECCEnabled: ::core::ffi::c_int,
    pub pciBusID: ::core::ffi::c_int,
    pub pciDeviceID: ::core::ffi::c_int,
    pub pciDomainID: ::core::ffi::c_int,
    pub tccDriver: ::core::ffi::c_int,
    pub asyncEngineCount: ::core::ffi::c_int,
    pub unifiedAddressing: ::core::ffi::c_int,
    pub memoryClockRate: ::core::ffi::c_int,
    pub memoryBusWidth: ::core::ffi::c_int,
    pub l2CacheSize: ::core::ffi::c_int,
    pub persistingL2CacheMaxSize: ::core::ffi::c_int,
    pub maxThreadsPerMultiProcessor: ::core::ffi::c_int,
    pub streamPrioritiesSupported: ::core::ffi::c_int,
    pub globalL1CacheSupported: ::core::ffi::c_int,
    pub localL1CacheSupported: ::core::ffi::c_int,
    pub sharedMemPerMultiprocessor: usize,
    pub regsPerMultiprocessor: ::core::ffi::c_int,
    pub managedMemory: ::core::ffi::c_int,
    pub isMultiGpuBoard: ::core::ffi::c_int,
    pub multiGpuBoardGroupID: ::core::ffi::c_int,
    pub hostNativeAtomicSupported: ::core::ffi::c_int,
    pub singleToDoublePrecisionPerfRatio: ::core::ffi::c_int,
    pub pageableMemoryAccess: ::core::ffi::c_int,
    pub concurrentManagedAccess: ::core::ffi::c_int,
    pub computePreemptionSupported: ::core::ffi::c_int,
    pub canUseHostPointerForRegisteredMem: ::core::ffi::c_int,
    pub cooperativeLaunch: ::core::ffi::c_int,
    pub cooperativeMultiDeviceLaunch: ::core::ffi::c_int,
    pub sharedMemPerBlockOptin: usize,
    pub pageableMemoryAccessUsesHostPageTables: ::core::ffi::c_int,
    pub directManagedMemAccessFromHost: ::core::ffi::c_int,
    pub maxBlocksPerMultiProcessor: ::core::ffi::c_int,
    pub accessPolicyMaxWindowSize: ::core::ffi::c_int,
    pub reservedSharedMemPerBlock: usize,
    pub hostRegisterSupported: ::core::ffi::c_int,
    pub sparseCudaArraySupported: ::core::ffi::c_int,
    pub hostRegisterReadOnlySupported: ::core::ffi::c_int,
    pub timelineSemaphoreInteropSupported: ::core::ffi::c_int,
    pub memoryPoolsSupported: ::core::ffi::c_int,
    pub gpuDirectRDMASupported: ::core::ffi::c_int,
    pub gpuDirectRDMAFlushWritesOptions: ::core::ffi::c_uint,
    pub gpuDirectRDMAWritesOrdering: ::core::ffi::c_int,
    pub memoryPoolSupportedHandleTypes: ::core::ffi::c_uint,
    pub deferredMappingCudaArraySupported: ::core::ffi::c_int,
    pub ipcEventSupported: ::core::ffi::c_int,
    pub clusterLaunch: ::core::ffi::c_int,
    pub unifiedFunctionPointers: ::core::ffi::c_int,
    pub reserved2: [::core::ffi::c_int; 2usize],
    pub reserved1: [::core::ffi::c_int; 1usize],
    pub reserved: [::core::ffi::c_int; 60usize],
}
#[cfg(any(feature = "cuda-12080", feature = "cuda-12090"))]
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct cudaDeviceProp {
    pub name: [::core::ffi::c_char; 256usize],
    pub uuid: cudaUUID_t,
    pub luid: [::core::ffi::c_char; 8usize],
    pub luidDeviceNodeMask: ::core::ffi::c_uint,
    pub totalGlobalMem: usize,
    pub sharedMemPerBlock: usize,
    pub regsPerBlock: ::core::ffi::c_int,
    pub warpSize: ::core::ffi::c_int,
    pub memPitch: usize,
    pub maxThreadsPerBlock: ::core::ffi::c_int,
    pub maxThreadsDim: [::core::ffi::c_int; 3usize],
    pub maxGridSize: [::core::ffi::c_int; 3usize],
    pub clockRate: ::core::ffi::c_int,
    pub totalConstMem: usize,
    pub major: ::core::ffi::c_int,
    pub minor: ::core::ffi::c_int,
    pub textureAlignment: usize,
    pub texturePitchAlignment: usize,
    pub deviceOverlap: ::core::ffi::c_int,
    pub multiProcessorCount: ::core::ffi::c_int,
    pub kernelExecTimeoutEnabled: ::core::ffi::c_int,
    pub integrated: ::core::ffi::c_int,
    pub canMapHostMemory: ::core::ffi::c_int,
    pub computeMode: ::core::ffi::c_int,
    pub maxTexture1D: ::core::ffi::c_int,
    pub maxTexture1DMipmap: ::core::ffi::c_int,
    pub maxTexture1DLinear: ::core::ffi::c_int,
    pub maxTexture2D: [::core::ffi::c_int; 2usize],
    pub maxTexture2DMipmap: [::core::ffi::c_int; 2usize],
    pub maxTexture2DLinear: [::core::ffi::c_int; 3usize],
    pub maxTexture2DGather: [::core::ffi::c_int; 2usize],
    pub maxTexture3D: [::core::ffi::c_int; 3usize],
    pub maxTexture3DAlt: [::core::ffi::c_int; 3usize],
    pub maxTextureCubemap: ::core::ffi::c_int,
    pub maxTexture1DLayered: [::core::ffi::c_int; 2usize],
    pub maxTexture2DLayered: [::core::ffi::c_int; 3usize],
    pub maxTextureCubemapLayered: [::core::ffi::c_int; 2usize],
    pub maxSurface1D: ::core::ffi::c_int,
    pub maxSurface2D: [::core::ffi::c_int; 2usize],
    pub maxSurface3D: [::core::ffi::c_int; 3usize],
    pub maxSurface1DLayered: [::core::ffi::c_int; 2usize],
    pub maxSurface2DLayered: [::core::ffi::c_int; 3usize],
    pub maxSurfaceCubemap: ::core::ffi::c_int,
    pub maxSurfaceCubemapLayered: [::core::ffi::c_int; 2usize],
    pub surfaceAlignment: usize,
    pub concurrentKernels: ::core::ffi::c_int,
    pub ECCEnabled: ::core::ffi::c_int,
    pub pciBusID: ::core::ffi::c_int,
    pub pciDeviceID: ::core::ffi::c_int,
    pub pciDomainID: ::core::ffi::c_int,
    pub tccDriver: ::core::ffi::c_int,
    pub asyncEngineCount: ::core::ffi::c_int,
    pub unifiedAddressing: ::core::ffi::c_int,
    pub memoryClockRate: ::core::ffi::c_int,
    pub memoryBusWidth: ::core::ffi::c_int,
    pub l2CacheSize: ::core::ffi::c_int,
    pub persistingL2CacheMaxSize: ::core::ffi::c_int,
    pub maxThreadsPerMultiProcessor: ::core::ffi::c_int,
    pub streamPrioritiesSupported: ::core::ffi::c_int,
    pub globalL1CacheSupported: ::core::ffi::c_int,
    pub localL1CacheSupported: ::core::ffi::c_int,
    pub sharedMemPerMultiprocessor: usize,
    pub regsPerMultiprocessor: ::core::ffi::c_int,
    pub managedMemory: ::core::ffi::c_int,
    pub isMultiGpuBoard: ::core::ffi::c_int,
    pub multiGpuBoardGroupID: ::core::ffi::c_int,
    pub hostNativeAtomicSupported: ::core::ffi::c_int,
    pub singleToDoublePrecisionPerfRatio: ::core::ffi::c_int,
    pub pageableMemoryAccess: ::core::ffi::c_int,
    pub concurrentManagedAccess: ::core::ffi::c_int,
    pub computePreemptionSupported: ::core::ffi::c_int,
    pub canUseHostPointerForRegisteredMem: ::core::ffi::c_int,
    pub cooperativeLaunch: ::core::ffi::c_int,
    pub cooperativeMultiDeviceLaunch: ::core::ffi::c_int,
    pub sharedMemPerBlockOptin: usize,
    pub pageableMemoryAccessUsesHostPageTables: ::core::ffi::c_int,
    pub directManagedMemAccessFromHost: ::core::ffi::c_int,
    pub maxBlocksPerMultiProcessor: ::core::ffi::c_int,
    pub accessPolicyMaxWindowSize: ::core::ffi::c_int,
    pub reservedSharedMemPerBlock: usize,
    pub hostRegisterSupported: ::core::ffi::c_int,
    pub sparseCudaArraySupported: ::core::ffi::c_int,
    pub hostRegisterReadOnlySupported: ::core::ffi::c_int,
    pub timelineSemaphoreInteropSupported: ::core::ffi::c_int,
    pub memoryPoolsSupported: ::core::ffi::c_int,
    pub gpuDirectRDMASupported: ::core::ffi::c_int,
    pub gpuDirectRDMAFlushWritesOptions: ::core::ffi::c_uint,
    pub gpuDirectRDMAWritesOrdering: ::core::ffi::c_int,
    pub memoryPoolSupportedHandleTypes: ::core::ffi::c_uint,
    pub deferredMappingCudaArraySupported: ::core::ffi::c_int,
    pub ipcEventSupported: ::core::ffi::c_int,
    pub clusterLaunch: ::core::ffi::c_int,
    pub unifiedFunctionPointers: ::core::ffi::c_int,
    pub reserved: [::core::ffi::c_int; 63usize],
}
#[cfg(any(feature = "cuda-13000"))]
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct cudaDeviceProp {
    pub name: [::core::ffi::c_char; 256usize],
    pub uuid: cudaUUID_t,
    pub luid: [::core::ffi::c_char; 8usize],
    pub luidDeviceNodeMask: ::core::ffi::c_uint,
    pub totalGlobalMem: usize,
    pub sharedMemPerBlock: usize,
    pub regsPerBlock: ::core::ffi::c_int,
    pub warpSize: ::core::ffi::c_int,
    pub memPitch: usize,
    pub maxThreadsPerBlock: ::core::ffi::c_int,
    pub maxThreadsDim: [::core::ffi::c_int; 3usize],
    pub maxGridSize: [::core::ffi::c_int; 3usize],
    pub totalConstMem: usize,
    pub major: ::core::ffi::c_int,
    pub minor: ::core::ffi::c_int,
    pub textureAlignment: usize,
    pub texturePitchAlignment: usize,
    pub multiProcessorCount: ::core::ffi::c_int,
    pub integrated: ::core::ffi::c_int,
    pub canMapHostMemory: ::core::ffi::c_int,
    pub maxTexture1D: ::core::ffi::c_int,
    pub maxTexture1DMipmap: ::core::ffi::c_int,
    pub maxTexture2D: [::core::ffi::c_int; 2usize],
    pub maxTexture2DMipmap: [::core::ffi::c_int; 2usize],
    pub maxTexture2DLinear: [::core::ffi::c_int; 3usize],
    pub maxTexture2DGather: [::core::ffi::c_int; 2usize],
    pub maxTexture3D: [::core::ffi::c_int; 3usize],
    pub maxTexture3DAlt: [::core::ffi::c_int; 3usize],
    pub maxTextureCubemap: ::core::ffi::c_int,
    pub maxTexture1DLayered: [::core::ffi::c_int; 2usize],
    pub maxTexture2DLayered: [::core::ffi::c_int; 3usize],
    pub maxTextureCubemapLayered: [::core::ffi::c_int; 2usize],
    pub maxSurface1D: ::core::ffi::c_int,
    pub maxSurface2D: [::core::ffi::c_int; 2usize],
    pub maxSurface3D: [::core::ffi::c_int; 3usize],
    pub maxSurface1DLayered: [::core::ffi::c_int; 2usize],
    pub maxSurface2DLayered: [::core::ffi::c_int; 3usize],
    pub maxSurfaceCubemap: ::core::ffi::c_int,
    pub maxSurfaceCubemapLayered: [::core::ffi::c_int; 2usize],
    pub surfaceAlignment: usize,
    pub concurrentKernels: ::core::ffi::c_int,
    pub ECCEnabled: ::core::ffi::c_int,
    pub pciBusID: ::core::ffi::c_int,
    pub pciDeviceID: ::core::ffi::c_int,
    pub pciDomainID: ::core::ffi::c_int,
    pub tccDriver: ::core::ffi::c_int,
    pub asyncEngineCount: ::core::ffi::c_int,
    pub unifiedAddressing: ::core::ffi::c_int,
    pub memoryBusWidth: ::core::ffi::c_int,
    pub l2CacheSize: ::core::ffi::c_int,
    pub persistingL2CacheMaxSize: ::core::ffi::c_int,
    pub maxThreadsPerMultiProcessor: ::core::ffi::c_int,
    pub streamPrioritiesSupported: ::core::ffi::c_int,
    pub globalL1CacheSupported: ::core::ffi::c_int,
    pub localL1CacheSupported: ::core::ffi::c_int,
    pub sharedMemPerMultiprocessor: usize,
    pub regsPerMultiprocessor: ::core::ffi::c_int,
    pub managedMemory: ::core::ffi::c_int,
    pub isMultiGpuBoard: ::core::ffi::c_int,
    pub multiGpuBoardGroupID: ::core::ffi::c_int,
    pub hostNativeAtomicSupported: ::core::ffi::c_int,
    pub pageableMemoryAccess: ::core::ffi::c_int,
    pub concurrentManagedAccess: ::core::ffi::c_int,
    pub computePreemptionSupported: ::core::ffi::c_int,
    pub canUseHostPointerForRegisteredMem: ::core::ffi::c_int,
    pub cooperativeLaunch: ::core::ffi::c_int,
    pub sharedMemPerBlockOptin: usize,
    pub pageableMemoryAccessUsesHostPageTables: ::core::ffi::c_int,
    pub directManagedMemAccessFromHost: ::core::ffi::c_int,
    pub maxBlocksPerMultiProcessor: ::core::ffi::c_int,
    pub accessPolicyMaxWindowSize: ::core::ffi::c_int,
    pub reservedSharedMemPerBlock: usize,
    pub hostRegisterSupported: ::core::ffi::c_int,
    pub sparseCudaArraySupported: ::core::ffi::c_int,
    pub hostRegisterReadOnlySupported: ::core::ffi::c_int,
    pub timelineSemaphoreInteropSupported: ::core::ffi::c_int,
    pub memoryPoolsSupported: ::core::ffi::c_int,
    pub gpuDirectRDMASupported: ::core::ffi::c_int,
    pub gpuDirectRDMAFlushWritesOptions: ::core::ffi::c_uint,
    pub gpuDirectRDMAWritesOrdering: ::core::ffi::c_int,
    pub memoryPoolSupportedHandleTypes: ::core::ffi::c_uint,
    pub deferredMappingCudaArraySupported: ::core::ffi::c_int,
    pub ipcEventSupported: ::core::ffi::c_int,
    pub clusterLaunch: ::core::ffi::c_int,
    pub unifiedFunctionPointers: ::core::ffi::c_int,
    pub deviceNumaConfig: ::core::ffi::c_int,
    pub deviceNumaId: ::core::ffi::c_int,
    pub mpsEnabled: ::core::ffi::c_int,
    pub hostNumaId: ::core::ffi::c_int,
    pub gpuPciDeviceID: ::core::ffi::c_uint,
    pub gpuPciSubsystemID: ::core::ffi::c_uint,
    pub hostNumaMultinodeIpcSupported: ::core::ffi::c_int,
    pub reserved: [::core::ffi::c_int; 56usize],
}
#[cfg(any(
    feature = "cuda-12020",
    feature = "cuda-12030",
    feature = "cuda-12040",
    feature = "cuda-12050",
    feature = "cuda-12060",
    feature = "cuda-12080",
    feature = "cuda-12090",
    feature = "cuda-13000"
))]
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct cudaEventRecordNodeParams {
    pub event: cudaEvent_t,
}
#[cfg(any(
    feature = "cuda-12020",
    feature = "cuda-12030",
    feature = "cuda-12040",
    feature = "cuda-12050",
    feature = "cuda-12060",
    feature = "cuda-12080",
    feature = "cuda-12090",
    feature = "cuda-13000"
))]
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct cudaEventWaitNodeParams {
    pub event: cudaEvent_t,
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct cudaExtent {
    pub width: usize,
    pub height: usize,
    pub depth: usize,
}
#[cfg(any(
    feature = "cuda-11040",
    feature = "cuda-11050",
    feature = "cuda-11060",
    feature = "cuda-11070",
    feature = "cuda-11080",
    feature = "cuda-12000",
    feature = "cuda-12010",
    feature = "cuda-12020",
    feature = "cuda-12030",
    feature = "cuda-12040",
    feature = "cuda-12050",
    feature = "cuda-12060",
    feature = "cuda-12080",
    feature = "cuda-12090"
))]
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct cudaExternalMemoryBufferDesc {
    pub offset: ::core::ffi::c_ulonglong,
    pub size: ::core::ffi::c_ulonglong,
    pub flags: ::core::ffi::c_uint,
}
#[cfg(any(feature = "cuda-13000"))]
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct cudaExternalMemoryBufferDesc {
    pub offset: ::core::ffi::c_ulonglong,
    pub size: ::core::ffi::c_ulonglong,
    pub flags: ::core::ffi::c_uint,
    pub reserved: [::core::ffi::c_uint; 16usize],
}
#[cfg(any(
    feature = "cuda-11040",
    feature = "cuda-11050",
    feature = "cuda-11060",
    feature = "cuda-11070",
    feature = "cuda-11080",
    feature = "cuda-12000",
    feature = "cuda-12010",
    feature = "cuda-12020",
    feature = "cuda-12030",
    feature = "cuda-12040",
    feature = "cuda-12050",
    feature = "cuda-12060",
    feature = "cuda-12080",
    feature = "cuda-12090"
))]
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cudaExternalMemoryHandleDesc {
    pub type_: cudaExternalMemoryHandleType,
    pub handle: cudaExternalMemoryHandleDesc__bindgen_ty_1,
    pub size: ::core::ffi::c_ulonglong,
    pub flags: ::core::ffi::c_uint,
}
#[cfg(any(feature = "cuda-13000"))]
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cudaExternalMemoryHandleDesc {
    pub type_: cudaExternalMemoryHandleType,
    pub handle: cudaExternalMemoryHandleDesc__bindgen_ty_1,
    pub size: ::core::ffi::c_ulonglong,
    pub flags: ::core::ffi::c_uint,
    pub reserved: [::core::ffi::c_uint; 16usize],
}
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct cudaExternalMemoryHandleDesc__bindgen_ty_1__bindgen_ty_1 {
    pub handle: *mut ::core::ffi::c_void,
    pub name: *const ::core::ffi::c_void,
}
#[cfg(any(
    feature = "cuda-11040",
    feature = "cuda-11050",
    feature = "cuda-11060",
    feature = "cuda-11070",
    feature = "cuda-11080",
    feature = "cuda-12000",
    feature = "cuda-12010",
    feature = "cuda-12020",
    feature = "cuda-12030",
    feature = "cuda-12040",
    feature = "cuda-12050",
    feature = "cuda-12060",
    feature = "cuda-12080",
    feature = "cuda-12090"
))]
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct cudaExternalMemoryMipmappedArrayDesc {
    pub offset: ::core::ffi::c_ulonglong,
    pub formatDesc: cudaChannelFormatDesc,
    pub extent: cudaExtent,
    pub flags: ::core::ffi::c_uint,
    pub numLevels: ::core::ffi::c_uint,
}
#[cfg(any(feature = "cuda-13000"))]
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct cudaExternalMemoryMipmappedArrayDesc {
    pub offset: ::core::ffi::c_ulonglong,
    pub formatDesc: cudaChannelFormatDesc,
    pub extent: cudaExtent,
    pub flags: ::core::ffi::c_uint,
    pub numLevels: ::core::ffi::c_uint,
    pub reserved: [::core::ffi::c_uint; 16usize],
}
#[cfg(any(
    feature = "cuda-11040",
    feature = "cuda-11050",
    feature = "cuda-11060",
    feature = "cuda-11070",
    feature = "cuda-11080",
    feature = "cuda-12000",
    feature = "cuda-12010",
    feature = "cuda-12020",
    feature = "cuda-12030",
    feature = "cuda-12040",
    feature = "cuda-12050",
    feature = "cuda-12060",
    feature = "cuda-12080",
    feature = "cuda-12090"
))]
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cudaExternalSemaphoreHandleDesc {
    pub type_: cudaExternalSemaphoreHandleType,
    pub handle: cudaExternalSemaphoreHandleDesc__bindgen_ty_1,
    pub flags: ::core::ffi::c_uint,
}
#[cfg(any(feature = "cuda-13000"))]
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cudaExternalSemaphoreHandleDesc {
    pub type_: cudaExternalSemaphoreHandleType,
    pub handle: cudaExternalSemaphoreHandleDesc__bindgen_ty_1,
    pub flags: ::core::ffi::c_uint,
    pub reserved: [::core::ffi::c_uint; 16usize],
}
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct cudaExternalSemaphoreHandleDesc__bindgen_ty_1__bindgen_ty_1 {
    pub handle: *mut ::core::ffi::c_void,
    pub name: *const ::core::ffi::c_void,
}
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct cudaExternalSemaphoreSignalNodeParams {
    pub extSemArray: *mut cudaExternalSemaphore_t,
    pub paramsArray: *const cudaExternalSemaphoreSignalParams,
    pub numExtSems: ::core::ffi::c_uint,
}
#[cfg(any(
    feature = "cuda-12020",
    feature = "cuda-12030",
    feature = "cuda-12040",
    feature = "cuda-12050",
    feature = "cuda-12060",
    feature = "cuda-12080",
    feature = "cuda-12090",
    feature = "cuda-13000"
))]
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct cudaExternalSemaphoreSignalNodeParamsV2 {
    pub extSemArray: *mut cudaExternalSemaphore_t,
    pub paramsArray: *const cudaExternalSemaphoreSignalParams,
    pub numExtSems: ::core::ffi::c_uint,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cudaExternalSemaphoreSignalParams {
    pub params: cudaExternalSemaphoreSignalParams__bindgen_ty_1,
    pub flags: ::core::ffi::c_uint,
    pub reserved: [::core::ffi::c_uint; 16usize],
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cudaExternalSemaphoreSignalParams__bindgen_ty_1 {
    pub fence: cudaExternalSemaphoreSignalParams__bindgen_ty_1__bindgen_ty_1,
    pub nvSciSync: cudaExternalSemaphoreSignalParams__bindgen_ty_1__bindgen_ty_2,
    pub keyedMutex: cudaExternalSemaphoreSignalParams__bindgen_ty_1__bindgen_ty_3,
    pub reserved: [::core::ffi::c_uint; 12usize],
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct cudaExternalSemaphoreSignalParams__bindgen_ty_1__bindgen_ty_1 {
    pub value: ::core::ffi::c_ulonglong,
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct cudaExternalSemaphoreSignalParams__bindgen_ty_1__bindgen_ty_3 {
    pub key: ::core::ffi::c_ulonglong,
}
#[cfg(any(
    feature = "cuda-11050",
    feature = "cuda-11060",
    feature = "cuda-11070",
    feature = "cuda-11080",
    feature = "cuda-12000",
    feature = "cuda-12010",
    feature = "cuda-12020",
    feature = "cuda-12030",
    feature = "cuda-12040",
    feature = "cuda-12050",
    feature = "cuda-12060",
    feature = "cuda-12080",
    feature = "cuda-12090"
))]
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cudaExternalSemaphoreSignalParams_v1 {
    pub params: cudaExternalSemaphoreSignalParams_v1__bindgen_ty_1,
    pub flags: ::core::ffi::c_uint,
}
#[cfg(any(
    feature = "cuda-11050",
    feature = "cuda-11060",
    feature = "cuda-11070",
    feature = "cuda-11080",
    feature = "cuda-12000",
    feature = "cuda-12010",
    feature = "cuda-12020",
    feature = "cuda-12030",
    feature = "cuda-12040",
    feature = "cuda-12050",
    feature = "cuda-12060",
    feature = "cuda-12080",
    feature = "cuda-12090"
))]
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cudaExternalSemaphoreSignalParams_v1__bindgen_ty_1 {
    pub fence: cudaExternalSemaphoreSignalParams_v1__bindgen_ty_1__bindgen_ty_1,
    pub nvSciSync: cudaExternalSemaphoreSignalParams_v1__bindgen_ty_1__bindgen_ty_2,
    pub keyedMutex: cudaExternalSemaphoreSignalParams_v1__bindgen_ty_1__bindgen_ty_3,
}
#[cfg(any(
    feature = "cuda-11050",
    feature = "cuda-11060",
    feature = "cuda-11070",
    feature = "cuda-11080",
    feature = "cuda-12000",
    feature = "cuda-12010",
    feature = "cuda-12020",
    feature = "cuda-12030",
    feature = "cuda-12040",
    feature = "cuda-12050",
    feature = "cuda-12060",
    feature = "cuda-12080",
    feature = "cuda-12090"
))]
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct cudaExternalSemaphoreSignalParams_v1__bindgen_ty_1__bindgen_ty_1 {
    pub value: ::core::ffi::c_ulonglong,
}
#[cfg(any(
    feature = "cuda-11050",
    feature = "cuda-11060",
    feature = "cuda-11070",
    feature = "cuda-11080",
    feature = "cuda-12000",
    feature = "cuda-12010",
    feature = "cuda-12020",
    feature = "cuda-12030",
    feature = "cuda-12040",
    feature = "cuda-12050",
    feature = "cuda-12060",
    feature = "cuda-12080",
    feature = "cuda-12090"
))]
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct cudaExternalSemaphoreSignalParams_v1__bindgen_ty_1__bindgen_ty_3 {
    pub key: ::core::ffi::c_ulonglong,
}
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct cudaExternalSemaphoreWaitNodeParams {
    pub extSemArray: *mut cudaExternalSemaphore_t,
    pub paramsArray: *const cudaExternalSemaphoreWaitParams,
    pub numExtSems: ::core::ffi::c_uint,
}
#[cfg(any(
    feature = "cuda-12020",
    feature = "cuda-12030",
    feature = "cuda-12040",
    feature = "cuda-12050",
    feature = "cuda-12060",
    feature = "cuda-12080",
    feature = "cuda-12090",
    feature = "cuda-13000"
))]
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct cudaExternalSemaphoreWaitNodeParamsV2 {
    pub extSemArray: *mut cudaExternalSemaphore_t,
    pub paramsArray: *const cudaExternalSemaphoreWaitParams,
    pub numExtSems: ::core::ffi::c_uint,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cudaExternalSemaphoreWaitParams {
    pub params: cudaExternalSemaphoreWaitParams__bindgen_ty_1,
    pub flags: ::core::ffi::c_uint,
    pub reserved: [::core::ffi::c_uint; 16usize],
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cudaExternalSemaphoreWaitParams__bindgen_ty_1 {
    pub fence: cudaExternalSemaphoreWaitParams__bindgen_ty_1__bindgen_ty_1,
    pub nvSciSync: cudaExternalSemaphoreWaitParams__bindgen_ty_1__bindgen_ty_2,
    pub keyedMutex: cudaExternalSemaphoreWaitParams__bindgen_ty_1__bindgen_ty_3,
    pub reserved: [::core::ffi::c_uint; 10usize],
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct cudaExternalSemaphoreWaitParams__bindgen_ty_1__bindgen_ty_1 {
    pub value: ::core::ffi::c_ulonglong,
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct cudaExternalSemaphoreWaitParams__bindgen_ty_1__bindgen_ty_3 {
    pub key: ::core::ffi::c_ulonglong,
    pub timeoutMs: ::core::ffi::c_uint,
}
#[cfg(any(
    feature = "cuda-11050",
    feature = "cuda-11060",
    feature = "cuda-11070",
    feature = "cuda-11080",
    feature = "cuda-12000",
    feature = "cuda-12010",
    feature = "cuda-12020",
    feature = "cuda-12030",
    feature = "cuda-12040",
    feature = "cuda-12050",
    feature = "cuda-12060",
    feature = "cuda-12080",
    feature = "cuda-12090"
))]
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cudaExternalSemaphoreWaitParams_v1 {
    pub params: cudaExternalSemaphoreWaitParams_v1__bindgen_ty_1,
    pub flags: ::core::ffi::c_uint,
}
#[cfg(any(
    feature = "cuda-11050",
    feature = "cuda-11060",
    feature = "cuda-11070",
    feature = "cuda-11080",
    feature = "cuda-12000",
    feature = "cuda-12010",
    feature = "cuda-12020",
    feature = "cuda-12030",
    feature = "cuda-12040",
    feature = "cuda-12050",
    feature = "cuda-12060",
    feature = "cuda-12080",
    feature = "cuda-12090"
))]
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cudaExternalSemaphoreWaitParams_v1__bindgen_ty_1 {
    pub fence: cudaExternalSemaphoreWaitParams_v1__bindgen_ty_1__bindgen_ty_1,
    pub nvSciSync: cudaExternalSemaphoreWaitParams_v1__bindgen_ty_1__bindgen_ty_2,
    pub keyedMutex: cudaExternalSemaphoreWaitParams_v1__bindgen_ty_1__bindgen_ty_3,
}
#[cfg(any(
    feature = "cuda-11050",
    feature = "cuda-11060",
    feature = "cuda-11070",
    feature = "cuda-11080",
    feature = "cuda-12000",
    feature = "cuda-12010",
    feature = "cuda-12020",
    feature = "cuda-12030",
    feature = "cuda-12040",
    feature = "cuda-12050",
    feature = "cuda-12060",
    feature = "cuda-12080",
    feature = "cuda-12090"
))]
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct cudaExternalSemaphoreWaitParams_v1__bindgen_ty_1__bindgen_ty_1 {
    pub value: ::core::ffi::c_ulonglong,
}
#[cfg(any(
    feature = "cuda-11050",
    feature = "cuda-11060",
    feature = "cuda-11070",
    feature = "cuda-11080",
    feature = "cuda-12000",
    feature = "cuda-12010",
    feature = "cuda-12020",
    feature = "cuda-12030",
    feature = "cuda-12040",
    feature = "cuda-12050",
    feature = "cuda-12060",
    feature = "cuda-12080",
    feature = "cuda-12090"
))]
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct cudaExternalSemaphoreWaitParams_v1__bindgen_ty_1__bindgen_ty_3 {
    pub key: ::core::ffi::c_ulonglong,
    pub timeoutMs: ::core::ffi::c_uint,
}
#[cfg(any(
    feature = "cuda-11040",
    feature = "cuda-11050",
    feature = "cuda-11060",
    feature = "cuda-11070",
    feature = "cuda-11080"
))]
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct cudaFuncAttributes {
    pub sharedSizeBytes: usize,
    pub constSizeBytes: usize,
    pub localSizeBytes: usize,
    pub maxThreadsPerBlock: ::core::ffi::c_int,
    pub numRegs: ::core::ffi::c_int,
    pub ptxVersion: ::core::ffi::c_int,
    pub binaryVersion: ::core::ffi::c_int,
    pub cacheModeCA: ::core::ffi::c_int,
    pub maxDynamicSharedSizeBytes: ::core::ffi::c_int,
    pub preferredShmemCarveout: ::core::ffi::c_int,
}
#[cfg(any(
    feature = "cuda-12000",
    feature = "cuda-12010",
    feature = "cuda-12020",
    feature = "cuda-12030",
    feature = "cuda-12040",
    feature = "cuda-12050",
    feature = "cuda-12060",
    feature = "cuda-12080",
    feature = "cuda-12090",
    feature = "cuda-13000"
))]
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct cudaFuncAttributes {
    pub sharedSizeBytes: usize,
    pub constSizeBytes: usize,
    pub localSizeBytes: usize,
    pub maxThreadsPerBlock: ::core::ffi::c_int,
    pub numRegs: ::core::ffi::c_int,
    pub ptxVersion: ::core::ffi::c_int,
    pub binaryVersion: ::core::ffi::c_int,
    pub cacheModeCA: ::core::ffi::c_int,
    pub maxDynamicSharedSizeBytes: ::core::ffi::c_int,
    pub preferredShmemCarveout: ::core::ffi::c_int,
    pub clusterDimMustBeSet: ::core::ffi::c_int,
    pub requiredClusterWidth: ::core::ffi::c_int,
    pub requiredClusterHeight: ::core::ffi::c_int,
    pub requiredClusterDepth: ::core::ffi::c_int,
    pub clusterSchedulingPolicyPreference: ::core::ffi::c_int,
    pub nonPortableClusterSizeAllowed: ::core::ffi::c_int,
    pub reserved: [::core::ffi::c_int; 16usize],
}
#[cfg(any(
    feature = "cuda-12030",
    feature = "cuda-12040",
    feature = "cuda-12050",
    feature = "cuda-12060",
    feature = "cuda-12080",
    feature = "cuda-12090",
    feature = "cuda-13000"
))]
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct cudaGraphEdgeData_st {
    pub from_port: ::core::ffi::c_uchar,
    pub to_port: ::core::ffi::c_uchar,
    pub type_: ::core::ffi::c_uchar,
    pub reserved: [::core::ffi::c_uchar; 5usize],
}
#[cfg(any(
    feature = "cuda-12000",
    feature = "cuda-12010",
    feature = "cuda-12020",
    feature = "cuda-12030",
    feature = "cuda-12040",
    feature = "cuda-12050",
    feature = "cuda-12060",
    feature = "cuda-12080",
    feature = "cuda-12090",
    feature = "cuda-13000"
))]
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct cudaGraphExecUpdateResultInfo_st {
    pub result: cudaGraphExecUpdateResult,
    pub errorNode: cudaGraphNode_t,
    pub errorFromNode: cudaGraphNode_t,
}
#[cfg(any(
    feature = "cuda-12000",
    feature = "cuda-12010",
    feature = "cuda-12020",
    feature = "cuda-12030",
    feature = "cuda-12040",
    feature = "cuda-12050",
    feature = "cuda-12060",
    feature = "cuda-12080",
    feature = "cuda-12090",
    feature = "cuda-13000"
))]
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct cudaGraphInstantiateParams_st {
    pub flags: ::core::ffi::c_ulonglong,
    pub uploadStream: cudaStream_t,
    pub errNode_out: cudaGraphNode_t,
    pub result_out: cudaGraphInstantiateResult,
}
#[cfg(any(
    feature = "cuda-12040",
    feature = "cuda-12050",
    feature = "cuda-12060",
    feature = "cuda-12080",
    feature = "cuda-12090",
    feature = "cuda-13000"
))]
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cudaGraphKernelNodeUpdate {
    pub node: cudaGraphDeviceNode_t,
    pub field: cudaGraphKernelNodeField,
    pub updateData: cudaGraphKernelNodeUpdate__bindgen_ty_1,
}
#[cfg(any(
    feature = "cuda-12040",
    feature = "cuda-12050",
    feature = "cuda-12060",
    feature = "cuda-12080",
    feature = "cuda-12090",
    feature = "cuda-13000"
))]
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct cudaGraphKernelNodeUpdate__bindgen_ty_1__bindgen_ty_1 {
    pub pValue: *const ::core::ffi::c_void,
    pub offset: usize,
    pub size: usize,
}
#[cfg(any(
    feature = "cuda-12020",
    feature = "cuda-12030",
    feature = "cuda-12040",
    feature = "cuda-12050",
    feature = "cuda-12060",
    feature = "cuda-12080",
    feature = "cuda-12090",
    feature = "cuda-13000"
))]
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cudaGraphNodeParams {
    pub type_: cudaGraphNodeType,
    pub reserved0: [::core::ffi::c_int; 3usize],
    pub __bindgen_anon_1: cudaGraphNodeParams__bindgen_ty_1,
    pub reserved2: ::core::ffi::c_longlong,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct cudaGraphicsResource {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct cudaHostNodeParams {
    pub fn_: cudaHostFn_t,
    pub userData: *mut ::core::ffi::c_void,
}
#[cfg(any(
    feature = "cuda-12020",
    feature = "cuda-12030",
    feature = "cuda-12040",
    feature = "cuda-12050",
    feature = "cuda-12060",
    feature = "cuda-12080",
    feature = "cuda-12090",
    feature = "cuda-13000"
))]
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct cudaHostNodeParamsV2 {
    pub fn_: cudaHostFn_t,
    pub userData: *mut ::core::ffi::c_void,
}
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct cudaIpcEventHandle_st {
    pub reserved: [::core::ffi::c_char; 64usize],
}
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct cudaIpcMemHandle_st {
    pub reserved: [::core::ffi::c_char; 64usize],
}
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct cudaKernelNodeParams {
    pub func: *mut ::core::ffi::c_void,
    pub gridDim: dim3,
    pub blockDim: dim3,
    pub sharedMemBytes: ::core::ffi::c_uint,
    pub kernelParams: *mut *mut ::core::ffi::c_void,
    pub extra: *mut *mut ::core::ffi::c_void,
}
#[cfg(any(
    feature = "cuda-12020",
    feature = "cuda-12030",
    feature = "cuda-12040",
    feature = "cuda-12050",
    feature = "cuda-12060",
    feature = "cuda-12080",
    feature = "cuda-12090",
    feature = "cuda-13000"
))]
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct cudaKernelNodeParamsV2 {
    pub func: *mut ::core::ffi::c_void,
    pub gridDim: dim3,
    pub blockDim: dim3,
    pub sharedMemBytes: ::core::ffi::c_uint,
    pub kernelParams: *mut *mut ::core::ffi::c_void,
    pub extra: *mut *mut ::core::ffi::c_void,
}
#[cfg(any(
    feature = "cuda-11080",
    feature = "cuda-12000",
    feature = "cuda-12010",
    feature = "cuda-12020",
    feature = "cuda-12030",
    feature = "cuda-12040",
    feature = "cuda-12050",
    feature = "cuda-12060",
    feature = "cuda-12080",
    feature = "cuda-12090",
    feature = "cuda-13000"
))]
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct cudaLaunchAttributeValue__bindgen_ty_1 {
    pub x: ::core::ffi::c_uint,
    pub y: ::core::ffi::c_uint,
    pub z: ::core::ffi::c_uint,
}
#[cfg(any(
    feature = "cuda-11080",
    feature = "cuda-12000",
    feature = "cuda-12010",
    feature = "cuda-12020",
    feature = "cuda-12030",
    feature = "cuda-12040",
    feature = "cuda-12050",
    feature = "cuda-12060",
    feature = "cuda-12080",
    feature = "cuda-12090",
    feature = "cuda-13000"
))]
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct cudaLaunchAttributeValue__bindgen_ty_2 {
    pub event: cudaEvent_t,
    pub flags: ::core::ffi::c_int,
    pub triggerAtBlockStart: ::core::ffi::c_int,
}
#[cfg(any(
    feature = "cuda-12030",
    feature = "cuda-12040",
    feature = "cuda-12050",
    feature = "cuda-12060"
))]
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct cudaLaunchAttributeValue__bindgen_ty_3 {
    pub event: cudaEvent_t,
    pub flags: ::core::ffi::c_int,
}
#[cfg(any(feature = "cuda-12080", feature = "cuda-12090", feature = "cuda-13000"))]
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct cudaLaunchAttributeValue__bindgen_ty_3 {
    pub x: ::core::ffi::c_uint,
    pub y: ::core::ffi::c_uint,
    pub z: ::core::ffi::c_uint,
}
#[cfg(any(feature = "cuda-12040", feature = "cuda-12050", feature = "cuda-12060"))]
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct cudaLaunchAttributeValue__bindgen_ty_4 {
    pub deviceUpdatable: ::core::ffi::c_int,
    pub devNode: cudaGraphDeviceNode_t,
}
#[cfg(any(feature = "cuda-12080", feature = "cuda-12090", feature = "cuda-13000"))]
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct cudaLaunchAttributeValue__bindgen_ty_4 {
    pub event: cudaEvent_t,
    pub flags: ::core::ffi::c_int,
}
#[cfg(any(feature = "cuda-12080", feature = "cuda-12090", feature = "cuda-13000"))]
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct cudaLaunchAttributeValue__bindgen_ty_5 {
    pub deviceUpdatable: ::core::ffi::c_int,
    pub devNode: cudaGraphDeviceNode_t,
}
#[cfg(any(
    feature = "cuda-11080",
    feature = "cuda-12000",
    feature = "cuda-12010",
    feature = "cuda-12020",
    feature = "cuda-12030",
    feature = "cuda-12040",
    feature = "cuda-12050",
    feature = "cuda-12060",
    feature = "cuda-12080",
    feature = "cuda-12090",
    feature = "cuda-13000"
))]
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cudaLaunchAttribute_st {
    pub id: cudaLaunchAttributeID,
    pub pad: [::core::ffi::c_char; 4usize],
    pub val: cudaLaunchAttributeValue,
}
#[cfg(any(
    feature = "cuda-11080",
    feature = "cuda-12000",
    feature = "cuda-12010",
    feature = "cuda-12020",
    feature = "cuda-12030",
    feature = "cuda-12040",
    feature = "cuda-12050",
    feature = "cuda-12060",
    feature = "cuda-12080",
    feature = "cuda-12090",
    feature = "cuda-13000"
))]
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct cudaLaunchConfig_st {
    pub gridDim: dim3,
    pub blockDim: dim3,
    pub dynamicSmemBytes: usize,
    pub stream: cudaStream_t,
    pub attrs: *mut cudaLaunchAttribute,
    pub numAttrs: ::core::ffi::c_uint,
}
#[cfg(any(
    feature = "cuda-12000",
    feature = "cuda-12010",
    feature = "cuda-12020",
    feature = "cuda-12030",
    feature = "cuda-12040",
    feature = "cuda-12050",
    feature = "cuda-12060",
    feature = "cuda-12080",
    feature = "cuda-12090",
    feature = "cuda-13000"
))]
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct cudaLaunchMemSyncDomainMap_st {
    pub default_: ::core::ffi::c_uchar,
    pub remote: ::core::ffi::c_uchar,
}
#[cfg(any(
    feature = "cuda-11040",
    feature = "cuda-11050",
    feature = "cuda-11060",
    feature = "cuda-11070",
    feature = "cuda-11080",
    feature = "cuda-12000",
    feature = "cuda-12010",
    feature = "cuda-12020",
    feature = "cuda-12030",
    feature = "cuda-12040",
    feature = "cuda-12050",
    feature = "cuda-12060",
    feature = "cuda-12080",
    feature = "cuda-12090"
))]
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct cudaLaunchParams {
    pub func: *mut ::core::ffi::c_void,
    pub gridDim: dim3,
    pub blockDim: dim3,
    pub args: *mut *mut ::core::ffi::c_void,
    pub sharedMem: usize,
    pub stream: cudaStream_t,
}
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct cudaMemAccessDesc {
    pub location: cudaMemLocation,
    pub flags: cudaMemAccessFlags,
}
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct cudaMemAllocNodeParams {
    pub poolProps: cudaMemPoolProps,
    pub accessDescs: *const cudaMemAccessDesc,
    pub accessDescCount: usize,
    pub bytesize: usize,
    pub dptr: *mut ::core::ffi::c_void,
}
#[cfg(any(
    feature = "cuda-12020",
    feature = "cuda-12030",
    feature = "cuda-12040",
    feature = "cuda-12050",
    feature = "cuda-12060",
    feature = "cuda-12080",
    feature = "cuda-12090",
    feature = "cuda-13000"
))]
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct cudaMemAllocNodeParamsV2 {
    pub poolProps: cudaMemPoolProps,
    pub accessDescs: *const cudaMemAccessDesc,
    pub accessDescCount: usize,
    pub bytesize: usize,
    pub dptr: *mut ::core::ffi::c_void,
}
#[cfg(any(
    feature = "cuda-12030",
    feature = "cuda-12040",
    feature = "cuda-12050",
    feature = "cuda-12060",
    feature = "cuda-12080",
    feature = "cuda-12090",
    feature = "cuda-13000"
))]
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct cudaMemFabricHandle_st {
    pub reserved: [::core::ffi::c_char; 64usize],
}
#[cfg(any(
    feature = "cuda-12020",
    feature = "cuda-12030",
    feature = "cuda-12040",
    feature = "cuda-12050",
    feature = "cuda-12060",
    feature = "cuda-12080",
    feature = "cuda-12090",
    feature = "cuda-13000"
))]
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct cudaMemFreeNodeParams {
    pub dptr: *mut ::core::ffi::c_void,
}
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct cudaMemLocation {
    pub type_: cudaMemLocationType,
    pub id: ::core::ffi::c_int,
}
#[cfg(any(
    feature = "cuda-11040",
    feature = "cuda-11050",
    feature = "cuda-11060",
    feature = "cuda-11070",
    feature = "cuda-11080",
    feature = "cuda-12000",
    feature = "cuda-12010"
))]
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct cudaMemPoolProps {
    pub allocType: cudaMemAllocationType,
    pub handleTypes: cudaMemAllocationHandleType,
    pub location: cudaMemLocation,
    pub win32SecurityAttributes: *mut ::core::ffi::c_void,
    pub reserved: [::core::ffi::c_uchar; 64usize],
}
#[cfg(any(
    feature = "cuda-12020",
    feature = "cuda-12030",
    feature = "cuda-12040",
    feature = "cuda-12050"
))]
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct cudaMemPoolProps {
    pub allocType: cudaMemAllocationType,
    pub handleTypes: cudaMemAllocationHandleType,
    pub location: cudaMemLocation,
    pub win32SecurityAttributes: *mut ::core::ffi::c_void,
    pub maxSize: usize,
    pub reserved: [::core::ffi::c_uchar; 56usize],
}
#[cfg(any(
    feature = "cuda-12060",
    feature = "cuda-12080",
    feature = "cuda-12090",
    feature = "cuda-13000"
))]
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct cudaMemPoolProps {
    pub allocType: cudaMemAllocationType,
    pub handleTypes: cudaMemAllocationHandleType,
    pub location: cudaMemLocation,
    pub win32SecurityAttributes: *mut ::core::ffi::c_void,
    pub maxSize: usize,
    pub usage: ::core::ffi::c_ushort,
    pub reserved: [::core::ffi::c_uchar; 54usize],
}
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct cudaMemPoolPtrExportData {
    pub reserved: [::core::ffi::c_uchar; 64usize],
}
#[cfg(any(feature = "cuda-12080", feature = "cuda-12090", feature = "cuda-13000"))]
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cudaMemcpy3DBatchOp {
    pub src: cudaMemcpy3DOperand,
    pub dst: cudaMemcpy3DOperand,
    pub extent: cudaExtent,
    pub srcAccessOrder: cudaMemcpySrcAccessOrder,
    pub flags: ::core::ffi::c_uint,
}
#[cfg(any(feature = "cuda-12080", feature = "cuda-12090", feature = "cuda-13000"))]
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cudaMemcpy3DOperand {
    pub type_: cudaMemcpy3DOperandType,
    pub op: cudaMemcpy3DOperand__bindgen_ty_1,
}
#[cfg(any(feature = "cuda-12080", feature = "cuda-12090", feature = "cuda-13000"))]
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct cudaMemcpy3DOperand__bindgen_ty_1__bindgen_ty_1 {
    pub ptr: *mut ::core::ffi::c_void,
    pub rowLength: usize,
    pub layerHeight: usize,
    pub locHint: cudaMemLocation,
}
#[cfg(any(feature = "cuda-12080", feature = "cuda-12090", feature = "cuda-13000"))]
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct cudaMemcpy3DOperand__bindgen_ty_1__bindgen_ty_2 {
    pub array: cudaArray_t,
    pub offset: cudaOffset3D,
}
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct cudaMemcpy3DParms {
    pub srcArray: cudaArray_t,
    pub srcPos: cudaPos,
    pub srcPtr: cudaPitchedPtr,
    pub dstArray: cudaArray_t,
    pub dstPos: cudaPos,
    pub dstPtr: cudaPitchedPtr,
    pub extent: cudaExtent,
    pub kind: cudaMemcpyKind,
}
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct cudaMemcpy3DPeerParms {
    pub srcArray: cudaArray_t,
    pub srcPos: cudaPos,
    pub srcPtr: cudaPitchedPtr,
    pub srcDevice: ::core::ffi::c_int,
    pub dstArray: cudaArray_t,
    pub dstPos: cudaPos,
    pub dstPtr: cudaPitchedPtr,
    pub dstDevice: ::core::ffi::c_int,
    pub extent: cudaExtent,
}
#[cfg(any(feature = "cuda-12080", feature = "cuda-12090", feature = "cuda-13000"))]
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct cudaMemcpyAttributes {
    pub srcAccessOrder: cudaMemcpySrcAccessOrder,
    pub srcLocHint: cudaMemLocation,
    pub dstLocHint: cudaMemLocation,
    pub flags: ::core::ffi::c_uint,
}
#[cfg(any(
    feature = "cuda-12020",
    feature = "cuda-12030",
    feature = "cuda-12040",
    feature = "cuda-12050",
    feature = "cuda-12060",
    feature = "cuda-12080",
    feature = "cuda-12090",
    feature = "cuda-13000"
))]
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct cudaMemcpyNodeParams {
    pub flags: ::core::ffi::c_int,
    pub reserved: [::core::ffi::c_int; 3usize],
    pub copyParams: cudaMemcpy3DParms,
}
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct cudaMemsetParams {
    pub dst: *mut ::core::ffi::c_void,
    pub pitch: usize,
    pub value: ::core::ffi::c_uint,
    pub elementSize: ::core::ffi::c_uint,
    pub width: usize,
    pub height: usize,
}
#[cfg(any(
    feature = "cuda-12020",
    feature = "cuda-12030",
    feature = "cuda-12040",
    feature = "cuda-12050",
    feature = "cuda-12060",
    feature = "cuda-12080",
    feature = "cuda-12090",
    feature = "cuda-13000"
))]
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct cudaMemsetParamsV2 {
    pub dst: *mut ::core::ffi::c_void,
    pub pitch: usize,
    pub value: ::core::ffi::c_uint,
    pub elementSize: ::core::ffi::c_uint,
    pub width: usize,
    pub height: usize,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct cudaMipmappedArray {
    _unused: [u8; 0],
}
#[cfg(any(feature = "cuda-12080", feature = "cuda-12090", feature = "cuda-13000"))]
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct cudaOffset3D {
    pub x: usize,
    pub y: usize,
    pub z: usize,
}
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct cudaPitchedPtr {
    pub ptr: *mut ::core::ffi::c_void,
    pub pitch: usize,
    pub xsize: usize,
    pub ysize: usize,
}
#[cfg(any(
    feature = "cuda-11040",
    feature = "cuda-11050",
    feature = "cuda-11060",
    feature = "cuda-11070",
    feature = "cuda-11080",
    feature = "cuda-12000",
    feature = "cuda-12010",
    feature = "cuda-12020",
    feature = "cuda-12030",
    feature = "cuda-12040",
    feature = "cuda-12050",
    feature = "cuda-12060",
    feature = "cuda-12080",
    feature = "cuda-12090"
))]
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct cudaPointerAttributes {
    pub type_: cudaMemoryType,
    pub device: ::core::ffi::c_int,
    pub devicePointer: *mut ::core::ffi::c_void,
    pub hostPointer: *mut ::core::ffi::c_void,
}
#[cfg(any(feature = "cuda-13000"))]
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct cudaPointerAttributes {
    pub type_: cudaMemoryType,
    pub device: ::core::ffi::c_int,
    pub devicePointer: *mut ::core::ffi::c_void,
    pub hostPointer: *mut ::core::ffi::c_void,
    pub reserved: [::core::ffi::c_long; 8usize],
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct cudaPos {
    pub x: usize,
    pub y: usize,
    pub z: usize,
}
#[cfg(any(
    feature = "cuda-11040",
    feature = "cuda-11050",
    feature = "cuda-11060",
    feature = "cuda-11070",
    feature = "cuda-11080",
    feature = "cuda-12000",
    feature = "cuda-12010",
    feature = "cuda-12020",
    feature = "cuda-12030",
    feature = "cuda-12040",
    feature = "cuda-12050",
    feature = "cuda-12060",
    feature = "cuda-12080",
    feature = "cuda-12090"
))]
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cudaResourceDesc {
    pub resType: cudaResourceType,
    pub res: cudaResourceDesc__bindgen_ty_1,
}
#[cfg(any(feature = "cuda-13000"))]
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cudaResourceDesc {
    pub resType: cudaResourceType,
    pub res: cudaResourceDesc__bindgen_ty_1,
    pub flags: ::core::ffi::c_uint,
}
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct cudaResourceDesc__bindgen_ty_1__bindgen_ty_1 {
    pub array: cudaArray_t,
}
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct cudaResourceDesc__bindgen_ty_1__bindgen_ty_2 {
    pub mipmap: cudaMipmappedArray_t,
}
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct cudaResourceDesc__bindgen_ty_1__bindgen_ty_3 {
    pub devPtr: *mut ::core::ffi::c_void,
    pub desc: cudaChannelFormatDesc,
    pub sizeInBytes: usize,
}
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct cudaResourceDesc__bindgen_ty_1__bindgen_ty_4 {
    pub devPtr: *mut ::core::ffi::c_void,
    pub desc: cudaChannelFormatDesc,
    pub width: usize,
    pub height: usize,
    pub pitchInBytes: usize,
}
#[cfg(any(feature = "cuda-13000"))]
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct cudaResourceDesc__bindgen_ty_1__bindgen_ty_5 {
    pub reserved: [::core::ffi::c_int; 32usize],
}
#[cfg(any(
    feature = "cuda-11040",
    feature = "cuda-11050",
    feature = "cuda-11060",
    feature = "cuda-11070",
    feature = "cuda-11080",
    feature = "cuda-12000",
    feature = "cuda-12010",
    feature = "cuda-12020",
    feature = "cuda-12030",
    feature = "cuda-12040",
    feature = "cuda-12050",
    feature = "cuda-12060",
    feature = "cuda-12080",
    feature = "cuda-12090"
))]
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct cudaResourceViewDesc {
    pub format: cudaResourceViewFormat,
    pub width: usize,
    pub height: usize,
    pub depth: usize,
    pub firstMipmapLevel: ::core::ffi::c_uint,
    pub lastMipmapLevel: ::core::ffi::c_uint,
    pub firstLayer: ::core::ffi::c_uint,
    pub lastLayer: ::core::ffi::c_uint,
}
#[cfg(any(feature = "cuda-13000"))]
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct cudaResourceViewDesc {
    pub format: cudaResourceViewFormat,
    pub width: usize,
    pub height: usize,
    pub depth: usize,
    pub firstMipmapLevel: ::core::ffi::c_uint,
    pub lastMipmapLevel: ::core::ffi::c_uint,
    pub firstLayer: ::core::ffi::c_uint,
    pub lastLayer: ::core::ffi::c_uint,
    pub reserved: [::core::ffi::c_uint; 16usize],
}
#[cfg(any(feature = "cuda-11040", feature = "cuda-11050"))]
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialOrd, PartialEq)]
pub struct cudaTextureDesc {
    pub addressMode: [cudaTextureAddressMode; 3usize],
    pub filterMode: cudaTextureFilterMode,
    pub readMode: cudaTextureReadMode,
    pub sRGB: ::core::ffi::c_int,
    pub borderColor: [f32; 4usize],
    pub normalizedCoords: ::core::ffi::c_int,
    pub maxAnisotropy: ::core::ffi::c_uint,
    pub mipmapFilterMode: cudaTextureFilterMode,
    pub mipmapLevelBias: f32,
    pub minMipmapLevelClamp: f32,
    pub maxMipmapLevelClamp: f32,
    pub disableTrilinearOptimization: ::core::ffi::c_int,
}
#[cfg(any(feature = "cuda-11060", feature = "cuda-11070"))]
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialOrd, PartialEq)]
pub struct cudaTextureDesc {
    pub addressMode: [cudaTextureAddressMode; 3usize],
    pub filterMode: cudaTextureFilterMode,
    pub readMode: cudaTextureReadMode,
    pub sRGB: ::core::ffi::c_int,
    pub borderColor: [f32; 4usize],
    pub normalizedCoords: ::core::ffi::c_int,
    pub maxAnisotropy: ::core::ffi::c_uint,
    pub mipmapFilterMode: cudaTextureFilterMode,
    pub mipmapLevelBias: f32,
    pub minMipmapLevelClamp: f32,
    pub maxMipmapLevelClamp: f32,
    pub disableTrilinearOptimization: ::core::ffi::c_int,
    pub seamlessCubemap: ::core::ffi::c_int,
}
#[cfg(any(feature = "cuda-11080"))]
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialOrd, PartialEq)]
pub struct cudaTextureDesc {
    pub addressMode: [cudaTextureAddressMode; 3usize],
    pub filterMode: cudaTextureFilterMode,
    pub readMode: cudaTextureReadMode,
    pub sRGB: ::core::ffi::c_int,
    pub borderColor: [f32; 4usize],
    pub normalizedCoords: ::core::ffi::c_int,
    pub maxAnisotropy: ::core::ffi::c_uint,
    pub mipmapFilterMode: cudaTextureFilterMode,
    pub mipmapLevelBias: f32,
    pub minMipmapLevelClamp: f32,
    pub maxMipmapLevelClamp: f32,
    pub disableTrilinearOptimization: ::core::ffi::c_int,
}
#[cfg(any(
    feature = "cuda-12000",
    feature = "cuda-12010",
    feature = "cuda-12020",
    feature = "cuda-12030",
    feature = "cuda-12040",
    feature = "cuda-12050",
    feature = "cuda-12060",
    feature = "cuda-12080",
    feature = "cuda-12090",
    feature = "cuda-13000"
))]
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialOrd, PartialEq)]
pub struct cudaTextureDesc {
    pub addressMode: [cudaTextureAddressMode; 3usize],
    pub filterMode: cudaTextureFilterMode,
    pub readMode: cudaTextureReadMode,
    pub sRGB: ::core::ffi::c_int,
    pub borderColor: [f32; 4usize],
    pub normalizedCoords: ::core::ffi::c_int,
    pub maxAnisotropy: ::core::ffi::c_uint,
    pub mipmapFilterMode: cudaTextureFilterMode,
    pub mipmapLevelBias: f32,
    pub minMipmapLevelClamp: f32,
    pub maxMipmapLevelClamp: f32,
    pub disableTrilinearOptimization: ::core::ffi::c_int,
    pub seamlessCubemap: ::core::ffi::c_int,
}
#[cfg(any(feature = "cuda-11080"))]
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialOrd, PartialEq)]
pub struct cudaTextureDesc_v2 {
    pub addressMode: [cudaTextureAddressMode; 3usize],
    pub filterMode: cudaTextureFilterMode,
    pub readMode: cudaTextureReadMode,
    pub sRGB: ::core::ffi::c_int,
    pub borderColor: [f32; 4usize],
    pub normalizedCoords: ::core::ffi::c_int,
    pub maxAnisotropy: ::core::ffi::c_uint,
    pub mipmapFilterMode: cudaTextureFilterMode,
    pub mipmapLevelBias: f32,
    pub minMipmapLevelClamp: f32,
    pub maxMipmapLevelClamp: f32,
    pub disableTrilinearOptimization: ::core::ffi::c_int,
    pub seamlessCubemap: ::core::ffi::c_int,
}
#[cfg(any(feature = "cuda-12080", feature = "cuda-12090", feature = "cuda-13000"))]
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct cudalibraryHostUniversalFunctionAndDataTable {
    pub functionTable: *mut ::core::ffi::c_void,
    pub functionWindowSize: usize,
    pub dataTable: *mut ::core::ffi::c_void,
    pub dataWindowSize: usize,
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct dim3 {
    pub x: ::core::ffi::c_uint,
    pub y: ::core::ffi::c_uint,
    pub z: ::core::ffi::c_uint,
}
#[cfg(any(
    feature = "cuda-11040",
    feature = "cuda-11050",
    feature = "cuda-11060",
    feature = "cuda-11070",
    feature = "cuda-11080"
))]
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct surfaceReference {
    pub channelDesc: cudaChannelFormatDesc,
}
#[cfg(any(
    feature = "cuda-11040",
    feature = "cuda-11050",
    feature = "cuda-11060",
    feature = "cuda-11070",
    feature = "cuda-11080"
))]
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialOrd, PartialEq)]
pub struct textureReference {
    pub normalized: ::core::ffi::c_int,
    pub filterMode: cudaTextureFilterMode,
    pub addressMode: [cudaTextureAddressMode; 3usize],
    pub channelDesc: cudaChannelFormatDesc,
    pub sRGB: ::core::ffi::c_int,
    pub maxAnisotropy: ::core::ffi::c_uint,
    pub mipmapFilterMode: cudaTextureFilterMode,
    pub mipmapLevelBias: f32,
    pub minMipmapLevelClamp: f32,
    pub maxMipmapLevelClamp: f32,
    pub disableTrilinearOptimization: ::core::ffi::c_int,
    pub __cudaReserved: [::core::ffi::c_int; 14usize],
}
#[cfg(any(feature = "cuda-12080", feature = "cuda-12090", feature = "cuda-13000"))]
impl cudaDataType_t {
    pub const CUDA_R_8F_UE4M3: cudaDataType_t = cudaDataType_t::CUDA_R_8F_E4M3;
}
#[cfg(any(
    feature = "cuda-11050",
    feature = "cuda-11060",
    feature = "cuda-11070",
    feature = "cuda-11080",
    feature = "cuda-12000",
    feature = "cuda-12010",
    feature = "cuda-12020",
    feature = "cuda-12030",
    feature = "cuda-12040",
    feature = "cuda-12050",
    feature = "cuda-12060",
    feature = "cuda-12080",
    feature = "cuda-12090"
))]
impl cudaDeviceAttr {
    pub const cudaDevAttrMaxTimelineSemaphoreInteropSupported: cudaDeviceAttr =
        cudaDeviceAttr::cudaDevAttrTimelineSemaphoreInteropSupported;
}
#[cfg(any(feature = "cuda-13000"))]
impl cudaMemLocationType {
    pub const cudaMemLocationTypeNone: cudaMemLocationType =
        cudaMemLocationType::cudaMemLocationTypeInvalid;
}
impl Default for cudaAccessPolicyWindow {
    fn default() -> Self {
        let mut s = ::core::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::core::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[cfg(any(
    feature = "cuda-12040",
    feature = "cuda-12050",
    feature = "cuda-12060",
    feature = "cuda-12080",
    feature = "cuda-12090",
    feature = "cuda-13000"
))]
impl Default for cudaAsyncNotificationInfo {
    fn default() -> Self {
        let mut s = ::core::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::core::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[cfg(any(
    feature = "cuda-12040",
    feature = "cuda-12050",
    feature = "cuda-12060",
    feature = "cuda-12080",
    feature = "cuda-12090",
    feature = "cuda-13000"
))]
impl Default for cudaAsyncNotificationInfo__bindgen_ty_1 {
    fn default() -> Self {
        let mut s = ::core::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::core::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
impl Default for cudaChannelFormatDesc {
    fn default() -> Self {
        let mut s = ::core::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::core::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[cfg(any(
    feature = "cuda-12020",
    feature = "cuda-12030",
    feature = "cuda-12040",
    feature = "cuda-12050",
    feature = "cuda-12060",
    feature = "cuda-12080",
    feature = "cuda-12090",
    feature = "cuda-13000"
))]
impl Default for cudaChildGraphNodeParams {
    fn default() -> Self {
        let mut s = ::core::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::core::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[cfg(any(
    feature = "cuda-12030",
    feature = "cuda-12040",
    feature = "cuda-12050",
    feature = "cuda-12060",
    feature = "cuda-12080",
    feature = "cuda-12090",
    feature = "cuda-13000"
))]
impl Default for cudaConditionalNodeParams {
    fn default() -> Self {
        let mut s = ::core::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::core::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
impl Default for cudaDeviceProp {
    fn default() -> Self {
        let mut s = ::core::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::core::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[cfg(any(
    feature = "cuda-12020",
    feature = "cuda-12030",
    feature = "cuda-12040",
    feature = "cuda-12050",
    feature = "cuda-12060",
    feature = "cuda-12080",
    feature = "cuda-12090",
    feature = "cuda-13000"
))]
impl Default for cudaEventRecordNodeParams {
    fn default() -> Self {
        let mut s = ::core::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::core::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[cfg(any(
    feature = "cuda-12020",
    feature = "cuda-12030",
    feature = "cuda-12040",
    feature = "cuda-12050",
    feature = "cuda-12060",
    feature = "cuda-12080",
    feature = "cuda-12090",
    feature = "cuda-13000"
))]
impl Default for cudaEventWaitNodeParams {
    fn default() -> Self {
        let mut s = ::core::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::core::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
impl Default for cudaExternalMemoryHandleDesc {
    fn default() -> Self {
        let mut s = ::core::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::core::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
impl Default for cudaExternalMemoryHandleDesc__bindgen_ty_1 {
    fn default() -> Self {
        let mut s = ::core::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::core::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
impl Default for cudaExternalMemoryHandleDesc__bindgen_ty_1__bindgen_ty_1 {
    fn default() -> Self {
        let mut s = ::core::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::core::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
impl Default for cudaExternalMemoryMipmappedArrayDesc {
    fn default() -> Self {
        let mut s = ::core::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::core::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
impl Default for cudaExternalSemaphoreHandleDesc {
    fn default() -> Self {
        let mut s = ::core::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::core::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
impl Default for cudaExternalSemaphoreHandleDesc__bindgen_ty_1 {
    fn default() -> Self {
        let mut s = ::core::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::core::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
impl Default for cudaExternalSemaphoreHandleDesc__bindgen_ty_1__bindgen_ty_1 {
    fn default() -> Self {
        let mut s = ::core::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::core::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
impl Default for cudaExternalSemaphoreSignalNodeParams {
    fn default() -> Self {
        let mut s = ::core::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::core::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[cfg(any(
    feature = "cuda-12020",
    feature = "cuda-12030",
    feature = "cuda-12040",
    feature = "cuda-12050",
    feature = "cuda-12060",
    feature = "cuda-12080",
    feature = "cuda-12090",
    feature = "cuda-13000"
))]
impl Default for cudaExternalSemaphoreSignalNodeParamsV2 {
    fn default() -> Self {
        let mut s = ::core::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::core::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
impl Default for cudaExternalSemaphoreSignalParams {
    fn default() -> Self {
        let mut s = ::core::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::core::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
impl Default for cudaExternalSemaphoreSignalParams__bindgen_ty_1 {
    fn default() -> Self {
        let mut s = ::core::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::core::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
impl Default for cudaExternalSemaphoreSignalParams__bindgen_ty_1__bindgen_ty_2 {
    fn default() -> Self {
        let mut s = ::core::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::core::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[cfg(any(
    feature = "cuda-11050",
    feature = "cuda-11060",
    feature = "cuda-11070",
    feature = "cuda-11080",
    feature = "cuda-12000",
    feature = "cuda-12010",
    feature = "cuda-12020",
    feature = "cuda-12030",
    feature = "cuda-12040",
    feature = "cuda-12050",
    feature = "cuda-12060",
    feature = "cuda-12080",
    feature = "cuda-12090"
))]
impl Default for cudaExternalSemaphoreSignalParams_v1 {
    fn default() -> Self {
        let mut s = ::core::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::core::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[cfg(any(
    feature = "cuda-11050",
    feature = "cuda-11060",
    feature = "cuda-11070",
    feature = "cuda-11080",
    feature = "cuda-12000",
    feature = "cuda-12010",
    feature = "cuda-12020",
    feature = "cuda-12030",
    feature = "cuda-12040",
    feature = "cuda-12050",
    feature = "cuda-12060",
    feature = "cuda-12080",
    feature = "cuda-12090"
))]
impl Default for cudaExternalSemaphoreSignalParams_v1__bindgen_ty_1 {
    fn default() -> Self {
        let mut s = ::core::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::core::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[cfg(any(
    feature = "cuda-11050",
    feature = "cuda-11060",
    feature = "cuda-11070",
    feature = "cuda-11080",
    feature = "cuda-12000",
    feature = "cuda-12010",
    feature = "cuda-12020",
    feature = "cuda-12030",
    feature = "cuda-12040",
    feature = "cuda-12050",
    feature = "cuda-12060",
    feature = "cuda-12080",
    feature = "cuda-12090"
))]
impl Default for cudaExternalSemaphoreSignalParams_v1__bindgen_ty_1__bindgen_ty_2 {
    fn default() -> Self {
        let mut s = ::core::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::core::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
impl Default for cudaExternalSemaphoreWaitNodeParams {
    fn default() -> Self {
        let mut s = ::core::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::core::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[cfg(any(
    feature = "cuda-12020",
    feature = "cuda-12030",
    feature = "cuda-12040",
    feature = "cuda-12050",
    feature = "cuda-12060",
    feature = "cuda-12080",
    feature = "cuda-12090",
    feature = "cuda-13000"
))]
impl Default for cudaExternalSemaphoreWaitNodeParamsV2 {
    fn default() -> Self {
        let mut s = ::core::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::core::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
impl Default for cudaExternalSemaphoreWaitParams {
    fn default() -> Self {
        let mut s = ::core::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::core::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
impl Default for cudaExternalSemaphoreWaitParams__bindgen_ty_1 {
    fn default() -> Self {
        let mut s = ::core::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::core::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
impl Default for cudaExternalSemaphoreWaitParams__bindgen_ty_1__bindgen_ty_2 {
    fn default() -> Self {
        let mut s = ::core::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::core::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[cfg(any(
    feature = "cuda-11050",
    feature = "cuda-11060",
    feature = "cuda-11070",
    feature = "cuda-11080",
    feature = "cuda-12000",
    feature = "cuda-12010",
    feature = "cuda-12020",
    feature = "cuda-12030",
    feature = "cuda-12040",
    feature = "cuda-12050",
    feature = "cuda-12060",
    feature = "cuda-12080",
    feature = "cuda-12090"
))]
impl Default for cudaExternalSemaphoreWaitParams_v1 {
    fn default() -> Self {
        let mut s = ::core::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::core::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[cfg(any(
    feature = "cuda-11050",
    feature = "cuda-11060",
    feature = "cuda-11070",
    feature = "cuda-11080",
    feature = "cuda-12000",
    feature = "cuda-12010",
    feature = "cuda-12020",
    feature = "cuda-12030",
    feature = "cuda-12040",
    feature = "cuda-12050",
    feature = "cuda-12060",
    feature = "cuda-12080",
    feature = "cuda-12090"
))]
impl Default for cudaExternalSemaphoreWaitParams_v1__bindgen_ty_1 {
    fn default() -> Self {
        let mut s = ::core::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::core::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[cfg(any(
    feature = "cuda-11050",
    feature = "cuda-11060",
    feature = "cuda-11070",
    feature = "cuda-11080",
    feature = "cuda-12000",
    feature = "cuda-12010",
    feature = "cuda-12020",
    feature = "cuda-12030",
    feature = "cuda-12040",
    feature = "cuda-12050",
    feature = "cuda-12060",
    feature = "cuda-12080",
    feature = "cuda-12090"
))]
impl Default for cudaExternalSemaphoreWaitParams_v1__bindgen_ty_1__bindgen_ty_2 {
    fn default() -> Self {
        let mut s = ::core::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::core::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[cfg(any(
    feature = "cuda-12000",
    feature = "cuda-12010",
    feature = "cuda-12020",
    feature = "cuda-12030",
    feature = "cuda-12040",
    feature = "cuda-12050",
    feature = "cuda-12060",
    feature = "cuda-12080",
    feature = "cuda-12090",
    feature = "cuda-13000"
))]
impl Default for cudaGraphExecUpdateResultInfo_st {
    fn default() -> Self {
        let mut s = ::core::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::core::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[cfg(any(
    feature = "cuda-12000",
    feature = "cuda-12010",
    feature = "cuda-12020",
    feature = "cuda-12030",
    feature = "cuda-12040",
    feature = "cuda-12050",
    feature = "cuda-12060",
    feature = "cuda-12080",
    feature = "cuda-12090",
    feature = "cuda-13000"
))]
impl Default for cudaGraphInstantiateParams_st {
    fn default() -> Self {
        let mut s = ::core::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::core::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[cfg(any(
    feature = "cuda-12040",
    feature = "cuda-12050",
    feature = "cuda-12060",
    feature = "cuda-12080",
    feature = "cuda-12090",
    feature = "cuda-13000"
))]
impl Default for cudaGraphKernelNodeUpdate {
    fn default() -> Self {
        let mut s = ::core::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::core::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[cfg(any(
    feature = "cuda-12040",
    feature = "cuda-12050",
    feature = "cuda-12060",
    feature = "cuda-12080",
    feature = "cuda-12090",
    feature = "cuda-13000"
))]
impl Default for cudaGraphKernelNodeUpdate__bindgen_ty_1 {
    fn default() -> Self {
        let mut s = ::core::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::core::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[cfg(any(
    feature = "cuda-12040",
    feature = "cuda-12050",
    feature = "cuda-12060",
    feature = "cuda-12080",
    feature = "cuda-12090",
    feature = "cuda-13000"
))]
impl Default for cudaGraphKernelNodeUpdate__bindgen_ty_1__bindgen_ty_1 {
    fn default() -> Self {
        let mut s = ::core::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::core::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[cfg(any(
    feature = "cuda-12020",
    feature = "cuda-12030",
    feature = "cuda-12040",
    feature = "cuda-12050",
    feature = "cuda-12060",
    feature = "cuda-12080",
    feature = "cuda-12090",
    feature = "cuda-13000"
))]
impl Default for cudaGraphNodeParams {
    fn default() -> Self {
        let mut s = ::core::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::core::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[cfg(any(
    feature = "cuda-12020",
    feature = "cuda-12030",
    feature = "cuda-12040",
    feature = "cuda-12050",
    feature = "cuda-12060",
    feature = "cuda-12080",
    feature = "cuda-12090",
    feature = "cuda-13000"
))]
impl Default for cudaGraphNodeParams__bindgen_ty_1 {
    fn default() -> Self {
        let mut s = ::core::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::core::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
impl Default for cudaHostNodeParams {
    fn default() -> Self {
        let mut s = ::core::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::core::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[cfg(any(
    feature = "cuda-12020",
    feature = "cuda-12030",
    feature = "cuda-12040",
    feature = "cuda-12050",
    feature = "cuda-12060",
    feature = "cuda-12080",
    feature = "cuda-12090",
    feature = "cuda-13000"
))]
impl Default for cudaHostNodeParamsV2 {
    fn default() -> Self {
        let mut s = ::core::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::core::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
impl Default for cudaIpcEventHandle_st {
    fn default() -> Self {
        let mut s = ::core::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::core::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
impl Default for cudaIpcMemHandle_st {
    fn default() -> Self {
        let mut s = ::core::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::core::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[cfg(any(
    feature = "cuda-11040",
    feature = "cuda-11050",
    feature = "cuda-11060",
    feature = "cuda-11070"
))]
impl Default for cudaKernelNodeAttrValue {
    fn default() -> Self {
        let mut s = ::core::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::core::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
impl Default for cudaKernelNodeParams {
    fn default() -> Self {
        let mut s = ::core::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::core::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[cfg(any(
    feature = "cuda-12020",
    feature = "cuda-12030",
    feature = "cuda-12040",
    feature = "cuda-12050",
    feature = "cuda-12060",
    feature = "cuda-12080",
    feature = "cuda-12090",
    feature = "cuda-13000"
))]
impl Default for cudaKernelNodeParamsV2 {
    fn default() -> Self {
        let mut s = ::core::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::core::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[cfg(any(
    feature = "cuda-11080",
    feature = "cuda-12000",
    feature = "cuda-12010",
    feature = "cuda-12020",
    feature = "cuda-12030",
    feature = "cuda-12040",
    feature = "cuda-12050",
    feature = "cuda-12060",
    feature = "cuda-12080",
    feature = "cuda-12090",
    feature = "cuda-13000"
))]
impl Default for cudaLaunchAttributeValue {
    fn default() -> Self {
        let mut s = ::core::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::core::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[cfg(any(
    feature = "cuda-11080",
    feature = "cuda-12000",
    feature = "cuda-12010",
    feature = "cuda-12020",
    feature = "cuda-12030",
    feature = "cuda-12040",
    feature = "cuda-12050",
    feature = "cuda-12060",
    feature = "cuda-12080",
    feature = "cuda-12090",
    feature = "cuda-13000"
))]
impl Default for cudaLaunchAttributeValue__bindgen_ty_2 {
    fn default() -> Self {
        let mut s = ::core::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::core::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[cfg(any(
    feature = "cuda-12030",
    feature = "cuda-12040",
    feature = "cuda-12050",
    feature = "cuda-12060"
))]
impl Default for cudaLaunchAttributeValue__bindgen_ty_3 {
    fn default() -> Self {
        let mut s = ::core::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::core::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[cfg(any(
    feature = "cuda-12040",
    feature = "cuda-12050",
    feature = "cuda-12060",
    feature = "cuda-12080",
    feature = "cuda-12090",
    feature = "cuda-13000"
))]
impl Default for cudaLaunchAttributeValue__bindgen_ty_4 {
    fn default() -> Self {
        let mut s = ::core::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::core::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[cfg(any(feature = "cuda-12080", feature = "cuda-12090", feature = "cuda-13000"))]
impl Default for cudaLaunchAttributeValue__bindgen_ty_5 {
    fn default() -> Self {
        let mut s = ::core::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::core::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[cfg(any(
    feature = "cuda-11080",
    feature = "cuda-12000",
    feature = "cuda-12010",
    feature = "cuda-12020",
    feature = "cuda-12030",
    feature = "cuda-12040",
    feature = "cuda-12050",
    feature = "cuda-12060",
    feature = "cuda-12080",
    feature = "cuda-12090",
    feature = "cuda-13000"
))]
impl Default for cudaLaunchAttribute_st {
    fn default() -> Self {
        let mut s = ::core::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::core::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[cfg(any(
    feature = "cuda-11080",
    feature = "cuda-12000",
    feature = "cuda-12010",
    feature = "cuda-12020",
    feature = "cuda-12030",
    feature = "cuda-12040",
    feature = "cuda-12050",
    feature = "cuda-12060",
    feature = "cuda-12080",
    feature = "cuda-12090",
    feature = "cuda-13000"
))]
impl Default for cudaLaunchConfig_st {
    fn default() -> Self {
        let mut s = ::core::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::core::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[cfg(any(
    feature = "cuda-11040",
    feature = "cuda-11050",
    feature = "cuda-11060",
    feature = "cuda-11070",
    feature = "cuda-11080",
    feature = "cuda-12000",
    feature = "cuda-12010",
    feature = "cuda-12020",
    feature = "cuda-12030",
    feature = "cuda-12040",
    feature = "cuda-12050",
    feature = "cuda-12060",
    feature = "cuda-12080",
    feature = "cuda-12090"
))]
impl Default for cudaLaunchParams {
    fn default() -> Self {
        let mut s = ::core::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::core::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
impl Default for cudaMemAccessDesc {
    fn default() -> Self {
        let mut s = ::core::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::core::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
impl Default for cudaMemAllocNodeParams {
    fn default() -> Self {
        let mut s = ::core::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::core::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[cfg(any(
    feature = "cuda-12020",
    feature = "cuda-12030",
    feature = "cuda-12040",
    feature = "cuda-12050",
    feature = "cuda-12060",
    feature = "cuda-12080",
    feature = "cuda-12090",
    feature = "cuda-13000"
))]
impl Default for cudaMemAllocNodeParamsV2 {
    fn default() -> Self {
        let mut s = ::core::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::core::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[cfg(any(
    feature = "cuda-12030",
    feature = "cuda-12040",
    feature = "cuda-12050",
    feature = "cuda-12060",
    feature = "cuda-12080",
    feature = "cuda-12090",
    feature = "cuda-13000"
))]
impl Default for cudaMemFabricHandle_st {
    fn default() -> Self {
        let mut s = ::core::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::core::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[cfg(any(
    feature = "cuda-12020",
    feature = "cuda-12030",
    feature = "cuda-12040",
    feature = "cuda-12050",
    feature = "cuda-12060",
    feature = "cuda-12080",
    feature = "cuda-12090",
    feature = "cuda-13000"
))]
impl Default for cudaMemFreeNodeParams {
    fn default() -> Self {
        let mut s = ::core::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::core::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
impl Default for cudaMemLocation {
    fn default() -> Self {
        let mut s = ::core::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::core::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
impl Default for cudaMemPoolProps {
    fn default() -> Self {
        let mut s = ::core::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::core::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
impl Default for cudaMemPoolPtrExportData {
    fn default() -> Self {
        let mut s = ::core::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::core::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[cfg(any(feature = "cuda-12080", feature = "cuda-12090", feature = "cuda-13000"))]
impl Default for cudaMemcpy3DBatchOp {
    fn default() -> Self {
        let mut s = ::core::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::core::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[cfg(any(feature = "cuda-12080", feature = "cuda-12090", feature = "cuda-13000"))]
impl Default for cudaMemcpy3DOperand {
    fn default() -> Self {
        let mut s = ::core::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::core::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[cfg(any(feature = "cuda-12080", feature = "cuda-12090", feature = "cuda-13000"))]
impl Default for cudaMemcpy3DOperand__bindgen_ty_1 {
    fn default() -> Self {
        let mut s = ::core::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::core::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[cfg(any(feature = "cuda-12080", feature = "cuda-12090", feature = "cuda-13000"))]
impl Default for cudaMemcpy3DOperand__bindgen_ty_1__bindgen_ty_1 {
    fn default() -> Self {
        let mut s = ::core::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::core::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[cfg(any(feature = "cuda-12080", feature = "cuda-12090", feature = "cuda-13000"))]
impl Default for cudaMemcpy3DOperand__bindgen_ty_1__bindgen_ty_2 {
    fn default() -> Self {
        let mut s = ::core::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::core::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
impl Default for cudaMemcpy3DParms {
    fn default() -> Self {
        let mut s = ::core::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::core::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
impl Default for cudaMemcpy3DPeerParms {
    fn default() -> Self {
        let mut s = ::core::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::core::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[cfg(any(feature = "cuda-12080", feature = "cuda-12090", feature = "cuda-13000"))]
impl Default for cudaMemcpyAttributes {
    fn default() -> Self {
        let mut s = ::core::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::core::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[cfg(any(
    feature = "cuda-12020",
    feature = "cuda-12030",
    feature = "cuda-12040",
    feature = "cuda-12050",
    feature = "cuda-12060",
    feature = "cuda-12080",
    feature = "cuda-12090",
    feature = "cuda-13000"
))]
impl Default for cudaMemcpyNodeParams {
    fn default() -> Self {
        let mut s = ::core::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::core::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
impl Default for cudaMemsetParams {
    fn default() -> Self {
        let mut s = ::core::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::core::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[cfg(any(
    feature = "cuda-12020",
    feature = "cuda-12030",
    feature = "cuda-12040",
    feature = "cuda-12050",
    feature = "cuda-12060",
    feature = "cuda-12080",
    feature = "cuda-12090",
    feature = "cuda-13000"
))]
impl Default for cudaMemsetParamsV2 {
    fn default() -> Self {
        let mut s = ::core::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::core::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
impl Default for cudaPitchedPtr {
    fn default() -> Self {
        let mut s = ::core::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::core::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
impl Default for cudaPointerAttributes {
    fn default() -> Self {
        let mut s = ::core::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::core::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
impl Default for cudaResourceDesc {
    fn default() -> Self {
        let mut s = ::core::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::core::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
impl Default for cudaResourceDesc__bindgen_ty_1 {
    fn default() -> Self {
        let mut s = ::core::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::core::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
impl Default for cudaResourceDesc__bindgen_ty_1__bindgen_ty_1 {
    fn default() -> Self {
        let mut s = ::core::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::core::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
impl Default for cudaResourceDesc__bindgen_ty_1__bindgen_ty_2 {
    fn default() -> Self {
        let mut s = ::core::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::core::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
impl Default for cudaResourceDesc__bindgen_ty_1__bindgen_ty_3 {
    fn default() -> Self {
        let mut s = ::core::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::core::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
impl Default for cudaResourceDesc__bindgen_ty_1__bindgen_ty_4 {
    fn default() -> Self {
        let mut s = ::core::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::core::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
impl Default for cudaResourceViewDesc {
    fn default() -> Self {
        let mut s = ::core::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::core::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[cfg(any(
    feature = "cuda-11040",
    feature = "cuda-11050",
    feature = "cuda-11060",
    feature = "cuda-11070"
))]
impl Default for cudaStreamAttrValue {
    fn default() -> Self {
        let mut s = ::core::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::core::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
impl Default for cudaTextureDesc {
    fn default() -> Self {
        let mut s = ::core::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::core::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[cfg(any(feature = "cuda-11080"))]
impl Default for cudaTextureDesc_v2 {
    fn default() -> Self {
        let mut s = ::core::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::core::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[cfg(any(feature = "cuda-12080", feature = "cuda-12090", feature = "cuda-13000"))]
impl Default for cudalibraryHostUniversalFunctionAndDataTable {
    fn default() -> Self {
        let mut s = ::core::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::core::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[cfg(any(
    feature = "cuda-11040",
    feature = "cuda-11050",
    feature = "cuda-11060",
    feature = "cuda-11070",
    feature = "cuda-11080"
))]
impl Default for surfaceReference {
    fn default() -> Self {
        let mut s = ::core::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::core::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[cfg(any(
    feature = "cuda-11040",
    feature = "cuda-11050",
    feature = "cuda-11060",
    feature = "cuda-11070",
    feature = "cuda-11080"
))]
impl Default for textureReference {
    fn default() -> Self {
        let mut s = ::core::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::core::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[cfg(any(
    feature = "cuda-12040",
    feature = "cuda-12050",
    feature = "cuda-12060",
    feature = "cuda-12080",
    feature = "cuda-12090",
    feature = "cuda-13000"
))]
#[repr(C)]
#[derive(Copy, Clone)]
pub union cudaAsyncNotificationInfo__bindgen_ty_1 {
    pub overBudget: cudaAsyncNotificationInfo__bindgen_ty_1__bindgen_ty_1,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union cudaExternalMemoryHandleDesc__bindgen_ty_1 {
    pub fd: ::core::ffi::c_int,
    pub win32: cudaExternalMemoryHandleDesc__bindgen_ty_1__bindgen_ty_1,
    pub nvSciBufObject: *const ::core::ffi::c_void,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union cudaExternalSemaphoreHandleDesc__bindgen_ty_1 {
    pub fd: ::core::ffi::c_int,
    pub win32: cudaExternalSemaphoreHandleDesc__bindgen_ty_1__bindgen_ty_1,
    pub nvSciSyncObj: *const ::core::ffi::c_void,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union cudaExternalSemaphoreSignalParams__bindgen_ty_1__bindgen_ty_2 {
    pub fence: *mut ::core::ffi::c_void,
    pub reserved: ::core::ffi::c_ulonglong,
}
#[cfg(any(
    feature = "cuda-11050",
    feature = "cuda-11060",
    feature = "cuda-11070",
    feature = "cuda-11080",
    feature = "cuda-12000",
    feature = "cuda-12010",
    feature = "cuda-12020",
    feature = "cuda-12030",
    feature = "cuda-12040",
    feature = "cuda-12050",
    feature = "cuda-12060",
    feature = "cuda-12080",
    feature = "cuda-12090"
))]
#[repr(C)]
#[derive(Copy, Clone)]
pub union cudaExternalSemaphoreSignalParams_v1__bindgen_ty_1__bindgen_ty_2 {
    pub fence: *mut ::core::ffi::c_void,
    pub reserved: ::core::ffi::c_ulonglong,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union cudaExternalSemaphoreWaitParams__bindgen_ty_1__bindgen_ty_2 {
    pub fence: *mut ::core::ffi::c_void,
    pub reserved: ::core::ffi::c_ulonglong,
}
#[cfg(any(
    feature = "cuda-11050",
    feature = "cuda-11060",
    feature = "cuda-11070",
    feature = "cuda-11080",
    feature = "cuda-12000",
    feature = "cuda-12010",
    feature = "cuda-12020",
    feature = "cuda-12030",
    feature = "cuda-12040",
    feature = "cuda-12050",
    feature = "cuda-12060",
    feature = "cuda-12080",
    feature = "cuda-12090"
))]
#[repr(C)]
#[derive(Copy, Clone)]
pub union cudaExternalSemaphoreWaitParams_v1__bindgen_ty_1__bindgen_ty_2 {
    pub fence: *mut ::core::ffi::c_void,
    pub reserved: ::core::ffi::c_ulonglong,
}
#[cfg(any(
    feature = "cuda-12040",
    feature = "cuda-12050",
    feature = "cuda-12060",
    feature = "cuda-12080",
    feature = "cuda-12090",
    feature = "cuda-13000"
))]
#[repr(C)]
#[derive(Copy, Clone)]
pub union cudaGraphKernelNodeUpdate__bindgen_ty_1 {
    pub gridDim: dim3,
    pub param: cudaGraphKernelNodeUpdate__bindgen_ty_1__bindgen_ty_1,
    pub isEnabled: ::core::ffi::c_uint,
}
#[cfg(any(feature = "cuda-12020"))]
#[repr(C)]
#[derive(Copy, Clone)]
pub union cudaGraphNodeParams__bindgen_ty_1 {
    pub reserved1: [::core::ffi::c_longlong; 29usize],
    pub kernel: cudaKernelNodeParamsV2,
    pub memcpy: cudaMemcpyNodeParams,
    pub memset: cudaMemsetParamsV2,
    pub host: cudaHostNodeParamsV2,
    pub graph: cudaChildGraphNodeParams,
    pub eventWait: cudaEventWaitNodeParams,
    pub eventRecord: cudaEventRecordNodeParams,
    pub extSemSignal: cudaExternalSemaphoreSignalNodeParamsV2,
    pub extSemWait: cudaExternalSemaphoreWaitNodeParamsV2,
    pub alloc: cudaMemAllocNodeParamsV2,
    pub free: cudaMemFreeNodeParams,
}
#[cfg(any(
    feature = "cuda-12030",
    feature = "cuda-12040",
    feature = "cuda-12050",
    feature = "cuda-12060",
    feature = "cuda-12080",
    feature = "cuda-12090",
    feature = "cuda-13000"
))]
#[repr(C)]
#[derive(Copy, Clone)]
pub union cudaGraphNodeParams__bindgen_ty_1 {
    pub reserved1: [::core::ffi::c_longlong; 29usize],
    pub kernel: cudaKernelNodeParamsV2,
    pub memcpy: cudaMemcpyNodeParams,
    pub memset: cudaMemsetParamsV2,
    pub host: cudaHostNodeParamsV2,
    pub graph: cudaChildGraphNodeParams,
    pub eventWait: cudaEventWaitNodeParams,
    pub eventRecord: cudaEventRecordNodeParams,
    pub extSemSignal: cudaExternalSemaphoreSignalNodeParamsV2,
    pub extSemWait: cudaExternalSemaphoreWaitNodeParamsV2,
    pub alloc: cudaMemAllocNodeParamsV2,
    pub free: cudaMemFreeNodeParams,
    pub conditional: cudaConditionalNodeParams,
}
#[cfg(any(feature = "cuda-11040", feature = "cuda-11050", feature = "cuda-11060"))]
#[repr(C)]
#[derive(Copy, Clone)]
pub union cudaKernelNodeAttrValue {
    pub accessPolicyWindow: cudaAccessPolicyWindow,
    pub cooperative: ::core::ffi::c_int,
}
#[cfg(any(feature = "cuda-11070"))]
#[repr(C)]
#[derive(Copy, Clone)]
pub union cudaKernelNodeAttrValue {
    pub accessPolicyWindow: cudaAccessPolicyWindow,
    pub cooperative: ::core::ffi::c_int,
    pub priority: ::core::ffi::c_int,
}
#[cfg(any(feature = "cuda-11080"))]
#[repr(C)]
#[derive(Copy, Clone)]
pub union cudaLaunchAttributeValue {
    pub pad: [::core::ffi::c_char; 64usize],
    pub accessPolicyWindow: cudaAccessPolicyWindow,
    pub cooperative: ::core::ffi::c_int,
    pub syncPolicy: cudaSynchronizationPolicy,
    pub clusterDim: cudaLaunchAttributeValue__bindgen_ty_1,
    pub clusterSchedulingPolicyPreference: cudaClusterSchedulingPolicy,
    pub programmaticStreamSerializationAllowed: ::core::ffi::c_int,
    pub programmaticEvent: cudaLaunchAttributeValue__bindgen_ty_2,
    pub priority: ::core::ffi::c_int,
}
#[cfg(any(feature = "cuda-12000", feature = "cuda-12010", feature = "cuda-12020"))]
#[repr(C)]
#[derive(Copy, Clone)]
pub union cudaLaunchAttributeValue {
    pub pad: [::core::ffi::c_char; 64usize],
    pub accessPolicyWindow: cudaAccessPolicyWindow,
    pub cooperative: ::core::ffi::c_int,
    pub syncPolicy: cudaSynchronizationPolicy,
    pub clusterDim: cudaLaunchAttributeValue__bindgen_ty_1,
    pub clusterSchedulingPolicyPreference: cudaClusterSchedulingPolicy,
    pub programmaticStreamSerializationAllowed: ::core::ffi::c_int,
    pub programmaticEvent: cudaLaunchAttributeValue__bindgen_ty_2,
    pub priority: ::core::ffi::c_int,
    pub memSyncDomainMap: cudaLaunchMemSyncDomainMap,
    pub memSyncDomain: cudaLaunchMemSyncDomain,
}
#[cfg(any(feature = "cuda-12030"))]
#[repr(C)]
#[derive(Copy, Clone)]
pub union cudaLaunchAttributeValue {
    pub pad: [::core::ffi::c_char; 64usize],
    pub accessPolicyWindow: cudaAccessPolicyWindow,
    pub cooperative: ::core::ffi::c_int,
    pub syncPolicy: cudaSynchronizationPolicy,
    pub clusterDim: cudaLaunchAttributeValue__bindgen_ty_1,
    pub clusterSchedulingPolicyPreference: cudaClusterSchedulingPolicy,
    pub programmaticStreamSerializationAllowed: ::core::ffi::c_int,
    pub programmaticEvent: cudaLaunchAttributeValue__bindgen_ty_2,
    pub priority: ::core::ffi::c_int,
    pub memSyncDomainMap: cudaLaunchMemSyncDomainMap,
    pub memSyncDomain: cudaLaunchMemSyncDomain,
    pub launchCompletionEvent: cudaLaunchAttributeValue__bindgen_ty_3,
}
#[cfg(any(feature = "cuda-12040"))]
#[repr(C)]
#[derive(Copy, Clone)]
pub union cudaLaunchAttributeValue {
    pub pad: [::core::ffi::c_char; 64usize],
    pub accessPolicyWindow: cudaAccessPolicyWindow,
    pub cooperative: ::core::ffi::c_int,
    pub syncPolicy: cudaSynchronizationPolicy,
    pub clusterDim: cudaLaunchAttributeValue__bindgen_ty_1,
    pub clusterSchedulingPolicyPreference: cudaClusterSchedulingPolicy,
    pub programmaticStreamSerializationAllowed: ::core::ffi::c_int,
    pub programmaticEvent: cudaLaunchAttributeValue__bindgen_ty_2,
    pub priority: ::core::ffi::c_int,
    pub memSyncDomainMap: cudaLaunchMemSyncDomainMap,
    pub memSyncDomain: cudaLaunchMemSyncDomain,
    pub launchCompletionEvent: cudaLaunchAttributeValue__bindgen_ty_3,
    pub deviceUpdatableKernelNode: cudaLaunchAttributeValue__bindgen_ty_4,
}
#[cfg(any(feature = "cuda-12050", feature = "cuda-12060"))]
#[repr(C)]
#[derive(Copy, Clone)]
pub union cudaLaunchAttributeValue {
    pub pad: [::core::ffi::c_char; 64usize],
    pub accessPolicyWindow: cudaAccessPolicyWindow,
    pub cooperative: ::core::ffi::c_int,
    pub syncPolicy: cudaSynchronizationPolicy,
    pub clusterDim: cudaLaunchAttributeValue__bindgen_ty_1,
    pub clusterSchedulingPolicyPreference: cudaClusterSchedulingPolicy,
    pub programmaticStreamSerializationAllowed: ::core::ffi::c_int,
    pub programmaticEvent: cudaLaunchAttributeValue__bindgen_ty_2,
    pub priority: ::core::ffi::c_int,
    pub memSyncDomainMap: cudaLaunchMemSyncDomainMap,
    pub memSyncDomain: cudaLaunchMemSyncDomain,
    pub launchCompletionEvent: cudaLaunchAttributeValue__bindgen_ty_3,
    pub deviceUpdatableKernelNode: cudaLaunchAttributeValue__bindgen_ty_4,
    pub sharedMemCarveout: ::core::ffi::c_uint,
}
#[cfg(any(feature = "cuda-12080", feature = "cuda-12090"))]
#[repr(C)]
#[derive(Copy, Clone)]
pub union cudaLaunchAttributeValue {
    pub pad: [::core::ffi::c_char; 64usize],
    pub accessPolicyWindow: cudaAccessPolicyWindow,
    pub cooperative: ::core::ffi::c_int,
    pub syncPolicy: cudaSynchronizationPolicy,
    pub clusterDim: cudaLaunchAttributeValue__bindgen_ty_1,
    pub clusterSchedulingPolicyPreference: cudaClusterSchedulingPolicy,
    pub programmaticStreamSerializationAllowed: ::core::ffi::c_int,
    pub programmaticEvent: cudaLaunchAttributeValue__bindgen_ty_2,
    pub priority: ::core::ffi::c_int,
    pub memSyncDomainMap: cudaLaunchMemSyncDomainMap,
    pub memSyncDomain: cudaLaunchMemSyncDomain,
    pub preferredClusterDim: cudaLaunchAttributeValue__bindgen_ty_3,
    pub launchCompletionEvent: cudaLaunchAttributeValue__bindgen_ty_4,
    pub deviceUpdatableKernelNode: cudaLaunchAttributeValue__bindgen_ty_5,
    pub sharedMemCarveout: ::core::ffi::c_uint,
}
#[cfg(any(feature = "cuda-13000"))]
#[repr(C)]
#[derive(Copy, Clone)]
pub union cudaLaunchAttributeValue {
    pub pad: [::core::ffi::c_char; 64usize],
    pub accessPolicyWindow: cudaAccessPolicyWindow,
    pub cooperative: ::core::ffi::c_int,
    pub syncPolicy: cudaSynchronizationPolicy,
    pub clusterDim: cudaLaunchAttributeValue__bindgen_ty_1,
    pub clusterSchedulingPolicyPreference: cudaClusterSchedulingPolicy,
    pub programmaticStreamSerializationAllowed: ::core::ffi::c_int,
    pub programmaticEvent: cudaLaunchAttributeValue__bindgen_ty_2,
    pub priority: ::core::ffi::c_int,
    pub memSyncDomainMap: cudaLaunchMemSyncDomainMap,
    pub memSyncDomain: cudaLaunchMemSyncDomain,
    pub preferredClusterDim: cudaLaunchAttributeValue__bindgen_ty_3,
    pub launchCompletionEvent: cudaLaunchAttributeValue__bindgen_ty_4,
    pub deviceUpdatableKernelNode: cudaLaunchAttributeValue__bindgen_ty_5,
    pub sharedMemCarveout: ::core::ffi::c_uint,
    pub nvlinkUtilCentricScheduling: ::core::ffi::c_uint,
}
#[cfg(any(feature = "cuda-12080", feature = "cuda-12090", feature = "cuda-13000"))]
#[repr(C)]
#[derive(Copy, Clone)]
pub union cudaMemcpy3DOperand__bindgen_ty_1 {
    pub ptr: cudaMemcpy3DOperand__bindgen_ty_1__bindgen_ty_1,
    pub array: cudaMemcpy3DOperand__bindgen_ty_1__bindgen_ty_2,
}
#[cfg(any(
    feature = "cuda-11040",
    feature = "cuda-11050",
    feature = "cuda-11060",
    feature = "cuda-11070",
    feature = "cuda-11080",
    feature = "cuda-12000",
    feature = "cuda-12010",
    feature = "cuda-12020",
    feature = "cuda-12030",
    feature = "cuda-12040",
    feature = "cuda-12050",
    feature = "cuda-12060",
    feature = "cuda-12080",
    feature = "cuda-12090"
))]
#[repr(C)]
#[derive(Copy, Clone)]
pub union cudaResourceDesc__bindgen_ty_1 {
    pub array: cudaResourceDesc__bindgen_ty_1__bindgen_ty_1,
    pub mipmap: cudaResourceDesc__bindgen_ty_1__bindgen_ty_2,
    pub linear: cudaResourceDesc__bindgen_ty_1__bindgen_ty_3,
    pub pitch2D: cudaResourceDesc__bindgen_ty_1__bindgen_ty_4,
}
#[cfg(any(feature = "cuda-13000"))]
#[repr(C)]
#[derive(Copy, Clone)]
pub union cudaResourceDesc__bindgen_ty_1 {
    pub array: cudaResourceDesc__bindgen_ty_1__bindgen_ty_1,
    pub mipmap: cudaResourceDesc__bindgen_ty_1__bindgen_ty_2,
    pub linear: cudaResourceDesc__bindgen_ty_1__bindgen_ty_3,
    pub pitch2D: cudaResourceDesc__bindgen_ty_1__bindgen_ty_4,
    pub reserved: cudaResourceDesc__bindgen_ty_1__bindgen_ty_5,
}
#[cfg(any(
    feature = "cuda-11040",
    feature = "cuda-11050",
    feature = "cuda-11060",
    feature = "cuda-11070"
))]
#[repr(C)]
#[derive(Copy, Clone)]
pub union cudaStreamAttrValue {
    pub accessPolicyWindow: cudaAccessPolicyWindow,
    pub syncPolicy: cudaSynchronizationPolicy,
}
#[cfg(not(feature = "dynamic-loading"))]
extern "C" {
    pub fn cudaArrayGetInfo(
        desc: *mut cudaChannelFormatDesc,
        extent: *mut cudaExtent,
        flags: *mut ::core::ffi::c_uint,
        array: cudaArray_t,
    ) -> cudaError_t;
    #[cfg(any(
        feature = "cuda-11060",
        feature = "cuda-11070",
        feature = "cuda-11080",
        feature = "cuda-12000",
        feature = "cuda-12010",
        feature = "cuda-12020",
        feature = "cuda-12030",
        feature = "cuda-12040",
        feature = "cuda-12050",
        feature = "cuda-12060",
        feature = "cuda-12080",
        feature = "cuda-12090",
        feature = "cuda-13000"
    ))]
    pub fn cudaArrayGetMemoryRequirements(
        memoryRequirements: *mut cudaArrayMemoryRequirements,
        array: cudaArray_t,
        device: ::core::ffi::c_int,
    ) -> cudaError_t;
    pub fn cudaArrayGetPlane(
        pPlaneArray: *mut cudaArray_t,
        hArray: cudaArray_t,
        planeIdx: ::core::ffi::c_uint,
    ) -> cudaError_t;
    pub fn cudaArrayGetSparseProperties(
        sparseProperties: *mut cudaArraySparseProperties,
        array: cudaArray_t,
    ) -> cudaError_t;
    #[cfg(any(
        feature = "cuda-11040",
        feature = "cuda-11050",
        feature = "cuda-11060",
        feature = "cuda-11070",
        feature = "cuda-11080"
    ))]
    pub fn cudaBindSurfaceToArray(
        surfref: *const surfaceReference,
        array: cudaArray_const_t,
        desc: *const cudaChannelFormatDesc,
    ) -> cudaError_t;
    #[cfg(any(
        feature = "cuda-11040",
        feature = "cuda-11050",
        feature = "cuda-11060",
        feature = "cuda-11070",
        feature = "cuda-11080"
    ))]
    pub fn cudaBindTexture(
        offset: *mut usize,
        texref: *const textureReference,
        devPtr: *const ::core::ffi::c_void,
        desc: *const cudaChannelFormatDesc,
        size: usize,
    ) -> cudaError_t;
    #[cfg(any(
        feature = "cuda-11040",
        feature = "cuda-11050",
        feature = "cuda-11060",
        feature = "cuda-11070",
        feature = "cuda-11080"
    ))]
    pub fn cudaBindTexture2D(
        offset: *mut usize,
        texref: *const textureReference,
        devPtr: *const ::core::ffi::c_void,
        desc: *const cudaChannelFormatDesc,
        width: usize,
        height: usize,
        pitch: usize,
    ) -> cudaError_t;
    #[cfg(any(
        feature = "cuda-11040",
        feature = "cuda-11050",
        feature = "cuda-11060",
        feature = "cuda-11070",
        feature = "cuda-11080"
    ))]
    pub fn cudaBindTextureToArray(
        texref: *const textureReference,
        array: cudaArray_const_t,
        desc: *const cudaChannelFormatDesc,
    ) -> cudaError_t;
    #[cfg(any(
        feature = "cuda-11040",
        feature = "cuda-11050",
        feature = "cuda-11060",
        feature = "cuda-11070",
        feature = "cuda-11080"
    ))]
    pub fn cudaBindTextureToMipmappedArray(
        texref: *const textureReference,
        mipmappedArray: cudaMipmappedArray_const_t,
        desc: *const cudaChannelFormatDesc,
    ) -> cudaError_t;
    pub fn cudaChooseDevice(
        device: *mut ::core::ffi::c_int,
        prop: *const cudaDeviceProp,
    ) -> cudaError_t;
    pub fn cudaCreateChannelDesc(
        x: ::core::ffi::c_int,
        y: ::core::ffi::c_int,
        z: ::core::ffi::c_int,
        w: ::core::ffi::c_int,
        f: cudaChannelFormatKind,
    ) -> cudaChannelFormatDesc;
    pub fn cudaCreateSurfaceObject(
        pSurfObject: *mut cudaSurfaceObject_t,
        pResDesc: *const cudaResourceDesc,
    ) -> cudaError_t;
    pub fn cudaCreateTextureObject(
        pTexObject: *mut cudaTextureObject_t,
        pResDesc: *const cudaResourceDesc,
        pTexDesc: *const cudaTextureDesc,
        pResViewDesc: *const cudaResourceViewDesc,
    ) -> cudaError_t;
    #[cfg(any(feature = "cuda-11080"))]
    pub fn cudaCreateTextureObject_v2(
        pTexObject: *mut cudaTextureObject_t,
        pResDesc: *const cudaResourceDesc,
        pTexDesc: *const cudaTextureDesc_v2,
        pResViewDesc: *const cudaResourceViewDesc,
    ) -> cudaError_t;
    pub fn cudaCtxResetPersistingL2Cache() -> cudaError_t;
    pub fn cudaDestroyExternalMemory(extMem: cudaExternalMemory_t) -> cudaError_t;
    pub fn cudaDestroyExternalSemaphore(extSem: cudaExternalSemaphore_t) -> cudaError_t;
    pub fn cudaDestroySurfaceObject(surfObject: cudaSurfaceObject_t) -> cudaError_t;
    pub fn cudaDestroyTextureObject(texObject: cudaTextureObject_t) -> cudaError_t;
    pub fn cudaDeviceCanAccessPeer(
        canAccessPeer: *mut ::core::ffi::c_int,
        device: ::core::ffi::c_int,
        peerDevice: ::core::ffi::c_int,
    ) -> cudaError_t;
    pub fn cudaDeviceDisablePeerAccess(peerDevice: ::core::ffi::c_int) -> cudaError_t;
    pub fn cudaDeviceEnablePeerAccess(
        peerDevice: ::core::ffi::c_int,
        flags: ::core::ffi::c_uint,
    ) -> cudaError_t;
    pub fn cudaDeviceFlushGPUDirectRDMAWrites(
        target: cudaFlushGPUDirectRDMAWritesTarget,
        scope: cudaFlushGPUDirectRDMAWritesScope,
    ) -> cudaError_t;
    pub fn cudaDeviceGetAttribute(
        value: *mut ::core::ffi::c_int,
        attr: cudaDeviceAttr,
        device: ::core::ffi::c_int,
    ) -> cudaError_t;
    pub fn cudaDeviceGetByPCIBusId(
        device: *mut ::core::ffi::c_int,
        pciBusId: *const ::core::ffi::c_char,
    ) -> cudaError_t;
    pub fn cudaDeviceGetCacheConfig(pCacheConfig: *mut cudaFuncCache) -> cudaError_t;
    pub fn cudaDeviceGetDefaultMemPool(
        memPool: *mut cudaMemPool_t,
        device: ::core::ffi::c_int,
    ) -> cudaError_t;
    pub fn cudaDeviceGetGraphMemAttribute(
        device: ::core::ffi::c_int,
        attr: cudaGraphMemAttributeType,
        value: *mut ::core::ffi::c_void,
    ) -> cudaError_t;
    #[cfg(any(feature = "cuda-13000"))]
    pub fn cudaDeviceGetHostAtomicCapabilities(
        capabilities: *mut ::core::ffi::c_uint,
        operations: *const cudaAtomicOperation,
        count: ::core::ffi::c_uint,
        device: ::core::ffi::c_int,
    ) -> cudaError_t;
    pub fn cudaDeviceGetLimit(pValue: *mut usize, limit: cudaLimit) -> cudaError_t;
    pub fn cudaDeviceGetMemPool(
        memPool: *mut cudaMemPool_t,
        device: ::core::ffi::c_int,
    ) -> cudaError_t;
    #[cfg(any(feature = "cuda-13000"))]
    pub fn cudaDeviceGetP2PAtomicCapabilities(
        capabilities: *mut ::core::ffi::c_uint,
        operations: *const cudaAtomicOperation,
        count: ::core::ffi::c_uint,
        srcDevice: ::core::ffi::c_int,
        dstDevice: ::core::ffi::c_int,
    ) -> cudaError_t;
    pub fn cudaDeviceGetP2PAttribute(
        value: *mut ::core::ffi::c_int,
        attr: cudaDeviceP2PAttr,
        srcDevice: ::core::ffi::c_int,
        dstDevice: ::core::ffi::c_int,
    ) -> cudaError_t;
    pub fn cudaDeviceGetPCIBusId(
        pciBusId: *mut ::core::ffi::c_char,
        len: ::core::ffi::c_int,
        device: ::core::ffi::c_int,
    ) -> cudaError_t;
    pub fn cudaDeviceGetSharedMemConfig(pConfig: *mut cudaSharedMemConfig) -> cudaError_t;
    pub fn cudaDeviceGetStreamPriorityRange(
        leastPriority: *mut ::core::ffi::c_int,
        greatestPriority: *mut ::core::ffi::c_int,
    ) -> cudaError_t;
    pub fn cudaDeviceGetTexture1DLinearMaxWidth(
        maxWidthInElements: *mut usize,
        fmtDesc: *const cudaChannelFormatDesc,
        device: ::core::ffi::c_int,
    ) -> cudaError_t;
    pub fn cudaDeviceGraphMemTrim(device: ::core::ffi::c_int) -> cudaError_t;
    #[cfg(any(
        feature = "cuda-12040",
        feature = "cuda-12050",
        feature = "cuda-12060",
        feature = "cuda-12080",
        feature = "cuda-12090",
        feature = "cuda-13000"
    ))]
    pub fn cudaDeviceRegisterAsyncNotification(
        device: ::core::ffi::c_int,
        callbackFunc: cudaAsyncCallback,
        userData: *mut ::core::ffi::c_void,
        callback: *mut cudaAsyncCallbackHandle_t,
    ) -> cudaError_t;
    pub fn cudaDeviceReset() -> cudaError_t;
    pub fn cudaDeviceSetCacheConfig(cacheConfig: cudaFuncCache) -> cudaError_t;
    pub fn cudaDeviceSetGraphMemAttribute(
        device: ::core::ffi::c_int,
        attr: cudaGraphMemAttributeType,
        value: *mut ::core::ffi::c_void,
    ) -> cudaError_t;
    pub fn cudaDeviceSetLimit(limit: cudaLimit, value: usize) -> cudaError_t;
    pub fn cudaDeviceSetMemPool(device: ::core::ffi::c_int, memPool: cudaMemPool_t) -> cudaError_t;
    pub fn cudaDeviceSetSharedMemConfig(config: cudaSharedMemConfig) -> cudaError_t;
    pub fn cudaDeviceSynchronize() -> cudaError_t;
    #[cfg(any(
        feature = "cuda-12040",
        feature = "cuda-12050",
        feature = "cuda-12060",
        feature = "cuda-12080",
        feature = "cuda-12090",
        feature = "cuda-13000"
    ))]
    pub fn cudaDeviceUnregisterAsyncNotification(
        device: ::core::ffi::c_int,
        callback: cudaAsyncCallbackHandle_t,
    ) -> cudaError_t;
    pub fn cudaDriverGetVersion(driverVersion: *mut ::core::ffi::c_int) -> cudaError_t;
    pub fn cudaEventCreate(event: *mut cudaEvent_t) -> cudaError_t;
    pub fn cudaEventCreateWithFlags(
        event: *mut cudaEvent_t,
        flags: ::core::ffi::c_uint,
    ) -> cudaError_t;
    pub fn cudaEventDestroy(event: cudaEvent_t) -> cudaError_t;
    pub fn cudaEventElapsedTime(ms: *mut f32, start: cudaEvent_t, end: cudaEvent_t) -> cudaError_t;
    #[cfg(any(feature = "cuda-12080", feature = "cuda-12090"))]
    pub fn cudaEventElapsedTime_v2(
        ms: *mut f32,
        start: cudaEvent_t,
        end: cudaEvent_t,
    ) -> cudaError_t;
    pub fn cudaEventQuery(event: cudaEvent_t) -> cudaError_t;
    pub fn cudaEventRecord(event: cudaEvent_t, stream: cudaStream_t) -> cudaError_t;
    pub fn cudaEventRecordWithFlags(
        event: cudaEvent_t,
        stream: cudaStream_t,
        flags: ::core::ffi::c_uint,
    ) -> cudaError_t;
    pub fn cudaEventSynchronize(event: cudaEvent_t) -> cudaError_t;
    pub fn cudaExternalMemoryGetMappedBuffer(
        devPtr: *mut *mut ::core::ffi::c_void,
        extMem: cudaExternalMemory_t,
        bufferDesc: *const cudaExternalMemoryBufferDesc,
    ) -> cudaError_t;
    pub fn cudaExternalMemoryGetMappedMipmappedArray(
        mipmap: *mut cudaMipmappedArray_t,
        extMem: cudaExternalMemory_t,
        mipmapDesc: *const cudaExternalMemoryMipmappedArrayDesc,
    ) -> cudaError_t;
    pub fn cudaFree(devPtr: *mut ::core::ffi::c_void) -> cudaError_t;
    pub fn cudaFreeArray(array: cudaArray_t) -> cudaError_t;
    pub fn cudaFreeAsync(devPtr: *mut ::core::ffi::c_void, hStream: cudaStream_t) -> cudaError_t;
    pub fn cudaFreeHost(ptr: *mut ::core::ffi::c_void) -> cudaError_t;
    pub fn cudaFreeMipmappedArray(mipmappedArray: cudaMipmappedArray_t) -> cudaError_t;
    pub fn cudaFuncGetAttributes(
        attr: *mut cudaFuncAttributes,
        func: *const ::core::ffi::c_void,
    ) -> cudaError_t;
    #[cfg(any(
        feature = "cuda-12030",
        feature = "cuda-12040",
        feature = "cuda-12050",
        feature = "cuda-12060",
        feature = "cuda-12080",
        feature = "cuda-12090",
        feature = "cuda-13000"
    ))]
    pub fn cudaFuncGetName(
        name: *mut *const ::core::ffi::c_char,
        func: *const ::core::ffi::c_void,
    ) -> cudaError_t;
    #[cfg(any(
        feature = "cuda-12040",
        feature = "cuda-12050",
        feature = "cuda-12060",
        feature = "cuda-12080",
        feature = "cuda-12090",
        feature = "cuda-13000"
    ))]
    pub fn cudaFuncGetParamInfo(
        func: *const ::core::ffi::c_void,
        paramIndex: usize,
        paramOffset: *mut usize,
        paramSize: *mut usize,
    ) -> cudaError_t;
    pub fn cudaFuncSetAttribute(
        func: *const ::core::ffi::c_void,
        attr: cudaFuncAttribute,
        value: ::core::ffi::c_int,
    ) -> cudaError_t;
    pub fn cudaFuncSetCacheConfig(
        func: *const ::core::ffi::c_void,
        cacheConfig: cudaFuncCache,
    ) -> cudaError_t;
    pub fn cudaFuncSetSharedMemConfig(
        func: *const ::core::ffi::c_void,
        config: cudaSharedMemConfig,
    ) -> cudaError_t;
    pub fn cudaGetChannelDesc(
        desc: *mut cudaChannelFormatDesc,
        array: cudaArray_const_t,
    ) -> cudaError_t;
    pub fn cudaGetDevice(device: *mut ::core::ffi::c_int) -> cudaError_t;
    pub fn cudaGetDeviceCount(count: *mut ::core::ffi::c_int) -> cudaError_t;
    pub fn cudaGetDeviceFlags(flags: *mut ::core::ffi::c_uint) -> cudaError_t;
    #[cfg(any(
        feature = "cuda-11040",
        feature = "cuda-11050",
        feature = "cuda-11060",
        feature = "cuda-11070",
        feature = "cuda-11080",
        feature = "cuda-13000"
    ))]
    pub fn cudaGetDeviceProperties(
        prop: *mut cudaDeviceProp,
        device: ::core::ffi::c_int,
    ) -> cudaError_t;
    #[cfg(any(
        feature = "cuda-12000",
        feature = "cuda-12010",
        feature = "cuda-12020",
        feature = "cuda-12030",
        feature = "cuda-12040",
        feature = "cuda-12050",
        feature = "cuda-12060",
        feature = "cuda-12080",
        feature = "cuda-12090"
    ))]
    pub fn cudaGetDeviceProperties_v2(
        prop: *mut cudaDeviceProp,
        device: ::core::ffi::c_int,
    ) -> cudaError_t;
    #[cfg(any(
        feature = "cuda-11040",
        feature = "cuda-11050",
        feature = "cuda-11060",
        feature = "cuda-11070",
        feature = "cuda-11080"
    ))]
    pub fn cudaGetDriverEntryPoint(
        symbol: *const ::core::ffi::c_char,
        funcPtr: *mut *mut ::core::ffi::c_void,
        flags: ::core::ffi::c_ulonglong,
    ) -> cudaError_t;
    #[cfg(any(
        feature = "cuda-12000",
        feature = "cuda-12010",
        feature = "cuda-12020",
        feature = "cuda-12030",
        feature = "cuda-12040",
        feature = "cuda-12050",
        feature = "cuda-12060",
        feature = "cuda-12080",
        feature = "cuda-12090",
        feature = "cuda-13000"
    ))]
    pub fn cudaGetDriverEntryPoint(
        symbol: *const ::core::ffi::c_char,
        funcPtr: *mut *mut ::core::ffi::c_void,
        flags: ::core::ffi::c_ulonglong,
        driverStatus: *mut cudaDriverEntryPointQueryResult,
    ) -> cudaError_t;
    #[cfg(any(
        feature = "cuda-12050",
        feature = "cuda-12060",
        feature = "cuda-12080",
        feature = "cuda-12090",
        feature = "cuda-13000"
    ))]
    pub fn cudaGetDriverEntryPointByVersion(
        symbol: *const ::core::ffi::c_char,
        funcPtr: *mut *mut ::core::ffi::c_void,
        cudaVersion: ::core::ffi::c_uint,
        flags: ::core::ffi::c_ulonglong,
        driverStatus: *mut cudaDriverEntryPointQueryResult,
    ) -> cudaError_t;
    pub fn cudaGetErrorName(error: cudaError_t) -> *const ::core::ffi::c_char;
    pub fn cudaGetErrorString(error: cudaError_t) -> *const ::core::ffi::c_char;
    pub fn cudaGetExportTable(
        ppExportTable: *mut *const ::core::ffi::c_void,
        pExportTableId: *const cudaUUID_t,
    ) -> cudaError_t;
    pub fn cudaGetFuncBySymbol(
        functionPtr: *mut cudaFunction_t,
        symbolPtr: *const ::core::ffi::c_void,
    ) -> cudaError_t;
    #[cfg(any(
        feature = "cuda-12010",
        feature = "cuda-12020",
        feature = "cuda-12030",
        feature = "cuda-12040",
        feature = "cuda-12050",
        feature = "cuda-12060",
        feature = "cuda-12080",
        feature = "cuda-12090",
        feature = "cuda-13000"
    ))]
    pub fn cudaGetKernel(
        kernelPtr: *mut cudaKernel_t,
        entryFuncAddr: *const ::core::ffi::c_void,
    ) -> cudaError_t;
    pub fn cudaGetLastError() -> cudaError_t;
    pub fn cudaGetMipmappedArrayLevel(
        levelArray: *mut cudaArray_t,
        mipmappedArray: cudaMipmappedArray_const_t,
        level: ::core::ffi::c_uint,
    ) -> cudaError_t;
    pub fn cudaGetSurfaceObjectResourceDesc(
        pResDesc: *mut cudaResourceDesc,
        surfObject: cudaSurfaceObject_t,
    ) -> cudaError_t;
    #[cfg(any(
        feature = "cuda-11040",
        feature = "cuda-11050",
        feature = "cuda-11060",
        feature = "cuda-11070",
        feature = "cuda-11080"
    ))]
    pub fn cudaGetSurfaceReference(
        surfref: *mut *const surfaceReference,
        symbol: *const ::core::ffi::c_void,
    ) -> cudaError_t;
    pub fn cudaGetSymbolAddress(
        devPtr: *mut *mut ::core::ffi::c_void,
        symbol: *const ::core::ffi::c_void,
    ) -> cudaError_t;
    pub fn cudaGetSymbolSize(size: *mut usize, symbol: *const ::core::ffi::c_void) -> cudaError_t;
    #[cfg(any(
        feature = "cuda-11040",
        feature = "cuda-11050",
        feature = "cuda-11060",
        feature = "cuda-11070",
        feature = "cuda-11080"
    ))]
    pub fn cudaGetTextureAlignmentOffset(
        offset: *mut usize,
        texref: *const textureReference,
    ) -> cudaError_t;
    pub fn cudaGetTextureObjectResourceDesc(
        pResDesc: *mut cudaResourceDesc,
        texObject: cudaTextureObject_t,
    ) -> cudaError_t;
    pub fn cudaGetTextureObjectResourceViewDesc(
        pResViewDesc: *mut cudaResourceViewDesc,
        texObject: cudaTextureObject_t,
    ) -> cudaError_t;
    pub fn cudaGetTextureObjectTextureDesc(
        pTexDesc: *mut cudaTextureDesc,
        texObject: cudaTextureObject_t,
    ) -> cudaError_t;
    #[cfg(any(feature = "cuda-11080"))]
    pub fn cudaGetTextureObjectTextureDesc_v2(
        pTexDesc: *mut cudaTextureDesc_v2,
        texObject: cudaTextureObject_t,
    ) -> cudaError_t;
    #[cfg(any(
        feature = "cuda-11040",
        feature = "cuda-11050",
        feature = "cuda-11060",
        feature = "cuda-11070",
        feature = "cuda-11080"
    ))]
    pub fn cudaGetTextureReference(
        texref: *mut *const textureReference,
        symbol: *const ::core::ffi::c_void,
    ) -> cudaError_t;
    pub fn cudaGraphAddChildGraphNode(
        pGraphNode: *mut cudaGraphNode_t,
        graph: cudaGraph_t,
        pDependencies: *const cudaGraphNode_t,
        numDependencies: usize,
        childGraph: cudaGraph_t,
    ) -> cudaError_t;
    #[cfg(any(
        feature = "cuda-11040",
        feature = "cuda-11050",
        feature = "cuda-11060",
        feature = "cuda-11070",
        feature = "cuda-11080",
        feature = "cuda-12000",
        feature = "cuda-12010",
        feature = "cuda-12020",
        feature = "cuda-12030",
        feature = "cuda-12040",
        feature = "cuda-12050",
        feature = "cuda-12060",
        feature = "cuda-12080",
        feature = "cuda-12090"
    ))]
    pub fn cudaGraphAddDependencies(
        graph: cudaGraph_t,
        from: *const cudaGraphNode_t,
        to: *const cudaGraphNode_t,
        numDependencies: usize,
    ) -> cudaError_t;
    #[cfg(any(feature = "cuda-13000"))]
    pub fn cudaGraphAddDependencies(
        graph: cudaGraph_t,
        from: *const cudaGraphNode_t,
        to: *const cudaGraphNode_t,
        edgeData: *const cudaGraphEdgeData,
        numDependencies: usize,
    ) -> cudaError_t;
    #[cfg(any(
        feature = "cuda-12030",
        feature = "cuda-12040",
        feature = "cuda-12050",
        feature = "cuda-12060",
        feature = "cuda-12080",
        feature = "cuda-12090"
    ))]
    pub fn cudaGraphAddDependencies_v2(
        graph: cudaGraph_t,
        from: *const cudaGraphNode_t,
        to: *const cudaGraphNode_t,
        edgeData: *const cudaGraphEdgeData,
        numDependencies: usize,
    ) -> cudaError_t;
    pub fn cudaGraphAddEmptyNode(
        pGraphNode: *mut cudaGraphNode_t,
        graph: cudaGraph_t,
        pDependencies: *const cudaGraphNode_t,
        numDependencies: usize,
    ) -> cudaError_t;
    pub fn cudaGraphAddEventRecordNode(
        pGraphNode: *mut cudaGraphNode_t,
        graph: cudaGraph_t,
        pDependencies: *const cudaGraphNode_t,
        numDependencies: usize,
        event: cudaEvent_t,
    ) -> cudaError_t;
    pub fn cudaGraphAddEventWaitNode(
        pGraphNode: *mut cudaGraphNode_t,
        graph: cudaGraph_t,
        pDependencies: *const cudaGraphNode_t,
        numDependencies: usize,
        event: cudaEvent_t,
    ) -> cudaError_t;
    pub fn cudaGraphAddExternalSemaphoresSignalNode(
        pGraphNode: *mut cudaGraphNode_t,
        graph: cudaGraph_t,
        pDependencies: *const cudaGraphNode_t,
        numDependencies: usize,
        nodeParams: *const cudaExternalSemaphoreSignalNodeParams,
    ) -> cudaError_t;
    pub fn cudaGraphAddExternalSemaphoresWaitNode(
        pGraphNode: *mut cudaGraphNode_t,
        graph: cudaGraph_t,
        pDependencies: *const cudaGraphNode_t,
        numDependencies: usize,
        nodeParams: *const cudaExternalSemaphoreWaitNodeParams,
    ) -> cudaError_t;
    pub fn cudaGraphAddHostNode(
        pGraphNode: *mut cudaGraphNode_t,
        graph: cudaGraph_t,
        pDependencies: *const cudaGraphNode_t,
        numDependencies: usize,
        pNodeParams: *const cudaHostNodeParams,
    ) -> cudaError_t;
    pub fn cudaGraphAddKernelNode(
        pGraphNode: *mut cudaGraphNode_t,
        graph: cudaGraph_t,
        pDependencies: *const cudaGraphNode_t,
        numDependencies: usize,
        pNodeParams: *const cudaKernelNodeParams,
    ) -> cudaError_t;
    pub fn cudaGraphAddMemAllocNode(
        pGraphNode: *mut cudaGraphNode_t,
        graph: cudaGraph_t,
        pDependencies: *const cudaGraphNode_t,
        numDependencies: usize,
        nodeParams: *mut cudaMemAllocNodeParams,
    ) -> cudaError_t;
    pub fn cudaGraphAddMemFreeNode(
        pGraphNode: *mut cudaGraphNode_t,
        graph: cudaGraph_t,
        pDependencies: *const cudaGraphNode_t,
        numDependencies: usize,
        dptr: *mut ::core::ffi::c_void,
    ) -> cudaError_t;
    pub fn cudaGraphAddMemcpyNode(
        pGraphNode: *mut cudaGraphNode_t,
        graph: cudaGraph_t,
        pDependencies: *const cudaGraphNode_t,
        numDependencies: usize,
        pCopyParams: *const cudaMemcpy3DParms,
    ) -> cudaError_t;
    pub fn cudaGraphAddMemcpyNode1D(
        pGraphNode: *mut cudaGraphNode_t,
        graph: cudaGraph_t,
        pDependencies: *const cudaGraphNode_t,
        numDependencies: usize,
        dst: *mut ::core::ffi::c_void,
        src: *const ::core::ffi::c_void,
        count: usize,
        kind: cudaMemcpyKind,
    ) -> cudaError_t;
    pub fn cudaGraphAddMemcpyNodeFromSymbol(
        pGraphNode: *mut cudaGraphNode_t,
        graph: cudaGraph_t,
        pDependencies: *const cudaGraphNode_t,
        numDependencies: usize,
        dst: *mut ::core::ffi::c_void,
        symbol: *const ::core::ffi::c_void,
        count: usize,
        offset: usize,
        kind: cudaMemcpyKind,
    ) -> cudaError_t;
    pub fn cudaGraphAddMemcpyNodeToSymbol(
        pGraphNode: *mut cudaGraphNode_t,
        graph: cudaGraph_t,
        pDependencies: *const cudaGraphNode_t,
        numDependencies: usize,
        symbol: *const ::core::ffi::c_void,
        src: *const ::core::ffi::c_void,
        count: usize,
        offset: usize,
        kind: cudaMemcpyKind,
    ) -> cudaError_t;
    pub fn cudaGraphAddMemsetNode(
        pGraphNode: *mut cudaGraphNode_t,
        graph: cudaGraph_t,
        pDependencies: *const cudaGraphNode_t,
        numDependencies: usize,
        pMemsetParams: *const cudaMemsetParams,
    ) -> cudaError_t;
    #[cfg(any(
        feature = "cuda-12020",
        feature = "cuda-12030",
        feature = "cuda-12040",
        feature = "cuda-12050",
        feature = "cuda-12060",
        feature = "cuda-12080",
        feature = "cuda-12090"
    ))]
    pub fn cudaGraphAddNode(
        pGraphNode: *mut cudaGraphNode_t,
        graph: cudaGraph_t,
        pDependencies: *const cudaGraphNode_t,
        numDependencies: usize,
        nodeParams: *mut cudaGraphNodeParams,
    ) -> cudaError_t;
    #[cfg(any(feature = "cuda-13000"))]
    pub fn cudaGraphAddNode(
        pGraphNode: *mut cudaGraphNode_t,
        graph: cudaGraph_t,
        pDependencies: *const cudaGraphNode_t,
        dependencyData: *const cudaGraphEdgeData,
        numDependencies: usize,
        nodeParams: *mut cudaGraphNodeParams,
    ) -> cudaError_t;
    #[cfg(any(
        feature = "cuda-12030",
        feature = "cuda-12040",
        feature = "cuda-12050",
        feature = "cuda-12060",
        feature = "cuda-12080",
        feature = "cuda-12090"
    ))]
    pub fn cudaGraphAddNode_v2(
        pGraphNode: *mut cudaGraphNode_t,
        graph: cudaGraph_t,
        pDependencies: *const cudaGraphNode_t,
        dependencyData: *const cudaGraphEdgeData,
        numDependencies: usize,
        nodeParams: *mut cudaGraphNodeParams,
    ) -> cudaError_t;
    pub fn cudaGraphChildGraphNodeGetGraph(
        node: cudaGraphNode_t,
        pGraph: *mut cudaGraph_t,
    ) -> cudaError_t;
    pub fn cudaGraphClone(pGraphClone: *mut cudaGraph_t, originalGraph: cudaGraph_t)
        -> cudaError_t;
    #[cfg(any(
        feature = "cuda-12030",
        feature = "cuda-12040",
        feature = "cuda-12050",
        feature = "cuda-12060",
        feature = "cuda-12080",
        feature = "cuda-12090",
        feature = "cuda-13000"
    ))]
    pub fn cudaGraphConditionalHandleCreate(
        pHandle_out: *mut cudaGraphConditionalHandle,
        graph: cudaGraph_t,
        defaultLaunchValue: ::core::ffi::c_uint,
        flags: ::core::ffi::c_uint,
    ) -> cudaError_t;
    pub fn cudaGraphCreate(pGraph: *mut cudaGraph_t, flags: ::core::ffi::c_uint) -> cudaError_t;
    pub fn cudaGraphDebugDotPrint(
        graph: cudaGraph_t,
        path: *const ::core::ffi::c_char,
        flags: ::core::ffi::c_uint,
    ) -> cudaError_t;
    pub fn cudaGraphDestroy(graph: cudaGraph_t) -> cudaError_t;
    pub fn cudaGraphDestroyNode(node: cudaGraphNode_t) -> cudaError_t;
    pub fn cudaGraphEventRecordNodeGetEvent(
        node: cudaGraphNode_t,
        event_out: *mut cudaEvent_t,
    ) -> cudaError_t;
    pub fn cudaGraphEventRecordNodeSetEvent(
        node: cudaGraphNode_t,
        event: cudaEvent_t,
    ) -> cudaError_t;
    pub fn cudaGraphEventWaitNodeGetEvent(
        node: cudaGraphNode_t,
        event_out: *mut cudaEvent_t,
    ) -> cudaError_t;
    pub fn cudaGraphEventWaitNodeSetEvent(node: cudaGraphNode_t, event: cudaEvent_t)
        -> cudaError_t;
    pub fn cudaGraphExecChildGraphNodeSetParams(
        hGraphExec: cudaGraphExec_t,
        node: cudaGraphNode_t,
        childGraph: cudaGraph_t,
    ) -> cudaError_t;
    pub fn cudaGraphExecDestroy(graphExec: cudaGraphExec_t) -> cudaError_t;
    pub fn cudaGraphExecEventRecordNodeSetEvent(
        hGraphExec: cudaGraphExec_t,
        hNode: cudaGraphNode_t,
        event: cudaEvent_t,
    ) -> cudaError_t;
    pub fn cudaGraphExecEventWaitNodeSetEvent(
        hGraphExec: cudaGraphExec_t,
        hNode: cudaGraphNode_t,
        event: cudaEvent_t,
    ) -> cudaError_t;
    pub fn cudaGraphExecExternalSemaphoresSignalNodeSetParams(
        hGraphExec: cudaGraphExec_t,
        hNode: cudaGraphNode_t,
        nodeParams: *const cudaExternalSemaphoreSignalNodeParams,
    ) -> cudaError_t;
    pub fn cudaGraphExecExternalSemaphoresWaitNodeSetParams(
        hGraphExec: cudaGraphExec_t,
        hNode: cudaGraphNode_t,
        nodeParams: *const cudaExternalSemaphoreWaitNodeParams,
    ) -> cudaError_t;
    #[cfg(any(
        feature = "cuda-12000",
        feature = "cuda-12010",
        feature = "cuda-12020",
        feature = "cuda-12030",
        feature = "cuda-12040",
        feature = "cuda-12050",
        feature = "cuda-12060",
        feature = "cuda-12080",
        feature = "cuda-12090",
        feature = "cuda-13000"
    ))]
    pub fn cudaGraphExecGetFlags(
        graphExec: cudaGraphExec_t,
        flags: *mut ::core::ffi::c_ulonglong,
    ) -> cudaError_t;
    pub fn cudaGraphExecHostNodeSetParams(
        hGraphExec: cudaGraphExec_t,
        node: cudaGraphNode_t,
        pNodeParams: *const cudaHostNodeParams,
    ) -> cudaError_t;
    pub fn cudaGraphExecKernelNodeSetParams(
        hGraphExec: cudaGraphExec_t,
        node: cudaGraphNode_t,
        pNodeParams: *const cudaKernelNodeParams,
    ) -> cudaError_t;
    pub fn cudaGraphExecMemcpyNodeSetParams(
        hGraphExec: cudaGraphExec_t,
        node: cudaGraphNode_t,
        pNodeParams: *const cudaMemcpy3DParms,
    ) -> cudaError_t;
    pub fn cudaGraphExecMemcpyNodeSetParams1D(
        hGraphExec: cudaGraphExec_t,
        node: cudaGraphNode_t,
        dst: *mut ::core::ffi::c_void,
        src: *const ::core::ffi::c_void,
        count: usize,
        kind: cudaMemcpyKind,
    ) -> cudaError_t;
    pub fn cudaGraphExecMemcpyNodeSetParamsFromSymbol(
        hGraphExec: cudaGraphExec_t,
        node: cudaGraphNode_t,
        dst: *mut ::core::ffi::c_void,
        symbol: *const ::core::ffi::c_void,
        count: usize,
        offset: usize,
        kind: cudaMemcpyKind,
    ) -> cudaError_t;
    pub fn cudaGraphExecMemcpyNodeSetParamsToSymbol(
        hGraphExec: cudaGraphExec_t,
        node: cudaGraphNode_t,
        symbol: *const ::core::ffi::c_void,
        src: *const ::core::ffi::c_void,
        count: usize,
        offset: usize,
        kind: cudaMemcpyKind,
    ) -> cudaError_t;
    pub fn cudaGraphExecMemsetNodeSetParams(
        hGraphExec: cudaGraphExec_t,
        node: cudaGraphNode_t,
        pNodeParams: *const cudaMemsetParams,
    ) -> cudaError_t;
    #[cfg(any(
        feature = "cuda-12020",
        feature = "cuda-12030",
        feature = "cuda-12040",
        feature = "cuda-12050",
        feature = "cuda-12060",
        feature = "cuda-12080",
        feature = "cuda-12090",
        feature = "cuda-13000"
    ))]
    pub fn cudaGraphExecNodeSetParams(
        graphExec: cudaGraphExec_t,
        node: cudaGraphNode_t,
        nodeParams: *mut cudaGraphNodeParams,
    ) -> cudaError_t;
    #[cfg(any(
        feature = "cuda-11040",
        feature = "cuda-11050",
        feature = "cuda-11060",
        feature = "cuda-11070",
        feature = "cuda-11080"
    ))]
    pub fn cudaGraphExecUpdate(
        hGraphExec: cudaGraphExec_t,
        hGraph: cudaGraph_t,
        hErrorNode_out: *mut cudaGraphNode_t,
        updateResult_out: *mut cudaGraphExecUpdateResult,
    ) -> cudaError_t;
    #[cfg(any(
        feature = "cuda-12000",
        feature = "cuda-12010",
        feature = "cuda-12020",
        feature = "cuda-12030",
        feature = "cuda-12040",
        feature = "cuda-12050",
        feature = "cuda-12060",
        feature = "cuda-12080",
        feature = "cuda-12090",
        feature = "cuda-13000"
    ))]
    pub fn cudaGraphExecUpdate(
        hGraphExec: cudaGraphExec_t,
        hGraph: cudaGraph_t,
        resultInfo: *mut cudaGraphExecUpdateResultInfo,
    ) -> cudaError_t;
    pub fn cudaGraphExternalSemaphoresSignalNodeGetParams(
        hNode: cudaGraphNode_t,
        params_out: *mut cudaExternalSemaphoreSignalNodeParams,
    ) -> cudaError_t;
    pub fn cudaGraphExternalSemaphoresSignalNodeSetParams(
        hNode: cudaGraphNode_t,
        nodeParams: *const cudaExternalSemaphoreSignalNodeParams,
    ) -> cudaError_t;
    pub fn cudaGraphExternalSemaphoresWaitNodeGetParams(
        hNode: cudaGraphNode_t,
        params_out: *mut cudaExternalSemaphoreWaitNodeParams,
    ) -> cudaError_t;
    pub fn cudaGraphExternalSemaphoresWaitNodeSetParams(
        hNode: cudaGraphNode_t,
        nodeParams: *const cudaExternalSemaphoreWaitNodeParams,
    ) -> cudaError_t;
    #[cfg(any(
        feature = "cuda-11040",
        feature = "cuda-11050",
        feature = "cuda-11060",
        feature = "cuda-11070",
        feature = "cuda-11080",
        feature = "cuda-12000",
        feature = "cuda-12010",
        feature = "cuda-12020",
        feature = "cuda-12030",
        feature = "cuda-12040",
        feature = "cuda-12050",
        feature = "cuda-12060",
        feature = "cuda-12080",
        feature = "cuda-12090"
    ))]
    pub fn cudaGraphGetEdges(
        graph: cudaGraph_t,
        from: *mut cudaGraphNode_t,
        to: *mut cudaGraphNode_t,
        numEdges: *mut usize,
    ) -> cudaError_t;
    #[cfg(any(feature = "cuda-13000"))]
    pub fn cudaGraphGetEdges(
        graph: cudaGraph_t,
        from: *mut cudaGraphNode_t,
        to: *mut cudaGraphNode_t,
        edgeData: *mut cudaGraphEdgeData,
        numEdges: *mut usize,
    ) -> cudaError_t;
    #[cfg(any(
        feature = "cuda-12030",
        feature = "cuda-12040",
        feature = "cuda-12050",
        feature = "cuda-12060",
        feature = "cuda-12080",
        feature = "cuda-12090"
    ))]
    pub fn cudaGraphGetEdges_v2(
        graph: cudaGraph_t,
        from: *mut cudaGraphNode_t,
        to: *mut cudaGraphNode_t,
        edgeData: *mut cudaGraphEdgeData,
        numEdges: *mut usize,
    ) -> cudaError_t;
    pub fn cudaGraphGetNodes(
        graph: cudaGraph_t,
        nodes: *mut cudaGraphNode_t,
        numNodes: *mut usize,
    ) -> cudaError_t;
    pub fn cudaGraphGetRootNodes(
        graph: cudaGraph_t,
        pRootNodes: *mut cudaGraphNode_t,
        pNumRootNodes: *mut usize,
    ) -> cudaError_t;
    pub fn cudaGraphHostNodeGetParams(
        node: cudaGraphNode_t,
        pNodeParams: *mut cudaHostNodeParams,
    ) -> cudaError_t;
    pub fn cudaGraphHostNodeSetParams(
        node: cudaGraphNode_t,
        pNodeParams: *const cudaHostNodeParams,
    ) -> cudaError_t;
    #[cfg(any(
        feature = "cuda-11040",
        feature = "cuda-11050",
        feature = "cuda-11060",
        feature = "cuda-11070",
        feature = "cuda-11080"
    ))]
    pub fn cudaGraphInstantiate(
        pGraphExec: *mut cudaGraphExec_t,
        graph: cudaGraph_t,
        pErrorNode: *mut cudaGraphNode_t,
        pLogBuffer: *mut ::core::ffi::c_char,
        bufferSize: usize,
    ) -> cudaError_t;
    #[cfg(any(
        feature = "cuda-12000",
        feature = "cuda-12010",
        feature = "cuda-12020",
        feature = "cuda-12030",
        feature = "cuda-12040",
        feature = "cuda-12050",
        feature = "cuda-12060",
        feature = "cuda-12080",
        feature = "cuda-12090",
        feature = "cuda-13000"
    ))]
    pub fn cudaGraphInstantiate(
        pGraphExec: *mut cudaGraphExec_t,
        graph: cudaGraph_t,
        flags: ::core::ffi::c_ulonglong,
    ) -> cudaError_t;
    pub fn cudaGraphInstantiateWithFlags(
        pGraphExec: *mut cudaGraphExec_t,
        graph: cudaGraph_t,
        flags: ::core::ffi::c_ulonglong,
    ) -> cudaError_t;
    #[cfg(any(
        feature = "cuda-12000",
        feature = "cuda-12010",
        feature = "cuda-12020",
        feature = "cuda-12030",
        feature = "cuda-12040",
        feature = "cuda-12050",
        feature = "cuda-12060",
        feature = "cuda-12080",
        feature = "cuda-12090",
        feature = "cuda-13000"
    ))]
    pub fn cudaGraphInstantiateWithParams(
        pGraphExec: *mut cudaGraphExec_t,
        graph: cudaGraph_t,
        instantiateParams: *mut cudaGraphInstantiateParams,
    ) -> cudaError_t;
    #[cfg(any(
        feature = "cuda-11040",
        feature = "cuda-11050",
        feature = "cuda-11060",
        feature = "cuda-11070",
        feature = "cuda-11080",
        feature = "cuda-12000",
        feature = "cuda-12010",
        feature = "cuda-12020",
        feature = "cuda-12030",
        feature = "cuda-12040",
        feature = "cuda-12050",
        feature = "cuda-12060",
        feature = "cuda-12080",
        feature = "cuda-12090"
    ))]
    pub fn cudaGraphKernelNodeCopyAttributes(
        hSrc: cudaGraphNode_t,
        hDst: cudaGraphNode_t,
    ) -> cudaError_t;
    #[cfg(any(feature = "cuda-13000"))]
    pub fn cudaGraphKernelNodeCopyAttributes(
        hDst: cudaGraphNode_t,
        hSrc: cudaGraphNode_t,
    ) -> cudaError_t;
    #[cfg(any(
        feature = "cuda-11040",
        feature = "cuda-11050",
        feature = "cuda-11060",
        feature = "cuda-11070"
    ))]
    pub fn cudaGraphKernelNodeGetAttribute(
        hNode: cudaGraphNode_t,
        attr: cudaKernelNodeAttrID,
        value_out: *mut cudaKernelNodeAttrValue,
    ) -> cudaError_t;
    #[cfg(any(
        feature = "cuda-11080",
        feature = "cuda-12000",
        feature = "cuda-12010",
        feature = "cuda-12020",
        feature = "cuda-12030",
        feature = "cuda-12040",
        feature = "cuda-12050",
        feature = "cuda-12060",
        feature = "cuda-12080",
        feature = "cuda-12090",
        feature = "cuda-13000"
    ))]
    pub fn cudaGraphKernelNodeGetAttribute(
        hNode: cudaGraphNode_t,
        attr: cudaLaunchAttributeID,
        value_out: *mut cudaLaunchAttributeValue,
    ) -> cudaError_t;
    pub fn cudaGraphKernelNodeGetParams(
        node: cudaGraphNode_t,
        pNodeParams: *mut cudaKernelNodeParams,
    ) -> cudaError_t;
    #[cfg(any(
        feature = "cuda-11040",
        feature = "cuda-11050",
        feature = "cuda-11060",
        feature = "cuda-11070"
    ))]
    pub fn cudaGraphKernelNodeSetAttribute(
        hNode: cudaGraphNode_t,
        attr: cudaKernelNodeAttrID,
        value: *const cudaKernelNodeAttrValue,
    ) -> cudaError_t;
    #[cfg(any(
        feature = "cuda-11080",
        feature = "cuda-12000",
        feature = "cuda-12010",
        feature = "cuda-12020",
        feature = "cuda-12030",
        feature = "cuda-12040",
        feature = "cuda-12050",
        feature = "cuda-12060",
        feature = "cuda-12080",
        feature = "cuda-12090",
        feature = "cuda-13000"
    ))]
    pub fn cudaGraphKernelNodeSetAttribute(
        hNode: cudaGraphNode_t,
        attr: cudaLaunchAttributeID,
        value: *const cudaLaunchAttributeValue,
    ) -> cudaError_t;
    pub fn cudaGraphKernelNodeSetParams(
        node: cudaGraphNode_t,
        pNodeParams: *const cudaKernelNodeParams,
    ) -> cudaError_t;
    pub fn cudaGraphLaunch(graphExec: cudaGraphExec_t, stream: cudaStream_t) -> cudaError_t;
    pub fn cudaGraphMemAllocNodeGetParams(
        node: cudaGraphNode_t,
        params_out: *mut cudaMemAllocNodeParams,
    ) -> cudaError_t;
    pub fn cudaGraphMemFreeNodeGetParams(
        node: cudaGraphNode_t,
        dptr_out: *mut ::core::ffi::c_void,
    ) -> cudaError_t;
    pub fn cudaGraphMemcpyNodeGetParams(
        node: cudaGraphNode_t,
        pNodeParams: *mut cudaMemcpy3DParms,
    ) -> cudaError_t;
    pub fn cudaGraphMemcpyNodeSetParams(
        node: cudaGraphNode_t,
        pNodeParams: *const cudaMemcpy3DParms,
    ) -> cudaError_t;
    pub fn cudaGraphMemcpyNodeSetParams1D(
        node: cudaGraphNode_t,
        dst: *mut ::core::ffi::c_void,
        src: *const ::core::ffi::c_void,
        count: usize,
        kind: cudaMemcpyKind,
    ) -> cudaError_t;
    pub fn cudaGraphMemcpyNodeSetParamsFromSymbol(
        node: cudaGraphNode_t,
        dst: *mut ::core::ffi::c_void,
        symbol: *const ::core::ffi::c_void,
        count: usize,
        offset: usize,
        kind: cudaMemcpyKind,
    ) -> cudaError_t;
    pub fn cudaGraphMemcpyNodeSetParamsToSymbol(
        node: cudaGraphNode_t,
        symbol: *const ::core::ffi::c_void,
        src: *const ::core::ffi::c_void,
        count: usize,
        offset: usize,
        kind: cudaMemcpyKind,
    ) -> cudaError_t;
    pub fn cudaGraphMemsetNodeGetParams(
        node: cudaGraphNode_t,
        pNodeParams: *mut cudaMemsetParams,
    ) -> cudaError_t;
    pub fn cudaGraphMemsetNodeSetParams(
        node: cudaGraphNode_t,
        pNodeParams: *const cudaMemsetParams,
    ) -> cudaError_t;
    pub fn cudaGraphNodeFindInClone(
        pNode: *mut cudaGraphNode_t,
        originalNode: cudaGraphNode_t,
        clonedGraph: cudaGraph_t,
    ) -> cudaError_t;
    #[cfg(any(
        feature = "cuda-11040",
        feature = "cuda-11050",
        feature = "cuda-11060",
        feature = "cuda-11070",
        feature = "cuda-11080",
        feature = "cuda-12000",
        feature = "cuda-12010",
        feature = "cuda-12020",
        feature = "cuda-12030",
        feature = "cuda-12040",
        feature = "cuda-12050",
        feature = "cuda-12060",
        feature = "cuda-12080",
        feature = "cuda-12090"
    ))]
    pub fn cudaGraphNodeGetDependencies(
        node: cudaGraphNode_t,
        pDependencies: *mut cudaGraphNode_t,
        pNumDependencies: *mut usize,
    ) -> cudaError_t;
    #[cfg(any(feature = "cuda-13000"))]
    pub fn cudaGraphNodeGetDependencies(
        node: cudaGraphNode_t,
        pDependencies: *mut cudaGraphNode_t,
        edgeData: *mut cudaGraphEdgeData,
        pNumDependencies: *mut usize,
    ) -> cudaError_t;
    #[cfg(any(
        feature = "cuda-12030",
        feature = "cuda-12040",
        feature = "cuda-12050",
        feature = "cuda-12060",
        feature = "cuda-12080",
        feature = "cuda-12090"
    ))]
    pub fn cudaGraphNodeGetDependencies_v2(
        node: cudaGraphNode_t,
        pDependencies: *mut cudaGraphNode_t,
        edgeData: *mut cudaGraphEdgeData,
        pNumDependencies: *mut usize,
    ) -> cudaError_t;
    #[cfg(any(
        feature = "cuda-11040",
        feature = "cuda-11050",
        feature = "cuda-11060",
        feature = "cuda-11070",
        feature = "cuda-11080",
        feature = "cuda-12000",
        feature = "cuda-12010",
        feature = "cuda-12020",
        feature = "cuda-12030",
        feature = "cuda-12040",
        feature = "cuda-12050",
        feature = "cuda-12060",
        feature = "cuda-12080",
        feature = "cuda-12090"
    ))]
    pub fn cudaGraphNodeGetDependentNodes(
        node: cudaGraphNode_t,
        pDependentNodes: *mut cudaGraphNode_t,
        pNumDependentNodes: *mut usize,
    ) -> cudaError_t;
    #[cfg(any(feature = "cuda-13000"))]
    pub fn cudaGraphNodeGetDependentNodes(
        node: cudaGraphNode_t,
        pDependentNodes: *mut cudaGraphNode_t,
        edgeData: *mut cudaGraphEdgeData,
        pNumDependentNodes: *mut usize,
    ) -> cudaError_t;
    #[cfg(any(
        feature = "cuda-12030",
        feature = "cuda-12040",
        feature = "cuda-12050",
        feature = "cuda-12060",
        feature = "cuda-12080",
        feature = "cuda-12090"
    ))]
    pub fn cudaGraphNodeGetDependentNodes_v2(
        node: cudaGraphNode_t,
        pDependentNodes: *mut cudaGraphNode_t,
        edgeData: *mut cudaGraphEdgeData,
        pNumDependentNodes: *mut usize,
    ) -> cudaError_t;
    #[cfg(any(
        feature = "cuda-11060",
        feature = "cuda-11070",
        feature = "cuda-11080",
        feature = "cuda-12000",
        feature = "cuda-12010",
        feature = "cuda-12020",
        feature = "cuda-12030",
        feature = "cuda-12040",
        feature = "cuda-12050",
        feature = "cuda-12060",
        feature = "cuda-12080",
        feature = "cuda-12090",
        feature = "cuda-13000"
    ))]
    pub fn cudaGraphNodeGetEnabled(
        hGraphExec: cudaGraphExec_t,
        hNode: cudaGraphNode_t,
        isEnabled: *mut ::core::ffi::c_uint,
    ) -> cudaError_t;
    pub fn cudaGraphNodeGetType(
        node: cudaGraphNode_t,
        pType: *mut cudaGraphNodeType,
    ) -> cudaError_t;
    #[cfg(any(
        feature = "cuda-11060",
        feature = "cuda-11070",
        feature = "cuda-11080",
        feature = "cuda-12000",
        feature = "cuda-12010",
        feature = "cuda-12020",
        feature = "cuda-12030",
        feature = "cuda-12040",
        feature = "cuda-12050",
        feature = "cuda-12060",
        feature = "cuda-12080",
        feature = "cuda-12090",
        feature = "cuda-13000"
    ))]
    pub fn cudaGraphNodeSetEnabled(
        hGraphExec: cudaGraphExec_t,
        hNode: cudaGraphNode_t,
        isEnabled: ::core::ffi::c_uint,
    ) -> cudaError_t;
    #[cfg(any(
        feature = "cuda-12020",
        feature = "cuda-12030",
        feature = "cuda-12040",
        feature = "cuda-12050",
        feature = "cuda-12060",
        feature = "cuda-12080",
        feature = "cuda-12090",
        feature = "cuda-13000"
    ))]
    pub fn cudaGraphNodeSetParams(
        node: cudaGraphNode_t,
        nodeParams: *mut cudaGraphNodeParams,
    ) -> cudaError_t;
    pub fn cudaGraphReleaseUserObject(
        graph: cudaGraph_t,
        object: cudaUserObject_t,
        count: ::core::ffi::c_uint,
    ) -> cudaError_t;
    #[cfg(any(
        feature = "cuda-11040",
        feature = "cuda-11050",
        feature = "cuda-11060",
        feature = "cuda-11070",
        feature = "cuda-11080",
        feature = "cuda-12000",
        feature = "cuda-12010",
        feature = "cuda-12020",
        feature = "cuda-12030",
        feature = "cuda-12040",
        feature = "cuda-12050",
        feature = "cuda-12060",
        feature = "cuda-12080",
        feature = "cuda-12090"
    ))]
    pub fn cudaGraphRemoveDependencies(
        graph: cudaGraph_t,
        from: *const cudaGraphNode_t,
        to: *const cudaGraphNode_t,
        numDependencies: usize,
    ) -> cudaError_t;
    #[cfg(any(feature = "cuda-13000"))]
    pub fn cudaGraphRemoveDependencies(
        graph: cudaGraph_t,
        from: *const cudaGraphNode_t,
        to: *const cudaGraphNode_t,
        edgeData: *const cudaGraphEdgeData,
        numDependencies: usize,
    ) -> cudaError_t;
    #[cfg(any(
        feature = "cuda-12030",
        feature = "cuda-12040",
        feature = "cuda-12050",
        feature = "cuda-12060",
        feature = "cuda-12080",
        feature = "cuda-12090"
    ))]
    pub fn cudaGraphRemoveDependencies_v2(
        graph: cudaGraph_t,
        from: *const cudaGraphNode_t,
        to: *const cudaGraphNode_t,
        edgeData: *const cudaGraphEdgeData,
        numDependencies: usize,
    ) -> cudaError_t;
    pub fn cudaGraphRetainUserObject(
        graph: cudaGraph_t,
        object: cudaUserObject_t,
        count: ::core::ffi::c_uint,
        flags: ::core::ffi::c_uint,
    ) -> cudaError_t;
    pub fn cudaGraphUpload(graphExec: cudaGraphExec_t, stream: cudaStream_t) -> cudaError_t;
    pub fn cudaGraphicsMapResources(
        count: ::core::ffi::c_int,
        resources: *mut cudaGraphicsResource_t,
        stream: cudaStream_t,
    ) -> cudaError_t;
    pub fn cudaGraphicsResourceGetMappedMipmappedArray(
        mipmappedArray: *mut cudaMipmappedArray_t,
        resource: cudaGraphicsResource_t,
    ) -> cudaError_t;
    pub fn cudaGraphicsResourceGetMappedPointer(
        devPtr: *mut *mut ::core::ffi::c_void,
        size: *mut usize,
        resource: cudaGraphicsResource_t,
    ) -> cudaError_t;
    pub fn cudaGraphicsResourceSetMapFlags(
        resource: cudaGraphicsResource_t,
        flags: ::core::ffi::c_uint,
    ) -> cudaError_t;
    pub fn cudaGraphicsSubResourceGetMappedArray(
        array: *mut cudaArray_t,
        resource: cudaGraphicsResource_t,
        arrayIndex: ::core::ffi::c_uint,
        mipLevel: ::core::ffi::c_uint,
    ) -> cudaError_t;
    pub fn cudaGraphicsUnmapResources(
        count: ::core::ffi::c_int,
        resources: *mut cudaGraphicsResource_t,
        stream: cudaStream_t,
    ) -> cudaError_t;
    pub fn cudaGraphicsUnregisterResource(resource: cudaGraphicsResource_t) -> cudaError_t;
    pub fn cudaHostAlloc(
        pHost: *mut *mut ::core::ffi::c_void,
        size: usize,
        flags: ::core::ffi::c_uint,
    ) -> cudaError_t;
    pub fn cudaHostGetDevicePointer(
        pDevice: *mut *mut ::core::ffi::c_void,
        pHost: *mut ::core::ffi::c_void,
        flags: ::core::ffi::c_uint,
    ) -> cudaError_t;
    pub fn cudaHostGetFlags(
        pFlags: *mut ::core::ffi::c_uint,
        pHost: *mut ::core::ffi::c_void,
    ) -> cudaError_t;
    pub fn cudaHostRegister(
        ptr: *mut ::core::ffi::c_void,
        size: usize,
        flags: ::core::ffi::c_uint,
    ) -> cudaError_t;
    pub fn cudaHostUnregister(ptr: *mut ::core::ffi::c_void) -> cudaError_t;
    pub fn cudaImportExternalMemory(
        extMem_out: *mut cudaExternalMemory_t,
        memHandleDesc: *const cudaExternalMemoryHandleDesc,
    ) -> cudaError_t;
    pub fn cudaImportExternalSemaphore(
        extSem_out: *mut cudaExternalSemaphore_t,
        semHandleDesc: *const cudaExternalSemaphoreHandleDesc,
    ) -> cudaError_t;
    #[cfg(any(
        feature = "cuda-12000",
        feature = "cuda-12010",
        feature = "cuda-12020",
        feature = "cuda-12030",
        feature = "cuda-12040",
        feature = "cuda-12050",
        feature = "cuda-12060",
        feature = "cuda-12080",
        feature = "cuda-12090",
        feature = "cuda-13000"
    ))]
    pub fn cudaInitDevice(
        device: ::core::ffi::c_int,
        deviceFlags: ::core::ffi::c_uint,
        flags: ::core::ffi::c_uint,
    ) -> cudaError_t;
    pub fn cudaIpcCloseMemHandle(devPtr: *mut ::core::ffi::c_void) -> cudaError_t;
    pub fn cudaIpcGetEventHandle(
        handle: *mut cudaIpcEventHandle_t,
        event: cudaEvent_t,
    ) -> cudaError_t;
    pub fn cudaIpcGetMemHandle(
        handle: *mut cudaIpcMemHandle_t,
        devPtr: *mut ::core::ffi::c_void,
    ) -> cudaError_t;
    pub fn cudaIpcOpenEventHandle(
        event: *mut cudaEvent_t,
        handle: cudaIpcEventHandle_t,
    ) -> cudaError_t;
    pub fn cudaIpcOpenMemHandle(
        devPtr: *mut *mut ::core::ffi::c_void,
        handle: cudaIpcMemHandle_t,
        flags: ::core::ffi::c_uint,
    ) -> cudaError_t;
    #[cfg(any(feature = "cuda-12080", feature = "cuda-12090", feature = "cuda-13000"))]
    pub fn cudaKernelSetAttributeForDevice(
        kernel: cudaKernel_t,
        attr: cudaFuncAttribute,
        value: ::core::ffi::c_int,
        device: ::core::ffi::c_int,
    ) -> cudaError_t;
    pub fn cudaLaunchCooperativeKernel(
        func: *const ::core::ffi::c_void,
        gridDim: dim3,
        blockDim: dim3,
        args: *mut *mut ::core::ffi::c_void,
        sharedMem: usize,
        stream: cudaStream_t,
    ) -> cudaError_t;
    #[cfg(any(
        feature = "cuda-11040",
        feature = "cuda-11050",
        feature = "cuda-11060",
        feature = "cuda-11070",
        feature = "cuda-11080",
        feature = "cuda-12000",
        feature = "cuda-12010",
        feature = "cuda-12020",
        feature = "cuda-12030",
        feature = "cuda-12040",
        feature = "cuda-12050",
        feature = "cuda-12060",
        feature = "cuda-12080",
        feature = "cuda-12090"
    ))]
    pub fn cudaLaunchCooperativeKernelMultiDevice(
        launchParamsList: *mut cudaLaunchParams,
        numDevices: ::core::ffi::c_uint,
        flags: ::core::ffi::c_uint,
    ) -> cudaError_t;
    pub fn cudaLaunchHostFunc(
        stream: cudaStream_t,
        fn_: cudaHostFn_t,
        userData: *mut ::core::ffi::c_void,
    ) -> cudaError_t;
    pub fn cudaLaunchKernel(
        func: *const ::core::ffi::c_void,
        gridDim: dim3,
        blockDim: dim3,
        args: *mut *mut ::core::ffi::c_void,
        sharedMem: usize,
        stream: cudaStream_t,
    ) -> cudaError_t;
    #[cfg(any(
        feature = "cuda-11080",
        feature = "cuda-12000",
        feature = "cuda-12010",
        feature = "cuda-12020",
        feature = "cuda-12030",
        feature = "cuda-12040",
        feature = "cuda-12050",
        feature = "cuda-12060",
        feature = "cuda-12080",
        feature = "cuda-12090",
        feature = "cuda-13000"
    ))]
    pub fn cudaLaunchKernelExC(
        config: *const cudaLaunchConfig_t,
        func: *const ::core::ffi::c_void,
        args: *mut *mut ::core::ffi::c_void,
    ) -> cudaError_t;
    #[cfg(any(feature = "cuda-12080", feature = "cuda-12090", feature = "cuda-13000"))]
    pub fn cudaLibraryEnumerateKernels(
        kernels: *mut cudaKernel_t,
        numKernels: ::core::ffi::c_uint,
        lib: cudaLibrary_t,
    ) -> cudaError_t;
    #[cfg(any(feature = "cuda-12080", feature = "cuda-12090", feature = "cuda-13000"))]
    pub fn cudaLibraryGetGlobal(
        dptr: *mut *mut ::core::ffi::c_void,
        bytes: *mut usize,
        library: cudaLibrary_t,
        name: *const ::core::ffi::c_char,
    ) -> cudaError_t;
    #[cfg(any(feature = "cuda-12080", feature = "cuda-12090", feature = "cuda-13000"))]
    pub fn cudaLibraryGetKernel(
        pKernel: *mut cudaKernel_t,
        library: cudaLibrary_t,
        name: *const ::core::ffi::c_char,
    ) -> cudaError_t;
    #[cfg(any(feature = "cuda-12080", feature = "cuda-12090", feature = "cuda-13000"))]
    pub fn cudaLibraryGetKernelCount(
        count: *mut ::core::ffi::c_uint,
        lib: cudaLibrary_t,
    ) -> cudaError_t;
    #[cfg(any(feature = "cuda-12080", feature = "cuda-12090", feature = "cuda-13000"))]
    pub fn cudaLibraryGetManaged(
        dptr: *mut *mut ::core::ffi::c_void,
        bytes: *mut usize,
        library: cudaLibrary_t,
        name: *const ::core::ffi::c_char,
    ) -> cudaError_t;
    #[cfg(any(feature = "cuda-12080", feature = "cuda-12090", feature = "cuda-13000"))]
    pub fn cudaLibraryGetUnifiedFunction(
        fptr: *mut *mut ::core::ffi::c_void,
        library: cudaLibrary_t,
        symbol: *const ::core::ffi::c_char,
    ) -> cudaError_t;
    #[cfg(any(feature = "cuda-12080", feature = "cuda-12090", feature = "cuda-13000"))]
    pub fn cudaLibraryLoadData(
        library: *mut cudaLibrary_t,
        code: *const ::core::ffi::c_void,
        jitOptions: *mut cudaJitOption,
        jitOptionsValues: *mut *mut ::core::ffi::c_void,
        numJitOptions: ::core::ffi::c_uint,
        libraryOptions: *mut cudaLibraryOption,
        libraryOptionValues: *mut *mut ::core::ffi::c_void,
        numLibraryOptions: ::core::ffi::c_uint,
    ) -> cudaError_t;
    #[cfg(any(feature = "cuda-12080", feature = "cuda-12090", feature = "cuda-13000"))]
    pub fn cudaLibraryLoadFromFile(
        library: *mut cudaLibrary_t,
        fileName: *const ::core::ffi::c_char,
        jitOptions: *mut cudaJitOption,
        jitOptionsValues: *mut *mut ::core::ffi::c_void,
        numJitOptions: ::core::ffi::c_uint,
        libraryOptions: *mut cudaLibraryOption,
        libraryOptionValues: *mut *mut ::core::ffi::c_void,
        numLibraryOptions: ::core::ffi::c_uint,
    ) -> cudaError_t;
    #[cfg(any(feature = "cuda-12080", feature = "cuda-12090", feature = "cuda-13000"))]
    pub fn cudaLibraryUnload(library: cudaLibrary_t) -> cudaError_t;
    #[cfg(any(feature = "cuda-13000"))]
    pub fn cudaLogsCurrent(
        iterator_out: *mut cudaLogIterator,
        flags: ::core::ffi::c_uint,
    ) -> cudaError_t;
    #[cfg(any(feature = "cuda-13000"))]
    pub fn cudaLogsDumpToFile(
        iterator: *mut cudaLogIterator,
        pathToFile: *const ::core::ffi::c_char,
        flags: ::core::ffi::c_uint,
    ) -> cudaError_t;
    #[cfg(any(feature = "cuda-13000"))]
    pub fn cudaLogsDumpToMemory(
        iterator: *mut cudaLogIterator,
        buffer: *mut ::core::ffi::c_char,
        size: *mut usize,
        flags: ::core::ffi::c_uint,
    ) -> cudaError_t;
    #[cfg(any(feature = "cuda-13000"))]
    pub fn cudaLogsRegisterCallback(
        callbackFunc: cudaLogsCallback_t,
        userData: *mut ::core::ffi::c_void,
        callback_out: *mut cudaLogsCallbackHandle,
    ) -> cudaError_t;
    #[cfg(any(feature = "cuda-13000"))]
    pub fn cudaLogsUnregisterCallback(callback: cudaLogsCallbackHandle) -> cudaError_t;
    pub fn cudaMalloc(devPtr: *mut *mut ::core::ffi::c_void, size: usize) -> cudaError_t;
    pub fn cudaMalloc3D(pitchedDevPtr: *mut cudaPitchedPtr, extent: cudaExtent) -> cudaError_t;
    pub fn cudaMalloc3DArray(
        array: *mut cudaArray_t,
        desc: *const cudaChannelFormatDesc,
        extent: cudaExtent,
        flags: ::core::ffi::c_uint,
    ) -> cudaError_t;
    pub fn cudaMallocArray(
        array: *mut cudaArray_t,
        desc: *const cudaChannelFormatDesc,
        width: usize,
        height: usize,
        flags: ::core::ffi::c_uint,
    ) -> cudaError_t;
    pub fn cudaMallocAsync(
        devPtr: *mut *mut ::core::ffi::c_void,
        size: usize,
        hStream: cudaStream_t,
    ) -> cudaError_t;
    pub fn cudaMallocFromPoolAsync(
        ptr: *mut *mut ::core::ffi::c_void,
        size: usize,
        memPool: cudaMemPool_t,
        stream: cudaStream_t,
    ) -> cudaError_t;
    pub fn cudaMallocHost(ptr: *mut *mut ::core::ffi::c_void, size: usize) -> cudaError_t;
    pub fn cudaMallocManaged(
        devPtr: *mut *mut ::core::ffi::c_void,
        size: usize,
        flags: ::core::ffi::c_uint,
    ) -> cudaError_t;
    pub fn cudaMallocMipmappedArray(
        mipmappedArray: *mut cudaMipmappedArray_t,
        desc: *const cudaChannelFormatDesc,
        extent: cudaExtent,
        numLevels: ::core::ffi::c_uint,
        flags: ::core::ffi::c_uint,
    ) -> cudaError_t;
    pub fn cudaMallocPitch(
        devPtr: *mut *mut ::core::ffi::c_void,
        pitch: *mut usize,
        width: usize,
        height: usize,
    ) -> cudaError_t;
    #[cfg(any(
        feature = "cuda-11040",
        feature = "cuda-11050",
        feature = "cuda-11060",
        feature = "cuda-11070",
        feature = "cuda-11080",
        feature = "cuda-12000",
        feature = "cuda-12010",
        feature = "cuda-12020",
        feature = "cuda-12030",
        feature = "cuda-12040",
        feature = "cuda-12050",
        feature = "cuda-12060",
        feature = "cuda-12080",
        feature = "cuda-12090"
    ))]
    pub fn cudaMemAdvise(
        devPtr: *const ::core::ffi::c_void,
        count: usize,
        advice: cudaMemoryAdvise,
        device: ::core::ffi::c_int,
    ) -> cudaError_t;
    #[cfg(any(feature = "cuda-13000"))]
    pub fn cudaMemAdvise(
        devPtr: *const ::core::ffi::c_void,
        count: usize,
        advice: cudaMemoryAdvise,
        location: cudaMemLocation,
    ) -> cudaError_t;
    #[cfg(any(
        feature = "cuda-12020",
        feature = "cuda-12030",
        feature = "cuda-12040",
        feature = "cuda-12050",
        feature = "cuda-12060",
        feature = "cuda-12080",
        feature = "cuda-12090"
    ))]
    pub fn cudaMemAdvise_v2(
        devPtr: *const ::core::ffi::c_void,
        count: usize,
        advice: cudaMemoryAdvise,
        location: cudaMemLocation,
    ) -> cudaError_t;
    #[cfg(any(feature = "cuda-13000"))]
    pub fn cudaMemDiscardAndPrefetchBatchAsync(
        dptrs: *mut *mut ::core::ffi::c_void,
        sizes: *mut usize,
        count: usize,
        prefetchLocs: *mut cudaMemLocation,
        prefetchLocIdxs: *mut usize,
        numPrefetchLocs: usize,
        flags: ::core::ffi::c_ulonglong,
        stream: cudaStream_t,
    ) -> cudaError_t;
    #[cfg(any(feature = "cuda-13000"))]
    pub fn cudaMemDiscardBatchAsync(
        dptrs: *mut *mut ::core::ffi::c_void,
        sizes: *mut usize,
        count: usize,
        flags: ::core::ffi::c_ulonglong,
        stream: cudaStream_t,
    ) -> cudaError_t;
    #[cfg(any(feature = "cuda-13000"))]
    pub fn cudaMemGetDefaultMemPool(
        memPool: *mut cudaMemPool_t,
        location: *mut cudaMemLocation,
        type_: cudaMemAllocationType,
    ) -> cudaError_t;
    pub fn cudaMemGetInfo(free: *mut usize, total: *mut usize) -> cudaError_t;
    #[cfg(any(feature = "cuda-13000"))]
    pub fn cudaMemGetMemPool(
        memPool: *mut cudaMemPool_t,
        location: *mut cudaMemLocation,
        type_: cudaMemAllocationType,
    ) -> cudaError_t;
    pub fn cudaMemPoolCreate(
        memPool: *mut cudaMemPool_t,
        poolProps: *const cudaMemPoolProps,
    ) -> cudaError_t;
    pub fn cudaMemPoolDestroy(memPool: cudaMemPool_t) -> cudaError_t;
    pub fn cudaMemPoolExportPointer(
        exportData: *mut cudaMemPoolPtrExportData,
        ptr: *mut ::core::ffi::c_void,
    ) -> cudaError_t;
    pub fn cudaMemPoolExportToShareableHandle(
        shareableHandle: *mut ::core::ffi::c_void,
        memPool: cudaMemPool_t,
        handleType: cudaMemAllocationHandleType,
        flags: ::core::ffi::c_uint,
    ) -> cudaError_t;
    pub fn cudaMemPoolGetAccess(
        flags: *mut cudaMemAccessFlags,
        memPool: cudaMemPool_t,
        location: *mut cudaMemLocation,
    ) -> cudaError_t;
    pub fn cudaMemPoolGetAttribute(
        memPool: cudaMemPool_t,
        attr: cudaMemPoolAttr,
        value: *mut ::core::ffi::c_void,
    ) -> cudaError_t;
    pub fn cudaMemPoolImportFromShareableHandle(
        memPool: *mut cudaMemPool_t,
        shareableHandle: *mut ::core::ffi::c_void,
        handleType: cudaMemAllocationHandleType,
        flags: ::core::ffi::c_uint,
    ) -> cudaError_t;
    pub fn cudaMemPoolImportPointer(
        ptr: *mut *mut ::core::ffi::c_void,
        memPool: cudaMemPool_t,
        exportData: *mut cudaMemPoolPtrExportData,
    ) -> cudaError_t;
    pub fn cudaMemPoolSetAccess(
        memPool: cudaMemPool_t,
        descList: *const cudaMemAccessDesc,
        count: usize,
    ) -> cudaError_t;
    pub fn cudaMemPoolSetAttribute(
        memPool: cudaMemPool_t,
        attr: cudaMemPoolAttr,
        value: *mut ::core::ffi::c_void,
    ) -> cudaError_t;
    pub fn cudaMemPoolTrimTo(memPool: cudaMemPool_t, minBytesToKeep: usize) -> cudaError_t;
    #[cfg(any(
        feature = "cuda-11040",
        feature = "cuda-11050",
        feature = "cuda-11060",
        feature = "cuda-11070",
        feature = "cuda-11080",
        feature = "cuda-12000",
        feature = "cuda-12010",
        feature = "cuda-12020",
        feature = "cuda-12030",
        feature = "cuda-12040",
        feature = "cuda-12050",
        feature = "cuda-12060",
        feature = "cuda-12080",
        feature = "cuda-12090"
    ))]
    pub fn cudaMemPrefetchAsync(
        devPtr: *const ::core::ffi::c_void,
        count: usize,
        dstDevice: ::core::ffi::c_int,
        stream: cudaStream_t,
    ) -> cudaError_t;
    #[cfg(any(feature = "cuda-13000"))]
    pub fn cudaMemPrefetchAsync(
        devPtr: *const ::core::ffi::c_void,
        count: usize,
        location: cudaMemLocation,
        flags: ::core::ffi::c_uint,
        stream: cudaStream_t,
    ) -> cudaError_t;
    #[cfg(any(
        feature = "cuda-12020",
        feature = "cuda-12030",
        feature = "cuda-12040",
        feature = "cuda-12050",
        feature = "cuda-12060",
        feature = "cuda-12080",
        feature = "cuda-12090"
    ))]
    pub fn cudaMemPrefetchAsync_v2(
        devPtr: *const ::core::ffi::c_void,
        count: usize,
        location: cudaMemLocation,
        flags: ::core::ffi::c_uint,
        stream: cudaStream_t,
    ) -> cudaError_t;
    #[cfg(any(feature = "cuda-13000"))]
    pub fn cudaMemPrefetchBatchAsync(
        dptrs: *mut *mut ::core::ffi::c_void,
        sizes: *mut usize,
        count: usize,
        prefetchLocs: *mut cudaMemLocation,
        prefetchLocIdxs: *mut usize,
        numPrefetchLocs: usize,
        flags: ::core::ffi::c_ulonglong,
        stream: cudaStream_t,
    ) -> cudaError_t;
    pub fn cudaMemRangeGetAttribute(
        data: *mut ::core::ffi::c_void,
        dataSize: usize,
        attribute: cudaMemRangeAttribute,
        devPtr: *const ::core::ffi::c_void,
        count: usize,
    ) -> cudaError_t;
    pub fn cudaMemRangeGetAttributes(
        data: *mut *mut ::core::ffi::c_void,
        dataSizes: *mut usize,
        attributes: *mut cudaMemRangeAttribute,
        numAttributes: usize,
        devPtr: *const ::core::ffi::c_void,
        count: usize,
    ) -> cudaError_t;
    #[cfg(any(feature = "cuda-13000"))]
    pub fn cudaMemSetMemPool(
        location: *mut cudaMemLocation,
        type_: cudaMemAllocationType,
        memPool: cudaMemPool_t,
    ) -> cudaError_t;
    pub fn cudaMemcpy(
        dst: *mut ::core::ffi::c_void,
        src: *const ::core::ffi::c_void,
        count: usize,
        kind: cudaMemcpyKind,
    ) -> cudaError_t;
    pub fn cudaMemcpy2D(
        dst: *mut ::core::ffi::c_void,
        dpitch: usize,
        src: *const ::core::ffi::c_void,
        spitch: usize,
        width: usize,
        height: usize,
        kind: cudaMemcpyKind,
    ) -> cudaError_t;
    pub fn cudaMemcpy2DArrayToArray(
        dst: cudaArray_t,
        wOffsetDst: usize,
        hOffsetDst: usize,
        src: cudaArray_const_t,
        wOffsetSrc: usize,
        hOffsetSrc: usize,
        width: usize,
        height: usize,
        kind: cudaMemcpyKind,
    ) -> cudaError_t;
    pub fn cudaMemcpy2DAsync(
        dst: *mut ::core::ffi::c_void,
        dpitch: usize,
        src: *const ::core::ffi::c_void,
        spitch: usize,
        width: usize,
        height: usize,
        kind: cudaMemcpyKind,
        stream: cudaStream_t,
    ) -> cudaError_t;
    pub fn cudaMemcpy2DFromArray(
        dst: *mut ::core::ffi::c_void,
        dpitch: usize,
        src: cudaArray_const_t,
        wOffset: usize,
        hOffset: usize,
        width: usize,
        height: usize,
        kind: cudaMemcpyKind,
    ) -> cudaError_t;
    pub fn cudaMemcpy2DFromArrayAsync(
        dst: *mut ::core::ffi::c_void,
        dpitch: usize,
        src: cudaArray_const_t,
        wOffset: usize,
        hOffset: usize,
        width: usize,
        height: usize,
        kind: cudaMemcpyKind,
        stream: cudaStream_t,
    ) -> cudaError_t;
    pub fn cudaMemcpy2DToArray(
        dst: cudaArray_t,
        wOffset: usize,
        hOffset: usize,
        src: *const ::core::ffi::c_void,
        spitch: usize,
        width: usize,
        height: usize,
        kind: cudaMemcpyKind,
    ) -> cudaError_t;
    pub fn cudaMemcpy2DToArrayAsync(
        dst: cudaArray_t,
        wOffset: usize,
        hOffset: usize,
        src: *const ::core::ffi::c_void,
        spitch: usize,
        width: usize,
        height: usize,
        kind: cudaMemcpyKind,
        stream: cudaStream_t,
    ) -> cudaError_t;
    pub fn cudaMemcpy3D(p: *const cudaMemcpy3DParms) -> cudaError_t;
    pub fn cudaMemcpy3DAsync(p: *const cudaMemcpy3DParms, stream: cudaStream_t) -> cudaError_t;
    #[cfg(any(feature = "cuda-12080", feature = "cuda-12090"))]
    pub fn cudaMemcpy3DBatchAsync(
        numOps: usize,
        opList: *mut cudaMemcpy3DBatchOp,
        failIdx: *mut usize,
        flags: ::core::ffi::c_ulonglong,
        stream: cudaStream_t,
    ) -> cudaError_t;
    #[cfg(any(feature = "cuda-13000"))]
    pub fn cudaMemcpy3DBatchAsync(
        numOps: usize,
        opList: *mut cudaMemcpy3DBatchOp,
        flags: ::core::ffi::c_ulonglong,
        stream: cudaStream_t,
    ) -> cudaError_t;
    pub fn cudaMemcpy3DPeer(p: *const cudaMemcpy3DPeerParms) -> cudaError_t;
    pub fn cudaMemcpy3DPeerAsync(
        p: *const cudaMemcpy3DPeerParms,
        stream: cudaStream_t,
    ) -> cudaError_t;
    pub fn cudaMemcpyArrayToArray(
        dst: cudaArray_t,
        wOffsetDst: usize,
        hOffsetDst: usize,
        src: cudaArray_const_t,
        wOffsetSrc: usize,
        hOffsetSrc: usize,
        count: usize,
        kind: cudaMemcpyKind,
    ) -> cudaError_t;
    pub fn cudaMemcpyAsync(
        dst: *mut ::core::ffi::c_void,
        src: *const ::core::ffi::c_void,
        count: usize,
        kind: cudaMemcpyKind,
        stream: cudaStream_t,
    ) -> cudaError_t;
    #[cfg(any(feature = "cuda-12080", feature = "cuda-12090"))]
    pub fn cudaMemcpyBatchAsync(
        dsts: *mut *mut ::core::ffi::c_void,
        srcs: *mut *mut ::core::ffi::c_void,
        sizes: *mut usize,
        count: usize,
        attrs: *mut cudaMemcpyAttributes,
        attrsIdxs: *mut usize,
        numAttrs: usize,
        failIdx: *mut usize,
        stream: cudaStream_t,
    ) -> cudaError_t;
    #[cfg(any(feature = "cuda-13000"))]
    pub fn cudaMemcpyBatchAsync(
        dsts: *const *mut ::core::ffi::c_void,
        srcs: *const *const ::core::ffi::c_void,
        sizes: *const usize,
        count: usize,
        attrs: *mut cudaMemcpyAttributes,
        attrsIdxs: *mut usize,
        numAttrs: usize,
        stream: cudaStream_t,
    ) -> cudaError_t;
    pub fn cudaMemcpyFromArray(
        dst: *mut ::core::ffi::c_void,
        src: cudaArray_const_t,
        wOffset: usize,
        hOffset: usize,
        count: usize,
        kind: cudaMemcpyKind,
    ) -> cudaError_t;
    pub fn cudaMemcpyFromArrayAsync(
        dst: *mut ::core::ffi::c_void,
        src: cudaArray_const_t,
        wOffset: usize,
        hOffset: usize,
        count: usize,
        kind: cudaMemcpyKind,
        stream: cudaStream_t,
    ) -> cudaError_t;
    pub fn cudaMemcpyFromSymbol(
        dst: *mut ::core::ffi::c_void,
        symbol: *const ::core::ffi::c_void,
        count: usize,
        offset: usize,
        kind: cudaMemcpyKind,
    ) -> cudaError_t;
    pub fn cudaMemcpyFromSymbolAsync(
        dst: *mut ::core::ffi::c_void,
        symbol: *const ::core::ffi::c_void,
        count: usize,
        offset: usize,
        kind: cudaMemcpyKind,
        stream: cudaStream_t,
    ) -> cudaError_t;
    pub fn cudaMemcpyPeer(
        dst: *mut ::core::ffi::c_void,
        dstDevice: ::core::ffi::c_int,
        src: *const ::core::ffi::c_void,
        srcDevice: ::core::ffi::c_int,
        count: usize,
    ) -> cudaError_t;
    pub fn cudaMemcpyPeerAsync(
        dst: *mut ::core::ffi::c_void,
        dstDevice: ::core::ffi::c_int,
        src: *const ::core::ffi::c_void,
        srcDevice: ::core::ffi::c_int,
        count: usize,
        stream: cudaStream_t,
    ) -> cudaError_t;
    pub fn cudaMemcpyToArray(
        dst: cudaArray_t,
        wOffset: usize,
        hOffset: usize,
        src: *const ::core::ffi::c_void,
        count: usize,
        kind: cudaMemcpyKind,
    ) -> cudaError_t;
    pub fn cudaMemcpyToArrayAsync(
        dst: cudaArray_t,
        wOffset: usize,
        hOffset: usize,
        src: *const ::core::ffi::c_void,
        count: usize,
        kind: cudaMemcpyKind,
        stream: cudaStream_t,
    ) -> cudaError_t;
    pub fn cudaMemcpyToSymbol(
        symbol: *const ::core::ffi::c_void,
        src: *const ::core::ffi::c_void,
        count: usize,
        offset: usize,
        kind: cudaMemcpyKind,
    ) -> cudaError_t;
    pub fn cudaMemcpyToSymbolAsync(
        symbol: *const ::core::ffi::c_void,
        src: *const ::core::ffi::c_void,
        count: usize,
        offset: usize,
        kind: cudaMemcpyKind,
        stream: cudaStream_t,
    ) -> cudaError_t;
    pub fn cudaMemset(
        devPtr: *mut ::core::ffi::c_void,
        value: ::core::ffi::c_int,
        count: usize,
    ) -> cudaError_t;
    pub fn cudaMemset2D(
        devPtr: *mut ::core::ffi::c_void,
        pitch: usize,
        value: ::core::ffi::c_int,
        width: usize,
        height: usize,
    ) -> cudaError_t;
    pub fn cudaMemset2DAsync(
        devPtr: *mut ::core::ffi::c_void,
        pitch: usize,
        value: ::core::ffi::c_int,
        width: usize,
        height: usize,
        stream: cudaStream_t,
    ) -> cudaError_t;
    pub fn cudaMemset3D(
        pitchedDevPtr: cudaPitchedPtr,
        value: ::core::ffi::c_int,
        extent: cudaExtent,
    ) -> cudaError_t;
    pub fn cudaMemset3DAsync(
        pitchedDevPtr: cudaPitchedPtr,
        value: ::core::ffi::c_int,
        extent: cudaExtent,
        stream: cudaStream_t,
    ) -> cudaError_t;
    pub fn cudaMemsetAsync(
        devPtr: *mut ::core::ffi::c_void,
        value: ::core::ffi::c_int,
        count: usize,
        stream: cudaStream_t,
    ) -> cudaError_t;
    #[cfg(any(
        feature = "cuda-11060",
        feature = "cuda-11070",
        feature = "cuda-11080",
        feature = "cuda-12000",
        feature = "cuda-12010",
        feature = "cuda-12020",
        feature = "cuda-12030",
        feature = "cuda-12040",
        feature = "cuda-12050",
        feature = "cuda-12060",
        feature = "cuda-12080",
        feature = "cuda-12090",
        feature = "cuda-13000"
    ))]
    pub fn cudaMipmappedArrayGetMemoryRequirements(
        memoryRequirements: *mut cudaArrayMemoryRequirements,
        mipmap: cudaMipmappedArray_t,
        device: ::core::ffi::c_int,
    ) -> cudaError_t;
    pub fn cudaMipmappedArrayGetSparseProperties(
        sparseProperties: *mut cudaArraySparseProperties,
        mipmap: cudaMipmappedArray_t,
    ) -> cudaError_t;
    pub fn cudaOccupancyAvailableDynamicSMemPerBlock(
        dynamicSmemSize: *mut usize,
        func: *const ::core::ffi::c_void,
        numBlocks: ::core::ffi::c_int,
        blockSize: ::core::ffi::c_int,
    ) -> cudaError_t;
    pub fn cudaOccupancyMaxActiveBlocksPerMultiprocessor(
        numBlocks: *mut ::core::ffi::c_int,
        func: *const ::core::ffi::c_void,
        blockSize: ::core::ffi::c_int,
        dynamicSMemSize: usize,
    ) -> cudaError_t;
    pub fn cudaOccupancyMaxActiveBlocksPerMultiprocessorWithFlags(
        numBlocks: *mut ::core::ffi::c_int,
        func: *const ::core::ffi::c_void,
        blockSize: ::core::ffi::c_int,
        dynamicSMemSize: usize,
        flags: ::core::ffi::c_uint,
    ) -> cudaError_t;
    #[cfg(any(
        feature = "cuda-11080",
        feature = "cuda-12000",
        feature = "cuda-12010",
        feature = "cuda-12020",
        feature = "cuda-12030",
        feature = "cuda-12040",
        feature = "cuda-12050",
        feature = "cuda-12060",
        feature = "cuda-12080",
        feature = "cuda-12090",
        feature = "cuda-13000"
    ))]
    pub fn cudaOccupancyMaxActiveClusters(
        numClusters: *mut ::core::ffi::c_int,
        func: *const ::core::ffi::c_void,
        launchConfig: *const cudaLaunchConfig_t,
    ) -> cudaError_t;
    #[cfg(any(
        feature = "cuda-11080",
        feature = "cuda-12000",
        feature = "cuda-12010",
        feature = "cuda-12020",
        feature = "cuda-12030",
        feature = "cuda-12040",
        feature = "cuda-12050",
        feature = "cuda-12060",
        feature = "cuda-12080",
        feature = "cuda-12090",
        feature = "cuda-13000"
    ))]
    pub fn cudaOccupancyMaxPotentialClusterSize(
        clusterSize: *mut ::core::ffi::c_int,
        func: *const ::core::ffi::c_void,
        launchConfig: *const cudaLaunchConfig_t,
    ) -> cudaError_t;
    pub fn cudaPeekAtLastError() -> cudaError_t;
    pub fn cudaPointerGetAttributes(
        attributes: *mut cudaPointerAttributes,
        ptr: *const ::core::ffi::c_void,
    ) -> cudaError_t;
    pub fn cudaProfilerStop() -> cudaError_t;
    pub fn cudaRuntimeGetVersion(runtimeVersion: *mut ::core::ffi::c_int) -> cudaError_t;
    pub fn cudaSetDevice(device: ::core::ffi::c_int) -> cudaError_t;
    pub fn cudaSetDeviceFlags(flags: ::core::ffi::c_uint) -> cudaError_t;
    #[cfg(any(
        feature = "cuda-11040",
        feature = "cuda-11050",
        feature = "cuda-11060",
        feature = "cuda-11070",
        feature = "cuda-11080",
        feature = "cuda-12000",
        feature = "cuda-12010",
        feature = "cuda-12020",
        feature = "cuda-12030",
        feature = "cuda-12040",
        feature = "cuda-12050",
        feature = "cuda-12060",
        feature = "cuda-12080",
        feature = "cuda-12090"
    ))]
    pub fn cudaSetDoubleForDevice(d: *mut f64) -> cudaError_t;
    #[cfg(any(
        feature = "cuda-11040",
        feature = "cuda-11050",
        feature = "cuda-11060",
        feature = "cuda-11070",
        feature = "cuda-11080",
        feature = "cuda-12000",
        feature = "cuda-12010",
        feature = "cuda-12020",
        feature = "cuda-12030",
        feature = "cuda-12040",
        feature = "cuda-12050",
        feature = "cuda-12060",
        feature = "cuda-12080",
        feature = "cuda-12090"
    ))]
    pub fn cudaSetDoubleForHost(d: *mut f64) -> cudaError_t;
    pub fn cudaSetValidDevices(
        device_arr: *mut ::core::ffi::c_int,
        len: ::core::ffi::c_int,
    ) -> cudaError_t;
    #[cfg(any(feature = "cuda-13000"))]
    pub fn cudaSignalExternalSemaphoresAsync(
        extSemArray: *const cudaExternalSemaphore_t,
        paramsArray: *const cudaExternalSemaphoreSignalParams,
        numExtSems: ::core::ffi::c_uint,
        stream: cudaStream_t,
    ) -> cudaError_t;
    #[cfg(any(
        feature = "cuda-11040",
        feature = "cuda-11050",
        feature = "cuda-11060",
        feature = "cuda-11070",
        feature = "cuda-11080",
        feature = "cuda-12000",
        feature = "cuda-12010",
        feature = "cuda-12020",
        feature = "cuda-12030",
        feature = "cuda-12040",
        feature = "cuda-12050",
        feature = "cuda-12060",
        feature = "cuda-12080",
        feature = "cuda-12090"
    ))]
    pub fn cudaSignalExternalSemaphoresAsync_v2(
        extSemArray: *const cudaExternalSemaphore_t,
        paramsArray: *const cudaExternalSemaphoreSignalParams,
        numExtSems: ::core::ffi::c_uint,
        stream: cudaStream_t,
    ) -> cudaError_t;
    pub fn cudaStreamAddCallback(
        stream: cudaStream_t,
        callback: cudaStreamCallback_t,
        userData: *mut ::core::ffi::c_void,
        flags: ::core::ffi::c_uint,
    ) -> cudaError_t;
    pub fn cudaStreamAttachMemAsync(
        stream: cudaStream_t,
        devPtr: *mut ::core::ffi::c_void,
        length: usize,
        flags: ::core::ffi::c_uint,
    ) -> cudaError_t;
    pub fn cudaStreamBeginCapture(stream: cudaStream_t, mode: cudaStreamCaptureMode)
        -> cudaError_t;
    #[cfg(any(
        feature = "cuda-12030",
        feature = "cuda-12040",
        feature = "cuda-12050",
        feature = "cuda-12060",
        feature = "cuda-12080",
        feature = "cuda-12090",
        feature = "cuda-13000"
    ))]
    pub fn cudaStreamBeginCaptureToGraph(
        stream: cudaStream_t,
        graph: cudaGraph_t,
        dependencies: *const cudaGraphNode_t,
        dependencyData: *const cudaGraphEdgeData,
        numDependencies: usize,
        mode: cudaStreamCaptureMode,
    ) -> cudaError_t;
    pub fn cudaStreamCopyAttributes(dst: cudaStream_t, src: cudaStream_t) -> cudaError_t;
    pub fn cudaStreamCreate(pStream: *mut cudaStream_t) -> cudaError_t;
    pub fn cudaStreamCreateWithFlags(
        pStream: *mut cudaStream_t,
        flags: ::core::ffi::c_uint,
    ) -> cudaError_t;
    pub fn cudaStreamCreateWithPriority(
        pStream: *mut cudaStream_t,
        flags: ::core::ffi::c_uint,
        priority: ::core::ffi::c_int,
    ) -> cudaError_t;
    pub fn cudaStreamDestroy(stream: cudaStream_t) -> cudaError_t;
    pub fn cudaStreamEndCapture(stream: cudaStream_t, pGraph: *mut cudaGraph_t) -> cudaError_t;
    #[cfg(any(
        feature = "cuda-11040",
        feature = "cuda-11050",
        feature = "cuda-11060",
        feature = "cuda-11070"
    ))]
    pub fn cudaStreamGetAttribute(
        hStream: cudaStream_t,
        attr: cudaStreamAttrID,
        value_out: *mut cudaStreamAttrValue,
    ) -> cudaError_t;
    #[cfg(any(
        feature = "cuda-11080",
        feature = "cuda-12000",
        feature = "cuda-12010",
        feature = "cuda-12020",
        feature = "cuda-12030",
        feature = "cuda-12040",
        feature = "cuda-12050",
        feature = "cuda-12060",
        feature = "cuda-12080",
        feature = "cuda-12090",
        feature = "cuda-13000"
    ))]
    pub fn cudaStreamGetAttribute(
        hStream: cudaStream_t,
        attr: cudaLaunchAttributeID,
        value_out: *mut cudaLaunchAttributeValue,
    ) -> cudaError_t;
    #[cfg(any(
        feature = "cuda-11040",
        feature = "cuda-11050",
        feature = "cuda-11060",
        feature = "cuda-11070",
        feature = "cuda-11080"
    ))]
    pub fn cudaStreamGetCaptureInfo(
        stream: cudaStream_t,
        pCaptureStatus: *mut cudaStreamCaptureStatus,
        pId: *mut ::core::ffi::c_ulonglong,
    ) -> cudaError_t;
    #[cfg(any(feature = "cuda-13000"))]
    pub fn cudaStreamGetCaptureInfo(
        stream: cudaStream_t,
        captureStatus_out: *mut cudaStreamCaptureStatus,
        id_out: *mut ::core::ffi::c_ulonglong,
        graph_out: *mut cudaGraph_t,
        dependencies_out: *mut *const cudaGraphNode_t,
        edgeData_out: *mut *const cudaGraphEdgeData,
        numDependencies_out: *mut usize,
    ) -> cudaError_t;
    #[cfg(any(
        feature = "cuda-11040",
        feature = "cuda-11050",
        feature = "cuda-11060",
        feature = "cuda-11070",
        feature = "cuda-11080",
        feature = "cuda-12000",
        feature = "cuda-12010",
        feature = "cuda-12020",
        feature = "cuda-12030",
        feature = "cuda-12040",
        feature = "cuda-12050",
        feature = "cuda-12060",
        feature = "cuda-12080",
        feature = "cuda-12090"
    ))]
    pub fn cudaStreamGetCaptureInfo_v2(
        stream: cudaStream_t,
        captureStatus_out: *mut cudaStreamCaptureStatus,
        id_out: *mut ::core::ffi::c_ulonglong,
        graph_out: *mut cudaGraph_t,
        dependencies_out: *mut *const cudaGraphNode_t,
        numDependencies_out: *mut usize,
    ) -> cudaError_t;
    #[cfg(any(
        feature = "cuda-12030",
        feature = "cuda-12040",
        feature = "cuda-12050",
        feature = "cuda-12060",
        feature = "cuda-12080",
        feature = "cuda-12090"
    ))]
    pub fn cudaStreamGetCaptureInfo_v3(
        stream: cudaStream_t,
        captureStatus_out: *mut cudaStreamCaptureStatus,
        id_out: *mut ::core::ffi::c_ulonglong,
        graph_out: *mut cudaGraph_t,
        dependencies_out: *mut *const cudaGraphNode_t,
        edgeData_out: *mut *const cudaGraphEdgeData,
        numDependencies_out: *mut usize,
    ) -> cudaError_t;
    #[cfg(any(feature = "cuda-12080", feature = "cuda-12090", feature = "cuda-13000"))]
    pub fn cudaStreamGetDevice(
        hStream: cudaStream_t,
        device: *mut ::core::ffi::c_int,
    ) -> cudaError_t;
    pub fn cudaStreamGetFlags(
        hStream: cudaStream_t,
        flags: *mut ::core::ffi::c_uint,
    ) -> cudaError_t;
    #[cfg(any(
        feature = "cuda-12000",
        feature = "cuda-12010",
        feature = "cuda-12020",
        feature = "cuda-12030",
        feature = "cuda-12040",
        feature = "cuda-12050",
        feature = "cuda-12060",
        feature = "cuda-12080",
        feature = "cuda-12090",
        feature = "cuda-13000"
    ))]
    pub fn cudaStreamGetId(
        hStream: cudaStream_t,
        streamId: *mut ::core::ffi::c_ulonglong,
    ) -> cudaError_t;
    pub fn cudaStreamGetPriority(
        hStream: cudaStream_t,
        priority: *mut ::core::ffi::c_int,
    ) -> cudaError_t;
    pub fn cudaStreamIsCapturing(
        stream: cudaStream_t,
        pCaptureStatus: *mut cudaStreamCaptureStatus,
    ) -> cudaError_t;
    pub fn cudaStreamQuery(stream: cudaStream_t) -> cudaError_t;
    #[cfg(any(
        feature = "cuda-11040",
        feature = "cuda-11050",
        feature = "cuda-11060",
        feature = "cuda-11070"
    ))]
    pub fn cudaStreamSetAttribute(
        hStream: cudaStream_t,
        attr: cudaStreamAttrID,
        value: *const cudaStreamAttrValue,
    ) -> cudaError_t;
    #[cfg(any(
        feature = "cuda-11080",
        feature = "cuda-12000",
        feature = "cuda-12010",
        feature = "cuda-12020",
        feature = "cuda-12030",
        feature = "cuda-12040",
        feature = "cuda-12050",
        feature = "cuda-12060",
        feature = "cuda-12080",
        feature = "cuda-12090",
        feature = "cuda-13000"
    ))]
    pub fn cudaStreamSetAttribute(
        hStream: cudaStream_t,
        attr: cudaLaunchAttributeID,
        value: *const cudaLaunchAttributeValue,
    ) -> cudaError_t;
    pub fn cudaStreamSynchronize(stream: cudaStream_t) -> cudaError_t;
    #[cfg(any(
        feature = "cuda-11040",
        feature = "cuda-11050",
        feature = "cuda-11060",
        feature = "cuda-11070",
        feature = "cuda-11080",
        feature = "cuda-12000",
        feature = "cuda-12010",
        feature = "cuda-12020",
        feature = "cuda-12030",
        feature = "cuda-12040",
        feature = "cuda-12050",
        feature = "cuda-12060",
        feature = "cuda-12080",
        feature = "cuda-12090"
    ))]
    pub fn cudaStreamUpdateCaptureDependencies(
        stream: cudaStream_t,
        dependencies: *mut cudaGraphNode_t,
        numDependencies: usize,
        flags: ::core::ffi::c_uint,
    ) -> cudaError_t;
    #[cfg(any(feature = "cuda-13000"))]
    pub fn cudaStreamUpdateCaptureDependencies(
        stream: cudaStream_t,
        dependencies: *mut cudaGraphNode_t,
        dependencyData: *const cudaGraphEdgeData,
        numDependencies: usize,
        flags: ::core::ffi::c_uint,
    ) -> cudaError_t;
    #[cfg(any(
        feature = "cuda-12030",
        feature = "cuda-12040",
        feature = "cuda-12050",
        feature = "cuda-12060",
        feature = "cuda-12080",
        feature = "cuda-12090"
    ))]
    pub fn cudaStreamUpdateCaptureDependencies_v2(
        stream: cudaStream_t,
        dependencies: *mut cudaGraphNode_t,
        dependencyData: *const cudaGraphEdgeData,
        numDependencies: usize,
        flags: ::core::ffi::c_uint,
    ) -> cudaError_t;
    pub fn cudaStreamWaitEvent(
        stream: cudaStream_t,
        event: cudaEvent_t,
        flags: ::core::ffi::c_uint,
    ) -> cudaError_t;
    pub fn cudaThreadExchangeStreamCaptureMode(mode: *mut cudaStreamCaptureMode) -> cudaError_t;
    #[cfg(any(
        feature = "cuda-11040",
        feature = "cuda-11050",
        feature = "cuda-11060",
        feature = "cuda-11070",
        feature = "cuda-11080",
        feature = "cuda-12000",
        feature = "cuda-12010",
        feature = "cuda-12020",
        feature = "cuda-12030",
        feature = "cuda-12040",
        feature = "cuda-12050",
        feature = "cuda-12060",
        feature = "cuda-12080",
        feature = "cuda-12090"
    ))]
    pub fn cudaThreadExit() -> cudaError_t;
    #[cfg(any(
        feature = "cuda-11040",
        feature = "cuda-11050",
        feature = "cuda-11060",
        feature = "cuda-11070",
        feature = "cuda-11080",
        feature = "cuda-12000",
        feature = "cuda-12010",
        feature = "cuda-12020",
        feature = "cuda-12030",
        feature = "cuda-12040",
        feature = "cuda-12050",
        feature = "cuda-12060",
        feature = "cuda-12080",
        feature = "cuda-12090"
    ))]
    pub fn cudaThreadGetCacheConfig(pCacheConfig: *mut cudaFuncCache) -> cudaError_t;
    #[cfg(any(
        feature = "cuda-11040",
        feature = "cuda-11050",
        feature = "cuda-11060",
        feature = "cuda-11070",
        feature = "cuda-11080",
        feature = "cuda-12000",
        feature = "cuda-12010",
        feature = "cuda-12020",
        feature = "cuda-12030",
        feature = "cuda-12040",
        feature = "cuda-12050",
        feature = "cuda-12060",
        feature = "cuda-12080",
        feature = "cuda-12090"
    ))]
    pub fn cudaThreadGetLimit(pValue: *mut usize, limit: cudaLimit) -> cudaError_t;
    #[cfg(any(
        feature = "cuda-11040",
        feature = "cuda-11050",
        feature = "cuda-11060",
        feature = "cuda-11070",
        feature = "cuda-11080",
        feature = "cuda-12000",
        feature = "cuda-12010",
        feature = "cuda-12020",
        feature = "cuda-12030",
        feature = "cuda-12040",
        feature = "cuda-12050",
        feature = "cuda-12060",
        feature = "cuda-12080",
        feature = "cuda-12090"
    ))]
    pub fn cudaThreadSetCacheConfig(cacheConfig: cudaFuncCache) -> cudaError_t;
    #[cfg(any(
        feature = "cuda-11040",
        feature = "cuda-11050",
        feature = "cuda-11060",
        feature = "cuda-11070",
        feature = "cuda-11080",
        feature = "cuda-12000",
        feature = "cuda-12010",
        feature = "cuda-12020",
        feature = "cuda-12030",
        feature = "cuda-12040",
        feature = "cuda-12050",
        feature = "cuda-12060",
        feature = "cuda-12080",
        feature = "cuda-12090"
    ))]
    pub fn cudaThreadSetLimit(limit: cudaLimit, value: usize) -> cudaError_t;
    #[cfg(any(
        feature = "cuda-11040",
        feature = "cuda-11050",
        feature = "cuda-11060",
        feature = "cuda-11070",
        feature = "cuda-11080",
        feature = "cuda-12000",
        feature = "cuda-12010",
        feature = "cuda-12020",
        feature = "cuda-12030",
        feature = "cuda-12040",
        feature = "cuda-12050",
        feature = "cuda-12060",
        feature = "cuda-12080",
        feature = "cuda-12090"
    ))]
    pub fn cudaThreadSynchronize() -> cudaError_t;
    #[cfg(any(
        feature = "cuda-11040",
        feature = "cuda-11050",
        feature = "cuda-11060",
        feature = "cuda-11070",
        feature = "cuda-11080"
    ))]
    pub fn cudaUnbindTexture(texref: *const textureReference) -> cudaError_t;
    pub fn cudaUserObjectCreate(
        object_out: *mut cudaUserObject_t,
        ptr: *mut ::core::ffi::c_void,
        destroy: cudaHostFn_t,
        initialRefcount: ::core::ffi::c_uint,
        flags: ::core::ffi::c_uint,
    ) -> cudaError_t;
    pub fn cudaUserObjectRelease(
        object: cudaUserObject_t,
        count: ::core::ffi::c_uint,
    ) -> cudaError_t;
    pub fn cudaUserObjectRetain(
        object: cudaUserObject_t,
        count: ::core::ffi::c_uint,
    ) -> cudaError_t;
    #[cfg(any(feature = "cuda-13000"))]
    pub fn cudaWaitExternalSemaphoresAsync(
        extSemArray: *const cudaExternalSemaphore_t,
        paramsArray: *const cudaExternalSemaphoreWaitParams,
        numExtSems: ::core::ffi::c_uint,
        stream: cudaStream_t,
    ) -> cudaError_t;
    #[cfg(any(
        feature = "cuda-11040",
        feature = "cuda-11050",
        feature = "cuda-11060",
        feature = "cuda-11070",
        feature = "cuda-11080",
        feature = "cuda-12000",
        feature = "cuda-12010",
        feature = "cuda-12020",
        feature = "cuda-12030",
        feature = "cuda-12040",
        feature = "cuda-12050",
        feature = "cuda-12060",
        feature = "cuda-12080",
        feature = "cuda-12090"
    ))]
    pub fn cudaWaitExternalSemaphoresAsync_v2(
        extSemArray: *const cudaExternalSemaphore_t,
        paramsArray: *const cudaExternalSemaphoreWaitParams,
        numExtSems: ::core::ffi::c_uint,
        stream: cudaStream_t,
    ) -> cudaError_t;
}
#[cfg(feature = "dynamic-loading")]
mod loaded {
    use super::*;
    pub unsafe fn cudaArrayGetInfo(
        desc: *mut cudaChannelFormatDesc,
        extent: *mut cudaExtent,
        flags: *mut ::core::ffi::c_uint,
        array: cudaArray_t,
    ) -> cudaError_t {
        if let Some(__function) = (culib().cudaArrayGetInfo) {
            __function(desc, extent, flags, array)
        } else {
            cudaError::cudaErrorNotSupported
        }
    }
    #[cfg(any(
        feature = "cuda-11060",
        feature = "cuda-11070",
        feature = "cuda-11080",
        feature = "cuda-12000",
        feature = "cuda-12010",
        feature = "cuda-12020",
        feature = "cuda-12030",
        feature = "cuda-12040",
        feature = "cuda-12050",
        feature = "cuda-12060",
        feature = "cuda-12080",
        feature = "cuda-12090",
        feature = "cuda-13000"
    ))]
    pub unsafe fn cudaArrayGetMemoryRequirements(
        memoryRequirements: *mut cudaArrayMemoryRequirements,
        array: cudaArray_t,
        device: ::core::ffi::c_int,
    ) -> cudaError_t {
        if let Some(__function) = (culib().cudaArrayGetMemoryRequirements) {
            __function(memoryRequirements, array, device)
        } else {
            cudaError::cudaErrorNotSupported
        }
    }
    pub unsafe fn cudaArrayGetPlane(
        pPlaneArray: *mut cudaArray_t,
        hArray: cudaArray_t,
        planeIdx: ::core::ffi::c_uint,
    ) -> cudaError_t {
        if let Some(__function) = (culib().cudaArrayGetPlane) {
            __function(pPlaneArray, hArray, planeIdx)
        } else {
            cudaError::cudaErrorNotSupported
        }
    }
    pub unsafe fn cudaArrayGetSparseProperties(
        sparseProperties: *mut cudaArraySparseProperties,
        array: cudaArray_t,
    ) -> cudaError_t {
        if let Some(__function) = (culib().cudaArrayGetSparseProperties) {
            __function(sparseProperties, array)
        } else {
            cudaError::cudaErrorNotSupported
        }
    }
    #[cfg(any(
        feature = "cuda-11040",
        feature = "cuda-11050",
        feature = "cuda-11060",
        feature = "cuda-11070",
        feature = "cuda-11080"
    ))]
    pub unsafe fn cudaBindSurfaceToArray(
        surfref: *const surfaceReference,
        array: cudaArray_const_t,
        desc: *const cudaChannelFormatDesc,
    ) -> cudaError_t {
        if let Some(__function) = (culib().cudaBindSurfaceToArray) {
            __function(surfref, array, desc)
        } else {
            cudaError::cudaErrorNotSupported
        }
    }
    #[cfg(any(
        feature = "cuda-11040",
        feature = "cuda-11050",
        feature = "cuda-11060",
        feature = "cuda-11070",
        feature = "cuda-11080"
    ))]
    pub unsafe fn cudaBindTexture(
        offset: *mut usize,
        texref: *const textureReference,
        devPtr: *const ::core::ffi::c_void,
        desc: *const cudaChannelFormatDesc,
        size: usize,
    ) -> cudaError_t {
        if let Some(__function) = (culib().cudaBindTexture) {
            __function(offset, texref, devPtr, desc, size)
        } else {
            cudaError::cudaErrorNotSupported
        }
    }
    #[cfg(any(
        feature = "cuda-11040",
        feature = "cuda-11050",
        feature = "cuda-11060",
        feature = "cuda-11070",
        feature = "cuda-11080"
    ))]
    pub unsafe fn cudaBindTexture2D(
        offset: *mut usize,
        texref: *const textureReference,
        devPtr: *const ::core::ffi::c_void,
        desc: *const cudaChannelFormatDesc,
        width: usize,
        height: usize,
        pitch: usize,
    ) -> cudaError_t {
        if let Some(__function) = (culib().cudaBindTexture2D) {
            __function(offset, texref, devPtr, desc, width, height, pitch)
        } else {
            cudaError::cudaErrorNotSupported
        }
    }
    #[cfg(any(
        feature = "cuda-11040",
        feature = "cuda-11050",
        feature = "cuda-11060",
        feature = "cuda-11070",
        feature = "cuda-11080"
    ))]
    pub unsafe fn cudaBindTextureToArray(
        texref: *const textureReference,
        array: cudaArray_const_t,
        desc: *const cudaChannelFormatDesc,
    ) -> cudaError_t {
        if let Some(__function) = (culib().cudaBindTextureToArray) {
            __function(texref, array, desc)
        } else {
            cudaError::cudaErrorNotSupported
        }
    }
    #[cfg(any(
        feature = "cuda-11040",
        feature = "cuda-11050",
        feature = "cuda-11060",
        feature = "cuda-11070",
        feature = "cuda-11080"
    ))]
    pub unsafe fn cudaBindTextureToMipmappedArray(
        texref: *const textureReference,
        mipmappedArray: cudaMipmappedArray_const_t,
        desc: *const cudaChannelFormatDesc,
    ) -> cudaError_t {
        if let Some(__function) = (culib().cudaBindTextureToMipmappedArray) {
            __function(texref, mipmappedArray, desc)
        } else {
            cudaError::cudaErrorNotSupported
        }
    }
    pub unsafe fn cudaChooseDevice(
        device: *mut ::core::ffi::c_int,
        prop: *const cudaDeviceProp,
    ) -> cudaError_t {
        if let Some(__function) = (culib().cudaChooseDevice) {
            __function(device, prop)
        } else {
            cudaError::cudaErrorNotSupported
        }
    }
    pub unsafe fn cudaCreateChannelDesc(
        x: ::core::ffi::c_int,
        y: ::core::ffi::c_int,
        z: ::core::ffi::c_int,
        w: ::core::ffi::c_int,
        f: cudaChannelFormatKind,
    ) -> cudaChannelFormatDesc {
        if let Some(__function) = (culib().cudaCreateChannelDesc) {
            __function(x, y, z, w, f)
        } else {
            panic!(
                "cudaCreateChannelDesc was not found during dynamic loading - this indicates a CUDA toolkit or driver version mismatch",
            )
        }
    }
    pub unsafe fn cudaCreateSurfaceObject(
        pSurfObject: *mut cudaSurfaceObject_t,
        pResDesc: *const cudaResourceDesc,
    ) -> cudaError_t {
        if let Some(__function) = (culib().cudaCreateSurfaceObject) {
            __function(pSurfObject, pResDesc)
        } else {
            cudaError::cudaErrorNotSupported
        }
    }
    pub unsafe fn cudaCreateTextureObject(
        pTexObject: *mut cudaTextureObject_t,
        pResDesc: *const cudaResourceDesc,
        pTexDesc: *const cudaTextureDesc,
        pResViewDesc: *const cudaResourceViewDesc,
    ) -> cudaError_t {
        if let Some(__function) = (culib().cudaCreateTextureObject) {
            __function(pTexObject, pResDesc, pTexDesc, pResViewDesc)
        } else {
            cudaError::cudaErrorNotSupported
        }
    }
    #[cfg(any(feature = "cuda-11080"))]
    pub unsafe fn cudaCreateTextureObject_v2(
        pTexObject: *mut cudaTextureObject_t,
        pResDesc: *const cudaResourceDesc,
        pTexDesc: *const cudaTextureDesc_v2,
        pResViewDesc: *const cudaResourceViewDesc,
    ) -> cudaError_t {
        if let Some(__function) = (culib().cudaCreateTextureObject_v2) {
            __function(pTexObject, pResDesc, pTexDesc, pResViewDesc)
        } else {
            cudaError::cudaErrorNotSupported
        }
    }
    pub unsafe fn cudaCtxResetPersistingL2Cache() -> cudaError_t {
        if let Some(__function) = (culib().cudaCtxResetPersistingL2Cache) {
            __function()
        } else {
            cudaError::cudaErrorNotSupported
        }
    }
    pub unsafe fn cudaDestroyExternalMemory(extMem: cudaExternalMemory_t) -> cudaError_t {
        if let Some(__function) = (culib().cudaDestroyExternalMemory) {
            __function(extMem)
        } else {
            cudaError::cudaErrorNotSupported
        }
    }
    pub unsafe fn cudaDestroyExternalSemaphore(extSem: cudaExternalSemaphore_t) -> cudaError_t {
        if let Some(__function) = (culib().cudaDestroyExternalSemaphore) {
            __function(extSem)
        } else {
            cudaError::cudaErrorNotSupported
        }
    }
    pub unsafe fn cudaDestroySurfaceObject(surfObject: cudaSurfaceObject_t) -> cudaError_t {
        if let Some(__function) = (culib().cudaDestroySurfaceObject) {
            __function(surfObject)
        } else {
            cudaError::cudaErrorNotSupported
        }
    }
    pub unsafe fn cudaDestroyTextureObject(texObject: cudaTextureObject_t) -> cudaError_t {
        if let Some(__function) = (culib().cudaDestroyTextureObject) {
            __function(texObject)
        } else {
            cudaError::cudaErrorNotSupported
        }
    }
    pub unsafe fn cudaDeviceCanAccessPeer(
        canAccessPeer: *mut ::core::ffi::c_int,
        device: ::core::ffi::c_int,
        peerDevice: ::core::ffi::c_int,
    ) -> cudaError_t {
        if let Some(__function) = (culib().cudaDeviceCanAccessPeer) {
            __function(canAccessPeer, device, peerDevice)
        } else {
            cudaError::cudaErrorNotSupported
        }
    }
    pub unsafe fn cudaDeviceDisablePeerAccess(peerDevice: ::core::ffi::c_int) -> cudaError_t {
        if let Some(__function) = (culib().cudaDeviceDisablePeerAccess) {
            __function(peerDevice)
        } else {
            cudaError::cudaErrorNotSupported
        }
    }
    pub unsafe fn cudaDeviceEnablePeerAccess(
        peerDevice: ::core::ffi::c_int,
        flags: ::core::ffi::c_uint,
    ) -> cudaError_t {
        if let Some(__function) = (culib().cudaDeviceEnablePeerAccess) {
            __function(peerDevice, flags)
        } else {
            cudaError::cudaErrorNotSupported
        }
    }
    pub unsafe fn cudaDeviceFlushGPUDirectRDMAWrites(
        target: cudaFlushGPUDirectRDMAWritesTarget,
        scope: cudaFlushGPUDirectRDMAWritesScope,
    ) -> cudaError_t {
        if let Some(__function) = (culib().cudaDeviceFlushGPUDirectRDMAWrites) {
            __function(target, scope)
        } else {
            cudaError::cudaErrorNotSupported
        }
    }
    pub unsafe fn cudaDeviceGetAttribute(
        value: *mut ::core::ffi::c_int,
        attr: cudaDeviceAttr,
        device: ::core::ffi::c_int,
    ) -> cudaError_t {
        if let Some(__function) = (culib().cudaDeviceGetAttribute) {
            __function(value, attr, device)
        } else {
            cudaError::cudaErrorNotSupported
        }
    }
    pub unsafe fn cudaDeviceGetByPCIBusId(
        device: *mut ::core::ffi::c_int,
        pciBusId: *const ::core::ffi::c_char,
    ) -> cudaError_t {
        if let Some(__function) = (culib().cudaDeviceGetByPCIBusId) {
            __function(device, pciBusId)
        } else {
            cudaError::cudaErrorNotSupported
        }
    }
    pub unsafe fn cudaDeviceGetCacheConfig(pCacheConfig: *mut cudaFuncCache) -> cudaError_t {
        if let Some(__function) = (culib().cudaDeviceGetCacheConfig) {
            __function(pCacheConfig)
        } else {
            cudaError::cudaErrorNotSupported
        }
    }
    pub unsafe fn cudaDeviceGetDefaultMemPool(
        memPool: *mut cudaMemPool_t,
        device: ::core::ffi::c_int,
    ) -> cudaError_t {
        if let Some(__function) = (culib().cudaDeviceGetDefaultMemPool) {
            __function(memPool, device)
        } else {
            cudaError::cudaErrorNotSupported
        }
    }
    pub unsafe fn cudaDeviceGetGraphMemAttribute(
        device: ::core::ffi::c_int,
        attr: cudaGraphMemAttributeType,
        value: *mut ::core::ffi::c_void,
    ) -> cudaError_t {
        if let Some(__function) = (culib().cudaDeviceGetGraphMemAttribute) {
            __function(device, attr, value)
        } else {
            cudaError::cudaErrorNotSupported
        }
    }
    #[cfg(any(feature = "cuda-13000"))]
    pub unsafe fn cudaDeviceGetHostAtomicCapabilities(
        capabilities: *mut ::core::ffi::c_uint,
        operations: *const cudaAtomicOperation,
        count: ::core::ffi::c_uint,
        device: ::core::ffi::c_int,
    ) -> cudaError_t {
        if let Some(__function) = (culib().cudaDeviceGetHostAtomicCapabilities) {
            __function(capabilities, operations, count, device)
        } else {
            cudaError::cudaErrorNotSupported
        }
    }
    pub unsafe fn cudaDeviceGetLimit(pValue: *mut usize, limit: cudaLimit) -> cudaError_t {
        if let Some(__function) = (culib().cudaDeviceGetLimit) {
            __function(pValue, limit)
        } else {
            cudaError::cudaErrorNotSupported
        }
    }
    pub unsafe fn cudaDeviceGetMemPool(
        memPool: *mut cudaMemPool_t,
        device: ::core::ffi::c_int,
    ) -> cudaError_t {
        if let Some(__function) = (culib().cudaDeviceGetMemPool) {
            __function(memPool, device)
        } else {
            cudaError::cudaErrorNotSupported
        }
    }
    #[cfg(any(feature = "cuda-13000"))]
    pub unsafe fn cudaDeviceGetP2PAtomicCapabilities(
        capabilities: *mut ::core::ffi::c_uint,
        operations: *const cudaAtomicOperation,
        count: ::core::ffi::c_uint,
        srcDevice: ::core::ffi::c_int,
        dstDevice: ::core::ffi::c_int,
    ) -> cudaError_t {
        if let Some(__function) = (culib().cudaDeviceGetP2PAtomicCapabilities) {
            __function(capabilities, operations, count, srcDevice, dstDevice)
        } else {
            cudaError::cudaErrorNotSupported
        }
    }
    pub unsafe fn cudaDeviceGetP2PAttribute(
        value: *mut ::core::ffi::c_int,
        attr: cudaDeviceP2PAttr,
        srcDevice: ::core::ffi::c_int,
        dstDevice: ::core::ffi::c_int,
    ) -> cudaError_t {
        if let Some(__function) = (culib().cudaDeviceGetP2PAttribute) {
            __function(value, attr, srcDevice, dstDevice)
        } else {
            cudaError::cudaErrorNotSupported
        }
    }
    pub unsafe fn cudaDeviceGetPCIBusId(
        pciBusId: *mut ::core::ffi::c_char,
        len: ::core::ffi::c_int,
        device: ::core::ffi::c_int,
    ) -> cudaError_t {
        if let Some(__function) = (culib().cudaDeviceGetPCIBusId) {
            __function(pciBusId, len, device)
        } else {
            cudaError::cudaErrorNotSupported
        }
    }
    pub unsafe fn cudaDeviceGetSharedMemConfig(pConfig: *mut cudaSharedMemConfig) -> cudaError_t {
        if let Some(__function) = (culib().cudaDeviceGetSharedMemConfig) {
            __function(pConfig)
        } else {
            cudaError::cudaErrorNotSupported
        }
    }
    pub unsafe fn cudaDeviceGetStreamPriorityRange(
        leastPriority: *mut ::core::ffi::c_int,
        greatestPriority: *mut ::core::ffi::c_int,
    ) -> cudaError_t {
        if let Some(__function) = (culib().cudaDeviceGetStreamPriorityRange) {
            __function(leastPriority, greatestPriority)
        } else {
            cudaError::cudaErrorNotSupported
        }
    }
    pub unsafe fn cudaDeviceGetTexture1DLinearMaxWidth(
        maxWidthInElements: *mut usize,
        fmtDesc: *const cudaChannelFormatDesc,
        device: ::core::ffi::c_int,
    ) -> cudaError_t {
        if let Some(__function) = (culib().cudaDeviceGetTexture1DLinearMaxWidth) {
            __function(maxWidthInElements, fmtDesc, device)
        } else {
            cudaError::cudaErrorNotSupported
        }
    }
    pub unsafe fn cudaDeviceGraphMemTrim(device: ::core::ffi::c_int) -> cudaError_t {
        if let Some(__function) = (culib().cudaDeviceGraphMemTrim) {
            __function(device)
        } else {
            cudaError::cudaErrorNotSupported
        }
    }
    #[cfg(any(
        feature = "cuda-12040",
        feature = "cuda-12050",
        feature = "cuda-12060",
        feature = "cuda-12080",
        feature = "cuda-12090",
        feature = "cuda-13000"
    ))]
    pub unsafe fn cudaDeviceRegisterAsyncNotification(
        device: ::core::ffi::c_int,
        callbackFunc: cudaAsyncCallback,
        userData: *mut ::core::ffi::c_void,
        callback: *mut cudaAsyncCallbackHandle_t,
    ) -> cudaError_t {
        if let Some(__function) = (culib().cudaDeviceRegisterAsyncNotification) {
            __function(device, callbackFunc, userData, callback)
        } else {
            cudaError::cudaErrorNotSupported
        }
    }
    pub unsafe fn cudaDeviceReset() -> cudaError_t {
        if let Some(__function) = (culib().cudaDeviceReset) {
            __function()
        } else {
            cudaError::cudaErrorNotSupported
        }
    }
    pub unsafe fn cudaDeviceSetCacheConfig(cacheConfig: cudaFuncCache) -> cudaError_t {
        if let Some(__function) = (culib().cudaDeviceSetCacheConfig) {
            __function(cacheConfig)
        } else {
            cudaError::cudaErrorNotSupported
        }
    }
    pub unsafe fn cudaDeviceSetGraphMemAttribute(
        device: ::core::ffi::c_int,
        attr: cudaGraphMemAttributeType,
        value: *mut ::core::ffi::c_void,
    ) -> cudaError_t {
        if let Some(__function) = (culib().cudaDeviceSetGraphMemAttribute) {
            __function(device, attr, value)
        } else {
            cudaError::cudaErrorNotSupported
        }
    }
    pub unsafe fn cudaDeviceSetLimit(limit: cudaLimit, value: usize) -> cudaError_t {
        if let Some(__function) = (culib().cudaDeviceSetLimit) {
            __function(limit, value)
        } else {
            cudaError::cudaErrorNotSupported
        }
    }
    pub unsafe fn cudaDeviceSetMemPool(
        device: ::core::ffi::c_int,
        memPool: cudaMemPool_t,
    ) -> cudaError_t {
        if let Some(__function) = (culib().cudaDeviceSetMemPool) {
            __function(device, memPool)
        } else {
            cudaError::cudaErrorNotSupported
        }
    }
    pub unsafe fn cudaDeviceSetSharedMemConfig(config: cudaSharedMemConfig) -> cudaError_t {
        if let Some(__function) = (culib().cudaDeviceSetSharedMemConfig) {
            __function(config)
        } else {
            cudaError::cudaErrorNotSupported
        }
    }
    pub unsafe fn cudaDeviceSynchronize() -> cudaError_t {
        if let Some(__function) = (culib().cudaDeviceSynchronize) {
            __function()
        } else {
            cudaError::cudaErrorNotSupported
        }
    }
    #[cfg(any(
        feature = "cuda-12040",
        feature = "cuda-12050",
        feature = "cuda-12060",
        feature = "cuda-12080",
        feature = "cuda-12090",
        feature = "cuda-13000"
    ))]
    pub unsafe fn cudaDeviceUnregisterAsyncNotification(
        device: ::core::ffi::c_int,
        callback: cudaAsyncCallbackHandle_t,
    ) -> cudaError_t {
        if let Some(__function) = (culib().cudaDeviceUnregisterAsyncNotification) {
            __function(device, callback)
        } else {
            cudaError::cudaErrorNotSupported
        }
    }
    pub unsafe fn cudaDriverGetVersion(driverVersion: *mut ::core::ffi::c_int) -> cudaError_t {
        if let Some(__function) = (culib().cudaDriverGetVersion) {
            __function(driverVersion)
        } else {
            cudaError::cudaErrorNotSupported
        }
    }
    pub unsafe fn cudaEventCreate(event: *mut cudaEvent_t) -> cudaError_t {
        if let Some(__function) = (culib().cudaEventCreate) {
            __function(event)
        } else {
            cudaError::cudaErrorNotSupported
        }
    }
    pub unsafe fn cudaEventCreateWithFlags(
        event: *mut cudaEvent_t,
        flags: ::core::ffi::c_uint,
    ) -> cudaError_t {
        if let Some(__function) = (culib().cudaEventCreateWithFlags) {
            __function(event, flags)
        } else {
            cudaError::cudaErrorNotSupported
        }
    }
    pub unsafe fn cudaEventDestroy(event: cudaEvent_t) -> cudaError_t {
        if let Some(__function) = (culib().cudaEventDestroy) {
            __function(event)
        } else {
            cudaError::cudaErrorNotSupported
        }
    }
    pub unsafe fn cudaEventElapsedTime(
        ms: *mut f32,
        start: cudaEvent_t,
        end: cudaEvent_t,
    ) -> cudaError_t {
        if let Some(__function) = (culib().cudaEventElapsedTime) {
            __function(ms, start, end)
        } else {
            cudaError::cudaErrorNotSupported
        }
    }
    #[cfg(any(feature = "cuda-12080", feature = "cuda-12090"))]
    pub unsafe fn cudaEventElapsedTime_v2(
        ms: *mut f32,
        start: cudaEvent_t,
        end: cudaEvent_t,
    ) -> cudaError_t {
        if let Some(__function) = (culib().cudaEventElapsedTime_v2) {
            __function(ms, start, end)
        } else {
            cudaError::cudaErrorNotSupported
        }
    }
    pub unsafe fn cudaEventQuery(event: cudaEvent_t) -> cudaError_t {
        if let Some(__function) = (culib().cudaEventQuery) {
            __function(event)
        } else {
            cudaError::cudaErrorNotSupported
        }
    }
    pub unsafe fn cudaEventRecord(event: cudaEvent_t, stream: cudaStream_t) -> cudaError_t {
        if let Some(__function) = (culib().cudaEventRecord) {
            __function(event, stream)
        } else {
            cudaError::cudaErrorNotSupported
        }
    }
    pub unsafe fn cudaEventRecordWithFlags(
        event: cudaEvent_t,
        stream: cudaStream_t,
        flags: ::core::ffi::c_uint,
    ) -> cudaError_t {
        if let Some(__function) = (culib().cudaEventRecordWithFlags) {
            __function(event, stream, flags)
        } else {
            cudaError::cudaErrorNotSupported
        }
    }
    pub unsafe fn cudaEventSynchronize(event: cudaEvent_t) -> cudaError_t {
        if let Some(__function) = (culib().cudaEventSynchronize) {
            __function(event)
        } else {
            cudaError::cudaErrorNotSupported
        }
    }
    pub unsafe fn cudaExternalMemoryGetMappedBuffer(
        devPtr: *mut *mut ::core::ffi::c_void,
        extMem: cudaExternalMemory_t,
        bufferDesc: *const cudaExternalMemoryBufferDesc,
    ) -> cudaError_t {
        if let Some(__function) = (culib().cudaExternalMemoryGetMappedBuffer) {
            __function(devPtr, extMem, bufferDesc)
        } else {
            cudaError::cudaErrorNotSupported
        }
    }
    pub unsafe fn cudaExternalMemoryGetMappedMipmappedArray(
        mipmap: *mut cudaMipmappedArray_t,
        extMem: cudaExternalMemory_t,
        mipmapDesc: *const cudaExternalMemoryMipmappedArrayDesc,
    ) -> cudaError_t {
        if let Some(__function) = (culib().cudaExternalMemoryGetMappedMipmappedArray) {
            __function(mipmap, extMem, mipmapDesc)
        } else {
            cudaError::cudaErrorNotSupported
        }
    }
    pub unsafe fn cudaFree(devPtr: *mut ::core::ffi::c_void) -> cudaError_t {
        if let Some(__function) = (culib().cudaFree) {
            __function(devPtr)
        } else {
            cudaError::cudaErrorNotSupported
        }
    }
    pub unsafe fn cudaFreeArray(array: cudaArray_t) -> cudaError_t {
        if let Some(__function) = (culib().cudaFreeArray) {
            __function(array)
        } else {
            cudaError::cudaErrorNotSupported
        }
    }
    pub unsafe fn cudaFreeAsync(
        devPtr: *mut ::core::ffi::c_void,
        hStream: cudaStream_t,
    ) -> cudaError_t {
        if let Some(__function) = (culib().cudaFreeAsync) {
            __function(devPtr, hStream)
        } else {
            cudaError::cudaErrorNotSupported
        }
    }
    pub unsafe fn cudaFreeHost(ptr: *mut ::core::ffi::c_void) -> cudaError_t {
        if let Some(__function) = (culib().cudaFreeHost) {
            __function(ptr)
        } else {
            cudaError::cudaErrorNotSupported
        }
    }
    pub unsafe fn cudaFreeMipmappedArray(mipmappedArray: cudaMipmappedArray_t) -> cudaError_t {
        if let Some(__function) = (culib().cudaFreeMipmappedArray) {
            __function(mipmappedArray)
        } else {
            cudaError::cudaErrorNotSupported
        }
    }
    pub unsafe fn cudaFuncGetAttributes(
        attr: *mut cudaFuncAttributes,
        func: *const ::core::ffi::c_void,
    ) -> cudaError_t {
        if let Some(__function) = (culib().cudaFuncGetAttributes) {
            __function(attr, func)
        } else {
            cudaError::cudaErrorNotSupported
        }
    }
    #[cfg(any(
        feature = "cuda-12030",
        feature = "cuda-12040",
        feature = "cuda-12050",
        feature = "cuda-12060",
        feature = "cuda-12080",
        feature = "cuda-12090",
        feature = "cuda-13000"
    ))]
    pub unsafe fn cudaFuncGetName(
        name: *mut *const ::core::ffi::c_char,
        func: *const ::core::ffi::c_void,
    ) -> cudaError_t {
        if let Some(__function) = (culib().cudaFuncGetName) {
            __function(name, func)
        } else {
            cudaError::cudaErrorNotSupported
        }
    }
    #[cfg(any(
        feature = "cuda-12040",
        feature = "cuda-12050",
        feature = "cuda-12060",
        feature = "cuda-12080",
        feature = "cuda-12090",
        feature = "cuda-13000"
    ))]
    pub unsafe fn cudaFuncGetParamInfo(
        func: *const ::core::ffi::c_void,
        paramIndex: usize,
        paramOffset: *mut usize,
        paramSize: *mut usize,
    ) -> cudaError_t {
        if let Some(__function) = (culib().cudaFuncGetParamInfo) {
            __function(func, paramIndex, paramOffset, paramSize)
        } else {
            cudaError::cudaErrorNotSupported
        }
    }
    pub unsafe fn cudaFuncSetAttribute(
        func: *const ::core::ffi::c_void,
        attr: cudaFuncAttribute,
        value: ::core::ffi::c_int,
    ) -> cudaError_t {
        if let Some(__function) = (culib().cudaFuncSetAttribute) {
            __function(func, attr, value)
        } else {
            cudaError::cudaErrorNotSupported
        }
    }
    pub unsafe fn cudaFuncSetCacheConfig(
        func: *const ::core::ffi::c_void,
        cacheConfig: cudaFuncCache,
    ) -> cudaError_t {
        if let Some(__function) = (culib().cudaFuncSetCacheConfig) {
            __function(func, cacheConfig)
        } else {
            cudaError::cudaErrorNotSupported
        }
    }
    pub unsafe fn cudaFuncSetSharedMemConfig(
        func: *const ::core::ffi::c_void,
        config: cudaSharedMemConfig,
    ) -> cudaError_t {
        if let Some(__function) = (culib().cudaFuncSetSharedMemConfig) {
            __function(func, config)
        } else {
            cudaError::cudaErrorNotSupported
        }
    }
    pub unsafe fn cudaGetChannelDesc(
        desc: *mut cudaChannelFormatDesc,
        array: cudaArray_const_t,
    ) -> cudaError_t {
        if let Some(__function) = (culib().cudaGetChannelDesc) {
            __function(desc, array)
        } else {
            cudaError::cudaErrorNotSupported
        }
    }
    pub unsafe fn cudaGetDevice(device: *mut ::core::ffi::c_int) -> cudaError_t {
        if let Some(__function) = (culib().cudaGetDevice) {
            __function(device)
        } else {
            cudaError::cudaErrorNotSupported
        }
    }
    pub unsafe fn cudaGetDeviceCount(count: *mut ::core::ffi::c_int) -> cudaError_t {
        if let Some(__function) = (culib().cudaGetDeviceCount) {
            __function(count)
        } else {
            cudaError::cudaErrorNotSupported
        }
    }
    pub unsafe fn cudaGetDeviceFlags(flags: *mut ::core::ffi::c_uint) -> cudaError_t {
        if let Some(__function) = (culib().cudaGetDeviceFlags) {
            __function(flags)
        } else {
            cudaError::cudaErrorNotSupported
        }
    }
    #[cfg(any(
        feature = "cuda-11040",
        feature = "cuda-11050",
        feature = "cuda-11060",
        feature = "cuda-11070",
        feature = "cuda-11080",
        feature = "cuda-13000"
    ))]
    pub unsafe fn cudaGetDeviceProperties(
        prop: *mut cudaDeviceProp,
        device: ::core::ffi::c_int,
    ) -> cudaError_t {
        if let Some(__function) = (culib().cudaGetDeviceProperties) {
            __function(prop, device)
        } else {
            cudaError::cudaErrorNotSupported
        }
    }
    #[cfg(any(
        feature = "cuda-12000",
        feature = "cuda-12010",
        feature = "cuda-12020",
        feature = "cuda-12030",
        feature = "cuda-12040",
        feature = "cuda-12050",
        feature = "cuda-12060",
        feature = "cuda-12080",
        feature = "cuda-12090"
    ))]
    pub unsafe fn cudaGetDeviceProperties_v2(
        prop: *mut cudaDeviceProp,
        device: ::core::ffi::c_int,
    ) -> cudaError_t {
        if let Some(__function) = (culib().cudaGetDeviceProperties_v2) {
            __function(prop, device)
        } else {
            cudaError::cudaErrorNotSupported
        }
    }
    #[cfg(any(
        feature = "cuda-11040",
        feature = "cuda-11050",
        feature = "cuda-11060",
        feature = "cuda-11070",
        feature = "cuda-11080"
    ))]
    pub unsafe fn cudaGetDriverEntryPoint(
        symbol: *const ::core::ffi::c_char,
        funcPtr: *mut *mut ::core::ffi::c_void,
        flags: ::core::ffi::c_ulonglong,
    ) -> cudaError_t {
        if let Some(__function) = (culib().cudaGetDriverEntryPoint) {
            __function(symbol, funcPtr, flags)
        } else {
            cudaError::cudaErrorNotSupported
        }
    }
    #[cfg(any(
        feature = "cuda-12000",
        feature = "cuda-12010",
        feature = "cuda-12020",
        feature = "cuda-12030",
        feature = "cuda-12040",
        feature = "cuda-12050",
        feature = "cuda-12060",
        feature = "cuda-12080",
        feature = "cuda-12090",
        feature = "cuda-13000"
    ))]
    pub unsafe fn cudaGetDriverEntryPoint(
        symbol: *const ::core::ffi::c_char,
        funcPtr: *mut *mut ::core::ffi::c_void,
        flags: ::core::ffi::c_ulonglong,
        driverStatus: *mut cudaDriverEntryPointQueryResult,
    ) -> cudaError_t {
        if let Some(__function) = (culib().cudaGetDriverEntryPoint) {
            __function(symbol, funcPtr, flags, driverStatus)
        } else {
            cudaError::cudaErrorNotSupported
        }
    }
    #[cfg(any(
        feature = "cuda-12050",
        feature = "cuda-12060",
        feature = "cuda-12080",
        feature = "cuda-12090",
        feature = "cuda-13000"
    ))]
    pub unsafe fn cudaGetDriverEntryPointByVersion(
        symbol: *const ::core::ffi::c_char,
        funcPtr: *mut *mut ::core::ffi::c_void,
        cudaVersion: ::core::ffi::c_uint,
        flags: ::core::ffi::c_ulonglong,
        driverStatus: *mut cudaDriverEntryPointQueryResult,
    ) -> cudaError_t {
        if let Some(__function) = (culib().cudaGetDriverEntryPointByVersion) {
            __function(symbol, funcPtr, cudaVersion, flags, driverStatus)
        } else {
            cudaError::cudaErrorNotSupported
        }
    }
    pub unsafe fn cudaGetErrorName(error: cudaError_t) -> *const ::core::ffi::c_char {
        if let Some(__function) = (culib().cudaGetErrorName) {
            __function(error)
        } else {
            panic!(
                "cudaGetErrorName was not found during dynamic loading - this indicates a CUDA toolkit or driver version mismatch",
            )
        }
    }
    pub unsafe fn cudaGetErrorString(error: cudaError_t) -> *const ::core::ffi::c_char {
        if let Some(__function) = (culib().cudaGetErrorString) {
            __function(error)
        } else {
            panic!(
                "cudaGetErrorString was not found during dynamic loading - this indicates a CUDA toolkit or driver version mismatch",
            )
        }
    }
    pub unsafe fn cudaGetExportTable(
        ppExportTable: *mut *const ::core::ffi::c_void,
        pExportTableId: *const cudaUUID_t,
    ) -> cudaError_t {
        if let Some(__function) = (culib().cudaGetExportTable) {
            __function(ppExportTable, pExportTableId)
        } else {
            cudaError::cudaErrorNotSupported
        }
    }
    pub unsafe fn cudaGetFuncBySymbol(
        functionPtr: *mut cudaFunction_t,
        symbolPtr: *const ::core::ffi::c_void,
    ) -> cudaError_t {
        if let Some(__function) = (culib().cudaGetFuncBySymbol) {
            __function(functionPtr, symbolPtr)
        } else {
            cudaError::cudaErrorNotSupported
        }
    }
    #[cfg(any(
        feature = "cuda-12010",
        feature = "cuda-12020",
        feature = "cuda-12030",
        feature = "cuda-12040",
        feature = "cuda-12050",
        feature = "cuda-12060",
        feature = "cuda-12080",
        feature = "cuda-12090",
        feature = "cuda-13000"
    ))]
    pub unsafe fn cudaGetKernel(
        kernelPtr: *mut cudaKernel_t,
        entryFuncAddr: *const ::core::ffi::c_void,
    ) -> cudaError_t {
        if let Some(__function) = (culib().cudaGetKernel) {
            __function(kernelPtr, entryFuncAddr)
        } else {
            cudaError::cudaErrorNotSupported
        }
    }
    pub unsafe fn cudaGetLastError() -> cudaError_t {
        if let Some(__function) = (culib().cudaGetLastError) {
            __function()
        } else {
            cudaError::cudaErrorNotSupported
        }
    }
    pub unsafe fn cudaGetMipmappedArrayLevel(
        levelArray: *mut cudaArray_t,
        mipmappedArray: cudaMipmappedArray_const_t,
        level: ::core::ffi::c_uint,
    ) -> cudaError_t {
        if let Some(__function) = (culib().cudaGetMipmappedArrayLevel) {
            __function(levelArray, mipmappedArray, level)
        } else {
            cudaError::cudaErrorNotSupported
        }
    }
    pub unsafe fn cudaGetSurfaceObjectResourceDesc(
        pResDesc: *mut cudaResourceDesc,
        surfObject: cudaSurfaceObject_t,
    ) -> cudaError_t {
        if let Some(__function) = (culib().cudaGetSurfaceObjectResourceDesc) {
            __function(pResDesc, surfObject)
        } else {
            cudaError::cudaErrorNotSupported
        }
    }
    #[cfg(any(
        feature = "cuda-11040",
        feature = "cuda-11050",
        feature = "cuda-11060",
        feature = "cuda-11070",
        feature = "cuda-11080"
    ))]
    pub unsafe fn cudaGetSurfaceReference(
        surfref: *mut *const surfaceReference,
        symbol: *const ::core::ffi::c_void,
    ) -> cudaError_t {
        if let Some(__function) = (culib().cudaGetSurfaceReference) {
            __function(surfref, symbol)
        } else {
            cudaError::cudaErrorNotSupported
        }
    }
    pub unsafe fn cudaGetSymbolAddress(
        devPtr: *mut *mut ::core::ffi::c_void,
        symbol: *const ::core::ffi::c_void,
    ) -> cudaError_t {
        if let Some(__function) = (culib().cudaGetSymbolAddress) {
            __function(devPtr, symbol)
        } else {
            cudaError::cudaErrorNotSupported
        }
    }
    pub unsafe fn cudaGetSymbolSize(
        size: *mut usize,
        symbol: *const ::core::ffi::c_void,
    ) -> cudaError_t {
        if let Some(__function) = (culib().cudaGetSymbolSize) {
            __function(size, symbol)
        } else {
            cudaError::cudaErrorNotSupported
        }
    }
    #[cfg(any(
        feature = "cuda-11040",
        feature = "cuda-11050",
        feature = "cuda-11060",
        feature = "cuda-11070",
        feature = "cuda-11080"
    ))]
    pub unsafe fn cudaGetTextureAlignmentOffset(
        offset: *mut usize,
        texref: *const textureReference,
    ) -> cudaError_t {
        if let Some(__function) = (culib().cudaGetTextureAlignmentOffset) {
            __function(offset, texref)
        } else {
            cudaError::cudaErrorNotSupported
        }
    }
    pub unsafe fn cudaGetTextureObjectResourceDesc(
        pResDesc: *mut cudaResourceDesc,
        texObject: cudaTextureObject_t,
    ) -> cudaError_t {
        if let Some(__function) = (culib().cudaGetTextureObjectResourceDesc) {
            __function(pResDesc, texObject)
        } else {
            cudaError::cudaErrorNotSupported
        }
    }
    pub unsafe fn cudaGetTextureObjectResourceViewDesc(
        pResViewDesc: *mut cudaResourceViewDesc,
        texObject: cudaTextureObject_t,
    ) -> cudaError_t {
        if let Some(__function) = (culib().cudaGetTextureObjectResourceViewDesc) {
            __function(pResViewDesc, texObject)
        } else {
            cudaError::cudaErrorNotSupported
        }
    }
    pub unsafe fn cudaGetTextureObjectTextureDesc(
        pTexDesc: *mut cudaTextureDesc,
        texObject: cudaTextureObject_t,
    ) -> cudaError_t {
        if let Some(__function) = (culib().cudaGetTextureObjectTextureDesc) {
            __function(pTexDesc, texObject)
        } else {
            cudaError::cudaErrorNotSupported
        }
    }
    #[cfg(any(feature = "cuda-11080"))]
    pub unsafe fn cudaGetTextureObjectTextureDesc_v2(
        pTexDesc: *mut cudaTextureDesc_v2,
        texObject: cudaTextureObject_t,
    ) -> cudaError_t {
        if let Some(__function) = (culib().cudaGetTextureObjectTextureDesc_v2) {
            __function(pTexDesc, texObject)
        } else {
            cudaError::cudaErrorNotSupported
        }
    }
    #[cfg(any(
        feature = "cuda-11040",
        feature = "cuda-11050",
        feature = "cuda-11060",
        feature = "cuda-11070",
        feature = "cuda-11080"
    ))]
    pub unsafe fn cudaGetTextureReference(
        texref: *mut *const textureReference,
        symbol: *const ::core::ffi::c_void,
    ) -> cudaError_t {
        if let Some(__function) = (culib().cudaGetTextureReference) {
            __function(texref, symbol)
        } else {
            cudaError::cudaErrorNotSupported
        }
    }
    pub unsafe fn cudaGraphAddChildGraphNode(
        pGraphNode: *mut cudaGraphNode_t,
        graph: cudaGraph_t,
        pDependencies: *const cudaGraphNode_t,
        numDependencies: usize,
        childGraph: cudaGraph_t,
    ) -> cudaError_t {
        if let Some(__function) = (culib().cudaGraphAddChildGraphNode) {
            __function(
                pGraphNode,
                graph,
                pDependencies,
                numDependencies,
                childGraph,
            )
        } else {
            cudaError::cudaErrorNotSupported
        }
    }
    #[cfg(any(
        feature = "cuda-11040",
        feature = "cuda-11050",
        feature = "cuda-11060",
        feature = "cuda-11070",
        feature = "cuda-11080",
        feature = "cuda-12000",
        feature = "cuda-12010",
        feature = "cuda-12020",
        feature = "cuda-12030",
        feature = "cuda-12040",
        feature = "cuda-12050",
        feature = "cuda-12060",
        feature = "cuda-12080",
        feature = "cuda-12090"
    ))]
    pub unsafe fn cudaGraphAddDependencies(
        graph: cudaGraph_t,
        from: *const cudaGraphNode_t,
        to: *const cudaGraphNode_t,
        numDependencies: usize,
    ) -> cudaError_t {
        if let Some(__function) = (culib().cudaGraphAddDependencies) {
            __function(graph, from, to, numDependencies)
        } else {
            cudaError::cudaErrorNotSupported
        }
    }
    #[cfg(any(feature = "cuda-13000"))]
    pub unsafe fn cudaGraphAddDependencies(
        graph: cudaGraph_t,
        from: *const cudaGraphNode_t,
        to: *const cudaGraphNode_t,
        edgeData: *const cudaGraphEdgeData,
        numDependencies: usize,
    ) -> cudaError_t {
        if let Some(__function) = (culib().cudaGraphAddDependencies) {
            __function(graph, from, to, edgeData, numDependencies)
        } else {
            cudaError::cudaErrorNotSupported
        }
    }
    #[cfg(any(
        feature = "cuda-12030",
        feature = "cuda-12040",
        feature = "cuda-12050",
        feature = "cuda-12060",
        feature = "cuda-12080",
        feature = "cuda-12090"
    ))]
    pub unsafe fn cudaGraphAddDependencies_v2(
        graph: cudaGraph_t,
        from: *const cudaGraphNode_t,
        to: *const cudaGraphNode_t,
        edgeData: *const cudaGraphEdgeData,
        numDependencies: usize,
    ) -> cudaError_t {
        if let Some(__function) = (culib().cudaGraphAddDependencies_v2) {
            __function(graph, from, to, edgeData, numDependencies)
        } else {
            cudaError::cudaErrorNotSupported
        }
    }
    pub unsafe fn cudaGraphAddEmptyNode(
        pGraphNode: *mut cudaGraphNode_t,
        graph: cudaGraph_t,
        pDependencies: *const cudaGraphNode_t,
        numDependencies: usize,
    ) -> cudaError_t {
        if let Some(__function) = (culib().cudaGraphAddEmptyNode) {
            __function(pGraphNode, graph, pDependencies, numDependencies)
        } else {
            cudaError::cudaErrorNotSupported
        }
    }
    pub unsafe fn cudaGraphAddEventRecordNode(
        pGraphNode: *mut cudaGraphNode_t,
        graph: cudaGraph_t,
        pDependencies: *const cudaGraphNode_t,
        numDependencies: usize,
        event: cudaEvent_t,
    ) -> cudaError_t {
        if let Some(__function) = (culib().cudaGraphAddEventRecordNode) {
            __function(pGraphNode, graph, pDependencies, numDependencies, event)
        } else {
            cudaError::cudaErrorNotSupported
        }
    }
    pub unsafe fn cudaGraphAddEventWaitNode(
        pGraphNode: *mut cudaGraphNode_t,
        graph: cudaGraph_t,
        pDependencies: *const cudaGraphNode_t,
        numDependencies: usize,
        event: cudaEvent_t,
    ) -> cudaError_t {
        if let Some(__function) = (culib().cudaGraphAddEventWaitNode) {
            __function(pGraphNode, graph, pDependencies, numDependencies, event)
        } else {
            cudaError::cudaErrorNotSupported
        }
    }
    pub unsafe fn cudaGraphAddExternalSemaphoresSignalNode(
        pGraphNode: *mut cudaGraphNode_t,
        graph: cudaGraph_t,
        pDependencies: *const cudaGraphNode_t,
        numDependencies: usize,
        nodeParams: *const cudaExternalSemaphoreSignalNodeParams,
    ) -> cudaError_t {
        if let Some(__function) = (culib().cudaGraphAddExternalSemaphoresSignalNode) {
            __function(
                pGraphNode,
                graph,
                pDependencies,
                numDependencies,
                nodeParams,
            )
        } else {
            cudaError::cudaErrorNotSupported
        }
    }
    pub unsafe fn cudaGraphAddExternalSemaphoresWaitNode(
        pGraphNode: *mut cudaGraphNode_t,
        graph: cudaGraph_t,
        pDependencies: *const cudaGraphNode_t,
        numDependencies: usize,
        nodeParams: *const cudaExternalSemaphoreWaitNodeParams,
    ) -> cudaError_t {
        if let Some(__function) = (culib().cudaGraphAddExternalSemaphoresWaitNode) {
            __function(
                pGraphNode,
                graph,
                pDependencies,
                numDependencies,
                nodeParams,
            )
        } else {
            cudaError::cudaErrorNotSupported
        }
    }
    pub unsafe fn cudaGraphAddHostNode(
        pGraphNode: *mut cudaGraphNode_t,
        graph: cudaGraph_t,
        pDependencies: *const cudaGraphNode_t,
        numDependencies: usize,
        pNodeParams: *const cudaHostNodeParams,
    ) -> cudaError_t {
        if let Some(__function) = (culib().cudaGraphAddHostNode) {
            __function(
                pGraphNode,
                graph,
                pDependencies,
                numDependencies,
                pNodeParams,
            )
        } else {
            cudaError::cudaErrorNotSupported
        }
    }
    pub unsafe fn cudaGraphAddKernelNode(
        pGraphNode: *mut cudaGraphNode_t,
        graph: cudaGraph_t,
        pDependencies: *const cudaGraphNode_t,
        numDependencies: usize,
        pNodeParams: *const cudaKernelNodeParams,
    ) -> cudaError_t {
        if let Some(__function) = (culib().cudaGraphAddKernelNode) {
            __function(
                pGraphNode,
                graph,
                pDependencies,
                numDependencies,
                pNodeParams,
            )
        } else {
            cudaError::cudaErrorNotSupported
        }
    }
    pub unsafe fn cudaGraphAddMemAllocNode(
        pGraphNode: *mut cudaGraphNode_t,
        graph: cudaGraph_t,
        pDependencies: *const cudaGraphNode_t,
        numDependencies: usize,
        nodeParams: *mut cudaMemAllocNodeParams,
    ) -> cudaError_t {
        if let Some(__function) = (culib().cudaGraphAddMemAllocNode) {
            __function(
                pGraphNode,
                graph,
                pDependencies,
                numDependencies,
                nodeParams,
            )
        } else {
            cudaError::cudaErrorNotSupported
        }
    }
    pub unsafe fn cudaGraphAddMemFreeNode(
        pGraphNode: *mut cudaGraphNode_t,
        graph: cudaGraph_t,
        pDependencies: *const cudaGraphNode_t,
        numDependencies: usize,
        dptr: *mut ::core::ffi::c_void,
    ) -> cudaError_t {
        if let Some(__function) = (culib().cudaGraphAddMemFreeNode) {
            __function(pGraphNode, graph, pDependencies, numDependencies, dptr)
        } else {
            cudaError::cudaErrorNotSupported
        }
    }
    pub unsafe fn cudaGraphAddMemcpyNode(
        pGraphNode: *mut cudaGraphNode_t,
        graph: cudaGraph_t,
        pDependencies: *const cudaGraphNode_t,
        numDependencies: usize,
        pCopyParams: *const cudaMemcpy3DParms,
    ) -> cudaError_t {
        if let Some(__function) = (culib().cudaGraphAddMemcpyNode) {
            __function(
                pGraphNode,
                graph,
                pDependencies,
                numDependencies,
                pCopyParams,
            )
        } else {
            cudaError::cudaErrorNotSupported
        }
    }
    pub unsafe fn cudaGraphAddMemcpyNode1D(
        pGraphNode: *mut cudaGraphNode_t,
        graph: cudaGraph_t,
        pDependencies: *const cudaGraphNode_t,
        numDependencies: usize,
        dst: *mut ::core::ffi::c_void,
        src: *const ::core::ffi::c_void,
        count: usize,
        kind: cudaMemcpyKind,
    ) -> cudaError_t {
        if let Some(__function) = (culib().cudaGraphAddMemcpyNode1D) {
            __function(
                pGraphNode,
                graph,
                pDependencies,
                numDependencies,
                dst,
                src,
                count,
                kind,
            )
        } else {
            cudaError::cudaErrorNotSupported
        }
    }
    pub unsafe fn cudaGraphAddMemcpyNodeFromSymbol(
        pGraphNode: *mut cudaGraphNode_t,
        graph: cudaGraph_t,
        pDependencies: *const cudaGraphNode_t,
        numDependencies: usize,
        dst: *mut ::core::ffi::c_void,
        symbol: *const ::core::ffi::c_void,
        count: usize,
        offset: usize,
        kind: cudaMemcpyKind,
    ) -> cudaError_t {
        if let Some(__function) = (culib().cudaGraphAddMemcpyNodeFromSymbol) {
            __function(
                pGraphNode,
                graph,
                pDependencies,
                numDependencies,
                dst,
                symbol,
                count,
                offset,
                kind,
            )
        } else {
            cudaError::cudaErrorNotSupported
        }
    }
    pub unsafe fn cudaGraphAddMemcpyNodeToSymbol(
        pGraphNode: *mut cudaGraphNode_t,
        graph: cudaGraph_t,
        pDependencies: *const cudaGraphNode_t,
        numDependencies: usize,
        symbol: *const ::core::ffi::c_void,
        src: *const ::core::ffi::c_void,
        count: usize,
        offset: usize,
        kind: cudaMemcpyKind,
    ) -> cudaError_t {
        if let Some(__function) = (culib().cudaGraphAddMemcpyNodeToSymbol) {
            __function(
                pGraphNode,
                graph,
                pDependencies,
                numDependencies,
                symbol,
                src,
                count,
                offset,
                kind,
            )
        } else {
            cudaError::cudaErrorNotSupported
        }
    }
    pub unsafe fn cudaGraphAddMemsetNode(
        pGraphNode: *mut cudaGraphNode_t,
        graph: cudaGraph_t,
        pDependencies: *const cudaGraphNode_t,
        numDependencies: usize,
        pMemsetParams: *const cudaMemsetParams,
    ) -> cudaError_t {
        if let Some(__function) = (culib().cudaGraphAddMemsetNode) {
            __function(
                pGraphNode,
                graph,
                pDependencies,
                numDependencies,
                pMemsetParams,
            )
        } else {
            cudaError::cudaErrorNotSupported
        }
    }
    #[cfg(any(
        feature = "cuda-12020",
        feature = "cuda-12030",
        feature = "cuda-12040",
        feature = "cuda-12050",
        feature = "cuda-12060",
        feature = "cuda-12080",
        feature = "cuda-12090"
    ))]
    pub unsafe fn cudaGraphAddNode(
        pGraphNode: *mut cudaGraphNode_t,
        graph: cudaGraph_t,
        pDependencies: *const cudaGraphNode_t,
        numDependencies: usize,
        nodeParams: *mut cudaGraphNodeParams,
    ) -> cudaError_t {
        if let Some(__function) = (culib().cudaGraphAddNode) {
            __function(
                pGraphNode,
                graph,
                pDependencies,
                numDependencies,
                nodeParams,
            )
        } else {
            cudaError::cudaErrorNotSupported
        }
    }
    #[cfg(any(feature = "cuda-13000"))]
    pub unsafe fn cudaGraphAddNode(
        pGraphNode: *mut cudaGraphNode_t,
        graph: cudaGraph_t,
        pDependencies: *const cudaGraphNode_t,
        dependencyData: *const cudaGraphEdgeData,
        numDependencies: usize,
        nodeParams: *mut cudaGraphNodeParams,
    ) -> cudaError_t {
        if let Some(__function) = (culib().cudaGraphAddNode) {
            __function(
                pGraphNode,
                graph,
                pDependencies,
                dependencyData,
                numDependencies,
                nodeParams,
            )
        } else {
            cudaError::cudaErrorNotSupported
        }
    }
    #[cfg(any(
        feature = "cuda-12030",
        feature = "cuda-12040",
        feature = "cuda-12050",
        feature = "cuda-12060",
        feature = "cuda-12080",
        feature = "cuda-12090"
    ))]
    pub unsafe fn cudaGraphAddNode_v2(
        pGraphNode: *mut cudaGraphNode_t,
        graph: cudaGraph_t,
        pDependencies: *const cudaGraphNode_t,
        dependencyData: *const cudaGraphEdgeData,
        numDependencies: usize,
        nodeParams: *mut cudaGraphNodeParams,
    ) -> cudaError_t {
        if let Some(__function) = (culib().cudaGraphAddNode_v2) {
            __function(
                pGraphNode,
                graph,
                pDependencies,
                dependencyData,
                numDependencies,
                nodeParams,
            )
        } else {
            cudaError::cudaErrorNotSupported
        }
    }
    pub unsafe fn cudaGraphChildGraphNodeGetGraph(
        node: cudaGraphNode_t,
        pGraph: *mut cudaGraph_t,
    ) -> cudaError_t {
        if let Some(__function) = (culib().cudaGraphChildGraphNodeGetGraph) {
            __function(node, pGraph)
        } else {
            cudaError::cudaErrorNotSupported
        }
    }
    pub unsafe fn cudaGraphClone(
        pGraphClone: *mut cudaGraph_t,
        originalGraph: cudaGraph_t,
    ) -> cudaError_t {
        if let Some(__function) = (culib().cudaGraphClone) {
            __function(pGraphClone, originalGraph)
        } else {
            cudaError::cudaErrorNotSupported
        }
    }
    #[cfg(any(
        feature = "cuda-12030",
        feature = "cuda-12040",
        feature = "cuda-12050",
        feature = "cuda-12060",
        feature = "cuda-12080",
        feature = "cuda-12090",
        feature = "cuda-13000"
    ))]
    pub unsafe fn cudaGraphConditionalHandleCreate(
        pHandle_out: *mut cudaGraphConditionalHandle,
        graph: cudaGraph_t,
        defaultLaunchValue: ::core::ffi::c_uint,
        flags: ::core::ffi::c_uint,
    ) -> cudaError_t {
        if let Some(__function) = (culib().cudaGraphConditionalHandleCreate) {
            __function(pHandle_out, graph, defaultLaunchValue, flags)
        } else {
            cudaError::cudaErrorNotSupported
        }
    }
    pub unsafe fn cudaGraphCreate(
        pGraph: *mut cudaGraph_t,
        flags: ::core::ffi::c_uint,
    ) -> cudaError_t {
        if let Some(__function) = (culib().cudaGraphCreate) {
            __function(pGraph, flags)
        } else {
            cudaError::cudaErrorNotSupported
        }
    }
    pub unsafe fn cudaGraphDebugDotPrint(
        graph: cudaGraph_t,
        path: *const ::core::ffi::c_char,
        flags: ::core::ffi::c_uint,
    ) -> cudaError_t {
        if let Some(__function) = (culib().cudaGraphDebugDotPrint) {
            __function(graph, path, flags)
        } else {
            cudaError::cudaErrorNotSupported
        }
    }
    pub unsafe fn cudaGraphDestroy(graph: cudaGraph_t) -> cudaError_t {
        if let Some(__function) = (culib().cudaGraphDestroy) {
            __function(graph)
        } else {
            cudaError::cudaErrorNotSupported
        }
    }
    pub unsafe fn cudaGraphDestroyNode(node: cudaGraphNode_t) -> cudaError_t {
        if let Some(__function) = (culib().cudaGraphDestroyNode) {
            __function(node)
        } else {
            cudaError::cudaErrorNotSupported
        }
    }
    pub unsafe fn cudaGraphEventRecordNodeGetEvent(
        node: cudaGraphNode_t,
        event_out: *mut cudaEvent_t,
    ) -> cudaError_t {
        if let Some(__function) = (culib().cudaGraphEventRecordNodeGetEvent) {
            __function(node, event_out)
        } else {
            cudaError::cudaErrorNotSupported
        }
    }
    pub unsafe fn cudaGraphEventRecordNodeSetEvent(
        node: cudaGraphNode_t,
        event: cudaEvent_t,
    ) -> cudaError_t {
        if let Some(__function) = (culib().cudaGraphEventRecordNodeSetEvent) {
            __function(node, event)
        } else {
            cudaError::cudaErrorNotSupported
        }
    }
    pub unsafe fn cudaGraphEventWaitNodeGetEvent(
        node: cudaGraphNode_t,
        event_out: *mut cudaEvent_t,
    ) -> cudaError_t {
        if let Some(__function) = (culib().cudaGraphEventWaitNodeGetEvent) {
            __function(node, event_out)
        } else {
            cudaError::cudaErrorNotSupported
        }
    }
    pub unsafe fn cudaGraphEventWaitNodeSetEvent(
        node: cudaGraphNode_t,
        event: cudaEvent_t,
    ) -> cudaError_t {
        if let Some(__function) = (culib().cudaGraphEventWaitNodeSetEvent) {
            __function(node, event)
        } else {
            cudaError::cudaErrorNotSupported
        }
    }
    pub unsafe fn cudaGraphExecChildGraphNodeSetParams(
        hGraphExec: cudaGraphExec_t,
        node: cudaGraphNode_t,
        childGraph: cudaGraph_t,
    ) -> cudaError_t {
        if let Some(__function) = (culib().cudaGraphExecChildGraphNodeSetParams) {
            __function(hGraphExec, node, childGraph)
        } else {
            cudaError::cudaErrorNotSupported
        }
    }
    pub unsafe fn cudaGraphExecDestroy(graphExec: cudaGraphExec_t) -> cudaError_t {
        if let Some(__function) = (culib().cudaGraphExecDestroy) {
            __function(graphExec)
        } else {
            cudaError::cudaErrorNotSupported
        }
    }
    pub unsafe fn cudaGraphExecEventRecordNodeSetEvent(
        hGraphExec: cudaGraphExec_t,
        hNode: cudaGraphNode_t,
        event: cudaEvent_t,
    ) -> cudaError_t {
        if let Some(__function) = (culib().cudaGraphExecEventRecordNodeSetEvent) {
            __function(hGraphExec, hNode, event)
        } else {
            cudaError::cudaErrorNotSupported
        }
    }
    pub unsafe fn cudaGraphExecEventWaitNodeSetEvent(
        hGraphExec: cudaGraphExec_t,
        hNode: cudaGraphNode_t,
        event: cudaEvent_t,
    ) -> cudaError_t {
        if let Some(__function) = (culib().cudaGraphExecEventWaitNodeSetEvent) {
            __function(hGraphExec, hNode, event)
        } else {
            cudaError::cudaErrorNotSupported
        }
    }
    pub unsafe fn cudaGraphExecExternalSemaphoresSignalNodeSetParams(
        hGraphExec: cudaGraphExec_t,
        hNode: cudaGraphNode_t,
        nodeParams: *const cudaExternalSemaphoreSignalNodeParams,
    ) -> cudaError_t {
        if let Some(__function) = (culib().cudaGraphExecExternalSemaphoresSignalNodeSetParams) {
            __function(hGraphExec, hNode, nodeParams)
        } else {
            cudaError::cudaErrorNotSupported
        }
    }
    pub unsafe fn cudaGraphExecExternalSemaphoresWaitNodeSetParams(
        hGraphExec: cudaGraphExec_t,
        hNode: cudaGraphNode_t,
        nodeParams: *const cudaExternalSemaphoreWaitNodeParams,
    ) -> cudaError_t {
        if let Some(__function) = (culib().cudaGraphExecExternalSemaphoresWaitNodeSetParams) {
            __function(hGraphExec, hNode, nodeParams)
        } else {
            cudaError::cudaErrorNotSupported
        }
    }
    #[cfg(any(
        feature = "cuda-12000",
        feature = "cuda-12010",
        feature = "cuda-12020",
        feature = "cuda-12030",
        feature = "cuda-12040",
        feature = "cuda-12050",
        feature = "cuda-12060",
        feature = "cuda-12080",
        feature = "cuda-12090",
        feature = "cuda-13000"
    ))]
    pub unsafe fn cudaGraphExecGetFlags(
        graphExec: cudaGraphExec_t,
        flags: *mut ::core::ffi::c_ulonglong,
    ) -> cudaError_t {
        if let Some(__function) = (culib().cudaGraphExecGetFlags) {
            __function(graphExec, flags)
        } else {
            cudaError::cudaErrorNotSupported
        }
    }
    pub unsafe fn cudaGraphExecHostNodeSetParams(
        hGraphExec: cudaGraphExec_t,
        node: cudaGraphNode_t,
        pNodeParams: *const cudaHostNodeParams,
    ) -> cudaError_t {
        if let Some(__function) = (culib().cudaGraphExecHostNodeSetParams) {
            __function(hGraphExec, node, pNodeParams)
        } else {
            cudaError::cudaErrorNotSupported
        }
    }
    pub unsafe fn cudaGraphExecKernelNodeSetParams(
        hGraphExec: cudaGraphExec_t,
        node: cudaGraphNode_t,
        pNodeParams: *const cudaKernelNodeParams,
    ) -> cudaError_t {
        if let Some(__function) = (culib().cudaGraphExecKernelNodeSetParams) {
            __function(hGraphExec, node, pNodeParams)
        } else {
            cudaError::cudaErrorNotSupported
        }
    }
    pub unsafe fn cudaGraphExecMemcpyNodeSetParams(
        hGraphExec: cudaGraphExec_t,
        node: cudaGraphNode_t,
        pNodeParams: *const cudaMemcpy3DParms,
    ) -> cudaError_t {
        if let Some(__function) = (culib().cudaGraphExecMemcpyNodeSetParams) {
            __function(hGraphExec, node, pNodeParams)
        } else {
            cudaError::cudaErrorNotSupported
        }
    }
    pub unsafe fn cudaGraphExecMemcpyNodeSetParams1D(
        hGraphExec: cudaGraphExec_t,
        node: cudaGraphNode_t,
        dst: *mut ::core::ffi::c_void,
        src: *const ::core::ffi::c_void,
        count: usize,
        kind: cudaMemcpyKind,
    ) -> cudaError_t {
        if let Some(__function) = (culib().cudaGraphExecMemcpyNodeSetParams1D) {
            __function(hGraphExec, node, dst, src, count, kind)
        } else {
            cudaError::cudaErrorNotSupported
        }
    }
    pub unsafe fn cudaGraphExecMemcpyNodeSetParamsFromSymbol(
        hGraphExec: cudaGraphExec_t,
        node: cudaGraphNode_t,
        dst: *mut ::core::ffi::c_void,
        symbol: *const ::core::ffi::c_void,
        count: usize,
        offset: usize,
        kind: cudaMemcpyKind,
    ) -> cudaError_t {
        if let Some(__function) = (culib().cudaGraphExecMemcpyNodeSetParamsFromSymbol) {
            __function(hGraphExec, node, dst, symbol, count, offset, kind)
        } else {
            cudaError::cudaErrorNotSupported
        }
    }
    pub unsafe fn cudaGraphExecMemcpyNodeSetParamsToSymbol(
        hGraphExec: cudaGraphExec_t,
        node: cudaGraphNode_t,
        symbol: *const ::core::ffi::c_void,
        src: *const ::core::ffi::c_void,
        count: usize,
        offset: usize,
        kind: cudaMemcpyKind,
    ) -> cudaError_t {
        if let Some(__function) = (culib().cudaGraphExecMemcpyNodeSetParamsToSymbol) {
            __function(hGraphExec, node, symbol, src, count, offset, kind)
        } else {
            cudaError::cudaErrorNotSupported
        }
    }
    pub unsafe fn cudaGraphExecMemsetNodeSetParams(
        hGraphExec: cudaGraphExec_t,
        node: cudaGraphNode_t,
        pNodeParams: *const cudaMemsetParams,
    ) -> cudaError_t {
        if let Some(__function) = (culib().cudaGraphExecMemsetNodeSetParams) {
            __function(hGraphExec, node, pNodeParams)
        } else {
            cudaError::cudaErrorNotSupported
        }
    }
    #[cfg(any(
        feature = "cuda-12020",
        feature = "cuda-12030",
        feature = "cuda-12040",
        feature = "cuda-12050",
        feature = "cuda-12060",
        feature = "cuda-12080",
        feature = "cuda-12090",
        feature = "cuda-13000"
    ))]
    pub unsafe fn cudaGraphExecNodeSetParams(
        graphExec: cudaGraphExec_t,
        node: cudaGraphNode_t,
        nodeParams: *mut cudaGraphNodeParams,
    ) -> cudaError_t {
        if let Some(__function) = (culib().cudaGraphExecNodeSetParams) {
            __function(graphExec, node, nodeParams)
        } else {
            cudaError::cudaErrorNotSupported
        }
    }
    #[cfg(any(
        feature = "cuda-11040",
        feature = "cuda-11050",
        feature = "cuda-11060",
        feature = "cuda-11070",
        feature = "cuda-11080"
    ))]
    pub unsafe fn cudaGraphExecUpdate(
        hGraphExec: cudaGraphExec_t,
        hGraph: cudaGraph_t,
        hErrorNode_out: *mut cudaGraphNode_t,
        updateResult_out: *mut cudaGraphExecUpdateResult,
    ) -> cudaError_t {
        if let Some(__function) = (culib().cudaGraphExecUpdate) {
            __function(hGraphExec, hGraph, hErrorNode_out, updateResult_out)
        } else {
            cudaError::cudaErrorNotSupported
        }
    }
    #[cfg(any(
        feature = "cuda-12000",
        feature = "cuda-12010",
        feature = "cuda-12020",
        feature = "cuda-12030",
        feature = "cuda-12040",
        feature = "cuda-12050",
        feature = "cuda-12060",
        feature = "cuda-12080",
        feature = "cuda-12090",
        feature = "cuda-13000"
    ))]
    pub unsafe fn cudaGraphExecUpdate(
        hGraphExec: cudaGraphExec_t,
        hGraph: cudaGraph_t,
        resultInfo: *mut cudaGraphExecUpdateResultInfo,
    ) -> cudaError_t {
        if let Some(__function) = (culib().cudaGraphExecUpdate) {
            __function(hGraphExec, hGraph, resultInfo)
        } else {
            cudaError::cudaErrorNotSupported
        }
    }
    pub unsafe fn cudaGraphExternalSemaphoresSignalNodeGetParams(
        hNode: cudaGraphNode_t,
        params_out: *mut cudaExternalSemaphoreSignalNodeParams,
    ) -> cudaError_t {
        if let Some(__function) = (culib().cudaGraphExternalSemaphoresSignalNodeGetParams) {
            __function(hNode, params_out)
        } else {
            cudaError::cudaErrorNotSupported
        }
    }
    pub unsafe fn cudaGraphExternalSemaphoresSignalNodeSetParams(
        hNode: cudaGraphNode_t,
        nodeParams: *const cudaExternalSemaphoreSignalNodeParams,
    ) -> cudaError_t {
        if let Some(__function) = (culib().cudaGraphExternalSemaphoresSignalNodeSetParams) {
            __function(hNode, nodeParams)
        } else {
            cudaError::cudaErrorNotSupported
        }
    }
    pub unsafe fn cudaGraphExternalSemaphoresWaitNodeGetParams(
        hNode: cudaGraphNode_t,
        params_out: *mut cudaExternalSemaphoreWaitNodeParams,
    ) -> cudaError_t {
        if let Some(__function) = (culib().cudaGraphExternalSemaphoresWaitNodeGetParams) {
            __function(hNode, params_out)
        } else {
            cudaError::cudaErrorNotSupported
        }
    }
    pub unsafe fn cudaGraphExternalSemaphoresWaitNodeSetParams(
        hNode: cudaGraphNode_t,
        nodeParams: *const cudaExternalSemaphoreWaitNodeParams,
    ) -> cudaError_t {
        if let Some(__function) = (culib().cudaGraphExternalSemaphoresWaitNodeSetParams) {
            __function(hNode, nodeParams)
        } else {
            cudaError::cudaErrorNotSupported
        }
    }
    #[cfg(any(
        feature = "cuda-11040",
        feature = "cuda-11050",
        feature = "cuda-11060",
        feature = "cuda-11070",
        feature = "cuda-11080",
        feature = "cuda-12000",
        feature = "cuda-12010",
        feature = "cuda-12020",
        feature = "cuda-12030",
        feature = "cuda-12040",
        feature = "cuda-12050",
        feature = "cuda-12060",
        feature = "cuda-12080",
        feature = "cuda-12090"
    ))]
    pub unsafe fn cudaGraphGetEdges(
        graph: cudaGraph_t,
        from: *mut cudaGraphNode_t,
        to: *mut cudaGraphNode_t,
        numEdges: *mut usize,
    ) -> cudaError_t {
        if let Some(__function) = (culib().cudaGraphGetEdges) {
            __function(graph, from, to, numEdges)
        } else {
            cudaError::cudaErrorNotSupported
        }
    }
    #[cfg(any(feature = "cuda-13000"))]
    pub unsafe fn cudaGraphGetEdges(
        graph: cudaGraph_t,
        from: *mut cudaGraphNode_t,
        to: *mut cudaGraphNode_t,
        edgeData: *mut cudaGraphEdgeData,
        numEdges: *mut usize,
    ) -> cudaError_t {
        if let Some(__function) = (culib().cudaGraphGetEdges) {
            __function(graph, from, to, edgeData, numEdges)
        } else {
            cudaError::cudaErrorNotSupported
        }
    }
    #[cfg(any(
        feature = "cuda-12030",
        feature = "cuda-12040",
        feature = "cuda-12050",
        feature = "cuda-12060",
        feature = "cuda-12080",
        feature = "cuda-12090"
    ))]
    pub unsafe fn cudaGraphGetEdges_v2(
        graph: cudaGraph_t,
        from: *mut cudaGraphNode_t,
        to: *mut cudaGraphNode_t,
        edgeData: *mut cudaGraphEdgeData,
        numEdges: *mut usize,
    ) -> cudaError_t {
        if let Some(__function) = (culib().cudaGraphGetEdges_v2) {
            __function(graph, from, to, edgeData, numEdges)
        } else {
            cudaError::cudaErrorNotSupported
        }
    }
    pub unsafe fn cudaGraphGetNodes(
        graph: cudaGraph_t,
        nodes: *mut cudaGraphNode_t,
        numNodes: *mut usize,
    ) -> cudaError_t {
        if let Some(__function) = (culib().cudaGraphGetNodes) {
            __function(graph, nodes, numNodes)
        } else {
            cudaError::cudaErrorNotSupported
        }
    }
    pub unsafe fn cudaGraphGetRootNodes(
        graph: cudaGraph_t,
        pRootNodes: *mut cudaGraphNode_t,
        pNumRootNodes: *mut usize,
    ) -> cudaError_t {
        if let Some(__function) = (culib().cudaGraphGetRootNodes) {
            __function(graph, pRootNodes, pNumRootNodes)
        } else {
            cudaError::cudaErrorNotSupported
        }
    }
    pub unsafe fn cudaGraphHostNodeGetParams(
        node: cudaGraphNode_t,
        pNodeParams: *mut cudaHostNodeParams,
    ) -> cudaError_t {
        if let Some(__function) = (culib().cudaGraphHostNodeGetParams) {
            __function(node, pNodeParams)
        } else {
            cudaError::cudaErrorNotSupported
        }
    }
    pub unsafe fn cudaGraphHostNodeSetParams(
        node: cudaGraphNode_t,
        pNodeParams: *const cudaHostNodeParams,
    ) -> cudaError_t {
        if let Some(__function) = (culib().cudaGraphHostNodeSetParams) {
            __function(node, pNodeParams)
        } else {
            cudaError::cudaErrorNotSupported
        }
    }
    #[cfg(any(
        feature = "cuda-11040",
        feature = "cuda-11050",
        feature = "cuda-11060",
        feature = "cuda-11070",
        feature = "cuda-11080"
    ))]
    pub unsafe fn cudaGraphInstantiate(
        pGraphExec: *mut cudaGraphExec_t,
        graph: cudaGraph_t,
        pErrorNode: *mut cudaGraphNode_t,
        pLogBuffer: *mut ::core::ffi::c_char,
        bufferSize: usize,
    ) -> cudaError_t {
        if let Some(__function) = (culib().cudaGraphInstantiate) {
            __function(pGraphExec, graph, pErrorNode, pLogBuffer, bufferSize)
        } else {
            cudaError::cudaErrorNotSupported
        }
    }
    #[cfg(any(
        feature = "cuda-12000",
        feature = "cuda-12010",
        feature = "cuda-12020",
        feature = "cuda-12030",
        feature = "cuda-12040",
        feature = "cuda-12050",
        feature = "cuda-12060",
        feature = "cuda-12080",
        feature = "cuda-12090",
        feature = "cuda-13000"
    ))]
    pub unsafe fn cudaGraphInstantiate(
        pGraphExec: *mut cudaGraphExec_t,
        graph: cudaGraph_t,
        flags: ::core::ffi::c_ulonglong,
    ) -> cudaError_t {
        if let Some(__function) = (culib().cudaGraphInstantiate) {
            __function(pGraphExec, graph, flags)
        } else {
            cudaError::cudaErrorNotSupported
        }
    }
    pub unsafe fn cudaGraphInstantiateWithFlags(
        pGraphExec: *mut cudaGraphExec_t,
        graph: cudaGraph_t,
        flags: ::core::ffi::c_ulonglong,
    ) -> cudaError_t {
        if let Some(__function) = (culib().cudaGraphInstantiateWithFlags) {
            __function(pGraphExec, graph, flags)
        } else {
            cudaError::cudaErrorNotSupported
        }
    }
    #[cfg(any(
        feature = "cuda-12000",
        feature = "cuda-12010",
        feature = "cuda-12020",
        feature = "cuda-12030",
        feature = "cuda-12040",
        feature = "cuda-12050",
        feature = "cuda-12060",
        feature = "cuda-12080",
        feature = "cuda-12090",
        feature = "cuda-13000"
    ))]
    pub unsafe fn cudaGraphInstantiateWithParams(
        pGraphExec: *mut cudaGraphExec_t,
        graph: cudaGraph_t,
        instantiateParams: *mut cudaGraphInstantiateParams,
    ) -> cudaError_t {
        if let Some(__function) = (culib().cudaGraphInstantiateWithParams) {
            __function(pGraphExec, graph, instantiateParams)
        } else {
            cudaError::cudaErrorNotSupported
        }
    }
    #[cfg(any(
        feature = "cuda-11040",
        feature = "cuda-11050",
        feature = "cuda-11060",
        feature = "cuda-11070",
        feature = "cuda-11080",
        feature = "cuda-12000",
        feature = "cuda-12010",
        feature = "cuda-12020",
        feature = "cuda-12030",
        feature = "cuda-12040",
        feature = "cuda-12050",
        feature = "cuda-12060",
        feature = "cuda-12080",
        feature = "cuda-12090"
    ))]
    pub unsafe fn cudaGraphKernelNodeCopyAttributes(
        hSrc: cudaGraphNode_t,
        hDst: cudaGraphNode_t,
    ) -> cudaError_t {
        if let Some(__function) = (culib().cudaGraphKernelNodeCopyAttributes) {
            __function(hSrc, hDst)
        } else {
            cudaError::cudaErrorNotSupported
        }
    }
    #[cfg(any(feature = "cuda-13000"))]
    pub unsafe fn cudaGraphKernelNodeCopyAttributes(
        hDst: cudaGraphNode_t,
        hSrc: cudaGraphNode_t,
    ) -> cudaError_t {
        if let Some(__function) = (culib().cudaGraphKernelNodeCopyAttributes) {
            __function(hDst, hSrc)
        } else {
            cudaError::cudaErrorNotSupported
        }
    }
    #[cfg(any(
        feature = "cuda-11040",
        feature = "cuda-11050",
        feature = "cuda-11060",
        feature = "cuda-11070"
    ))]
    pub unsafe fn cudaGraphKernelNodeGetAttribute(
        hNode: cudaGraphNode_t,
        attr: cudaKernelNodeAttrID,
        value_out: *mut cudaKernelNodeAttrValue,
    ) -> cudaError_t {
        if let Some(__function) = (culib().cudaGraphKernelNodeGetAttribute) {
            __function(hNode, attr, value_out)
        } else {
            cudaError::cudaErrorNotSupported
        }
    }
    #[cfg(any(
        feature = "cuda-11080",
        feature = "cuda-12000",
        feature = "cuda-12010",
        feature = "cuda-12020",
        feature = "cuda-12030",
        feature = "cuda-12040",
        feature = "cuda-12050",
        feature = "cuda-12060",
        feature = "cuda-12080",
        feature = "cuda-12090",
        feature = "cuda-13000"
    ))]
    pub unsafe fn cudaGraphKernelNodeGetAttribute(
        hNode: cudaGraphNode_t,
        attr: cudaLaunchAttributeID,
        value_out: *mut cudaLaunchAttributeValue,
    ) -> cudaError_t {
        if let Some(__function) = (culib().cudaGraphKernelNodeGetAttribute) {
            __function(hNode, attr, value_out)
        } else {
            cudaError::cudaErrorNotSupported
        }
    }
    pub unsafe fn cudaGraphKernelNodeGetParams(
        node: cudaGraphNode_t,
        pNodeParams: *mut cudaKernelNodeParams,
    ) -> cudaError_t {
        if let Some(__function) = (culib().cudaGraphKernelNodeGetParams) {
            __function(node, pNodeParams)
        } else {
            cudaError::cudaErrorNotSupported
        }
    }
    #[cfg(any(
        feature = "cuda-11040",
        feature = "cuda-11050",
        feature = "cuda-11060",
        feature = "cuda-11070"
    ))]
    pub unsafe fn cudaGraphKernelNodeSetAttribute(
        hNode: cudaGraphNode_t,
        attr: cudaKernelNodeAttrID,
        value: *const cudaKernelNodeAttrValue,
    ) -> cudaError_t {
        if let Some(__function) = (culib().cudaGraphKernelNodeSetAttribute) {
            __function(hNode, attr, value)
        } else {
            cudaError::cudaErrorNotSupported
        }
    }
    #[cfg(any(
        feature = "cuda-11080",
        feature = "cuda-12000",
        feature = "cuda-12010",
        feature = "cuda-12020",
        feature = "cuda-12030",
        feature = "cuda-12040",
        feature = "cuda-12050",
        feature = "cuda-12060",
        feature = "cuda-12080",
        feature = "cuda-12090",
        feature = "cuda-13000"
    ))]
    pub unsafe fn cudaGraphKernelNodeSetAttribute(
        hNode: cudaGraphNode_t,
        attr: cudaLaunchAttributeID,
        value: *const cudaLaunchAttributeValue,
    ) -> cudaError_t {
        if let Some(__function) = (culib().cudaGraphKernelNodeSetAttribute) {
            __function(hNode, attr, value)
        } else {
            cudaError::cudaErrorNotSupported
        }
    }
    pub unsafe fn cudaGraphKernelNodeSetParams(
        node: cudaGraphNode_t,
        pNodeParams: *const cudaKernelNodeParams,
    ) -> cudaError_t {
        if let Some(__function) = (culib().cudaGraphKernelNodeSetParams) {
            __function(node, pNodeParams)
        } else {
            cudaError::cudaErrorNotSupported
        }
    }
    pub unsafe fn cudaGraphLaunch(graphExec: cudaGraphExec_t, stream: cudaStream_t) -> cudaError_t {
        if let Some(__function) = (culib().cudaGraphLaunch) {
            __function(graphExec, stream)
        } else {
            cudaError::cudaErrorNotSupported
        }
    }
    pub unsafe fn cudaGraphMemAllocNodeGetParams(
        node: cudaGraphNode_t,
        params_out: *mut cudaMemAllocNodeParams,
    ) -> cudaError_t {
        if let Some(__function) = (culib().cudaGraphMemAllocNodeGetParams) {
            __function(node, params_out)
        } else {
            cudaError::cudaErrorNotSupported
        }
    }
    pub unsafe fn cudaGraphMemFreeNodeGetParams(
        node: cudaGraphNode_t,
        dptr_out: *mut ::core::ffi::c_void,
    ) -> cudaError_t {
        if let Some(__function) = (culib().cudaGraphMemFreeNodeGetParams) {
            __function(node, dptr_out)
        } else {
            cudaError::cudaErrorNotSupported
        }
    }
    pub unsafe fn cudaGraphMemcpyNodeGetParams(
        node: cudaGraphNode_t,
        pNodeParams: *mut cudaMemcpy3DParms,
    ) -> cudaError_t {
        if let Some(__function) = (culib().cudaGraphMemcpyNodeGetParams) {
            __function(node, pNodeParams)
        } else {
            cudaError::cudaErrorNotSupported
        }
    }
    pub unsafe fn cudaGraphMemcpyNodeSetParams(
        node: cudaGraphNode_t,
        pNodeParams: *const cudaMemcpy3DParms,
    ) -> cudaError_t {
        if let Some(__function) = (culib().cudaGraphMemcpyNodeSetParams) {
            __function(node, pNodeParams)
        } else {
            cudaError::cudaErrorNotSupported
        }
    }
    pub unsafe fn cudaGraphMemcpyNodeSetParams1D(
        node: cudaGraphNode_t,
        dst: *mut ::core::ffi::c_void,
        src: *const ::core::ffi::c_void,
        count: usize,
        kind: cudaMemcpyKind,
    ) -> cudaError_t {
        if let Some(__function) = (culib().cudaGraphMemcpyNodeSetParams1D) {
            __function(node, dst, src, count, kind)
        } else {
            cudaError::cudaErrorNotSupported
        }
    }
    pub unsafe fn cudaGraphMemcpyNodeSetParamsFromSymbol(
        node: cudaGraphNode_t,
        dst: *mut ::core::ffi::c_void,
        symbol: *const ::core::ffi::c_void,
        count: usize,
        offset: usize,
        kind: cudaMemcpyKind,
    ) -> cudaError_t {
        if let Some(__function) = (culib().cudaGraphMemcpyNodeSetParamsFromSymbol) {
            __function(node, dst, symbol, count, offset, kind)
        } else {
            cudaError::cudaErrorNotSupported
        }
    }
    pub unsafe fn cudaGraphMemcpyNodeSetParamsToSymbol(
        node: cudaGraphNode_t,
        symbol: *const ::core::ffi::c_void,
        src: *const ::core::ffi::c_void,
        count: usize,
        offset: usize,
        kind: cudaMemcpyKind,
    ) -> cudaError_t {
        if let Some(__function) = (culib().cudaGraphMemcpyNodeSetParamsToSymbol) {
            __function(node, symbol, src, count, offset, kind)
        } else {
            cudaError::cudaErrorNotSupported
        }
    }
    pub unsafe fn cudaGraphMemsetNodeGetParams(
        node: cudaGraphNode_t,
        pNodeParams: *mut cudaMemsetParams,
    ) -> cudaError_t {
        if let Some(__function) = (culib().cudaGraphMemsetNodeGetParams) {
            __function(node, pNodeParams)
        } else {
            cudaError::cudaErrorNotSupported
        }
    }
    pub unsafe fn cudaGraphMemsetNodeSetParams(
        node: cudaGraphNode_t,
        pNodeParams: *const cudaMemsetParams,
    ) -> cudaError_t {
        if let Some(__function) = (culib().cudaGraphMemsetNodeSetParams) {
            __function(node, pNodeParams)
        } else {
            cudaError::cudaErrorNotSupported
        }
    }
    pub unsafe fn cudaGraphNodeFindInClone(
        pNode: *mut cudaGraphNode_t,
        originalNode: cudaGraphNode_t,
        clonedGraph: cudaGraph_t,
    ) -> cudaError_t {
        if let Some(__function) = (culib().cudaGraphNodeFindInClone) {
            __function(pNode, originalNode, clonedGraph)
        } else {
            cudaError::cudaErrorNotSupported
        }
    }
    #[cfg(any(
        feature = "cuda-11040",
        feature = "cuda-11050",
        feature = "cuda-11060",
        feature = "cuda-11070",
        feature = "cuda-11080",
        feature = "cuda-12000",
        feature = "cuda-12010",
        feature = "cuda-12020",
        feature = "cuda-12030",
        feature = "cuda-12040",
        feature = "cuda-12050",
        feature = "cuda-12060",
        feature = "cuda-12080",
        feature = "cuda-12090"
    ))]
    pub unsafe fn cudaGraphNodeGetDependencies(
        node: cudaGraphNode_t,
        pDependencies: *mut cudaGraphNode_t,
        pNumDependencies: *mut usize,
    ) -> cudaError_t {
        if let Some(__function) = (culib().cudaGraphNodeGetDependencies) {
            __function(node, pDependencies, pNumDependencies)
        } else {
            cudaError::cudaErrorNotSupported
        }
    }
    #[cfg(any(feature = "cuda-13000"))]
    pub unsafe fn cudaGraphNodeGetDependencies(
        node: cudaGraphNode_t,
        pDependencies: *mut cudaGraphNode_t,
        edgeData: *mut cudaGraphEdgeData,
        pNumDependencies: *mut usize,
    ) -> cudaError_t {
        if let Some(__function) = (culib().cudaGraphNodeGetDependencies) {
            __function(node, pDependencies, edgeData, pNumDependencies)
        } else {
            cudaError::cudaErrorNotSupported
        }
    }
    #[cfg(any(
        feature = "cuda-12030",
        feature = "cuda-12040",
        feature = "cuda-12050",
        feature = "cuda-12060",
        feature = "cuda-12080",
        feature = "cuda-12090"
    ))]
    pub unsafe fn cudaGraphNodeGetDependencies_v2(
        node: cudaGraphNode_t,
        pDependencies: *mut cudaGraphNode_t,
        edgeData: *mut cudaGraphEdgeData,
        pNumDependencies: *mut usize,
    ) -> cudaError_t {
        if let Some(__function) = (culib().cudaGraphNodeGetDependencies_v2) {
            __function(node, pDependencies, edgeData, pNumDependencies)
        } else {
            cudaError::cudaErrorNotSupported
        }
    }
    #[cfg(any(
        feature = "cuda-11040",
        feature = "cuda-11050",
        feature = "cuda-11060",
        feature = "cuda-11070",
        feature = "cuda-11080",
        feature = "cuda-12000",
        feature = "cuda-12010",
        feature = "cuda-12020",
        feature = "cuda-12030",
        feature = "cuda-12040",
        feature = "cuda-12050",
        feature = "cuda-12060",
        feature = "cuda-12080",
        feature = "cuda-12090"
    ))]
    pub unsafe fn cudaGraphNodeGetDependentNodes(
        node: cudaGraphNode_t,
        pDependentNodes: *mut cudaGraphNode_t,
        pNumDependentNodes: *mut usize,
    ) -> cudaError_t {
        if let Some(__function) = (culib().cudaGraphNodeGetDependentNodes) {
            __function(node, pDependentNodes, pNumDependentNodes)
        } else {
            cudaError::cudaErrorNotSupported
        }
    }
    #[cfg(any(feature = "cuda-13000"))]
    pub unsafe fn cudaGraphNodeGetDependentNodes(
        node: cudaGraphNode_t,
        pDependentNodes: *mut cudaGraphNode_t,
        edgeData: *mut cudaGraphEdgeData,
        pNumDependentNodes: *mut usize,
    ) -> cudaError_t {
        if let Some(__function) = (culib().cudaGraphNodeGetDependentNodes) {
            __function(node, pDependentNodes, edgeData, pNumDependentNodes)
        } else {
            cudaError::cudaErrorNotSupported
        }
    }
    #[cfg(any(
        feature = "cuda-12030",
        feature = "cuda-12040",
        feature = "cuda-12050",
        feature = "cuda-12060",
        feature = "cuda-12080",
        feature = "cuda-12090"
    ))]
    pub unsafe fn cudaGraphNodeGetDependentNodes_v2(
        node: cudaGraphNode_t,
        pDependentNodes: *mut cudaGraphNode_t,
        edgeData: *mut cudaGraphEdgeData,
        pNumDependentNodes: *mut usize,
    ) -> cudaError_t {
        if let Some(__function) = (culib().cudaGraphNodeGetDependentNodes_v2) {
            __function(node, pDependentNodes, edgeData, pNumDependentNodes)
        } else {
            cudaError::cudaErrorNotSupported
        }
    }
    #[cfg(any(
        feature = "cuda-11060",
        feature = "cuda-11070",
        feature = "cuda-11080",
        feature = "cuda-12000",
        feature = "cuda-12010",
        feature = "cuda-12020",
        feature = "cuda-12030",
        feature = "cuda-12040",
        feature = "cuda-12050",
        feature = "cuda-12060",
        feature = "cuda-12080",
        feature = "cuda-12090",
        feature = "cuda-13000"
    ))]
    pub unsafe fn cudaGraphNodeGetEnabled(
        hGraphExec: cudaGraphExec_t,
        hNode: cudaGraphNode_t,
        isEnabled: *mut ::core::ffi::c_uint,
    ) -> cudaError_t {
        if let Some(__function) = (culib().cudaGraphNodeGetEnabled) {
            __function(hGraphExec, hNode, isEnabled)
        } else {
            cudaError::cudaErrorNotSupported
        }
    }
    pub unsafe fn cudaGraphNodeGetType(
        node: cudaGraphNode_t,
        pType: *mut cudaGraphNodeType,
    ) -> cudaError_t {
        if let Some(__function) = (culib().cudaGraphNodeGetType) {
            __function(node, pType)
        } else {
            cudaError::cudaErrorNotSupported
        }
    }
    #[cfg(any(
        feature = "cuda-11060",
        feature = "cuda-11070",
        feature = "cuda-11080",
        feature = "cuda-12000",
        feature = "cuda-12010",
        feature = "cuda-12020",
        feature = "cuda-12030",
        feature = "cuda-12040",
        feature = "cuda-12050",
        feature = "cuda-12060",
        feature = "cuda-12080",
        feature = "cuda-12090",
        feature = "cuda-13000"
    ))]
    pub unsafe fn cudaGraphNodeSetEnabled(
        hGraphExec: cudaGraphExec_t,
        hNode: cudaGraphNode_t,
        isEnabled: ::core::ffi::c_uint,
    ) -> cudaError_t {
        if let Some(__function) = (culib().cudaGraphNodeSetEnabled) {
            __function(hGraphExec, hNode, isEnabled)
        } else {
            cudaError::cudaErrorNotSupported
        }
    }
    #[cfg(any(
        feature = "cuda-12020",
        feature = "cuda-12030",
        feature = "cuda-12040",
        feature = "cuda-12050",
        feature = "cuda-12060",
        feature = "cuda-12080",
        feature = "cuda-12090",
        feature = "cuda-13000"
    ))]
    pub unsafe fn cudaGraphNodeSetParams(
        node: cudaGraphNode_t,
        nodeParams: *mut cudaGraphNodeParams,
    ) -> cudaError_t {
        if let Some(__function) = (culib().cudaGraphNodeSetParams) {
            __function(node, nodeParams)
        } else {
            cudaError::cudaErrorNotSupported
        }
    }
    pub unsafe fn cudaGraphReleaseUserObject(
        graph: cudaGraph_t,
        object: cudaUserObject_t,
        count: ::core::ffi::c_uint,
    ) -> cudaError_t {
        if let Some(__function) = (culib().cudaGraphReleaseUserObject) {
            __function(graph, object, count)
        } else {
            cudaError::cudaErrorNotSupported
        }
    }
    #[cfg(any(
        feature = "cuda-11040",
        feature = "cuda-11050",
        feature = "cuda-11060",
        feature = "cuda-11070",
        feature = "cuda-11080",
        feature = "cuda-12000",
        feature = "cuda-12010",
        feature = "cuda-12020",
        feature = "cuda-12030",
        feature = "cuda-12040",
        feature = "cuda-12050",
        feature = "cuda-12060",
        feature = "cuda-12080",
        feature = "cuda-12090"
    ))]
    pub unsafe fn cudaGraphRemoveDependencies(
        graph: cudaGraph_t,
        from: *const cudaGraphNode_t,
        to: *const cudaGraphNode_t,
        numDependencies: usize,
    ) -> cudaError_t {
        if let Some(__function) = (culib().cudaGraphRemoveDependencies) {
            __function(graph, from, to, numDependencies)
        } else {
            cudaError::cudaErrorNotSupported
        }
    }
    #[cfg(any(feature = "cuda-13000"))]
    pub unsafe fn cudaGraphRemoveDependencies(
        graph: cudaGraph_t,
        from: *const cudaGraphNode_t,
        to: *const cudaGraphNode_t,
        edgeData: *const cudaGraphEdgeData,
        numDependencies: usize,
    ) -> cudaError_t {
        if let Some(__function) = (culib().cudaGraphRemoveDependencies) {
            __function(graph, from, to, edgeData, numDependencies)
        } else {
            cudaError::cudaErrorNotSupported
        }
    }
    #[cfg(any(
        feature = "cuda-12030",
        feature = "cuda-12040",
        feature = "cuda-12050",
        feature = "cuda-12060",
        feature = "cuda-12080",
        feature = "cuda-12090"
    ))]
    pub unsafe fn cudaGraphRemoveDependencies_v2(
        graph: cudaGraph_t,
        from: *const cudaGraphNode_t,
        to: *const cudaGraphNode_t,
        edgeData: *const cudaGraphEdgeData,
        numDependencies: usize,
    ) -> cudaError_t {
        if let Some(__function) = (culib().cudaGraphRemoveDependencies_v2) {
            __function(graph, from, to, edgeData, numDependencies)
        } else {
            cudaError::cudaErrorNotSupported
        }
    }
    pub unsafe fn cudaGraphRetainUserObject(
        graph: cudaGraph_t,
        object: cudaUserObject_t,
        count: ::core::ffi::c_uint,
        flags: ::core::ffi::c_uint,
    ) -> cudaError_t {
        if let Some(__function) = (culib().cudaGraphRetainUserObject) {
            __function(graph, object, count, flags)
        } else {
            cudaError::cudaErrorNotSupported
        }
    }
    pub unsafe fn cudaGraphUpload(graphExec: cudaGraphExec_t, stream: cudaStream_t) -> cudaError_t {
        if let Some(__function) = (culib().cudaGraphUpload) {
            __function(graphExec, stream)
        } else {
            cudaError::cudaErrorNotSupported
        }
    }
    pub unsafe fn cudaGraphicsMapResources(
        count: ::core::ffi::c_int,
        resources: *mut cudaGraphicsResource_t,
        stream: cudaStream_t,
    ) -> cudaError_t {
        if let Some(__function) = (culib().cudaGraphicsMapResources) {
            __function(count, resources, stream)
        } else {
            cudaError::cudaErrorNotSupported
        }
    }
    pub unsafe fn cudaGraphicsResourceGetMappedMipmappedArray(
        mipmappedArray: *mut cudaMipmappedArray_t,
        resource: cudaGraphicsResource_t,
    ) -> cudaError_t {
        if let Some(__function) = (culib().cudaGraphicsResourceGetMappedMipmappedArray) {
            __function(mipmappedArray, resource)
        } else {
            cudaError::cudaErrorNotSupported
        }
    }
    pub unsafe fn cudaGraphicsResourceGetMappedPointer(
        devPtr: *mut *mut ::core::ffi::c_void,
        size: *mut usize,
        resource: cudaGraphicsResource_t,
    ) -> cudaError_t {
        if let Some(__function) = (culib().cudaGraphicsResourceGetMappedPointer) {
            __function(devPtr, size, resource)
        } else {
            cudaError::cudaErrorNotSupported
        }
    }
    pub unsafe fn cudaGraphicsResourceSetMapFlags(
        resource: cudaGraphicsResource_t,
        flags: ::core::ffi::c_uint,
    ) -> cudaError_t {
        if let Some(__function) = (culib().cudaGraphicsResourceSetMapFlags) {
            __function(resource, flags)
        } else {
            cudaError::cudaErrorNotSupported
        }
    }
    pub unsafe fn cudaGraphicsSubResourceGetMappedArray(
        array: *mut cudaArray_t,
        resource: cudaGraphicsResource_t,
        arrayIndex: ::core::ffi::c_uint,
        mipLevel: ::core::ffi::c_uint,
    ) -> cudaError_t {
        if let Some(__function) = (culib().cudaGraphicsSubResourceGetMappedArray) {
            __function(array, resource, arrayIndex, mipLevel)
        } else {
            cudaError::cudaErrorNotSupported
        }
    }
    pub unsafe fn cudaGraphicsUnmapResources(
        count: ::core::ffi::c_int,
        resources: *mut cudaGraphicsResource_t,
        stream: cudaStream_t,
    ) -> cudaError_t {
        if let Some(__function) = (culib().cudaGraphicsUnmapResources) {
            __function(count, resources, stream)
        } else {
            cudaError::cudaErrorNotSupported
        }
    }
    pub unsafe fn cudaGraphicsUnregisterResource(resource: cudaGraphicsResource_t) -> cudaError_t {
        if let Some(__function) = (culib().cudaGraphicsUnregisterResource) {
            __function(resource)
        } else {
            cudaError::cudaErrorNotSupported
        }
    }
    pub unsafe fn cudaHostAlloc(
        pHost: *mut *mut ::core::ffi::c_void,
        size: usize,
        flags: ::core::ffi::c_uint,
    ) -> cudaError_t {
        if let Some(__function) = (culib().cudaHostAlloc) {
            __function(pHost, size, flags)
        } else {
            cudaError::cudaErrorNotSupported
        }
    }
    pub unsafe fn cudaHostGetDevicePointer(
        pDevice: *mut *mut ::core::ffi::c_void,
        pHost: *mut ::core::ffi::c_void,
        flags: ::core::ffi::c_uint,
    ) -> cudaError_t {
        if let Some(__function) = (culib().cudaHostGetDevicePointer) {
            __function(pDevice, pHost, flags)
        } else {
            cudaError::cudaErrorNotSupported
        }
    }
    pub unsafe fn cudaHostGetFlags(
        pFlags: *mut ::core::ffi::c_uint,
        pHost: *mut ::core::ffi::c_void,
    ) -> cudaError_t {
        if let Some(__function) = (culib().cudaHostGetFlags) {
            __function(pFlags, pHost)
        } else {
            cudaError::cudaErrorNotSupported
        }
    }
    pub unsafe fn cudaHostRegister(
        ptr: *mut ::core::ffi::c_void,
        size: usize,
        flags: ::core::ffi::c_uint,
    ) -> cudaError_t {
        if let Some(__function) = (culib().cudaHostRegister) {
            __function(ptr, size, flags)
        } else {
            cudaError::cudaErrorNotSupported
        }
    }
    pub unsafe fn cudaHostUnregister(ptr: *mut ::core::ffi::c_void) -> cudaError_t {
        if let Some(__function) = (culib().cudaHostUnregister) {
            __function(ptr)
        } else {
            cudaError::cudaErrorNotSupported
        }
    }
    pub unsafe fn cudaImportExternalMemory(
        extMem_out: *mut cudaExternalMemory_t,
        memHandleDesc: *const cudaExternalMemoryHandleDesc,
    ) -> cudaError_t {
        if let Some(__function) = (culib().cudaImportExternalMemory) {
            __function(extMem_out, memHandleDesc)
        } else {
            cudaError::cudaErrorNotSupported
        }
    }
    pub unsafe fn cudaImportExternalSemaphore(
        extSem_out: *mut cudaExternalSemaphore_t,
        semHandleDesc: *const cudaExternalSemaphoreHandleDesc,
    ) -> cudaError_t {
        if let Some(__function) = (culib().cudaImportExternalSemaphore) {
            __function(extSem_out, semHandleDesc)
        } else {
            cudaError::cudaErrorNotSupported
        }
    }
    #[cfg(any(
        feature = "cuda-12000",
        feature = "cuda-12010",
        feature = "cuda-12020",
        feature = "cuda-12030",
        feature = "cuda-12040",
        feature = "cuda-12050",
        feature = "cuda-12060",
        feature = "cuda-12080",
        feature = "cuda-12090",
        feature = "cuda-13000"
    ))]
    pub unsafe fn cudaInitDevice(
        device: ::core::ffi::c_int,
        deviceFlags: ::core::ffi::c_uint,
        flags: ::core::ffi::c_uint,
    ) -> cudaError_t {
        if let Some(__function) = (culib().cudaInitDevice) {
            __function(device, deviceFlags, flags)
        } else {
            cudaError::cudaErrorNotSupported
        }
    }
    pub unsafe fn cudaIpcCloseMemHandle(devPtr: *mut ::core::ffi::c_void) -> cudaError_t {
        if let Some(__function) = (culib().cudaIpcCloseMemHandle) {
            __function(devPtr)
        } else {
            cudaError::cudaErrorNotSupported
        }
    }
    pub unsafe fn cudaIpcGetEventHandle(
        handle: *mut cudaIpcEventHandle_t,
        event: cudaEvent_t,
    ) -> cudaError_t {
        if let Some(__function) = (culib().cudaIpcGetEventHandle) {
            __function(handle, event)
        } else {
            cudaError::cudaErrorNotSupported
        }
    }
    pub unsafe fn cudaIpcGetMemHandle(
        handle: *mut cudaIpcMemHandle_t,
        devPtr: *mut ::core::ffi::c_void,
    ) -> cudaError_t {
        if let Some(__function) = (culib().cudaIpcGetMemHandle) {
            __function(handle, devPtr)
        } else {
            cudaError::cudaErrorNotSupported
        }
    }
    pub unsafe fn cudaIpcOpenEventHandle(
        event: *mut cudaEvent_t,
        handle: cudaIpcEventHandle_t,
    ) -> cudaError_t {
        if let Some(__function) = (culib().cudaIpcOpenEventHandle) {
            __function(event, handle)
        } else {
            cudaError::cudaErrorNotSupported
        }
    }
    pub unsafe fn cudaIpcOpenMemHandle(
        devPtr: *mut *mut ::core::ffi::c_void,
        handle: cudaIpcMemHandle_t,
        flags: ::core::ffi::c_uint,
    ) -> cudaError_t {
        if let Some(__function) = (culib().cudaIpcOpenMemHandle) {
            __function(devPtr, handle, flags)
        } else {
            cudaError::cudaErrorNotSupported
        }
    }
    #[cfg(any(feature = "cuda-12080", feature = "cuda-12090", feature = "cuda-13000"))]
    pub unsafe fn cudaKernelSetAttributeForDevice(
        kernel: cudaKernel_t,
        attr: cudaFuncAttribute,
        value: ::core::ffi::c_int,
        device: ::core::ffi::c_int,
    ) -> cudaError_t {
        if let Some(__function) = (culib().cudaKernelSetAttributeForDevice) {
            __function(kernel, attr, value, device)
        } else {
            cudaError::cudaErrorNotSupported
        }
    }
    pub unsafe fn cudaLaunchCooperativeKernel(
        func: *const ::core::ffi::c_void,
        gridDim: dim3,
        blockDim: dim3,
        args: *mut *mut ::core::ffi::c_void,
        sharedMem: usize,
        stream: cudaStream_t,
    ) -> cudaError_t {
        if let Some(__function) = (culib().cudaLaunchCooperativeKernel) {
            __function(func, gridDim, blockDim, args, sharedMem, stream)
        } else {
            cudaError::cudaErrorNotSupported
        }
    }
    #[cfg(any(
        feature = "cuda-11040",
        feature = "cuda-11050",
        feature = "cuda-11060",
        feature = "cuda-11070",
        feature = "cuda-11080",
        feature = "cuda-12000",
        feature = "cuda-12010",
        feature = "cuda-12020",
        feature = "cuda-12030",
        feature = "cuda-12040",
        feature = "cuda-12050",
        feature = "cuda-12060",
        feature = "cuda-12080",
        feature = "cuda-12090"
    ))]
    pub unsafe fn cudaLaunchCooperativeKernelMultiDevice(
        launchParamsList: *mut cudaLaunchParams,
        numDevices: ::core::ffi::c_uint,
        flags: ::core::ffi::c_uint,
    ) -> cudaError_t {
        if let Some(__function) = (culib().cudaLaunchCooperativeKernelMultiDevice) {
            __function(launchParamsList, numDevices, flags)
        } else {
            cudaError::cudaErrorNotSupported
        }
    }
    pub unsafe fn cudaLaunchHostFunc(
        stream: cudaStream_t,
        fn_: cudaHostFn_t,
        userData: *mut ::core::ffi::c_void,
    ) -> cudaError_t {
        if let Some(__function) = (culib().cudaLaunchHostFunc) {
            __function(stream, fn_, userData)
        } else {
            cudaError::cudaErrorNotSupported
        }
    }
    pub unsafe fn cudaLaunchKernel(
        func: *const ::core::ffi::c_void,
        gridDim: dim3,
        blockDim: dim3,
        args: *mut *mut ::core::ffi::c_void,
        sharedMem: usize,
        stream: cudaStream_t,
    ) -> cudaError_t {
        if let Some(__function) = (culib().cudaLaunchKernel) {
            __function(func, gridDim, blockDim, args, sharedMem, stream)
        } else {
            cudaError::cudaErrorNotSupported
        }
    }
    #[cfg(any(
        feature = "cuda-11080",
        feature = "cuda-12000",
        feature = "cuda-12010",
        feature = "cuda-12020",
        feature = "cuda-12030",
        feature = "cuda-12040",
        feature = "cuda-12050",
        feature = "cuda-12060",
        feature = "cuda-12080",
        feature = "cuda-12090",
        feature = "cuda-13000"
    ))]
    pub unsafe fn cudaLaunchKernelExC(
        config: *const cudaLaunchConfig_t,
        func: *const ::core::ffi::c_void,
        args: *mut *mut ::core::ffi::c_void,
    ) -> cudaError_t {
        if let Some(__function) = (culib().cudaLaunchKernelExC) {
            __function(config, func, args)
        } else {
            cudaError::cudaErrorNotSupported
        }
    }
    #[cfg(any(feature = "cuda-12080", feature = "cuda-12090", feature = "cuda-13000"))]
    pub unsafe fn cudaLibraryEnumerateKernels(
        kernels: *mut cudaKernel_t,
        numKernels: ::core::ffi::c_uint,
        lib: cudaLibrary_t,
    ) -> cudaError_t {
        if let Some(__function) = (culib().cudaLibraryEnumerateKernels) {
            __function(kernels, numKernels, lib)
        } else {
            cudaError::cudaErrorNotSupported
        }
    }
    #[cfg(any(feature = "cuda-12080", feature = "cuda-12090", feature = "cuda-13000"))]
    pub unsafe fn cudaLibraryGetGlobal(
        dptr: *mut *mut ::core::ffi::c_void,
        bytes: *mut usize,
        library: cudaLibrary_t,
        name: *const ::core::ffi::c_char,
    ) -> cudaError_t {
        if let Some(__function) = (culib().cudaLibraryGetGlobal) {
            __function(dptr, bytes, library, name)
        } else {
            cudaError::cudaErrorNotSupported
        }
    }
    #[cfg(any(feature = "cuda-12080", feature = "cuda-12090", feature = "cuda-13000"))]
    pub unsafe fn cudaLibraryGetKernel(
        pKernel: *mut cudaKernel_t,
        library: cudaLibrary_t,
        name: *const ::core::ffi::c_char,
    ) -> cudaError_t {
        if let Some(__function) = (culib().cudaLibraryGetKernel) {
            __function(pKernel, library, name)
        } else {
            cudaError::cudaErrorNotSupported
        }
    }
    #[cfg(any(feature = "cuda-12080", feature = "cuda-12090", feature = "cuda-13000"))]
    pub unsafe fn cudaLibraryGetKernelCount(
        count: *mut ::core::ffi::c_uint,
        lib: cudaLibrary_t,
    ) -> cudaError_t {
        if let Some(__function) = (culib().cudaLibraryGetKernelCount) {
            __function(count, lib)
        } else {
            cudaError::cudaErrorNotSupported
        }
    }
    #[cfg(any(feature = "cuda-12080", feature = "cuda-12090", feature = "cuda-13000"))]
    pub unsafe fn cudaLibraryGetManaged(
        dptr: *mut *mut ::core::ffi::c_void,
        bytes: *mut usize,
        library: cudaLibrary_t,
        name: *const ::core::ffi::c_char,
    ) -> cudaError_t {
        if let Some(__function) = (culib().cudaLibraryGetManaged) {
            __function(dptr, bytes, library, name)
        } else {
            cudaError::cudaErrorNotSupported
        }
    }
    #[cfg(any(feature = "cuda-12080", feature = "cuda-12090", feature = "cuda-13000"))]
    pub unsafe fn cudaLibraryGetUnifiedFunction(
        fptr: *mut *mut ::core::ffi::c_void,
        library: cudaLibrary_t,
        symbol: *const ::core::ffi::c_char,
    ) -> cudaError_t {
        if let Some(__function) = (culib().cudaLibraryGetUnifiedFunction) {
            __function(fptr, library, symbol)
        } else {
            cudaError::cudaErrorNotSupported
        }
    }
    #[cfg(any(feature = "cuda-12080", feature = "cuda-12090", feature = "cuda-13000"))]
    pub unsafe fn cudaLibraryLoadData(
        library: *mut cudaLibrary_t,
        code: *const ::core::ffi::c_void,
        jitOptions: *mut cudaJitOption,
        jitOptionsValues: *mut *mut ::core::ffi::c_void,
        numJitOptions: ::core::ffi::c_uint,
        libraryOptions: *mut cudaLibraryOption,
        libraryOptionValues: *mut *mut ::core::ffi::c_void,
        numLibraryOptions: ::core::ffi::c_uint,
    ) -> cudaError_t {
        if let Some(__function) = (culib().cudaLibraryLoadData) {
            __function(
                library,
                code,
                jitOptions,
                jitOptionsValues,
                numJitOptions,
                libraryOptions,
                libraryOptionValues,
                numLibraryOptions,
            )
        } else {
            cudaError::cudaErrorNotSupported
        }
    }
    #[cfg(any(feature = "cuda-12080", feature = "cuda-12090", feature = "cuda-13000"))]
    pub unsafe fn cudaLibraryLoadFromFile(
        library: *mut cudaLibrary_t,
        fileName: *const ::core::ffi::c_char,
        jitOptions: *mut cudaJitOption,
        jitOptionsValues: *mut *mut ::core::ffi::c_void,
        numJitOptions: ::core::ffi::c_uint,
        libraryOptions: *mut cudaLibraryOption,
        libraryOptionValues: *mut *mut ::core::ffi::c_void,
        numLibraryOptions: ::core::ffi::c_uint,
    ) -> cudaError_t {
        if let Some(__function) = (culib().cudaLibraryLoadFromFile) {
            __function(
                library,
                fileName,
                jitOptions,
                jitOptionsValues,
                numJitOptions,
                libraryOptions,
                libraryOptionValues,
                numLibraryOptions,
            )
        } else {
            cudaError::cudaErrorNotSupported
        }
    }
    #[cfg(any(feature = "cuda-12080", feature = "cuda-12090", feature = "cuda-13000"))]
    pub unsafe fn cudaLibraryUnload(library: cudaLibrary_t) -> cudaError_t {
        if let Some(__function) = (culib().cudaLibraryUnload) {
            __function(library)
        } else {
            cudaError::cudaErrorNotSupported
        }
    }
    #[cfg(any(feature = "cuda-13000"))]
    pub unsafe fn cudaLogsCurrent(
        iterator_out: *mut cudaLogIterator,
        flags: ::core::ffi::c_uint,
    ) -> cudaError_t {
        if let Some(__function) = (culib().cudaLogsCurrent) {
            __function(iterator_out, flags)
        } else {
            cudaError::cudaErrorNotSupported
        }
    }
    #[cfg(any(feature = "cuda-13000"))]
    pub unsafe fn cudaLogsDumpToFile(
        iterator: *mut cudaLogIterator,
        pathToFile: *const ::core::ffi::c_char,
        flags: ::core::ffi::c_uint,
    ) -> cudaError_t {
        if let Some(__function) = (culib().cudaLogsDumpToFile) {
            __function(iterator, pathToFile, flags)
        } else {
            cudaError::cudaErrorNotSupported
        }
    }
    #[cfg(any(feature = "cuda-13000"))]
    pub unsafe fn cudaLogsDumpToMemory(
        iterator: *mut cudaLogIterator,
        buffer: *mut ::core::ffi::c_char,
        size: *mut usize,
        flags: ::core::ffi::c_uint,
    ) -> cudaError_t {
        if let Some(__function) = (culib().cudaLogsDumpToMemory) {
            __function(iterator, buffer, size, flags)
        } else {
            cudaError::cudaErrorNotSupported
        }
    }
    #[cfg(any(feature = "cuda-13000"))]
    pub unsafe fn cudaLogsRegisterCallback(
        callbackFunc: cudaLogsCallback_t,
        userData: *mut ::core::ffi::c_void,
        callback_out: *mut cudaLogsCallbackHandle,
    ) -> cudaError_t {
        if let Some(__function) = (culib().cudaLogsRegisterCallback) {
            __function(callbackFunc, userData, callback_out)
        } else {
            cudaError::cudaErrorNotSupported
        }
    }
    #[cfg(any(feature = "cuda-13000"))]
    pub unsafe fn cudaLogsUnregisterCallback(callback: cudaLogsCallbackHandle) -> cudaError_t {
        if let Some(__function) = (culib().cudaLogsUnregisterCallback) {
            __function(callback)
        } else {
            cudaError::cudaErrorNotSupported
        }
    }
    pub unsafe fn cudaMalloc(devPtr: *mut *mut ::core::ffi::c_void, size: usize) -> cudaError_t {
        if let Some(__function) = (culib().cudaMalloc) {
            __function(devPtr, size)
        } else {
            cudaError::cudaErrorNotSupported
        }
    }
    pub unsafe fn cudaMalloc3D(
        pitchedDevPtr: *mut cudaPitchedPtr,
        extent: cudaExtent,
    ) -> cudaError_t {
        if let Some(__function) = (culib().cudaMalloc3D) {
            __function(pitchedDevPtr, extent)
        } else {
            cudaError::cudaErrorNotSupported
        }
    }
    pub unsafe fn cudaMalloc3DArray(
        array: *mut cudaArray_t,
        desc: *const cudaChannelFormatDesc,
        extent: cudaExtent,
        flags: ::core::ffi::c_uint,
    ) -> cudaError_t {
        if let Some(__function) = (culib().cudaMalloc3DArray) {
            __function(array, desc, extent, flags)
        } else {
            cudaError::cudaErrorNotSupported
        }
    }
    pub unsafe fn cudaMallocArray(
        array: *mut cudaArray_t,
        desc: *const cudaChannelFormatDesc,
        width: usize,
        height: usize,
        flags: ::core::ffi::c_uint,
    ) -> cudaError_t {
        if let Some(__function) = (culib().cudaMallocArray) {
            __function(array, desc, width, height, flags)
        } else {
            cudaError::cudaErrorNotSupported
        }
    }
    pub unsafe fn cudaMallocAsync(
        devPtr: *mut *mut ::core::ffi::c_void,
        size: usize,
        hStream: cudaStream_t,
    ) -> cudaError_t {
        if let Some(__function) = (culib().cudaMallocAsync) {
            __function(devPtr, size, hStream)
        } else {
            cudaError::cudaErrorNotSupported
        }
    }
    pub unsafe fn cudaMallocFromPoolAsync(
        ptr: *mut *mut ::core::ffi::c_void,
        size: usize,
        memPool: cudaMemPool_t,
        stream: cudaStream_t,
    ) -> cudaError_t {
        if let Some(__function) = (culib().cudaMallocFromPoolAsync) {
            __function(ptr, size, memPool, stream)
        } else {
            cudaError::cudaErrorNotSupported
        }
    }
    pub unsafe fn cudaMallocHost(ptr: *mut *mut ::core::ffi::c_void, size: usize) -> cudaError_t {
        if let Some(__function) = (culib().cudaMallocHost) {
            __function(ptr, size)
        } else {
            cudaError::cudaErrorNotSupported
        }
    }
    pub unsafe fn cudaMallocManaged(
        devPtr: *mut *mut ::core::ffi::c_void,
        size: usize,
        flags: ::core::ffi::c_uint,
    ) -> cudaError_t {
        if let Some(__function) = (culib().cudaMallocManaged) {
            __function(devPtr, size, flags)
        } else {
            cudaError::cudaErrorNotSupported
        }
    }
    pub unsafe fn cudaMallocMipmappedArray(
        mipmappedArray: *mut cudaMipmappedArray_t,
        desc: *const cudaChannelFormatDesc,
        extent: cudaExtent,
        numLevels: ::core::ffi::c_uint,
        flags: ::core::ffi::c_uint,
    ) -> cudaError_t {
        if let Some(__function) = (culib().cudaMallocMipmappedArray) {
            __function(mipmappedArray, desc, extent, numLevels, flags)
        } else {
            cudaError::cudaErrorNotSupported
        }
    }
    pub unsafe fn cudaMallocPitch(
        devPtr: *mut *mut ::core::ffi::c_void,
        pitch: *mut usize,
        width: usize,
        height: usize,
    ) -> cudaError_t {
        if let Some(__function) = (culib().cudaMallocPitch) {
            __function(devPtr, pitch, width, height)
        } else {
            cudaError::cudaErrorNotSupported
        }
    }
    #[cfg(any(
        feature = "cuda-11040",
        feature = "cuda-11050",
        feature = "cuda-11060",
        feature = "cuda-11070",
        feature = "cuda-11080",
        feature = "cuda-12000",
        feature = "cuda-12010",
        feature = "cuda-12020",
        feature = "cuda-12030",
        feature = "cuda-12040",
        feature = "cuda-12050",
        feature = "cuda-12060",
        feature = "cuda-12080",
        feature = "cuda-12090"
    ))]
    pub unsafe fn cudaMemAdvise(
        devPtr: *const ::core::ffi::c_void,
        count: usize,
        advice: cudaMemoryAdvise,
        device: ::core::ffi::c_int,
    ) -> cudaError_t {
        if let Some(__function) = (culib().cudaMemAdvise) {
            __function(devPtr, count, advice, device)
        } else {
            cudaError::cudaErrorNotSupported
        }
    }
    #[cfg(any(feature = "cuda-13000"))]
    pub unsafe fn cudaMemAdvise(
        devPtr: *const ::core::ffi::c_void,
        count: usize,
        advice: cudaMemoryAdvise,
        location: cudaMemLocation,
    ) -> cudaError_t {
        if let Some(__function) = (culib().cudaMemAdvise) {
            __function(devPtr, count, advice, location)
        } else {
            cudaError::cudaErrorNotSupported
        }
    }
    #[cfg(any(
        feature = "cuda-12020",
        feature = "cuda-12030",
        feature = "cuda-12040",
        feature = "cuda-12050",
        feature = "cuda-12060",
        feature = "cuda-12080",
        feature = "cuda-12090"
    ))]
    pub unsafe fn cudaMemAdvise_v2(
        devPtr: *const ::core::ffi::c_void,
        count: usize,
        advice: cudaMemoryAdvise,
        location: cudaMemLocation,
    ) -> cudaError_t {
        if let Some(__function) = (culib().cudaMemAdvise_v2) {
            __function(devPtr, count, advice, location)
        } else {
            cudaError::cudaErrorNotSupported
        }
    }
    #[cfg(any(feature = "cuda-13000"))]
    pub unsafe fn cudaMemDiscardAndPrefetchBatchAsync(
        dptrs: *mut *mut ::core::ffi::c_void,
        sizes: *mut usize,
        count: usize,
        prefetchLocs: *mut cudaMemLocation,
        prefetchLocIdxs: *mut usize,
        numPrefetchLocs: usize,
        flags: ::core::ffi::c_ulonglong,
        stream: cudaStream_t,
    ) -> cudaError_t {
        if let Some(__function) = (culib().cudaMemDiscardAndPrefetchBatchAsync) {
            __function(
                dptrs,
                sizes,
                count,
                prefetchLocs,
                prefetchLocIdxs,
                numPrefetchLocs,
                flags,
                stream,
            )
        } else {
            cudaError::cudaErrorNotSupported
        }
    }
    #[cfg(any(feature = "cuda-13000"))]
    pub unsafe fn cudaMemDiscardBatchAsync(
        dptrs: *mut *mut ::core::ffi::c_void,
        sizes: *mut usize,
        count: usize,
        flags: ::core::ffi::c_ulonglong,
        stream: cudaStream_t,
    ) -> cudaError_t {
        if let Some(__function) = (culib().cudaMemDiscardBatchAsync) {
            __function(dptrs, sizes, count, flags, stream)
        } else {
            cudaError::cudaErrorNotSupported
        }
    }
    #[cfg(any(feature = "cuda-13000"))]
    pub unsafe fn cudaMemGetDefaultMemPool(
        memPool: *mut cudaMemPool_t,
        location: *mut cudaMemLocation,
        type_: cudaMemAllocationType,
    ) -> cudaError_t {
        if let Some(__function) = (culib().cudaMemGetDefaultMemPool) {
            __function(memPool, location, type_)
        } else {
            cudaError::cudaErrorNotSupported
        }
    }
    pub unsafe fn cudaMemGetInfo(free: *mut usize, total: *mut usize) -> cudaError_t {
        if let Some(__function) = (culib().cudaMemGetInfo) {
            __function(free, total)
        } else {
            cudaError::cudaErrorNotSupported
        }
    }
    #[cfg(any(feature = "cuda-13000"))]
    pub unsafe fn cudaMemGetMemPool(
        memPool: *mut cudaMemPool_t,
        location: *mut cudaMemLocation,
        type_: cudaMemAllocationType,
    ) -> cudaError_t {
        if let Some(__function) = (culib().cudaMemGetMemPool) {
            __function(memPool, location, type_)
        } else {
            cudaError::cudaErrorNotSupported
        }
    }
    pub unsafe fn cudaMemPoolCreate(
        memPool: *mut cudaMemPool_t,
        poolProps: *const cudaMemPoolProps,
    ) -> cudaError_t {
        if let Some(__function) = (culib().cudaMemPoolCreate) {
            __function(memPool, poolProps)
        } else {
            cudaError::cudaErrorNotSupported
        }
    }
    pub unsafe fn cudaMemPoolDestroy(memPool: cudaMemPool_t) -> cudaError_t {
        if let Some(__function) = (culib().cudaMemPoolDestroy) {
            __function(memPool)
        } else {
            cudaError::cudaErrorNotSupported
        }
    }
    pub unsafe fn cudaMemPoolExportPointer(
        exportData: *mut cudaMemPoolPtrExportData,
        ptr: *mut ::core::ffi::c_void,
    ) -> cudaError_t {
        if let Some(__function) = (culib().cudaMemPoolExportPointer) {
            __function(exportData, ptr)
        } else {
            cudaError::cudaErrorNotSupported
        }
    }
    pub unsafe fn cudaMemPoolExportToShareableHandle(
        shareableHandle: *mut ::core::ffi::c_void,
        memPool: cudaMemPool_t,
        handleType: cudaMemAllocationHandleType,
        flags: ::core::ffi::c_uint,
    ) -> cudaError_t {
        if let Some(__function) = (culib().cudaMemPoolExportToShareableHandle) {
            __function(shareableHandle, memPool, handleType, flags)
        } else {
            cudaError::cudaErrorNotSupported
        }
    }
    pub unsafe fn cudaMemPoolGetAccess(
        flags: *mut cudaMemAccessFlags,
        memPool: cudaMemPool_t,
        location: *mut cudaMemLocation,
    ) -> cudaError_t {
        if let Some(__function) = (culib().cudaMemPoolGetAccess) {
            __function(flags, memPool, location)
        } else {
            cudaError::cudaErrorNotSupported
        }
    }
    pub unsafe fn cudaMemPoolGetAttribute(
        memPool: cudaMemPool_t,
        attr: cudaMemPoolAttr,
        value: *mut ::core::ffi::c_void,
    ) -> cudaError_t {
        if let Some(__function) = (culib().cudaMemPoolGetAttribute) {
            __function(memPool, attr, value)
        } else {
            cudaError::cudaErrorNotSupported
        }
    }
    pub unsafe fn cudaMemPoolImportFromShareableHandle(
        memPool: *mut cudaMemPool_t,
        shareableHandle: *mut ::core::ffi::c_void,
        handleType: cudaMemAllocationHandleType,
        flags: ::core::ffi::c_uint,
    ) -> cudaError_t {
        if let Some(__function) = (culib().cudaMemPoolImportFromShareableHandle) {
            __function(memPool, shareableHandle, handleType, flags)
        } else {
            cudaError::cudaErrorNotSupported
        }
    }
    pub unsafe fn cudaMemPoolImportPointer(
        ptr: *mut *mut ::core::ffi::c_void,
        memPool: cudaMemPool_t,
        exportData: *mut cudaMemPoolPtrExportData,
    ) -> cudaError_t {
        if let Some(__function) = (culib().cudaMemPoolImportPointer) {
            __function(ptr, memPool, exportData)
        } else {
            cudaError::cudaErrorNotSupported
        }
    }
    pub unsafe fn cudaMemPoolSetAccess(
        memPool: cudaMemPool_t,
        descList: *const cudaMemAccessDesc,
        count: usize,
    ) -> cudaError_t {
        if let Some(__function) = (culib().cudaMemPoolSetAccess) {
            __function(memPool, descList, count)
        } else {
            cudaError::cudaErrorNotSupported
        }
    }
    pub unsafe fn cudaMemPoolSetAttribute(
        memPool: cudaMemPool_t,
        attr: cudaMemPoolAttr,
        value: *mut ::core::ffi::c_void,
    ) -> cudaError_t {
        if let Some(__function) = (culib().cudaMemPoolSetAttribute) {
            __function(memPool, attr, value)
        } else {
            cudaError::cudaErrorNotSupported
        }
    }
    pub unsafe fn cudaMemPoolTrimTo(memPool: cudaMemPool_t, minBytesToKeep: usize) -> cudaError_t {
        if let Some(__function) = (culib().cudaMemPoolTrimTo) {
            __function(memPool, minBytesToKeep)
        } else {
            cudaError::cudaErrorNotSupported
        }
    }
    #[cfg(any(
        feature = "cuda-11040",
        feature = "cuda-11050",
        feature = "cuda-11060",
        feature = "cuda-11070",
        feature = "cuda-11080",
        feature = "cuda-12000",
        feature = "cuda-12010",
        feature = "cuda-12020",
        feature = "cuda-12030",
        feature = "cuda-12040",
        feature = "cuda-12050",
        feature = "cuda-12060",
        feature = "cuda-12080",
        feature = "cuda-12090"
    ))]
    pub unsafe fn cudaMemPrefetchAsync(
        devPtr: *const ::core::ffi::c_void,
        count: usize,
        dstDevice: ::core::ffi::c_int,
        stream: cudaStream_t,
    ) -> cudaError_t {
        if let Some(__function) = (culib().cudaMemPrefetchAsync) {
            __function(devPtr, count, dstDevice, stream)
        } else {
            cudaError::cudaErrorNotSupported
        }
    }
    #[cfg(any(feature = "cuda-13000"))]
    pub unsafe fn cudaMemPrefetchAsync(
        devPtr: *const ::core::ffi::c_void,
        count: usize,
        location: cudaMemLocation,
        flags: ::core::ffi::c_uint,
        stream: cudaStream_t,
    ) -> cudaError_t {
        if let Some(__function) = (culib().cudaMemPrefetchAsync) {
            __function(devPtr, count, location, flags, stream)
        } else {
            cudaError::cudaErrorNotSupported
        }
    }
    #[cfg(any(
        feature = "cuda-12020",
        feature = "cuda-12030",
        feature = "cuda-12040",
        feature = "cuda-12050",
        feature = "cuda-12060",
        feature = "cuda-12080",
        feature = "cuda-12090"
    ))]
    pub unsafe fn cudaMemPrefetchAsync_v2(
        devPtr: *const ::core::ffi::c_void,
        count: usize,
        location: cudaMemLocation,
        flags: ::core::ffi::c_uint,
        stream: cudaStream_t,
    ) -> cudaError_t {
        if let Some(__function) = (culib().cudaMemPrefetchAsync_v2) {
            __function(devPtr, count, location, flags, stream)
        } else {
            cudaError::cudaErrorNotSupported
        }
    }
    #[cfg(any(feature = "cuda-13000"))]
    pub unsafe fn cudaMemPrefetchBatchAsync(
        dptrs: *mut *mut ::core::ffi::c_void,
        sizes: *mut usize,
        count: usize,
        prefetchLocs: *mut cudaMemLocation,
        prefetchLocIdxs: *mut usize,
        numPrefetchLocs: usize,
        flags: ::core::ffi::c_ulonglong,
        stream: cudaStream_t,
    ) -> cudaError_t {
        if let Some(__function) = (culib().cudaMemPrefetchBatchAsync) {
            __function(
                dptrs,
                sizes,
                count,
                prefetchLocs,
                prefetchLocIdxs,
                numPrefetchLocs,
                flags,
                stream,
            )
        } else {
            cudaError::cudaErrorNotSupported
        }
    }
    pub unsafe fn cudaMemRangeGetAttribute(
        data: *mut ::core::ffi::c_void,
        dataSize: usize,
        attribute: cudaMemRangeAttribute,
        devPtr: *const ::core::ffi::c_void,
        count: usize,
    ) -> cudaError_t {
        if let Some(__function) = (culib().cudaMemRangeGetAttribute) {
            __function(data, dataSize, attribute, devPtr, count)
        } else {
            cudaError::cudaErrorNotSupported
        }
    }
    pub unsafe fn cudaMemRangeGetAttributes(
        data: *mut *mut ::core::ffi::c_void,
        dataSizes: *mut usize,
        attributes: *mut cudaMemRangeAttribute,
        numAttributes: usize,
        devPtr: *const ::core::ffi::c_void,
        count: usize,
    ) -> cudaError_t {
        if let Some(__function) = (culib().cudaMemRangeGetAttributes) {
            __function(data, dataSizes, attributes, numAttributes, devPtr, count)
        } else {
            cudaError::cudaErrorNotSupported
        }
    }
    #[cfg(any(feature = "cuda-13000"))]
    pub unsafe fn cudaMemSetMemPool(
        location: *mut cudaMemLocation,
        type_: cudaMemAllocationType,
        memPool: cudaMemPool_t,
    ) -> cudaError_t {
        if let Some(__function) = (culib().cudaMemSetMemPool) {
            __function(location, type_, memPool)
        } else {
            cudaError::cudaErrorNotSupported
        }
    }
    pub unsafe fn cudaMemcpy(
        dst: *mut ::core::ffi::c_void,
        src: *const ::core::ffi::c_void,
        count: usize,
        kind: cudaMemcpyKind,
    ) -> cudaError_t {
        if let Some(__function) = (culib().cudaMemcpy) {
            __function(dst, src, count, kind)
        } else {
            cudaError::cudaErrorNotSupported
        }
    }
    pub unsafe fn cudaMemcpy2D(
        dst: *mut ::core::ffi::c_void,
        dpitch: usize,
        src: *const ::core::ffi::c_void,
        spitch: usize,
        width: usize,
        height: usize,
        kind: cudaMemcpyKind,
    ) -> cudaError_t {
        if let Some(__function) = (culib().cudaMemcpy2D) {
            __function(dst, dpitch, src, spitch, width, height, kind)
        } else {
            cudaError::cudaErrorNotSupported
        }
    }
    pub unsafe fn cudaMemcpy2DArrayToArray(
        dst: cudaArray_t,
        wOffsetDst: usize,
        hOffsetDst: usize,
        src: cudaArray_const_t,
        wOffsetSrc: usize,
        hOffsetSrc: usize,
        width: usize,
        height: usize,
        kind: cudaMemcpyKind,
    ) -> cudaError_t {
        if let Some(__function) = (culib().cudaMemcpy2DArrayToArray) {
            __function(
                dst, wOffsetDst, hOffsetDst, src, wOffsetSrc, hOffsetSrc, width, height, kind,
            )
        } else {
            cudaError::cudaErrorNotSupported
        }
    }
    pub unsafe fn cudaMemcpy2DAsync(
        dst: *mut ::core::ffi::c_void,
        dpitch: usize,
        src: *const ::core::ffi::c_void,
        spitch: usize,
        width: usize,
        height: usize,
        kind: cudaMemcpyKind,
        stream: cudaStream_t,
    ) -> cudaError_t {
        if let Some(__function) = (culib().cudaMemcpy2DAsync) {
            __function(dst, dpitch, src, spitch, width, height, kind, stream)
        } else {
            cudaError::cudaErrorNotSupported
        }
    }
    pub unsafe fn cudaMemcpy2DFromArray(
        dst: *mut ::core::ffi::c_void,
        dpitch: usize,
        src: cudaArray_const_t,
        wOffset: usize,
        hOffset: usize,
        width: usize,
        height: usize,
        kind: cudaMemcpyKind,
    ) -> cudaError_t {
        if let Some(__function) = (culib().cudaMemcpy2DFromArray) {
            __function(dst, dpitch, src, wOffset, hOffset, width, height, kind)
        } else {
            cudaError::cudaErrorNotSupported
        }
    }
    pub unsafe fn cudaMemcpy2DFromArrayAsync(
        dst: *mut ::core::ffi::c_void,
        dpitch: usize,
        src: cudaArray_const_t,
        wOffset: usize,
        hOffset: usize,
        width: usize,
        height: usize,
        kind: cudaMemcpyKind,
        stream: cudaStream_t,
    ) -> cudaError_t {
        if let Some(__function) = (culib().cudaMemcpy2DFromArrayAsync) {
            __function(
                dst, dpitch, src, wOffset, hOffset, width, height, kind, stream,
            )
        } else {
            cudaError::cudaErrorNotSupported
        }
    }
    pub unsafe fn cudaMemcpy2DToArray(
        dst: cudaArray_t,
        wOffset: usize,
        hOffset: usize,
        src: *const ::core::ffi::c_void,
        spitch: usize,
        width: usize,
        height: usize,
        kind: cudaMemcpyKind,
    ) -> cudaError_t {
        if let Some(__function) = (culib().cudaMemcpy2DToArray) {
            __function(dst, wOffset, hOffset, src, spitch, width, height, kind)
        } else {
            cudaError::cudaErrorNotSupported
        }
    }
    pub unsafe fn cudaMemcpy2DToArrayAsync(
        dst: cudaArray_t,
        wOffset: usize,
        hOffset: usize,
        src: *const ::core::ffi::c_void,
        spitch: usize,
        width: usize,
        height: usize,
        kind: cudaMemcpyKind,
        stream: cudaStream_t,
    ) -> cudaError_t {
        if let Some(__function) = (culib().cudaMemcpy2DToArrayAsync) {
            __function(
                dst, wOffset, hOffset, src, spitch, width, height, kind, stream,
            )
        } else {
            cudaError::cudaErrorNotSupported
        }
    }
    pub unsafe fn cudaMemcpy3D(p: *const cudaMemcpy3DParms) -> cudaError_t {
        if let Some(__function) = (culib().cudaMemcpy3D) {
            __function(p)
        } else {
            cudaError::cudaErrorNotSupported
        }
    }
    pub unsafe fn cudaMemcpy3DAsync(
        p: *const cudaMemcpy3DParms,
        stream: cudaStream_t,
    ) -> cudaError_t {
        if let Some(__function) = (culib().cudaMemcpy3DAsync) {
            __function(p, stream)
        } else {
            cudaError::cudaErrorNotSupported
        }
    }
    #[cfg(any(feature = "cuda-12080", feature = "cuda-12090"))]
    pub unsafe fn cudaMemcpy3DBatchAsync(
        numOps: usize,
        opList: *mut cudaMemcpy3DBatchOp,
        failIdx: *mut usize,
        flags: ::core::ffi::c_ulonglong,
        stream: cudaStream_t,
    ) -> cudaError_t {
        if let Some(__function) = (culib().cudaMemcpy3DBatchAsync) {
            __function(numOps, opList, failIdx, flags, stream)
        } else {
            cudaError::cudaErrorNotSupported
        }
    }
    #[cfg(any(feature = "cuda-13000"))]
    pub unsafe fn cudaMemcpy3DBatchAsync(
        numOps: usize,
        opList: *mut cudaMemcpy3DBatchOp,
        flags: ::core::ffi::c_ulonglong,
        stream: cudaStream_t,
    ) -> cudaError_t {
        if let Some(__function) = (culib().cudaMemcpy3DBatchAsync) {
            __function(numOps, opList, flags, stream)
        } else {
            cudaError::cudaErrorNotSupported
        }
    }
    pub unsafe fn cudaMemcpy3DPeer(p: *const cudaMemcpy3DPeerParms) -> cudaError_t {
        if let Some(__function) = (culib().cudaMemcpy3DPeer) {
            __function(p)
        } else {
            cudaError::cudaErrorNotSupported
        }
    }
    pub unsafe fn cudaMemcpy3DPeerAsync(
        p: *const cudaMemcpy3DPeerParms,
        stream: cudaStream_t,
    ) -> cudaError_t {
        if let Some(__function) = (culib().cudaMemcpy3DPeerAsync) {
            __function(p, stream)
        } else {
            cudaError::cudaErrorNotSupported
        }
    }
    pub unsafe fn cudaMemcpyArrayToArray(
        dst: cudaArray_t,
        wOffsetDst: usize,
        hOffsetDst: usize,
        src: cudaArray_const_t,
        wOffsetSrc: usize,
        hOffsetSrc: usize,
        count: usize,
        kind: cudaMemcpyKind,
    ) -> cudaError_t {
        if let Some(__function) = (culib().cudaMemcpyArrayToArray) {
            __function(
                dst, wOffsetDst, hOffsetDst, src, wOffsetSrc, hOffsetSrc, count, kind,
            )
        } else {
            cudaError::cudaErrorNotSupported
        }
    }
    pub unsafe fn cudaMemcpyAsync(
        dst: *mut ::core::ffi::c_void,
        src: *const ::core::ffi::c_void,
        count: usize,
        kind: cudaMemcpyKind,
        stream: cudaStream_t,
    ) -> cudaError_t {
        if let Some(__function) = (culib().cudaMemcpyAsync) {
            __function(dst, src, count, kind, stream)
        } else {
            cudaError::cudaErrorNotSupported
        }
    }
    #[cfg(any(feature = "cuda-12080", feature = "cuda-12090"))]
    pub unsafe fn cudaMemcpyBatchAsync(
        dsts: *mut *mut ::core::ffi::c_void,
        srcs: *mut *mut ::core::ffi::c_void,
        sizes: *mut usize,
        count: usize,
        attrs: *mut cudaMemcpyAttributes,
        attrsIdxs: *mut usize,
        numAttrs: usize,
        failIdx: *mut usize,
        stream: cudaStream_t,
    ) -> cudaError_t {
        if let Some(__function) = (culib().cudaMemcpyBatchAsync) {
            __function(
                dsts, srcs, sizes, count, attrs, attrsIdxs, numAttrs, failIdx, stream,
            )
        } else {
            cudaError::cudaErrorNotSupported
        }
    }
    #[cfg(any(feature = "cuda-13000"))]
    pub unsafe fn cudaMemcpyBatchAsync(
        dsts: *const *mut ::core::ffi::c_void,
        srcs: *const *const ::core::ffi::c_void,
        sizes: *const usize,
        count: usize,
        attrs: *mut cudaMemcpyAttributes,
        attrsIdxs: *mut usize,
        numAttrs: usize,
        stream: cudaStream_t,
    ) -> cudaError_t {
        if let Some(__function) = (culib().cudaMemcpyBatchAsync) {
            __function(dsts, srcs, sizes, count, attrs, attrsIdxs, numAttrs, stream)
        } else {
            cudaError::cudaErrorNotSupported
        }
    }
    pub unsafe fn cudaMemcpyFromArray(
        dst: *mut ::core::ffi::c_void,
        src: cudaArray_const_t,
        wOffset: usize,
        hOffset: usize,
        count: usize,
        kind: cudaMemcpyKind,
    ) -> cudaError_t {
        if let Some(__function) = (culib().cudaMemcpyFromArray) {
            __function(dst, src, wOffset, hOffset, count, kind)
        } else {
            cudaError::cudaErrorNotSupported
        }
    }
    pub unsafe fn cudaMemcpyFromArrayAsync(
        dst: *mut ::core::ffi::c_void,
        src: cudaArray_const_t,
        wOffset: usize,
        hOffset: usize,
        count: usize,
        kind: cudaMemcpyKind,
        stream: cudaStream_t,
    ) -> cudaError_t {
        if let Some(__function) = (culib().cudaMemcpyFromArrayAsync) {
            __function(dst, src, wOffset, hOffset, count, kind, stream)
        } else {
            cudaError::cudaErrorNotSupported
        }
    }
    pub unsafe fn cudaMemcpyFromSymbol(
        dst: *mut ::core::ffi::c_void,
        symbol: *const ::core::ffi::c_void,
        count: usize,
        offset: usize,
        kind: cudaMemcpyKind,
    ) -> cudaError_t {
        if let Some(__function) = (culib().cudaMemcpyFromSymbol) {
            __function(dst, symbol, count, offset, kind)
        } else {
            cudaError::cudaErrorNotSupported
        }
    }
    pub unsafe fn cudaMemcpyFromSymbolAsync(
        dst: *mut ::core::ffi::c_void,
        symbol: *const ::core::ffi::c_void,
        count: usize,
        offset: usize,
        kind: cudaMemcpyKind,
        stream: cudaStream_t,
    ) -> cudaError_t {
        if let Some(__function) = (culib().cudaMemcpyFromSymbolAsync) {
            __function(dst, symbol, count, offset, kind, stream)
        } else {
            cudaError::cudaErrorNotSupported
        }
    }
    pub unsafe fn cudaMemcpyPeer(
        dst: *mut ::core::ffi::c_void,
        dstDevice: ::core::ffi::c_int,
        src: *const ::core::ffi::c_void,
        srcDevice: ::core::ffi::c_int,
        count: usize,
    ) -> cudaError_t {
        if let Some(__function) = (culib().cudaMemcpyPeer) {
            __function(dst, dstDevice, src, srcDevice, count)
        } else {
            cudaError::cudaErrorNotSupported
        }
    }
    pub unsafe fn cudaMemcpyPeerAsync(
        dst: *mut ::core::ffi::c_void,
        dstDevice: ::core::ffi::c_int,
        src: *const ::core::ffi::c_void,
        srcDevice: ::core::ffi::c_int,
        count: usize,
        stream: cudaStream_t,
    ) -> cudaError_t {
        if let Some(__function) = (culib().cudaMemcpyPeerAsync) {
            __function(dst, dstDevice, src, srcDevice, count, stream)
        } else {
            cudaError::cudaErrorNotSupported
        }
    }
    pub unsafe fn cudaMemcpyToArray(
        dst: cudaArray_t,
        wOffset: usize,
        hOffset: usize,
        src: *const ::core::ffi::c_void,
        count: usize,
        kind: cudaMemcpyKind,
    ) -> cudaError_t {
        if let Some(__function) = (culib().cudaMemcpyToArray) {
            __function(dst, wOffset, hOffset, src, count, kind)
        } else {
            cudaError::cudaErrorNotSupported
        }
    }
    pub unsafe fn cudaMemcpyToArrayAsync(
        dst: cudaArray_t,
        wOffset: usize,
        hOffset: usize,
        src: *const ::core::ffi::c_void,
        count: usize,
        kind: cudaMemcpyKind,
        stream: cudaStream_t,
    ) -> cudaError_t {
        if let Some(__function) = (culib().cudaMemcpyToArrayAsync) {
            __function(dst, wOffset, hOffset, src, count, kind, stream)
        } else {
            cudaError::cudaErrorNotSupported
        }
    }
    pub unsafe fn cudaMemcpyToSymbol(
        symbol: *const ::core::ffi::c_void,
        src: *const ::core::ffi::c_void,
        count: usize,
        offset: usize,
        kind: cudaMemcpyKind,
    ) -> cudaError_t {
        if let Some(__function) = (culib().cudaMemcpyToSymbol) {
            __function(symbol, src, count, offset, kind)
        } else {
            cudaError::cudaErrorNotSupported
        }
    }
    pub unsafe fn cudaMemcpyToSymbolAsync(
        symbol: *const ::core::ffi::c_void,
        src: *const ::core::ffi::c_void,
        count: usize,
        offset: usize,
        kind: cudaMemcpyKind,
        stream: cudaStream_t,
    ) -> cudaError_t {
        if let Some(__function) = (culib().cudaMemcpyToSymbolAsync) {
            __function(symbol, src, count, offset, kind, stream)
        } else {
            cudaError::cudaErrorNotSupported
        }
    }
    pub unsafe fn cudaMemset(
        devPtr: *mut ::core::ffi::c_void,
        value: ::core::ffi::c_int,
        count: usize,
    ) -> cudaError_t {
        if let Some(__function) = (culib().cudaMemset) {
            __function(devPtr, value, count)
        } else {
            cudaError::cudaErrorNotSupported
        }
    }
    pub unsafe fn cudaMemset2D(
        devPtr: *mut ::core::ffi::c_void,
        pitch: usize,
        value: ::core::ffi::c_int,
        width: usize,
        height: usize,
    ) -> cudaError_t {
        if let Some(__function) = (culib().cudaMemset2D) {
            __function(devPtr, pitch, value, width, height)
        } else {
            cudaError::cudaErrorNotSupported
        }
    }
    pub unsafe fn cudaMemset2DAsync(
        devPtr: *mut ::core::ffi::c_void,
        pitch: usize,
        value: ::core::ffi::c_int,
        width: usize,
        height: usize,
        stream: cudaStream_t,
    ) -> cudaError_t {
        if let Some(__function) = (culib().cudaMemset2DAsync) {
            __function(devPtr, pitch, value, width, height, stream)
        } else {
            cudaError::cudaErrorNotSupported
        }
    }
    pub unsafe fn cudaMemset3D(
        pitchedDevPtr: cudaPitchedPtr,
        value: ::core::ffi::c_int,
        extent: cudaExtent,
    ) -> cudaError_t {
        if let Some(__function) = (culib().cudaMemset3D) {
            __function(pitchedDevPtr, value, extent)
        } else {
            cudaError::cudaErrorNotSupported
        }
    }
    pub unsafe fn cudaMemset3DAsync(
        pitchedDevPtr: cudaPitchedPtr,
        value: ::core::ffi::c_int,
        extent: cudaExtent,
        stream: cudaStream_t,
    ) -> cudaError_t {
        if let Some(__function) = (culib().cudaMemset3DAsync) {
            __function(pitchedDevPtr, value, extent, stream)
        } else {
            cudaError::cudaErrorNotSupported
        }
    }
    pub unsafe fn cudaMemsetAsync(
        devPtr: *mut ::core::ffi::c_void,
        value: ::core::ffi::c_int,
        count: usize,
        stream: cudaStream_t,
    ) -> cudaError_t {
        if let Some(__function) = (culib().cudaMemsetAsync) {
            __function(devPtr, value, count, stream)
        } else {
            cudaError::cudaErrorNotSupported
        }
    }
    #[cfg(any(
        feature = "cuda-11060",
        feature = "cuda-11070",
        feature = "cuda-11080",
        feature = "cuda-12000",
        feature = "cuda-12010",
        feature = "cuda-12020",
        feature = "cuda-12030",
        feature = "cuda-12040",
        feature = "cuda-12050",
        feature = "cuda-12060",
        feature = "cuda-12080",
        feature = "cuda-12090",
        feature = "cuda-13000"
    ))]
    pub unsafe fn cudaMipmappedArrayGetMemoryRequirements(
        memoryRequirements: *mut cudaArrayMemoryRequirements,
        mipmap: cudaMipmappedArray_t,
        device: ::core::ffi::c_int,
    ) -> cudaError_t {
        if let Some(__function) = (culib().cudaMipmappedArrayGetMemoryRequirements) {
            __function(memoryRequirements, mipmap, device)
        } else {
            cudaError::cudaErrorNotSupported
        }
    }
    pub unsafe fn cudaMipmappedArrayGetSparseProperties(
        sparseProperties: *mut cudaArraySparseProperties,
        mipmap: cudaMipmappedArray_t,
    ) -> cudaError_t {
        if let Some(__function) = (culib().cudaMipmappedArrayGetSparseProperties) {
            __function(sparseProperties, mipmap)
        } else {
            cudaError::cudaErrorNotSupported
        }
    }
    pub unsafe fn cudaOccupancyAvailableDynamicSMemPerBlock(
        dynamicSmemSize: *mut usize,
        func: *const ::core::ffi::c_void,
        numBlocks: ::core::ffi::c_int,
        blockSize: ::core::ffi::c_int,
    ) -> cudaError_t {
        if let Some(__function) = (culib().cudaOccupancyAvailableDynamicSMemPerBlock) {
            __function(dynamicSmemSize, func, numBlocks, blockSize)
        } else {
            cudaError::cudaErrorNotSupported
        }
    }
    pub unsafe fn cudaOccupancyMaxActiveBlocksPerMultiprocessor(
        numBlocks: *mut ::core::ffi::c_int,
        func: *const ::core::ffi::c_void,
        blockSize: ::core::ffi::c_int,
        dynamicSMemSize: usize,
    ) -> cudaError_t {
        if let Some(__function) = (culib().cudaOccupancyMaxActiveBlocksPerMultiprocessor) {
            __function(numBlocks, func, blockSize, dynamicSMemSize)
        } else {
            cudaError::cudaErrorNotSupported
        }
    }
    pub unsafe fn cudaOccupancyMaxActiveBlocksPerMultiprocessorWithFlags(
        numBlocks: *mut ::core::ffi::c_int,
        func: *const ::core::ffi::c_void,
        blockSize: ::core::ffi::c_int,
        dynamicSMemSize: usize,
        flags: ::core::ffi::c_uint,
    ) -> cudaError_t {
        if let Some(__function) = (culib().cudaOccupancyMaxActiveBlocksPerMultiprocessorWithFlags) {
            __function(numBlocks, func, blockSize, dynamicSMemSize, flags)
        } else {
            cudaError::cudaErrorNotSupported
        }
    }
    #[cfg(any(
        feature = "cuda-11080",
        feature = "cuda-12000",
        feature = "cuda-12010",
        feature = "cuda-12020",
        feature = "cuda-12030",
        feature = "cuda-12040",
        feature = "cuda-12050",
        feature = "cuda-12060",
        feature = "cuda-12080",
        feature = "cuda-12090",
        feature = "cuda-13000"
    ))]
    pub unsafe fn cudaOccupancyMaxActiveClusters(
        numClusters: *mut ::core::ffi::c_int,
        func: *const ::core::ffi::c_void,
        launchConfig: *const cudaLaunchConfig_t,
    ) -> cudaError_t {
        if let Some(__function) = (culib().cudaOccupancyMaxActiveClusters) {
            __function(numClusters, func, launchConfig)
        } else {
            cudaError::cudaErrorNotSupported
        }
    }
    #[cfg(any(
        feature = "cuda-11080",
        feature = "cuda-12000",
        feature = "cuda-12010",
        feature = "cuda-12020",
        feature = "cuda-12030",
        feature = "cuda-12040",
        feature = "cuda-12050",
        feature = "cuda-12060",
        feature = "cuda-12080",
        feature = "cuda-12090",
        feature = "cuda-13000"
    ))]
    pub unsafe fn cudaOccupancyMaxPotentialClusterSize(
        clusterSize: *mut ::core::ffi::c_int,
        func: *const ::core::ffi::c_void,
        launchConfig: *const cudaLaunchConfig_t,
    ) -> cudaError_t {
        if let Some(__function) = (culib().cudaOccupancyMaxPotentialClusterSize) {
            __function(clusterSize, func, launchConfig)
        } else {
            cudaError::cudaErrorNotSupported
        }
    }
    pub unsafe fn cudaPeekAtLastError() -> cudaError_t {
        if let Some(__function) = (culib().cudaPeekAtLastError) {
            __function()
        } else {
            cudaError::cudaErrorNotSupported
        }
    }
    pub unsafe fn cudaPointerGetAttributes(
        attributes: *mut cudaPointerAttributes,
        ptr: *const ::core::ffi::c_void,
    ) -> cudaError_t {
        if let Some(__function) = (culib().cudaPointerGetAttributes) {
            __function(attributes, ptr)
        } else {
            cudaError::cudaErrorNotSupported
        }
    }
    pub unsafe fn cudaProfilerStop() -> cudaError_t {
        if let Some(__function) = (culib().cudaProfilerStop) {
            __function()
        } else {
            cudaError::cudaErrorNotSupported
        }
    }
    pub unsafe fn cudaRuntimeGetVersion(runtimeVersion: *mut ::core::ffi::c_int) -> cudaError_t {
        if let Some(__function) = (culib().cudaRuntimeGetVersion) {
            __function(runtimeVersion)
        } else {
            cudaError::cudaErrorNotSupported
        }
    }
    pub unsafe fn cudaSetDevice(device: ::core::ffi::c_int) -> cudaError_t {
        if let Some(__function) = (culib().cudaSetDevice) {
            __function(device)
        } else {
            cudaError::cudaErrorNotSupported
        }
    }
    pub unsafe fn cudaSetDeviceFlags(flags: ::core::ffi::c_uint) -> cudaError_t {
        if let Some(__function) = (culib().cudaSetDeviceFlags) {
            __function(flags)
        } else {
            cudaError::cudaErrorNotSupported
        }
    }
    #[cfg(any(
        feature = "cuda-11040",
        feature = "cuda-11050",
        feature = "cuda-11060",
        feature = "cuda-11070",
        feature = "cuda-11080",
        feature = "cuda-12000",
        feature = "cuda-12010",
        feature = "cuda-12020",
        feature = "cuda-12030",
        feature = "cuda-12040",
        feature = "cuda-12050",
        feature = "cuda-12060",
        feature = "cuda-12080",
        feature = "cuda-12090"
    ))]
    pub unsafe fn cudaSetDoubleForDevice(d: *mut f64) -> cudaError_t {
        if let Some(__function) = (culib().cudaSetDoubleForDevice) {
            __function(d)
        } else {
            cudaError::cudaErrorNotSupported
        }
    }
    #[cfg(any(
        feature = "cuda-11040",
        feature = "cuda-11050",
        feature = "cuda-11060",
        feature = "cuda-11070",
        feature = "cuda-11080",
        feature = "cuda-12000",
        feature = "cuda-12010",
        feature = "cuda-12020",
        feature = "cuda-12030",
        feature = "cuda-12040",
        feature = "cuda-12050",
        feature = "cuda-12060",
        feature = "cuda-12080",
        feature = "cuda-12090"
    ))]
    pub unsafe fn cudaSetDoubleForHost(d: *mut f64) -> cudaError_t {
        if let Some(__function) = (culib().cudaSetDoubleForHost) {
            __function(d)
        } else {
            cudaError::cudaErrorNotSupported
        }
    }
    pub unsafe fn cudaSetValidDevices(
        device_arr: *mut ::core::ffi::c_int,
        len: ::core::ffi::c_int,
    ) -> cudaError_t {
        if let Some(__function) = (culib().cudaSetValidDevices) {
            __function(device_arr, len)
        } else {
            cudaError::cudaErrorNotSupported
        }
    }
    #[cfg(any(feature = "cuda-13000"))]
    pub unsafe fn cudaSignalExternalSemaphoresAsync(
        extSemArray: *const cudaExternalSemaphore_t,
        paramsArray: *const cudaExternalSemaphoreSignalParams,
        numExtSems: ::core::ffi::c_uint,
        stream: cudaStream_t,
    ) -> cudaError_t {
        if let Some(__function) = (culib().cudaSignalExternalSemaphoresAsync) {
            __function(extSemArray, paramsArray, numExtSems, stream)
        } else {
            cudaError::cudaErrorNotSupported
        }
    }
    #[cfg(any(
        feature = "cuda-11040",
        feature = "cuda-11050",
        feature = "cuda-11060",
        feature = "cuda-11070",
        feature = "cuda-11080",
        feature = "cuda-12000",
        feature = "cuda-12010",
        feature = "cuda-12020",
        feature = "cuda-12030",
        feature = "cuda-12040",
        feature = "cuda-12050",
        feature = "cuda-12060",
        feature = "cuda-12080",
        feature = "cuda-12090"
    ))]
    pub unsafe fn cudaSignalExternalSemaphoresAsync_v2(
        extSemArray: *const cudaExternalSemaphore_t,
        paramsArray: *const cudaExternalSemaphoreSignalParams,
        numExtSems: ::core::ffi::c_uint,
        stream: cudaStream_t,
    ) -> cudaError_t {
        if let Some(__function) = (culib().cudaSignalExternalSemaphoresAsync_v2) {
            __function(extSemArray, paramsArray, numExtSems, stream)
        } else {
            cudaError::cudaErrorNotSupported
        }
    }
    pub unsafe fn cudaStreamAddCallback(
        stream: cudaStream_t,
        callback: cudaStreamCallback_t,
        userData: *mut ::core::ffi::c_void,
        flags: ::core::ffi::c_uint,
    ) -> cudaError_t {
        if let Some(__function) = (culib().cudaStreamAddCallback) {
            __function(stream, callback, userData, flags)
        } else {
            cudaError::cudaErrorNotSupported
        }
    }
    pub unsafe fn cudaStreamAttachMemAsync(
        stream: cudaStream_t,
        devPtr: *mut ::core::ffi::c_void,
        length: usize,
        flags: ::core::ffi::c_uint,
    ) -> cudaError_t {
        if let Some(__function) = (culib().cudaStreamAttachMemAsync) {
            __function(stream, devPtr, length, flags)
        } else {
            cudaError::cudaErrorNotSupported
        }
    }
    pub unsafe fn cudaStreamBeginCapture(
        stream: cudaStream_t,
        mode: cudaStreamCaptureMode,
    ) -> cudaError_t {
        if let Some(__function) = (culib().cudaStreamBeginCapture) {
            __function(stream, mode)
        } else {
            cudaError::cudaErrorNotSupported
        }
    }
    #[cfg(any(
        feature = "cuda-12030",
        feature = "cuda-12040",
        feature = "cuda-12050",
        feature = "cuda-12060",
        feature = "cuda-12080",
        feature = "cuda-12090",
        feature = "cuda-13000"
    ))]
    pub unsafe fn cudaStreamBeginCaptureToGraph(
        stream: cudaStream_t,
        graph: cudaGraph_t,
        dependencies: *const cudaGraphNode_t,
        dependencyData: *const cudaGraphEdgeData,
        numDependencies: usize,
        mode: cudaStreamCaptureMode,
    ) -> cudaError_t {
        if let Some(__function) = (culib().cudaStreamBeginCaptureToGraph) {
            __function(
                stream,
                graph,
                dependencies,
                dependencyData,
                numDependencies,
                mode,
            )
        } else {
            cudaError::cudaErrorNotSupported
        }
    }
    pub unsafe fn cudaStreamCopyAttributes(dst: cudaStream_t, src: cudaStream_t) -> cudaError_t {
        if let Some(__function) = (culib().cudaStreamCopyAttributes) {
            __function(dst, src)
        } else {
            cudaError::cudaErrorNotSupported
        }
    }
    pub unsafe fn cudaStreamCreate(pStream: *mut cudaStream_t) -> cudaError_t {
        if let Some(__function) = (culib().cudaStreamCreate) {
            __function(pStream)
        } else {
            cudaError::cudaErrorNotSupported
        }
    }
    pub unsafe fn cudaStreamCreateWithFlags(
        pStream: *mut cudaStream_t,
        flags: ::core::ffi::c_uint,
    ) -> cudaError_t {
        if let Some(__function) = (culib().cudaStreamCreateWithFlags) {
            __function(pStream, flags)
        } else {
            cudaError::cudaErrorNotSupported
        }
    }
    pub unsafe fn cudaStreamCreateWithPriority(
        pStream: *mut cudaStream_t,
        flags: ::core::ffi::c_uint,
        priority: ::core::ffi::c_int,
    ) -> cudaError_t {
        if let Some(__function) = (culib().cudaStreamCreateWithPriority) {
            __function(pStream, flags, priority)
        } else {
            cudaError::cudaErrorNotSupported
        }
    }
    pub unsafe fn cudaStreamDestroy(stream: cudaStream_t) -> cudaError_t {
        if let Some(__function) = (culib().cudaStreamDestroy) {
            __function(stream)
        } else {
            cudaError::cudaErrorNotSupported
        }
    }
    pub unsafe fn cudaStreamEndCapture(
        stream: cudaStream_t,
        pGraph: *mut cudaGraph_t,
    ) -> cudaError_t {
        if let Some(__function) = (culib().cudaStreamEndCapture) {
            __function(stream, pGraph)
        } else {
            cudaError::cudaErrorNotSupported
        }
    }
    #[cfg(any(
        feature = "cuda-11040",
        feature = "cuda-11050",
        feature = "cuda-11060",
        feature = "cuda-11070"
    ))]
    pub unsafe fn cudaStreamGetAttribute(
        hStream: cudaStream_t,
        attr: cudaStreamAttrID,
        value_out: *mut cudaStreamAttrValue,
    ) -> cudaError_t {
        if let Some(__function) = (culib().cudaStreamGetAttribute) {
            __function(hStream, attr, value_out)
        } else {
            cudaError::cudaErrorNotSupported
        }
    }
    #[cfg(any(
        feature = "cuda-11080",
        feature = "cuda-12000",
        feature = "cuda-12010",
        feature = "cuda-12020",
        feature = "cuda-12030",
        feature = "cuda-12040",
        feature = "cuda-12050",
        feature = "cuda-12060",
        feature = "cuda-12080",
        feature = "cuda-12090",
        feature = "cuda-13000"
    ))]
    pub unsafe fn cudaStreamGetAttribute(
        hStream: cudaStream_t,
        attr: cudaLaunchAttributeID,
        value_out: *mut cudaLaunchAttributeValue,
    ) -> cudaError_t {
        if let Some(__function) = (culib().cudaStreamGetAttribute) {
            __function(hStream, attr, value_out)
        } else {
            cudaError::cudaErrorNotSupported
        }
    }
    #[cfg(any(
        feature = "cuda-11040",
        feature = "cuda-11050",
        feature = "cuda-11060",
        feature = "cuda-11070",
        feature = "cuda-11080"
    ))]
    pub unsafe fn cudaStreamGetCaptureInfo(
        stream: cudaStream_t,
        pCaptureStatus: *mut cudaStreamCaptureStatus,
        pId: *mut ::core::ffi::c_ulonglong,
    ) -> cudaError_t {
        if let Some(__function) = (culib().cudaStreamGetCaptureInfo) {
            __function(stream, pCaptureStatus, pId)
        } else {
            cudaError::cudaErrorNotSupported
        }
    }
    #[cfg(any(feature = "cuda-13000"))]
    pub unsafe fn cudaStreamGetCaptureInfo(
        stream: cudaStream_t,
        captureStatus_out: *mut cudaStreamCaptureStatus,
        id_out: *mut ::core::ffi::c_ulonglong,
        graph_out: *mut cudaGraph_t,
        dependencies_out: *mut *const cudaGraphNode_t,
        edgeData_out: *mut *const cudaGraphEdgeData,
        numDependencies_out: *mut usize,
    ) -> cudaError_t {
        if let Some(__function) = (culib().cudaStreamGetCaptureInfo) {
            __function(
                stream,
                captureStatus_out,
                id_out,
                graph_out,
                dependencies_out,
                edgeData_out,
                numDependencies_out,
            )
        } else {
            cudaError::cudaErrorNotSupported
        }
    }
    #[cfg(any(
        feature = "cuda-11040",
        feature = "cuda-11050",
        feature = "cuda-11060",
        feature = "cuda-11070",
        feature = "cuda-11080",
        feature = "cuda-12000",
        feature = "cuda-12010",
        feature = "cuda-12020",
        feature = "cuda-12030",
        feature = "cuda-12040",
        feature = "cuda-12050",
        feature = "cuda-12060",
        feature = "cuda-12080",
        feature = "cuda-12090"
    ))]
    pub unsafe fn cudaStreamGetCaptureInfo_v2(
        stream: cudaStream_t,
        captureStatus_out: *mut cudaStreamCaptureStatus,
        id_out: *mut ::core::ffi::c_ulonglong,
        graph_out: *mut cudaGraph_t,
        dependencies_out: *mut *const cudaGraphNode_t,
        numDependencies_out: *mut usize,
    ) -> cudaError_t {
        if let Some(__function) = (culib().cudaStreamGetCaptureInfo_v2) {
            __function(
                stream,
                captureStatus_out,
                id_out,
                graph_out,
                dependencies_out,
                numDependencies_out,
            )
        } else {
            cudaError::cudaErrorNotSupported
        }
    }
    #[cfg(any(
        feature = "cuda-12030",
        feature = "cuda-12040",
        feature = "cuda-12050",
        feature = "cuda-12060",
        feature = "cuda-12080",
        feature = "cuda-12090"
    ))]
    pub unsafe fn cudaStreamGetCaptureInfo_v3(
        stream: cudaStream_t,
        captureStatus_out: *mut cudaStreamCaptureStatus,
        id_out: *mut ::core::ffi::c_ulonglong,
        graph_out: *mut cudaGraph_t,
        dependencies_out: *mut *const cudaGraphNode_t,
        edgeData_out: *mut *const cudaGraphEdgeData,
        numDependencies_out: *mut usize,
    ) -> cudaError_t {
        if let Some(__function) = (culib().cudaStreamGetCaptureInfo_v3) {
            __function(
                stream,
                captureStatus_out,
                id_out,
                graph_out,
                dependencies_out,
                edgeData_out,
                numDependencies_out,
            )
        } else {
            cudaError::cudaErrorNotSupported
        }
    }
    #[cfg(any(feature = "cuda-12080", feature = "cuda-12090", feature = "cuda-13000"))]
    pub unsafe fn cudaStreamGetDevice(
        hStream: cudaStream_t,
        device: *mut ::core::ffi::c_int,
    ) -> cudaError_t {
        if let Some(__function) = (culib().cudaStreamGetDevice) {
            __function(hStream, device)
        } else {
            cudaError::cudaErrorNotSupported
        }
    }
    pub unsafe fn cudaStreamGetFlags(
        hStream: cudaStream_t,
        flags: *mut ::core::ffi::c_uint,
    ) -> cudaError_t {
        if let Some(__function) = (culib().cudaStreamGetFlags) {
            __function(hStream, flags)
        } else {
            cudaError::cudaErrorNotSupported
        }
    }
    #[cfg(any(
        feature = "cuda-12000",
        feature = "cuda-12010",
        feature = "cuda-12020",
        feature = "cuda-12030",
        feature = "cuda-12040",
        feature = "cuda-12050",
        feature = "cuda-12060",
        feature = "cuda-12080",
        feature = "cuda-12090",
        feature = "cuda-13000"
    ))]
    pub unsafe fn cudaStreamGetId(
        hStream: cudaStream_t,
        streamId: *mut ::core::ffi::c_ulonglong,
    ) -> cudaError_t {
        if let Some(__function) = (culib().cudaStreamGetId) {
            __function(hStream, streamId)
        } else {
            cudaError::cudaErrorNotSupported
        }
    }
    pub unsafe fn cudaStreamGetPriority(
        hStream: cudaStream_t,
        priority: *mut ::core::ffi::c_int,
    ) -> cudaError_t {
        if let Some(__function) = (culib().cudaStreamGetPriority) {
            __function(hStream, priority)
        } else {
            cudaError::cudaErrorNotSupported
        }
    }
    pub unsafe fn cudaStreamIsCapturing(
        stream: cudaStream_t,
        pCaptureStatus: *mut cudaStreamCaptureStatus,
    ) -> cudaError_t {
        if let Some(__function) = (culib().cudaStreamIsCapturing) {
            __function(stream, pCaptureStatus)
        } else {
            cudaError::cudaErrorNotSupported
        }
    }
    pub unsafe fn cudaStreamQuery(stream: cudaStream_t) -> cudaError_t {
        if let Some(__function) = (culib().cudaStreamQuery) {
            __function(stream)
        } else {
            cudaError::cudaErrorNotSupported
        }
    }
    #[cfg(any(
        feature = "cuda-11040",
        feature = "cuda-11050",
        feature = "cuda-11060",
        feature = "cuda-11070"
    ))]
    pub unsafe fn cudaStreamSetAttribute(
        hStream: cudaStream_t,
        attr: cudaStreamAttrID,
        value: *const cudaStreamAttrValue,
    ) -> cudaError_t {
        if let Some(__function) = (culib().cudaStreamSetAttribute) {
            __function(hStream, attr, value)
        } else {
            cudaError::cudaErrorNotSupported
        }
    }
    #[cfg(any(
        feature = "cuda-11080",
        feature = "cuda-12000",
        feature = "cuda-12010",
        feature = "cuda-12020",
        feature = "cuda-12030",
        feature = "cuda-12040",
        feature = "cuda-12050",
        feature = "cuda-12060",
        feature = "cuda-12080",
        feature = "cuda-12090",
        feature = "cuda-13000"
    ))]
    pub unsafe fn cudaStreamSetAttribute(
        hStream: cudaStream_t,
        attr: cudaLaunchAttributeID,
        value: *const cudaLaunchAttributeValue,
    ) -> cudaError_t {
        if let Some(__function) = (culib().cudaStreamSetAttribute) {
            __function(hStream, attr, value)
        } else {
            cudaError::cudaErrorNotSupported
        }
    }
    pub unsafe fn cudaStreamSynchronize(stream: cudaStream_t) -> cudaError_t {
        if let Some(__function) = (culib().cudaStreamSynchronize) {
            __function(stream)
        } else {
            cudaError::cudaErrorNotSupported
        }
    }
    #[cfg(any(
        feature = "cuda-11040",
        feature = "cuda-11050",
        feature = "cuda-11060",
        feature = "cuda-11070",
        feature = "cuda-11080",
        feature = "cuda-12000",
        feature = "cuda-12010",
        feature = "cuda-12020",
        feature = "cuda-12030",
        feature = "cuda-12040",
        feature = "cuda-12050",
        feature = "cuda-12060",
        feature = "cuda-12080",
        feature = "cuda-12090"
    ))]
    pub unsafe fn cudaStreamUpdateCaptureDependencies(
        stream: cudaStream_t,
        dependencies: *mut cudaGraphNode_t,
        numDependencies: usize,
        flags: ::core::ffi::c_uint,
    ) -> cudaError_t {
        if let Some(__function) = (culib().cudaStreamUpdateCaptureDependencies) {
            __function(stream, dependencies, numDependencies, flags)
        } else {
            cudaError::cudaErrorNotSupported
        }
    }
    #[cfg(any(feature = "cuda-13000"))]
    pub unsafe fn cudaStreamUpdateCaptureDependencies(
        stream: cudaStream_t,
        dependencies: *mut cudaGraphNode_t,
        dependencyData: *const cudaGraphEdgeData,
        numDependencies: usize,
        flags: ::core::ffi::c_uint,
    ) -> cudaError_t {
        if let Some(__function) = (culib().cudaStreamUpdateCaptureDependencies) {
            __function(stream, dependencies, dependencyData, numDependencies, flags)
        } else {
            cudaError::cudaErrorNotSupported
        }
    }
    #[cfg(any(
        feature = "cuda-12030",
        feature = "cuda-12040",
        feature = "cuda-12050",
        feature = "cuda-12060",
        feature = "cuda-12080",
        feature = "cuda-12090"
    ))]
    pub unsafe fn cudaStreamUpdateCaptureDependencies_v2(
        stream: cudaStream_t,
        dependencies: *mut cudaGraphNode_t,
        dependencyData: *const cudaGraphEdgeData,
        numDependencies: usize,
        flags: ::core::ffi::c_uint,
    ) -> cudaError_t {
        if let Some(__function) = (culib().cudaStreamUpdateCaptureDependencies_v2) {
            __function(stream, dependencies, dependencyData, numDependencies, flags)
        } else {
            cudaError::cudaErrorNotSupported
        }
    }
    pub unsafe fn cudaStreamWaitEvent(
        stream: cudaStream_t,
        event: cudaEvent_t,
        flags: ::core::ffi::c_uint,
    ) -> cudaError_t {
        if let Some(__function) = (culib().cudaStreamWaitEvent) {
            __function(stream, event, flags)
        } else {
            cudaError::cudaErrorNotSupported
        }
    }
    pub unsafe fn cudaThreadExchangeStreamCaptureMode(
        mode: *mut cudaStreamCaptureMode,
    ) -> cudaError_t {
        if let Some(__function) = (culib().cudaThreadExchangeStreamCaptureMode) {
            __function(mode)
        } else {
            cudaError::cudaErrorNotSupported
        }
    }
    #[cfg(any(
        feature = "cuda-11040",
        feature = "cuda-11050",
        feature = "cuda-11060",
        feature = "cuda-11070",
        feature = "cuda-11080",
        feature = "cuda-12000",
        feature = "cuda-12010",
        feature = "cuda-12020",
        feature = "cuda-12030",
        feature = "cuda-12040",
        feature = "cuda-12050",
        feature = "cuda-12060",
        feature = "cuda-12080",
        feature = "cuda-12090"
    ))]
    pub unsafe fn cudaThreadExit() -> cudaError_t {
        if let Some(__function) = (culib().cudaThreadExit) {
            __function()
        } else {
            cudaError::cudaErrorNotSupported
        }
    }
    #[cfg(any(
        feature = "cuda-11040",
        feature = "cuda-11050",
        feature = "cuda-11060",
        feature = "cuda-11070",
        feature = "cuda-11080",
        feature = "cuda-12000",
        feature = "cuda-12010",
        feature = "cuda-12020",
        feature = "cuda-12030",
        feature = "cuda-12040",
        feature = "cuda-12050",
        feature = "cuda-12060",
        feature = "cuda-12080",
        feature = "cuda-12090"
    ))]
    pub unsafe fn cudaThreadGetCacheConfig(pCacheConfig: *mut cudaFuncCache) -> cudaError_t {
        if let Some(__function) = (culib().cudaThreadGetCacheConfig) {
            __function(pCacheConfig)
        } else {
            cudaError::cudaErrorNotSupported
        }
    }
    #[cfg(any(
        feature = "cuda-11040",
        feature = "cuda-11050",
        feature = "cuda-11060",
        feature = "cuda-11070",
        feature = "cuda-11080",
        feature = "cuda-12000",
        feature = "cuda-12010",
        feature = "cuda-12020",
        feature = "cuda-12030",
        feature = "cuda-12040",
        feature = "cuda-12050",
        feature = "cuda-12060",
        feature = "cuda-12080",
        feature = "cuda-12090"
    ))]
    pub unsafe fn cudaThreadGetLimit(pValue: *mut usize, limit: cudaLimit) -> cudaError_t {
        if let Some(__function) = (culib().cudaThreadGetLimit) {
            __function(pValue, limit)
        } else {
            cudaError::cudaErrorNotSupported
        }
    }
    #[cfg(any(
        feature = "cuda-11040",
        feature = "cuda-11050",
        feature = "cuda-11060",
        feature = "cuda-11070",
        feature = "cuda-11080",
        feature = "cuda-12000",
        feature = "cuda-12010",
        feature = "cuda-12020",
        feature = "cuda-12030",
        feature = "cuda-12040",
        feature = "cuda-12050",
        feature = "cuda-12060",
        feature = "cuda-12080",
        feature = "cuda-12090"
    ))]
    pub unsafe fn cudaThreadSetCacheConfig(cacheConfig: cudaFuncCache) -> cudaError_t {
        if let Some(__function) = (culib().cudaThreadSetCacheConfig) {
            __function(cacheConfig)
        } else {
            cudaError::cudaErrorNotSupported
        }
    }
    #[cfg(any(
        feature = "cuda-11040",
        feature = "cuda-11050",
        feature = "cuda-11060",
        feature = "cuda-11070",
        feature = "cuda-11080",
        feature = "cuda-12000",
        feature = "cuda-12010",
        feature = "cuda-12020",
        feature = "cuda-12030",
        feature = "cuda-12040",
        feature = "cuda-12050",
        feature = "cuda-12060",
        feature = "cuda-12080",
        feature = "cuda-12090"
    ))]
    pub unsafe fn cudaThreadSetLimit(limit: cudaLimit, value: usize) -> cudaError_t {
        if let Some(__function) = (culib().cudaThreadSetLimit) {
            __function(limit, value)
        } else {
            cudaError::cudaErrorNotSupported
        }
    }
    #[cfg(any(
        feature = "cuda-11040",
        feature = "cuda-11050",
        feature = "cuda-11060",
        feature = "cuda-11070",
        feature = "cuda-11080",
        feature = "cuda-12000",
        feature = "cuda-12010",
        feature = "cuda-12020",
        feature = "cuda-12030",
        feature = "cuda-12040",
        feature = "cuda-12050",
        feature = "cuda-12060",
        feature = "cuda-12080",
        feature = "cuda-12090"
    ))]
    pub unsafe fn cudaThreadSynchronize() -> cudaError_t {
        if let Some(__function) = (culib().cudaThreadSynchronize) {
            __function()
        } else {
            cudaError::cudaErrorNotSupported
        }
    }
    #[cfg(any(
        feature = "cuda-11040",
        feature = "cuda-11050",
        feature = "cuda-11060",
        feature = "cuda-11070",
        feature = "cuda-11080"
    ))]
    pub unsafe fn cudaUnbindTexture(texref: *const textureReference) -> cudaError_t {
        if let Some(__function) = (culib().cudaUnbindTexture) {
            __function(texref)
        } else {
            cudaError::cudaErrorNotSupported
        }
    }
    pub unsafe fn cudaUserObjectCreate(
        object_out: *mut cudaUserObject_t,
        ptr: *mut ::core::ffi::c_void,
        destroy: cudaHostFn_t,
        initialRefcount: ::core::ffi::c_uint,
        flags: ::core::ffi::c_uint,
    ) -> cudaError_t {
        if let Some(__function) = (culib().cudaUserObjectCreate) {
            __function(object_out, ptr, destroy, initialRefcount, flags)
        } else {
            cudaError::cudaErrorNotSupported
        }
    }
    pub unsafe fn cudaUserObjectRelease(
        object: cudaUserObject_t,
        count: ::core::ffi::c_uint,
    ) -> cudaError_t {
        if let Some(__function) = (culib().cudaUserObjectRelease) {
            __function(object, count)
        } else {
            cudaError::cudaErrorNotSupported
        }
    }
    pub unsafe fn cudaUserObjectRetain(
        object: cudaUserObject_t,
        count: ::core::ffi::c_uint,
    ) -> cudaError_t {
        if let Some(__function) = (culib().cudaUserObjectRetain) {
            __function(object, count)
        } else {
            cudaError::cudaErrorNotSupported
        }
    }
    #[cfg(any(feature = "cuda-13000"))]
    pub unsafe fn cudaWaitExternalSemaphoresAsync(
        extSemArray: *const cudaExternalSemaphore_t,
        paramsArray: *const cudaExternalSemaphoreWaitParams,
        numExtSems: ::core::ffi::c_uint,
        stream: cudaStream_t,
    ) -> cudaError_t {
        if let Some(__function) = (culib().cudaWaitExternalSemaphoresAsync) {
            __function(extSemArray, paramsArray, numExtSems, stream)
        } else {
            cudaError::cudaErrorNotSupported
        }
    }
    #[cfg(any(
        feature = "cuda-11040",
        feature = "cuda-11050",
        feature = "cuda-11060",
        feature = "cuda-11070",
        feature = "cuda-11080",
        feature = "cuda-12000",
        feature = "cuda-12010",
        feature = "cuda-12020",
        feature = "cuda-12030",
        feature = "cuda-12040",
        feature = "cuda-12050",
        feature = "cuda-12060",
        feature = "cuda-12080",
        feature = "cuda-12090"
    ))]
    pub unsafe fn cudaWaitExternalSemaphoresAsync_v2(
        extSemArray: *const cudaExternalSemaphore_t,
        paramsArray: *const cudaExternalSemaphoreWaitParams,
        numExtSems: ::core::ffi::c_uint,
        stream: cudaStream_t,
    ) -> cudaError_t {
        if let Some(__function) = (culib().cudaWaitExternalSemaphoresAsync_v2) {
            __function(extSemArray, paramsArray, numExtSems, stream)
        } else {
            cudaError::cudaErrorNotSupported
        }
    }
    pub struct Lib {
        __library: ::libloading::Library,
        pub cudaArrayGetInfo: Option<
            unsafe extern "C" fn(
                desc: *mut cudaChannelFormatDesc,
                extent: *mut cudaExtent,
                flags: *mut ::core::ffi::c_uint,
                array: cudaArray_t,
            ) -> cudaError_t,
        >,
        #[cfg(any(
            feature = "cuda-11060",
            feature = "cuda-11070",
            feature = "cuda-11080",
            feature = "cuda-12000",
            feature = "cuda-12010",
            feature = "cuda-12020",
            feature = "cuda-12030",
            feature = "cuda-12040",
            feature = "cuda-12050",
            feature = "cuda-12060",
            feature = "cuda-12080",
            feature = "cuda-12090",
            feature = "cuda-13000"
        ))]
        pub cudaArrayGetMemoryRequirements: Option<
            unsafe extern "C" fn(
                memoryRequirements: *mut cudaArrayMemoryRequirements,
                array: cudaArray_t,
                device: ::core::ffi::c_int,
            ) -> cudaError_t,
        >,
        pub cudaArrayGetPlane: Option<
            unsafe extern "C" fn(
                pPlaneArray: *mut cudaArray_t,
                hArray: cudaArray_t,
                planeIdx: ::core::ffi::c_uint,
            ) -> cudaError_t,
        >,
        pub cudaArrayGetSparseProperties: Option<
            unsafe extern "C" fn(
                sparseProperties: *mut cudaArraySparseProperties,
                array: cudaArray_t,
            ) -> cudaError_t,
        >,
        #[cfg(any(
            feature = "cuda-11040",
            feature = "cuda-11050",
            feature = "cuda-11060",
            feature = "cuda-11070",
            feature = "cuda-11080"
        ))]
        pub cudaBindSurfaceToArray: Option<
            unsafe extern "C" fn(
                surfref: *const surfaceReference,
                array: cudaArray_const_t,
                desc: *const cudaChannelFormatDesc,
            ) -> cudaError_t,
        >,
        #[cfg(any(
            feature = "cuda-11040",
            feature = "cuda-11050",
            feature = "cuda-11060",
            feature = "cuda-11070",
            feature = "cuda-11080"
        ))]
        pub cudaBindTexture: Option<
            unsafe extern "C" fn(
                offset: *mut usize,
                texref: *const textureReference,
                devPtr: *const ::core::ffi::c_void,
                desc: *const cudaChannelFormatDesc,
                size: usize,
            ) -> cudaError_t,
        >,
        #[cfg(any(
            feature = "cuda-11040",
            feature = "cuda-11050",
            feature = "cuda-11060",
            feature = "cuda-11070",
            feature = "cuda-11080"
        ))]
        pub cudaBindTexture2D: Option<
            unsafe extern "C" fn(
                offset: *mut usize,
                texref: *const textureReference,
                devPtr: *const ::core::ffi::c_void,
                desc: *const cudaChannelFormatDesc,
                width: usize,
                height: usize,
                pitch: usize,
            ) -> cudaError_t,
        >,
        #[cfg(any(
            feature = "cuda-11040",
            feature = "cuda-11050",
            feature = "cuda-11060",
            feature = "cuda-11070",
            feature = "cuda-11080"
        ))]
        pub cudaBindTextureToArray: Option<
            unsafe extern "C" fn(
                texref: *const textureReference,
                array: cudaArray_const_t,
                desc: *const cudaChannelFormatDesc,
            ) -> cudaError_t,
        >,
        #[cfg(any(
            feature = "cuda-11040",
            feature = "cuda-11050",
            feature = "cuda-11060",
            feature = "cuda-11070",
            feature = "cuda-11080"
        ))]
        pub cudaBindTextureToMipmappedArray: Option<
            unsafe extern "C" fn(
                texref: *const textureReference,
                mipmappedArray: cudaMipmappedArray_const_t,
                desc: *const cudaChannelFormatDesc,
            ) -> cudaError_t,
        >,
        pub cudaChooseDevice: Option<
            unsafe extern "C" fn(
                device: *mut ::core::ffi::c_int,
                prop: *const cudaDeviceProp,
            ) -> cudaError_t,
        >,
        pub cudaCreateChannelDesc: Option<
            unsafe extern "C" fn(
                x: ::core::ffi::c_int,
                y: ::core::ffi::c_int,
                z: ::core::ffi::c_int,
                w: ::core::ffi::c_int,
                f: cudaChannelFormatKind,
            ) -> cudaChannelFormatDesc,
        >,
        pub cudaCreateSurfaceObject: Option<
            unsafe extern "C" fn(
                pSurfObject: *mut cudaSurfaceObject_t,
                pResDesc: *const cudaResourceDesc,
            ) -> cudaError_t,
        >,
        pub cudaCreateTextureObject: Option<
            unsafe extern "C" fn(
                pTexObject: *mut cudaTextureObject_t,
                pResDesc: *const cudaResourceDesc,
                pTexDesc: *const cudaTextureDesc,
                pResViewDesc: *const cudaResourceViewDesc,
            ) -> cudaError_t,
        >,
        #[cfg(any(feature = "cuda-11080"))]
        pub cudaCreateTextureObject_v2: Option<
            unsafe extern "C" fn(
                pTexObject: *mut cudaTextureObject_t,
                pResDesc: *const cudaResourceDesc,
                pTexDesc: *const cudaTextureDesc_v2,
                pResViewDesc: *const cudaResourceViewDesc,
            ) -> cudaError_t,
        >,
        pub cudaCtxResetPersistingL2Cache: Option<unsafe extern "C" fn() -> cudaError_t>,
        pub cudaDestroyExternalMemory:
            Option<unsafe extern "C" fn(extMem: cudaExternalMemory_t) -> cudaError_t>,
        pub cudaDestroyExternalSemaphore:
            Option<unsafe extern "C" fn(extSem: cudaExternalSemaphore_t) -> cudaError_t>,
        pub cudaDestroySurfaceObject:
            Option<unsafe extern "C" fn(surfObject: cudaSurfaceObject_t) -> cudaError_t>,
        pub cudaDestroyTextureObject:
            Option<unsafe extern "C" fn(texObject: cudaTextureObject_t) -> cudaError_t>,
        pub cudaDeviceCanAccessPeer: Option<
            unsafe extern "C" fn(
                canAccessPeer: *mut ::core::ffi::c_int,
                device: ::core::ffi::c_int,
                peerDevice: ::core::ffi::c_int,
            ) -> cudaError_t,
        >,
        pub cudaDeviceDisablePeerAccess:
            Option<unsafe extern "C" fn(peerDevice: ::core::ffi::c_int) -> cudaError_t>,
        pub cudaDeviceEnablePeerAccess: Option<
            unsafe extern "C" fn(
                peerDevice: ::core::ffi::c_int,
                flags: ::core::ffi::c_uint,
            ) -> cudaError_t,
        >,
        pub cudaDeviceFlushGPUDirectRDMAWrites: Option<
            unsafe extern "C" fn(
                target: cudaFlushGPUDirectRDMAWritesTarget,
                scope: cudaFlushGPUDirectRDMAWritesScope,
            ) -> cudaError_t,
        >,
        pub cudaDeviceGetAttribute: Option<
            unsafe extern "C" fn(
                value: *mut ::core::ffi::c_int,
                attr: cudaDeviceAttr,
                device: ::core::ffi::c_int,
            ) -> cudaError_t,
        >,
        pub cudaDeviceGetByPCIBusId: Option<
            unsafe extern "C" fn(
                device: *mut ::core::ffi::c_int,
                pciBusId: *const ::core::ffi::c_char,
            ) -> cudaError_t,
        >,
        pub cudaDeviceGetCacheConfig:
            Option<unsafe extern "C" fn(pCacheConfig: *mut cudaFuncCache) -> cudaError_t>,
        pub cudaDeviceGetDefaultMemPool: Option<
            unsafe extern "C" fn(
                memPool: *mut cudaMemPool_t,
                device: ::core::ffi::c_int,
            ) -> cudaError_t,
        >,
        pub cudaDeviceGetGraphMemAttribute: Option<
            unsafe extern "C" fn(
                device: ::core::ffi::c_int,
                attr: cudaGraphMemAttributeType,
                value: *mut ::core::ffi::c_void,
            ) -> cudaError_t,
        >,
        #[cfg(any(feature = "cuda-13000"))]
        pub cudaDeviceGetHostAtomicCapabilities: Option<
            unsafe extern "C" fn(
                capabilities: *mut ::core::ffi::c_uint,
                operations: *const cudaAtomicOperation,
                count: ::core::ffi::c_uint,
                device: ::core::ffi::c_int,
            ) -> cudaError_t,
        >,
        pub cudaDeviceGetLimit:
            Option<unsafe extern "C" fn(pValue: *mut usize, limit: cudaLimit) -> cudaError_t>,
        pub cudaDeviceGetMemPool: Option<
            unsafe extern "C" fn(
                memPool: *mut cudaMemPool_t,
                device: ::core::ffi::c_int,
            ) -> cudaError_t,
        >,
        #[cfg(any(feature = "cuda-13000"))]
        pub cudaDeviceGetP2PAtomicCapabilities: Option<
            unsafe extern "C" fn(
                capabilities: *mut ::core::ffi::c_uint,
                operations: *const cudaAtomicOperation,
                count: ::core::ffi::c_uint,
                srcDevice: ::core::ffi::c_int,
                dstDevice: ::core::ffi::c_int,
            ) -> cudaError_t,
        >,
        pub cudaDeviceGetP2PAttribute: Option<
            unsafe extern "C" fn(
                value: *mut ::core::ffi::c_int,
                attr: cudaDeviceP2PAttr,
                srcDevice: ::core::ffi::c_int,
                dstDevice: ::core::ffi::c_int,
            ) -> cudaError_t,
        >,
        pub cudaDeviceGetPCIBusId: Option<
            unsafe extern "C" fn(
                pciBusId: *mut ::core::ffi::c_char,
                len: ::core::ffi::c_int,
                device: ::core::ffi::c_int,
            ) -> cudaError_t,
        >,
        pub cudaDeviceGetSharedMemConfig:
            Option<unsafe extern "C" fn(pConfig: *mut cudaSharedMemConfig) -> cudaError_t>,
        pub cudaDeviceGetStreamPriorityRange: Option<
            unsafe extern "C" fn(
                leastPriority: *mut ::core::ffi::c_int,
                greatestPriority: *mut ::core::ffi::c_int,
            ) -> cudaError_t,
        >,
        pub cudaDeviceGetTexture1DLinearMaxWidth: Option<
            unsafe extern "C" fn(
                maxWidthInElements: *mut usize,
                fmtDesc: *const cudaChannelFormatDesc,
                device: ::core::ffi::c_int,
            ) -> cudaError_t,
        >,
        pub cudaDeviceGraphMemTrim:
            Option<unsafe extern "C" fn(device: ::core::ffi::c_int) -> cudaError_t>,
        #[cfg(any(
            feature = "cuda-12040",
            feature = "cuda-12050",
            feature = "cuda-12060",
            feature = "cuda-12080",
            feature = "cuda-12090",
            feature = "cuda-13000"
        ))]
        pub cudaDeviceRegisterAsyncNotification: Option<
            unsafe extern "C" fn(
                device: ::core::ffi::c_int,
                callbackFunc: cudaAsyncCallback,
                userData: *mut ::core::ffi::c_void,
                callback: *mut cudaAsyncCallbackHandle_t,
            ) -> cudaError_t,
        >,
        pub cudaDeviceReset: Option<unsafe extern "C" fn() -> cudaError_t>,
        pub cudaDeviceSetCacheConfig:
            Option<unsafe extern "C" fn(cacheConfig: cudaFuncCache) -> cudaError_t>,
        pub cudaDeviceSetGraphMemAttribute: Option<
            unsafe extern "C" fn(
                device: ::core::ffi::c_int,
                attr: cudaGraphMemAttributeType,
                value: *mut ::core::ffi::c_void,
            ) -> cudaError_t,
        >,
        pub cudaDeviceSetLimit:
            Option<unsafe extern "C" fn(limit: cudaLimit, value: usize) -> cudaError_t>,
        pub cudaDeviceSetMemPool: Option<
            unsafe extern "C" fn(device: ::core::ffi::c_int, memPool: cudaMemPool_t) -> cudaError_t,
        >,
        pub cudaDeviceSetSharedMemConfig:
            Option<unsafe extern "C" fn(config: cudaSharedMemConfig) -> cudaError_t>,
        pub cudaDeviceSynchronize: Option<unsafe extern "C" fn() -> cudaError_t>,
        #[cfg(any(
            feature = "cuda-12040",
            feature = "cuda-12050",
            feature = "cuda-12060",
            feature = "cuda-12080",
            feature = "cuda-12090",
            feature = "cuda-13000"
        ))]
        pub cudaDeviceUnregisterAsyncNotification: Option<
            unsafe extern "C" fn(
                device: ::core::ffi::c_int,
                callback: cudaAsyncCallbackHandle_t,
            ) -> cudaError_t,
        >,
        pub cudaDriverGetVersion:
            Option<unsafe extern "C" fn(driverVersion: *mut ::core::ffi::c_int) -> cudaError_t>,
        pub cudaEventCreate: Option<unsafe extern "C" fn(event: *mut cudaEvent_t) -> cudaError_t>,
        pub cudaEventCreateWithFlags: Option<
            unsafe extern "C" fn(
                event: *mut cudaEvent_t,
                flags: ::core::ffi::c_uint,
            ) -> cudaError_t,
        >,
        pub cudaEventDestroy: Option<unsafe extern "C" fn(event: cudaEvent_t) -> cudaError_t>,
        pub cudaEventElapsedTime: Option<
            unsafe extern "C" fn(ms: *mut f32, start: cudaEvent_t, end: cudaEvent_t) -> cudaError_t,
        >,
        #[cfg(any(feature = "cuda-12080", feature = "cuda-12090"))]
        pub cudaEventElapsedTime_v2: Option<
            unsafe extern "C" fn(ms: *mut f32, start: cudaEvent_t, end: cudaEvent_t) -> cudaError_t,
        >,
        pub cudaEventQuery: Option<unsafe extern "C" fn(event: cudaEvent_t) -> cudaError_t>,
        pub cudaEventRecord:
            Option<unsafe extern "C" fn(event: cudaEvent_t, stream: cudaStream_t) -> cudaError_t>,
        pub cudaEventRecordWithFlags: Option<
            unsafe extern "C" fn(
                event: cudaEvent_t,
                stream: cudaStream_t,
                flags: ::core::ffi::c_uint,
            ) -> cudaError_t,
        >,
        pub cudaEventSynchronize: Option<unsafe extern "C" fn(event: cudaEvent_t) -> cudaError_t>,
        pub cudaExternalMemoryGetMappedBuffer: Option<
            unsafe extern "C" fn(
                devPtr: *mut *mut ::core::ffi::c_void,
                extMem: cudaExternalMemory_t,
                bufferDesc: *const cudaExternalMemoryBufferDesc,
            ) -> cudaError_t,
        >,
        pub cudaExternalMemoryGetMappedMipmappedArray: Option<
            unsafe extern "C" fn(
                mipmap: *mut cudaMipmappedArray_t,
                extMem: cudaExternalMemory_t,
                mipmapDesc: *const cudaExternalMemoryMipmappedArrayDesc,
            ) -> cudaError_t,
        >,
        pub cudaFree: Option<unsafe extern "C" fn(devPtr: *mut ::core::ffi::c_void) -> cudaError_t>,
        pub cudaFreeArray: Option<unsafe extern "C" fn(array: cudaArray_t) -> cudaError_t>,
        pub cudaFreeAsync: Option<
            unsafe extern "C" fn(
                devPtr: *mut ::core::ffi::c_void,
                hStream: cudaStream_t,
            ) -> cudaError_t,
        >,
        pub cudaFreeHost:
            Option<unsafe extern "C" fn(ptr: *mut ::core::ffi::c_void) -> cudaError_t>,
        pub cudaFreeMipmappedArray:
            Option<unsafe extern "C" fn(mipmappedArray: cudaMipmappedArray_t) -> cudaError_t>,
        pub cudaFuncGetAttributes: Option<
            unsafe extern "C" fn(
                attr: *mut cudaFuncAttributes,
                func: *const ::core::ffi::c_void,
            ) -> cudaError_t,
        >,
        #[cfg(any(
            feature = "cuda-12030",
            feature = "cuda-12040",
            feature = "cuda-12050",
            feature = "cuda-12060",
            feature = "cuda-12080",
            feature = "cuda-12090",
            feature = "cuda-13000"
        ))]
        pub cudaFuncGetName: Option<
            unsafe extern "C" fn(
                name: *mut *const ::core::ffi::c_char,
                func: *const ::core::ffi::c_void,
            ) -> cudaError_t,
        >,
        #[cfg(any(
            feature = "cuda-12040",
            feature = "cuda-12050",
            feature = "cuda-12060",
            feature = "cuda-12080",
            feature = "cuda-12090",
            feature = "cuda-13000"
        ))]
        pub cudaFuncGetParamInfo: Option<
            unsafe extern "C" fn(
                func: *const ::core::ffi::c_void,
                paramIndex: usize,
                paramOffset: *mut usize,
                paramSize: *mut usize,
            ) -> cudaError_t,
        >,
        pub cudaFuncSetAttribute: Option<
            unsafe extern "C" fn(
                func: *const ::core::ffi::c_void,
                attr: cudaFuncAttribute,
                value: ::core::ffi::c_int,
            ) -> cudaError_t,
        >,
        pub cudaFuncSetCacheConfig: Option<
            unsafe extern "C" fn(
                func: *const ::core::ffi::c_void,
                cacheConfig: cudaFuncCache,
            ) -> cudaError_t,
        >,
        pub cudaFuncSetSharedMemConfig: Option<
            unsafe extern "C" fn(
                func: *const ::core::ffi::c_void,
                config: cudaSharedMemConfig,
            ) -> cudaError_t,
        >,
        pub cudaGetChannelDesc: Option<
            unsafe extern "C" fn(
                desc: *mut cudaChannelFormatDesc,
                array: cudaArray_const_t,
            ) -> cudaError_t,
        >,
        pub cudaGetDevice:
            Option<unsafe extern "C" fn(device: *mut ::core::ffi::c_int) -> cudaError_t>,
        pub cudaGetDeviceCount:
            Option<unsafe extern "C" fn(count: *mut ::core::ffi::c_int) -> cudaError_t>,
        pub cudaGetDeviceFlags:
            Option<unsafe extern "C" fn(flags: *mut ::core::ffi::c_uint) -> cudaError_t>,
        #[cfg(any(
            feature = "cuda-11040",
            feature = "cuda-11050",
            feature = "cuda-11060",
            feature = "cuda-11070",
            feature = "cuda-11080",
            feature = "cuda-13000"
        ))]
        pub cudaGetDeviceProperties: Option<
            unsafe extern "C" fn(
                prop: *mut cudaDeviceProp,
                device: ::core::ffi::c_int,
            ) -> cudaError_t,
        >,
        #[cfg(any(
            feature = "cuda-12000",
            feature = "cuda-12010",
            feature = "cuda-12020",
            feature = "cuda-12030",
            feature = "cuda-12040",
            feature = "cuda-12050",
            feature = "cuda-12060",
            feature = "cuda-12080",
            feature = "cuda-12090"
        ))]
        pub cudaGetDeviceProperties_v2: Option<
            unsafe extern "C" fn(
                prop: *mut cudaDeviceProp,
                device: ::core::ffi::c_int,
            ) -> cudaError_t,
        >,
        #[cfg(any(
            feature = "cuda-11040",
            feature = "cuda-11050",
            feature = "cuda-11060",
            feature = "cuda-11070",
            feature = "cuda-11080"
        ))]
        pub cudaGetDriverEntryPoint: Option<
            unsafe extern "C" fn(
                symbol: *const ::core::ffi::c_char,
                funcPtr: *mut *mut ::core::ffi::c_void,
                flags: ::core::ffi::c_ulonglong,
            ) -> cudaError_t,
        >,
        #[cfg(any(
            feature = "cuda-12000",
            feature = "cuda-12010",
            feature = "cuda-12020",
            feature = "cuda-12030",
            feature = "cuda-12040",
            feature = "cuda-12050",
            feature = "cuda-12060",
            feature = "cuda-12080",
            feature = "cuda-12090",
            feature = "cuda-13000"
        ))]
        pub cudaGetDriverEntryPoint: Option<
            unsafe extern "C" fn(
                symbol: *const ::core::ffi::c_char,
                funcPtr: *mut *mut ::core::ffi::c_void,
                flags: ::core::ffi::c_ulonglong,
                driverStatus: *mut cudaDriverEntryPointQueryResult,
            ) -> cudaError_t,
        >,
        #[cfg(any(
            feature = "cuda-12050",
            feature = "cuda-12060",
            feature = "cuda-12080",
            feature = "cuda-12090",
            feature = "cuda-13000"
        ))]
        pub cudaGetDriverEntryPointByVersion: Option<
            unsafe extern "C" fn(
                symbol: *const ::core::ffi::c_char,
                funcPtr: *mut *mut ::core::ffi::c_void,
                cudaVersion: ::core::ffi::c_uint,
                flags: ::core::ffi::c_ulonglong,
                driverStatus: *mut cudaDriverEntryPointQueryResult,
            ) -> cudaError_t,
        >,
        pub cudaGetErrorName:
            Option<unsafe extern "C" fn(error: cudaError_t) -> *const ::core::ffi::c_char>,
        pub cudaGetErrorString:
            Option<unsafe extern "C" fn(error: cudaError_t) -> *const ::core::ffi::c_char>,
        pub cudaGetExportTable: Option<
            unsafe extern "C" fn(
                ppExportTable: *mut *const ::core::ffi::c_void,
                pExportTableId: *const cudaUUID_t,
            ) -> cudaError_t,
        >,
        pub cudaGetFuncBySymbol: Option<
            unsafe extern "C" fn(
                functionPtr: *mut cudaFunction_t,
                symbolPtr: *const ::core::ffi::c_void,
            ) -> cudaError_t,
        >,
        #[cfg(any(
            feature = "cuda-12010",
            feature = "cuda-12020",
            feature = "cuda-12030",
            feature = "cuda-12040",
            feature = "cuda-12050",
            feature = "cuda-12060",
            feature = "cuda-12080",
            feature = "cuda-12090",
            feature = "cuda-13000"
        ))]
        pub cudaGetKernel: Option<
            unsafe extern "C" fn(
                kernelPtr: *mut cudaKernel_t,
                entryFuncAddr: *const ::core::ffi::c_void,
            ) -> cudaError_t,
        >,
        pub cudaGetLastError: Option<unsafe extern "C" fn() -> cudaError_t>,
        pub cudaGetMipmappedArrayLevel: Option<
            unsafe extern "C" fn(
                levelArray: *mut cudaArray_t,
                mipmappedArray: cudaMipmappedArray_const_t,
                level: ::core::ffi::c_uint,
            ) -> cudaError_t,
        >,
        pub cudaGetSurfaceObjectResourceDesc: Option<
            unsafe extern "C" fn(
                pResDesc: *mut cudaResourceDesc,
                surfObject: cudaSurfaceObject_t,
            ) -> cudaError_t,
        >,
        #[cfg(any(
            feature = "cuda-11040",
            feature = "cuda-11050",
            feature = "cuda-11060",
            feature = "cuda-11070",
            feature = "cuda-11080"
        ))]
        pub cudaGetSurfaceReference: Option<
            unsafe extern "C" fn(
                surfref: *mut *const surfaceReference,
                symbol: *const ::core::ffi::c_void,
            ) -> cudaError_t,
        >,
        pub cudaGetSymbolAddress: Option<
            unsafe extern "C" fn(
                devPtr: *mut *mut ::core::ffi::c_void,
                symbol: *const ::core::ffi::c_void,
            ) -> cudaError_t,
        >,
        pub cudaGetSymbolSize: Option<
            unsafe extern "C" fn(
                size: *mut usize,
                symbol: *const ::core::ffi::c_void,
            ) -> cudaError_t,
        >,
        #[cfg(any(
            feature = "cuda-11040",
            feature = "cuda-11050",
            feature = "cuda-11060",
            feature = "cuda-11070",
            feature = "cuda-11080"
        ))]
        pub cudaGetTextureAlignmentOffset: Option<
            unsafe extern "C" fn(
                offset: *mut usize,
                texref: *const textureReference,
            ) -> cudaError_t,
        >,
        pub cudaGetTextureObjectResourceDesc: Option<
            unsafe extern "C" fn(
                pResDesc: *mut cudaResourceDesc,
                texObject: cudaTextureObject_t,
            ) -> cudaError_t,
        >,
        pub cudaGetTextureObjectResourceViewDesc: Option<
            unsafe extern "C" fn(
                pResViewDesc: *mut cudaResourceViewDesc,
                texObject: cudaTextureObject_t,
            ) -> cudaError_t,
        >,
        pub cudaGetTextureObjectTextureDesc: Option<
            unsafe extern "C" fn(
                pTexDesc: *mut cudaTextureDesc,
                texObject: cudaTextureObject_t,
            ) -> cudaError_t,
        >,
        #[cfg(any(feature = "cuda-11080"))]
        pub cudaGetTextureObjectTextureDesc_v2: Option<
            unsafe extern "C" fn(
                pTexDesc: *mut cudaTextureDesc_v2,
                texObject: cudaTextureObject_t,
            ) -> cudaError_t,
        >,
        #[cfg(any(
            feature = "cuda-11040",
            feature = "cuda-11050",
            feature = "cuda-11060",
            feature = "cuda-11070",
            feature = "cuda-11080"
        ))]
        pub cudaGetTextureReference: Option<
            unsafe extern "C" fn(
                texref: *mut *const textureReference,
                symbol: *const ::core::ffi::c_void,
            ) -> cudaError_t,
        >,
        pub cudaGraphAddChildGraphNode: Option<
            unsafe extern "C" fn(
                pGraphNode: *mut cudaGraphNode_t,
                graph: cudaGraph_t,
                pDependencies: *const cudaGraphNode_t,
                numDependencies: usize,
                childGraph: cudaGraph_t,
            ) -> cudaError_t,
        >,
        #[cfg(any(
            feature = "cuda-11040",
            feature = "cuda-11050",
            feature = "cuda-11060",
            feature = "cuda-11070",
            feature = "cuda-11080",
            feature = "cuda-12000",
            feature = "cuda-12010",
            feature = "cuda-12020",
            feature = "cuda-12030",
            feature = "cuda-12040",
            feature = "cuda-12050",
            feature = "cuda-12060",
            feature = "cuda-12080",
            feature = "cuda-12090"
        ))]
        pub cudaGraphAddDependencies: Option<
            unsafe extern "C" fn(
                graph: cudaGraph_t,
                from: *const cudaGraphNode_t,
                to: *const cudaGraphNode_t,
                numDependencies: usize,
            ) -> cudaError_t,
        >,
        #[cfg(any(feature = "cuda-13000"))]
        pub cudaGraphAddDependencies: Option<
            unsafe extern "C" fn(
                graph: cudaGraph_t,
                from: *const cudaGraphNode_t,
                to: *const cudaGraphNode_t,
                edgeData: *const cudaGraphEdgeData,
                numDependencies: usize,
            ) -> cudaError_t,
        >,
        #[cfg(any(
            feature = "cuda-12030",
            feature = "cuda-12040",
            feature = "cuda-12050",
            feature = "cuda-12060",
            feature = "cuda-12080",
            feature = "cuda-12090"
        ))]
        pub cudaGraphAddDependencies_v2: Option<
            unsafe extern "C" fn(
                graph: cudaGraph_t,
                from: *const cudaGraphNode_t,
                to: *const cudaGraphNode_t,
                edgeData: *const cudaGraphEdgeData,
                numDependencies: usize,
            ) -> cudaError_t,
        >,
        pub cudaGraphAddEmptyNode: Option<
            unsafe extern "C" fn(
                pGraphNode: *mut cudaGraphNode_t,
                graph: cudaGraph_t,
                pDependencies: *const cudaGraphNode_t,
                numDependencies: usize,
            ) -> cudaError_t,
        >,
        pub cudaGraphAddEventRecordNode: Option<
            unsafe extern "C" fn(
                pGraphNode: *mut cudaGraphNode_t,
                graph: cudaGraph_t,
                pDependencies: *const cudaGraphNode_t,
                numDependencies: usize,
                event: cudaEvent_t,
            ) -> cudaError_t,
        >,
        pub cudaGraphAddEventWaitNode: Option<
            unsafe extern "C" fn(
                pGraphNode: *mut cudaGraphNode_t,
                graph: cudaGraph_t,
                pDependencies: *const cudaGraphNode_t,
                numDependencies: usize,
                event: cudaEvent_t,
            ) -> cudaError_t,
        >,
        pub cudaGraphAddExternalSemaphoresSignalNode: Option<
            unsafe extern "C" fn(
                pGraphNode: *mut cudaGraphNode_t,
                graph: cudaGraph_t,
                pDependencies: *const cudaGraphNode_t,
                numDependencies: usize,
                nodeParams: *const cudaExternalSemaphoreSignalNodeParams,
            ) -> cudaError_t,
        >,
        pub cudaGraphAddExternalSemaphoresWaitNode: Option<
            unsafe extern "C" fn(
                pGraphNode: *mut cudaGraphNode_t,
                graph: cudaGraph_t,
                pDependencies: *const cudaGraphNode_t,
                numDependencies: usize,
                nodeParams: *const cudaExternalSemaphoreWaitNodeParams,
            ) -> cudaError_t,
        >,
        pub cudaGraphAddHostNode: Option<
            unsafe extern "C" fn(
                pGraphNode: *mut cudaGraphNode_t,
                graph: cudaGraph_t,
                pDependencies: *const cudaGraphNode_t,
                numDependencies: usize,
                pNodeParams: *const cudaHostNodeParams,
            ) -> cudaError_t,
        >,
        pub cudaGraphAddKernelNode: Option<
            unsafe extern "C" fn(
                pGraphNode: *mut cudaGraphNode_t,
                graph: cudaGraph_t,
                pDependencies: *const cudaGraphNode_t,
                numDependencies: usize,
                pNodeParams: *const cudaKernelNodeParams,
            ) -> cudaError_t,
        >,
        pub cudaGraphAddMemAllocNode: Option<
            unsafe extern "C" fn(
                pGraphNode: *mut cudaGraphNode_t,
                graph: cudaGraph_t,
                pDependencies: *const cudaGraphNode_t,
                numDependencies: usize,
                nodeParams: *mut cudaMemAllocNodeParams,
            ) -> cudaError_t,
        >,
        pub cudaGraphAddMemFreeNode: Option<
            unsafe extern "C" fn(
                pGraphNode: *mut cudaGraphNode_t,
                graph: cudaGraph_t,
                pDependencies: *const cudaGraphNode_t,
                numDependencies: usize,
                dptr: *mut ::core::ffi::c_void,
            ) -> cudaError_t,
        >,
        pub cudaGraphAddMemcpyNode: Option<
            unsafe extern "C" fn(
                pGraphNode: *mut cudaGraphNode_t,
                graph: cudaGraph_t,
                pDependencies: *const cudaGraphNode_t,
                numDependencies: usize,
                pCopyParams: *const cudaMemcpy3DParms,
            ) -> cudaError_t,
        >,
        pub cudaGraphAddMemcpyNode1D: Option<
            unsafe extern "C" fn(
                pGraphNode: *mut cudaGraphNode_t,
                graph: cudaGraph_t,
                pDependencies: *const cudaGraphNode_t,
                numDependencies: usize,
                dst: *mut ::core::ffi::c_void,
                src: *const ::core::ffi::c_void,
                count: usize,
                kind: cudaMemcpyKind,
            ) -> cudaError_t,
        >,
        pub cudaGraphAddMemcpyNodeFromSymbol: Option<
            unsafe extern "C" fn(
                pGraphNode: *mut cudaGraphNode_t,
                graph: cudaGraph_t,
                pDependencies: *const cudaGraphNode_t,
                numDependencies: usize,
                dst: *mut ::core::ffi::c_void,
                symbol: *const ::core::ffi::c_void,
                count: usize,
                offset: usize,
                kind: cudaMemcpyKind,
            ) -> cudaError_t,
        >,
        pub cudaGraphAddMemcpyNodeToSymbol: Option<
            unsafe extern "C" fn(
                pGraphNode: *mut cudaGraphNode_t,
                graph: cudaGraph_t,
                pDependencies: *const cudaGraphNode_t,
                numDependencies: usize,
                symbol: *const ::core::ffi::c_void,
                src: *const ::core::ffi::c_void,
                count: usize,
                offset: usize,
                kind: cudaMemcpyKind,
            ) -> cudaError_t,
        >,
        pub cudaGraphAddMemsetNode: Option<
            unsafe extern "C" fn(
                pGraphNode: *mut cudaGraphNode_t,
                graph: cudaGraph_t,
                pDependencies: *const cudaGraphNode_t,
                numDependencies: usize,
                pMemsetParams: *const cudaMemsetParams,
            ) -> cudaError_t,
        >,
        #[cfg(any(
            feature = "cuda-12020",
            feature = "cuda-12030",
            feature = "cuda-12040",
            feature = "cuda-12050",
            feature = "cuda-12060",
            feature = "cuda-12080",
            feature = "cuda-12090"
        ))]
        pub cudaGraphAddNode: Option<
            unsafe extern "C" fn(
                pGraphNode: *mut cudaGraphNode_t,
                graph: cudaGraph_t,
                pDependencies: *const cudaGraphNode_t,
                numDependencies: usize,
                nodeParams: *mut cudaGraphNodeParams,
            ) -> cudaError_t,
        >,
        #[cfg(any(feature = "cuda-13000"))]
        pub cudaGraphAddNode: Option<
            unsafe extern "C" fn(
                pGraphNode: *mut cudaGraphNode_t,
                graph: cudaGraph_t,
                pDependencies: *const cudaGraphNode_t,
                dependencyData: *const cudaGraphEdgeData,
                numDependencies: usize,
                nodeParams: *mut cudaGraphNodeParams,
            ) -> cudaError_t,
        >,
        #[cfg(any(
            feature = "cuda-12030",
            feature = "cuda-12040",
            feature = "cuda-12050",
            feature = "cuda-12060",
            feature = "cuda-12080",
            feature = "cuda-12090"
        ))]
        pub cudaGraphAddNode_v2: Option<
            unsafe extern "C" fn(
                pGraphNode: *mut cudaGraphNode_t,
                graph: cudaGraph_t,
                pDependencies: *const cudaGraphNode_t,
                dependencyData: *const cudaGraphEdgeData,
                numDependencies: usize,
                nodeParams: *mut cudaGraphNodeParams,
            ) -> cudaError_t,
        >,
        pub cudaGraphChildGraphNodeGetGraph: Option<
            unsafe extern "C" fn(node: cudaGraphNode_t, pGraph: *mut cudaGraph_t) -> cudaError_t,
        >,
        pub cudaGraphClone: Option<
            unsafe extern "C" fn(
                pGraphClone: *mut cudaGraph_t,
                originalGraph: cudaGraph_t,
            ) -> cudaError_t,
        >,
        #[cfg(any(
            feature = "cuda-12030",
            feature = "cuda-12040",
            feature = "cuda-12050",
            feature = "cuda-12060",
            feature = "cuda-12080",
            feature = "cuda-12090",
            feature = "cuda-13000"
        ))]
        pub cudaGraphConditionalHandleCreate: Option<
            unsafe extern "C" fn(
                pHandle_out: *mut cudaGraphConditionalHandle,
                graph: cudaGraph_t,
                defaultLaunchValue: ::core::ffi::c_uint,
                flags: ::core::ffi::c_uint,
            ) -> cudaError_t,
        >,
        pub cudaGraphCreate: Option<
            unsafe extern "C" fn(
                pGraph: *mut cudaGraph_t,
                flags: ::core::ffi::c_uint,
            ) -> cudaError_t,
        >,
        pub cudaGraphDebugDotPrint: Option<
            unsafe extern "C" fn(
                graph: cudaGraph_t,
                path: *const ::core::ffi::c_char,
                flags: ::core::ffi::c_uint,
            ) -> cudaError_t,
        >,
        pub cudaGraphDestroy: Option<unsafe extern "C" fn(graph: cudaGraph_t) -> cudaError_t>,
        pub cudaGraphDestroyNode:
            Option<unsafe extern "C" fn(node: cudaGraphNode_t) -> cudaError_t>,
        pub cudaGraphEventRecordNodeGetEvent: Option<
            unsafe extern "C" fn(node: cudaGraphNode_t, event_out: *mut cudaEvent_t) -> cudaError_t,
        >,
        pub cudaGraphEventRecordNodeSetEvent:
            Option<unsafe extern "C" fn(node: cudaGraphNode_t, event: cudaEvent_t) -> cudaError_t>,
        pub cudaGraphEventWaitNodeGetEvent: Option<
            unsafe extern "C" fn(node: cudaGraphNode_t, event_out: *mut cudaEvent_t) -> cudaError_t,
        >,
        pub cudaGraphEventWaitNodeSetEvent:
            Option<unsafe extern "C" fn(node: cudaGraphNode_t, event: cudaEvent_t) -> cudaError_t>,
        pub cudaGraphExecChildGraphNodeSetParams: Option<
            unsafe extern "C" fn(
                hGraphExec: cudaGraphExec_t,
                node: cudaGraphNode_t,
                childGraph: cudaGraph_t,
            ) -> cudaError_t,
        >,
        pub cudaGraphExecDestroy:
            Option<unsafe extern "C" fn(graphExec: cudaGraphExec_t) -> cudaError_t>,
        pub cudaGraphExecEventRecordNodeSetEvent: Option<
            unsafe extern "C" fn(
                hGraphExec: cudaGraphExec_t,
                hNode: cudaGraphNode_t,
                event: cudaEvent_t,
            ) -> cudaError_t,
        >,
        pub cudaGraphExecEventWaitNodeSetEvent: Option<
            unsafe extern "C" fn(
                hGraphExec: cudaGraphExec_t,
                hNode: cudaGraphNode_t,
                event: cudaEvent_t,
            ) -> cudaError_t,
        >,
        pub cudaGraphExecExternalSemaphoresSignalNodeSetParams: Option<
            unsafe extern "C" fn(
                hGraphExec: cudaGraphExec_t,
                hNode: cudaGraphNode_t,
                nodeParams: *const cudaExternalSemaphoreSignalNodeParams,
            ) -> cudaError_t,
        >,
        pub cudaGraphExecExternalSemaphoresWaitNodeSetParams: Option<
            unsafe extern "C" fn(
                hGraphExec: cudaGraphExec_t,
                hNode: cudaGraphNode_t,
                nodeParams: *const cudaExternalSemaphoreWaitNodeParams,
            ) -> cudaError_t,
        >,
        #[cfg(any(
            feature = "cuda-12000",
            feature = "cuda-12010",
            feature = "cuda-12020",
            feature = "cuda-12030",
            feature = "cuda-12040",
            feature = "cuda-12050",
            feature = "cuda-12060",
            feature = "cuda-12080",
            feature = "cuda-12090",
            feature = "cuda-13000"
        ))]
        pub cudaGraphExecGetFlags: Option<
            unsafe extern "C" fn(
                graphExec: cudaGraphExec_t,
                flags: *mut ::core::ffi::c_ulonglong,
            ) -> cudaError_t,
        >,
        pub cudaGraphExecHostNodeSetParams: Option<
            unsafe extern "C" fn(
                hGraphExec: cudaGraphExec_t,
                node: cudaGraphNode_t,
                pNodeParams: *const cudaHostNodeParams,
            ) -> cudaError_t,
        >,
        pub cudaGraphExecKernelNodeSetParams: Option<
            unsafe extern "C" fn(
                hGraphExec: cudaGraphExec_t,
                node: cudaGraphNode_t,
                pNodeParams: *const cudaKernelNodeParams,
            ) -> cudaError_t,
        >,
        pub cudaGraphExecMemcpyNodeSetParams: Option<
            unsafe extern "C" fn(
                hGraphExec: cudaGraphExec_t,
                node: cudaGraphNode_t,
                pNodeParams: *const cudaMemcpy3DParms,
            ) -> cudaError_t,
        >,
        pub cudaGraphExecMemcpyNodeSetParams1D: Option<
            unsafe extern "C" fn(
                hGraphExec: cudaGraphExec_t,
                node: cudaGraphNode_t,
                dst: *mut ::core::ffi::c_void,
                src: *const ::core::ffi::c_void,
                count: usize,
                kind: cudaMemcpyKind,
            ) -> cudaError_t,
        >,
        pub cudaGraphExecMemcpyNodeSetParamsFromSymbol: Option<
            unsafe extern "C" fn(
                hGraphExec: cudaGraphExec_t,
                node: cudaGraphNode_t,
                dst: *mut ::core::ffi::c_void,
                symbol: *const ::core::ffi::c_void,
                count: usize,
                offset: usize,
                kind: cudaMemcpyKind,
            ) -> cudaError_t,
        >,
        pub cudaGraphExecMemcpyNodeSetParamsToSymbol: Option<
            unsafe extern "C" fn(
                hGraphExec: cudaGraphExec_t,
                node: cudaGraphNode_t,
                symbol: *const ::core::ffi::c_void,
                src: *const ::core::ffi::c_void,
                count: usize,
                offset: usize,
                kind: cudaMemcpyKind,
            ) -> cudaError_t,
        >,
        pub cudaGraphExecMemsetNodeSetParams: Option<
            unsafe extern "C" fn(
                hGraphExec: cudaGraphExec_t,
                node: cudaGraphNode_t,
                pNodeParams: *const cudaMemsetParams,
            ) -> cudaError_t,
        >,
        #[cfg(any(
            feature = "cuda-12020",
            feature = "cuda-12030",
            feature = "cuda-12040",
            feature = "cuda-12050",
            feature = "cuda-12060",
            feature = "cuda-12080",
            feature = "cuda-12090",
            feature = "cuda-13000"
        ))]
        pub cudaGraphExecNodeSetParams: Option<
            unsafe extern "C" fn(
                graphExec: cudaGraphExec_t,
                node: cudaGraphNode_t,
                nodeParams: *mut cudaGraphNodeParams,
            ) -> cudaError_t,
        >,
        #[cfg(any(
            feature = "cuda-11040",
            feature = "cuda-11050",
            feature = "cuda-11060",
            feature = "cuda-11070",
            feature = "cuda-11080"
        ))]
        pub cudaGraphExecUpdate: Option<
            unsafe extern "C" fn(
                hGraphExec: cudaGraphExec_t,
                hGraph: cudaGraph_t,
                hErrorNode_out: *mut cudaGraphNode_t,
                updateResult_out: *mut cudaGraphExecUpdateResult,
            ) -> cudaError_t,
        >,
        #[cfg(any(
            feature = "cuda-12000",
            feature = "cuda-12010",
            feature = "cuda-12020",
            feature = "cuda-12030",
            feature = "cuda-12040",
            feature = "cuda-12050",
            feature = "cuda-12060",
            feature = "cuda-12080",
            feature = "cuda-12090",
            feature = "cuda-13000"
        ))]
        pub cudaGraphExecUpdate: Option<
            unsafe extern "C" fn(
                hGraphExec: cudaGraphExec_t,
                hGraph: cudaGraph_t,
                resultInfo: *mut cudaGraphExecUpdateResultInfo,
            ) -> cudaError_t,
        >,
        pub cudaGraphExternalSemaphoresSignalNodeGetParams: Option<
            unsafe extern "C" fn(
                hNode: cudaGraphNode_t,
                params_out: *mut cudaExternalSemaphoreSignalNodeParams,
            ) -> cudaError_t,
        >,
        pub cudaGraphExternalSemaphoresSignalNodeSetParams: Option<
            unsafe extern "C" fn(
                hNode: cudaGraphNode_t,
                nodeParams: *const cudaExternalSemaphoreSignalNodeParams,
            ) -> cudaError_t,
        >,
        pub cudaGraphExternalSemaphoresWaitNodeGetParams: Option<
            unsafe extern "C" fn(
                hNode: cudaGraphNode_t,
                params_out: *mut cudaExternalSemaphoreWaitNodeParams,
            ) -> cudaError_t,
        >,
        pub cudaGraphExternalSemaphoresWaitNodeSetParams: Option<
            unsafe extern "C" fn(
                hNode: cudaGraphNode_t,
                nodeParams: *const cudaExternalSemaphoreWaitNodeParams,
            ) -> cudaError_t,
        >,
        #[cfg(any(
            feature = "cuda-11040",
            feature = "cuda-11050",
            feature = "cuda-11060",
            feature = "cuda-11070",
            feature = "cuda-11080",
            feature = "cuda-12000",
            feature = "cuda-12010",
            feature = "cuda-12020",
            feature = "cuda-12030",
            feature = "cuda-12040",
            feature = "cuda-12050",
            feature = "cuda-12060",
            feature = "cuda-12080",
            feature = "cuda-12090"
        ))]
        pub cudaGraphGetEdges: Option<
            unsafe extern "C" fn(
                graph: cudaGraph_t,
                from: *mut cudaGraphNode_t,
                to: *mut cudaGraphNode_t,
                numEdges: *mut usize,
            ) -> cudaError_t,
        >,
        #[cfg(any(feature = "cuda-13000"))]
        pub cudaGraphGetEdges: Option<
            unsafe extern "C" fn(
                graph: cudaGraph_t,
                from: *mut cudaGraphNode_t,
                to: *mut cudaGraphNode_t,
                edgeData: *mut cudaGraphEdgeData,
                numEdges: *mut usize,
            ) -> cudaError_t,
        >,
        #[cfg(any(
            feature = "cuda-12030",
            feature = "cuda-12040",
            feature = "cuda-12050",
            feature = "cuda-12060",
            feature = "cuda-12080",
            feature = "cuda-12090"
        ))]
        pub cudaGraphGetEdges_v2: Option<
            unsafe extern "C" fn(
                graph: cudaGraph_t,
                from: *mut cudaGraphNode_t,
                to: *mut cudaGraphNode_t,
                edgeData: *mut cudaGraphEdgeData,
                numEdges: *mut usize,
            ) -> cudaError_t,
        >,
        pub cudaGraphGetNodes: Option<
            unsafe extern "C" fn(
                graph: cudaGraph_t,
                nodes: *mut cudaGraphNode_t,
                numNodes: *mut usize,
            ) -> cudaError_t,
        >,
        pub cudaGraphGetRootNodes: Option<
            unsafe extern "C" fn(
                graph: cudaGraph_t,
                pRootNodes: *mut cudaGraphNode_t,
                pNumRootNodes: *mut usize,
            ) -> cudaError_t,
        >,
        pub cudaGraphHostNodeGetParams: Option<
            unsafe extern "C" fn(
                node: cudaGraphNode_t,
                pNodeParams: *mut cudaHostNodeParams,
            ) -> cudaError_t,
        >,
        pub cudaGraphHostNodeSetParams: Option<
            unsafe extern "C" fn(
                node: cudaGraphNode_t,
                pNodeParams: *const cudaHostNodeParams,
            ) -> cudaError_t,
        >,
        #[cfg(any(
            feature = "cuda-11040",
            feature = "cuda-11050",
            feature = "cuda-11060",
            feature = "cuda-11070",
            feature = "cuda-11080"
        ))]
        pub cudaGraphInstantiate: Option<
            unsafe extern "C" fn(
                pGraphExec: *mut cudaGraphExec_t,
                graph: cudaGraph_t,
                pErrorNode: *mut cudaGraphNode_t,
                pLogBuffer: *mut ::core::ffi::c_char,
                bufferSize: usize,
            ) -> cudaError_t,
        >,
        #[cfg(any(
            feature = "cuda-12000",
            feature = "cuda-12010",
            feature = "cuda-12020",
            feature = "cuda-12030",
            feature = "cuda-12040",
            feature = "cuda-12050",
            feature = "cuda-12060",
            feature = "cuda-12080",
            feature = "cuda-12090",
            feature = "cuda-13000"
        ))]
        pub cudaGraphInstantiate: Option<
            unsafe extern "C" fn(
                pGraphExec: *mut cudaGraphExec_t,
                graph: cudaGraph_t,
                flags: ::core::ffi::c_ulonglong,
            ) -> cudaError_t,
        >,
        pub cudaGraphInstantiateWithFlags: Option<
            unsafe extern "C" fn(
                pGraphExec: *mut cudaGraphExec_t,
                graph: cudaGraph_t,
                flags: ::core::ffi::c_ulonglong,
            ) -> cudaError_t,
        >,
        #[cfg(any(
            feature = "cuda-12000",
            feature = "cuda-12010",
            feature = "cuda-12020",
            feature = "cuda-12030",
            feature = "cuda-12040",
            feature = "cuda-12050",
            feature = "cuda-12060",
            feature = "cuda-12080",
            feature = "cuda-12090",
            feature = "cuda-13000"
        ))]
        pub cudaGraphInstantiateWithParams: Option<
            unsafe extern "C" fn(
                pGraphExec: *mut cudaGraphExec_t,
                graph: cudaGraph_t,
                instantiateParams: *mut cudaGraphInstantiateParams,
            ) -> cudaError_t,
        >,
        #[cfg(any(
            feature = "cuda-11040",
            feature = "cuda-11050",
            feature = "cuda-11060",
            feature = "cuda-11070",
            feature = "cuda-11080",
            feature = "cuda-12000",
            feature = "cuda-12010",
            feature = "cuda-12020",
            feature = "cuda-12030",
            feature = "cuda-12040",
            feature = "cuda-12050",
            feature = "cuda-12060",
            feature = "cuda-12080",
            feature = "cuda-12090"
        ))]
        pub cudaGraphKernelNodeCopyAttributes: Option<
            unsafe extern "C" fn(hSrc: cudaGraphNode_t, hDst: cudaGraphNode_t) -> cudaError_t,
        >,
        #[cfg(any(feature = "cuda-13000"))]
        pub cudaGraphKernelNodeCopyAttributes: Option<
            unsafe extern "C" fn(hDst: cudaGraphNode_t, hSrc: cudaGraphNode_t) -> cudaError_t,
        >,
        #[cfg(any(
            feature = "cuda-11040",
            feature = "cuda-11050",
            feature = "cuda-11060",
            feature = "cuda-11070"
        ))]
        pub cudaGraphKernelNodeGetAttribute: Option<
            unsafe extern "C" fn(
                hNode: cudaGraphNode_t,
                attr: cudaKernelNodeAttrID,
                value_out: *mut cudaKernelNodeAttrValue,
            ) -> cudaError_t,
        >,
        #[cfg(any(
            feature = "cuda-11080",
            feature = "cuda-12000",
            feature = "cuda-12010",
            feature = "cuda-12020",
            feature = "cuda-12030",
            feature = "cuda-12040",
            feature = "cuda-12050",
            feature = "cuda-12060",
            feature = "cuda-12080",
            feature = "cuda-12090",
            feature = "cuda-13000"
        ))]
        pub cudaGraphKernelNodeGetAttribute: Option<
            unsafe extern "C" fn(
                hNode: cudaGraphNode_t,
                attr: cudaLaunchAttributeID,
                value_out: *mut cudaLaunchAttributeValue,
            ) -> cudaError_t,
        >,
        pub cudaGraphKernelNodeGetParams: Option<
            unsafe extern "C" fn(
                node: cudaGraphNode_t,
                pNodeParams: *mut cudaKernelNodeParams,
            ) -> cudaError_t,
        >,
        #[cfg(any(
            feature = "cuda-11040",
            feature = "cuda-11050",
            feature = "cuda-11060",
            feature = "cuda-11070"
        ))]
        pub cudaGraphKernelNodeSetAttribute: Option<
            unsafe extern "C" fn(
                hNode: cudaGraphNode_t,
                attr: cudaKernelNodeAttrID,
                value: *const cudaKernelNodeAttrValue,
            ) -> cudaError_t,
        >,
        #[cfg(any(
            feature = "cuda-11080",
            feature = "cuda-12000",
            feature = "cuda-12010",
            feature = "cuda-12020",
            feature = "cuda-12030",
            feature = "cuda-12040",
            feature = "cuda-12050",
            feature = "cuda-12060",
            feature = "cuda-12080",
            feature = "cuda-12090",
            feature = "cuda-13000"
        ))]
        pub cudaGraphKernelNodeSetAttribute: Option<
            unsafe extern "C" fn(
                hNode: cudaGraphNode_t,
                attr: cudaLaunchAttributeID,
                value: *const cudaLaunchAttributeValue,
            ) -> cudaError_t,
        >,
        pub cudaGraphKernelNodeSetParams: Option<
            unsafe extern "C" fn(
                node: cudaGraphNode_t,
                pNodeParams: *const cudaKernelNodeParams,
            ) -> cudaError_t,
        >,
        pub cudaGraphLaunch: Option<
            unsafe extern "C" fn(graphExec: cudaGraphExec_t, stream: cudaStream_t) -> cudaError_t,
        >,
        pub cudaGraphMemAllocNodeGetParams: Option<
            unsafe extern "C" fn(
                node: cudaGraphNode_t,
                params_out: *mut cudaMemAllocNodeParams,
            ) -> cudaError_t,
        >,
        pub cudaGraphMemFreeNodeGetParams: Option<
            unsafe extern "C" fn(
                node: cudaGraphNode_t,
                dptr_out: *mut ::core::ffi::c_void,
            ) -> cudaError_t,
        >,
        pub cudaGraphMemcpyNodeGetParams: Option<
            unsafe extern "C" fn(
                node: cudaGraphNode_t,
                pNodeParams: *mut cudaMemcpy3DParms,
            ) -> cudaError_t,
        >,
        pub cudaGraphMemcpyNodeSetParams: Option<
            unsafe extern "C" fn(
                node: cudaGraphNode_t,
                pNodeParams: *const cudaMemcpy3DParms,
            ) -> cudaError_t,
        >,
        pub cudaGraphMemcpyNodeSetParams1D: Option<
            unsafe extern "C" fn(
                node: cudaGraphNode_t,
                dst: *mut ::core::ffi::c_void,
                src: *const ::core::ffi::c_void,
                count: usize,
                kind: cudaMemcpyKind,
            ) -> cudaError_t,
        >,
        pub cudaGraphMemcpyNodeSetParamsFromSymbol: Option<
            unsafe extern "C" fn(
                node: cudaGraphNode_t,
                dst: *mut ::core::ffi::c_void,
                symbol: *const ::core::ffi::c_void,
                count: usize,
                offset: usize,
                kind: cudaMemcpyKind,
            ) -> cudaError_t,
        >,
        pub cudaGraphMemcpyNodeSetParamsToSymbol: Option<
            unsafe extern "C" fn(
                node: cudaGraphNode_t,
                symbol: *const ::core::ffi::c_void,
                src: *const ::core::ffi::c_void,
                count: usize,
                offset: usize,
                kind: cudaMemcpyKind,
            ) -> cudaError_t,
        >,
        pub cudaGraphMemsetNodeGetParams: Option<
            unsafe extern "C" fn(
                node: cudaGraphNode_t,
                pNodeParams: *mut cudaMemsetParams,
            ) -> cudaError_t,
        >,
        pub cudaGraphMemsetNodeSetParams: Option<
            unsafe extern "C" fn(
                node: cudaGraphNode_t,
                pNodeParams: *const cudaMemsetParams,
            ) -> cudaError_t,
        >,
        pub cudaGraphNodeFindInClone: Option<
            unsafe extern "C" fn(
                pNode: *mut cudaGraphNode_t,
                originalNode: cudaGraphNode_t,
                clonedGraph: cudaGraph_t,
            ) -> cudaError_t,
        >,
        #[cfg(any(
            feature = "cuda-11040",
            feature = "cuda-11050",
            feature = "cuda-11060",
            feature = "cuda-11070",
            feature = "cuda-11080",
            feature = "cuda-12000",
            feature = "cuda-12010",
            feature = "cuda-12020",
            feature = "cuda-12030",
            feature = "cuda-12040",
            feature = "cuda-12050",
            feature = "cuda-12060",
            feature = "cuda-12080",
            feature = "cuda-12090"
        ))]
        pub cudaGraphNodeGetDependencies: Option<
            unsafe extern "C" fn(
                node: cudaGraphNode_t,
                pDependencies: *mut cudaGraphNode_t,
                pNumDependencies: *mut usize,
            ) -> cudaError_t,
        >,
        #[cfg(any(feature = "cuda-13000"))]
        pub cudaGraphNodeGetDependencies: Option<
            unsafe extern "C" fn(
                node: cudaGraphNode_t,
                pDependencies: *mut cudaGraphNode_t,
                edgeData: *mut cudaGraphEdgeData,
                pNumDependencies: *mut usize,
            ) -> cudaError_t,
        >,
        #[cfg(any(
            feature = "cuda-12030",
            feature = "cuda-12040",
            feature = "cuda-12050",
            feature = "cuda-12060",
            feature = "cuda-12080",
            feature = "cuda-12090"
        ))]
        pub cudaGraphNodeGetDependencies_v2: Option<
            unsafe extern "C" fn(
                node: cudaGraphNode_t,
                pDependencies: *mut cudaGraphNode_t,
                edgeData: *mut cudaGraphEdgeData,
                pNumDependencies: *mut usize,
            ) -> cudaError_t,
        >,
        #[cfg(any(
            feature = "cuda-11040",
            feature = "cuda-11050",
            feature = "cuda-11060",
            feature = "cuda-11070",
            feature = "cuda-11080",
            feature = "cuda-12000",
            feature = "cuda-12010",
            feature = "cuda-12020",
            feature = "cuda-12030",
            feature = "cuda-12040",
            feature = "cuda-12050",
            feature = "cuda-12060",
            feature = "cuda-12080",
            feature = "cuda-12090"
        ))]
        pub cudaGraphNodeGetDependentNodes: Option<
            unsafe extern "C" fn(
                node: cudaGraphNode_t,
                pDependentNodes: *mut cudaGraphNode_t,
                pNumDependentNodes: *mut usize,
            ) -> cudaError_t,
        >,
        #[cfg(any(feature = "cuda-13000"))]
        pub cudaGraphNodeGetDependentNodes: Option<
            unsafe extern "C" fn(
                node: cudaGraphNode_t,
                pDependentNodes: *mut cudaGraphNode_t,
                edgeData: *mut cudaGraphEdgeData,
                pNumDependentNodes: *mut usize,
            ) -> cudaError_t,
        >,
        #[cfg(any(
            feature = "cuda-12030",
            feature = "cuda-12040",
            feature = "cuda-12050",
            feature = "cuda-12060",
            feature = "cuda-12080",
            feature = "cuda-12090"
        ))]
        pub cudaGraphNodeGetDependentNodes_v2: Option<
            unsafe extern "C" fn(
                node: cudaGraphNode_t,
                pDependentNodes: *mut cudaGraphNode_t,
                edgeData: *mut cudaGraphEdgeData,
                pNumDependentNodes: *mut usize,
            ) -> cudaError_t,
        >,
        #[cfg(any(
            feature = "cuda-11060",
            feature = "cuda-11070",
            feature = "cuda-11080",
            feature = "cuda-12000",
            feature = "cuda-12010",
            feature = "cuda-12020",
            feature = "cuda-12030",
            feature = "cuda-12040",
            feature = "cuda-12050",
            feature = "cuda-12060",
            feature = "cuda-12080",
            feature = "cuda-12090",
            feature = "cuda-13000"
        ))]
        pub cudaGraphNodeGetEnabled: Option<
            unsafe extern "C" fn(
                hGraphExec: cudaGraphExec_t,
                hNode: cudaGraphNode_t,
                isEnabled: *mut ::core::ffi::c_uint,
            ) -> cudaError_t,
        >,
        pub cudaGraphNodeGetType: Option<
            unsafe extern "C" fn(
                node: cudaGraphNode_t,
                pType: *mut cudaGraphNodeType,
            ) -> cudaError_t,
        >,
        #[cfg(any(
            feature = "cuda-11060",
            feature = "cuda-11070",
            feature = "cuda-11080",
            feature = "cuda-12000",
            feature = "cuda-12010",
            feature = "cuda-12020",
            feature = "cuda-12030",
            feature = "cuda-12040",
            feature = "cuda-12050",
            feature = "cuda-12060",
            feature = "cuda-12080",
            feature = "cuda-12090",
            feature = "cuda-13000"
        ))]
        pub cudaGraphNodeSetEnabled: Option<
            unsafe extern "C" fn(
                hGraphExec: cudaGraphExec_t,
                hNode: cudaGraphNode_t,
                isEnabled: ::core::ffi::c_uint,
            ) -> cudaError_t,
        >,
        #[cfg(any(
            feature = "cuda-12020",
            feature = "cuda-12030",
            feature = "cuda-12040",
            feature = "cuda-12050",
            feature = "cuda-12060",
            feature = "cuda-12080",
            feature = "cuda-12090",
            feature = "cuda-13000"
        ))]
        pub cudaGraphNodeSetParams: Option<
            unsafe extern "C" fn(
                node: cudaGraphNode_t,
                nodeParams: *mut cudaGraphNodeParams,
            ) -> cudaError_t,
        >,
        pub cudaGraphReleaseUserObject: Option<
            unsafe extern "C" fn(
                graph: cudaGraph_t,
                object: cudaUserObject_t,
                count: ::core::ffi::c_uint,
            ) -> cudaError_t,
        >,
        #[cfg(any(
            feature = "cuda-11040",
            feature = "cuda-11050",
            feature = "cuda-11060",
            feature = "cuda-11070",
            feature = "cuda-11080",
            feature = "cuda-12000",
            feature = "cuda-12010",
            feature = "cuda-12020",
            feature = "cuda-12030",
            feature = "cuda-12040",
            feature = "cuda-12050",
            feature = "cuda-12060",
            feature = "cuda-12080",
            feature = "cuda-12090"
        ))]
        pub cudaGraphRemoveDependencies: Option<
            unsafe extern "C" fn(
                graph: cudaGraph_t,
                from: *const cudaGraphNode_t,
                to: *const cudaGraphNode_t,
                numDependencies: usize,
            ) -> cudaError_t,
        >,
        #[cfg(any(feature = "cuda-13000"))]
        pub cudaGraphRemoveDependencies: Option<
            unsafe extern "C" fn(
                graph: cudaGraph_t,
                from: *const cudaGraphNode_t,
                to: *const cudaGraphNode_t,
                edgeData: *const cudaGraphEdgeData,
                numDependencies: usize,
            ) -> cudaError_t,
        >,
        #[cfg(any(
            feature = "cuda-12030",
            feature = "cuda-12040",
            feature = "cuda-12050",
            feature = "cuda-12060",
            feature = "cuda-12080",
            feature = "cuda-12090"
        ))]
        pub cudaGraphRemoveDependencies_v2: Option<
            unsafe extern "C" fn(
                graph: cudaGraph_t,
                from: *const cudaGraphNode_t,
                to: *const cudaGraphNode_t,
                edgeData: *const cudaGraphEdgeData,
                numDependencies: usize,
            ) -> cudaError_t,
        >,
        pub cudaGraphRetainUserObject: Option<
            unsafe extern "C" fn(
                graph: cudaGraph_t,
                object: cudaUserObject_t,
                count: ::core::ffi::c_uint,
                flags: ::core::ffi::c_uint,
            ) -> cudaError_t,
        >,
        pub cudaGraphUpload: Option<
            unsafe extern "C" fn(graphExec: cudaGraphExec_t, stream: cudaStream_t) -> cudaError_t,
        >,
        pub cudaGraphicsMapResources: Option<
            unsafe extern "C" fn(
                count: ::core::ffi::c_int,
                resources: *mut cudaGraphicsResource_t,
                stream: cudaStream_t,
            ) -> cudaError_t,
        >,
        pub cudaGraphicsResourceGetMappedMipmappedArray: Option<
            unsafe extern "C" fn(
                mipmappedArray: *mut cudaMipmappedArray_t,
                resource: cudaGraphicsResource_t,
            ) -> cudaError_t,
        >,
        pub cudaGraphicsResourceGetMappedPointer: Option<
            unsafe extern "C" fn(
                devPtr: *mut *mut ::core::ffi::c_void,
                size: *mut usize,
                resource: cudaGraphicsResource_t,
            ) -> cudaError_t,
        >,
        pub cudaGraphicsResourceSetMapFlags: Option<
            unsafe extern "C" fn(
                resource: cudaGraphicsResource_t,
                flags: ::core::ffi::c_uint,
            ) -> cudaError_t,
        >,
        pub cudaGraphicsSubResourceGetMappedArray: Option<
            unsafe extern "C" fn(
                array: *mut cudaArray_t,
                resource: cudaGraphicsResource_t,
                arrayIndex: ::core::ffi::c_uint,
                mipLevel: ::core::ffi::c_uint,
            ) -> cudaError_t,
        >,
        pub cudaGraphicsUnmapResources: Option<
            unsafe extern "C" fn(
                count: ::core::ffi::c_int,
                resources: *mut cudaGraphicsResource_t,
                stream: cudaStream_t,
            ) -> cudaError_t,
        >,
        pub cudaGraphicsUnregisterResource:
            Option<unsafe extern "C" fn(resource: cudaGraphicsResource_t) -> cudaError_t>,
        pub cudaHostAlloc: Option<
            unsafe extern "C" fn(
                pHost: *mut *mut ::core::ffi::c_void,
                size: usize,
                flags: ::core::ffi::c_uint,
            ) -> cudaError_t,
        >,
        pub cudaHostGetDevicePointer: Option<
            unsafe extern "C" fn(
                pDevice: *mut *mut ::core::ffi::c_void,
                pHost: *mut ::core::ffi::c_void,
                flags: ::core::ffi::c_uint,
            ) -> cudaError_t,
        >,
        pub cudaHostGetFlags: Option<
            unsafe extern "C" fn(
                pFlags: *mut ::core::ffi::c_uint,
                pHost: *mut ::core::ffi::c_void,
            ) -> cudaError_t,
        >,
        pub cudaHostRegister: Option<
            unsafe extern "C" fn(
                ptr: *mut ::core::ffi::c_void,
                size: usize,
                flags: ::core::ffi::c_uint,
            ) -> cudaError_t,
        >,
        pub cudaHostUnregister:
            Option<unsafe extern "C" fn(ptr: *mut ::core::ffi::c_void) -> cudaError_t>,
        pub cudaImportExternalMemory: Option<
            unsafe extern "C" fn(
                extMem_out: *mut cudaExternalMemory_t,
                memHandleDesc: *const cudaExternalMemoryHandleDesc,
            ) -> cudaError_t,
        >,
        pub cudaImportExternalSemaphore: Option<
            unsafe extern "C" fn(
                extSem_out: *mut cudaExternalSemaphore_t,
                semHandleDesc: *const cudaExternalSemaphoreHandleDesc,
            ) -> cudaError_t,
        >,
        #[cfg(any(
            feature = "cuda-12000",
            feature = "cuda-12010",
            feature = "cuda-12020",
            feature = "cuda-12030",
            feature = "cuda-12040",
            feature = "cuda-12050",
            feature = "cuda-12060",
            feature = "cuda-12080",
            feature = "cuda-12090",
            feature = "cuda-13000"
        ))]
        pub cudaInitDevice: Option<
            unsafe extern "C" fn(
                device: ::core::ffi::c_int,
                deviceFlags: ::core::ffi::c_uint,
                flags: ::core::ffi::c_uint,
            ) -> cudaError_t,
        >,
        pub cudaIpcCloseMemHandle:
            Option<unsafe extern "C" fn(devPtr: *mut ::core::ffi::c_void) -> cudaError_t>,
        pub cudaIpcGetEventHandle: Option<
            unsafe extern "C" fn(
                handle: *mut cudaIpcEventHandle_t,
                event: cudaEvent_t,
            ) -> cudaError_t,
        >,
        pub cudaIpcGetMemHandle: Option<
            unsafe extern "C" fn(
                handle: *mut cudaIpcMemHandle_t,
                devPtr: *mut ::core::ffi::c_void,
            ) -> cudaError_t,
        >,
        pub cudaIpcOpenEventHandle: Option<
            unsafe extern "C" fn(
                event: *mut cudaEvent_t,
                handle: cudaIpcEventHandle_t,
            ) -> cudaError_t,
        >,
        pub cudaIpcOpenMemHandle: Option<
            unsafe extern "C" fn(
                devPtr: *mut *mut ::core::ffi::c_void,
                handle: cudaIpcMemHandle_t,
                flags: ::core::ffi::c_uint,
            ) -> cudaError_t,
        >,
        #[cfg(any(feature = "cuda-12080", feature = "cuda-12090", feature = "cuda-13000"))]
        pub cudaKernelSetAttributeForDevice: Option<
            unsafe extern "C" fn(
                kernel: cudaKernel_t,
                attr: cudaFuncAttribute,
                value: ::core::ffi::c_int,
                device: ::core::ffi::c_int,
            ) -> cudaError_t,
        >,
        pub cudaLaunchCooperativeKernel: Option<
            unsafe extern "C" fn(
                func: *const ::core::ffi::c_void,
                gridDim: dim3,
                blockDim: dim3,
                args: *mut *mut ::core::ffi::c_void,
                sharedMem: usize,
                stream: cudaStream_t,
            ) -> cudaError_t,
        >,
        #[cfg(any(
            feature = "cuda-11040",
            feature = "cuda-11050",
            feature = "cuda-11060",
            feature = "cuda-11070",
            feature = "cuda-11080",
            feature = "cuda-12000",
            feature = "cuda-12010",
            feature = "cuda-12020",
            feature = "cuda-12030",
            feature = "cuda-12040",
            feature = "cuda-12050",
            feature = "cuda-12060",
            feature = "cuda-12080",
            feature = "cuda-12090"
        ))]
        pub cudaLaunchCooperativeKernelMultiDevice: Option<
            unsafe extern "C" fn(
                launchParamsList: *mut cudaLaunchParams,
                numDevices: ::core::ffi::c_uint,
                flags: ::core::ffi::c_uint,
            ) -> cudaError_t,
        >,
        pub cudaLaunchHostFunc: Option<
            unsafe extern "C" fn(
                stream: cudaStream_t,
                fn_: cudaHostFn_t,
                userData: *mut ::core::ffi::c_void,
            ) -> cudaError_t,
        >,
        pub cudaLaunchKernel: Option<
            unsafe extern "C" fn(
                func: *const ::core::ffi::c_void,
                gridDim: dim3,
                blockDim: dim3,
                args: *mut *mut ::core::ffi::c_void,
                sharedMem: usize,
                stream: cudaStream_t,
            ) -> cudaError_t,
        >,
        #[cfg(any(
            feature = "cuda-11080",
            feature = "cuda-12000",
            feature = "cuda-12010",
            feature = "cuda-12020",
            feature = "cuda-12030",
            feature = "cuda-12040",
            feature = "cuda-12050",
            feature = "cuda-12060",
            feature = "cuda-12080",
            feature = "cuda-12090",
            feature = "cuda-13000"
        ))]
        pub cudaLaunchKernelExC: Option<
            unsafe extern "C" fn(
                config: *const cudaLaunchConfig_t,
                func: *const ::core::ffi::c_void,
                args: *mut *mut ::core::ffi::c_void,
            ) -> cudaError_t,
        >,
        #[cfg(any(feature = "cuda-12080", feature = "cuda-12090", feature = "cuda-13000"))]
        pub cudaLibraryEnumerateKernels: Option<
            unsafe extern "C" fn(
                kernels: *mut cudaKernel_t,
                numKernels: ::core::ffi::c_uint,
                lib: cudaLibrary_t,
            ) -> cudaError_t,
        >,
        #[cfg(any(feature = "cuda-12080", feature = "cuda-12090", feature = "cuda-13000"))]
        pub cudaLibraryGetGlobal: Option<
            unsafe extern "C" fn(
                dptr: *mut *mut ::core::ffi::c_void,
                bytes: *mut usize,
                library: cudaLibrary_t,
                name: *const ::core::ffi::c_char,
            ) -> cudaError_t,
        >,
        #[cfg(any(feature = "cuda-12080", feature = "cuda-12090", feature = "cuda-13000"))]
        pub cudaLibraryGetKernel: Option<
            unsafe extern "C" fn(
                pKernel: *mut cudaKernel_t,
                library: cudaLibrary_t,
                name: *const ::core::ffi::c_char,
            ) -> cudaError_t,
        >,
        #[cfg(any(feature = "cuda-12080", feature = "cuda-12090", feature = "cuda-13000"))]
        pub cudaLibraryGetKernelCount: Option<
            unsafe extern "C" fn(
                count: *mut ::core::ffi::c_uint,
                lib: cudaLibrary_t,
            ) -> cudaError_t,
        >,
        #[cfg(any(feature = "cuda-12080", feature = "cuda-12090", feature = "cuda-13000"))]
        pub cudaLibraryGetManaged: Option<
            unsafe extern "C" fn(
                dptr: *mut *mut ::core::ffi::c_void,
                bytes: *mut usize,
                library: cudaLibrary_t,
                name: *const ::core::ffi::c_char,
            ) -> cudaError_t,
        >,
        #[cfg(any(feature = "cuda-12080", feature = "cuda-12090", feature = "cuda-13000"))]
        pub cudaLibraryGetUnifiedFunction: Option<
            unsafe extern "C" fn(
                fptr: *mut *mut ::core::ffi::c_void,
                library: cudaLibrary_t,
                symbol: *const ::core::ffi::c_char,
            ) -> cudaError_t,
        >,
        #[cfg(any(feature = "cuda-12080", feature = "cuda-12090", feature = "cuda-13000"))]
        pub cudaLibraryLoadData: Option<
            unsafe extern "C" fn(
                library: *mut cudaLibrary_t,
                code: *const ::core::ffi::c_void,
                jitOptions: *mut cudaJitOption,
                jitOptionsValues: *mut *mut ::core::ffi::c_void,
                numJitOptions: ::core::ffi::c_uint,
                libraryOptions: *mut cudaLibraryOption,
                libraryOptionValues: *mut *mut ::core::ffi::c_void,
                numLibraryOptions: ::core::ffi::c_uint,
            ) -> cudaError_t,
        >,
        #[cfg(any(feature = "cuda-12080", feature = "cuda-12090", feature = "cuda-13000"))]
        pub cudaLibraryLoadFromFile: Option<
            unsafe extern "C" fn(
                library: *mut cudaLibrary_t,
                fileName: *const ::core::ffi::c_char,
                jitOptions: *mut cudaJitOption,
                jitOptionsValues: *mut *mut ::core::ffi::c_void,
                numJitOptions: ::core::ffi::c_uint,
                libraryOptions: *mut cudaLibraryOption,
                libraryOptionValues: *mut *mut ::core::ffi::c_void,
                numLibraryOptions: ::core::ffi::c_uint,
            ) -> cudaError_t,
        >,
        #[cfg(any(feature = "cuda-12080", feature = "cuda-12090", feature = "cuda-13000"))]
        pub cudaLibraryUnload: Option<unsafe extern "C" fn(library: cudaLibrary_t) -> cudaError_t>,
        #[cfg(any(feature = "cuda-13000"))]
        pub cudaLogsCurrent: Option<
            unsafe extern "C" fn(
                iterator_out: *mut cudaLogIterator,
                flags: ::core::ffi::c_uint,
            ) -> cudaError_t,
        >,
        #[cfg(any(feature = "cuda-13000"))]
        pub cudaLogsDumpToFile: Option<
            unsafe extern "C" fn(
                iterator: *mut cudaLogIterator,
                pathToFile: *const ::core::ffi::c_char,
                flags: ::core::ffi::c_uint,
            ) -> cudaError_t,
        >,
        #[cfg(any(feature = "cuda-13000"))]
        pub cudaLogsDumpToMemory: Option<
            unsafe extern "C" fn(
                iterator: *mut cudaLogIterator,
                buffer: *mut ::core::ffi::c_char,
                size: *mut usize,
                flags: ::core::ffi::c_uint,
            ) -> cudaError_t,
        >,
        #[cfg(any(feature = "cuda-13000"))]
        pub cudaLogsRegisterCallback: Option<
            unsafe extern "C" fn(
                callbackFunc: cudaLogsCallback_t,
                userData: *mut ::core::ffi::c_void,
                callback_out: *mut cudaLogsCallbackHandle,
            ) -> cudaError_t,
        >,
        #[cfg(any(feature = "cuda-13000"))]
        pub cudaLogsUnregisterCallback:
            Option<unsafe extern "C" fn(callback: cudaLogsCallbackHandle) -> cudaError_t>,
        pub cudaMalloc: Option<
            unsafe extern "C" fn(devPtr: *mut *mut ::core::ffi::c_void, size: usize) -> cudaError_t,
        >,
        pub cudaMalloc3D: Option<
            unsafe extern "C" fn(
                pitchedDevPtr: *mut cudaPitchedPtr,
                extent: cudaExtent,
            ) -> cudaError_t,
        >,
        pub cudaMalloc3DArray: Option<
            unsafe extern "C" fn(
                array: *mut cudaArray_t,
                desc: *const cudaChannelFormatDesc,
                extent: cudaExtent,
                flags: ::core::ffi::c_uint,
            ) -> cudaError_t,
        >,
        pub cudaMallocArray: Option<
            unsafe extern "C" fn(
                array: *mut cudaArray_t,
                desc: *const cudaChannelFormatDesc,
                width: usize,
                height: usize,
                flags: ::core::ffi::c_uint,
            ) -> cudaError_t,
        >,
        pub cudaMallocAsync: Option<
            unsafe extern "C" fn(
                devPtr: *mut *mut ::core::ffi::c_void,
                size: usize,
                hStream: cudaStream_t,
            ) -> cudaError_t,
        >,
        pub cudaMallocFromPoolAsync: Option<
            unsafe extern "C" fn(
                ptr: *mut *mut ::core::ffi::c_void,
                size: usize,
                memPool: cudaMemPool_t,
                stream: cudaStream_t,
            ) -> cudaError_t,
        >,
        pub cudaMallocHost: Option<
            unsafe extern "C" fn(ptr: *mut *mut ::core::ffi::c_void, size: usize) -> cudaError_t,
        >,
        pub cudaMallocManaged: Option<
            unsafe extern "C" fn(
                devPtr: *mut *mut ::core::ffi::c_void,
                size: usize,
                flags: ::core::ffi::c_uint,
            ) -> cudaError_t,
        >,
        pub cudaMallocMipmappedArray: Option<
            unsafe extern "C" fn(
                mipmappedArray: *mut cudaMipmappedArray_t,
                desc: *const cudaChannelFormatDesc,
                extent: cudaExtent,
                numLevels: ::core::ffi::c_uint,
                flags: ::core::ffi::c_uint,
            ) -> cudaError_t,
        >,
        pub cudaMallocPitch: Option<
            unsafe extern "C" fn(
                devPtr: *mut *mut ::core::ffi::c_void,
                pitch: *mut usize,
                width: usize,
                height: usize,
            ) -> cudaError_t,
        >,
        #[cfg(any(
            feature = "cuda-11040",
            feature = "cuda-11050",
            feature = "cuda-11060",
            feature = "cuda-11070",
            feature = "cuda-11080",
            feature = "cuda-12000",
            feature = "cuda-12010",
            feature = "cuda-12020",
            feature = "cuda-12030",
            feature = "cuda-12040",
            feature = "cuda-12050",
            feature = "cuda-12060",
            feature = "cuda-12080",
            feature = "cuda-12090"
        ))]
        pub cudaMemAdvise: Option<
            unsafe extern "C" fn(
                devPtr: *const ::core::ffi::c_void,
                count: usize,
                advice: cudaMemoryAdvise,
                device: ::core::ffi::c_int,
            ) -> cudaError_t,
        >,
        #[cfg(any(feature = "cuda-13000"))]
        pub cudaMemAdvise: Option<
            unsafe extern "C" fn(
                devPtr: *const ::core::ffi::c_void,
                count: usize,
                advice: cudaMemoryAdvise,
                location: cudaMemLocation,
            ) -> cudaError_t,
        >,
        #[cfg(any(
            feature = "cuda-12020",
            feature = "cuda-12030",
            feature = "cuda-12040",
            feature = "cuda-12050",
            feature = "cuda-12060",
            feature = "cuda-12080",
            feature = "cuda-12090"
        ))]
        pub cudaMemAdvise_v2: Option<
            unsafe extern "C" fn(
                devPtr: *const ::core::ffi::c_void,
                count: usize,
                advice: cudaMemoryAdvise,
                location: cudaMemLocation,
            ) -> cudaError_t,
        >,
        #[cfg(any(feature = "cuda-13000"))]
        pub cudaMemDiscardAndPrefetchBatchAsync: Option<
            unsafe extern "C" fn(
                dptrs: *mut *mut ::core::ffi::c_void,
                sizes: *mut usize,
                count: usize,
                prefetchLocs: *mut cudaMemLocation,
                prefetchLocIdxs: *mut usize,
                numPrefetchLocs: usize,
                flags: ::core::ffi::c_ulonglong,
                stream: cudaStream_t,
            ) -> cudaError_t,
        >,
        #[cfg(any(feature = "cuda-13000"))]
        pub cudaMemDiscardBatchAsync: Option<
            unsafe extern "C" fn(
                dptrs: *mut *mut ::core::ffi::c_void,
                sizes: *mut usize,
                count: usize,
                flags: ::core::ffi::c_ulonglong,
                stream: cudaStream_t,
            ) -> cudaError_t,
        >,
        #[cfg(any(feature = "cuda-13000"))]
        pub cudaMemGetDefaultMemPool: Option<
            unsafe extern "C" fn(
                memPool: *mut cudaMemPool_t,
                location: *mut cudaMemLocation,
                type_: cudaMemAllocationType,
            ) -> cudaError_t,
        >,
        pub cudaMemGetInfo:
            Option<unsafe extern "C" fn(free: *mut usize, total: *mut usize) -> cudaError_t>,
        #[cfg(any(feature = "cuda-13000"))]
        pub cudaMemGetMemPool: Option<
            unsafe extern "C" fn(
                memPool: *mut cudaMemPool_t,
                location: *mut cudaMemLocation,
                type_: cudaMemAllocationType,
            ) -> cudaError_t,
        >,
        pub cudaMemPoolCreate: Option<
            unsafe extern "C" fn(
                memPool: *mut cudaMemPool_t,
                poolProps: *const cudaMemPoolProps,
            ) -> cudaError_t,
        >,
        pub cudaMemPoolDestroy: Option<unsafe extern "C" fn(memPool: cudaMemPool_t) -> cudaError_t>,
        pub cudaMemPoolExportPointer: Option<
            unsafe extern "C" fn(
                exportData: *mut cudaMemPoolPtrExportData,
                ptr: *mut ::core::ffi::c_void,
            ) -> cudaError_t,
        >,
        pub cudaMemPoolExportToShareableHandle: Option<
            unsafe extern "C" fn(
                shareableHandle: *mut ::core::ffi::c_void,
                memPool: cudaMemPool_t,
                handleType: cudaMemAllocationHandleType,
                flags: ::core::ffi::c_uint,
            ) -> cudaError_t,
        >,
        pub cudaMemPoolGetAccess: Option<
            unsafe extern "C" fn(
                flags: *mut cudaMemAccessFlags,
                memPool: cudaMemPool_t,
                location: *mut cudaMemLocation,
            ) -> cudaError_t,
        >,
        pub cudaMemPoolGetAttribute: Option<
            unsafe extern "C" fn(
                memPool: cudaMemPool_t,
                attr: cudaMemPoolAttr,
                value: *mut ::core::ffi::c_void,
            ) -> cudaError_t,
        >,
        pub cudaMemPoolImportFromShareableHandle: Option<
            unsafe extern "C" fn(
                memPool: *mut cudaMemPool_t,
                shareableHandle: *mut ::core::ffi::c_void,
                handleType: cudaMemAllocationHandleType,
                flags: ::core::ffi::c_uint,
            ) -> cudaError_t,
        >,
        pub cudaMemPoolImportPointer: Option<
            unsafe extern "C" fn(
                ptr: *mut *mut ::core::ffi::c_void,
                memPool: cudaMemPool_t,
                exportData: *mut cudaMemPoolPtrExportData,
            ) -> cudaError_t,
        >,
        pub cudaMemPoolSetAccess: Option<
            unsafe extern "C" fn(
                memPool: cudaMemPool_t,
                descList: *const cudaMemAccessDesc,
                count: usize,
            ) -> cudaError_t,
        >,
        pub cudaMemPoolSetAttribute: Option<
            unsafe extern "C" fn(
                memPool: cudaMemPool_t,
                attr: cudaMemPoolAttr,
                value: *mut ::core::ffi::c_void,
            ) -> cudaError_t,
        >,
        pub cudaMemPoolTrimTo: Option<
            unsafe extern "C" fn(memPool: cudaMemPool_t, minBytesToKeep: usize) -> cudaError_t,
        >,
        #[cfg(any(
            feature = "cuda-11040",
            feature = "cuda-11050",
            feature = "cuda-11060",
            feature = "cuda-11070",
            feature = "cuda-11080",
            feature = "cuda-12000",
            feature = "cuda-12010",
            feature = "cuda-12020",
            feature = "cuda-12030",
            feature = "cuda-12040",
            feature = "cuda-12050",
            feature = "cuda-12060",
            feature = "cuda-12080",
            feature = "cuda-12090"
        ))]
        pub cudaMemPrefetchAsync: Option<
            unsafe extern "C" fn(
                devPtr: *const ::core::ffi::c_void,
                count: usize,
                dstDevice: ::core::ffi::c_int,
                stream: cudaStream_t,
            ) -> cudaError_t,
        >,
        #[cfg(any(feature = "cuda-13000"))]
        pub cudaMemPrefetchAsync: Option<
            unsafe extern "C" fn(
                devPtr: *const ::core::ffi::c_void,
                count: usize,
                location: cudaMemLocation,
                flags: ::core::ffi::c_uint,
                stream: cudaStream_t,
            ) -> cudaError_t,
        >,
        #[cfg(any(
            feature = "cuda-12020",
            feature = "cuda-12030",
            feature = "cuda-12040",
            feature = "cuda-12050",
            feature = "cuda-12060",
            feature = "cuda-12080",
            feature = "cuda-12090"
        ))]
        pub cudaMemPrefetchAsync_v2: Option<
            unsafe extern "C" fn(
                devPtr: *const ::core::ffi::c_void,
                count: usize,
                location: cudaMemLocation,
                flags: ::core::ffi::c_uint,
                stream: cudaStream_t,
            ) -> cudaError_t,
        >,
        #[cfg(any(feature = "cuda-13000"))]
        pub cudaMemPrefetchBatchAsync: Option<
            unsafe extern "C" fn(
                dptrs: *mut *mut ::core::ffi::c_void,
                sizes: *mut usize,
                count: usize,
                prefetchLocs: *mut cudaMemLocation,
                prefetchLocIdxs: *mut usize,
                numPrefetchLocs: usize,
                flags: ::core::ffi::c_ulonglong,
                stream: cudaStream_t,
            ) -> cudaError_t,
        >,
        pub cudaMemRangeGetAttribute: Option<
            unsafe extern "C" fn(
                data: *mut ::core::ffi::c_void,
                dataSize: usize,
                attribute: cudaMemRangeAttribute,
                devPtr: *const ::core::ffi::c_void,
                count: usize,
            ) -> cudaError_t,
        >,
        pub cudaMemRangeGetAttributes: Option<
            unsafe extern "C" fn(
                data: *mut *mut ::core::ffi::c_void,
                dataSizes: *mut usize,
                attributes: *mut cudaMemRangeAttribute,
                numAttributes: usize,
                devPtr: *const ::core::ffi::c_void,
                count: usize,
            ) -> cudaError_t,
        >,
        #[cfg(any(feature = "cuda-13000"))]
        pub cudaMemSetMemPool: Option<
            unsafe extern "C" fn(
                location: *mut cudaMemLocation,
                type_: cudaMemAllocationType,
                memPool: cudaMemPool_t,
            ) -> cudaError_t,
        >,
        pub cudaMemcpy: Option<
            unsafe extern "C" fn(
                dst: *mut ::core::ffi::c_void,
                src: *const ::core::ffi::c_void,
                count: usize,
                kind: cudaMemcpyKind,
            ) -> cudaError_t,
        >,
        pub cudaMemcpy2D: Option<
            unsafe extern "C" fn(
                dst: *mut ::core::ffi::c_void,
                dpitch: usize,
                src: *const ::core::ffi::c_void,
                spitch: usize,
                width: usize,
                height: usize,
                kind: cudaMemcpyKind,
            ) -> cudaError_t,
        >,
        pub cudaMemcpy2DArrayToArray: Option<
            unsafe extern "C" fn(
                dst: cudaArray_t,
                wOffsetDst: usize,
                hOffsetDst: usize,
                src: cudaArray_const_t,
                wOffsetSrc: usize,
                hOffsetSrc: usize,
                width: usize,
                height: usize,
                kind: cudaMemcpyKind,
            ) -> cudaError_t,
        >,
        pub cudaMemcpy2DAsync: Option<
            unsafe extern "C" fn(
                dst: *mut ::core::ffi::c_void,
                dpitch: usize,
                src: *const ::core::ffi::c_void,
                spitch: usize,
                width: usize,
                height: usize,
                kind: cudaMemcpyKind,
                stream: cudaStream_t,
            ) -> cudaError_t,
        >,
        pub cudaMemcpy2DFromArray: Option<
            unsafe extern "C" fn(
                dst: *mut ::core::ffi::c_void,
                dpitch: usize,
                src: cudaArray_const_t,
                wOffset: usize,
                hOffset: usize,
                width: usize,
                height: usize,
                kind: cudaMemcpyKind,
            ) -> cudaError_t,
        >,
        pub cudaMemcpy2DFromArrayAsync: Option<
            unsafe extern "C" fn(
                dst: *mut ::core::ffi::c_void,
                dpitch: usize,
                src: cudaArray_const_t,
                wOffset: usize,
                hOffset: usize,
                width: usize,
                height: usize,
                kind: cudaMemcpyKind,
                stream: cudaStream_t,
            ) -> cudaError_t,
        >,
        pub cudaMemcpy2DToArray: Option<
            unsafe extern "C" fn(
                dst: cudaArray_t,
                wOffset: usize,
                hOffset: usize,
                src: *const ::core::ffi::c_void,
                spitch: usize,
                width: usize,
                height: usize,
                kind: cudaMemcpyKind,
            ) -> cudaError_t,
        >,
        pub cudaMemcpy2DToArrayAsync: Option<
            unsafe extern "C" fn(
                dst: cudaArray_t,
                wOffset: usize,
                hOffset: usize,
                src: *const ::core::ffi::c_void,
                spitch: usize,
                width: usize,
                height: usize,
                kind: cudaMemcpyKind,
                stream: cudaStream_t,
            ) -> cudaError_t,
        >,
        pub cudaMemcpy3D: Option<unsafe extern "C" fn(p: *const cudaMemcpy3DParms) -> cudaError_t>,
        pub cudaMemcpy3DAsync: Option<
            unsafe extern "C" fn(p: *const cudaMemcpy3DParms, stream: cudaStream_t) -> cudaError_t,
        >,
        #[cfg(any(feature = "cuda-12080", feature = "cuda-12090"))]
        pub cudaMemcpy3DBatchAsync: Option<
            unsafe extern "C" fn(
                numOps: usize,
                opList: *mut cudaMemcpy3DBatchOp,
                failIdx: *mut usize,
                flags: ::core::ffi::c_ulonglong,
                stream: cudaStream_t,
            ) -> cudaError_t,
        >,
        #[cfg(any(feature = "cuda-13000"))]
        pub cudaMemcpy3DBatchAsync: Option<
            unsafe extern "C" fn(
                numOps: usize,
                opList: *mut cudaMemcpy3DBatchOp,
                flags: ::core::ffi::c_ulonglong,
                stream: cudaStream_t,
            ) -> cudaError_t,
        >,
        pub cudaMemcpy3DPeer:
            Option<unsafe extern "C" fn(p: *const cudaMemcpy3DPeerParms) -> cudaError_t>,
        pub cudaMemcpy3DPeerAsync: Option<
            unsafe extern "C" fn(
                p: *const cudaMemcpy3DPeerParms,
                stream: cudaStream_t,
            ) -> cudaError_t,
        >,
        pub cudaMemcpyArrayToArray: Option<
            unsafe extern "C" fn(
                dst: cudaArray_t,
                wOffsetDst: usize,
                hOffsetDst: usize,
                src: cudaArray_const_t,
                wOffsetSrc: usize,
                hOffsetSrc: usize,
                count: usize,
                kind: cudaMemcpyKind,
            ) -> cudaError_t,
        >,
        pub cudaMemcpyAsync: Option<
            unsafe extern "C" fn(
                dst: *mut ::core::ffi::c_void,
                src: *const ::core::ffi::c_void,
                count: usize,
                kind: cudaMemcpyKind,
                stream: cudaStream_t,
            ) -> cudaError_t,
        >,
        #[cfg(any(feature = "cuda-12080", feature = "cuda-12090"))]
        pub cudaMemcpyBatchAsync: Option<
            unsafe extern "C" fn(
                dsts: *mut *mut ::core::ffi::c_void,
                srcs: *mut *mut ::core::ffi::c_void,
                sizes: *mut usize,
                count: usize,
                attrs: *mut cudaMemcpyAttributes,
                attrsIdxs: *mut usize,
                numAttrs: usize,
                failIdx: *mut usize,
                stream: cudaStream_t,
            ) -> cudaError_t,
        >,
        #[cfg(any(feature = "cuda-13000"))]
        pub cudaMemcpyBatchAsync: Option<
            unsafe extern "C" fn(
                dsts: *const *mut ::core::ffi::c_void,
                srcs: *const *const ::core::ffi::c_void,
                sizes: *const usize,
                count: usize,
                attrs: *mut cudaMemcpyAttributes,
                attrsIdxs: *mut usize,
                numAttrs: usize,
                stream: cudaStream_t,
            ) -> cudaError_t,
        >,
        pub cudaMemcpyFromArray: Option<
            unsafe extern "C" fn(
                dst: *mut ::core::ffi::c_void,
                src: cudaArray_const_t,
                wOffset: usize,
                hOffset: usize,
                count: usize,
                kind: cudaMemcpyKind,
            ) -> cudaError_t,
        >,
        pub cudaMemcpyFromArrayAsync: Option<
            unsafe extern "C" fn(
                dst: *mut ::core::ffi::c_void,
                src: cudaArray_const_t,
                wOffset: usize,
                hOffset: usize,
                count: usize,
                kind: cudaMemcpyKind,
                stream: cudaStream_t,
            ) -> cudaError_t,
        >,
        pub cudaMemcpyFromSymbol: Option<
            unsafe extern "C" fn(
                dst: *mut ::core::ffi::c_void,
                symbol: *const ::core::ffi::c_void,
                count: usize,
                offset: usize,
                kind: cudaMemcpyKind,
            ) -> cudaError_t,
        >,
        pub cudaMemcpyFromSymbolAsync: Option<
            unsafe extern "C" fn(
                dst: *mut ::core::ffi::c_void,
                symbol: *const ::core::ffi::c_void,
                count: usize,
                offset: usize,
                kind: cudaMemcpyKind,
                stream: cudaStream_t,
            ) -> cudaError_t,
        >,
        pub cudaMemcpyPeer: Option<
            unsafe extern "C" fn(
                dst: *mut ::core::ffi::c_void,
                dstDevice: ::core::ffi::c_int,
                src: *const ::core::ffi::c_void,
                srcDevice: ::core::ffi::c_int,
                count: usize,
            ) -> cudaError_t,
        >,
        pub cudaMemcpyPeerAsync: Option<
            unsafe extern "C" fn(
                dst: *mut ::core::ffi::c_void,
                dstDevice: ::core::ffi::c_int,
                src: *const ::core::ffi::c_void,
                srcDevice: ::core::ffi::c_int,
                count: usize,
                stream: cudaStream_t,
            ) -> cudaError_t,
        >,
        pub cudaMemcpyToArray: Option<
            unsafe extern "C" fn(
                dst: cudaArray_t,
                wOffset: usize,
                hOffset: usize,
                src: *const ::core::ffi::c_void,
                count: usize,
                kind: cudaMemcpyKind,
            ) -> cudaError_t,
        >,
        pub cudaMemcpyToArrayAsync: Option<
            unsafe extern "C" fn(
                dst: cudaArray_t,
                wOffset: usize,
                hOffset: usize,
                src: *const ::core::ffi::c_void,
                count: usize,
                kind: cudaMemcpyKind,
                stream: cudaStream_t,
            ) -> cudaError_t,
        >,
        pub cudaMemcpyToSymbol: Option<
            unsafe extern "C" fn(
                symbol: *const ::core::ffi::c_void,
                src: *const ::core::ffi::c_void,
                count: usize,
                offset: usize,
                kind: cudaMemcpyKind,
            ) -> cudaError_t,
        >,
        pub cudaMemcpyToSymbolAsync: Option<
            unsafe extern "C" fn(
                symbol: *const ::core::ffi::c_void,
                src: *const ::core::ffi::c_void,
                count: usize,
                offset: usize,
                kind: cudaMemcpyKind,
                stream: cudaStream_t,
            ) -> cudaError_t,
        >,
        pub cudaMemset: Option<
            unsafe extern "C" fn(
                devPtr: *mut ::core::ffi::c_void,
                value: ::core::ffi::c_int,
                count: usize,
            ) -> cudaError_t,
        >,
        pub cudaMemset2D: Option<
            unsafe extern "C" fn(
                devPtr: *mut ::core::ffi::c_void,
                pitch: usize,
                value: ::core::ffi::c_int,
                width: usize,
                height: usize,
            ) -> cudaError_t,
        >,
        pub cudaMemset2DAsync: Option<
            unsafe extern "C" fn(
                devPtr: *mut ::core::ffi::c_void,
                pitch: usize,
                value: ::core::ffi::c_int,
                width: usize,
                height: usize,
                stream: cudaStream_t,
            ) -> cudaError_t,
        >,
        pub cudaMemset3D: Option<
            unsafe extern "C" fn(
                pitchedDevPtr: cudaPitchedPtr,
                value: ::core::ffi::c_int,
                extent: cudaExtent,
            ) -> cudaError_t,
        >,
        pub cudaMemset3DAsync: Option<
            unsafe extern "C" fn(
                pitchedDevPtr: cudaPitchedPtr,
                value: ::core::ffi::c_int,
                extent: cudaExtent,
                stream: cudaStream_t,
            ) -> cudaError_t,
        >,
        pub cudaMemsetAsync: Option<
            unsafe extern "C" fn(
                devPtr: *mut ::core::ffi::c_void,
                value: ::core::ffi::c_int,
                count: usize,
                stream: cudaStream_t,
            ) -> cudaError_t,
        >,
        #[cfg(any(
            feature = "cuda-11060",
            feature = "cuda-11070",
            feature = "cuda-11080",
            feature = "cuda-12000",
            feature = "cuda-12010",
            feature = "cuda-12020",
            feature = "cuda-12030",
            feature = "cuda-12040",
            feature = "cuda-12050",
            feature = "cuda-12060",
            feature = "cuda-12080",
            feature = "cuda-12090",
            feature = "cuda-13000"
        ))]
        pub cudaMipmappedArrayGetMemoryRequirements: Option<
            unsafe extern "C" fn(
                memoryRequirements: *mut cudaArrayMemoryRequirements,
                mipmap: cudaMipmappedArray_t,
                device: ::core::ffi::c_int,
            ) -> cudaError_t,
        >,
        pub cudaMipmappedArrayGetSparseProperties: Option<
            unsafe extern "C" fn(
                sparseProperties: *mut cudaArraySparseProperties,
                mipmap: cudaMipmappedArray_t,
            ) -> cudaError_t,
        >,
        pub cudaOccupancyAvailableDynamicSMemPerBlock: Option<
            unsafe extern "C" fn(
                dynamicSmemSize: *mut usize,
                func: *const ::core::ffi::c_void,
                numBlocks: ::core::ffi::c_int,
                blockSize: ::core::ffi::c_int,
            ) -> cudaError_t,
        >,
        pub cudaOccupancyMaxActiveBlocksPerMultiprocessor: Option<
            unsafe extern "C" fn(
                numBlocks: *mut ::core::ffi::c_int,
                func: *const ::core::ffi::c_void,
                blockSize: ::core::ffi::c_int,
                dynamicSMemSize: usize,
            ) -> cudaError_t,
        >,
        pub cudaOccupancyMaxActiveBlocksPerMultiprocessorWithFlags: Option<
            unsafe extern "C" fn(
                numBlocks: *mut ::core::ffi::c_int,
                func: *const ::core::ffi::c_void,
                blockSize: ::core::ffi::c_int,
                dynamicSMemSize: usize,
                flags: ::core::ffi::c_uint,
            ) -> cudaError_t,
        >,
        #[cfg(any(
            feature = "cuda-11080",
            feature = "cuda-12000",
            feature = "cuda-12010",
            feature = "cuda-12020",
            feature = "cuda-12030",
            feature = "cuda-12040",
            feature = "cuda-12050",
            feature = "cuda-12060",
            feature = "cuda-12080",
            feature = "cuda-12090",
            feature = "cuda-13000"
        ))]
        pub cudaOccupancyMaxActiveClusters: Option<
            unsafe extern "C" fn(
                numClusters: *mut ::core::ffi::c_int,
                func: *const ::core::ffi::c_void,
                launchConfig: *const cudaLaunchConfig_t,
            ) -> cudaError_t,
        >,
        #[cfg(any(
            feature = "cuda-11080",
            feature = "cuda-12000",
            feature = "cuda-12010",
            feature = "cuda-12020",
            feature = "cuda-12030",
            feature = "cuda-12040",
            feature = "cuda-12050",
            feature = "cuda-12060",
            feature = "cuda-12080",
            feature = "cuda-12090",
            feature = "cuda-13000"
        ))]
        pub cudaOccupancyMaxPotentialClusterSize: Option<
            unsafe extern "C" fn(
                clusterSize: *mut ::core::ffi::c_int,
                func: *const ::core::ffi::c_void,
                launchConfig: *const cudaLaunchConfig_t,
            ) -> cudaError_t,
        >,
        pub cudaPeekAtLastError: Option<unsafe extern "C" fn() -> cudaError_t>,
        pub cudaPointerGetAttributes: Option<
            unsafe extern "C" fn(
                attributes: *mut cudaPointerAttributes,
                ptr: *const ::core::ffi::c_void,
            ) -> cudaError_t,
        >,
        pub cudaProfilerStop: Option<unsafe extern "C" fn() -> cudaError_t>,
        pub cudaRuntimeGetVersion:
            Option<unsafe extern "C" fn(runtimeVersion: *mut ::core::ffi::c_int) -> cudaError_t>,
        pub cudaSetDevice: Option<unsafe extern "C" fn(device: ::core::ffi::c_int) -> cudaError_t>,
        pub cudaSetDeviceFlags:
            Option<unsafe extern "C" fn(flags: ::core::ffi::c_uint) -> cudaError_t>,
        #[cfg(any(
            feature = "cuda-11040",
            feature = "cuda-11050",
            feature = "cuda-11060",
            feature = "cuda-11070",
            feature = "cuda-11080",
            feature = "cuda-12000",
            feature = "cuda-12010",
            feature = "cuda-12020",
            feature = "cuda-12030",
            feature = "cuda-12040",
            feature = "cuda-12050",
            feature = "cuda-12060",
            feature = "cuda-12080",
            feature = "cuda-12090"
        ))]
        pub cudaSetDoubleForDevice: Option<unsafe extern "C" fn(d: *mut f64) -> cudaError_t>,
        #[cfg(any(
            feature = "cuda-11040",
            feature = "cuda-11050",
            feature = "cuda-11060",
            feature = "cuda-11070",
            feature = "cuda-11080",
            feature = "cuda-12000",
            feature = "cuda-12010",
            feature = "cuda-12020",
            feature = "cuda-12030",
            feature = "cuda-12040",
            feature = "cuda-12050",
            feature = "cuda-12060",
            feature = "cuda-12080",
            feature = "cuda-12090"
        ))]
        pub cudaSetDoubleForHost: Option<unsafe extern "C" fn(d: *mut f64) -> cudaError_t>,
        pub cudaSetValidDevices: Option<
            unsafe extern "C" fn(
                device_arr: *mut ::core::ffi::c_int,
                len: ::core::ffi::c_int,
            ) -> cudaError_t,
        >,
        #[cfg(any(feature = "cuda-13000"))]
        pub cudaSignalExternalSemaphoresAsync: Option<
            unsafe extern "C" fn(
                extSemArray: *const cudaExternalSemaphore_t,
                paramsArray: *const cudaExternalSemaphoreSignalParams,
                numExtSems: ::core::ffi::c_uint,
                stream: cudaStream_t,
            ) -> cudaError_t,
        >,
        #[cfg(any(
            feature = "cuda-11040",
            feature = "cuda-11050",
            feature = "cuda-11060",
            feature = "cuda-11070",
            feature = "cuda-11080",
            feature = "cuda-12000",
            feature = "cuda-12010",
            feature = "cuda-12020",
            feature = "cuda-12030",
            feature = "cuda-12040",
            feature = "cuda-12050",
            feature = "cuda-12060",
            feature = "cuda-12080",
            feature = "cuda-12090"
        ))]
        pub cudaSignalExternalSemaphoresAsync_v2: Option<
            unsafe extern "C" fn(
                extSemArray: *const cudaExternalSemaphore_t,
                paramsArray: *const cudaExternalSemaphoreSignalParams,
                numExtSems: ::core::ffi::c_uint,
                stream: cudaStream_t,
            ) -> cudaError_t,
        >,
        pub cudaStreamAddCallback: Option<
            unsafe extern "C" fn(
                stream: cudaStream_t,
                callback: cudaStreamCallback_t,
                userData: *mut ::core::ffi::c_void,
                flags: ::core::ffi::c_uint,
            ) -> cudaError_t,
        >,
        pub cudaStreamAttachMemAsync: Option<
            unsafe extern "C" fn(
                stream: cudaStream_t,
                devPtr: *mut ::core::ffi::c_void,
                length: usize,
                flags: ::core::ffi::c_uint,
            ) -> cudaError_t,
        >,
        pub cudaStreamBeginCapture: Option<
            unsafe extern "C" fn(stream: cudaStream_t, mode: cudaStreamCaptureMode) -> cudaError_t,
        >,
        #[cfg(any(
            feature = "cuda-12030",
            feature = "cuda-12040",
            feature = "cuda-12050",
            feature = "cuda-12060",
            feature = "cuda-12080",
            feature = "cuda-12090",
            feature = "cuda-13000"
        ))]
        pub cudaStreamBeginCaptureToGraph: Option<
            unsafe extern "C" fn(
                stream: cudaStream_t,
                graph: cudaGraph_t,
                dependencies: *const cudaGraphNode_t,
                dependencyData: *const cudaGraphEdgeData,
                numDependencies: usize,
                mode: cudaStreamCaptureMode,
            ) -> cudaError_t,
        >,
        pub cudaStreamCopyAttributes:
            Option<unsafe extern "C" fn(dst: cudaStream_t, src: cudaStream_t) -> cudaError_t>,
        pub cudaStreamCreate:
            Option<unsafe extern "C" fn(pStream: *mut cudaStream_t) -> cudaError_t>,
        pub cudaStreamCreateWithFlags: Option<
            unsafe extern "C" fn(
                pStream: *mut cudaStream_t,
                flags: ::core::ffi::c_uint,
            ) -> cudaError_t,
        >,
        pub cudaStreamCreateWithPriority: Option<
            unsafe extern "C" fn(
                pStream: *mut cudaStream_t,
                flags: ::core::ffi::c_uint,
                priority: ::core::ffi::c_int,
            ) -> cudaError_t,
        >,
        pub cudaStreamDestroy: Option<unsafe extern "C" fn(stream: cudaStream_t) -> cudaError_t>,
        pub cudaStreamEndCapture: Option<
            unsafe extern "C" fn(stream: cudaStream_t, pGraph: *mut cudaGraph_t) -> cudaError_t,
        >,
        #[cfg(any(
            feature = "cuda-11040",
            feature = "cuda-11050",
            feature = "cuda-11060",
            feature = "cuda-11070"
        ))]
        pub cudaStreamGetAttribute: Option<
            unsafe extern "C" fn(
                hStream: cudaStream_t,
                attr: cudaStreamAttrID,
                value_out: *mut cudaStreamAttrValue,
            ) -> cudaError_t,
        >,
        #[cfg(any(
            feature = "cuda-11080",
            feature = "cuda-12000",
            feature = "cuda-12010",
            feature = "cuda-12020",
            feature = "cuda-12030",
            feature = "cuda-12040",
            feature = "cuda-12050",
            feature = "cuda-12060",
            feature = "cuda-12080",
            feature = "cuda-12090",
            feature = "cuda-13000"
        ))]
        pub cudaStreamGetAttribute: Option<
            unsafe extern "C" fn(
                hStream: cudaStream_t,
                attr: cudaLaunchAttributeID,
                value_out: *mut cudaLaunchAttributeValue,
            ) -> cudaError_t,
        >,
        #[cfg(any(
            feature = "cuda-11040",
            feature = "cuda-11050",
            feature = "cuda-11060",
            feature = "cuda-11070",
            feature = "cuda-11080"
        ))]
        pub cudaStreamGetCaptureInfo: Option<
            unsafe extern "C" fn(
                stream: cudaStream_t,
                pCaptureStatus: *mut cudaStreamCaptureStatus,
                pId: *mut ::core::ffi::c_ulonglong,
            ) -> cudaError_t,
        >,
        #[cfg(any(feature = "cuda-13000"))]
        pub cudaStreamGetCaptureInfo: Option<
            unsafe extern "C" fn(
                stream: cudaStream_t,
                captureStatus_out: *mut cudaStreamCaptureStatus,
                id_out: *mut ::core::ffi::c_ulonglong,
                graph_out: *mut cudaGraph_t,
                dependencies_out: *mut *const cudaGraphNode_t,
                edgeData_out: *mut *const cudaGraphEdgeData,
                numDependencies_out: *mut usize,
            ) -> cudaError_t,
        >,
        #[cfg(any(
            feature = "cuda-11040",
            feature = "cuda-11050",
            feature = "cuda-11060",
            feature = "cuda-11070",
            feature = "cuda-11080",
            feature = "cuda-12000",
            feature = "cuda-12010",
            feature = "cuda-12020",
            feature = "cuda-12030",
            feature = "cuda-12040",
            feature = "cuda-12050",
            feature = "cuda-12060",
            feature = "cuda-12080",
            feature = "cuda-12090"
        ))]
        pub cudaStreamGetCaptureInfo_v2: Option<
            unsafe extern "C" fn(
                stream: cudaStream_t,
                captureStatus_out: *mut cudaStreamCaptureStatus,
                id_out: *mut ::core::ffi::c_ulonglong,
                graph_out: *mut cudaGraph_t,
                dependencies_out: *mut *const cudaGraphNode_t,
                numDependencies_out: *mut usize,
            ) -> cudaError_t,
        >,
        #[cfg(any(
            feature = "cuda-12030",
            feature = "cuda-12040",
            feature = "cuda-12050",
            feature = "cuda-12060",
            feature = "cuda-12080",
            feature = "cuda-12090"
        ))]
        pub cudaStreamGetCaptureInfo_v3: Option<
            unsafe extern "C" fn(
                stream: cudaStream_t,
                captureStatus_out: *mut cudaStreamCaptureStatus,
                id_out: *mut ::core::ffi::c_ulonglong,
                graph_out: *mut cudaGraph_t,
                dependencies_out: *mut *const cudaGraphNode_t,
                edgeData_out: *mut *const cudaGraphEdgeData,
                numDependencies_out: *mut usize,
            ) -> cudaError_t,
        >,
        #[cfg(any(feature = "cuda-12080", feature = "cuda-12090", feature = "cuda-13000"))]
        pub cudaStreamGetDevice: Option<
            unsafe extern "C" fn(
                hStream: cudaStream_t,
                device: *mut ::core::ffi::c_int,
            ) -> cudaError_t,
        >,
        pub cudaStreamGetFlags: Option<
            unsafe extern "C" fn(
                hStream: cudaStream_t,
                flags: *mut ::core::ffi::c_uint,
            ) -> cudaError_t,
        >,
        #[cfg(any(
            feature = "cuda-12000",
            feature = "cuda-12010",
            feature = "cuda-12020",
            feature = "cuda-12030",
            feature = "cuda-12040",
            feature = "cuda-12050",
            feature = "cuda-12060",
            feature = "cuda-12080",
            feature = "cuda-12090",
            feature = "cuda-13000"
        ))]
        pub cudaStreamGetId: Option<
            unsafe extern "C" fn(
                hStream: cudaStream_t,
                streamId: *mut ::core::ffi::c_ulonglong,
            ) -> cudaError_t,
        >,
        pub cudaStreamGetPriority: Option<
            unsafe extern "C" fn(
                hStream: cudaStream_t,
                priority: *mut ::core::ffi::c_int,
            ) -> cudaError_t,
        >,
        pub cudaStreamIsCapturing: Option<
            unsafe extern "C" fn(
                stream: cudaStream_t,
                pCaptureStatus: *mut cudaStreamCaptureStatus,
            ) -> cudaError_t,
        >,
        pub cudaStreamQuery: Option<unsafe extern "C" fn(stream: cudaStream_t) -> cudaError_t>,
        #[cfg(any(
            feature = "cuda-11040",
            feature = "cuda-11050",
            feature = "cuda-11060",
            feature = "cuda-11070"
        ))]
        pub cudaStreamSetAttribute: Option<
            unsafe extern "C" fn(
                hStream: cudaStream_t,
                attr: cudaStreamAttrID,
                value: *const cudaStreamAttrValue,
            ) -> cudaError_t,
        >,
        #[cfg(any(
            feature = "cuda-11080",
            feature = "cuda-12000",
            feature = "cuda-12010",
            feature = "cuda-12020",
            feature = "cuda-12030",
            feature = "cuda-12040",
            feature = "cuda-12050",
            feature = "cuda-12060",
            feature = "cuda-12080",
            feature = "cuda-12090",
            feature = "cuda-13000"
        ))]
        pub cudaStreamSetAttribute: Option<
            unsafe extern "C" fn(
                hStream: cudaStream_t,
                attr: cudaLaunchAttributeID,
                value: *const cudaLaunchAttributeValue,
            ) -> cudaError_t,
        >,
        pub cudaStreamSynchronize:
            Option<unsafe extern "C" fn(stream: cudaStream_t) -> cudaError_t>,
        #[cfg(any(
            feature = "cuda-11040",
            feature = "cuda-11050",
            feature = "cuda-11060",
            feature = "cuda-11070",
            feature = "cuda-11080",
            feature = "cuda-12000",
            feature = "cuda-12010",
            feature = "cuda-12020",
            feature = "cuda-12030",
            feature = "cuda-12040",
            feature = "cuda-12050",
            feature = "cuda-12060",
            feature = "cuda-12080",
            feature = "cuda-12090"
        ))]
        pub cudaStreamUpdateCaptureDependencies: Option<
            unsafe extern "C" fn(
                stream: cudaStream_t,
                dependencies: *mut cudaGraphNode_t,
                numDependencies: usize,
                flags: ::core::ffi::c_uint,
            ) -> cudaError_t,
        >,
        #[cfg(any(feature = "cuda-13000"))]
        pub cudaStreamUpdateCaptureDependencies: Option<
            unsafe extern "C" fn(
                stream: cudaStream_t,
                dependencies: *mut cudaGraphNode_t,
                dependencyData: *const cudaGraphEdgeData,
                numDependencies: usize,
                flags: ::core::ffi::c_uint,
            ) -> cudaError_t,
        >,
        #[cfg(any(
            feature = "cuda-12030",
            feature = "cuda-12040",
            feature = "cuda-12050",
            feature = "cuda-12060",
            feature = "cuda-12080",
            feature = "cuda-12090"
        ))]
        pub cudaStreamUpdateCaptureDependencies_v2: Option<
            unsafe extern "C" fn(
                stream: cudaStream_t,
                dependencies: *mut cudaGraphNode_t,
                dependencyData: *const cudaGraphEdgeData,
                numDependencies: usize,
                flags: ::core::ffi::c_uint,
            ) -> cudaError_t,
        >,
        pub cudaStreamWaitEvent: Option<
            unsafe extern "C" fn(
                stream: cudaStream_t,
                event: cudaEvent_t,
                flags: ::core::ffi::c_uint,
            ) -> cudaError_t,
        >,
        pub cudaThreadExchangeStreamCaptureMode:
            Option<unsafe extern "C" fn(mode: *mut cudaStreamCaptureMode) -> cudaError_t>,
        #[cfg(any(
            feature = "cuda-11040",
            feature = "cuda-11050",
            feature = "cuda-11060",
            feature = "cuda-11070",
            feature = "cuda-11080",
            feature = "cuda-12000",
            feature = "cuda-12010",
            feature = "cuda-12020",
            feature = "cuda-12030",
            feature = "cuda-12040",
            feature = "cuda-12050",
            feature = "cuda-12060",
            feature = "cuda-12080",
            feature = "cuda-12090"
        ))]
        pub cudaThreadExit: Option<unsafe extern "C" fn() -> cudaError_t>,
        #[cfg(any(
            feature = "cuda-11040",
            feature = "cuda-11050",
            feature = "cuda-11060",
            feature = "cuda-11070",
            feature = "cuda-11080",
            feature = "cuda-12000",
            feature = "cuda-12010",
            feature = "cuda-12020",
            feature = "cuda-12030",
            feature = "cuda-12040",
            feature = "cuda-12050",
            feature = "cuda-12060",
            feature = "cuda-12080",
            feature = "cuda-12090"
        ))]
        pub cudaThreadGetCacheConfig:
            Option<unsafe extern "C" fn(pCacheConfig: *mut cudaFuncCache) -> cudaError_t>,
        #[cfg(any(
            feature = "cuda-11040",
            feature = "cuda-11050",
            feature = "cuda-11060",
            feature = "cuda-11070",
            feature = "cuda-11080",
            feature = "cuda-12000",
            feature = "cuda-12010",
            feature = "cuda-12020",
            feature = "cuda-12030",
            feature = "cuda-12040",
            feature = "cuda-12050",
            feature = "cuda-12060",
            feature = "cuda-12080",
            feature = "cuda-12090"
        ))]
        pub cudaThreadGetLimit:
            Option<unsafe extern "C" fn(pValue: *mut usize, limit: cudaLimit) -> cudaError_t>,
        #[cfg(any(
            feature = "cuda-11040",
            feature = "cuda-11050",
            feature = "cuda-11060",
            feature = "cuda-11070",
            feature = "cuda-11080",
            feature = "cuda-12000",
            feature = "cuda-12010",
            feature = "cuda-12020",
            feature = "cuda-12030",
            feature = "cuda-12040",
            feature = "cuda-12050",
            feature = "cuda-12060",
            feature = "cuda-12080",
            feature = "cuda-12090"
        ))]
        pub cudaThreadSetCacheConfig:
            Option<unsafe extern "C" fn(cacheConfig: cudaFuncCache) -> cudaError_t>,
        #[cfg(any(
            feature = "cuda-11040",
            feature = "cuda-11050",
            feature = "cuda-11060",
            feature = "cuda-11070",
            feature = "cuda-11080",
            feature = "cuda-12000",
            feature = "cuda-12010",
            feature = "cuda-12020",
            feature = "cuda-12030",
            feature = "cuda-12040",
            feature = "cuda-12050",
            feature = "cuda-12060",
            feature = "cuda-12080",
            feature = "cuda-12090"
        ))]
        pub cudaThreadSetLimit:
            Option<unsafe extern "C" fn(limit: cudaLimit, value: usize) -> cudaError_t>,
        #[cfg(any(
            feature = "cuda-11040",
            feature = "cuda-11050",
            feature = "cuda-11060",
            feature = "cuda-11070",
            feature = "cuda-11080",
            feature = "cuda-12000",
            feature = "cuda-12010",
            feature = "cuda-12020",
            feature = "cuda-12030",
            feature = "cuda-12040",
            feature = "cuda-12050",
            feature = "cuda-12060",
            feature = "cuda-12080",
            feature = "cuda-12090"
        ))]
        pub cudaThreadSynchronize: Option<unsafe extern "C" fn() -> cudaError_t>,
        #[cfg(any(
            feature = "cuda-11040",
            feature = "cuda-11050",
            feature = "cuda-11060",
            feature = "cuda-11070",
            feature = "cuda-11080"
        ))]
        pub cudaUnbindTexture:
            Option<unsafe extern "C" fn(texref: *const textureReference) -> cudaError_t>,
        pub cudaUserObjectCreate: Option<
            unsafe extern "C" fn(
                object_out: *mut cudaUserObject_t,
                ptr: *mut ::core::ffi::c_void,
                destroy: cudaHostFn_t,
                initialRefcount: ::core::ffi::c_uint,
                flags: ::core::ffi::c_uint,
            ) -> cudaError_t,
        >,
        pub cudaUserObjectRelease: Option<
            unsafe extern "C" fn(
                object: cudaUserObject_t,
                count: ::core::ffi::c_uint,
            ) -> cudaError_t,
        >,
        pub cudaUserObjectRetain: Option<
            unsafe extern "C" fn(
                object: cudaUserObject_t,
                count: ::core::ffi::c_uint,
            ) -> cudaError_t,
        >,
        #[cfg(any(feature = "cuda-13000"))]
        pub cudaWaitExternalSemaphoresAsync: Option<
            unsafe extern "C" fn(
                extSemArray: *const cudaExternalSemaphore_t,
                paramsArray: *const cudaExternalSemaphoreWaitParams,
                numExtSems: ::core::ffi::c_uint,
                stream: cudaStream_t,
            ) -> cudaError_t,
        >,
        #[cfg(any(
            feature = "cuda-11040",
            feature = "cuda-11050",
            feature = "cuda-11060",
            feature = "cuda-11070",
            feature = "cuda-11080",
            feature = "cuda-12000",
            feature = "cuda-12010",
            feature = "cuda-12020",
            feature = "cuda-12030",
            feature = "cuda-12040",
            feature = "cuda-12050",
            feature = "cuda-12060",
            feature = "cuda-12080",
            feature = "cuda-12090"
        ))]
        pub cudaWaitExternalSemaphoresAsync_v2: Option<
            unsafe extern "C" fn(
                extSemArray: *const cudaExternalSemaphore_t,
                paramsArray: *const cudaExternalSemaphoreWaitParams,
                numExtSems: ::core::ffi::c_uint,
                stream: cudaStream_t,
            ) -> cudaError_t,
        >,
    }
    impl Lib {
        pub unsafe fn new<P>(path: P) -> Result<Self, ::libloading::Error>
        where
            P: AsRef<::std::ffi::OsStr>,
        {
            let library = ::libloading::Library::new(path)?;
            Self::from_library(library)
        }
        pub unsafe fn from_library<L>(library: L) -> Result<Self, ::libloading::Error>
        where
            L: Into<::libloading::Library>,
        {
            let __library = library.into();
            let cudaArrayGetInfo = __library.get(b"cudaArrayGetInfo\0").ok().map(|sym| *sym);
            #[cfg(any(
                feature = "cuda-11060",
                feature = "cuda-11070",
                feature = "cuda-11080",
                feature = "cuda-12000",
                feature = "cuda-12010",
                feature = "cuda-12020",
                feature = "cuda-12030",
                feature = "cuda-12040",
                feature = "cuda-12050",
                feature = "cuda-12060",
                feature = "cuda-12080",
                feature = "cuda-12090",
                feature = "cuda-13000"
            ))]
            let cudaArrayGetMemoryRequirements = __library
                .get(b"cudaArrayGetMemoryRequirements\0")
                .ok()
                .map(|sym| *sym);
            let cudaArrayGetPlane = __library.get(b"cudaArrayGetPlane\0").ok().map(|sym| *sym);
            let cudaArrayGetSparseProperties = __library
                .get(b"cudaArrayGetSparseProperties\0")
                .ok()
                .map(|sym| *sym);
            #[cfg(any(
                feature = "cuda-11040",
                feature = "cuda-11050",
                feature = "cuda-11060",
                feature = "cuda-11070",
                feature = "cuda-11080"
            ))]
            let cudaBindSurfaceToArray = __library
                .get(b"cudaBindSurfaceToArray\0")
                .ok()
                .map(|sym| *sym);
            #[cfg(any(
                feature = "cuda-11040",
                feature = "cuda-11050",
                feature = "cuda-11060",
                feature = "cuda-11070",
                feature = "cuda-11080"
            ))]
            let cudaBindTexture = __library.get(b"cudaBindTexture\0").ok().map(|sym| *sym);
            #[cfg(any(
                feature = "cuda-11040",
                feature = "cuda-11050",
                feature = "cuda-11060",
                feature = "cuda-11070",
                feature = "cuda-11080"
            ))]
            let cudaBindTexture2D = __library.get(b"cudaBindTexture2D\0").ok().map(|sym| *sym);
            #[cfg(any(
                feature = "cuda-11040",
                feature = "cuda-11050",
                feature = "cuda-11060",
                feature = "cuda-11070",
                feature = "cuda-11080"
            ))]
            let cudaBindTextureToArray = __library
                .get(b"cudaBindTextureToArray\0")
                .ok()
                .map(|sym| *sym);
            #[cfg(any(
                feature = "cuda-11040",
                feature = "cuda-11050",
                feature = "cuda-11060",
                feature = "cuda-11070",
                feature = "cuda-11080"
            ))]
            let cudaBindTextureToMipmappedArray = __library
                .get(b"cudaBindTextureToMipmappedArray\0")
                .ok()
                .map(|sym| *sym);
            let cudaChooseDevice = __library.get(b"cudaChooseDevice\0").ok().map(|sym| *sym);
            let cudaCreateChannelDesc = __library
                .get(b"cudaCreateChannelDesc\0")
                .ok()
                .map(|sym| *sym);
            let cudaCreateSurfaceObject = __library
                .get(b"cudaCreateSurfaceObject\0")
                .ok()
                .map(|sym| *sym);
            let cudaCreateTextureObject = __library
                .get(b"cudaCreateTextureObject\0")
                .ok()
                .map(|sym| *sym);
            #[cfg(any(feature = "cuda-11080"))]
            let cudaCreateTextureObject_v2 = __library
                .get(b"cudaCreateTextureObject_v2\0")
                .ok()
                .map(|sym| *sym);
            let cudaCtxResetPersistingL2Cache = __library
                .get(b"cudaCtxResetPersistingL2Cache\0")
                .ok()
                .map(|sym| *sym);
            let cudaDestroyExternalMemory = __library
                .get(b"cudaDestroyExternalMemory\0")
                .ok()
                .map(|sym| *sym);
            let cudaDestroyExternalSemaphore = __library
                .get(b"cudaDestroyExternalSemaphore\0")
                .ok()
                .map(|sym| *sym);
            let cudaDestroySurfaceObject = __library
                .get(b"cudaDestroySurfaceObject\0")
                .ok()
                .map(|sym| *sym);
            let cudaDestroyTextureObject = __library
                .get(b"cudaDestroyTextureObject\0")
                .ok()
                .map(|sym| *sym);
            let cudaDeviceCanAccessPeer = __library
                .get(b"cudaDeviceCanAccessPeer\0")
                .ok()
                .map(|sym| *sym);
            let cudaDeviceDisablePeerAccess = __library
                .get(b"cudaDeviceDisablePeerAccess\0")
                .ok()
                .map(|sym| *sym);
            let cudaDeviceEnablePeerAccess = __library
                .get(b"cudaDeviceEnablePeerAccess\0")
                .ok()
                .map(|sym| *sym);
            let cudaDeviceFlushGPUDirectRDMAWrites = __library
                .get(b"cudaDeviceFlushGPUDirectRDMAWrites\0")
                .ok()
                .map(|sym| *sym);
            let cudaDeviceGetAttribute = __library
                .get(b"cudaDeviceGetAttribute\0")
                .ok()
                .map(|sym| *sym);
            let cudaDeviceGetByPCIBusId = __library
                .get(b"cudaDeviceGetByPCIBusId\0")
                .ok()
                .map(|sym| *sym);
            let cudaDeviceGetCacheConfig = __library
                .get(b"cudaDeviceGetCacheConfig\0")
                .ok()
                .map(|sym| *sym);
            let cudaDeviceGetDefaultMemPool = __library
                .get(b"cudaDeviceGetDefaultMemPool\0")
                .ok()
                .map(|sym| *sym);
            let cudaDeviceGetGraphMemAttribute = __library
                .get(b"cudaDeviceGetGraphMemAttribute\0")
                .ok()
                .map(|sym| *sym);
            #[cfg(any(feature = "cuda-13000"))]
            let cudaDeviceGetHostAtomicCapabilities = __library
                .get(b"cudaDeviceGetHostAtomicCapabilities\0")
                .ok()
                .map(|sym| *sym);
            let cudaDeviceGetLimit = __library.get(b"cudaDeviceGetLimit\0").ok().map(|sym| *sym);
            let cudaDeviceGetMemPool = __library
                .get(b"cudaDeviceGetMemPool\0")
                .ok()
                .map(|sym| *sym);
            #[cfg(any(feature = "cuda-13000"))]
            let cudaDeviceGetP2PAtomicCapabilities = __library
                .get(b"cudaDeviceGetP2PAtomicCapabilities\0")
                .ok()
                .map(|sym| *sym);
            let cudaDeviceGetP2PAttribute = __library
                .get(b"cudaDeviceGetP2PAttribute\0")
                .ok()
                .map(|sym| *sym);
            let cudaDeviceGetPCIBusId = __library
                .get(b"cudaDeviceGetPCIBusId\0")
                .ok()
                .map(|sym| *sym);
            let cudaDeviceGetSharedMemConfig = __library
                .get(b"cudaDeviceGetSharedMemConfig\0")
                .ok()
                .map(|sym| *sym);
            let cudaDeviceGetStreamPriorityRange = __library
                .get(b"cudaDeviceGetStreamPriorityRange\0")
                .ok()
                .map(|sym| *sym);
            let cudaDeviceGetTexture1DLinearMaxWidth = __library
                .get(b"cudaDeviceGetTexture1DLinearMaxWidth\0")
                .ok()
                .map(|sym| *sym);
            let cudaDeviceGraphMemTrim = __library
                .get(b"cudaDeviceGraphMemTrim\0")
                .ok()
                .map(|sym| *sym);
            #[cfg(any(
                feature = "cuda-12040",
                feature = "cuda-12050",
                feature = "cuda-12060",
                feature = "cuda-12080",
                feature = "cuda-12090",
                feature = "cuda-13000"
            ))]
            let cudaDeviceRegisterAsyncNotification = __library
                .get(b"cudaDeviceRegisterAsyncNotification\0")
                .ok()
                .map(|sym| *sym);
            let cudaDeviceReset = __library.get(b"cudaDeviceReset\0").ok().map(|sym| *sym);
            let cudaDeviceSetCacheConfig = __library
                .get(b"cudaDeviceSetCacheConfig\0")
                .ok()
                .map(|sym| *sym);
            let cudaDeviceSetGraphMemAttribute = __library
                .get(b"cudaDeviceSetGraphMemAttribute\0")
                .ok()
                .map(|sym| *sym);
            let cudaDeviceSetLimit = __library.get(b"cudaDeviceSetLimit\0").ok().map(|sym| *sym);
            let cudaDeviceSetMemPool = __library
                .get(b"cudaDeviceSetMemPool\0")
                .ok()
                .map(|sym| *sym);
            let cudaDeviceSetSharedMemConfig = __library
                .get(b"cudaDeviceSetSharedMemConfig\0")
                .ok()
                .map(|sym| *sym);
            let cudaDeviceSynchronize = __library
                .get(b"cudaDeviceSynchronize\0")
                .ok()
                .map(|sym| *sym);
            #[cfg(any(
                feature = "cuda-12040",
                feature = "cuda-12050",
                feature = "cuda-12060",
                feature = "cuda-12080",
                feature = "cuda-12090",
                feature = "cuda-13000"
            ))]
            let cudaDeviceUnregisterAsyncNotification = __library
                .get(b"cudaDeviceUnregisterAsyncNotification\0")
                .ok()
                .map(|sym| *sym);
            let cudaDriverGetVersion = __library
                .get(b"cudaDriverGetVersion\0")
                .ok()
                .map(|sym| *sym);
            let cudaEventCreate = __library.get(b"cudaEventCreate\0").ok().map(|sym| *sym);
            let cudaEventCreateWithFlags = __library
                .get(b"cudaEventCreateWithFlags\0")
                .ok()
                .map(|sym| *sym);
            let cudaEventDestroy = __library.get(b"cudaEventDestroy\0").ok().map(|sym| *sym);
            let cudaEventElapsedTime = __library
                .get(b"cudaEventElapsedTime\0")
                .ok()
                .map(|sym| *sym);
            #[cfg(any(feature = "cuda-12080", feature = "cuda-12090"))]
            let cudaEventElapsedTime_v2 = __library
                .get(b"cudaEventElapsedTime_v2\0")
                .ok()
                .map(|sym| *sym);
            let cudaEventQuery = __library.get(b"cudaEventQuery\0").ok().map(|sym| *sym);
            let cudaEventRecord = __library.get(b"cudaEventRecord\0").ok().map(|sym| *sym);
            let cudaEventRecordWithFlags = __library
                .get(b"cudaEventRecordWithFlags\0")
                .ok()
                .map(|sym| *sym);
            let cudaEventSynchronize = __library
                .get(b"cudaEventSynchronize\0")
                .ok()
                .map(|sym| *sym);
            let cudaExternalMemoryGetMappedBuffer = __library
                .get(b"cudaExternalMemoryGetMappedBuffer\0")
                .ok()
                .map(|sym| *sym);
            let cudaExternalMemoryGetMappedMipmappedArray = __library
                .get(b"cudaExternalMemoryGetMappedMipmappedArray\0")
                .ok()
                .map(|sym| *sym);
            let cudaFree = __library.get(b"cudaFree\0").ok().map(|sym| *sym);
            let cudaFreeArray = __library.get(b"cudaFreeArray\0").ok().map(|sym| *sym);
            let cudaFreeAsync = __library.get(b"cudaFreeAsync\0").ok().map(|sym| *sym);
            let cudaFreeHost = __library.get(b"cudaFreeHost\0").ok().map(|sym| *sym);
            let cudaFreeMipmappedArray = __library
                .get(b"cudaFreeMipmappedArray\0")
                .ok()
                .map(|sym| *sym);
            let cudaFuncGetAttributes = __library
                .get(b"cudaFuncGetAttributes\0")
                .ok()
                .map(|sym| *sym);
            #[cfg(any(
                feature = "cuda-12030",
                feature = "cuda-12040",
                feature = "cuda-12050",
                feature = "cuda-12060",
                feature = "cuda-12080",
                feature = "cuda-12090",
                feature = "cuda-13000"
            ))]
            let cudaFuncGetName = __library.get(b"cudaFuncGetName\0").ok().map(|sym| *sym);
            #[cfg(any(
                feature = "cuda-12040",
                feature = "cuda-12050",
                feature = "cuda-12060",
                feature = "cuda-12080",
                feature = "cuda-12090",
                feature = "cuda-13000"
            ))]
            let cudaFuncGetParamInfo = __library
                .get(b"cudaFuncGetParamInfo\0")
                .ok()
                .map(|sym| *sym);
            let cudaFuncSetAttribute = __library
                .get(b"cudaFuncSetAttribute\0")
                .ok()
                .map(|sym| *sym);
            let cudaFuncSetCacheConfig = __library
                .get(b"cudaFuncSetCacheConfig\0")
                .ok()
                .map(|sym| *sym);
            let cudaFuncSetSharedMemConfig = __library
                .get(b"cudaFuncSetSharedMemConfig\0")
                .ok()
                .map(|sym| *sym);
            let cudaGetChannelDesc = __library.get(b"cudaGetChannelDesc\0").ok().map(|sym| *sym);
            let cudaGetDevice = __library.get(b"cudaGetDevice\0").ok().map(|sym| *sym);
            let cudaGetDeviceCount = __library.get(b"cudaGetDeviceCount\0").ok().map(|sym| *sym);
            let cudaGetDeviceFlags = __library.get(b"cudaGetDeviceFlags\0").ok().map(|sym| *sym);
            #[cfg(any(
                feature = "cuda-11040",
                feature = "cuda-11050",
                feature = "cuda-11060",
                feature = "cuda-11070",
                feature = "cuda-11080",
                feature = "cuda-13000"
            ))]
            let cudaGetDeviceProperties = __library
                .get(b"cudaGetDeviceProperties\0")
                .ok()
                .map(|sym| *sym);
            #[cfg(any(
                feature = "cuda-12000",
                feature = "cuda-12010",
                feature = "cuda-12020",
                feature = "cuda-12030",
                feature = "cuda-12040",
                feature = "cuda-12050",
                feature = "cuda-12060",
                feature = "cuda-12080",
                feature = "cuda-12090"
            ))]
            let cudaGetDeviceProperties_v2 = __library
                .get(b"cudaGetDeviceProperties_v2\0")
                .ok()
                .map(|sym| *sym);
            #[cfg(any(
                feature = "cuda-11040",
                feature = "cuda-11050",
                feature = "cuda-11060",
                feature = "cuda-11070",
                feature = "cuda-11080"
            ))]
            let cudaGetDriverEntryPoint = __library
                .get(b"cudaGetDriverEntryPoint\0")
                .ok()
                .map(|sym| *sym);
            #[cfg(any(
                feature = "cuda-12000",
                feature = "cuda-12010",
                feature = "cuda-12020",
                feature = "cuda-12030",
                feature = "cuda-12040",
                feature = "cuda-12050",
                feature = "cuda-12060",
                feature = "cuda-12080",
                feature = "cuda-12090",
                feature = "cuda-13000"
            ))]
            let cudaGetDriverEntryPoint = __library
                .get(b"cudaGetDriverEntryPoint\0")
                .ok()
                .map(|sym| *sym);
            #[cfg(any(
                feature = "cuda-12050",
                feature = "cuda-12060",
                feature = "cuda-12080",
                feature = "cuda-12090",
                feature = "cuda-13000"
            ))]
            let cudaGetDriverEntryPointByVersion = __library
                .get(b"cudaGetDriverEntryPointByVersion\0")
                .ok()
                .map(|sym| *sym);
            let cudaGetErrorName = __library.get(b"cudaGetErrorName\0").ok().map(|sym| *sym);
            let cudaGetErrorString = __library.get(b"cudaGetErrorString\0").ok().map(|sym| *sym);
            let cudaGetExportTable = __library.get(b"cudaGetExportTable\0").ok().map(|sym| *sym);
            let cudaGetFuncBySymbol = __library.get(b"cudaGetFuncBySymbol\0").ok().map(|sym| *sym);
            #[cfg(any(
                feature = "cuda-12010",
                feature = "cuda-12020",
                feature = "cuda-12030",
                feature = "cuda-12040",
                feature = "cuda-12050",
                feature = "cuda-12060",
                feature = "cuda-12080",
                feature = "cuda-12090",
                feature = "cuda-13000"
            ))]
            let cudaGetKernel = __library.get(b"cudaGetKernel\0").ok().map(|sym| *sym);
            let cudaGetLastError = __library.get(b"cudaGetLastError\0").ok().map(|sym| *sym);
            let cudaGetMipmappedArrayLevel = __library
                .get(b"cudaGetMipmappedArrayLevel\0")
                .ok()
                .map(|sym| *sym);
            let cudaGetSurfaceObjectResourceDesc = __library
                .get(b"cudaGetSurfaceObjectResourceDesc\0")
                .ok()
                .map(|sym| *sym);
            #[cfg(any(
                feature = "cuda-11040",
                feature = "cuda-11050",
                feature = "cuda-11060",
                feature = "cuda-11070",
                feature = "cuda-11080"
            ))]
            let cudaGetSurfaceReference = __library
                .get(b"cudaGetSurfaceReference\0")
                .ok()
                .map(|sym| *sym);
            let cudaGetSymbolAddress = __library
                .get(b"cudaGetSymbolAddress\0")
                .ok()
                .map(|sym| *sym);
            let cudaGetSymbolSize = __library.get(b"cudaGetSymbolSize\0").ok().map(|sym| *sym);
            #[cfg(any(
                feature = "cuda-11040",
                feature = "cuda-11050",
                feature = "cuda-11060",
                feature = "cuda-11070",
                feature = "cuda-11080"
            ))]
            let cudaGetTextureAlignmentOffset = __library
                .get(b"cudaGetTextureAlignmentOffset\0")
                .ok()
                .map(|sym| *sym);
            let cudaGetTextureObjectResourceDesc = __library
                .get(b"cudaGetTextureObjectResourceDesc\0")
                .ok()
                .map(|sym| *sym);
            let cudaGetTextureObjectResourceViewDesc = __library
                .get(b"cudaGetTextureObjectResourceViewDesc\0")
                .ok()
                .map(|sym| *sym);
            let cudaGetTextureObjectTextureDesc = __library
                .get(b"cudaGetTextureObjectTextureDesc\0")
                .ok()
                .map(|sym| *sym);
            #[cfg(any(feature = "cuda-11080"))]
            let cudaGetTextureObjectTextureDesc_v2 = __library
                .get(b"cudaGetTextureObjectTextureDesc_v2\0")
                .ok()
                .map(|sym| *sym);
            #[cfg(any(
                feature = "cuda-11040",
                feature = "cuda-11050",
                feature = "cuda-11060",
                feature = "cuda-11070",
                feature = "cuda-11080"
            ))]
            let cudaGetTextureReference = __library
                .get(b"cudaGetTextureReference\0")
                .ok()
                .map(|sym| *sym);
            let cudaGraphAddChildGraphNode = __library
                .get(b"cudaGraphAddChildGraphNode\0")
                .ok()
                .map(|sym| *sym);
            #[cfg(any(
                feature = "cuda-11040",
                feature = "cuda-11050",
                feature = "cuda-11060",
                feature = "cuda-11070",
                feature = "cuda-11080",
                feature = "cuda-12000",
                feature = "cuda-12010",
                feature = "cuda-12020",
                feature = "cuda-12030",
                feature = "cuda-12040",
                feature = "cuda-12050",
                feature = "cuda-12060",
                feature = "cuda-12080",
                feature = "cuda-12090"
            ))]
            let cudaGraphAddDependencies = __library
                .get(b"cudaGraphAddDependencies\0")
                .ok()
                .map(|sym| *sym);
            #[cfg(any(feature = "cuda-13000"))]
            let cudaGraphAddDependencies = __library
                .get(b"cudaGraphAddDependencies\0")
                .ok()
                .map(|sym| *sym);
            #[cfg(any(
                feature = "cuda-12030",
                feature = "cuda-12040",
                feature = "cuda-12050",
                feature = "cuda-12060",
                feature = "cuda-12080",
                feature = "cuda-12090"
            ))]
            let cudaGraphAddDependencies_v2 = __library
                .get(b"cudaGraphAddDependencies_v2\0")
                .ok()
                .map(|sym| *sym);
            let cudaGraphAddEmptyNode = __library
                .get(b"cudaGraphAddEmptyNode\0")
                .ok()
                .map(|sym| *sym);
            let cudaGraphAddEventRecordNode = __library
                .get(b"cudaGraphAddEventRecordNode\0")
                .ok()
                .map(|sym| *sym);
            let cudaGraphAddEventWaitNode = __library
                .get(b"cudaGraphAddEventWaitNode\0")
                .ok()
                .map(|sym| *sym);
            let cudaGraphAddExternalSemaphoresSignalNode = __library
                .get(b"cudaGraphAddExternalSemaphoresSignalNode\0")
                .ok()
                .map(|sym| *sym);
            let cudaGraphAddExternalSemaphoresWaitNode = __library
                .get(b"cudaGraphAddExternalSemaphoresWaitNode\0")
                .ok()
                .map(|sym| *sym);
            let cudaGraphAddHostNode = __library
                .get(b"cudaGraphAddHostNode\0")
                .ok()
                .map(|sym| *sym);
            let cudaGraphAddKernelNode = __library
                .get(b"cudaGraphAddKernelNode\0")
                .ok()
                .map(|sym| *sym);
            let cudaGraphAddMemAllocNode = __library
                .get(b"cudaGraphAddMemAllocNode\0")
                .ok()
                .map(|sym| *sym);
            let cudaGraphAddMemFreeNode = __library
                .get(b"cudaGraphAddMemFreeNode\0")
                .ok()
                .map(|sym| *sym);
            let cudaGraphAddMemcpyNode = __library
                .get(b"cudaGraphAddMemcpyNode\0")
                .ok()
                .map(|sym| *sym);
            let cudaGraphAddMemcpyNode1D = __library
                .get(b"cudaGraphAddMemcpyNode1D\0")
                .ok()
                .map(|sym| *sym);
            let cudaGraphAddMemcpyNodeFromSymbol = __library
                .get(b"cudaGraphAddMemcpyNodeFromSymbol\0")
                .ok()
                .map(|sym| *sym);
            let cudaGraphAddMemcpyNodeToSymbol = __library
                .get(b"cudaGraphAddMemcpyNodeToSymbol\0")
                .ok()
                .map(|sym| *sym);
            let cudaGraphAddMemsetNode = __library
                .get(b"cudaGraphAddMemsetNode\0")
                .ok()
                .map(|sym| *sym);
            #[cfg(any(
                feature = "cuda-12020",
                feature = "cuda-12030",
                feature = "cuda-12040",
                feature = "cuda-12050",
                feature = "cuda-12060",
                feature = "cuda-12080",
                feature = "cuda-12090"
            ))]
            let cudaGraphAddNode = __library.get(b"cudaGraphAddNode\0").ok().map(|sym| *sym);
            #[cfg(any(feature = "cuda-13000"))]
            let cudaGraphAddNode = __library.get(b"cudaGraphAddNode\0").ok().map(|sym| *sym);
            #[cfg(any(
                feature = "cuda-12030",
                feature = "cuda-12040",
                feature = "cuda-12050",
                feature = "cuda-12060",
                feature = "cuda-12080",
                feature = "cuda-12090"
            ))]
            let cudaGraphAddNode_v2 = __library.get(b"cudaGraphAddNode_v2\0").ok().map(|sym| *sym);
            let cudaGraphChildGraphNodeGetGraph = __library
                .get(b"cudaGraphChildGraphNodeGetGraph\0")
                .ok()
                .map(|sym| *sym);
            let cudaGraphClone = __library.get(b"cudaGraphClone\0").ok().map(|sym| *sym);
            #[cfg(any(
                feature = "cuda-12030",
                feature = "cuda-12040",
                feature = "cuda-12050",
                feature = "cuda-12060",
                feature = "cuda-12080",
                feature = "cuda-12090",
                feature = "cuda-13000"
            ))]
            let cudaGraphConditionalHandleCreate = __library
                .get(b"cudaGraphConditionalHandleCreate\0")
                .ok()
                .map(|sym| *sym);
            let cudaGraphCreate = __library.get(b"cudaGraphCreate\0").ok().map(|sym| *sym);
            let cudaGraphDebugDotPrint = __library
                .get(b"cudaGraphDebugDotPrint\0")
                .ok()
                .map(|sym| *sym);
            let cudaGraphDestroy = __library.get(b"cudaGraphDestroy\0").ok().map(|sym| *sym);
            let cudaGraphDestroyNode = __library
                .get(b"cudaGraphDestroyNode\0")
                .ok()
                .map(|sym| *sym);
            let cudaGraphEventRecordNodeGetEvent = __library
                .get(b"cudaGraphEventRecordNodeGetEvent\0")
                .ok()
                .map(|sym| *sym);
            let cudaGraphEventRecordNodeSetEvent = __library
                .get(b"cudaGraphEventRecordNodeSetEvent\0")
                .ok()
                .map(|sym| *sym);
            let cudaGraphEventWaitNodeGetEvent = __library
                .get(b"cudaGraphEventWaitNodeGetEvent\0")
                .ok()
                .map(|sym| *sym);
            let cudaGraphEventWaitNodeSetEvent = __library
                .get(b"cudaGraphEventWaitNodeSetEvent\0")
                .ok()
                .map(|sym| *sym);
            let cudaGraphExecChildGraphNodeSetParams = __library
                .get(b"cudaGraphExecChildGraphNodeSetParams\0")
                .ok()
                .map(|sym| *sym);
            let cudaGraphExecDestroy = __library
                .get(b"cudaGraphExecDestroy\0")
                .ok()
                .map(|sym| *sym);
            let cudaGraphExecEventRecordNodeSetEvent = __library
                .get(b"cudaGraphExecEventRecordNodeSetEvent\0")
                .ok()
                .map(|sym| *sym);
            let cudaGraphExecEventWaitNodeSetEvent = __library
                .get(b"cudaGraphExecEventWaitNodeSetEvent\0")
                .ok()
                .map(|sym| *sym);
            let cudaGraphExecExternalSemaphoresSignalNodeSetParams = __library
                .get(b"cudaGraphExecExternalSemaphoresSignalNodeSetParams\0")
                .ok()
                .map(|sym| *sym);
            let cudaGraphExecExternalSemaphoresWaitNodeSetParams = __library
                .get(b"cudaGraphExecExternalSemaphoresWaitNodeSetParams\0")
                .ok()
                .map(|sym| *sym);
            #[cfg(any(
                feature = "cuda-12000",
                feature = "cuda-12010",
                feature = "cuda-12020",
                feature = "cuda-12030",
                feature = "cuda-12040",
                feature = "cuda-12050",
                feature = "cuda-12060",
                feature = "cuda-12080",
                feature = "cuda-12090",
                feature = "cuda-13000"
            ))]
            let cudaGraphExecGetFlags = __library
                .get(b"cudaGraphExecGetFlags\0")
                .ok()
                .map(|sym| *sym);
            let cudaGraphExecHostNodeSetParams = __library
                .get(b"cudaGraphExecHostNodeSetParams\0")
                .ok()
                .map(|sym| *sym);
            let cudaGraphExecKernelNodeSetParams = __library
                .get(b"cudaGraphExecKernelNodeSetParams\0")
                .ok()
                .map(|sym| *sym);
            let cudaGraphExecMemcpyNodeSetParams = __library
                .get(b"cudaGraphExecMemcpyNodeSetParams\0")
                .ok()
                .map(|sym| *sym);
            let cudaGraphExecMemcpyNodeSetParams1D = __library
                .get(b"cudaGraphExecMemcpyNodeSetParams1D\0")
                .ok()
                .map(|sym| *sym);
            let cudaGraphExecMemcpyNodeSetParamsFromSymbol = __library
                .get(b"cudaGraphExecMemcpyNodeSetParamsFromSymbol\0")
                .ok()
                .map(|sym| *sym);
            let cudaGraphExecMemcpyNodeSetParamsToSymbol = __library
                .get(b"cudaGraphExecMemcpyNodeSetParamsToSymbol\0")
                .ok()
                .map(|sym| *sym);
            let cudaGraphExecMemsetNodeSetParams = __library
                .get(b"cudaGraphExecMemsetNodeSetParams\0")
                .ok()
                .map(|sym| *sym);
            #[cfg(any(
                feature = "cuda-12020",
                feature = "cuda-12030",
                feature = "cuda-12040",
                feature = "cuda-12050",
                feature = "cuda-12060",
                feature = "cuda-12080",
                feature = "cuda-12090",
                feature = "cuda-13000"
            ))]
            let cudaGraphExecNodeSetParams = __library
                .get(b"cudaGraphExecNodeSetParams\0")
                .ok()
                .map(|sym| *sym);
            #[cfg(any(
                feature = "cuda-11040",
                feature = "cuda-11050",
                feature = "cuda-11060",
                feature = "cuda-11070",
                feature = "cuda-11080"
            ))]
            let cudaGraphExecUpdate = __library.get(b"cudaGraphExecUpdate\0").ok().map(|sym| *sym);
            #[cfg(any(
                feature = "cuda-12000",
                feature = "cuda-12010",
                feature = "cuda-12020",
                feature = "cuda-12030",
                feature = "cuda-12040",
                feature = "cuda-12050",
                feature = "cuda-12060",
                feature = "cuda-12080",
                feature = "cuda-12090",
                feature = "cuda-13000"
            ))]
            let cudaGraphExecUpdate = __library.get(b"cudaGraphExecUpdate\0").ok().map(|sym| *sym);
            let cudaGraphExternalSemaphoresSignalNodeGetParams = __library
                .get(b"cudaGraphExternalSemaphoresSignalNodeGetParams\0")
                .ok()
                .map(|sym| *sym);
            let cudaGraphExternalSemaphoresSignalNodeSetParams = __library
                .get(b"cudaGraphExternalSemaphoresSignalNodeSetParams\0")
                .ok()
                .map(|sym| *sym);
            let cudaGraphExternalSemaphoresWaitNodeGetParams = __library
                .get(b"cudaGraphExternalSemaphoresWaitNodeGetParams\0")
                .ok()
                .map(|sym| *sym);
            let cudaGraphExternalSemaphoresWaitNodeSetParams = __library
                .get(b"cudaGraphExternalSemaphoresWaitNodeSetParams\0")
                .ok()
                .map(|sym| *sym);
            #[cfg(any(
                feature = "cuda-11040",
                feature = "cuda-11050",
                feature = "cuda-11060",
                feature = "cuda-11070",
                feature = "cuda-11080",
                feature = "cuda-12000",
                feature = "cuda-12010",
                feature = "cuda-12020",
                feature = "cuda-12030",
                feature = "cuda-12040",
                feature = "cuda-12050",
                feature = "cuda-12060",
                feature = "cuda-12080",
                feature = "cuda-12090"
            ))]
            let cudaGraphGetEdges = __library.get(b"cudaGraphGetEdges\0").ok().map(|sym| *sym);
            #[cfg(any(feature = "cuda-13000"))]
            let cudaGraphGetEdges = __library.get(b"cudaGraphGetEdges\0").ok().map(|sym| *sym);
            #[cfg(any(
                feature = "cuda-12030",
                feature = "cuda-12040",
                feature = "cuda-12050",
                feature = "cuda-12060",
                feature = "cuda-12080",
                feature = "cuda-12090"
            ))]
            let cudaGraphGetEdges_v2 = __library
                .get(b"cudaGraphGetEdges_v2\0")
                .ok()
                .map(|sym| *sym);
            let cudaGraphGetNodes = __library.get(b"cudaGraphGetNodes\0").ok().map(|sym| *sym);
            let cudaGraphGetRootNodes = __library
                .get(b"cudaGraphGetRootNodes\0")
                .ok()
                .map(|sym| *sym);
            let cudaGraphHostNodeGetParams = __library
                .get(b"cudaGraphHostNodeGetParams\0")
                .ok()
                .map(|sym| *sym);
            let cudaGraphHostNodeSetParams = __library
                .get(b"cudaGraphHostNodeSetParams\0")
                .ok()
                .map(|sym| *sym);
            #[cfg(any(
                feature = "cuda-11040",
                feature = "cuda-11050",
                feature = "cuda-11060",
                feature = "cuda-11070",
                feature = "cuda-11080"
            ))]
            let cudaGraphInstantiate = __library
                .get(b"cudaGraphInstantiate\0")
                .ok()
                .map(|sym| *sym);
            #[cfg(any(
                feature = "cuda-12000",
                feature = "cuda-12010",
                feature = "cuda-12020",
                feature = "cuda-12030",
                feature = "cuda-12040",
                feature = "cuda-12050",
                feature = "cuda-12060",
                feature = "cuda-12080",
                feature = "cuda-12090",
                feature = "cuda-13000"
            ))]
            let cudaGraphInstantiate = __library
                .get(b"cudaGraphInstantiate\0")
                .ok()
                .map(|sym| *sym);
            let cudaGraphInstantiateWithFlags = __library
                .get(b"cudaGraphInstantiateWithFlags\0")
                .ok()
                .map(|sym| *sym);
            #[cfg(any(
                feature = "cuda-12000",
                feature = "cuda-12010",
                feature = "cuda-12020",
                feature = "cuda-12030",
                feature = "cuda-12040",
                feature = "cuda-12050",
                feature = "cuda-12060",
                feature = "cuda-12080",
                feature = "cuda-12090",
                feature = "cuda-13000"
            ))]
            let cudaGraphInstantiateWithParams = __library
                .get(b"cudaGraphInstantiateWithParams\0")
                .ok()
                .map(|sym| *sym);
            #[cfg(any(
                feature = "cuda-11040",
                feature = "cuda-11050",
                feature = "cuda-11060",
                feature = "cuda-11070",
                feature = "cuda-11080",
                feature = "cuda-12000",
                feature = "cuda-12010",
                feature = "cuda-12020",
                feature = "cuda-12030",
                feature = "cuda-12040",
                feature = "cuda-12050",
                feature = "cuda-12060",
                feature = "cuda-12080",
                feature = "cuda-12090"
            ))]
            let cudaGraphKernelNodeCopyAttributes = __library
                .get(b"cudaGraphKernelNodeCopyAttributes\0")
                .ok()
                .map(|sym| *sym);
            #[cfg(any(feature = "cuda-13000"))]
            let cudaGraphKernelNodeCopyAttributes = __library
                .get(b"cudaGraphKernelNodeCopyAttributes\0")
                .ok()
                .map(|sym| *sym);
            #[cfg(any(
                feature = "cuda-11040",
                feature = "cuda-11050",
                feature = "cuda-11060",
                feature = "cuda-11070"
            ))]
            let cudaGraphKernelNodeGetAttribute = __library
                .get(b"cudaGraphKernelNodeGetAttribute\0")
                .ok()
                .map(|sym| *sym);
            #[cfg(any(
                feature = "cuda-11080",
                feature = "cuda-12000",
                feature = "cuda-12010",
                feature = "cuda-12020",
                feature = "cuda-12030",
                feature = "cuda-12040",
                feature = "cuda-12050",
                feature = "cuda-12060",
                feature = "cuda-12080",
                feature = "cuda-12090",
                feature = "cuda-13000"
            ))]
            let cudaGraphKernelNodeGetAttribute = __library
                .get(b"cudaGraphKernelNodeGetAttribute\0")
                .ok()
                .map(|sym| *sym);
            let cudaGraphKernelNodeGetParams = __library
                .get(b"cudaGraphKernelNodeGetParams\0")
                .ok()
                .map(|sym| *sym);
            #[cfg(any(
                feature = "cuda-11040",
                feature = "cuda-11050",
                feature = "cuda-11060",
                feature = "cuda-11070"
            ))]
            let cudaGraphKernelNodeSetAttribute = __library
                .get(b"cudaGraphKernelNodeSetAttribute\0")
                .ok()
                .map(|sym| *sym);
            #[cfg(any(
                feature = "cuda-11080",
                feature = "cuda-12000",
                feature = "cuda-12010",
                feature = "cuda-12020",
                feature = "cuda-12030",
                feature = "cuda-12040",
                feature = "cuda-12050",
                feature = "cuda-12060",
                feature = "cuda-12080",
                feature = "cuda-12090",
                feature = "cuda-13000"
            ))]
            let cudaGraphKernelNodeSetAttribute = __library
                .get(b"cudaGraphKernelNodeSetAttribute\0")
                .ok()
                .map(|sym| *sym);
            let cudaGraphKernelNodeSetParams = __library
                .get(b"cudaGraphKernelNodeSetParams\0")
                .ok()
                .map(|sym| *sym);
            let cudaGraphLaunch = __library.get(b"cudaGraphLaunch\0").ok().map(|sym| *sym);
            let cudaGraphMemAllocNodeGetParams = __library
                .get(b"cudaGraphMemAllocNodeGetParams\0")
                .ok()
                .map(|sym| *sym);
            let cudaGraphMemFreeNodeGetParams = __library
                .get(b"cudaGraphMemFreeNodeGetParams\0")
                .ok()
                .map(|sym| *sym);
            let cudaGraphMemcpyNodeGetParams = __library
                .get(b"cudaGraphMemcpyNodeGetParams\0")
                .ok()
                .map(|sym| *sym);
            let cudaGraphMemcpyNodeSetParams = __library
                .get(b"cudaGraphMemcpyNodeSetParams\0")
                .ok()
                .map(|sym| *sym);
            let cudaGraphMemcpyNodeSetParams1D = __library
                .get(b"cudaGraphMemcpyNodeSetParams1D\0")
                .ok()
                .map(|sym| *sym);
            let cudaGraphMemcpyNodeSetParamsFromSymbol = __library
                .get(b"cudaGraphMemcpyNodeSetParamsFromSymbol\0")
                .ok()
                .map(|sym| *sym);
            let cudaGraphMemcpyNodeSetParamsToSymbol = __library
                .get(b"cudaGraphMemcpyNodeSetParamsToSymbol\0")
                .ok()
                .map(|sym| *sym);
            let cudaGraphMemsetNodeGetParams = __library
                .get(b"cudaGraphMemsetNodeGetParams\0")
                .ok()
                .map(|sym| *sym);
            let cudaGraphMemsetNodeSetParams = __library
                .get(b"cudaGraphMemsetNodeSetParams\0")
                .ok()
                .map(|sym| *sym);
            let cudaGraphNodeFindInClone = __library
                .get(b"cudaGraphNodeFindInClone\0")
                .ok()
                .map(|sym| *sym);
            #[cfg(any(
                feature = "cuda-11040",
                feature = "cuda-11050",
                feature = "cuda-11060",
                feature = "cuda-11070",
                feature = "cuda-11080",
                feature = "cuda-12000",
                feature = "cuda-12010",
                feature = "cuda-12020",
                feature = "cuda-12030",
                feature = "cuda-12040",
                feature = "cuda-12050",
                feature = "cuda-12060",
                feature = "cuda-12080",
                feature = "cuda-12090"
            ))]
            let cudaGraphNodeGetDependencies = __library
                .get(b"cudaGraphNodeGetDependencies\0")
                .ok()
                .map(|sym| *sym);
            #[cfg(any(feature = "cuda-13000"))]
            let cudaGraphNodeGetDependencies = __library
                .get(b"cudaGraphNodeGetDependencies\0")
                .ok()
                .map(|sym| *sym);
            #[cfg(any(
                feature = "cuda-12030",
                feature = "cuda-12040",
                feature = "cuda-12050",
                feature = "cuda-12060",
                feature = "cuda-12080",
                feature = "cuda-12090"
            ))]
            let cudaGraphNodeGetDependencies_v2 = __library
                .get(b"cudaGraphNodeGetDependencies_v2\0")
                .ok()
                .map(|sym| *sym);
            #[cfg(any(
                feature = "cuda-11040",
                feature = "cuda-11050",
                feature = "cuda-11060",
                feature = "cuda-11070",
                feature = "cuda-11080",
                feature = "cuda-12000",
                feature = "cuda-12010",
                feature = "cuda-12020",
                feature = "cuda-12030",
                feature = "cuda-12040",
                feature = "cuda-12050",
                feature = "cuda-12060",
                feature = "cuda-12080",
                feature = "cuda-12090"
            ))]
            let cudaGraphNodeGetDependentNodes = __library
                .get(b"cudaGraphNodeGetDependentNodes\0")
                .ok()
                .map(|sym| *sym);
            #[cfg(any(feature = "cuda-13000"))]
            let cudaGraphNodeGetDependentNodes = __library
                .get(b"cudaGraphNodeGetDependentNodes\0")
                .ok()
                .map(|sym| *sym);
            #[cfg(any(
                feature = "cuda-12030",
                feature = "cuda-12040",
                feature = "cuda-12050",
                feature = "cuda-12060",
                feature = "cuda-12080",
                feature = "cuda-12090"
            ))]
            let cudaGraphNodeGetDependentNodes_v2 = __library
                .get(b"cudaGraphNodeGetDependentNodes_v2\0")
                .ok()
                .map(|sym| *sym);
            #[cfg(any(
                feature = "cuda-11060",
                feature = "cuda-11070",
                feature = "cuda-11080",
                feature = "cuda-12000",
                feature = "cuda-12010",
                feature = "cuda-12020",
                feature = "cuda-12030",
                feature = "cuda-12040",
                feature = "cuda-12050",
                feature = "cuda-12060",
                feature = "cuda-12080",
                feature = "cuda-12090",
                feature = "cuda-13000"
            ))]
            let cudaGraphNodeGetEnabled = __library
                .get(b"cudaGraphNodeGetEnabled\0")
                .ok()
                .map(|sym| *sym);
            let cudaGraphNodeGetType = __library
                .get(b"cudaGraphNodeGetType\0")
                .ok()
                .map(|sym| *sym);
            #[cfg(any(
                feature = "cuda-11060",
                feature = "cuda-11070",
                feature = "cuda-11080",
                feature = "cuda-12000",
                feature = "cuda-12010",
                feature = "cuda-12020",
                feature = "cuda-12030",
                feature = "cuda-12040",
                feature = "cuda-12050",
                feature = "cuda-12060",
                feature = "cuda-12080",
                feature = "cuda-12090",
                feature = "cuda-13000"
            ))]
            let cudaGraphNodeSetEnabled = __library
                .get(b"cudaGraphNodeSetEnabled\0")
                .ok()
                .map(|sym| *sym);
            #[cfg(any(
                feature = "cuda-12020",
                feature = "cuda-12030",
                feature = "cuda-12040",
                feature = "cuda-12050",
                feature = "cuda-12060",
                feature = "cuda-12080",
                feature = "cuda-12090",
                feature = "cuda-13000"
            ))]
            let cudaGraphNodeSetParams = __library
                .get(b"cudaGraphNodeSetParams\0")
                .ok()
                .map(|sym| *sym);
            let cudaGraphReleaseUserObject = __library
                .get(b"cudaGraphReleaseUserObject\0")
                .ok()
                .map(|sym| *sym);
            #[cfg(any(
                feature = "cuda-11040",
                feature = "cuda-11050",
                feature = "cuda-11060",
                feature = "cuda-11070",
                feature = "cuda-11080",
                feature = "cuda-12000",
                feature = "cuda-12010",
                feature = "cuda-12020",
                feature = "cuda-12030",
                feature = "cuda-12040",
                feature = "cuda-12050",
                feature = "cuda-12060",
                feature = "cuda-12080",
                feature = "cuda-12090"
            ))]
            let cudaGraphRemoveDependencies = __library
                .get(b"cudaGraphRemoveDependencies\0")
                .ok()
                .map(|sym| *sym);
            #[cfg(any(feature = "cuda-13000"))]
            let cudaGraphRemoveDependencies = __library
                .get(b"cudaGraphRemoveDependencies\0")
                .ok()
                .map(|sym| *sym);
            #[cfg(any(
                feature = "cuda-12030",
                feature = "cuda-12040",
                feature = "cuda-12050",
                feature = "cuda-12060",
                feature = "cuda-12080",
                feature = "cuda-12090"
            ))]
            let cudaGraphRemoveDependencies_v2 = __library
                .get(b"cudaGraphRemoveDependencies_v2\0")
                .ok()
                .map(|sym| *sym);
            let cudaGraphRetainUserObject = __library
                .get(b"cudaGraphRetainUserObject\0")
                .ok()
                .map(|sym| *sym);
            let cudaGraphUpload = __library.get(b"cudaGraphUpload\0").ok().map(|sym| *sym);
            let cudaGraphicsMapResources = __library
                .get(b"cudaGraphicsMapResources\0")
                .ok()
                .map(|sym| *sym);
            let cudaGraphicsResourceGetMappedMipmappedArray = __library
                .get(b"cudaGraphicsResourceGetMappedMipmappedArray\0")
                .ok()
                .map(|sym| *sym);
            let cudaGraphicsResourceGetMappedPointer = __library
                .get(b"cudaGraphicsResourceGetMappedPointer\0")
                .ok()
                .map(|sym| *sym);
            let cudaGraphicsResourceSetMapFlags = __library
                .get(b"cudaGraphicsResourceSetMapFlags\0")
                .ok()
                .map(|sym| *sym);
            let cudaGraphicsSubResourceGetMappedArray = __library
                .get(b"cudaGraphicsSubResourceGetMappedArray\0")
                .ok()
                .map(|sym| *sym);
            let cudaGraphicsUnmapResources = __library
                .get(b"cudaGraphicsUnmapResources\0")
                .ok()
                .map(|sym| *sym);
            let cudaGraphicsUnregisterResource = __library
                .get(b"cudaGraphicsUnregisterResource\0")
                .ok()
                .map(|sym| *sym);
            let cudaHostAlloc = __library.get(b"cudaHostAlloc\0").ok().map(|sym| *sym);
            let cudaHostGetDevicePointer = __library
                .get(b"cudaHostGetDevicePointer\0")
                .ok()
                .map(|sym| *sym);
            let cudaHostGetFlags = __library.get(b"cudaHostGetFlags\0").ok().map(|sym| *sym);
            let cudaHostRegister = __library.get(b"cudaHostRegister\0").ok().map(|sym| *sym);
            let cudaHostUnregister = __library.get(b"cudaHostUnregister\0").ok().map(|sym| *sym);
            let cudaImportExternalMemory = __library
                .get(b"cudaImportExternalMemory\0")
                .ok()
                .map(|sym| *sym);
            let cudaImportExternalSemaphore = __library
                .get(b"cudaImportExternalSemaphore\0")
                .ok()
                .map(|sym| *sym);
            #[cfg(any(
                feature = "cuda-12000",
                feature = "cuda-12010",
                feature = "cuda-12020",
                feature = "cuda-12030",
                feature = "cuda-12040",
                feature = "cuda-12050",
                feature = "cuda-12060",
                feature = "cuda-12080",
                feature = "cuda-12090",
                feature = "cuda-13000"
            ))]
            let cudaInitDevice = __library.get(b"cudaInitDevice\0").ok().map(|sym| *sym);
            let cudaIpcCloseMemHandle = __library
                .get(b"cudaIpcCloseMemHandle\0")
                .ok()
                .map(|sym| *sym);
            let cudaIpcGetEventHandle = __library
                .get(b"cudaIpcGetEventHandle\0")
                .ok()
                .map(|sym| *sym);
            let cudaIpcGetMemHandle = __library.get(b"cudaIpcGetMemHandle\0").ok().map(|sym| *sym);
            let cudaIpcOpenEventHandle = __library
                .get(b"cudaIpcOpenEventHandle\0")
                .ok()
                .map(|sym| *sym);
            let cudaIpcOpenMemHandle = __library
                .get(b"cudaIpcOpenMemHandle\0")
                .ok()
                .map(|sym| *sym);
            #[cfg(any(feature = "cuda-12080", feature = "cuda-12090", feature = "cuda-13000"))]
            let cudaKernelSetAttributeForDevice = __library
                .get(b"cudaKernelSetAttributeForDevice\0")
                .ok()
                .map(|sym| *sym);
            let cudaLaunchCooperativeKernel = __library
                .get(b"cudaLaunchCooperativeKernel\0")
                .ok()
                .map(|sym| *sym);
            #[cfg(any(
                feature = "cuda-11040",
                feature = "cuda-11050",
                feature = "cuda-11060",
                feature = "cuda-11070",
                feature = "cuda-11080",
                feature = "cuda-12000",
                feature = "cuda-12010",
                feature = "cuda-12020",
                feature = "cuda-12030",
                feature = "cuda-12040",
                feature = "cuda-12050",
                feature = "cuda-12060",
                feature = "cuda-12080",
                feature = "cuda-12090"
            ))]
            let cudaLaunchCooperativeKernelMultiDevice = __library
                .get(b"cudaLaunchCooperativeKernelMultiDevice\0")
                .ok()
                .map(|sym| *sym);
            let cudaLaunchHostFunc = __library.get(b"cudaLaunchHostFunc\0").ok().map(|sym| *sym);
            let cudaLaunchKernel = __library.get(b"cudaLaunchKernel\0").ok().map(|sym| *sym);
            #[cfg(any(
                feature = "cuda-11080",
                feature = "cuda-12000",
                feature = "cuda-12010",
                feature = "cuda-12020",
                feature = "cuda-12030",
                feature = "cuda-12040",
                feature = "cuda-12050",
                feature = "cuda-12060",
                feature = "cuda-12080",
                feature = "cuda-12090",
                feature = "cuda-13000"
            ))]
            let cudaLaunchKernelExC = __library.get(b"cudaLaunchKernelExC\0").ok().map(|sym| *sym);
            #[cfg(any(feature = "cuda-12080", feature = "cuda-12090", feature = "cuda-13000"))]
            let cudaLibraryEnumerateKernels = __library
                .get(b"cudaLibraryEnumerateKernels\0")
                .ok()
                .map(|sym| *sym);
            #[cfg(any(feature = "cuda-12080", feature = "cuda-12090", feature = "cuda-13000"))]
            let cudaLibraryGetGlobal = __library
                .get(b"cudaLibraryGetGlobal\0")
                .ok()
                .map(|sym| *sym);
            #[cfg(any(feature = "cuda-12080", feature = "cuda-12090", feature = "cuda-13000"))]
            let cudaLibraryGetKernel = __library
                .get(b"cudaLibraryGetKernel\0")
                .ok()
                .map(|sym| *sym);
            #[cfg(any(feature = "cuda-12080", feature = "cuda-12090", feature = "cuda-13000"))]
            let cudaLibraryGetKernelCount = __library
                .get(b"cudaLibraryGetKernelCount\0")
                .ok()
                .map(|sym| *sym);
            #[cfg(any(feature = "cuda-12080", feature = "cuda-12090", feature = "cuda-13000"))]
            let cudaLibraryGetManaged = __library
                .get(b"cudaLibraryGetManaged\0")
                .ok()
                .map(|sym| *sym);
            #[cfg(any(feature = "cuda-12080", feature = "cuda-12090", feature = "cuda-13000"))]
            let cudaLibraryGetUnifiedFunction = __library
                .get(b"cudaLibraryGetUnifiedFunction\0")
                .ok()
                .map(|sym| *sym);
            #[cfg(any(feature = "cuda-12080", feature = "cuda-12090", feature = "cuda-13000"))]
            let cudaLibraryLoadData = __library.get(b"cudaLibraryLoadData\0").ok().map(|sym| *sym);
            #[cfg(any(feature = "cuda-12080", feature = "cuda-12090", feature = "cuda-13000"))]
            let cudaLibraryLoadFromFile = __library
                .get(b"cudaLibraryLoadFromFile\0")
                .ok()
                .map(|sym| *sym);
            #[cfg(any(feature = "cuda-12080", feature = "cuda-12090", feature = "cuda-13000"))]
            let cudaLibraryUnload = __library.get(b"cudaLibraryUnload\0").ok().map(|sym| *sym);
            #[cfg(any(feature = "cuda-13000"))]
            let cudaLogsCurrent = __library.get(b"cudaLogsCurrent\0").ok().map(|sym| *sym);
            #[cfg(any(feature = "cuda-13000"))]
            let cudaLogsDumpToFile = __library.get(b"cudaLogsDumpToFile\0").ok().map(|sym| *sym);
            #[cfg(any(feature = "cuda-13000"))]
            let cudaLogsDumpToMemory = __library
                .get(b"cudaLogsDumpToMemory\0")
                .ok()
                .map(|sym| *sym);
            #[cfg(any(feature = "cuda-13000"))]
            let cudaLogsRegisterCallback = __library
                .get(b"cudaLogsRegisterCallback\0")
                .ok()
                .map(|sym| *sym);
            #[cfg(any(feature = "cuda-13000"))]
            let cudaLogsUnregisterCallback = __library
                .get(b"cudaLogsUnregisterCallback\0")
                .ok()
                .map(|sym| *sym);
            let cudaMalloc = __library.get(b"cudaMalloc\0").ok().map(|sym| *sym);
            let cudaMalloc3D = __library.get(b"cudaMalloc3D\0").ok().map(|sym| *sym);
            let cudaMalloc3DArray = __library.get(b"cudaMalloc3DArray\0").ok().map(|sym| *sym);
            let cudaMallocArray = __library.get(b"cudaMallocArray\0").ok().map(|sym| *sym);
            let cudaMallocAsync = __library.get(b"cudaMallocAsync\0").ok().map(|sym| *sym);
            let cudaMallocFromPoolAsync = __library
                .get(b"cudaMallocFromPoolAsync\0")
                .ok()
                .map(|sym| *sym);
            let cudaMallocHost = __library.get(b"cudaMallocHost\0").ok().map(|sym| *sym);
            let cudaMallocManaged = __library.get(b"cudaMallocManaged\0").ok().map(|sym| *sym);
            let cudaMallocMipmappedArray = __library
                .get(b"cudaMallocMipmappedArray\0")
                .ok()
                .map(|sym| *sym);
            let cudaMallocPitch = __library.get(b"cudaMallocPitch\0").ok().map(|sym| *sym);
            #[cfg(any(
                feature = "cuda-11040",
                feature = "cuda-11050",
                feature = "cuda-11060",
                feature = "cuda-11070",
                feature = "cuda-11080",
                feature = "cuda-12000",
                feature = "cuda-12010",
                feature = "cuda-12020",
                feature = "cuda-12030",
                feature = "cuda-12040",
                feature = "cuda-12050",
                feature = "cuda-12060",
                feature = "cuda-12080",
                feature = "cuda-12090"
            ))]
            let cudaMemAdvise = __library.get(b"cudaMemAdvise\0").ok().map(|sym| *sym);
            #[cfg(any(feature = "cuda-13000"))]
            let cudaMemAdvise = __library.get(b"cudaMemAdvise\0").ok().map(|sym| *sym);
            #[cfg(any(
                feature = "cuda-12020",
                feature = "cuda-12030",
                feature = "cuda-12040",
                feature = "cuda-12050",
                feature = "cuda-12060",
                feature = "cuda-12080",
                feature = "cuda-12090"
            ))]
            let cudaMemAdvise_v2 = __library.get(b"cudaMemAdvise_v2\0").ok().map(|sym| *sym);
            #[cfg(any(feature = "cuda-13000"))]
            let cudaMemDiscardAndPrefetchBatchAsync = __library
                .get(b"cudaMemDiscardAndPrefetchBatchAsync\0")
                .ok()
                .map(|sym| *sym);
            #[cfg(any(feature = "cuda-13000"))]
            let cudaMemDiscardBatchAsync = __library
                .get(b"cudaMemDiscardBatchAsync\0")
                .ok()
                .map(|sym| *sym);
            #[cfg(any(feature = "cuda-13000"))]
            let cudaMemGetDefaultMemPool = __library
                .get(b"cudaMemGetDefaultMemPool\0")
                .ok()
                .map(|sym| *sym);
            let cudaMemGetInfo = __library.get(b"cudaMemGetInfo\0").ok().map(|sym| *sym);
            #[cfg(any(feature = "cuda-13000"))]
            let cudaMemGetMemPool = __library.get(b"cudaMemGetMemPool\0").ok().map(|sym| *sym);
            let cudaMemPoolCreate = __library.get(b"cudaMemPoolCreate\0").ok().map(|sym| *sym);
            let cudaMemPoolDestroy = __library.get(b"cudaMemPoolDestroy\0").ok().map(|sym| *sym);
            let cudaMemPoolExportPointer = __library
                .get(b"cudaMemPoolExportPointer\0")
                .ok()
                .map(|sym| *sym);
            let cudaMemPoolExportToShareableHandle = __library
                .get(b"cudaMemPoolExportToShareableHandle\0")
                .ok()
                .map(|sym| *sym);
            let cudaMemPoolGetAccess = __library
                .get(b"cudaMemPoolGetAccess\0")
                .ok()
                .map(|sym| *sym);
            let cudaMemPoolGetAttribute = __library
                .get(b"cudaMemPoolGetAttribute\0")
                .ok()
                .map(|sym| *sym);
            let cudaMemPoolImportFromShareableHandle = __library
                .get(b"cudaMemPoolImportFromShareableHandle\0")
                .ok()
                .map(|sym| *sym);
            let cudaMemPoolImportPointer = __library
                .get(b"cudaMemPoolImportPointer\0")
                .ok()
                .map(|sym| *sym);
            let cudaMemPoolSetAccess = __library
                .get(b"cudaMemPoolSetAccess\0")
                .ok()
                .map(|sym| *sym);
            let cudaMemPoolSetAttribute = __library
                .get(b"cudaMemPoolSetAttribute\0")
                .ok()
                .map(|sym| *sym);
            let cudaMemPoolTrimTo = __library.get(b"cudaMemPoolTrimTo\0").ok().map(|sym| *sym);
            #[cfg(any(
                feature = "cuda-11040",
                feature = "cuda-11050",
                feature = "cuda-11060",
                feature = "cuda-11070",
                feature = "cuda-11080",
                feature = "cuda-12000",
                feature = "cuda-12010",
                feature = "cuda-12020",
                feature = "cuda-12030",
                feature = "cuda-12040",
                feature = "cuda-12050",
                feature = "cuda-12060",
                feature = "cuda-12080",
                feature = "cuda-12090"
            ))]
            let cudaMemPrefetchAsync = __library
                .get(b"cudaMemPrefetchAsync\0")
                .ok()
                .map(|sym| *sym);
            #[cfg(any(feature = "cuda-13000"))]
            let cudaMemPrefetchAsync = __library
                .get(b"cudaMemPrefetchAsync\0")
                .ok()
                .map(|sym| *sym);
            #[cfg(any(
                feature = "cuda-12020",
                feature = "cuda-12030",
                feature = "cuda-12040",
                feature = "cuda-12050",
                feature = "cuda-12060",
                feature = "cuda-12080",
                feature = "cuda-12090"
            ))]
            let cudaMemPrefetchAsync_v2 = __library
                .get(b"cudaMemPrefetchAsync_v2\0")
                .ok()
                .map(|sym| *sym);
            #[cfg(any(feature = "cuda-13000"))]
            let cudaMemPrefetchBatchAsync = __library
                .get(b"cudaMemPrefetchBatchAsync\0")
                .ok()
                .map(|sym| *sym);
            let cudaMemRangeGetAttribute = __library
                .get(b"cudaMemRangeGetAttribute\0")
                .ok()
                .map(|sym| *sym);
            let cudaMemRangeGetAttributes = __library
                .get(b"cudaMemRangeGetAttributes\0")
                .ok()
                .map(|sym| *sym);
            #[cfg(any(feature = "cuda-13000"))]
            let cudaMemSetMemPool = __library.get(b"cudaMemSetMemPool\0").ok().map(|sym| *sym);
            let cudaMemcpy = __library.get(b"cudaMemcpy\0").ok().map(|sym| *sym);
            let cudaMemcpy2D = __library.get(b"cudaMemcpy2D\0").ok().map(|sym| *sym);
            let cudaMemcpy2DArrayToArray = __library
                .get(b"cudaMemcpy2DArrayToArray\0")
                .ok()
                .map(|sym| *sym);
            let cudaMemcpy2DAsync = __library.get(b"cudaMemcpy2DAsync\0").ok().map(|sym| *sym);
            let cudaMemcpy2DFromArray = __library
                .get(b"cudaMemcpy2DFromArray\0")
                .ok()
                .map(|sym| *sym);
            let cudaMemcpy2DFromArrayAsync = __library
                .get(b"cudaMemcpy2DFromArrayAsync\0")
                .ok()
                .map(|sym| *sym);
            let cudaMemcpy2DToArray = __library.get(b"cudaMemcpy2DToArray\0").ok().map(|sym| *sym);
            let cudaMemcpy2DToArrayAsync = __library
                .get(b"cudaMemcpy2DToArrayAsync\0")
                .ok()
                .map(|sym| *sym);
            let cudaMemcpy3D = __library.get(b"cudaMemcpy3D\0").ok().map(|sym| *sym);
            let cudaMemcpy3DAsync = __library.get(b"cudaMemcpy3DAsync\0").ok().map(|sym| *sym);
            #[cfg(any(feature = "cuda-12080", feature = "cuda-12090"))]
            let cudaMemcpy3DBatchAsync = __library
                .get(b"cudaMemcpy3DBatchAsync\0")
                .ok()
                .map(|sym| *sym);
            #[cfg(any(feature = "cuda-13000"))]
            let cudaMemcpy3DBatchAsync = __library
                .get(b"cudaMemcpy3DBatchAsync\0")
                .ok()
                .map(|sym| *sym);
            let cudaMemcpy3DPeer = __library.get(b"cudaMemcpy3DPeer\0").ok().map(|sym| *sym);
            let cudaMemcpy3DPeerAsync = __library
                .get(b"cudaMemcpy3DPeerAsync\0")
                .ok()
                .map(|sym| *sym);
            let cudaMemcpyArrayToArray = __library
                .get(b"cudaMemcpyArrayToArray\0")
                .ok()
                .map(|sym| *sym);
            let cudaMemcpyAsync = __library.get(b"cudaMemcpyAsync\0").ok().map(|sym| *sym);
            #[cfg(any(feature = "cuda-12080", feature = "cuda-12090"))]
            let cudaMemcpyBatchAsync = __library
                .get(b"cudaMemcpyBatchAsync\0")
                .ok()
                .map(|sym| *sym);
            #[cfg(any(feature = "cuda-13000"))]
            let cudaMemcpyBatchAsync = __library
                .get(b"cudaMemcpyBatchAsync\0")
                .ok()
                .map(|sym| *sym);
            let cudaMemcpyFromArray = __library.get(b"cudaMemcpyFromArray\0").ok().map(|sym| *sym);
            let cudaMemcpyFromArrayAsync = __library
                .get(b"cudaMemcpyFromArrayAsync\0")
                .ok()
                .map(|sym| *sym);
            let cudaMemcpyFromSymbol = __library
                .get(b"cudaMemcpyFromSymbol\0")
                .ok()
                .map(|sym| *sym);
            let cudaMemcpyFromSymbolAsync = __library
                .get(b"cudaMemcpyFromSymbolAsync\0")
                .ok()
                .map(|sym| *sym);
            let cudaMemcpyPeer = __library.get(b"cudaMemcpyPeer\0").ok().map(|sym| *sym);
            let cudaMemcpyPeerAsync = __library.get(b"cudaMemcpyPeerAsync\0").ok().map(|sym| *sym);
            let cudaMemcpyToArray = __library.get(b"cudaMemcpyToArray\0").ok().map(|sym| *sym);
            let cudaMemcpyToArrayAsync = __library
                .get(b"cudaMemcpyToArrayAsync\0")
                .ok()
                .map(|sym| *sym);
            let cudaMemcpyToSymbol = __library.get(b"cudaMemcpyToSymbol\0").ok().map(|sym| *sym);
            let cudaMemcpyToSymbolAsync = __library
                .get(b"cudaMemcpyToSymbolAsync\0")
                .ok()
                .map(|sym| *sym);
            let cudaMemset = __library.get(b"cudaMemset\0").ok().map(|sym| *sym);
            let cudaMemset2D = __library.get(b"cudaMemset2D\0").ok().map(|sym| *sym);
            let cudaMemset2DAsync = __library.get(b"cudaMemset2DAsync\0").ok().map(|sym| *sym);
            let cudaMemset3D = __library.get(b"cudaMemset3D\0").ok().map(|sym| *sym);
            let cudaMemset3DAsync = __library.get(b"cudaMemset3DAsync\0").ok().map(|sym| *sym);
            let cudaMemsetAsync = __library.get(b"cudaMemsetAsync\0").ok().map(|sym| *sym);
            #[cfg(any(
                feature = "cuda-11060",
                feature = "cuda-11070",
                feature = "cuda-11080",
                feature = "cuda-12000",
                feature = "cuda-12010",
                feature = "cuda-12020",
                feature = "cuda-12030",
                feature = "cuda-12040",
                feature = "cuda-12050",
                feature = "cuda-12060",
                feature = "cuda-12080",
                feature = "cuda-12090",
                feature = "cuda-13000"
            ))]
            let cudaMipmappedArrayGetMemoryRequirements = __library
                .get(b"cudaMipmappedArrayGetMemoryRequirements\0")
                .ok()
                .map(|sym| *sym);
            let cudaMipmappedArrayGetSparseProperties = __library
                .get(b"cudaMipmappedArrayGetSparseProperties\0")
                .ok()
                .map(|sym| *sym);
            let cudaOccupancyAvailableDynamicSMemPerBlock = __library
                .get(b"cudaOccupancyAvailableDynamicSMemPerBlock\0")
                .ok()
                .map(|sym| *sym);
            let cudaOccupancyMaxActiveBlocksPerMultiprocessor = __library
                .get(b"cudaOccupancyMaxActiveBlocksPerMultiprocessor\0")
                .ok()
                .map(|sym| *sym);
            let cudaOccupancyMaxActiveBlocksPerMultiprocessorWithFlags = __library
                .get(b"cudaOccupancyMaxActiveBlocksPerMultiprocessorWithFlags\0")
                .ok()
                .map(|sym| *sym);
            #[cfg(any(
                feature = "cuda-11080",
                feature = "cuda-12000",
                feature = "cuda-12010",
                feature = "cuda-12020",
                feature = "cuda-12030",
                feature = "cuda-12040",
                feature = "cuda-12050",
                feature = "cuda-12060",
                feature = "cuda-12080",
                feature = "cuda-12090",
                feature = "cuda-13000"
            ))]
            let cudaOccupancyMaxActiveClusters = __library
                .get(b"cudaOccupancyMaxActiveClusters\0")
                .ok()
                .map(|sym| *sym);
            #[cfg(any(
                feature = "cuda-11080",
                feature = "cuda-12000",
                feature = "cuda-12010",
                feature = "cuda-12020",
                feature = "cuda-12030",
                feature = "cuda-12040",
                feature = "cuda-12050",
                feature = "cuda-12060",
                feature = "cuda-12080",
                feature = "cuda-12090",
                feature = "cuda-13000"
            ))]
            let cudaOccupancyMaxPotentialClusterSize = __library
                .get(b"cudaOccupancyMaxPotentialClusterSize\0")
                .ok()
                .map(|sym| *sym);
            let cudaPeekAtLastError = __library.get(b"cudaPeekAtLastError\0").ok().map(|sym| *sym);
            let cudaPointerGetAttributes = __library
                .get(b"cudaPointerGetAttributes\0")
                .ok()
                .map(|sym| *sym);
            let cudaProfilerStop = __library.get(b"cudaProfilerStop\0").ok().map(|sym| *sym);
            let cudaRuntimeGetVersion = __library
                .get(b"cudaRuntimeGetVersion\0")
                .ok()
                .map(|sym| *sym);
            let cudaSetDevice = __library.get(b"cudaSetDevice\0").ok().map(|sym| *sym);
            let cudaSetDeviceFlags = __library.get(b"cudaSetDeviceFlags\0").ok().map(|sym| *sym);
            #[cfg(any(
                feature = "cuda-11040",
                feature = "cuda-11050",
                feature = "cuda-11060",
                feature = "cuda-11070",
                feature = "cuda-11080",
                feature = "cuda-12000",
                feature = "cuda-12010",
                feature = "cuda-12020",
                feature = "cuda-12030",
                feature = "cuda-12040",
                feature = "cuda-12050",
                feature = "cuda-12060",
                feature = "cuda-12080",
                feature = "cuda-12090"
            ))]
            let cudaSetDoubleForDevice = __library
                .get(b"cudaSetDoubleForDevice\0")
                .ok()
                .map(|sym| *sym);
            #[cfg(any(
                feature = "cuda-11040",
                feature = "cuda-11050",
                feature = "cuda-11060",
                feature = "cuda-11070",
                feature = "cuda-11080",
                feature = "cuda-12000",
                feature = "cuda-12010",
                feature = "cuda-12020",
                feature = "cuda-12030",
                feature = "cuda-12040",
                feature = "cuda-12050",
                feature = "cuda-12060",
                feature = "cuda-12080",
                feature = "cuda-12090"
            ))]
            let cudaSetDoubleForHost = __library
                .get(b"cudaSetDoubleForHost\0")
                .ok()
                .map(|sym| *sym);
            let cudaSetValidDevices = __library.get(b"cudaSetValidDevices\0").ok().map(|sym| *sym);
            #[cfg(any(feature = "cuda-13000"))]
            let cudaSignalExternalSemaphoresAsync = __library
                .get(b"cudaSignalExternalSemaphoresAsync\0")
                .ok()
                .map(|sym| *sym);
            #[cfg(any(
                feature = "cuda-11040",
                feature = "cuda-11050",
                feature = "cuda-11060",
                feature = "cuda-11070",
                feature = "cuda-11080",
                feature = "cuda-12000",
                feature = "cuda-12010",
                feature = "cuda-12020",
                feature = "cuda-12030",
                feature = "cuda-12040",
                feature = "cuda-12050",
                feature = "cuda-12060",
                feature = "cuda-12080",
                feature = "cuda-12090"
            ))]
            let cudaSignalExternalSemaphoresAsync_v2 = __library
                .get(b"cudaSignalExternalSemaphoresAsync_v2\0")
                .ok()
                .map(|sym| *sym);
            let cudaStreamAddCallback = __library
                .get(b"cudaStreamAddCallback\0")
                .ok()
                .map(|sym| *sym);
            let cudaStreamAttachMemAsync = __library
                .get(b"cudaStreamAttachMemAsync\0")
                .ok()
                .map(|sym| *sym);
            let cudaStreamBeginCapture = __library
                .get(b"cudaStreamBeginCapture\0")
                .ok()
                .map(|sym| *sym);
            #[cfg(any(
                feature = "cuda-12030",
                feature = "cuda-12040",
                feature = "cuda-12050",
                feature = "cuda-12060",
                feature = "cuda-12080",
                feature = "cuda-12090",
                feature = "cuda-13000"
            ))]
            let cudaStreamBeginCaptureToGraph = __library
                .get(b"cudaStreamBeginCaptureToGraph\0")
                .ok()
                .map(|sym| *sym);
            let cudaStreamCopyAttributes = __library
                .get(b"cudaStreamCopyAttributes\0")
                .ok()
                .map(|sym| *sym);
            let cudaStreamCreate = __library.get(b"cudaStreamCreate\0").ok().map(|sym| *sym);
            let cudaStreamCreateWithFlags = __library
                .get(b"cudaStreamCreateWithFlags\0")
                .ok()
                .map(|sym| *sym);
            let cudaStreamCreateWithPriority = __library
                .get(b"cudaStreamCreateWithPriority\0")
                .ok()
                .map(|sym| *sym);
            let cudaStreamDestroy = __library.get(b"cudaStreamDestroy\0").ok().map(|sym| *sym);
            let cudaStreamEndCapture = __library
                .get(b"cudaStreamEndCapture\0")
                .ok()
                .map(|sym| *sym);
            #[cfg(any(
                feature = "cuda-11040",
                feature = "cuda-11050",
                feature = "cuda-11060",
                feature = "cuda-11070"
            ))]
            let cudaStreamGetAttribute = __library
                .get(b"cudaStreamGetAttribute\0")
                .ok()
                .map(|sym| *sym);
            #[cfg(any(
                feature = "cuda-11080",
                feature = "cuda-12000",
                feature = "cuda-12010",
                feature = "cuda-12020",
                feature = "cuda-12030",
                feature = "cuda-12040",
                feature = "cuda-12050",
                feature = "cuda-12060",
                feature = "cuda-12080",
                feature = "cuda-12090",
                feature = "cuda-13000"
            ))]
            let cudaStreamGetAttribute = __library
                .get(b"cudaStreamGetAttribute\0")
                .ok()
                .map(|sym| *sym);
            #[cfg(any(
                feature = "cuda-11040",
                feature = "cuda-11050",
                feature = "cuda-11060",
                feature = "cuda-11070",
                feature = "cuda-11080"
            ))]
            let cudaStreamGetCaptureInfo = __library
                .get(b"cudaStreamGetCaptureInfo\0")
                .ok()
                .map(|sym| *sym);
            #[cfg(any(feature = "cuda-13000"))]
            let cudaStreamGetCaptureInfo = __library
                .get(b"cudaStreamGetCaptureInfo\0")
                .ok()
                .map(|sym| *sym);
            #[cfg(any(
                feature = "cuda-11040",
                feature = "cuda-11050",
                feature = "cuda-11060",
                feature = "cuda-11070",
                feature = "cuda-11080",
                feature = "cuda-12000",
                feature = "cuda-12010",
                feature = "cuda-12020",
                feature = "cuda-12030",
                feature = "cuda-12040",
                feature = "cuda-12050",
                feature = "cuda-12060",
                feature = "cuda-12080",
                feature = "cuda-12090"
            ))]
            let cudaStreamGetCaptureInfo_v2 = __library
                .get(b"cudaStreamGetCaptureInfo_v2\0")
                .ok()
                .map(|sym| *sym);
            #[cfg(any(
                feature = "cuda-12030",
                feature = "cuda-12040",
                feature = "cuda-12050",
                feature = "cuda-12060",
                feature = "cuda-12080",
                feature = "cuda-12090"
            ))]
            let cudaStreamGetCaptureInfo_v3 = __library
                .get(b"cudaStreamGetCaptureInfo_v3\0")
                .ok()
                .map(|sym| *sym);
            #[cfg(any(feature = "cuda-12080", feature = "cuda-12090", feature = "cuda-13000"))]
            let cudaStreamGetDevice = __library.get(b"cudaStreamGetDevice\0").ok().map(|sym| *sym);
            let cudaStreamGetFlags = __library.get(b"cudaStreamGetFlags\0").ok().map(|sym| *sym);
            #[cfg(any(
                feature = "cuda-12000",
                feature = "cuda-12010",
                feature = "cuda-12020",
                feature = "cuda-12030",
                feature = "cuda-12040",
                feature = "cuda-12050",
                feature = "cuda-12060",
                feature = "cuda-12080",
                feature = "cuda-12090",
                feature = "cuda-13000"
            ))]
            let cudaStreamGetId = __library.get(b"cudaStreamGetId\0").ok().map(|sym| *sym);
            let cudaStreamGetPriority = __library
                .get(b"cudaStreamGetPriority\0")
                .ok()
                .map(|sym| *sym);
            let cudaStreamIsCapturing = __library
                .get(b"cudaStreamIsCapturing\0")
                .ok()
                .map(|sym| *sym);
            let cudaStreamQuery = __library.get(b"cudaStreamQuery\0").ok().map(|sym| *sym);
            #[cfg(any(
                feature = "cuda-11040",
                feature = "cuda-11050",
                feature = "cuda-11060",
                feature = "cuda-11070"
            ))]
            let cudaStreamSetAttribute = __library
                .get(b"cudaStreamSetAttribute\0")
                .ok()
                .map(|sym| *sym);
            #[cfg(any(
                feature = "cuda-11080",
                feature = "cuda-12000",
                feature = "cuda-12010",
                feature = "cuda-12020",
                feature = "cuda-12030",
                feature = "cuda-12040",
                feature = "cuda-12050",
                feature = "cuda-12060",
                feature = "cuda-12080",
                feature = "cuda-12090",
                feature = "cuda-13000"
            ))]
            let cudaStreamSetAttribute = __library
                .get(b"cudaStreamSetAttribute\0")
                .ok()
                .map(|sym| *sym);
            let cudaStreamSynchronize = __library
                .get(b"cudaStreamSynchronize\0")
                .ok()
                .map(|sym| *sym);
            #[cfg(any(
                feature = "cuda-11040",
                feature = "cuda-11050",
                feature = "cuda-11060",
                feature = "cuda-11070",
                feature = "cuda-11080",
                feature = "cuda-12000",
                feature = "cuda-12010",
                feature = "cuda-12020",
                feature = "cuda-12030",
                feature = "cuda-12040",
                feature = "cuda-12050",
                feature = "cuda-12060",
                feature = "cuda-12080",
                feature = "cuda-12090"
            ))]
            let cudaStreamUpdateCaptureDependencies = __library
                .get(b"cudaStreamUpdateCaptureDependencies\0")
                .ok()
                .map(|sym| *sym);
            #[cfg(any(feature = "cuda-13000"))]
            let cudaStreamUpdateCaptureDependencies = __library
                .get(b"cudaStreamUpdateCaptureDependencies\0")
                .ok()
                .map(|sym| *sym);
            #[cfg(any(
                feature = "cuda-12030",
                feature = "cuda-12040",
                feature = "cuda-12050",
                feature = "cuda-12060",
                feature = "cuda-12080",
                feature = "cuda-12090"
            ))]
            let cudaStreamUpdateCaptureDependencies_v2 = __library
                .get(b"cudaStreamUpdateCaptureDependencies_v2\0")
                .ok()
                .map(|sym| *sym);
            let cudaStreamWaitEvent = __library.get(b"cudaStreamWaitEvent\0").ok().map(|sym| *sym);
            let cudaThreadExchangeStreamCaptureMode = __library
                .get(b"cudaThreadExchangeStreamCaptureMode\0")
                .ok()
                .map(|sym| *sym);
            #[cfg(any(
                feature = "cuda-11040",
                feature = "cuda-11050",
                feature = "cuda-11060",
                feature = "cuda-11070",
                feature = "cuda-11080",
                feature = "cuda-12000",
                feature = "cuda-12010",
                feature = "cuda-12020",
                feature = "cuda-12030",
                feature = "cuda-12040",
                feature = "cuda-12050",
                feature = "cuda-12060",
                feature = "cuda-12080",
                feature = "cuda-12090"
            ))]
            let cudaThreadExit = __library.get(b"cudaThreadExit\0").ok().map(|sym| *sym);
            #[cfg(any(
                feature = "cuda-11040",
                feature = "cuda-11050",
                feature = "cuda-11060",
                feature = "cuda-11070",
                feature = "cuda-11080",
                feature = "cuda-12000",
                feature = "cuda-12010",
                feature = "cuda-12020",
                feature = "cuda-12030",
                feature = "cuda-12040",
                feature = "cuda-12050",
                feature = "cuda-12060",
                feature = "cuda-12080",
                feature = "cuda-12090"
            ))]
            let cudaThreadGetCacheConfig = __library
                .get(b"cudaThreadGetCacheConfig\0")
                .ok()
                .map(|sym| *sym);
            #[cfg(any(
                feature = "cuda-11040",
                feature = "cuda-11050",
                feature = "cuda-11060",
                feature = "cuda-11070",
                feature = "cuda-11080",
                feature = "cuda-12000",
                feature = "cuda-12010",
                feature = "cuda-12020",
                feature = "cuda-12030",
                feature = "cuda-12040",
                feature = "cuda-12050",
                feature = "cuda-12060",
                feature = "cuda-12080",
                feature = "cuda-12090"
            ))]
            let cudaThreadGetLimit = __library.get(b"cudaThreadGetLimit\0").ok().map(|sym| *sym);
            #[cfg(any(
                feature = "cuda-11040",
                feature = "cuda-11050",
                feature = "cuda-11060",
                feature = "cuda-11070",
                feature = "cuda-11080",
                feature = "cuda-12000",
                feature = "cuda-12010",
                feature = "cuda-12020",
                feature = "cuda-12030",
                feature = "cuda-12040",
                feature = "cuda-12050",
                feature = "cuda-12060",
                feature = "cuda-12080",
                feature = "cuda-12090"
            ))]
            let cudaThreadSetCacheConfig = __library
                .get(b"cudaThreadSetCacheConfig\0")
                .ok()
                .map(|sym| *sym);
            #[cfg(any(
                feature = "cuda-11040",
                feature = "cuda-11050",
                feature = "cuda-11060",
                feature = "cuda-11070",
                feature = "cuda-11080",
                feature = "cuda-12000",
                feature = "cuda-12010",
                feature = "cuda-12020",
                feature = "cuda-12030",
                feature = "cuda-12040",
                feature = "cuda-12050",
                feature = "cuda-12060",
                feature = "cuda-12080",
                feature = "cuda-12090"
            ))]
            let cudaThreadSetLimit = __library.get(b"cudaThreadSetLimit\0").ok().map(|sym| *sym);
            #[cfg(any(
                feature = "cuda-11040",
                feature = "cuda-11050",
                feature = "cuda-11060",
                feature = "cuda-11070",
                feature = "cuda-11080",
                feature = "cuda-12000",
                feature = "cuda-12010",
                feature = "cuda-12020",
                feature = "cuda-12030",
                feature = "cuda-12040",
                feature = "cuda-12050",
                feature = "cuda-12060",
                feature = "cuda-12080",
                feature = "cuda-12090"
            ))]
            let cudaThreadSynchronize = __library
                .get(b"cudaThreadSynchronize\0")
                .ok()
                .map(|sym| *sym);
            #[cfg(any(
                feature = "cuda-11040",
                feature = "cuda-11050",
                feature = "cuda-11060",
                feature = "cuda-11070",
                feature = "cuda-11080"
            ))]
            let cudaUnbindTexture = __library.get(b"cudaUnbindTexture\0").ok().map(|sym| *sym);
            let cudaUserObjectCreate = __library
                .get(b"cudaUserObjectCreate\0")
                .ok()
                .map(|sym| *sym);
            let cudaUserObjectRelease = __library
                .get(b"cudaUserObjectRelease\0")
                .ok()
                .map(|sym| *sym);
            let cudaUserObjectRetain = __library
                .get(b"cudaUserObjectRetain\0")
                .ok()
                .map(|sym| *sym);
            #[cfg(any(feature = "cuda-13000"))]
            let cudaWaitExternalSemaphoresAsync = __library
                .get(b"cudaWaitExternalSemaphoresAsync\0")
                .ok()
                .map(|sym| *sym);
            #[cfg(any(
                feature = "cuda-11040",
                feature = "cuda-11050",
                feature = "cuda-11060",
                feature = "cuda-11070",
                feature = "cuda-11080",
                feature = "cuda-12000",
                feature = "cuda-12010",
                feature = "cuda-12020",
                feature = "cuda-12030",
                feature = "cuda-12040",
                feature = "cuda-12050",
                feature = "cuda-12060",
                feature = "cuda-12080",
                feature = "cuda-12090"
            ))]
            let cudaWaitExternalSemaphoresAsync_v2 = __library
                .get(b"cudaWaitExternalSemaphoresAsync_v2\0")
                .ok()
                .map(|sym| *sym);
            Ok(Self {
                __library,
                cudaArrayGetInfo,
                #[cfg(any(
                    feature = "cuda-11060",
                    feature = "cuda-11070",
                    feature = "cuda-11080",
                    feature = "cuda-12000",
                    feature = "cuda-12010",
                    feature = "cuda-12020",
                    feature = "cuda-12030",
                    feature = "cuda-12040",
                    feature = "cuda-12050",
                    feature = "cuda-12060",
                    feature = "cuda-12080",
                    feature = "cuda-12090",
                    feature = "cuda-13000"
                ))]
                cudaArrayGetMemoryRequirements,
                cudaArrayGetPlane,
                cudaArrayGetSparseProperties,
                #[cfg(any(
                    feature = "cuda-11040",
                    feature = "cuda-11050",
                    feature = "cuda-11060",
                    feature = "cuda-11070",
                    feature = "cuda-11080"
                ))]
                cudaBindSurfaceToArray,
                #[cfg(any(
                    feature = "cuda-11040",
                    feature = "cuda-11050",
                    feature = "cuda-11060",
                    feature = "cuda-11070",
                    feature = "cuda-11080"
                ))]
                cudaBindTexture,
                #[cfg(any(
                    feature = "cuda-11040",
                    feature = "cuda-11050",
                    feature = "cuda-11060",
                    feature = "cuda-11070",
                    feature = "cuda-11080"
                ))]
                cudaBindTexture2D,
                #[cfg(any(
                    feature = "cuda-11040",
                    feature = "cuda-11050",
                    feature = "cuda-11060",
                    feature = "cuda-11070",
                    feature = "cuda-11080"
                ))]
                cudaBindTextureToArray,
                #[cfg(any(
                    feature = "cuda-11040",
                    feature = "cuda-11050",
                    feature = "cuda-11060",
                    feature = "cuda-11070",
                    feature = "cuda-11080"
                ))]
                cudaBindTextureToMipmappedArray,
                cudaChooseDevice,
                cudaCreateChannelDesc,
                cudaCreateSurfaceObject,
                cudaCreateTextureObject,
                #[cfg(any(feature = "cuda-11080"))]
                cudaCreateTextureObject_v2,
                cudaCtxResetPersistingL2Cache,
                cudaDestroyExternalMemory,
                cudaDestroyExternalSemaphore,
                cudaDestroySurfaceObject,
                cudaDestroyTextureObject,
                cudaDeviceCanAccessPeer,
                cudaDeviceDisablePeerAccess,
                cudaDeviceEnablePeerAccess,
                cudaDeviceFlushGPUDirectRDMAWrites,
                cudaDeviceGetAttribute,
                cudaDeviceGetByPCIBusId,
                cudaDeviceGetCacheConfig,
                cudaDeviceGetDefaultMemPool,
                cudaDeviceGetGraphMemAttribute,
                #[cfg(any(feature = "cuda-13000"))]
                cudaDeviceGetHostAtomicCapabilities,
                cudaDeviceGetLimit,
                cudaDeviceGetMemPool,
                #[cfg(any(feature = "cuda-13000"))]
                cudaDeviceGetP2PAtomicCapabilities,
                cudaDeviceGetP2PAttribute,
                cudaDeviceGetPCIBusId,
                cudaDeviceGetSharedMemConfig,
                cudaDeviceGetStreamPriorityRange,
                cudaDeviceGetTexture1DLinearMaxWidth,
                cudaDeviceGraphMemTrim,
                #[cfg(any(
                    feature = "cuda-12040",
                    feature = "cuda-12050",
                    feature = "cuda-12060",
                    feature = "cuda-12080",
                    feature = "cuda-12090",
                    feature = "cuda-13000"
                ))]
                cudaDeviceRegisterAsyncNotification,
                cudaDeviceReset,
                cudaDeviceSetCacheConfig,
                cudaDeviceSetGraphMemAttribute,
                cudaDeviceSetLimit,
                cudaDeviceSetMemPool,
                cudaDeviceSetSharedMemConfig,
                cudaDeviceSynchronize,
                #[cfg(any(
                    feature = "cuda-12040",
                    feature = "cuda-12050",
                    feature = "cuda-12060",
                    feature = "cuda-12080",
                    feature = "cuda-12090",
                    feature = "cuda-13000"
                ))]
                cudaDeviceUnregisterAsyncNotification,
                cudaDriverGetVersion,
                cudaEventCreate,
                cudaEventCreateWithFlags,
                cudaEventDestroy,
                cudaEventElapsedTime,
                #[cfg(any(feature = "cuda-12080", feature = "cuda-12090"))]
                cudaEventElapsedTime_v2,
                cudaEventQuery,
                cudaEventRecord,
                cudaEventRecordWithFlags,
                cudaEventSynchronize,
                cudaExternalMemoryGetMappedBuffer,
                cudaExternalMemoryGetMappedMipmappedArray,
                cudaFree,
                cudaFreeArray,
                cudaFreeAsync,
                cudaFreeHost,
                cudaFreeMipmappedArray,
                cudaFuncGetAttributes,
                #[cfg(any(
                    feature = "cuda-12030",
                    feature = "cuda-12040",
                    feature = "cuda-12050",
                    feature = "cuda-12060",
                    feature = "cuda-12080",
                    feature = "cuda-12090",
                    feature = "cuda-13000"
                ))]
                cudaFuncGetName,
                #[cfg(any(
                    feature = "cuda-12040",
                    feature = "cuda-12050",
                    feature = "cuda-12060",
                    feature = "cuda-12080",
                    feature = "cuda-12090",
                    feature = "cuda-13000"
                ))]
                cudaFuncGetParamInfo,
                cudaFuncSetAttribute,
                cudaFuncSetCacheConfig,
                cudaFuncSetSharedMemConfig,
                cudaGetChannelDesc,
                cudaGetDevice,
                cudaGetDeviceCount,
                cudaGetDeviceFlags,
                #[cfg(any(
                    feature = "cuda-11040",
                    feature = "cuda-11050",
                    feature = "cuda-11060",
                    feature = "cuda-11070",
                    feature = "cuda-11080",
                    feature = "cuda-13000"
                ))]
                cudaGetDeviceProperties,
                #[cfg(any(
                    feature = "cuda-12000",
                    feature = "cuda-12010",
                    feature = "cuda-12020",
                    feature = "cuda-12030",
                    feature = "cuda-12040",
                    feature = "cuda-12050",
                    feature = "cuda-12060",
                    feature = "cuda-12080",
                    feature = "cuda-12090"
                ))]
                cudaGetDeviceProperties_v2,
                #[cfg(any(
                    feature = "cuda-11040",
                    feature = "cuda-11050",
                    feature = "cuda-11060",
                    feature = "cuda-11070",
                    feature = "cuda-11080"
                ))]
                cudaGetDriverEntryPoint,
                #[cfg(any(
                    feature = "cuda-12000",
                    feature = "cuda-12010",
                    feature = "cuda-12020",
                    feature = "cuda-12030",
                    feature = "cuda-12040",
                    feature = "cuda-12050",
                    feature = "cuda-12060",
                    feature = "cuda-12080",
                    feature = "cuda-12090",
                    feature = "cuda-13000"
                ))]
                cudaGetDriverEntryPoint,
                #[cfg(any(
                    feature = "cuda-12050",
                    feature = "cuda-12060",
                    feature = "cuda-12080",
                    feature = "cuda-12090",
                    feature = "cuda-13000"
                ))]
                cudaGetDriverEntryPointByVersion,
                cudaGetErrorName,
                cudaGetErrorString,
                cudaGetExportTable,
                cudaGetFuncBySymbol,
                #[cfg(any(
                    feature = "cuda-12010",
                    feature = "cuda-12020",
                    feature = "cuda-12030",
                    feature = "cuda-12040",
                    feature = "cuda-12050",
                    feature = "cuda-12060",
                    feature = "cuda-12080",
                    feature = "cuda-12090",
                    feature = "cuda-13000"
                ))]
                cudaGetKernel,
                cudaGetLastError,
                cudaGetMipmappedArrayLevel,
                cudaGetSurfaceObjectResourceDesc,
                #[cfg(any(
                    feature = "cuda-11040",
                    feature = "cuda-11050",
                    feature = "cuda-11060",
                    feature = "cuda-11070",
                    feature = "cuda-11080"
                ))]
                cudaGetSurfaceReference,
                cudaGetSymbolAddress,
                cudaGetSymbolSize,
                #[cfg(any(
                    feature = "cuda-11040",
                    feature = "cuda-11050",
                    feature = "cuda-11060",
                    feature = "cuda-11070",
                    feature = "cuda-11080"
                ))]
                cudaGetTextureAlignmentOffset,
                cudaGetTextureObjectResourceDesc,
                cudaGetTextureObjectResourceViewDesc,
                cudaGetTextureObjectTextureDesc,
                #[cfg(any(feature = "cuda-11080"))]
                cudaGetTextureObjectTextureDesc_v2,
                #[cfg(any(
                    feature = "cuda-11040",
                    feature = "cuda-11050",
                    feature = "cuda-11060",
                    feature = "cuda-11070",
                    feature = "cuda-11080"
                ))]
                cudaGetTextureReference,
                cudaGraphAddChildGraphNode,
                #[cfg(any(
                    feature = "cuda-11040",
                    feature = "cuda-11050",
                    feature = "cuda-11060",
                    feature = "cuda-11070",
                    feature = "cuda-11080",
                    feature = "cuda-12000",
                    feature = "cuda-12010",
                    feature = "cuda-12020",
                    feature = "cuda-12030",
                    feature = "cuda-12040",
                    feature = "cuda-12050",
                    feature = "cuda-12060",
                    feature = "cuda-12080",
                    feature = "cuda-12090"
                ))]
                cudaGraphAddDependencies,
                #[cfg(any(feature = "cuda-13000"))]
                cudaGraphAddDependencies,
                #[cfg(any(
                    feature = "cuda-12030",
                    feature = "cuda-12040",
                    feature = "cuda-12050",
                    feature = "cuda-12060",
                    feature = "cuda-12080",
                    feature = "cuda-12090"
                ))]
                cudaGraphAddDependencies_v2,
                cudaGraphAddEmptyNode,
                cudaGraphAddEventRecordNode,
                cudaGraphAddEventWaitNode,
                cudaGraphAddExternalSemaphoresSignalNode,
                cudaGraphAddExternalSemaphoresWaitNode,
                cudaGraphAddHostNode,
                cudaGraphAddKernelNode,
                cudaGraphAddMemAllocNode,
                cudaGraphAddMemFreeNode,
                cudaGraphAddMemcpyNode,
                cudaGraphAddMemcpyNode1D,
                cudaGraphAddMemcpyNodeFromSymbol,
                cudaGraphAddMemcpyNodeToSymbol,
                cudaGraphAddMemsetNode,
                #[cfg(any(
                    feature = "cuda-12020",
                    feature = "cuda-12030",
                    feature = "cuda-12040",
                    feature = "cuda-12050",
                    feature = "cuda-12060",
                    feature = "cuda-12080",
                    feature = "cuda-12090"
                ))]
                cudaGraphAddNode,
                #[cfg(any(feature = "cuda-13000"))]
                cudaGraphAddNode,
                #[cfg(any(
                    feature = "cuda-12030",
                    feature = "cuda-12040",
                    feature = "cuda-12050",
                    feature = "cuda-12060",
                    feature = "cuda-12080",
                    feature = "cuda-12090"
                ))]
                cudaGraphAddNode_v2,
                cudaGraphChildGraphNodeGetGraph,
                cudaGraphClone,
                #[cfg(any(
                    feature = "cuda-12030",
                    feature = "cuda-12040",
                    feature = "cuda-12050",
                    feature = "cuda-12060",
                    feature = "cuda-12080",
                    feature = "cuda-12090",
                    feature = "cuda-13000"
                ))]
                cudaGraphConditionalHandleCreate,
                cudaGraphCreate,
                cudaGraphDebugDotPrint,
                cudaGraphDestroy,
                cudaGraphDestroyNode,
                cudaGraphEventRecordNodeGetEvent,
                cudaGraphEventRecordNodeSetEvent,
                cudaGraphEventWaitNodeGetEvent,
                cudaGraphEventWaitNodeSetEvent,
                cudaGraphExecChildGraphNodeSetParams,
                cudaGraphExecDestroy,
                cudaGraphExecEventRecordNodeSetEvent,
                cudaGraphExecEventWaitNodeSetEvent,
                cudaGraphExecExternalSemaphoresSignalNodeSetParams,
                cudaGraphExecExternalSemaphoresWaitNodeSetParams,
                #[cfg(any(
                    feature = "cuda-12000",
                    feature = "cuda-12010",
                    feature = "cuda-12020",
                    feature = "cuda-12030",
                    feature = "cuda-12040",
                    feature = "cuda-12050",
                    feature = "cuda-12060",
                    feature = "cuda-12080",
                    feature = "cuda-12090",
                    feature = "cuda-13000"
                ))]
                cudaGraphExecGetFlags,
                cudaGraphExecHostNodeSetParams,
                cudaGraphExecKernelNodeSetParams,
                cudaGraphExecMemcpyNodeSetParams,
                cudaGraphExecMemcpyNodeSetParams1D,
                cudaGraphExecMemcpyNodeSetParamsFromSymbol,
                cudaGraphExecMemcpyNodeSetParamsToSymbol,
                cudaGraphExecMemsetNodeSetParams,
                #[cfg(any(
                    feature = "cuda-12020",
                    feature = "cuda-12030",
                    feature = "cuda-12040",
                    feature = "cuda-12050",
                    feature = "cuda-12060",
                    feature = "cuda-12080",
                    feature = "cuda-12090",
                    feature = "cuda-13000"
                ))]
                cudaGraphExecNodeSetParams,
                #[cfg(any(
                    feature = "cuda-11040",
                    feature = "cuda-11050",
                    feature = "cuda-11060",
                    feature = "cuda-11070",
                    feature = "cuda-11080"
                ))]
                cudaGraphExecUpdate,
                #[cfg(any(
                    feature = "cuda-12000",
                    feature = "cuda-12010",
                    feature = "cuda-12020",
                    feature = "cuda-12030",
                    feature = "cuda-12040",
                    feature = "cuda-12050",
                    feature = "cuda-12060",
                    feature = "cuda-12080",
                    feature = "cuda-12090",
                    feature = "cuda-13000"
                ))]
                cudaGraphExecUpdate,
                cudaGraphExternalSemaphoresSignalNodeGetParams,
                cudaGraphExternalSemaphoresSignalNodeSetParams,
                cudaGraphExternalSemaphoresWaitNodeGetParams,
                cudaGraphExternalSemaphoresWaitNodeSetParams,
                #[cfg(any(
                    feature = "cuda-11040",
                    feature = "cuda-11050",
                    feature = "cuda-11060",
                    feature = "cuda-11070",
                    feature = "cuda-11080",
                    feature = "cuda-12000",
                    feature = "cuda-12010",
                    feature = "cuda-12020",
                    feature = "cuda-12030",
                    feature = "cuda-12040",
                    feature = "cuda-12050",
                    feature = "cuda-12060",
                    feature = "cuda-12080",
                    feature = "cuda-12090"
                ))]
                cudaGraphGetEdges,
                #[cfg(any(feature = "cuda-13000"))]
                cudaGraphGetEdges,
                #[cfg(any(
                    feature = "cuda-12030",
                    feature = "cuda-12040",
                    feature = "cuda-12050",
                    feature = "cuda-12060",
                    feature = "cuda-12080",
                    feature = "cuda-12090"
                ))]
                cudaGraphGetEdges_v2,
                cudaGraphGetNodes,
                cudaGraphGetRootNodes,
                cudaGraphHostNodeGetParams,
                cudaGraphHostNodeSetParams,
                #[cfg(any(
                    feature = "cuda-11040",
                    feature = "cuda-11050",
                    feature = "cuda-11060",
                    feature = "cuda-11070",
                    feature = "cuda-11080"
                ))]
                cudaGraphInstantiate,
                #[cfg(any(
                    feature = "cuda-12000",
                    feature = "cuda-12010",
                    feature = "cuda-12020",
                    feature = "cuda-12030",
                    feature = "cuda-12040",
                    feature = "cuda-12050",
                    feature = "cuda-12060",
                    feature = "cuda-12080",
                    feature = "cuda-12090",
                    feature = "cuda-13000"
                ))]
                cudaGraphInstantiate,
                cudaGraphInstantiateWithFlags,
                #[cfg(any(
                    feature = "cuda-12000",
                    feature = "cuda-12010",
                    feature = "cuda-12020",
                    feature = "cuda-12030",
                    feature = "cuda-12040",
                    feature = "cuda-12050",
                    feature = "cuda-12060",
                    feature = "cuda-12080",
                    feature = "cuda-12090",
                    feature = "cuda-13000"
                ))]
                cudaGraphInstantiateWithParams,
                #[cfg(any(
                    feature = "cuda-11040",
                    feature = "cuda-11050",
                    feature = "cuda-11060",
                    feature = "cuda-11070",
                    feature = "cuda-11080",
                    feature = "cuda-12000",
                    feature = "cuda-12010",
                    feature = "cuda-12020",
                    feature = "cuda-12030",
                    feature = "cuda-12040",
                    feature = "cuda-12050",
                    feature = "cuda-12060",
                    feature = "cuda-12080",
                    feature = "cuda-12090"
                ))]
                cudaGraphKernelNodeCopyAttributes,
                #[cfg(any(feature = "cuda-13000"))]
                cudaGraphKernelNodeCopyAttributes,
                #[cfg(any(
                    feature = "cuda-11040",
                    feature = "cuda-11050",
                    feature = "cuda-11060",
                    feature = "cuda-11070"
                ))]
                cudaGraphKernelNodeGetAttribute,
                #[cfg(any(
                    feature = "cuda-11080",
                    feature = "cuda-12000",
                    feature = "cuda-12010",
                    feature = "cuda-12020",
                    feature = "cuda-12030",
                    feature = "cuda-12040",
                    feature = "cuda-12050",
                    feature = "cuda-12060",
                    feature = "cuda-12080",
                    feature = "cuda-12090",
                    feature = "cuda-13000"
                ))]
                cudaGraphKernelNodeGetAttribute,
                cudaGraphKernelNodeGetParams,
                #[cfg(any(
                    feature = "cuda-11040",
                    feature = "cuda-11050",
                    feature = "cuda-11060",
                    feature = "cuda-11070"
                ))]
                cudaGraphKernelNodeSetAttribute,
                #[cfg(any(
                    feature = "cuda-11080",
                    feature = "cuda-12000",
                    feature = "cuda-12010",
                    feature = "cuda-12020",
                    feature = "cuda-12030",
                    feature = "cuda-12040",
                    feature = "cuda-12050",
                    feature = "cuda-12060",
                    feature = "cuda-12080",
                    feature = "cuda-12090",
                    feature = "cuda-13000"
                ))]
                cudaGraphKernelNodeSetAttribute,
                cudaGraphKernelNodeSetParams,
                cudaGraphLaunch,
                cudaGraphMemAllocNodeGetParams,
                cudaGraphMemFreeNodeGetParams,
                cudaGraphMemcpyNodeGetParams,
                cudaGraphMemcpyNodeSetParams,
                cudaGraphMemcpyNodeSetParams1D,
                cudaGraphMemcpyNodeSetParamsFromSymbol,
                cudaGraphMemcpyNodeSetParamsToSymbol,
                cudaGraphMemsetNodeGetParams,
                cudaGraphMemsetNodeSetParams,
                cudaGraphNodeFindInClone,
                #[cfg(any(
                    feature = "cuda-11040",
                    feature = "cuda-11050",
                    feature = "cuda-11060",
                    feature = "cuda-11070",
                    feature = "cuda-11080",
                    feature = "cuda-12000",
                    feature = "cuda-12010",
                    feature = "cuda-12020",
                    feature = "cuda-12030",
                    feature = "cuda-12040",
                    feature = "cuda-12050",
                    feature = "cuda-12060",
                    feature = "cuda-12080",
                    feature = "cuda-12090"
                ))]
                cudaGraphNodeGetDependencies,
                #[cfg(any(feature = "cuda-13000"))]
                cudaGraphNodeGetDependencies,
                #[cfg(any(
                    feature = "cuda-12030",
                    feature = "cuda-12040",
                    feature = "cuda-12050",
                    feature = "cuda-12060",
                    feature = "cuda-12080",
                    feature = "cuda-12090"
                ))]
                cudaGraphNodeGetDependencies_v2,
                #[cfg(any(
                    feature = "cuda-11040",
                    feature = "cuda-11050",
                    feature = "cuda-11060",
                    feature = "cuda-11070",
                    feature = "cuda-11080",
                    feature = "cuda-12000",
                    feature = "cuda-12010",
                    feature = "cuda-12020",
                    feature = "cuda-12030",
                    feature = "cuda-12040",
                    feature = "cuda-12050",
                    feature = "cuda-12060",
                    feature = "cuda-12080",
                    feature = "cuda-12090"
                ))]
                cudaGraphNodeGetDependentNodes,
                #[cfg(any(feature = "cuda-13000"))]
                cudaGraphNodeGetDependentNodes,
                #[cfg(any(
                    feature = "cuda-12030",
                    feature = "cuda-12040",
                    feature = "cuda-12050",
                    feature = "cuda-12060",
                    feature = "cuda-12080",
                    feature = "cuda-12090"
                ))]
                cudaGraphNodeGetDependentNodes_v2,
                #[cfg(any(
                    feature = "cuda-11060",
                    feature = "cuda-11070",
                    feature = "cuda-11080",
                    feature = "cuda-12000",
                    feature = "cuda-12010",
                    feature = "cuda-12020",
                    feature = "cuda-12030",
                    feature = "cuda-12040",
                    feature = "cuda-12050",
                    feature = "cuda-12060",
                    feature = "cuda-12080",
                    feature = "cuda-12090",
                    feature = "cuda-13000"
                ))]
                cudaGraphNodeGetEnabled,
                cudaGraphNodeGetType,
                #[cfg(any(
                    feature = "cuda-11060",
                    feature = "cuda-11070",
                    feature = "cuda-11080",
                    feature = "cuda-12000",
                    feature = "cuda-12010",
                    feature = "cuda-12020",
                    feature = "cuda-12030",
                    feature = "cuda-12040",
                    feature = "cuda-12050",
                    feature = "cuda-12060",
                    feature = "cuda-12080",
                    feature = "cuda-12090",
                    feature = "cuda-13000"
                ))]
                cudaGraphNodeSetEnabled,
                #[cfg(any(
                    feature = "cuda-12020",
                    feature = "cuda-12030",
                    feature = "cuda-12040",
                    feature = "cuda-12050",
                    feature = "cuda-12060",
                    feature = "cuda-12080",
                    feature = "cuda-12090",
                    feature = "cuda-13000"
                ))]
                cudaGraphNodeSetParams,
                cudaGraphReleaseUserObject,
                #[cfg(any(
                    feature = "cuda-11040",
                    feature = "cuda-11050",
                    feature = "cuda-11060",
                    feature = "cuda-11070",
                    feature = "cuda-11080",
                    feature = "cuda-12000",
                    feature = "cuda-12010",
                    feature = "cuda-12020",
                    feature = "cuda-12030",
                    feature = "cuda-12040",
                    feature = "cuda-12050",
                    feature = "cuda-12060",
                    feature = "cuda-12080",
                    feature = "cuda-12090"
                ))]
                cudaGraphRemoveDependencies,
                #[cfg(any(feature = "cuda-13000"))]
                cudaGraphRemoveDependencies,
                #[cfg(any(
                    feature = "cuda-12030",
                    feature = "cuda-12040",
                    feature = "cuda-12050",
                    feature = "cuda-12060",
                    feature = "cuda-12080",
                    feature = "cuda-12090"
                ))]
                cudaGraphRemoveDependencies_v2,
                cudaGraphRetainUserObject,
                cudaGraphUpload,
                cudaGraphicsMapResources,
                cudaGraphicsResourceGetMappedMipmappedArray,
                cudaGraphicsResourceGetMappedPointer,
                cudaGraphicsResourceSetMapFlags,
                cudaGraphicsSubResourceGetMappedArray,
                cudaGraphicsUnmapResources,
                cudaGraphicsUnregisterResource,
                cudaHostAlloc,
                cudaHostGetDevicePointer,
                cudaHostGetFlags,
                cudaHostRegister,
                cudaHostUnregister,
                cudaImportExternalMemory,
                cudaImportExternalSemaphore,
                #[cfg(any(
                    feature = "cuda-12000",
                    feature = "cuda-12010",
                    feature = "cuda-12020",
                    feature = "cuda-12030",
                    feature = "cuda-12040",
                    feature = "cuda-12050",
                    feature = "cuda-12060",
                    feature = "cuda-12080",
                    feature = "cuda-12090",
                    feature = "cuda-13000"
                ))]
                cudaInitDevice,
                cudaIpcCloseMemHandle,
                cudaIpcGetEventHandle,
                cudaIpcGetMemHandle,
                cudaIpcOpenEventHandle,
                cudaIpcOpenMemHandle,
                #[cfg(any(
                    feature = "cuda-12080",
                    feature = "cuda-12090",
                    feature = "cuda-13000"
                ))]
                cudaKernelSetAttributeForDevice,
                cudaLaunchCooperativeKernel,
                #[cfg(any(
                    feature = "cuda-11040",
                    feature = "cuda-11050",
                    feature = "cuda-11060",
                    feature = "cuda-11070",
                    feature = "cuda-11080",
                    feature = "cuda-12000",
                    feature = "cuda-12010",
                    feature = "cuda-12020",
                    feature = "cuda-12030",
                    feature = "cuda-12040",
                    feature = "cuda-12050",
                    feature = "cuda-12060",
                    feature = "cuda-12080",
                    feature = "cuda-12090"
                ))]
                cudaLaunchCooperativeKernelMultiDevice,
                cudaLaunchHostFunc,
                cudaLaunchKernel,
                #[cfg(any(
                    feature = "cuda-11080",
                    feature = "cuda-12000",
                    feature = "cuda-12010",
                    feature = "cuda-12020",
                    feature = "cuda-12030",
                    feature = "cuda-12040",
                    feature = "cuda-12050",
                    feature = "cuda-12060",
                    feature = "cuda-12080",
                    feature = "cuda-12090",
                    feature = "cuda-13000"
                ))]
                cudaLaunchKernelExC,
                #[cfg(any(
                    feature = "cuda-12080",
                    feature = "cuda-12090",
                    feature = "cuda-13000"
                ))]
                cudaLibraryEnumerateKernels,
                #[cfg(any(
                    feature = "cuda-12080",
                    feature = "cuda-12090",
                    feature = "cuda-13000"
                ))]
                cudaLibraryGetGlobal,
                #[cfg(any(
                    feature = "cuda-12080",
                    feature = "cuda-12090",
                    feature = "cuda-13000"
                ))]
                cudaLibraryGetKernel,
                #[cfg(any(
                    feature = "cuda-12080",
                    feature = "cuda-12090",
                    feature = "cuda-13000"
                ))]
                cudaLibraryGetKernelCount,
                #[cfg(any(
                    feature = "cuda-12080",
                    feature = "cuda-12090",
                    feature = "cuda-13000"
                ))]
                cudaLibraryGetManaged,
                #[cfg(any(
                    feature = "cuda-12080",
                    feature = "cuda-12090",
                    feature = "cuda-13000"
                ))]
                cudaLibraryGetUnifiedFunction,
                #[cfg(any(
                    feature = "cuda-12080",
                    feature = "cuda-12090",
                    feature = "cuda-13000"
                ))]
                cudaLibraryLoadData,
                #[cfg(any(
                    feature = "cuda-12080",
                    feature = "cuda-12090",
                    feature = "cuda-13000"
                ))]
                cudaLibraryLoadFromFile,
                #[cfg(any(
                    feature = "cuda-12080",
                    feature = "cuda-12090",
                    feature = "cuda-13000"
                ))]
                cudaLibraryUnload,
                #[cfg(any(feature = "cuda-13000"))]
                cudaLogsCurrent,
                #[cfg(any(feature = "cuda-13000"))]
                cudaLogsDumpToFile,
                #[cfg(any(feature = "cuda-13000"))]
                cudaLogsDumpToMemory,
                #[cfg(any(feature = "cuda-13000"))]
                cudaLogsRegisterCallback,
                #[cfg(any(feature = "cuda-13000"))]
                cudaLogsUnregisterCallback,
                cudaMalloc,
                cudaMalloc3D,
                cudaMalloc3DArray,
                cudaMallocArray,
                cudaMallocAsync,
                cudaMallocFromPoolAsync,
                cudaMallocHost,
                cudaMallocManaged,
                cudaMallocMipmappedArray,
                cudaMallocPitch,
                #[cfg(any(
                    feature = "cuda-11040",
                    feature = "cuda-11050",
                    feature = "cuda-11060",
                    feature = "cuda-11070",
                    feature = "cuda-11080",
                    feature = "cuda-12000",
                    feature = "cuda-12010",
                    feature = "cuda-12020",
                    feature = "cuda-12030",
                    feature = "cuda-12040",
                    feature = "cuda-12050",
                    feature = "cuda-12060",
                    feature = "cuda-12080",
                    feature = "cuda-12090"
                ))]
                cudaMemAdvise,
                #[cfg(any(feature = "cuda-13000"))]
                cudaMemAdvise,
                #[cfg(any(
                    feature = "cuda-12020",
                    feature = "cuda-12030",
                    feature = "cuda-12040",
                    feature = "cuda-12050",
                    feature = "cuda-12060",
                    feature = "cuda-12080",
                    feature = "cuda-12090"
                ))]
                cudaMemAdvise_v2,
                #[cfg(any(feature = "cuda-13000"))]
                cudaMemDiscardAndPrefetchBatchAsync,
                #[cfg(any(feature = "cuda-13000"))]
                cudaMemDiscardBatchAsync,
                #[cfg(any(feature = "cuda-13000"))]
                cudaMemGetDefaultMemPool,
                cudaMemGetInfo,
                #[cfg(any(feature = "cuda-13000"))]
                cudaMemGetMemPool,
                cudaMemPoolCreate,
                cudaMemPoolDestroy,
                cudaMemPoolExportPointer,
                cudaMemPoolExportToShareableHandle,
                cudaMemPoolGetAccess,
                cudaMemPoolGetAttribute,
                cudaMemPoolImportFromShareableHandle,
                cudaMemPoolImportPointer,
                cudaMemPoolSetAccess,
                cudaMemPoolSetAttribute,
                cudaMemPoolTrimTo,
                #[cfg(any(
                    feature = "cuda-11040",
                    feature = "cuda-11050",
                    feature = "cuda-11060",
                    feature = "cuda-11070",
                    feature = "cuda-11080",
                    feature = "cuda-12000",
                    feature = "cuda-12010",
                    feature = "cuda-12020",
                    feature = "cuda-12030",
                    feature = "cuda-12040",
                    feature = "cuda-12050",
                    feature = "cuda-12060",
                    feature = "cuda-12080",
                    feature = "cuda-12090"
                ))]
                cudaMemPrefetchAsync,
                #[cfg(any(feature = "cuda-13000"))]
                cudaMemPrefetchAsync,
                #[cfg(any(
                    feature = "cuda-12020",
                    feature = "cuda-12030",
                    feature = "cuda-12040",
                    feature = "cuda-12050",
                    feature = "cuda-12060",
                    feature = "cuda-12080",
                    feature = "cuda-12090"
                ))]
                cudaMemPrefetchAsync_v2,
                #[cfg(any(feature = "cuda-13000"))]
                cudaMemPrefetchBatchAsync,
                cudaMemRangeGetAttribute,
                cudaMemRangeGetAttributes,
                #[cfg(any(feature = "cuda-13000"))]
                cudaMemSetMemPool,
                cudaMemcpy,
                cudaMemcpy2D,
                cudaMemcpy2DArrayToArray,
                cudaMemcpy2DAsync,
                cudaMemcpy2DFromArray,
                cudaMemcpy2DFromArrayAsync,
                cudaMemcpy2DToArray,
                cudaMemcpy2DToArrayAsync,
                cudaMemcpy3D,
                cudaMemcpy3DAsync,
                #[cfg(any(feature = "cuda-12080", feature = "cuda-12090"))]
                cudaMemcpy3DBatchAsync,
                #[cfg(any(feature = "cuda-13000"))]
                cudaMemcpy3DBatchAsync,
                cudaMemcpy3DPeer,
                cudaMemcpy3DPeerAsync,
                cudaMemcpyArrayToArray,
                cudaMemcpyAsync,
                #[cfg(any(feature = "cuda-12080", feature = "cuda-12090"))]
                cudaMemcpyBatchAsync,
                #[cfg(any(feature = "cuda-13000"))]
                cudaMemcpyBatchAsync,
                cudaMemcpyFromArray,
                cudaMemcpyFromArrayAsync,
                cudaMemcpyFromSymbol,
                cudaMemcpyFromSymbolAsync,
                cudaMemcpyPeer,
                cudaMemcpyPeerAsync,
                cudaMemcpyToArray,
                cudaMemcpyToArrayAsync,
                cudaMemcpyToSymbol,
                cudaMemcpyToSymbolAsync,
                cudaMemset,
                cudaMemset2D,
                cudaMemset2DAsync,
                cudaMemset3D,
                cudaMemset3DAsync,
                cudaMemsetAsync,
                #[cfg(any(
                    feature = "cuda-11060",
                    feature = "cuda-11070",
                    feature = "cuda-11080",
                    feature = "cuda-12000",
                    feature = "cuda-12010",
                    feature = "cuda-12020",
                    feature = "cuda-12030",
                    feature = "cuda-12040",
                    feature = "cuda-12050",
                    feature = "cuda-12060",
                    feature = "cuda-12080",
                    feature = "cuda-12090",
                    feature = "cuda-13000"
                ))]
                cudaMipmappedArrayGetMemoryRequirements,
                cudaMipmappedArrayGetSparseProperties,
                cudaOccupancyAvailableDynamicSMemPerBlock,
                cudaOccupancyMaxActiveBlocksPerMultiprocessor,
                cudaOccupancyMaxActiveBlocksPerMultiprocessorWithFlags,
                #[cfg(any(
                    feature = "cuda-11080",
                    feature = "cuda-12000",
                    feature = "cuda-12010",
                    feature = "cuda-12020",
                    feature = "cuda-12030",
                    feature = "cuda-12040",
                    feature = "cuda-12050",
                    feature = "cuda-12060",
                    feature = "cuda-12080",
                    feature = "cuda-12090",
                    feature = "cuda-13000"
                ))]
                cudaOccupancyMaxActiveClusters,
                #[cfg(any(
                    feature = "cuda-11080",
                    feature = "cuda-12000",
                    feature = "cuda-12010",
                    feature = "cuda-12020",
                    feature = "cuda-12030",
                    feature = "cuda-12040",
                    feature = "cuda-12050",
                    feature = "cuda-12060",
                    feature = "cuda-12080",
                    feature = "cuda-12090",
                    feature = "cuda-13000"
                ))]
                cudaOccupancyMaxPotentialClusterSize,
                cudaPeekAtLastError,
                cudaPointerGetAttributes,
                cudaProfilerStop,
                cudaRuntimeGetVersion,
                cudaSetDevice,
                cudaSetDeviceFlags,
                #[cfg(any(
                    feature = "cuda-11040",
                    feature = "cuda-11050",
                    feature = "cuda-11060",
                    feature = "cuda-11070",
                    feature = "cuda-11080",
                    feature = "cuda-12000",
                    feature = "cuda-12010",
                    feature = "cuda-12020",
                    feature = "cuda-12030",
                    feature = "cuda-12040",
                    feature = "cuda-12050",
                    feature = "cuda-12060",
                    feature = "cuda-12080",
                    feature = "cuda-12090"
                ))]
                cudaSetDoubleForDevice,
                #[cfg(any(
                    feature = "cuda-11040",
                    feature = "cuda-11050",
                    feature = "cuda-11060",
                    feature = "cuda-11070",
                    feature = "cuda-11080",
                    feature = "cuda-12000",
                    feature = "cuda-12010",
                    feature = "cuda-12020",
                    feature = "cuda-12030",
                    feature = "cuda-12040",
                    feature = "cuda-12050",
                    feature = "cuda-12060",
                    feature = "cuda-12080",
                    feature = "cuda-12090"
                ))]
                cudaSetDoubleForHost,
                cudaSetValidDevices,
                #[cfg(any(feature = "cuda-13000"))]
                cudaSignalExternalSemaphoresAsync,
                #[cfg(any(
                    feature = "cuda-11040",
                    feature = "cuda-11050",
                    feature = "cuda-11060",
                    feature = "cuda-11070",
                    feature = "cuda-11080",
                    feature = "cuda-12000",
                    feature = "cuda-12010",
                    feature = "cuda-12020",
                    feature = "cuda-12030",
                    feature = "cuda-12040",
                    feature = "cuda-12050",
                    feature = "cuda-12060",
                    feature = "cuda-12080",
                    feature = "cuda-12090"
                ))]
                cudaSignalExternalSemaphoresAsync_v2,
                cudaStreamAddCallback,
                cudaStreamAttachMemAsync,
                cudaStreamBeginCapture,
                #[cfg(any(
                    feature = "cuda-12030",
                    feature = "cuda-12040",
                    feature = "cuda-12050",
                    feature = "cuda-12060",
                    feature = "cuda-12080",
                    feature = "cuda-12090",
                    feature = "cuda-13000"
                ))]
                cudaStreamBeginCaptureToGraph,
                cudaStreamCopyAttributes,
                cudaStreamCreate,
                cudaStreamCreateWithFlags,
                cudaStreamCreateWithPriority,
                cudaStreamDestroy,
                cudaStreamEndCapture,
                #[cfg(any(
                    feature = "cuda-11040",
                    feature = "cuda-11050",
                    feature = "cuda-11060",
                    feature = "cuda-11070"
                ))]
                cudaStreamGetAttribute,
                #[cfg(any(
                    feature = "cuda-11080",
                    feature = "cuda-12000",
                    feature = "cuda-12010",
                    feature = "cuda-12020",
                    feature = "cuda-12030",
                    feature = "cuda-12040",
                    feature = "cuda-12050",
                    feature = "cuda-12060",
                    feature = "cuda-12080",
                    feature = "cuda-12090",
                    feature = "cuda-13000"
                ))]
                cudaStreamGetAttribute,
                #[cfg(any(
                    feature = "cuda-11040",
                    feature = "cuda-11050",
                    feature = "cuda-11060",
                    feature = "cuda-11070",
                    feature = "cuda-11080"
                ))]
                cudaStreamGetCaptureInfo,
                #[cfg(any(feature = "cuda-13000"))]
                cudaStreamGetCaptureInfo,
                #[cfg(any(
                    feature = "cuda-11040",
                    feature = "cuda-11050",
                    feature = "cuda-11060",
                    feature = "cuda-11070",
                    feature = "cuda-11080",
                    feature = "cuda-12000",
                    feature = "cuda-12010",
                    feature = "cuda-12020",
                    feature = "cuda-12030",
                    feature = "cuda-12040",
                    feature = "cuda-12050",
                    feature = "cuda-12060",
                    feature = "cuda-12080",
                    feature = "cuda-12090"
                ))]
                cudaStreamGetCaptureInfo_v2,
                #[cfg(any(
                    feature = "cuda-12030",
                    feature = "cuda-12040",
                    feature = "cuda-12050",
                    feature = "cuda-12060",
                    feature = "cuda-12080",
                    feature = "cuda-12090"
                ))]
                cudaStreamGetCaptureInfo_v3,
                #[cfg(any(
                    feature = "cuda-12080",
                    feature = "cuda-12090",
                    feature = "cuda-13000"
                ))]
                cudaStreamGetDevice,
                cudaStreamGetFlags,
                #[cfg(any(
                    feature = "cuda-12000",
                    feature = "cuda-12010",
                    feature = "cuda-12020",
                    feature = "cuda-12030",
                    feature = "cuda-12040",
                    feature = "cuda-12050",
                    feature = "cuda-12060",
                    feature = "cuda-12080",
                    feature = "cuda-12090",
                    feature = "cuda-13000"
                ))]
                cudaStreamGetId,
                cudaStreamGetPriority,
                cudaStreamIsCapturing,
                cudaStreamQuery,
                #[cfg(any(
                    feature = "cuda-11040",
                    feature = "cuda-11050",
                    feature = "cuda-11060",
                    feature = "cuda-11070"
                ))]
                cudaStreamSetAttribute,
                #[cfg(any(
                    feature = "cuda-11080",
                    feature = "cuda-12000",
                    feature = "cuda-12010",
                    feature = "cuda-12020",
                    feature = "cuda-12030",
                    feature = "cuda-12040",
                    feature = "cuda-12050",
                    feature = "cuda-12060",
                    feature = "cuda-12080",
                    feature = "cuda-12090",
                    feature = "cuda-13000"
                ))]
                cudaStreamSetAttribute,
                cudaStreamSynchronize,
                #[cfg(any(
                    feature = "cuda-11040",
                    feature = "cuda-11050",
                    feature = "cuda-11060",
                    feature = "cuda-11070",
                    feature = "cuda-11080",
                    feature = "cuda-12000",
                    feature = "cuda-12010",
                    feature = "cuda-12020",
                    feature = "cuda-12030",
                    feature = "cuda-12040",
                    feature = "cuda-12050",
                    feature = "cuda-12060",
                    feature = "cuda-12080",
                    feature = "cuda-12090"
                ))]
                cudaStreamUpdateCaptureDependencies,
                #[cfg(any(feature = "cuda-13000"))]
                cudaStreamUpdateCaptureDependencies,
                #[cfg(any(
                    feature = "cuda-12030",
                    feature = "cuda-12040",
                    feature = "cuda-12050",
                    feature = "cuda-12060",
                    feature = "cuda-12080",
                    feature = "cuda-12090"
                ))]
                cudaStreamUpdateCaptureDependencies_v2,
                cudaStreamWaitEvent,
                cudaThreadExchangeStreamCaptureMode,
                #[cfg(any(
                    feature = "cuda-11040",
                    feature = "cuda-11050",
                    feature = "cuda-11060",
                    feature = "cuda-11070",
                    feature = "cuda-11080",
                    feature = "cuda-12000",
                    feature = "cuda-12010",
                    feature = "cuda-12020",
                    feature = "cuda-12030",
                    feature = "cuda-12040",
                    feature = "cuda-12050",
                    feature = "cuda-12060",
                    feature = "cuda-12080",
                    feature = "cuda-12090"
                ))]
                cudaThreadExit,
                #[cfg(any(
                    feature = "cuda-11040",
                    feature = "cuda-11050",
                    feature = "cuda-11060",
                    feature = "cuda-11070",
                    feature = "cuda-11080",
                    feature = "cuda-12000",
                    feature = "cuda-12010",
                    feature = "cuda-12020",
                    feature = "cuda-12030",
                    feature = "cuda-12040",
                    feature = "cuda-12050",
                    feature = "cuda-12060",
                    feature = "cuda-12080",
                    feature = "cuda-12090"
                ))]
                cudaThreadGetCacheConfig,
                #[cfg(any(
                    feature = "cuda-11040",
                    feature = "cuda-11050",
                    feature = "cuda-11060",
                    feature = "cuda-11070",
                    feature = "cuda-11080",
                    feature = "cuda-12000",
                    feature = "cuda-12010",
                    feature = "cuda-12020",
                    feature = "cuda-12030",
                    feature = "cuda-12040",
                    feature = "cuda-12050",
                    feature = "cuda-12060",
                    feature = "cuda-12080",
                    feature = "cuda-12090"
                ))]
                cudaThreadGetLimit,
                #[cfg(any(
                    feature = "cuda-11040",
                    feature = "cuda-11050",
                    feature = "cuda-11060",
                    feature = "cuda-11070",
                    feature = "cuda-11080",
                    feature = "cuda-12000",
                    feature = "cuda-12010",
                    feature = "cuda-12020",
                    feature = "cuda-12030",
                    feature = "cuda-12040",
                    feature = "cuda-12050",
                    feature = "cuda-12060",
                    feature = "cuda-12080",
                    feature = "cuda-12090"
                ))]
                cudaThreadSetCacheConfig,
                #[cfg(any(
                    feature = "cuda-11040",
                    feature = "cuda-11050",
                    feature = "cuda-11060",
                    feature = "cuda-11070",
                    feature = "cuda-11080",
                    feature = "cuda-12000",
                    feature = "cuda-12010",
                    feature = "cuda-12020",
                    feature = "cuda-12030",
                    feature = "cuda-12040",
                    feature = "cuda-12050",
                    feature = "cuda-12060",
                    feature = "cuda-12080",
                    feature = "cuda-12090"
                ))]
                cudaThreadSetLimit,
                #[cfg(any(
                    feature = "cuda-11040",
                    feature = "cuda-11050",
                    feature = "cuda-11060",
                    feature = "cuda-11070",
                    feature = "cuda-11080",
                    feature = "cuda-12000",
                    feature = "cuda-12010",
                    feature = "cuda-12020",
                    feature = "cuda-12030",
                    feature = "cuda-12040",
                    feature = "cuda-12050",
                    feature = "cuda-12060",
                    feature = "cuda-12080",
                    feature = "cuda-12090"
                ))]
                cudaThreadSynchronize,
                #[cfg(any(
                    feature = "cuda-11040",
                    feature = "cuda-11050",
                    feature = "cuda-11060",
                    feature = "cuda-11070",
                    feature = "cuda-11080"
                ))]
                cudaUnbindTexture,
                cudaUserObjectCreate,
                cudaUserObjectRelease,
                cudaUserObjectRetain,
                #[cfg(any(feature = "cuda-13000"))]
                cudaWaitExternalSemaphoresAsync,
                #[cfg(any(
                    feature = "cuda-11040",
                    feature = "cuda-11050",
                    feature = "cuda-11060",
                    feature = "cuda-11070",
                    feature = "cuda-11080",
                    feature = "cuda-12000",
                    feature = "cuda-12010",
                    feature = "cuda-12020",
                    feature = "cuda-12030",
                    feature = "cuda-12040",
                    feature = "cuda-12050",
                    feature = "cuda-12060",
                    feature = "cuda-12080",
                    feature = "cuda-12090"
                ))]
                cudaWaitExternalSemaphoresAsync_v2,
            })
        }
    }
    pub unsafe fn is_culib_present() -> bool {
        let lib_names = ["cudart"];
        let choices = lib_names
            .iter()
            .map(|l| crate::get_lib_name_candidates(l))
            .flatten();
        for choice in choices {
            if Lib::new(choice).is_ok() {
                return true;
            }
        }
        false
    }
    pub unsafe fn culib() -> &'static Lib {
        static LIB: std::sync::OnceLock<Lib> = std::sync::OnceLock::new();
        LIB.get_or_init(|| {
            let lib_names = std::vec!["cudart"];
            let choices: std::vec::Vec<_> = lib_names
                .iter()
                .map(|l| crate::get_lib_name_candidates(l))
                .flatten()
                .collect();
            for choice in choices.iter() {
                if let Ok(lib) = Lib::new(choice) {
                    return lib;
                }
            }
            crate::panic_no_lib_found(lib_names[0], &choices);
        })
    }
}
#[cfg(feature = "dynamic-loading")]
pub use loaded::*;
