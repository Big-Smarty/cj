use cudarc::driver::{CudaContext, CudaStream};

use crate::{backend::Backend, backends::BackendKind};

pub struct NativeCudaBackend;

impl Backend for NativeCudaBackend {
    type DeviceLocator = usize;

    type Context = CudaContext;

    type Queue = CudaStream;

    const KIND: BackendKind = BackendKind::NativeCuda;
}
