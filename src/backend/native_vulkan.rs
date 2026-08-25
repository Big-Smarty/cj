use std::sync::Arc;

use vulkano::device::{Device, Queue, physical::PhysicalDevice};

use crate::{backend::Backend, backends::BackendKind};

pub struct NativeVulkanBackend;

impl Backend for NativeVulkanBackend {
    type DeviceLocator = Arc<PhysicalDevice>;

    type Context = Arc<Device>;

    type Queue = Arc<Queue>;

    const KIND: BackendKind = BackendKind::NativeVulkan;

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
