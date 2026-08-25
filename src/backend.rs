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
    type Module;
    type InputBuffer;
    type OutputBuffer;

    const KIND: BackendKind;

    fn create_context(locator: &Self::DeviceLocator);
    fn create_queue(context: &Self::Context);
    fn compile_module(context: &Self::Context, queue: &Self::Queue, kernel: ());
    fn create_input_buffer(context: &Self::Context, queue: &Self::Queue, size: usize);
    fn create_output_buffer(context: &Self::Context, queue: &Self::Queue, size: usize);
    fn bench(
        queue: &Self::Queue,
        module: &Self::Module,
        input: &Self::InputBuffer,
        output: &mut Self::OutputBuffer,
        bench_info: (),
    );
}
