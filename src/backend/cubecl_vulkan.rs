use cubecl::{device::DeviceId, wgpu::WgpuRuntime};

use crate::{backend::Backend, backends::BackendKind};

pub struct CubeClVulkanBackend;

impl Backend for CubeClVulkanBackend {
    type DeviceLocator = DeviceId;

    type Context = WgpuRuntime;

    type Queue = ();

    const KIND: BackendKind = BackendKind::CubeClVulkan;
}
