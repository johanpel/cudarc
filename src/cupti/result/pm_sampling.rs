//! Functions of the PM Sampling API

use super::super::{result::CuptiError, sys};

/// Get the sample info (start and end time stamp) for the given sample index.
pub unsafe fn counter_data_get_sample_info(
    params_ptr: *mut sys::CUpti_PmSampling_CounterData_GetSampleInfo_Params,
) -> Result<(), CuptiError> {
    unsafe { sys::cuptiPmSamplingCounterDataGetSampleInfo(params_ptr) }.result()
}
/// Initialize the counter data to CUPTI record format for storing the metric data.
pub unsafe fn counter_data_image_initialize(
    params_ptr: *mut sys::CUpti_PmSampling_CounterDataImage_Initialize_Params,
) -> Result<(), CuptiError> {
    unsafe { sys::cuptiPmSamplingCounterDataImageInitialize(params_ptr) }.result()
}
/// Decode the metrics data stored in the hardware buffer to the counter data image.
pub unsafe fn decode_data(
    params_ptr: *mut sys::CUpti_PmSampling_DecodeData_Params,
) -> Result<(), CuptiError> {
    unsafe { sys::cuptiPmSamplingDecodeData(params_ptr) }.result()
}
/// Disable PM sampling on the CUDA device and destroy the PM sampling object.
pub unsafe fn disable(
    params_ptr: *mut sys::CUpti_PmSampling_Disable_Params,
) -> Result<(), CuptiError> {
    unsafe { sys::cuptiPmSamplingDisable(params_ptr) }.result()
}
/// Create a PM sampling object and enable PM sampling on the CUDA device.
pub unsafe fn enable(
    params_ptr: *mut sys::CUpti_PmSampling_Enable_Params,
) -> Result<(), CuptiError> {
    unsafe { sys::cuptiPmSamplingEnable(params_ptr) }.result()
}
/// Query counter availibility information in a buffer which can be used to filter unavailable raw metrics on host.
pub unsafe fn get_counter_availability(
    params_ptr: *mut sys::CUpti_PmSampling_GetCounterAvailability_Params,
) -> Result<(), CuptiError> {
    unsafe { sys::cuptiPmSamplingGetCounterAvailability(params_ptr) }.result()
}
/// Get the counter data info like number of samples, number of populated samples and number of completed samples in a counter data image.
pub unsafe fn get_counter_data_info(
    params_ptr: *mut sys::CUpti_PmSampling_GetCounterDataInfo_Params,
) -> Result<(), CuptiError> {
    unsafe { sys::cuptiPmSamplingGetCounterDataInfo(params_ptr) }.result()
}
/// Query the size of the counter data image which will be used to store the metrics data.
pub unsafe fn get_counter_data_size(
    params_ptr: *mut sys::CUpti_PmSampling_GetCounterDataSize_Params,
) -> Result<(), CuptiError> {
    unsafe { sys::cuptiPmSamplingGetCounterDataSize(params_ptr) }.result()
}
/// Set the configuration for PM sampling like sampling interval, maximum number of samples filled in HW buffer, trigger mode and the config image which has scheduling info for metric collection.
pub unsafe fn set_config(
    params_ptr: *mut sys::CUpti_PmSampling_SetConfig_Params,
) -> Result<(), CuptiError> {
    unsafe { sys::cuptiPmSamplingSetConfig(params_ptr) }.result()
}
/// Start the PM sampling.
pub unsafe fn start(params_ptr: *mut sys::CUpti_PmSampling_Start_Params) -> Result<(), CuptiError> {
    unsafe { sys::cuptiPmSamplingStart(params_ptr) }.result()
}
/// Stop the PM sampling.
pub unsafe fn stop(params_ptr: *mut sys::CUpti_PmSampling_Stop_Params) -> Result<(), CuptiError> {
    unsafe { sys::cuptiPmSamplingStop(params_ptr) }.result()
}
