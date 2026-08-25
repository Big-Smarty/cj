use crate::backends::BackendKind;

#[cfg(feature = "backend-cuda")]
pub mod native_cuda;

#[cfg(feature = "backend-opencl")]
pub mod native_opencl;

#[cfg(feature = "backend-vulkan")]
pub mod native_vulkan;

#[cfg(feature = "backend-hip")]
pub mod native_hip;

#[cfg(feature = "backend-cubecl-cuda")]
pub mod cubecl_cuda;

#[cfg(feature = "backend-cubecl-hip")]
pub mod cubecl_hip;

#[cfg(feature = "backend-cubecl-vulkan")]
pub mod cubecl_vulkan;

pub trait Backend: Sized + 'static {
    type DeviceLocator;
    type Context;
    type Queue;

    const KIND: BackendKind;

    fn create_context(&mut self);
    fn create_queue(&mut self);
    fn compile_module(&mut self);
    fn create_input_buffer(&mut self);
    fn create_output_buffer(&mut self);
    fn bench(&mut self);
}
