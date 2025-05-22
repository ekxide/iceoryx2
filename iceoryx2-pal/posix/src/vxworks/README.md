# Limitations

## Number of Endpoints

* the default number of shared memory segments is limited
* this lead to a limited number of endpoints
* system limit can be increased in custom VxWorks images
    * if the limit cannot sufficiently be increased, multiple endpoints need to
      be grouped into one shared memory segment

## Events

* VxWorks does not support Unix Domain Sockets with the connectionless
  `SOCK_DGRAM`, only with the connection-based `SOCK_SEQPACKET`
* unnamed semaphores are also not supported for inter-process communication
    * according to the manpages, `pshared` of `sem_init` will be ignored
* events are currently implemented by a spin based semaphore since the unnamed
  semaphores are not inter-process capable
* an event concept implementation based on named semaphores is required

## User Management

* `getpwnam_r`, `getpwuid_r`, `getgrnam_r` and `getgrgid_r` are not available
* `getpwnam` and `getpwuid` can be activated with the user management feature
* `getpwnam`, `getpwuid`, `getgrnam` and `getgrgid` are not thread-safe
    * iceoryx2 calls this functions with `uid` and `gid` of the process and
      therefore does not trigger data races
        * for iceoryx2 itself, they can be made thread-safe but even for calls
          with arbitrary `uid` and `gid`
    * this cannot be guaranteed if there are calls to these functions outside
      of iceoryx2
    * ideally WindRiver would provide this functionality
* this is only requires for zero-trust deployments
    * if it can be guaranteed by other means that only authorized applications
      are running, this functionality is not requires

## iceoryx2 CLI

* non-core functionality
* the iceoryx2 CLI tools, e.g. `iox2 node`, are not available for VxWorks due
  to some dependencies

## iceoryx2 Zenoh Tunnel

* non-core functionality
* Zenoh is not available for VxWorks

## Tests

* some tests use POSIX functionality which is not available on VxWorx
* some refactoring is required to make all tests run
    * e.g. `_SC_NPROCESSORS_CONF` is not available; alternatively
      `VxCpuEnabledGet()` can be used but that the platform abstraction needs
      some refactoring

## Conformity Check for Platform Abstraction

* some VxWorks system defines can be changed at compile time
* a conformity check is required to detect a mismatch with the Rust defines

## Process Local Communication

* the `socketpair` leads to linker errors
* semaphores are used for local events

## Credential Passing

* `SO_PASSCRED` is not available on VxWorks
* the upcoming resource communication pattern requires a custom implementation
  on VxWorks
