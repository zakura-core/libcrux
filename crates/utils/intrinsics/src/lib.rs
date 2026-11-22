#![no_std]

#[cfg(all(hax, not(feature = "hax")))]
compile_error!(
    "building with `--cfg hax` (e.g. `cargo hax`) requires the `libcrux-intrinsics/hax` feature (or `hax` on the libcrux crate you depend on)"
);

#[cfg(all(feature = "simd128", not(hax)))]
pub mod arm64;
#[cfg(all(feature = "simd256", not(hax)))]
pub mod avx2;

// When extracting F* we only want dummy files here.
#[cfg(all(feature = "simd128", hax))]
pub mod arm64_extract;
#[cfg(all(feature = "simd128", hax))]
pub use arm64_extract as arm64;

#[cfg(all(feature = "simd256", hax, pre_core_models))]
pub mod avx2_extract;
#[cfg(all(feature = "simd256", hax, pre_core_models))]
pub use avx2_extract as avx2;

#[cfg(all(feature = "simd256", hax, not(pre_core_models)))]
pub mod avx2;
