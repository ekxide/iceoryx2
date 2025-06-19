// Copyright (c) 2025 Contributors to the Eclipse Foundation
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

/// Provides the recommended inter-process [`Event`](crate::event::Event) concept implementation
/// for the target.
#[cfg(not(target_os = "vxworks"))]
pub type Ipc = crate::event::unix_datagram_socket::EventImpl;

/// Provides the recommended inter-process [`Event`](crate::event::Event) concept implementation
/// for the target on VxWorks.
#[cfg(target_os = "vxworks")]
pub type Ipc = crate::event::sem_bitset_posix_shared_memory::Event;

/// Provides the recommended process-local [`Event`](crate::event::Event) concept implementation
/// for the target.
#[cfg(not(target_os = "vxworks"))]
pub type Local = crate::event::process_local_socketpair::EventImpl;

/// Provides the recommended process-local [`Event`](crate::event::Event) concept implementation
/// for the target.
#[cfg(target_os = "vxworks")]
pub type Local = crate::event::sem_bitset_posix_shared_memory::Event;
