use cubecl::{cuda::CudaRuntime, device::DeviceId};

use crate::{backend::Backend, backends::BackendKind};

pub struct CubeClCudaBackend;

impl Backend for CubeClCudaBackend {
    type DeviceLocator = DeviceId;

    type Context = CudaRuntime;

    type Queue = ();

    const KIND: BackendKind = BackendKind::CubeClCuda;
}
