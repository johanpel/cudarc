use super::super::{result::CuptiError, sys};
use crate::driver;

pub unsafe fn enum_event_domains(
    device: driver::sys::CUdevice,
    array_size_bytes: *mut usize,
    domain_array: *mut sys::CUpti_EventDomainID,
) -> Result<(), CuptiError> {
    unsafe { sys::cuptiDeviceEnumEventDomains(device, array_size_bytes, domain_array) }.result()
}

pub unsafe fn enum_metrics(
    device: driver::sys::CUdevice,
    array_size_bytes: *mut usize,
    metric_array: *mut sys::CUpti_MetricID,
) -> Result<(), CuptiError> {
    unsafe { sys::cuptiDeviceEnumMetrics(device, array_size_bytes, metric_array) }.result()
}

pub unsafe fn get_attribute(
    device: driver::sys::CUdevice,
    attrib: sys::CUpti_DeviceAttribute,
    value_size: *mut usize,
    value: *mut ::core::ffi::c_void,
) -> Result<(), CuptiError> {
    unsafe { sys::cuptiDeviceGetAttribute(device, attrib, value_size, value) }.result()
}

pub unsafe fn get_chip_name(
    p_params: *mut sys::CUpti_Device_GetChipName_Params,
) -> Result<(), CuptiError> {
    unsafe { sys::cuptiDeviceGetChipName(p_params) }.result()
}

pub unsafe fn get_event_domain_attribute(
    device: driver::sys::CUdevice,
    event_domain: sys::CUpti_EventDomainID,
    attrib: sys::CUpti_EventDomainAttribute,
    value_size: *mut usize,
    value: *mut ::core::ffi::c_void,
) -> Result<(), CuptiError> {
    unsafe {
        sys::cuptiDeviceGetEventDomainAttribute(device, event_domain, attrib, value_size, value)
    }
    .result()
}

pub unsafe fn get_num_event_domains(
    device: driver::sys::CUdevice,
    num_domains: *mut u32,
) -> Result<(), CuptiError> {
    unsafe { sys::cuptiDeviceGetNumEventDomains(device, num_domains) }.result()
}

pub unsafe fn get_num_metrics(
    device: driver::sys::CUdevice,
    num_metrics: *mut u32,
) -> Result<(), CuptiError> {
    unsafe { sys::cuptiDeviceGetNumMetrics(device, num_metrics) }.result()
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
pub fn cuptiDeviceGetTimestamp(
    context: driver::sys::CUcontext,
    timestamp: *mut u64,
) -> Result<(), CuptiError>;

/// Check support for a compute device.
///
/// See [cuptiDeviceSupported()](https://docs.nvidia.com/cupti/api/group__CUPTI__ACTIVITY__API.html#group__cupti__activity__api_1ga2493c952b9ceccf953ade5a6816fefdb).
///
/// # Safety
/// Support must exist.
pub unsafe fn device_supported(
    dev: driver::sys::CUdevice,
    support: *mut core::ffi::c_int,
) -> Result<(), CuptiError> {
    unsafe { sys::cuptiDeviceSupported(dev, support) }.result()
}

/// Query the virtualization mode of the device.
///
/// See [cuptiDeviceVirtualizationMode()](https://docs.nvidia.com/cupti/api/group__CUPTI__ACTIVITY__API.html#group__cupti__activity__api_1ga395c59b62aeac395e38ced9d40677c76).
///
/// # Safety
/// Mode must exist.
pub unsafe fn device_virtualization_mode(
    dev: driver::sys::CUdevice,
    mode: *mut sys::CUpti_DeviceVirtualizationMode,
) -> Result<(), CuptiError> {
    unsafe { sys::cuptiDeviceVirtualizationMode(dev, mode) }.result()
}
