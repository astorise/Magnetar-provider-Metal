# magnetar-provider-metal

## Purpose

An Apple Metal execution Provider for the
[Magnetar](https://github.com/astorise/Magnetar) local AI Runtime,
implementing Magnetar's `Provider` contract for Apple GPU Devices.

## Status

**Structural placeholder, unconditionally unavailable on every platform.**
Unlike
[`providers/rocm`](https://github.com/astorise/Magnetar-provider-ROCm)'s
`RocmProvider` (a real device-discovery skeleton that dynamically loads
the real HIP runtime library and genuinely detects its absence), Metal is
an Apple operating-system framework, not a redistributable vendor driver
library -- there is no `dlopen`-and-see-if-it-fails story for Metal on a
non-macOS host, because the framework does not exist there at all, in any
form. No macOS development environment or CI runner exists anywhere in
this repository's current tooling.

`MetalProvider::new()` always constructs successfully and unconditionally
reports itself unavailable (zero Devices,
[`ProviderHealth::Unavailable`](https://github.com/astorise/Magnetar)),
on every platform. This is a real, honestly-verified statement of what
this crate does today -- confirmed by its own tests, which pass the same
way on every platform this repository's own tooling runs on -- not a
placeholder standing in for untested real Metal FFI code. Writing
`#[cfg(target_os = "macos")]`-gated Metal framework bindings here would be
code this repository's own tooling could never even type-check, let alone
verify against real Apple Silicon hardware -- exactly the kind of
confident-but-untested work this repository's development practice avoids
everywhere else.

## Relationship to `providers/wgpu` -- not a dead end, a real future need

[`providers/wgpu`](https://github.com/astorise/Magnetar-provider-WGPU)
gives Magnetar a real, portable path to Apple GPUs today (Vulkan-verified
here, selecting the real Metal backend automatically on macOS). But WGSL
compute shaders -- what `wgpu` compiles down to Metal Shading Language --
cannot reach Apple Silicon's `simdgroup_matrix` matrix-multiply-accumulate
instructions (the M-series equivalent of NVIDIA Tensor Cores) or route
through Metal Performance Shaders to the AMX matrix coprocessor. Neither
is exposed through `wgpu`'s cross-platform abstraction at all. This
matters unevenly across the two phases of LLM inference:

- **Decode** (autoregressive, one token at a time): memory-bandwidth-bound,
  not compute-bound -- `wgpu` should perform close to native Metal here,
  since Apple's unified memory bandwidth is a hardware property `wgpu`
  reaches the same way native Metal does.
- **Prefill** (processing the prompt): compute-bound -- this is exactly
  where `simdgroup_matrix`/AMX/MPS access matters most, and where a
  `wgpu`-only path is expected to be substantially slower than native
  Metal or Apple's own MLX framework. Not measured by this repository
  (no Apple Silicon hardware available anywhere in its tooling) -- a real,
  expected gap stated honestly, not a measured one.

So this crate is real, wanted future work, specifically for prefill's
compute-bound kernels -- not superseded by `providers/wgpu`, which covers
decode and cross-platform portability well but structurally cannot reach
Apple's own matrix hardware. **Call for contributors**: if you have real
Apple Silicon (M-series) hardware, verifying `providers/wgpu`'s Metal
backend for correctness and measuring the real prefill slowdown against
native Metal/MLX would be immediately useful and does not require writing
any new code -- see the numbered list below for what comes after that.

## What a future contributor with real macOS/Apple Silicon hardware would need to do

1. First, verify `providers/wgpu` actually works correctly through its
   real Metal backend on real Apple Silicon, and measure the real prefill
   throughput gap against native Metal/MLX for a representative model --
   turning the expected-but-unmeasured gap above into a real number.
2. Add a real, hand-written Metal binding dependency (the `metal`/
   `objc2-metal` crates are the established choices in the Rust ecosystem)
   behind `#[cfg(target_os = "macos")]` in *this* crate, with its current
   unconditional fallback kept for every other target.
3. Real device discovery via `MTLCopyAllDevices()`, building
   `magnetar_runtime::device::DeviceDescriptor` values from it, mirroring
   `providers/cuda`'s own `device.rs`.
4. Real compute Kernels via Metal Shading Language using `simdgroup_matrix`
   directly (or routing through Metal Performance Shaders for GEMM, to
   reach the AMX coprocessor) for the compute-bound prefill kernels
   specifically (`matmul` above all), and `ProviderExecutionApi`,
   mirroring `providers/cuda`'s own `CudaKernels`/`CudaExecutor`
   structure. Decode-phase, memory-bound kernels may not be worth
   re-implementing natively at all, given step 1's own expectation that
   `providers/wgpu` already performs close to native there.
5. Verify every Kernel against `providers/cpu`'s reference implementation
   *and* `providers/wgpu`'s own Metal-backend output on real Apple Silicon
   hardware -- and set up a macOS CI runner (this repository has none
   today; `providers/cuda`'s own real-hardware verification runs on a
   self-hosted `arc-gpu-magnetar` runner, the precedent to follow for a
   macOS equivalent).

## Governing contract

Implements Magnetar's generic `Provider`/`Device`/`ProviderExecutionApi`
contracts from the main [Magnetar](https://github.com/astorise/Magnetar)
repository's `magnetar-runtime` crate. No dedicated OpenSpec capability
exists yet for this crate specifically; one should be scoped in the main
repository's `openspec/specs/` once real Kernel execution work here
begins, the same way `providers/cuda`'s own `cuda-provider` capability was.

## Relationship to magnetar-runtime

This Provider is loaded and driven by the Runtime's own Provider registry,
never the reverse -- `magnetar-runtime` has zero compile-time dependency
on this crate (the same externalization invariant every Provider/Component/
Format module in this workspace observes). It is pinned into the main
Magnetar repository as a git submodule at `providers/metal`.
