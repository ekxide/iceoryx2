# Setup The Development Environment and Build iceoryx2 on VxWorks

## Get and Setup VxWorks

For OSS, the [VxWorks Software Development Kit (SDK)](https://forums.windriver.com/t/vxworks-software-development-kit-sdk/43)
can be used via the non-commercial license agreement (NCLA).

> [!NOTE]
> It can only be used for non-commercial projects.

<!-- workaround for:  MD028/no-blanks-blockquote Blank line inside blockquote -->

> [!NOTE]
> The setup is done with VxWorks Version 25.03 and assumes the SDK to be
> extracted to `/opt/vxworks/wrsdk-vxworks7-qemu-25-03`.

```bash
cd /opt
sudo mkdir vxworks
sudo chown $USER:$USER vxworks
cd vxworks
wget https://d13321s3lxgewa.cloudfront.net/wrsdk-vxworks7-qemu-1.15.tar.bz2
tar -xjf wrsdk-vxworks7-qemu-1.15.tar.bz2
mv wrsdk-vxworks7-qemu wrsdk-vxworks7-qemu-25-03
```

To verify if the setup works, `sdkenv.sh` file needs to be sourced and the
`WIND_SDK_HOME` environment variable can be checked.

```bash
source /opt/vxworks/wrsdk-vxworks7-qemu-25-03/sdkenv.sh
echo $WIND_SDK_HOME
```

This should print `/opt/vxworks/wrsdk-vxworks7-qemu-25-03`.

An easy way to access the documentation are the man pages:

```bash
export MANPATH="/opt/vxworks/wrsdk-vxworks7-qemu-25-03/docs/vxworks-7/man:$MANPATH"
man sem_overview
man sem_wait
man shm_open
```

## Qemu Setup

In order to run the applications, `qemu` and `pyftpdlib` needs to be installed.
Refer to your distribution on how to install the dependencies.

The applications are made available to `qemu` via the `pyftpdlib` FTP server.

The easiest setup is to start the FTP server from the git-root of the icoryx2
workspace and share the VxWorks target folder.

```bash
cd path/to/iceoryx2
sudo python -m pyftpdlib -p 21 -u target -P vxTarget -i 127.0.0.1 -d $(pwd)/target/x86_64-wrs-vxworks
```

In case `bash` is not used but e.g. `fish`, which uses `()` instead of `$()`
for command substitution, `bash` can be invoked directly.

```bash
# with bash invocation
sudo bash -c "python -m pyftpdlib -p 21 -u target -P vxTarget -i 127.0.0.1 -d $(pwd)/target/x86_64-wrs-vxworks"
# with fish
sudo python -m pyftpdlib -p 21 -u target -P vxTarget -i 127.0.0.1 -d (pwd)/target/x86_64-wrs-vxworks
```

Now, `VxWorks` can be started in a second terminal.

```bash
qemu-system-x86_64 \
  -m 2048M \
  -kernel /opt/vxworks/wrsdk-vxworks7-qemu-25-03/vxsdk/bsps/itl_generic_3_0_0_4/vxWorks \
  -net nic \
  -net user,hostfwd=tcp::1534-:1534,hostfwd=tcp::2345-:2345 \
  -display none \
  -serial stdio \
  -monitor none \
  -append "bootline:fs(0,0)host:vxWorks h=10.0.2.2 e=10.0.2.15 u=target pw=vxTarget o=gei0"
```

Once `VxWorks` has booted, one needs to enter the `cmd` shell by entering `cmd`.
The prompt the changes to `[vxWorks *]#` and one can navigate into the folder
with the respective artifacts, e.g. `debug` or `release`.

More information can be found at
`cat /opt/vxworks/wrsdk-vxworks7-qemu-25-03/README.md`.

## Build iceoryx2

When starting with a new bash shell, the `sdkenv.sh` file needs to be sourced
first.

```bash
source /opt/vxworks/wrsdk-vxworks7-qemu-25-03/sdkenv.sh
```

### Build the Rust Crates

> [!NOTE]
> Currently, iceoryx2 on VxWorks is only available as developer setup and some
parts are not yet fully functional.

Not all workspace crates are working on VxWorks and we need to build the crates
separately.

```bash
cargo build --package iceoryx2
```

After the build, the artefacts are available in `qemu`. In the terminal with
the running `Works`, just do a `ls debug` to display the artifacts.

In the next step we build the basic publish-subscibe example.

```bash
cargo build --example publish_subscribe_publisher
cargo build --example publish_subscribe_subscriber
```

Once build, we can run them in the `qemu` environment. Since the main task on
`VxWorks` has only 65kB for the stack and iceoryx2 creates larger objects on
the stack, the stack size needs to be increased when the applications are run.
This can be done with the `rtp` command. See also `help rtp` for more
information.

We start the `subscriber` in the background and the the `publisher`.

```bash
rtp exec -u 2097152 debug/examples/publish_subscribe_subscriber.vxe&
rtp exec -u 2097152 debug/examples/publish_subscribe_publisher.vxe
```

> [!NOTE]
> Since the artifacts run from the FTP server, the start of the applications
> quite slow. Copying them directly to qemu speeds the start up.

## Build the C and C++ Bindings

### Build and install iceoryx_hoofs

The C++ bindings are using some components from `iceoryx_hoofs`, which needs to
be build and installed separately. The path to the install directory can be
specified with `-DCMAKE_PREFIX_PATH`.

`iceoryx_hoofs` can be build with this steps:

```bash
git clone --depth 1 --branch v2.95.5 https://github.com/eclipse-iceoryx/iceoryx.git target/iceoryx/src

cmake -S target/iceoryx/src/iceoryx_platform \
  -B target/iceoryx/build/platform \
  -DCMAKE_BUILD_TYPE=Release \
  -DCMAKE_INSTALL_PREFIX=target/install \
  -DIOX_PLATFORM_MINIMAL_POSIX=ON \
  -DCMAKE_SYSTEM_NAME=VxWorks
cmake --build target/iceoryx/build/platform
cmake --install target/iceoryx/build/platform

cmake -S target/iceoryx/src/iceoryx_hoofs \
  -B target/iceoryx/build/hoofs \
  -DCMAKE_BUILD_TYPE=Release \
  -DCMAKE_INSTALL_PREFIX=target/install \
  -DCMAKE_PREFIX_PATH="$(pwd)/target/install" \
  -DIOX_USE_HOOFS_SUBSET_ONLY=ON \
  -DCMAKE_SYSTEM_NAME=VxWorks
cmake --build target/iceoryx/build/hoofs
cmake --install target/iceoryx/build/hoofs
```

### Build the iceoryx2 C++ bindings

```bash
cargo build --release --package iceoryx2-ffi

cmake -S . \
  -B target/ffi/build \
  -DCMAKE_INSTALL_PREFIX=target/install \
  -DCMAKE_PREFIX_PATH="$(pwd)/target/install" \
  -DRUST_BUILD_ARTIFACT_PATH="$(pwd)/target/x86_64-wrs-vxworks/release"
cmake --build target/ffi/build
cmake --install target/ffi/build
```

### Build selected examples

```bash
cmake -S examples/cxx/publish_subscribe \
  -B target/ffi/out-of-tree \
  -DCMAKE_INSTALL_PREFIX=target/install \
  -DCMAKE_PREFIX_PATH="$(pwd)/target/install" \
  -DCMAKE_CXX_STANDARD=17 \
  -DCMAKE_SYSTEM_NAME=VxWorks
cmake --build target/ffi/out-of-tree
```

<!-- markdownlint-disable MD025 Multiple top-level headings -->
# Limitations

For limitations, please read `iceoryx2-pal/posix/src/vxworks/README.md`
