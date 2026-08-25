use cubecl::{client::ComputeClient, device::DeviceId};

use crate::{backend::Backend, backends::BackendKind};

pub struct CubeClVulkanBackend;

impl Backend for CubeClVulkanBackend {
    type DeviceLocator = DeviceId;

    type Context = ComputeClient<cubecl::wgpu::WgpuRuntime>;

    type Queue = ();

    const KIND: BackendKind = BackendKind::CubeClVulkan;
}
