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

## Real Apple Metal support already exists -- it lives elsewhere

This crate stays an intentional dead end. Magnetar's real, verified path
to Apple Metal is
[`providers/wgpu`](https://github.com/astorise/Magnetar-provider-WGPU): a
cross-platform GPU compute Provider built on `wgpu`, whose real device
discovery and compute Kernels are verified on real Vulkan-backed hardware
(this repository's own development machine) and which selects the real
Metal backend automatically on macOS, with zero Metal-specific code of
its own. That crate's own README explains the reasoning in full -- this
crate exists as the documented record of why hand-written Metal FFI was
rejected in favor of it.

## What a future contributor with real macOS/Apple Silicon hardware would need to do

1. First, verify `providers/wgpu` actually works correctly through its
   real Metal backend on real Apple Silicon -- its own real Vulkan-backed
   tests have never been confirmed to carry over to Metal in practice,
   only assumed to on `wgpu`'s own cross-backend consistency guarantees.
   This is very likely the higher-value next step for most real workloads
   before anything below.
2. Only if `wgpu`'s abstraction overhead turns out to matter for a real,
   measured workload: add a real, hand-written Metal binding dependency
   (the `metal`/`objc2-metal` crates are the established choices in the
   Rust ecosystem) behind `#[cfg(target_os = "macos")]` in *this* crate,
   with its current unconditional fallback kept for every other target.
3. Real device discovery via `MTLCopyAllDevices()`, building
   `magnetar_runtime::device::DeviceDescriptor` values from it, mirroring
   `providers/cuda`'s own `device.rs`.
4. Real compute Kernels via Metal Shading Language compiled through
   `MTLDevice::newLibraryWithSource`, and `ProviderExecutionApi`, mirroring
   `providers/cuda`'s own `CudaKernels`/`CudaExecutor` structure.
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
