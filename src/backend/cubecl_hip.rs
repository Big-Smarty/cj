use cubecl::{device::DeviceId, hip::HipRuntime};

use crate::{backend::Backend, backends::BackendKind};

pub struct CubeClHipBackend;

impl Backend for CubeClHipBackend {
    type DeviceLocator = DeviceId;

    type Context = HipRuntime;

    type Queue = ();

    const KIND: BackendKind = BackendKind::CubeClHip;
}
