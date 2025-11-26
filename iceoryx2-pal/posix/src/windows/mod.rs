// Copyright (c) 2023 Contributors to the Eclipse Foundation
//
// See the NOTICE file(s) distributed with this work for additional
// information regarding copyright ownership.
//
// This program and the accompanying materials are made available under the
// terms of the Apache Software License 2.0 which is available at
// https://www.apache.org/licenses/LICENSE-2.0, or the MIT license
// which is available at https://opensource.org/licenses/MIT.
//
// SPDX-License-Identifier: Apache-2.0 OR MIT

pub mod constants;
pub mod dirent;
pub mod errno;
pub mod fcntl;
pub mod mman;
pub mod pthread;
pub mod pwd;
pub mod resource;
pub mod sched;
pub mod select;
pub mod semaphore;
#[doc(hidden)]
pub mod settings;
pub mod signal;
pub mod socket;
pub mod stat;
pub mod stdio;
pub mod stdlib;
pub mod string;
pub mod support;
pub mod time;
pub mod types;
pub mod unistd;
#[macro_use]
mod win32_call;
pub mod win32_handle_translator;
pub mod win32_security_attributes;
mod win32_udp_port_to_uds_name;

pub use crate::iox2_pal_posix::platform::constants::*;
pub use crate::iox2_pal_posix::platform::dirent::*;
pub use crate::iox2_pal_posix::platform::errno::*;
pub use crate::iox2_pal_posix::platform::fcntl::*;
pub use crate::iox2_pal_posix::platform::mman::*;
pub use crate::iox2_pal_posix::platform::pthread::*;
pub use crate::iox2_pal_posix::platform::pwd::*;
pub use crate::iox2_pal_posix::platform::resource::*;
pub use crate::iox2_pal_posix::platform::sched::*;
pub use crate::iox2_pal_posix::platform::select::*;
pub use crate::iox2_pal_posix::platform::semaphore::*;
pub use crate::iox2_pal_posix::platform::signal::*;
pub use crate::iox2_pal_posix::platform::socket::*;
pub use crate::iox2_pal_posix::platform::stat::*;
pub use crate::iox2_pal_posix::platform::stdio::*;
pub use crate::iox2_pal_posix::platform::stdlib::*;
pub use crate::iox2_pal_posix::platform::string::*;
pub use crate::iox2_pal_posix::platform::support::*;
pub use crate::iox2_pal_posix::platform::time::*;
pub use crate::iox2_pal_posix::platform::types::*;
pub use crate::iox2_pal_posix::platform::unistd::*;
