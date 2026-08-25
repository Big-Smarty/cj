use std::sync::Arc;

use vulkano::device::{Device, Queue, physical::PhysicalDevice};

use crate::{backend::Backend, backends::BackendKind};

pub struct NativeVulkanBackend;

impl Backend for NativeVulkanBackend {
    type DeviceLocator = Arc<PhysicalDevice>;

    type Context = Arc<Device>;

    type Queue = Arc<Queue>;

    const KIND: BackendKind = BackendKind::NativeVulkan;
}
