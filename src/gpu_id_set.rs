use std::fmt::Display;

#[cfg(feature = "backend-opencl")]
use cl3::ext::cl_device_id;

use crate::pci_id::PciId;

#[derive(Clone, Copy, Debug, Eq, PartialEq, PartialOrd, Ord, Hash)]
pub struct GpuIdSet {
    #[cfg(any(feature = "backend-cuda", feature = "backend-cubecl-cuda"))]
    pub cuda_ordinal_id: Option<usize>,
    #[cfg(feature = "backend-opencl")]
    pub opencl_ordinal_id: Option<cl_device_id>,
    #[cfg(any(feature = "backend-hip", feature = "backend-cubecl-hip"))]
    pub hip_ordinal_id: Option<usize>,
    pub pci_id: PciId,
}

impl GpuIdSet {
    pub fn new(pci_id: PciId) -> Self {
        Self {
            #[cfg(any(feature = "backend-cuda", feature = "backend-cubecl-cuda"))]
            cuda_ordinal_id: None,
            #[cfg(feature = "backend-opencl")]
            opencl_ordinal_id: None,
            #[cfg(any(feature = "backend-hip", feature = "backend-cubecl-hip"))]
            hip_ordinal_id: None,
            pci_id,
        }
    }
}

impl Display for GpuIdSet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut out = format!("PCI ID: {}\n", self.pci_id);

        #[cfg(any(feature = "backend-cuda", feature = "backend-cubecl-cuda"))]
        out.extend(format!("CUDA ordinal: {:?}\n", self.cuda_ordinal_id).chars());

        #[cfg(feature = "backend-opencl")]
        out.extend(format!("OpenCL ordinal: {:?}\n", self.opencl_ordinal_id).chars());

        #[cfg(any(feature = "backend-hip", feature = "backend-cubecl-hip"))]
        out.extend(format!("HIP ordinal: {:?}\n", self.hip_ordinal_id).chars());

        f.write_str(&out)
    }
}
