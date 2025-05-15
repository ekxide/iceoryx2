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

use std::sync::Mutex;

pub unsafe fn getpwnam_r(
    name: *const c_char,
    pwd: *mut passwd,
    buf: *mut c_char,
    buflen: size_t,
    result: *mut *mut passwd,
) -> int {
    // NOTE getpwnam_r return ENOSYS and reimplementing it with getpwnam does also not work since it also returns ENOSYS
    internal::getpwnam_r(name, pwd, buf, buflen, result)
}

pub unsafe fn getpwuid_r(
    uid: uid_t,
    pwd: *mut passwd,
    buf: *mut c_char,
    buflen: size_t,
    result: *mut *mut passwd,
) -> int {
    // NOTE getpwuid_r return ENOSYS and reimplementing it with getpwuid does also not work since it also returns ENOSYS
    internal::getpwuid_r(uid, pwd, buf, buflen, result)
}

struct GetGrWorkaround {}

impl GetGrWorkaround {
    unsafe fn getgrnam_r(
        &self,
        name: *const c_char,
        grp: *mut group,
        _buf: *mut c_char,
        _buflen: size_t,
        result: *mut *mut group,
    ) -> int {
        // FIXME: this is broken and we must copy the data from from the returned group into the provided buffer
        unsafe {
            // NOTE: getgrnam() Thread safety: MT-Unsafe race:grnam locale
            let gr = internal::getgrnam(name);
            if gr.is_null() {
                return -1;
            }

            // Copy the group struct and buffer
            core::ptr::copy_nonoverlapping(gr, grp, 1);

            // Set the result pointer
            *result = grp;

            0
        }
    }

    pub unsafe fn getgrgid_r(
        &self,
        gid: gid_t,
        grp: *mut group,
        _buf: *mut c_char,
        _buflen: size_t,
        result: *mut *mut group,
    ) -> int {
        // FIXME: this is broke and we must copy the data from from the returned group into the provided buffer
        unsafe {
            // NOTE: getgrgid() Thread safety: MT-Unsafe race:grgid locale
            let gr = internal::getgrgid(gid);
            if gr.is_null() {
                return -1;
            }

            // Copy the group struct and buffer
            core::ptr::copy_nonoverlapping(gr, grp, 1);

            // Set the result pointer
            *result = grp;

            0
        }
    }
}

static GETGR_MUTEX: Mutex<GetGrWorkaround> = Mutex::new(GetGrWorkaround {});

pub unsafe fn getgrnam_r(
    name: *const c_char,
    grp: *mut group,
    buf: *mut c_char,
    buflen: size_t,
    result: *mut *mut group,
) -> int {
    GETGR_MUTEX
        .lock()
        .unwrap()
        .getgrnam_r(name, grp, buf, buflen, result)
}

pub unsafe fn getgrgid_r(
    gid: gid_t,
    grp: *mut group,
    buf: *mut c_char,
    buflen: size_t,
    result: *mut *mut group,
) -> int {
    GETGR_MUTEX
        .lock()
        .unwrap()
        .getgrgid_r(gid, grp, buf, buflen, result)
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

        pub(super) fn getgrnam(name: *const c_char) -> *mut group;

        pub(super) fn getgrgid(gid: gid_t) -> *mut group;
    }
}
