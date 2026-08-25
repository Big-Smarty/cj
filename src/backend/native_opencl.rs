use opencl3::{command_queue::CommandQueue, context::Context, device::Device};

use crate::{backend::Backend, backends::BackendKind};

pub struct NativeOpenClBackend;

impl Backend for NativeOpenClBackend {
    type DeviceLocator = Device;

    type Context = Context;

    type Queue = CommandQueue;

    const KIND: BackendKind = BackendKind::NativeOpenCl;

    type Module = ();

    type InputBuffer = ();

    type OutputBuffer = ();

    fn create_context(locator: &Self::DeviceLocator) -> anyhow::Result<Self::Context> {
        todo!()
    }

    fn create_queue(context: &Self::Context) -> anyhow::Result<Self::Queue> {
        todo!()
    }

    fn compile_module(
        context: &Self::Context,
        queue: &Self::Queue,
        kernel: (),
    ) -> anyhow::Result<Self::Module> {
        todo!()
    }

    fn create_input_buffer(
        context: &Self::Context,
        queue: &Self::Queue,
        size: usize,
    ) -> anyhow::Result<Self::InputBuffer> {
        todo!()
    }

    fn create_output_buffer(
        context: &Self::Context,
        queue: &Self::Queue,
        size: usize,
    ) -> anyhow::Result<Self::OutputBuffer> {
        todo!()
    }

    fn bench(
        queue: &Self::Queue,
        module: &Self::Module,
        input: &Self::InputBuffer,
        output: &mut Self::OutputBuffer,
        bench_info: (),
    ) -> anyhow::Result<()> {
        todo!()
    }
}
