use rocmrc::{HipContext, HipStream};

use crate::{backend::Backend, backends::BackendKind};

pub struct NativeHipBackend;

impl Backend for NativeHipBackend {
    type DeviceLocator = usize;

    type Context = HipContext;

    type Queue = HipStream;

    const KIND: BackendKind = BackendKind::NativeHip;

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
