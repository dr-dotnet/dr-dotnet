#![feature(
    const_fn_floating_point_arithmetic,
    const_fn_trait_bound,
    // The next one is necessary to prevent E0658, at least currently
    const_refs_to_cell,
    const_trait_impl
)]

mod profilers;
mod report;
mod interop;