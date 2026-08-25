use opencl3::{command_queue::CommandQueue, context::Context, device::Device, kernel::Kernel};

use crate::{backend::Backend, backends::BackendKind};

pub struct NativeOpenClBackend;

impl Backend for NativeOpenClBackend {
    type Session = OpenClSession;

    const KIND: BackendKind = BackendKind::NativeOpenCl;

    type Module = Kernel;

    type InputBuffer = ();

    type OutputBuffer = ();
}

pub struct OpenClSession {
    context: Context,
    device: Device,

    // TODO: create the CommandQueue with CL_QUEUE_PROFILING_ENABLE
    command_queue: CommandQueue,
}
