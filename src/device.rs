use std::fmt::Display;

use crate::gpu_id_set::GpuIdSet;

#[derive(Debug)]
pub struct Device {
    gpu_id: GpuIdSet,
    #[cfg(any(feature = "backend-cuda", feature = "backend-cubecl-cuda"))]
    cuda_name: Option<String>,
    #[cfg(feature = "backend-opencl")]
    opencl_name: Option<String>,
    #[cfg(any(feature = "backend-hip", feature = "backend-cubecl-hip"))]
    hip_name: Option<String>,
}

impl Device {
    pub fn new(gpu_id: GpuIdSet) -> Self {
        Self {
            gpu_id,
            #[cfg(any(feature = "backend-cuda", feature = "backend-cubecl-cuda"))]
            cuda_name: None,
            #[cfg(feature = "backend-opencl")]
            opencl_name: None,
            #[cfg(any(feature = "backend-hip", feature = "backend-cubecl-hip"))]
            hip_name: None,
        }
    }

    #[cfg(any(feature = "backend-cuda", feature = "backend-cubecl-cuda"))]
    pub fn set_cuda_name(&mut self, name: String) {
        self.cuda_name = Some(name);
    }

    #[cfg(any(feature = "backend-cuda", feature = "backend-cubecl-cuda"))]
    pub fn with_cuda_name(&self, name: String) -> Self {
        Self {
            gpu_id: self.gpu_id,
            cuda_name: Some(name),
            #[cfg(feature = "backend-opencl")]
            opencl_name: self.opencl_name.clone(),
            #[cfg(any(feature = "backend-hip", feature = "backend-cubecl-hip"))]
            hip_name: self.hip_name.clone(),
        }
    }

    #[cfg(feature = "backend-opencl")]
    pub fn set_opencl_name(&mut self, name: String) {
        self.opencl_name = Some(name);
    }

    #[cfg(feature = "backend-opencl")]
    pub fn with_opencl_name(&self, name: String) -> Self {
        Self {
            gpu_id: self.gpu_id,
            #[cfg(any(feature = "backend-cuda", feature = "backend-cubecl-cuda"))]
            cuda_name: self.cuda_name.clone(),
            opencl_name: Some(name),
            #[cfg(any(feature = "backend-hip", feature = "backend-cubecl-hip"))]
            hip_name: self.hip_name.clone(),
        }
    }

    #[cfg(any(feature = "backend-hip", feature = "backend-cubecl-hip"))]
    pub fn set_hip_name(&mut self, name: String) {
        self.hip_name = Some(name);
    }

    #[cfg(any(feature = "backend-hip", feature = "backend-cubecl-hip"))]
    pub fn with_hip_name(&self, name: String) -> Self {
        Self {
            gpu_id: self.gpu_id,
            #[cfg(any(feature = "backend-cuda", feature = "backend-cubecl-cuda"))]
            cuda_name: self.cuda_name.clone(),
            #[cfg(feature = "backend-opencl")]
            opencl_name: self.opencl_name.clone(),
            hip_name: Some(name),
        }
    }
}

impl Display for Device {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut out = format!("GPU IDs: {}\n", self.gpu_id);

        #[cfg(any(feature = "backend-cuda", feature = "backend-cubecl-cuda"))]
        out.extend(format!("CUDA name: {:?}\n", self.cuda_name).chars());

        #[cfg(feature = "backend-opencl")]
        out.extend(format!("OpenCL name: {:?}\n", self.opencl_name).chars());

        #[cfg(any(feature = "backend-hip", feature = "backend-cubecl-hip"))]
        out.extend(format!("HIP name: {:?}\n", self.hip_name).chars());

        f.write_str(&out)
    }
}
