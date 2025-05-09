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

use crate::posix::types::*;

// TODO: create a C test lib which has a ton of functions which just return the constants and size and alignment of types
//       create tests that call these functions and compare the result to the values defined here
//       similarly, return the enum values and compare them to the ones set here

pub const CPU_SETSIZE: usize = core::mem::size_of::<libc::cpuset_t>() * 8; // CPU_SETSIZE is the size in bit
pub const FD_SETSIZE: usize = core::mem::size_of::<fd_set>() * 8; // FD_SETSIZE is the size in bit
pub const NULL_TERMINATOR: c_char = 0;

pub const USER_NAME_LENGTH: usize = 31;

pub const GROUP_NAME_LENGTH: usize = 31;

pub const O_RDONLY: int = libc::O_RDONLY as _;
pub const O_WRONLY: int = libc::O_WRONLY as _;
pub const O_RDWR: int = libc::O_RDWR as _;

pub const O_CREAT: int = libc::O_CREAT as _;
pub const O_EXCL: int = libc::O_EXCL as _;
pub const O_NOCTTY: int = 0x8000; // from vxsdk/sysroot/usr/h/public/sys/fcntlcom.h
pub const O_APPEND: int = libc::O_APPEND as _;
pub const O_NONBLOCK: int = libc::O_NONBLOCK as _;
pub const O_DIRECTORY: int = -1; // NOTE: not available; TODO use Option<int> to make this visible

pub const F_RDLCK: int = 1; // from vxsdk/sysroot/usr/h/public/sys/fcntlcom.h
pub const F_WRLCK: int = 2; // from vxsdk/sysroot/usr/h/public/sys/fcntlcom.h
pub const F_UNLCK: int = 3; // from vxsdk/sysroot/usr/h/public/sys/fcntlcom.h
pub const F_GETFD: int = libc::F_GETFD as _;
pub const F_GETFL: int = libc::F_GETFL as _;
pub const F_SETFL: int = libc::F_SETFL as _;
pub const F_GETLK: int = libc::F_GETLK as _;
pub const F_SETLK: int = libc::F_SETLK as _;
pub const F_SETLKW: int = libc::F_SETLKW as _;

pub const PROT_NONE: int = libc::PROT_NONE as _;
pub const PROT_READ: int = libc::PROT_READ as _;
pub const PROT_WRITE: int = libc::PROT_WRITE as _;
pub const PROT_EXEC: int = libc::PROT_EXEC as _;
pub const MCL_CURRENT: int = 0x1; // from vxsdk/sysroot/usr/h/public/sys/mman.h
pub const MCL_FUTURE: int = 0x2; // from vxsdk/sysroot/usr/h/public/sys/mman.h
pub const MAP_SHARED: int = libc::MAP_SHARED as _;
pub const MAP_FAILED: *mut void = u64::MAX as *mut void;

pub const PTHREAD_BARRIER_SERIAL_THREAD: int = 1; // from vxsdk/sysroot/usr/h/public/pthread.h
pub const PTHREAD_EXPLICIT_SCHED: int = 1; // from vxsdk/sysroot/usr/h/public/pthread.h
pub const PTHREAD_INHERIT_SCHED: int = 0; // from vxsdk/sysroot/usr/h/public/pthread.h

pub const MAX_SIGNAL_VALUE: usize = 32;

pub const SO_PASSCRED: int = -1; // NOTE: not available
pub const SO_PEERCRED: int = -1; // NOTE: not available
pub const SCM_CREDENTIALS: int = 0x02;

pub const PTHREAD_MUTEX_NORMAL: int = libc::PTHREAD_MUTEX_NORMAL as _;
pub const PTHREAD_MUTEX_RECURSIVE: int = libc::PTHREAD_MUTEX_RECURSIVE as _;
pub const PTHREAD_MUTEX_ERRORCHECK: int = libc::PTHREAD_MUTEX_ERRORCHECK as _;
pub const PTHREAD_MUTEX_STALLED: int = 0; // from vxsdk/sysroot/usr/h/public/pthread.h
pub const PTHREAD_MUTEX_ROBUST: int = 4; // from vxsdk/sysroot/usr/h/public/pthread.h

pub const _SC_UIO_MAXIOV: int = -1; // NOTE: not available
pub const _SC_IOV_MAX: int = 23;

pub const _SC_AVPHYS_PAGES: int = -1; // NOTE: not available
pub const _SC_PASS_MAX: int = -1; // NOTE: not available
pub const _SC_XOPEN_XPG2: int = -1; // NOTE: not available
pub const _SC_XOPEN_XPG3: int = -1; // NOTE: not available
pub const _SC_XOPEN_XPG4: int = -1; // NOTE: not available
pub const _SC_NZERO: int = -1; // NOTE: not available

pub const _SC_XBS5_ILP32_OFF32: int = 95;
pub const _SC_XBS5_ILP32_OFFBIG: int = 96;
pub const _SC_XBS5_LP64_OFF64: int = 97;
pub const _SC_XBS5_LPBIG_OFFBIG: int = 98;

pub const _SC_STREAMS: int = -1; // NOTE: not available
pub const _SC_V7_ILP32_OFF32: int = -1; // NOTE: not available
pub const _SC_V7_ILP32_OFFBIG: int = -1; // NOTE: not available
pub const _SC_V7_LP64_OFF64: int = -1; // NOTE: not available
pub const _SC_V7_LPBIG_OFFBIG: int = -1; // NOTE: not available

pub const _SC_SS_REPL_MAX: int = 58;
pub const _SC_TRACE_EVENT_NAME_MAX: int = 81;
pub const _SC_TRACE_NAME_MAX: int = 84;
pub const _SC_TRACE_SYS_MAX: int = 85;

pub const _SC_THREAD_ROBUST_PRIO_INHERIT: int = -1; // NOTE: not available
pub const _SC_THREAD_ROBUST_PRIO_PROTECT: int = -2; // NOTE: not available

pub const _PC_SOCK_MAXBUF: int = -1; // NOTE: not available
pub const _PC_2_SYMLINKS: int = 1;
pub const _SC_TRACE_USER_EVENT_MAX: int = 86;

pub const PTHREAD_PROCESS_PRIVATE: int = 1; // from vxsdk/sysroot/usr/h/public/pthread.h
pub const PTHREAD_PROCESS_SHARED: int = 2; // from vxsdk/sysroot/usr/h/public/pthread.h
pub const PTHREAD_PRIO_NONE: int = libc::PTHREAD_PRIO_NONE as _;
pub const PTHREAD_PRIO_INHERIT: int = libc::PTHREAD_PRIO_INHERIT as _;
pub const PTHREAD_PRIO_PROTECT: int = libc::PTHREAD_PRIO_PROTECT as _;

pub const RLIMIT_CPU: __rlim_t = 0;
pub const RLIMIT_FSIZE: __rlim_t = 1;
pub const RLIMIT_DATA: __rlim_t = 2;
pub const RLIMIT_STACK: __rlim_t = 3;
pub const RLIMIT_CORE: __rlim_t = 4;
pub const RLIMIT_RSS: __rlim_t = 5;
pub const RLIMIT_NPROC: __rlim_t = 6;
pub const RLIMIT_NOFILE: __rlim_t = 7;
pub const RLIMIT_MEMLOCK: __rlim_t = 8;
pub const RLIMIT_AS: __rlim_t = 9;
pub const RLIMIT_LOCKS: __rlim_t = 10;
pub const RLIMIT_SIGPENDING: __rlim_t = 11;
pub const RLIMIT_MSGQUEUE: __rlim_t = 12;
pub const RLIMIT_NICE: __rlim_t = 13;
pub const RLIMIT_RTPRIO: __rlim_t = 14;
pub const RLIMIT_RTTIME: __rlim_t = 15;
pub const RLIMIT_NLIMITS: __rlim_t = 16;
pub const RLIMIT_INFINITY: __rlim_t = __rlim_t::MAX;

pub const SCHED_OTHER: int = libc::SCHED_OTHER as _;
pub const SCHED_FIFO: int = libc::SCHED_FIFO as _;
pub const SCHED_RR: int = libc::SCHED_RR as _;

pub const SEEK_SET: int = libc::SEEK_SET as _;
pub const SEEK_CUR: int = libc::SEEK_CUR as _;
pub const SEEK_END: int = libc::SEEK_END as _;

pub const SEM_FAILED: *mut sem_t = 0 as *mut sem_t;

pub const SIGABRT: int = libc::SIGABRT as _;
pub const SIGALRM: int = libc::SIGALRM as _;
pub const SIGBUS: int = libc::SIGBUS as _;
pub const SIGCHLD: int = libc::SIGCHLD as _;
pub const SIGCONT: int = libc::SIGCONT as _;
pub const SIGFPE: int = libc::SIGFPE as _;
pub const SIGHUP: int = libc::SIGHUP as _;
pub const SIGILL: int = libc::SIGILL as _;
pub const SIGINT: int = libc::SIGINT as _;
pub const SIGKILL: int = libc::SIGKILL as _;
pub const SIGPIPE: int = libc::SIGPIPE as _;
pub const SIGQUIT: int = libc::SIGQUIT as _;
pub const SIGSEGV: int = libc::SIGSEGV as _;
pub const SIGSTOP: int = libc::SIGSTOP as _;
pub const SIGTERM: int = libc::SIGTERM as _;
pub const SIGTSTP: int = libc::SIGTSTP as _;
pub const SIGTTIN: int = libc::SIGTTIN as _;
pub const SIGTTOU: int = libc::SIGTTOU as _;
pub const SIGUSR1: int = libc::SIGUSR1 as _;
pub const SIGUSR2: int = libc::SIGUSR2 as _;
pub const SIGPROF: int = libc::SIGPROF as _;
pub const SIGSYS: int = libc::SIGSYS as _;
pub const SIGTRAP: int = libc::SIGTRAP as _;
pub const SIGURG: int = libc::SIGURG as _;
pub const SIGVTALRM: int = libc::SIGVTALRM as _;
pub const SIGXCPU: int = libc::SIGXCPU as _;
pub const SIGXFSZ: int = libc::SIGXFSZ as _;
pub const SIG_ERR: sighandler_t = sighandler_t::MAX;
pub const SIG_DFL: int = 0;
pub const SIG_IGN: int = 1;
pub const SA_RESTART: int = libc::SA_RESTART as _;

pub const AF_LOCAL: sa_family_t = libc::AF_UNIX as _;
pub const AF_UNIX: sa_family_t = libc::AF_UNIX as _;
pub const AF_INET: sa_family_t = libc::AF_INET as _;
pub const PF_INET: sa_family_t = libc::AF_INET as _; // according to vxsdk/sysroot/usr/h/public/sys/socket.h it is the same as AF_INET
pub const PF_LOCAL: sa_family_t = libc::AF_UNIX as _;
pub const PF_UNIX: sa_family_t = libc::AF_UNIX as _;
pub const INADDR_ANY: in_addr_t = 0;
pub const SO_SNDBUF: int = libc::SO_SNDBUF as _;
pub const SO_RCVBUF: int = libc::SO_RCVBUF as _;
pub const SO_RCVTIMEO: int = libc::SO_RCVTIMEO as _;
pub const SO_SNDTIMEO: int = libc::SO_SNDTIMEO as _;
pub const SOCK_STREAM: int = libc::SOCK_STREAM as _;
pub const SOCK_DGRAM: int = libc::SOCK_DGRAM as _;
pub const IPPROTO_UDP: int = 17; // from vxsdk/sysroot/usr/h/public/netinet/in.h
pub const SOCK_NONBLOCK: int = O_NONBLOCK;
pub const MSG_PEEK: int = libc::MSG_PEEK as _;
pub const SCM_MAX_FD: u32 = 253; // TODO not sure where this is from; could not find it on Linux or VxWorks; the value is from the iceoryx2 Linux abstraction but all abstraction set it to 253
pub const SCM_RIGHTS: int = -1 as _; // NOTE: not available; TODO use Option<int> to make this visible
pub const SOL_SOCKET: int = libc::SOL_SOCKET as _;
pub const SUN_PATH_LEN: usize = 108;
pub const SA_DATA_LEN: usize = 14;

pub const S_IFMT: mode_t = libc::S_IFMT as _;
pub const S_IFSOCK: mode_t = libc::S_IFSOCK as _;
pub const S_IFLNK: mode_t = libc::S_IFLNK as _;
pub const S_IFREG: mode_t = libc::S_IFREG as _;
pub const S_IFBLK: mode_t = libc::S_IFBLK as _;
pub const S_IFDIR: mode_t = libc::S_IFDIR as _;
pub const S_IFCHR: mode_t = libc::S_IFCHR as _;
pub const S_IFIFO: mode_t = libc::S_IFIFO as _;
pub const S_IRWXU: mode_t = libc::S_IRWXU as _;
pub const S_IXUSR: mode_t = libc::S_IXUSR as _;
pub const S_IWUSR: mode_t = libc::S_IWUSR as _;
pub const S_IRUSR: mode_t = libc::S_IRUSR as _;
pub const S_IRWXG: mode_t = libc::S_IRWXG as _;
pub const S_IXGRP: mode_t = libc::S_IXGRP as _;
pub const S_IWGRP: mode_t = libc::S_IWGRP as _;
pub const S_IRGRP: mode_t = libc::S_IRGRP as _;
pub const S_IRWXO: mode_t = libc::S_IRWXO as _;
pub const S_IXOTH: mode_t = libc::S_IXOTH as _;
pub const S_IWOTH: mode_t = libc::S_IWOTH as _;
pub const S_IROTH: mode_t = libc::S_IROTH as _;
pub const S_ISUID: mode_t = libc::S_ISUID as _;
pub const S_ISGID: mode_t = libc::S_ISGID as _;
pub const S_ISVTX: mode_t = libc::S_ISVTX as _;

pub const CLOCK_REALTIME: clockid_t = libc::CLOCK_REALTIME as _;
pub const CLOCK_MONOTONIC: clockid_t = libc::CLOCK_MONOTONIC as _;
pub const CLOCK_TIMER_ABSTIME: int = 1;

pub const F_OK: int = 0; // from vxsdk/sysroot/usr/h/public/unistd.h
pub const R_OK: int = 4; // from vxsdk/sysroot/usr/h/public/unistd.h
pub const W_OK: int = 2; // from vxsdk/sysroot/usr/h/public/unistd.h
pub const X_OK: int = 1; // from vxsdk/sysroot/usr/h/public/unistd.h

// NOTE: these are the indices of the '_VX_SYSCONF_VALUE' tags from vxsdk/sysroot/usr/h/public/unistd.h
pub const _SC_ARG_MAX: int = 4;
pub const _SC_CHILD_MAX: int = 12;
pub const _SC_CLK_TCK: int = 13;
pub const _SC_NGROUPS_MAX: int = 36;
pub const _SC_OPEN_MAX: int = 37;
pub const _SC_STREAM_MAX: int = 59;
pub const _SC_TZNAME_MAX: int = 89;
pub const _SC_JOB_CONTROL: int = 25;
pub const _SC_SAVED_IDS: int = 48;
pub const _SC_REALTIME_SIGNALS: int = 45;
pub const _SC_PRIORITY_SCHEDULING: int = 41;
pub const _SC_TIMERS: int = 78;
pub const _SC_ASYNCHRONOUS_IO: int = 5;
pub const _SC_PRIORITIZED_IO: int = 40;
pub const _SC_SYNCHRONIZED_IO: int = 61;
pub const _SC_FSYNC: int = 19;
pub const _SC_MAPPED_FILES: int = 28;
pub const _SC_MEMLOCK: int = 29;
pub const _SC_MEMLOCK_RANGE: int = 30;
pub const _SC_MEMORY_PROTECTION: int = 31;
pub const _SC_MESSAGE_PASSING: int = 32;
pub const _SC_SEMAPHORES: int = 51;
pub const _SC_SHARED_MEMORY_OBJECTS: int = 52;
pub const _SC_AIO_LISTIO_MAX: int = 0;
pub const _SC_AIO_MAX: int = 1;
pub const _SC_AIO_PRIO_DELTA_MAX: int = 2;
pub const _SC_DELAYTIMER_MAX: int = 17;
pub const _SC_MQ_OPEN_MAX: int = 34;
pub const _SC_MQ_PRIO_MAX: int = 35;
pub const _SC_VERSION: int = 94;
pub const _SC_PAGESIZE: int = 39;
pub const _SC_RTSIG_MAX: int = 47;
pub const _SC_SEM_NSEMS_MAX: int = 49;
pub const _SC_SEM_VALUE_MAX: int = 50;
pub const _SC_SIGQUEUE_MAX: int = 54;
pub const _SC_TIMER_MAX: int = 77;
pub const _SC_BC_BASE_MAX: int = 8;
pub const _SC_BC_DIM_MAX: int = 9;
pub const _SC_BC_SCALE_MAX: int = 10;
pub const _SC_BC_STRING_MAX: int = 11;
pub const _SC_COLL_WEIGHTS_MAX: int = 15;
pub const _SC_EXPR_NEST_MAX: int = 18;
pub const _SC_LINE_MAX: int = 26;
pub const _SC_RE_DUP_MAX: int = 43;
pub const _SC_2_VERSION: int = 122;
pub const _SC_2_C_BIND: int = 108;
pub const _SC_2_C_DEV: int = 109;
pub const _SC_2_FORT_DEV: int = 111;
pub const _SC_2_FORT_RUN: int = 112;
pub const _SC_2_SW_DEV: int = 120;
pub const _SC_2_LOCALEDEF: int = 113;
pub const _SC_THREADS: int = 75;
pub const _SC_THREAD_SAFE_FUNCTIONS: int = 71;
pub const _SC_GETGR_R_SIZE_MAX: int = 20;
pub const _SC_GETPW_R_SIZE_MAX: int = 21;
pub const _SC_LOGIN_NAME_MAX: int = 27;
pub const _SC_TTY_NAME_MAX: int = 87;
pub const _SC_THREAD_DESTRUCTOR_ITERATIONS: int = 65;
pub const _SC_THREAD_KEYS_MAX: int = 66;
pub const _SC_THREAD_STACK_MIN: int = 73;
pub const _SC_THREAD_THREADS_MAX: int = 74;
pub const _SC_THREAD_ATTR_STACKADDR: int = 62;
pub const _SC_THREAD_ATTR_STACKSIZE: int = 63;
pub const _SC_THREAD_PRIORITY_SCHEDULING: int = 69;
pub const _SC_THREAD_PRIO_INHERIT: int = 67;
pub const _SC_THREAD_PRIO_PROTECT: int = 68;
pub const _SC_THREAD_PROCESS_SHARED: int = 70;
pub const _SC_NPROCESSORS_CONF: int = -1 as _; // NOTE: not available; maybe CTL_HW with HW_NCPU can be used
pub const _SC_NPROCESSORS_ONLN: int = -1 as _; // NOTE: not available
pub const _SC_PHYS_PAGES: int = -1 as _; // NOTE: not available
pub const _SC_ATEXIT_MAX: int = 6;
pub const _SC_XOPEN_VERSION: int = 107;
pub const _SC_XOPEN_XCU_VERSION: int = -1 as _; // NOTE: not available
pub const _SC_XOPEN_UNIX: int = 106;
pub const _SC_XOPEN_CRYPT: int = 99;
pub const _SC_XOPEN_ENH_I18N: int = 100;
pub const _SC_XOPEN_SHM: int = 104;
pub const _SC_2_CHAR_TERM: int = 110;
pub const _SC_2_UPE: int = 121;
pub const _SC_XOPEN_LEGACY: int = 101;
pub const _SC_XOPEN_REALTIME: int = 102;
pub const _SC_XOPEN_REALTIME_THREADS: int = 103;
pub const _SC_ADVISORY_INFO: int = 3;
pub const _SC_BARRIERS: int = 7;
pub const _SC_CLOCK_SELECTION: int = 14;
pub const _SC_CPUTIME: int = 16;
pub const _SC_THREAD_CPUTIME: int = 64;
pub const _SC_MONOTONIC_CLOCK: int = 33;
pub const _SC_READER_WRITER_LOCKS: int = 44;
pub const _SC_SPIN_LOCKS: int = 56;
pub const _SC_REGEXP: int = 46;
pub const _SC_SHELL: int = 53;
pub const _SC_SPAWN: int = 55;
pub const _SC_SPORADIC_SERVER: int = 57;
pub const _SC_THREAD_SPORADIC_SERVER: int = 72;
pub const _SC_TIMEOUTS: int = 76;
pub const _SC_TYPED_MEMORY_OBJECTS: int = 88;
pub const _SC_2_PBS: int = 114;
pub const _SC_2_PBS_ACCOUNTING: int = 115;
pub const _SC_2_PBS_LOCATE: int = 117;
pub const _SC_2_PBS_MESSAGE: int = 118;
pub const _SC_2_PBS_TRACK: int = 119;
pub const _SC_SYMLOOP_MAX: int = 60;
pub const _SC_2_PBS_CHECKPOINT: int = 116;
pub const _SC_V6_ILP32_OFF32: int = 90;
pub const _SC_V6_ILP32_OFFBIG: int = 91;
pub const _SC_V6_LP64_OFF64: int = 92;
pub const _SC_V6_LPBIG_OFFBIG: int = 93;
pub const _SC_HOST_NAME_MAX: int = 22;
pub const _SC_TRACE: int = 79;
pub const _SC_TRACE_EVENT_FILTER: int = 80;
pub const _SC_TRACE_INHERIT: int = 82;
pub const _SC_TRACE_LOG: int = 83;
pub const _SC_IPV6: int = 24;
pub const _SC_RAW_SOCKETS: int = 42;
pub const _SC_XOPEN_STREAMS: int = 105;

// defined in vxsdk/sysroot/usr/h/public/unistd.h
pub const _PC_LINK_MAX: int = 6;
pub const _PC_MAX_CANON: int = 7;
pub const _PC_MAX_INPUT: int = 8;
pub const _PC_NAME_MAX: int = 9;
pub const _PC_PATH_MAX: int = 11;
pub const _PC_PIPE_BUF: int = 12;
pub const _PC_CHOWN_RESTRICTED: int = 4;
pub const _PC_NO_TRUNC: int = 10;
pub const _PC_VDISABLE: int = 20;
pub const _PC_SYNC_IO: int = 19;
pub const _PC_ASYNC_IO: int = 3;
pub const _PC_PRIO_IO: int = 13;
pub const _PC_FILESIZEBITS: int = 5;
pub const _PC_REC_INCR_XFER_SIZE: int = 14;
pub const _PC_REC_MAX_XFER_SIZE: int = 15;
pub const _PC_REC_MIN_XFER_SIZE: int = 16;
pub const _PC_REC_XFER_ALIGN: int = 17;
pub const _PC_ALLOC_SIZE_MIN: int = 2;
pub const _PC_SYMLINK_MAX: int = 18;
