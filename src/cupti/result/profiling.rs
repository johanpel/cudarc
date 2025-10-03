//! Functions of the Profiling API

use super::super::{result::CuptiError, sys};

/// Replay API: used for multipass collection.
pub unsafe fn begin_pass(
    params_ptr: *mut sys::CUpti_Profiler_BeginPass_Params,
) -> Result<(), CuptiError> {
    unsafe { sys::cuptiProfilerBeginPass(params_ptr) }.result()
}
/// Begin profiling session sets up the profiling on the device.
pub unsafe fn begin_session(
    params_ptr: *mut sys::CUpti_Profiler_BeginSession_Params,
) -> Result<(), CuptiError> {
    unsafe { sys::cuptiProfilerBeginSession(params_ptr) }.result()
}
/// A temporary storage for CounterData image needed for internal operations.
pub unsafe fn counter_data_image_calculate_scratch_buffer_size(
    params_ptr: *mut sys::CUpti_Profiler_CounterDataImage_CalculateScratchBufferSize_Params,
) -> Result<(), CuptiError> {
    unsafe { sys::cuptiProfilerCounterDataImageCalculateScratchBufferSize(params_ptr) }.result()
}
/// A CounterData image allocates space for values for each counter for each range.
pub unsafe fn counter_data_image_calculate_size(
    params_ptr: *mut sys::CUpti_Profiler_CounterDataImage_CalculateSize_Params,
) -> Result<(), CuptiError> {
    unsafe { sys::cuptiProfilerCounterDataImageCalculateSize(params_ptr) }.result()
}
/// Undocumented
pub unsafe fn counter_data_image_initialize(
    params_ptr: *mut sys::CUpti_Profiler_CounterDataImage_Initialize_Params,
) -> Result<(), CuptiError> {
    unsafe { sys::cuptiProfilerCounterDataImageInitialize(params_ptr) }.result()
}
/// Undocumented
pub unsafe fn counter_data_image_initialize_scratch_buffer(
    params_ptr: *mut sys::CUpti_Profiler_CounterDataImage_InitializeScratchBuffer_Params,
) -> Result<(), CuptiError> {
    unsafe { sys::cuptiProfilerCounterDataImageInitializeScratchBuffer(params_ptr) }.result()
}
/// DeInitializes the profiler interface.
pub unsafe fn de_initialize(
    params_ptr: *mut sys::CUpti_Profiler_DeInitialize_Params,
) -> Result<(), CuptiError> {
    unsafe { sys::cuptiProfilerDeInitialize(params_ptr) }.result()
}
/// Query device compatibility with Profiling API.
pub unsafe fn device_supported(
    params_ptr: *mut sys::CUpti_Profiler_DeviceSupported_Params,
) -> Result<(), CuptiError> {
    unsafe { sys::cuptiProfilerDeviceSupported(params_ptr) }.result()
}
/// Disable Profiling.
pub unsafe fn disable_profiling(
    params_ptr: *mut sys::CUpti_Profiler_DisableProfiling_Params,
) -> Result<(), CuptiError> {
    unsafe { sys::cuptiProfilerDisableProfiling(params_ptr) }.result()
}
/// Enables Profiling.
pub unsafe fn enable_profiling(
    params_ptr: *mut sys::CUpti_Profiler_EnableProfiling_Params,
) -> Result<(), CuptiError> {
    unsafe { sys::cuptiProfilerEnableProfiling(params_ptr) }.result()
}
/// Replay API: used for multipass collection.
pub unsafe fn end_pass(
    params_ptr: *mut sys::CUpti_Profiler_EndPass_Params,
) -> Result<(), CuptiError> {
    unsafe { sys::cuptiProfilerEndPass(params_ptr) }.result()
}
/// Ends profiling session.
pub unsafe fn end_session(
    params_ptr: *mut sys::CUpti_Profiler_EndSession_Params,
) -> Result<(), CuptiError> {
    unsafe { sys::cuptiProfilerEndSession(params_ptr) }.result()
}
/// Decode all the submitted passes.
pub unsafe fn flush_counter_data(
    params_ptr: *mut sys::CUpti_Profiler_FlushCounterData_Params,
) -> Result<(), CuptiError> {
    unsafe { sys::cuptiProfilerFlushCounterData(params_ptr) }.result()
}
/// Query counter availibility.
pub unsafe fn get_counter_availability(
    params_ptr: *mut sys::CUpti_Profiler_GetCounterAvailability_Params,
) -> Result<(), CuptiError> {
    unsafe { sys::cuptiProfilerGetCounterAvailability(params_ptr) }.result()
}
/// Initializes the profiler interface.
pub unsafe fn initialize(
    params_ptr: *mut sys::CUpti_Profiler_Initialize_Params,
) -> Result<(), CuptiError> {
    unsafe { sys::cuptiProfilerInitialize(params_ptr) }.result()
}
/// Asynchronous call to query if the submitted pass to GPU is collected.
pub unsafe fn is_pass_collected(
    params_ptr: *mut sys::CUpti_Profiler_IsPassCollected_Params,
) -> Result<(), CuptiError> {
    unsafe { sys::cuptiProfilerIsPassCollected(params_ptr) }.result()
}
/// Range API's : Pop user range.
pub unsafe fn pop_range(
    params_ptr: *mut sys::CUpti_Profiler_PopRange_Params,
) -> Result<(), CuptiError> {
    unsafe { sys::cuptiProfilerPopRange(params_ptr) }.result()
}
/// Range API's : Push user range.
pub unsafe fn push_range(
    params_ptr: *mut sys::CUpti_Profiler_PushRange_Params,
) -> Result<(), CuptiError> {
    unsafe { sys::cuptiProfilerPushRange(params_ptr) }.result()
}
/// Set metrics configuration to be profiled.
pub unsafe fn set_config(
    params_ptr: *mut sys::CUpti_Profiler_SetConfig_Params,
) -> Result<(), CuptiError> {
    unsafe { sys::cuptiProfilerSetConfig(params_ptr) }.result()
}
/// Unset metrics configuration profiled.
pub unsafe fn unset_config(
    params_ptr: *mut sys::CUpti_Profiler_UnsetConfig_Params,
) -> Result<(), CuptiError> {
    unsafe { sys::cuptiProfilerUnsetConfig(params_ptr) }.result()
}
