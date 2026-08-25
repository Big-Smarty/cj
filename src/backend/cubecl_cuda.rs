use cubecl::{cuda::CudaRuntime, device::DeviceId};

use crate::{backend::Backend, backends::BackendKind};

pub struct CubeClCudaBackend;

impl Backend for CubeClCudaBackend {
    type DeviceLocator = DeviceId;

    type Context = CudaRuntime;

    type Queue = ();

    const KIND: BackendKind = BackendKind::CubeClCuda;

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
