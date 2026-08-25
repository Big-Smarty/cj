use opencl3::{command_queue::CommandQueue, context::Context, device::Device};

use crate::{backend::Backend, backends::BackendKind};

pub struct NativeOpenClBackend;

impl Backend for NativeOpenClBackend {
    type DeviceLocator = Device;

    type Context = Context;

    type Queue = CommandQueue;

    const KIND: BackendKind = BackendKind::NativeOpenCl;
}
