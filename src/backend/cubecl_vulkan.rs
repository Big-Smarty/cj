use cubecl::{device::DeviceId, wgpu::WgpuRuntime};

use crate::{backend::Backend, backends::BackendKind};

pub struct CubeClVulkanBackend;

impl Backend for CubeClVulkanBackend {
    type DeviceLocator = DeviceId;

    type Context = WgpuRuntime;

    type Queue = ();

    const KIND: BackendKind = BackendKind::CubeClVulkan;

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
