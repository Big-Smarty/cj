use cubecl::{client::ComputeClient, device::DeviceId};

use crate::{backend::Backend, backends::BackendKind};

pub struct CubeClHipBackend;

impl Backend for CubeClHipBackend {
    type DeviceLocator = DeviceId;

    type Context = ComputeClient<cubecl::hip::HipRuntime>;

    type Queue = ();

    const KIND: BackendKind = BackendKind::CubeClHip;
}
