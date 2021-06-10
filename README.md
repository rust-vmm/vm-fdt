# vm-fdt

The vm-fdt crate provides the ability to write Flattened Devicetree blobs as defined in the
[Devicetree specification](https://devicetree-specification.readthedocs.io/en/stable/flattened-format.html).

## Design

The [`FdtWriter` structure](src/writer.rs) provides an interface suitable
for dynamically generating a Devicetree blob at runtime.

## Usage

TODO: This section describes how the crate is used.

Some questions that might help in writing this section:
- What traits do users need to implement?
- Does the crate have any default/optional features? What is each feature
  doing?
- Is this crate used by other rust-vmm components? If yes, how?

This crate defines a development feature: `long_running_test`. This feature
SHOULD NOT be used in production as it might enable functionality that is safe
only for development use cases.

## Examples

The following is a simple example of creating a device tree with various
types of properties and finalizing it as a Devicetree Blob (DTB) contained
in a vector of bytes.

```rust
use vm_fdt::FdtWriter;

let mut fdt = FdtWriter::new(&[]);
let root_node = fdt.begin_node("")?;
fdt.property_string("compatible", "linux,dummy-virt")?;
fdt.property_u32("#address-cells", 0x2)?;
fdt.property_u32("#size-cells", 0x2)?;
let chosen_node = fdt.begin_node("chosen")?;
fdt.property_u32("linux,pci-probe-only", 1)?;
fdt.property_string("bootargs", "panic=-1 console=hvc0 root=/dev/vda")?;
fdt.end_node(chosen_node)?;
fdt.end_node(root_node)?;
let dtb = fdt.finish(0x1000)?;
```

## License

This project is licensed under either of

- [Apache License](http://www.apache.org/licenses/LICENSE-2.0), Version 2.0
- [BSD-3-Clause License](https://opensource.org/licenses/BSD-3-Clause)
