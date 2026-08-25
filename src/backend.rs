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

    fn create_context(locator: &Self::DeviceLocator) -> anyhow::Result<Self::Context>;
    fn create_queue(context: &Self::Context) -> anyhow::Result<Self::Queue>;
    fn compile_module(
        context: &Self::Context,
        queue: &Self::Queue,
        kernel: (),
    ) -> anyhow::Result<Self::Module>;
    fn create_input_buffer(
        context: &Self::Context,
        queue: &Self::Queue,
        size: usize,
    ) -> anyhow::Result<Self::InputBuffer>;
    fn create_output_buffer(
        context: &Self::Context,
        queue: &Self::Queue,
        size: usize,
    ) -> anyhow::Result<Self::OutputBuffer>;
    fn bench(
        queue: &Self::Queue,
        module: &Self::Module,
        input: &Self::InputBuffer,
        output: &mut Self::OutputBuffer,
        bench_info: (),
    ) -> anyhow::Result<()>;
}
