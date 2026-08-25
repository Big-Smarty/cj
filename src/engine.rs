use std::marker::PhantomData;

#[cfg(feature = "backend-cubecl-cuda")]
use crate::backend::cubecl_cuda::CubeClCudaBackend;
#[cfg(feature = "backend-cubecl-hip")]
use crate::backend::cubecl_hip::CubeClHipBackend;
#[cfg(feature = "backend-cuda")]
use crate::backend::native_cuda::NativeCudaBackend;
#[cfg(feature = "backend-hip")]
use crate::backend::native_hip::NativeHipBackend;
#[cfg(feature = "backend-opencl")]
use crate::backend::native_opencl::NativeOpenClBackend;
use crate::{backend::Backend, backends::BackendKind};

pub struct Engine<B>
where
    B: Backend,
{
    device_locator: B::DeviceLocator,
    context: B::Context,
    queue: B::Queue,
    _backend: PhantomData<B>,
}

impl<B> Engine<B>
where
    B: Backend,
{
    pub const KIND: BackendKind = B::KIND;
}

pub enum EngineWrapper {
    #[cfg(feature = "backend-cuda")]
    NativeCuda(Engine<NativeCudaBackend>),

    #[cfg(feature = "backend-opencl")]
    NativeOpenCl(Engine<NativeOpenClBackend>),

    #[cfg(feature = "backend-hip")]
    NativeHip(Engine<NativeHipBackend>),

    #[cfg(feature = "backend-cubecl-cuda")]
    CubeClCuda(Engine<CubeClCudaBackend>),

    #[cfg(feature = "backend-cubecl-hip")]
    CubeClHip(Engine<CubeClHipBackend>),
}
