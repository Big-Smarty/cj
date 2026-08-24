#[cfg(feature = "backend-cuda")]
use std::ffi::CStr;
use std::ffi::c_int;
use std::{collections::BTreeMap, ops::DerefMut};

use crate::{engine::Engine, gpu_id::GpuId, pci_id::PciId};

pub struct Cracker {
    pub gpu_ids: BTreeMap<PciId, GpuId>,
    pub engines: Vec<Engine>,
}

impl Cracker {
    pub fn new() -> anyhow::Result<Self> {
        let gpu_ids = Self::discover();
        Ok(Self {
            gpu_ids,
            engines: Vec::new(),
        })
    }

    fn discover() -> BTreeMap<PciId, GpuId> {
        let maps = vec![
            #[cfg(feature = "backend-cuda")]
            Self::list_cuda_devices(),
            #[cfg(feature = "backend-opencl")]
            Self::list_opencl_devices(),
            #[cfg(feature = "backend-hip")]
            Self::list_hip_devices(),
            #[cfg(feature = "backend-vulkan")]
            Self::list_vulkan_devices(),
        ];

        let mut joined: BTreeMap<PciId, GpuId> = BTreeMap::new();

        for map in maps
            .iter()
            .filter_map(|map: &anyhow::Result<BTreeMap<PciId, GpuId>>| map.as_ref().ok())
        {
            for (pci_id, discovered) in map {
                let entry = joined.entry(*pci_id).or_insert_with(|| GpuId::new(*pci_id));
                #[cfg(feature = "backend-cuda")]
                if entry.cuda_ordinal_id.is_none() && discovered.cuda_ordinal_id.is_some() {
                    entry.cuda_ordinal_id = discovered.cuda_ordinal_id;
                }
                #[cfg(feature = "backend-opencl")]
                if entry.opencl_ordinal_id.is_none() && discovered.opencl_ordinal_id.is_some() {
                    entry.opencl_ordinal_id = discovered.opencl_ordinal_id;
                }
                #[cfg(feature = "backend-hip")]
                if entry.hip_ordinal_id.is_none() && discovered.hip_ordinal_id.is_some() {
                    entry.hip_ordinal_id = discovered.hip_ordinal_id;
                }
                #[cfg(feature = "backend-vulkan")]
                if entry.vulkan_ordinal_id.is_none() && discovered.vulkan_ordinal_id.is_some() {
                    entry.vulkan_ordinal_id = discovered.vulkan_ordinal_id;
                }
            }
        }
        joined
    }

    #[cfg(feature = "backend-cuda")]
    fn list_cuda_devices() -> anyhow::Result<BTreeMap<PciId, GpuId>> {
        use cudarc::driver::{CudaContext, sys::is_culib_present};
        if !unsafe { is_culib_present() } {
            log::error!("no CUDA libraries found");
            return Err(anyhow::Error::msg("no CUDA libraries found"));
        }
        let device_count = match CudaContext::device_count() {
            Ok(count) => count as usize,
            Err(error) => {
                log::error!("no CUDA devices available: {error}");
                return Err(anyhow::Error::msg(format!(
                    "no CUDA devices available: {error}"
                )));
            }
        };

        Ok((0..device_count)
            .filter_map(|ordinal| CudaContext::new(ordinal).ok())
            .map(|device| (device.ordinal(), cuda_device_pci_id(&device)))
            .filter(|(_, id)| id.is_ok())
            .map(|(ordinal, id)| {
                let id = id.unwrap();
                let gpu_id = GpuId {
                    cuda_ordinal_id: Some(ordinal),
                    opencl_ordinal_id: None,
                    vulkan_ordinal_id: None,
                    hip_ordinal_id: None,
                    pci_id: id,
                };
                (id, gpu_id)
            })
            .collect())
    }
    #[cfg(feature = "backend-opencl")]
    fn list_opencl_devices() -> anyhow::Result<BTreeMap<PciId, GpuId>> {
        todo!()
    }
    #[cfg(feature = "backend-vulkan")]
    fn list_vulkan_devices() -> anyhow::Result<BTreeMap<PciId, GpuId>> {
        todo!()
    }
    #[cfg(feature = "backend-hip")]
    fn list_hip_devices() -> anyhow::Result<BTreeMap<PciId, GpuId>> {
        todo!()
    }
}

#[cfg(feature = "backend-cuda")]
use cudarc::{driver::CudaContext, runtime::sys::cudaDeviceGetPCIBusId};

#[cfg(feature = "backend-cuda")]
fn cuda_device_pci_id(device: &CudaContext) -> anyhow::Result<PciId> {
    let mut bus_id = [0_i8; 13];

    match unsafe {
        cudaDeviceGetPCIBusId(
            bus_id.as_mut_ptr(),
            bus_id.len() as c_int,
            device.ordinal() as c_int,
        )
    } {
        cudarc::runtime::sys::cudaError::cudaSuccess => {
            let bus_id = unsafe { CStr::from_ptr(bus_id.as_ptr()) }.to_str()?;
            PciId::try_from(bus_id)
        }
        cudarc::runtime::sys::cudaError::cudaErrorInvalidValue => {
            log::error!("cuda list IDs: invalid value");
            Err(anyhow::Error::msg("CUDA list IDs: invalid value"))
        }
        cudarc::runtime::sys::cudaError::cudaErrorInvalidDevice => {
            log::error!("cuda list IDs: invalid device");
            Err(anyhow::Error::msg("CUDA list IDs: invalid device"))
        }
        error => {
            log::error!("cuda list IDs failed: {error:?}");
            Err(anyhow::Error::msg(format!(
                "CUDA list IDs failed: {error:?}"
            )))
        }
    }
}
