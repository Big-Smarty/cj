use cudarc::driver::{CudaContext, CudaStream};

use crate::{backend::Backend, backends::BackendKind};

pub struct NativeCudaBackend;

impl Backend for NativeCudaBackend {
    type DeviceLocator = usize;

    type Context = CudaContext;

    type Queue = CudaStream;

    const KIND: BackendKind = BackendKind::NativeCuda;

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
