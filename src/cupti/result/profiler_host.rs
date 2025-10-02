//! Functions of the Profiler Host API.

use super::super::{result::CuptiError, sys};

// Add the metrics to the profiler host object for generating the config image.
pub unsafe fn config_add_metrics(
    params: *mut sys::CUpti_Profiler_Host_ConfigAddMetrics_Params,
) -> Result<(), CuptiError> {
    unsafe { sys::cuptiProfilerHostConfigAddMetrics(params) }.result()
}

// Deinitialize and destroy the profiler host object (CUpti_Profiler_Host_Object).
pub unsafe fn deinitialize(
    params: *mut sys::CUpti_Profiler_Host_Deinitialize_Params,
) -> Result<(), CuptiError> {
    unsafe { sys::cuptiProfilerHostDeinitialize(params) }.result()
}

// Evaluate the metric values for the range index stored in the counter data.
pub unsafe fn evaluate_to_gpu_values(
    params: *mut sys::CUpti_Profiler_Host_EvaluateToGpuValues_Params,
) -> Result<(), CuptiError> {
    unsafe { sys::cuptiProfilerHostEvaluateToGpuValues(params) }.result()
}

// Get the list of supported base metrics for the chip.
pub unsafe fn get_base_metrics(
    params: *mut sys::CUpti_Profiler_Host_GetBaseMetrics_Params,
) -> Result<(), CuptiError> {
    unsafe { sys::cuptiProfilerHostGetBaseMetrics(params) }.result()
}

// Get the config image for the metrics added to the profiler host object.
pub unsafe fn get_config_image(
    params: *mut sys::CUpti_Profiler_Host_GetConfigImage_Params,
) -> Result<(), CuptiError> {
    unsafe { sys::cuptiProfilerHostGetConfigImage(params) }.result()
}

// Get the size of the config image for the metrics added to the profiler host object.
pub unsafe fn get_config_image_size(
    params: *mut sys::CUpti_Profiler_Host_GetConfigImageSize_Params,
) -> Result<(), CuptiError> {
    unsafe { sys::cuptiProfilerHostGetConfigImageSize(params) }.result()
}

// Get the maximum number of hardware metrics (metric names which doesn't include sass keyword) that can be scheduled in a single pass for a chip.
pub unsafe fn get_max_num_hardware_metrics_per_pass(
    params: *mut sys::CUpti_Profiler_Host_GetMaxNumHardwareMetricsPerPass_Params,
) -> Result<(), CuptiError> {
    unsafe { sys::cuptiProfilerHostGetMaxNumHardwareMetricsPerPass(params) }.result()
}

// Get the properties of the metric.
pub unsafe fn get_metric_properties(
    params: *mut sys::CUpti_Profiler_Host_GetMetricProperties_Params,
) -> Result<(), CuptiError> {
    unsafe { sys::cuptiProfilerHostGetMetricProperties(params) }.result()
}

// Get the number of passes required for profiling the scheduled metrics in the config image.
pub unsafe fn get_num_of_passes(
    params: *mut sys::CUpti_Profiler_Host_GetNumOfPasses_Params,
) -> Result<(), CuptiError> {
    unsafe { sys::cuptiProfilerHostGetNumOfPasses(params) }.result()
}

// Get the range name for the range index stored in the counter data.
pub unsafe fn get_range_name(
    params: *mut sys::CUpti_Profiler_Host_GetRangeName_Params,
) -> Result<(), CuptiError> {
    unsafe { sys::cuptiProfilerHostGetRangeName(params) }.result()
}

// Get the list of supported sub-metrics for the metric.
pub unsafe fn get_sub_metrics(
    params: *mut sys::CUpti_Profiler_Host_GetSubMetrics_Params,
) -> Result<(), CuptiError> {
    unsafe { sys::cuptiProfilerHostGetSubMetrics(params) }.result()
}

// Get the list of supported chips.
pub unsafe fn get_supported_chips(
    params: *mut sys::CUpti_Profiler_Host_GetSupportedChips_Params,
) -> Result<(), CuptiError> {
    unsafe { sys::cuptiProfilerHostGetSupportedChips(params) }.result()
}

// Create and initialize the profiler host object (CUpti_Profiler_Host_Object).
pub unsafe fn initialize(
    params: *mut sys::CUpti_Profiler_Host_Initialize_Params,
) -> Result<(), CuptiError> {
    unsafe { sys::cuptiProfilerHostInitialize(params) }.result()
}
