use cubecl::{device::DeviceId, hip::HipRuntime};

use crate::{backend::Backend, backends::BackendKind};

pub struct CubeClHipBackend;

impl Backend for CubeClHipBackend {
    type DeviceLocator = DeviceId;

    type Context = HipRuntime;

    type Queue = ();

    const KIND: BackendKind = BackendKind::CubeClHip;

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
