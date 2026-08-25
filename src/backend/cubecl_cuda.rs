use cubecl::{client::ComputeClient, device::DeviceId};

use crate::{backend::Backend, backends::BackendKind};

pub struct CubeClCudaBackend;

impl Backend for CubeClCudaBackend {
    type DeviceLocator = DeviceId;

    type Context = ComputeClient<cubecl::cuda::CudaRuntime>;

    type Queue = ();

    const KIND: BackendKind = BackendKind::CubeClCuda;
}
