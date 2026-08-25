use std::collections::BTreeMap;

use crate::{algorithms::Algorithm, backends::BackendKind};

//#[cfg(feature = "backend-cuda")]
//pub mod native_cuda;

#[cfg(feature = "backend-opencl")]
pub mod native_opencl;

//#[cfg(feature = "backend-hip")]
//pub mod native_hip;

//#[cfg(feature = "backend-cubecl-cuda")]
//pub mod cubecl_cuda;

//#[cfg(feature = "backend-cubecl-hip")]
//pub mod cubecl_hip;

pub trait Backend: Sized + 'static {
    type Session;
    type Module;
    type ModuleStorage = BTreeMap<Algorithm, Self::Module>;
    type InputBuffer;
    type OutputBuffer;

    const KIND: BackendKind;
}
