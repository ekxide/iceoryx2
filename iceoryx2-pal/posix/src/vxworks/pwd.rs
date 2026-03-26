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

use core::ffi::CStr;
use iceoryx2_pal_concurrency_sync::WaitAction;
use iceoryx2_pal_concurrency_sync::strategy::mutex::Mutex;

use super::Errno;

pub unsafe fn getpwnam_r(
    _name: *const c_char,
    _pwd: *mut passwd,
    _buf: *mut c_char,
    _buflen: size_t,
    _result: *mut *mut passwd,
) -> int {
    // NOTE getpwnam_r return ENOSYS and reimplementing it with getpwnam does also not work since it also returns ENOSYS;
    //      on ARM the the function does not even exist and there is a linker error
    Errno::set(Errno::ENOSYS);
    -1
}

pub unsafe fn getpwuid_r(
    _uid: uid_t,
    _pwd: *mut passwd,
    _buf: *mut c_char,
    _buflen: size_t,
    _result: *mut *mut passwd,
) -> int {
    // NOTE getpwuid_r return ENOSYS and reimplementing it with getpwuid does also not work since it also returns ENOSYS;
    //      on ARM the the function does not even exist and there is a linker error
    Errno::set(Errno::ENOSYS);
    -1
}

struct GetGrWorkaround {}

impl GetGrWorkaround {
    unsafe fn getgrnam_r(
        name: *const c_char,
        grp: *mut group,
        buf: *mut c_char,
        buflen: size_t,
        result: *mut *mut group,
    ) -> int {
        unsafe {
            *result = core::ptr::null_mut();
            Errno::set(Errno::ESUCCES);
            // NOTE: getgrnam() Thread safety: MT-Unsafe race:grnam locale
            let src_grp = internal::getgrnam(name);
            if src_grp.is_null() {
                return Errno::get() as _;
            }

            let copy_result = Self::copy_group_struct(&*src_grp, &mut *grp, buf, buflen);

            if copy_result != 0 {
                return copy_result;
            }

            // Set the result pointer
            *result = grp;

            0
        }
    }

    pub unsafe fn getgrgid_r(
        gid: gid_t,
        grp: *mut group,
        buf: *mut c_char,
        buflen: size_t,
        result: *mut *mut group,
    ) -> int {
        unsafe {
            *result = core::ptr::null_mut();
            Errno::set(Errno::ESUCCES);
            // NOTE: getgrgid() Thread safety: MT-Unsafe race:grgid locale
            let src_grp = internal::getgrgid(gid);
            if src_grp.is_null() {
                return Errno::get() as _;
            }

            let copy_result = Self::copy_group_struct(&*src_grp, &mut *grp, buf, buflen);

            if copy_result != 0 {
                return copy_result;
            }

            // Set the result pointer
            *result = grp;

            0
        }
    }

    unsafe fn copy_group_struct(
        src: &group,
        dst: &mut group,
        buf: *mut c_char,
        buflen: size_t,
    ) -> int {
        let gr_name = unsafe { core::ffi::CStr::from_ptr(src.gr_name) };
        let gr_passwd = unsafe { core::ffi::CStr::from_ptr(src.gr_passwd) };
        let gr_mem = src.gr_mem;

        let mut required_buf_len = gr_name.to_bytes_with_nul().len();
        required_buf_len += gr_passwd.to_bytes_with_nul().len();

        let mut member = gr_mem;
        let mut member_count = 0;
        while !member.is_null() && unsafe { !(*member).is_null() } {
            let member_str = unsafe { CStr::from_ptr(*member) };
            required_buf_len += member_str.to_bytes_with_nul().len();
            member = unsafe { member.add(1) };
            member_count += 1;
        }

        const NULL_TERMINATION: usize = 1;
        required_buf_len += (member_count + NULL_TERMINATION) * core::mem::size_of::<*mut c_char>()
            + (core::mem::align_of::<*mut c_char>() - 1);

        if required_buf_len > buflen {
            return Errno::ERANGE as _;
        }

        dst.gr_gid = src.gr_gid;

        // get memory for group member pointer array
        let mut buf = unsafe { buf.add(buf.align_offset(core::mem::align_of::<*mut c_char>())) };
        dst.gr_mem = buf as *mut *mut _;
        buf = unsafe {
            buf.add((member_count + NULL_TERMINATION) * core::mem::size_of::<*mut c_char>())
        };

        // copy group name
        unsafe {
            core::ptr::copy_nonoverlapping(
                gr_name.as_ptr(),
                buf,
                gr_name.to_bytes_with_nul().len(),
            );
        }
        dst.gr_name = buf;
        buf = unsafe { buf.add(gr_name.to_bytes_with_nul().len()) };

        // copy group passwd
        unsafe {
            core::ptr::copy_nonoverlapping(
                gr_passwd.as_ptr(),
                buf,
                gr_passwd.to_bytes_with_nul().len(),
            );
        }
        dst.gr_passwd = buf;
        buf = unsafe { buf.add(gr_passwd.to_bytes_with_nul().len()) };

        // copy group members
        let mut member = gr_mem;
        let mut member_index = 0;
        while !member.is_null() && unsafe { !(*member).is_null() } {
            let member_str = unsafe { CStr::from_ptr(*member) };
            unsafe {
                core::ptr::copy_nonoverlapping(
                    member_str.as_ptr(),
                    buf,
                    member_str.to_bytes_with_nul().len(),
                );
            }
            unsafe {
                dst.gr_mem.add(member_index).write(buf);
            }
            buf = unsafe { buf.add(member_str.to_bytes_with_nul().len()) };

            member_index += 1;
            member = unsafe { member.add(1) };
        }

        // write null termination to group member pointer array
        unsafe {
            dst.gr_mem.add(member_index).write(core::ptr::null_mut());
        }

        0
    }
}

static GETGR_MUTEX: Mutex = Mutex::new();

pub unsafe fn getgrnam_r(
    name: *const c_char,
    grp: *mut group,
    buf: *mut c_char,
    buflen: size_t,
    result: *mut *mut group,
) -> int {
    GETGR_MUTEX.lock(|_, _| WaitAction::Continue);
    let ret_val = unsafe { GetGrWorkaround::getgrnam_r(name, grp, buf, buflen, result) };
    GETGR_MUTEX.unlock(|_| {});

    ret_val
}

pub unsafe fn getgrgid_r(
    gid: gid_t,
    grp: *mut group,
    buf: *mut c_char,
    buflen: size_t,
    result: *mut *mut group,
) -> int {
    GETGR_MUTEX.lock(|_, _| WaitAction::Continue);
    let ret_val = unsafe { GetGrWorkaround::getgrgid_r(gid, grp, buf, buflen, result) };
    GETGR_MUTEX.unlock(|_| {});

    ret_val
}

mod internal {
    use super::*;

    unsafe extern "C" {
        pub(super) fn getgrnam(name: *const c_char) -> *mut group;

        pub(super) fn getgrgid(gid: gid_t) -> *mut group;
    }
}
