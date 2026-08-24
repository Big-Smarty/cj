use clap::{ValueEnum, builder::PossibleValue};

#[derive(Debug, Copy, Clone)]
pub enum Backends {
    All,
    NativeCuda,
    NativeOpenCl,
    NativeVulkan,
    CubeClCuda,
    CubeClOpenCl,
    CubeClVulkan,
}

impl ValueEnum for Backends {
    fn value_variants<'a>() -> &'a [Self] {
        &[
            Self::All,
            Self::NativeCuda,
            Self::NativeOpenCl,
            Self::NativeVulkan,
            Self::CubeClCuda,
            Self::CubeClOpenCl,
            Self::CubeClVulkan,
        ]
    }

    fn to_possible_value(&self) -> Option<clap::builder::PossibleValue> {
        Some(match self {
            Self::All => PossibleValue::new("all").help("Benchmark all available backends"),
            Self::NativeCuda => PossibleValue::new("native-cuda").help("Benchmark native CUDA"),
            Self::NativeOpenCl => {
                PossibleValue::new("native-opencl").help("Benchmark native OpenCL")
            }
            Self::NativeVulkan => {
                PossibleValue::new("native-vulkan").help("Benchmark native Vulkan")
            }
            Self::CubeClCuda => PossibleValue::new("cubecl-cuda").help("Benchmark CubeCL CUDA"),
            Self::CubeClOpenCl => {
                PossibleValue::new("cubecl-opencl").help("Benchmark CubeCL OpenCL")
            }
            Self::CubeClVulkan => {
                PossibleValue::new("cubecl-vulkan").help("Benchmark CubeCL Vulkan")
            }
        })
    }
}
