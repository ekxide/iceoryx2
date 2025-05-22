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

#![allow(non_camel_case_types)]
#![allow(clippy::missing_safety_doc)]

use iceoryx2_pal_concurrency_sync::semaphore::Semaphore;
use iceoryx2_pal_concurrency_sync::{WaitAction, WaitResult};

use crate::posix::*;

pub unsafe fn sem_create(
    _name: *const c_char,
    _oflag: int,
    _mode: mode_t,
    _value: uint,
) -> *mut sem_t {
    SEM_FAILED
}

pub unsafe fn sem_post(sem: *mut sem_t) -> int {
    if (*sem).semaphore.value() == u32::MAX {
        Errno::set(Errno::EOVERFLOW);
        return -1;
    }

    (*sem).semaphore.post(|_| {}, 1);

    Errno::set(Errno::ESUCCES);
    0
}

pub unsafe fn sem_wait(sem: *mut sem_t) -> int {
    (*sem).semaphore.wait(|_atomic, _value| -> WaitAction {
        let mut current_time = timespec::new_zeroed();
        current_time.tv_sec = 0;
        current_time.tv_nsec = 1000000;

        libc::nanosleep(&current_time, core::ptr::null_mut());
        WaitAction::Continue
    });

    Errno::set(Errno::ESUCCES);
    0
}

pub unsafe fn sem_trywait(sem: *mut sem_t) -> int {
    match (*sem).semaphore.try_wait() {
        WaitResult::Success => {
            Errno::set(Errno::ESUCCES);
            0
        }
        WaitResult::Interrupted => {
            Errno::set(Errno::EAGAIN);
            -1
        }
    }
}

pub unsafe fn sem_timedwait(sem: *mut sem_t, abs_timeout: *const timespec) -> int {
    let mut current_time = timespec::new_zeroed();
    let mut wait_time = timespec::new_zeroed();

    loop {
        if sem_trywait(sem) == 0 {
            return 0;
        }

        clock_gettime(CLOCK_REALTIME, &mut current_time);

        if (current_time.tv_sec > (*abs_timeout).tv_sec)
            || (current_time.tv_sec == (*abs_timeout).tv_sec
                && current_time.tv_nsec > (*abs_timeout).tv_nsec)
        {
            Errno::set(Errno::ETIMEDOUT);
            return -1;
        }

        current_time.tv_sec = 0;
        current_time.tv_nsec = 1000000;

        libc::nanosleep(&current_time, &mut wait_time);
    }
}

pub unsafe fn sem_unlink(_name: *const c_char) -> int {
    -1
}

pub unsafe fn sem_open(_name: *const c_char, _oflag: int) -> *mut sem_t {
    SEM_FAILED
}

pub unsafe fn sem_close(_sem: *mut sem_t) -> int {
    -1
}

pub unsafe fn sem_destroy(_sem: *mut sem_t) -> int {
    0
}

pub unsafe fn sem_init(sem: *mut sem_t, _pshared: int, value: uint) -> int {
    (*sem).semaphore = Semaphore::new(value as _);
    Errno::set(Errno::ESUCCES);
    0
}
