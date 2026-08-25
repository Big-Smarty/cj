use crate::backends::BackendKind;

#[cfg(feature = "backend-cuda")]
mod native_cuda;

#[cfg(feature = "backend-opencl")]
mod native_opencl;

#[cfg(feature = "backend-vulkan")]
mod native_vulkan;

#[cfg(feature = "backend-hip")]
mod native_hip;

#[cfg(feature = "backend-cubecl-cuda")]
mod cubecl_cuda;

#[cfg(feature = "backend-cubecl-hip")]
mod cubecl_hip;

#[cfg(feature = "backend-cubecl-vulkan")]
mod cubecl_vulkan;

pub trait Backend: Sized + 'static {
    type DeviceLocator;
    type Context;
    type Queue;

    const KIND: BackendKind;
}
