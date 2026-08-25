use clap::{ValueEnum, builder::PossibleValue};

#[cfg(feature = "backend-cuda")]
pub struct NativeCudaBackend;
#[cfg(feature = "backend-opencl")]
pub struct NativeOpenClBackend;
#[cfg(feature = "backend-vulkan")]
pub struct NativeVulkanBackend;
#[cfg(feature = "backend-cubecl-cuda")]
pub struct CubeClCudaBackend;
#[cfg(feature = "backend-cubecl-hip")]
pub struct CubeClHipBackend;
#[cfg(feature = "backend-cubecl-vulkan")]
pub struct CubeClVulkanBackend;

#[derive(Debug, Copy, Clone)]
pub enum BackendKind {
    All,
    #[cfg(feature = "backend-cuda")]
    NativeCuda,
    #[cfg(feature = "backend-opencl")]
    NativeOpenCl,
    #[cfg(feature = "backend-vulkan")]
    NativeVulkan,
    #[cfg(feature = "backend-cubecl-cuda")]
    CubeClCuda,
    #[cfg(feature = "backend-cubecl-hip")]
    CubeClHip,
    #[cfg(feature = "backend-cubecl-vulkan")]
    CubeClVulkan,
}

impl ValueEnum for BackendKind {
    fn value_variants<'a>() -> &'a [Self] {
        &[
            Self::All,
            #[cfg(feature = "backend-cuda")]
            Self::NativeCuda,
            #[cfg(feature = "backend-opencl")]
            Self::NativeOpenCl,
            #[cfg(feature = "backend-vulkan")]
            Self::NativeVulkan,
            #[cfg(feature = "backend-cubecl-cuda")]
            Self::CubeClCuda,
            #[cfg(feature = "backend-cubecl-hip")]
            Self::CubeClHip,
            #[cfg(feature = "backend-cubecl-vulkan")]
            Self::CubeClVulkan,
        ]
    }

    fn to_possible_value(&self) -> Option<clap::builder::PossibleValue> {
        Some(match self {
            Self::All => PossibleValue::new("all").help("Benchmark all available backends"),
            #[cfg(feature = "backend-cuda")]
            Self::NativeCuda => PossibleValue::new("native-cuda").help("Benchmark native CUDA"),
            #[cfg(feature = "backend-opencl")]
            Self::NativeOpenCl => {
                PossibleValue::new("native-opencl").help("Benchmark native OpenCL")
            }
            #[cfg(feature = "backend-vulkan")]
            Self::NativeVulkan => {
                PossibleValue::new("native-vulkan").help("Benchmark native Vulkan")
            }
            #[cfg(feature = "backend-cubecl-cuda")]
            Self::CubeClCuda => PossibleValue::new("cubecl-cuda").help("Benchmark CubeCL CUDA"),
            #[cfg(feature = "backend-cubecl-hip")]
            Self::CubeClHip => PossibleValue::new("cubecl-opencl").help("Benchmark CubeCL OpenCL"),
            #[cfg(feature = "backend-cubecl-vulkan")]
            Self::CubeClVulkan => {
                PossibleValue::new("cubecl-vulkan").help("Benchmark CubeCL Vulkan")
            }
        })
    }
}
