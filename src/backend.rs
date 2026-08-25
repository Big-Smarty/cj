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

// TODO: add PreparedWorkload struct, defining how a GPU thread should generate and hash passwords
// TODO: get rid of InputBuffer and replace with each API's analogue of PushConstants

pub trait Backend: Sized + 'static {
    type Session;
    type Module;
    type ModuleStorage = BTreeMap<Algorithm, Self::Module>;
    type InputBuffer;
    type OutputBuffer;

    const KIND: BackendKind;
}
