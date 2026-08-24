#[cfg(feature = "backend-opencl")]
use cl3::ext::cl_device_id;

use crate::pci_id::PciId;

#[derive(Clone, Copy, Debug, Eq, PartialEq, PartialOrd, Ord, Hash)]
pub struct GpuIdSet {
    #[cfg(feature = "backend-cuda")]
    pub cuda_ordinal_id: Option<usize>,
    #[cfg(feature = "backend-opencl")]
    pub opencl_ordinal_id: Option<cl_device_id>,
    #[cfg(feature = "backend-vulkan")]
    pub vulkan_ordinal_id: Option<usize>,
    #[cfg(feature = "backend-hip")]
    pub hip_ordinal_id: Option<usize>,
    pub pci_id: PciId,
}

impl GpuIdSet {
    pub fn new(pci_id: PciId) -> Self {
        Self {
            #[cfg(feature = "backend-cuda")]
            cuda_ordinal_id: None,
            #[cfg(feature = "backend-opencl")]
            opencl_ordinal_id: None,
            #[cfg(feature = "backend-vulkan")]
            vulkan_ordinal_id: None,
            #[cfg(feature = "backend-hip")]
            hip_ordinal_id: None,
            pci_id,
        }
    }
}
