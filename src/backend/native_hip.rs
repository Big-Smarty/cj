use rocmrc::{HipContext, HipStream};

use crate::{backend::Backend, backends::BackendKind};

pub struct NativeHipBackend;

impl Backend for NativeHipBackend {
    type DeviceLocator = usize;

    type Context = HipContext;

    type Queue = HipStream;

    const KIND: BackendKind = BackendKind::NativeHip;
}
