use clap::{ValueEnum, builder::PossibleValue};

#[derive(Debug, Copy, Clone)]
pub enum BackendKind {
    #[cfg(feature = "backend-cuda")]
    NativeCuda,
    #[cfg(feature = "backend-opencl")]
    NativeOpenCl,
    #[cfg(feature = "backend-hip")]
    NativeHip,
    #[cfg(feature = "backend-cubecl-cuda")]
    CubeClCuda,
    #[cfg(feature = "backend-cubecl-hip")]
    CubeClHip,
}

impl ValueEnum for BackendKind {
    fn value_variants<'a>() -> &'a [Self] {
        &[
            #[cfg(feature = "backend-cuda")]
            Self::NativeCuda,
            #[cfg(feature = "backend-opencl")]
            Self::NativeOpenCl,
            #[cfg(feature = "backend-hip")]
            Self::NativeHip,
            #[cfg(feature = "backend-cubecl-cuda")]
            Self::CubeClCuda,
            #[cfg(feature = "backend-cubecl-hip")]
            Self::CubeClHip,
        ]
    }

    fn to_possible_value(&self) -> Option<clap::builder::PossibleValue> {
        Some(match self {
            #[cfg(feature = "backend-cuda")]
            Self::NativeCuda => PossibleValue::new("native-cuda").help("Benchmark native CUDA"),
            #[cfg(feature = "backend-opencl")]
            Self::NativeOpenCl => {
                PossibleValue::new("native-opencl").help("Benchmark native OpenCL")
            }
            #[cfg(feature = "backend-hip")]
            Self::NativeHip => PossibleValue::new("native-hip").help("Benchmark native HIP"),
            #[cfg(feature = "backend-cubecl-cuda")]
            Self::CubeClCuda => PossibleValue::new("cubecl-cuda").help("Benchmark CubeCL CUDA"),
            #[cfg(feature = "backend-cubecl-hip")]
            Self::CubeClHip => PossibleValue::new("cubecl-hip").help("Benchmark CubeCL OpenCL"),
        })
    }
}
