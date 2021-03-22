// Copyright 2021 The Chromium OS Authors. All rights reserved.
// SPDX-License-Identifier: Apache-2.0 OR BSD-3-Clause

#![deny(missing_docs)]

//! This crate provides the ability to manipulate Flattened Devicetree blobs.

mod writer;

pub use writer::FdtWriter;
pub use writer::Result as FdtWriterResult;

/// Magic number used in the FDT header.
const FDT_MAGIC: u32 = 0xd00dfeed;

const FDT_BEGIN_NODE: u32 = 0x00000001;
const FDT_END_NODE: u32 = 0x00000002;
const FDT_PROP: u32 = 0x00000003;
const FDT_END: u32 = 0x00000009;
