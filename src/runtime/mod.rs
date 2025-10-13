//! Wrappers around the [CUDA Runtime API](https://docs.nvidia.com/cuda/cuda-runtime-api/index.html),
//! in two levels: an unsafe low-level API and a (still unsafe) thin wrapper around it.

#[cfg(feature = "runtime")]
pub mod result;
#[cfg(feature = "runtime-sys")]
#[allow(warnings)]
pub mod sys;
