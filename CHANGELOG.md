# v0.3.0

## Added
- [[#68](https://github.com/rust-vmm/vm-fdt/pull/68)] Implemented `no_std`
  support by adding a default `std` feature.

## Fixed
- [[#69](https://github.com/rust-vmm/vm-fdt/pull/69)] Fixed
  `clippy::incorrect_partial_ord_impl_on_ord_type`.

# v0.2.1

## Added
- [[#79](https://github.com/rust-vmm/vm-fdt/pull/70)] Added `relaxed_checks` feature
  that allows parsing non-standard device trees with node names starting with
  numbers or property names exceeding the standard 31-character limit. This
  enables compatibility with certain vendor-specific device trees that do not
  strictly follow the Device Tree Specification.

# v0.2.0

## Added
- [[#34](https://github.com/rust-vmm/vm-fdt/pull/34)] Implemented the `Error`
  trait for the errors defined in vm-fdt.
- [[#41](https://github.com/rust-vmm/vm-fdt/pull/41)] The threat model
  documentation is now available in the README.md.
- [[#37](https://github.com/rust-vmm/vm-fdt/issues/37)] Added
  `property_phandle` which is checking values for uniqueness.

## Fixed
- [[#32](https://github.com/rust-vmm/vm-fdt/issues/32)] Validate that node
  names are following the specification.
- [[#46](https://github.com/rust-vmm/vm-fdt/pull/46)] Fix potential overflow
  in `FdtWriter::begin_node()`.

# v0.1.0

This is the first release of vm-fdt.
The vm-fdt crate provides the ability to write Flattened Devicetree blobs.
