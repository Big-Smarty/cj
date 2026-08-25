use opencl3::{command_queue::CommandQueue, context::Context, device::Device};

use crate::{backend::Backend, backends::BackendKind};

pub struct NativeOpenClBackend;

impl Backend for NativeOpenClBackend {
    type DeviceLocator = Device;

    type Context = Context;

    type Queue = CommandQueue;

    const KIND: BackendKind = BackendKind::NativeOpenCl;

    fn create_context(&mut self) {
        todo!()
    }

    fn create_queue(&mut self) {
        todo!()
    }

    fn compile_module(&mut self) {
        todo!()
    }

    fn create_input_buffer(&mut self) {
        todo!()
    }

    fn create_output_buffer(&mut self) {
        todo!()
    }

    fn bench(&mut self) {
        todo!()
    }
}
