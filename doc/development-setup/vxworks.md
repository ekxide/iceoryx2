# Setup The Development Environment and Build iceoryx2 on VxWorks

## Get and Setup VxWorks

For OSS, the [VxWorks Software Development Kit (SDK)](https://forums.windriver.com/t/vxworks-software-development-kit-sdk/43)
can be used via the non-commercial license agreement (NCLA).

> [!NOTE]
> It can only be used for non-commercial projects.

<!-- workaround for:  MD028/no-blanks-blockquote Blank line inside blockquote -->

> [!NOTE]
> The setup is done with VxWorks Version 25.09 and assumes the SDK to be
> extracted to `/opt/vxworks/wrsdk-vxworks7-qemu-25-09`.

```bash
cd /opt
sudo mkdir vxworks
sudo chown $USER:$USER vxworks
cd vxworks
wget https://d13321s3lxgewa.cloudfront.net/wrsdk-vxworks7-qemu-1.16.tar.bz2
tar -xjf wrsdk-vxworks7-qemu-1.16.tar.bz2
mv wrsdk-vxworks7-qemu wrsdk-vxworks7-qemu-25-09
```

To verify if the setup works, `sdkenv.sh` file needs to be sourced and the
`WIND_SDK_HOME` environment variable can be checked.

```bash
source /opt/vxworks/wrsdk-vxworks7-qemu-25-09/sdkenv.sh
echo $WIND_SDK_HOME
```

This should print `/opt/vxworks/wrsdk-vxworks7-qemu-25-09`.

An easy way to access the documentation are the man pages:

```bash
export MANPATH="/opt/vxworks/wrsdk-vxworks7-qemu-25-09/docs/vxworks-7/man:$MANPATH"
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
  -kernel /opt/vxworks/wrsdk-vxworks7-qemu-25-09/vxsdk/bsps/itl_generic_3_0_0_4/vxWorks \
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
`cat /opt/vxworks/wrsdk-vxworks7-qemu-25-09/README.md`.

## Build iceoryx2

When starting with a new bash shell, the `sdkenv.sh` file needs to be sourced
first.

```bash
source /opt/vxworks/wrsdk-vxworks7-qemu-25-09/sdkenv.sh
```

### Build the Rust Crates

> [!NOTE]
> Currently, iceoryx2 on VxWorks is only available as developer setup and some
parts are not yet fully functional.

Not all workspace crates are working on VxWorks and we need to build the crates
separately.

```bash
cargo build --release --package iceoryx2
```

After the build, the artefacts are available in `qemu`. In the terminal with
the running `Works`, just do a `ls release` to display the artifacts.

In the next step we build the basic publish-subscibe example.

```bash
cargo build --release --example publish_subscribe_publisher
cargo build --release --example publish_subscribe_subscriber
```

Once build, we can run them in the `qemu` environment. Since the main task on
`VxWorks` has only 65kB for the stack and iceoryx2 creates larger objects on
the stack, the stack size needs to be increased to at least 256kB when the
applications are run. This can be done with the `rtp` command. See also
`help rtp` for more information.

We start the `subscriber` in the background and the the `publisher`.

```bash
rtp exec -u 262144 release/examples/publish_subscribe_subscriber.vxe&
rtp exec -u 262144 release/examples/publish_subscribe_publisher.vxe
```

If the application terminates on startup with an error message like

```console
RTP 0xffff8000095ea000 has been deleted due to signal 11.
```

the stack size is still too small and needs to be increased.

> [!NOTE]
> Since the artifacts run from the FTP server, the start of the applications
> quite slow. Copying them directly to qemu speeds the start up.

## Build the C and C++ Bindings

### Build the iceoryx2 C and C++ bindings

```bash
cargo build --release --package iceoryx2-ffi-c

cmake -S . \
  -B target/x86_64-wrs-vxworks/ff/cc/build \
  -DRUST_BUILD_ARTIFACT_PATH="$(pwd)/target/x86_64-wrs-vxworks/release" \
  -DCMAKE_SYSTEM_NAME=VxWorks
cmake --build target/x86_64-wrs-vxworks/ff/cc/build
cmake --install target/x86_64-wrs-vxworks/ff/cc/build --prefix target/x86_64-wrs-vxworks/ff/cc/install
```

### Build and run selected examples

```bash
cmake -S examples/cxx/publish_subscribe \
  -B target/x86_64-wrs-vxworks/ff/cc/out-of-tree \
  -DCMAKE_PREFIX_PATH="$(pwd)/target/x86_64-wrs-vxworks/ff/cc/install" \
  -DCMAKE_CXX_STANDARD=17 \
  -DCMAKE_SYSTEM_NAME=VxWorks
cmake --build target/x86_64-wrs-vxworks/ff/cc/out-of-tree
```

The examples can now be run in a `qemu` environment with:

```bash
rtp exec -u 262144 ff/cc/out-of-tree/example_cxx_publish_subscribe_subscriber&
rtp exec -u 262144 ff/cc/out-of-tree/example_cxx_publish_subscribe_publisher
```

### Build for Rust no_std

In order to build the iceoryx2 C and C++ bindings without Rust std lib,
the `--no-default-features` parameter needs to be added to `cargo build`.
The example above then becomes

```bash
cargo build --release --package iceoryx2-ffi-c --no-default-features
```

<!-- markdownlint-disable MD025 Multiple top-level headings -->
# Limitations

For limitations, please read `iceoryx2-pal/posix/src/vxworks/README.md`
