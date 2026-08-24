# cj

`cj` is an experimental, GPU-focused hash-cracking engine for comparing compute backends and selecting the most suitable backend for a given workload.

The project was created to support research into GPU hash-cracking performance. Its primary goal is to measure, explain, and compare performance differences between portable and backend-specific GPU implementations.

`cj` is currently under development. Commands, supported algorithms, and backend availability may change.

## Goals

- Provide a common interface for multiple GPU compute backends.
- Measure GPU hashrate under controlled and reproducible conditions.
- Compare portable kernels with backend-specific implementations.
- Automatically select an appropriate backend for the current device and workload.
- Produce structured benchmark data suitable for further analysis.
- Keep the cracking engine independent of networking and storage infrastructure.

## Backends

Planned backends include:

- CubeCL CUDA
- CubeCL HIP
- CubeCL Vulkan
- Native CUDA using `cudarc`
- Native OpenCL using `opencl3`
- Native Vulkan using `vulkano`

Backend support is controlled through Cargo features so that `cj` can be built without requiring every GPU SDK.

## Features

- GPU-only hash computation
- Multiple interchangeable compute backends
- Automatic device and backend selection
- Explicit backend selection for controlled comparisons
- GPU hashrate benchmarking
- Deterministic benchmark workloads
- Structured JSON Lines benchmark output
- Configurable batch and work-group sizes
- Cross-backend correctness testing
- Modular support for additional hash algorithms and backends

## How cj Differs

Unlike distributed cracking systems, `cj` contains no networking, database, worker coordination, or persistent job management. It focuses only on local GPU execution.

Unlike tools designed primarily for practical password recovery, `cj` is also intended as a research platform. It provides comparable implementations across multiple GPU APIs and records the information needed to investigate why their performance differs.

The project is not intended to replace mature tools such as Hashcat or John the Ripper. Its focus is backend comparison, reproducible measurement, and automatic backend selection.

## Building

Build all available backends:

```shell
cargo build --release --features all-backends
```

Build selected backends:

```shell
cargo build --release \
  --features backend-cuda,backend-opencl,backend-vulkan
```

The required GPU drivers and development libraries must be installed for each enabled backend.

## Usage

The following examples show the planned command-line interface.

List available devices and backends:

```shell
cj devices
```

Benchmark every available backend:

```shell
cj benchmark \
  --backend all \
  --algorithm sha256
```

Benchmark a specific backend:

```shell
cj benchmark \
  --backend native-cuda \
  --algorithm sha256 \
  --batch-size 1048576 \
  --warmup 30s \
  --sample-time 2s \
  --samples 15
```

Write raw benchmark samples to a file:

```shell
cj benchmark \
  --backend all \
  --algorithm sha256 \
  --output results.jsonl
```

Run with automatic backend selection:

```shell
cj crack \
  --backend auto \
  --algorithm sha256 \
  --target-file hashes.txt \
  --mask '?l?l?l?l?l?l?l?l'
```

Run with an explicitly selected backend:

```shell
cj crack \
  --backend cubecl-vulkan \
  --algorithm sha256 \
  --target-file hashes.txt \
  --wordlist candidates.txt
```

## Benchmarking

`cj` measures GPU execution using backend-native timestamp mechanisms where available. Initialization, kernel compilation, buffer allocation, and host-to-device transfers are excluded from the primary GPU hashrate measurement.

Benchmark output includes the backend, device, algorithm, workload parameters, candidate count, GPU execution time, and calculated hashrate. Raw samples are retained so results can be independently analyzed.

## Project Status

`cj` is in early development. The current priorities are:

1. Define the common backend interface.
2. Implement device discovery.
3. Implement and validate the first hash kernel.
4. Add native GPU timestamp measurement.
5. Add additional backends.
6. Implement automatic backend selection.

## Responsible Use

`cj` is intended for authorized password recovery, security testing, and performance research. Only use it with hashes and systems that you own or have explicit permission to test.

## License

Licensed under either of the following licenses, at your option:

- Apache License, Version 2.0
- MIT License

See `LICENSE-APACHE` and `LICENSE-MIT` for details.
