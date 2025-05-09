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

#![allow(non_camel_case_types, non_snake_case)]
#![allow(clippy::missing_safety_doc)]
use crate::posix::types::*;

pub unsafe fn getpwnam_r(
    name: *const c_char,
    pwd: *mut passwd,
    buf: *mut c_char,
    buflen: size_t,
    result: *mut *mut passwd,
) -> int {
    internal::getpwnam_r(name, pwd, buf, buflen, result)
}

pub unsafe fn getpwuid_r(
    uid: uid_t,
    pwd: *mut passwd,
    buf: *mut c_char,
    buflen: size_t,
    result: *mut *mut passwd,
) -> int {
    internal::getpwuid_r(uid, pwd, buf, buflen, result)
}

pub unsafe fn getgrnam_r(
    _name: *const c_char,
    _grp: *mut group,
    _buf: *mut c_char,
    _buflen: size_t,
    _result: *mut *mut group,
) -> int {
    // libc::getgrnam_r(name, grp, buf, buflen, result)
    todo!() // FIXME HIGH PRIO getgrnam_r is not available; can getgrnam be made thread-safe
}

// TODO: check if only multiple calls to getgrgid are not thread safe and if a local mutex could help
pub unsafe fn getgrgid_r(
    _gid: gid_t,
    _grp: *mut group,
    _buf: *mut c_char,
    _buflen: size_t,
    _result: *mut *mut group,
) -> int {
    // libc::getgrgid_r(gid, grp, buf, buflen, result)
    todo!() // FIXME HIGH PRIO getgrgid_r is not available; can getgrgid be made thread-safe
}

mod internal {
    use super::*;

    extern "C" {
        pub(super) fn getpwnam_r(
            name: *const c_char,
            pwd: *mut passwd,
            buf: *mut c_char,
            buflen: size_t,
            result: *mut *mut passwd,
        ) -> int;

        pub(super) fn getpwuid_r(
            uid: uid_t,
            pwd: *mut passwd,
            buf: *mut c_char,
            buflen: size_t,
            result: *mut *mut passwd,
        ) -> int;
    }
}
