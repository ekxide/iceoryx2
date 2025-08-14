# QNX Development Environment

> [!NOTE]
> The versions of Ubuntu supported by the QNX development platform can be found [here](
> https://support.qnx.com/developers/docs/relnotes8.0/com.qnx.doc.release_notes/topic/sdp8_rn.html)
>
> These instructions have been tested on Ubuntu 20.04.

## Build Instructions

### Access the QNX toolchain

> [!NOTE]
> The QNX [non-commercial license](https://blackberry.qnx.com/en/products/qnx-everywhere/licensing)
> is available for open-source software development with QNX 8.0.
>
> A license is required for QNX 7.1.

Follow [these instructions](https://www.qnx.com/developers/docs/8.0/com.qnx.doc.qnxsdp.quickstart/topic/install_host.html)
to install the "QNX Software Center" for "Linux Hosts". Then, add your license and install the
"QNX Software Development Platform" via the "QNX Software Center".

Following successful installation, a directory named either `qnx800` or `qnx710` containing the QNX
toolchain should be available in your `$HOME` directory.

```bash
$ tree -L 2 ~/qnx800
qnx800
├── host
│   ├── common
│   └── linux
├── qnxsdp-env.bat
├── qnxsdp-env.sh
└── target
    └── qnx
```

### Create a QNX image for QEMU

The [`mkqnximage`](https://www.qnx.com/developers/docs/8.0/com.qnx.doc.neutrino.utilities/topic/m/mkqnximage.html)
CLI can be used to create a QNX image for development. See the `--help` for available
configuration options:

```bash
mkqnximage --help
```

The following image configuration was used for development of the platform abstraction for QNX:

#### x86_64

##### QNX 7.1

```bash
export QNX_TOOLCHAIN="$HOME/qnx710"
source $QNX_TOOLCHAIN/qnxsdp-env.sh

export VM_HOSTNAME="x86_64-qnx-vm"
export VM_IPV4_ADDR="172.31.1.11"

export IMAGE_DIR="$HOME/images/minimal"
mkdir -p $IMAGE_DIR
cd $IMAGE_DIR

mkqnximage \
    --noprompt \
    --hostname="$VM_HOSTNAME" \
    --type=qemu \
    --arch=x86_64 \
    --ip="$VM_IPV4_ADDR" \
    --sys-size=256 \
    --sys-inodes=24000 \
    --data-size=256 \
    --data-inodes=24000
```

##### QNX 8.0

```bash
export QNX_TOOLCHAIN="$HOME/qnx800"
source $QNX_TOOLCHAIN/qnxsdp-env.sh

export VM_HOSTNAME="x86_64-qnx-vm"
export VM_IPV4_ADDR="172.31.1.11"

export IMAGE_DIR="$HOME/images/minimal"
mkdir -p $IMAGE_DIR
cd $IMAGE_DIR

mkqnximage \
    --noprompt \
    --hostname="$VM_HOSTNAME" \
    --type=qemu \
    --arch=x86_64 \
    --ip="$VM_IPV4_ADDR" \
    --sys-size=256 \
    --sys-inodes=24000 \
    --data-size=256 \
    --data-inodes=24000
```

### Run a QNX image on QEMU

Images build with `mkqnximage` can be run using the `--run` option.

#### x86_64

##### QNX 7.1

```bash
export QNX_TOOLCHAIN="$HOME/qnx710"
source $QNX_TOOLCHAIN/qnxsdp-env.sh

export IMAGE_DIR="$HOME/images/minimal"
cd $IMAGE_DIR

mkqnximage --run
```

Alternatively, use QEMU directly for more fine-grained control over the emulation:

```bash
export QNX_TOOLCHAIN="$HOME/qnx710"
source $QNX_TOOLCHAIN/qnxsdp-env.sh

export IMAGE_DIR="$HOME/images/minimal"
export SHARED_DIR="${IMAGE_DIR}/shared"
mkdir -p $SHARED_DIR

export MAC=$(printf "52:54:00:%02x:%02x:%02x" $(( $RANDOM & 0xff)) $(( $RANDOM & 0xff )) $(( $RANDOM & 0xff)))

sudo ${QNX_TOOLCHAIN}/host/common/mkqnximage/qemu/net.sh /usr/lib/qemu/qemu-bridge-helper /etc/qemu/bridge.conf

qemu-system-x86_64 \
  -smp 2 \
  -m 1G \
  -drive file=${IMAGE_DIR}/output/disk-qemu.vmdk,if=ide,id=drv0 \
  -hdb fat:rw:${IMAGE_DIR}/shared \
  -netdev bridge,br=br0,id=net0 \
  -device e1000,netdev=net0,mac=$MAC \
  -nographic \
  -kernel ${IMAGE_DIR}/output/ifs.bin \
  -serial mon:stdio \
  -object rng-random,filename=/dev/urandom,id=rng0 \
  -device virtio-rng-pci,rng=rng0
```

##### QNX 8.0

```bash
export QNX_TOOLCHAIN="$HOME/qnx800"
source $QNX_TOOLCHAIN/qnxsdp-env.sh

export IMAGE_DIR="$HOME/images/minimal"
cd $IMAGE_DIR

mkqnximage --run
```

Alternatively, use QEMU directly for more fine-grained control over the emulation:

```bash
export QNX_TOOLCHAIN="$HOME/qnx800"
source $QNX_TOOLCHAIN/qnxsdp-env.sh

export IMAGE_DIR="$HOME/images/minimal"
export SHARED_DIR="${IMAGE_DIR}/shared"
mkdir -p $SHARED_DIR

export MAC=$(printf "52:54:00:%02x:%02x:%02x" $(( $RANDOM & 0xff)) $(( $RANDOM & 0xff )) $(( $RANDOM & 0xff)))

sudo ${QNX_TOOLCHAIN}/host/common/mkqnximage/qemu/net.sh /usr/lib/qemu/qemu-bridge-helper /etc/qemu/bridge.conf bridge

qemu-system-x86_64 \
  -smp 2 \
  -m 1G \
  -drive file=${IMAGE_DIR}/output/disk-qemu.vmdk,if=ide,id=drv0 \
  -hdb fat:rw:${IMAGE_DIR}/shared \
  -netdev bridge,br=br0,id=net0 \
  -device e1000,netdev=net0,mac=$MAC \
  -nographic \
  -kernel ${IMAGE_DIR}/output/ifs.bin \
  -serial mon:stdio \
  -object rng-random,filename=/dev/urandom,id=rng0 \
  -device virtio-rng-pci,rng=rng0
```

### Build the Rust compiler for QNX

In order to build Rust applications for QNX targets, a custom-built Rust compiler is required
due to the dependence on the QNX toolchain.

The QNX targets supported by the Rust compiler can be found in [the `rustc` book](
https://doc.rust-lang.org/rustc/platform-support/nto-qnx.html).

#### QNX 7.1:

```bash
export QNX_TOOLCHAIN="$HOME/qnx710"
source ${QNX_TOOLCHAIN}/qnxsdp-env.sh

# Clone Rust source
export RUSTDIR=~/source/rust
git clone https://github.com/rust-lang/rust.git -b 1.88.0 --depth 1 $RUSTDIR

# Configure the build
echo -e "[build]\nextended = true" > $RUSTDIR/config.toml

# Build the compiler (x86_64 and aarch64 targets)
cd $RUSTDIR

export build_env='
    CC_x86_64_pc_nto_qnx710=qcc
    CFLAGS_x86_64_pc_nto_qnx710=-Vgcc_ntox86_64_cxx
    CXX_x86_64_pc_nto_qnx710=qcc
    AR_x86_64_pc_nto_qnx710=ntox86_64-ar
    CC_aarch64_unknown_nto_qnx710=qcc
    CFLAGS_aarch64_unknown_nto_qnx710=-Vgcc_ntoaarch64le_cxx
    CXX_aarch64_unknown_nto_qnx710=qcc
    AR_aarch64_unknown_nto_qnx710=ntoaarch64-ar
    '
./x.py build --target aarch64-unknown-nto-qnx710,x86_64-pc-nto-qnx710,x86_64-unknown-linux-gnu rustc library/core library/alloc library/std library tools/rustfmt

# Create a symlink for easier use
rustup toolchain link qnx-custom $RUSTDIR/build/host/stage1
```

#### QNX 8.0:

```bash
export QNX_TOOLCHAIN="$HOME/qnx800"
source ${QNX_TOOLCHAIN}/qnxsdp-env.sh

# Clone Rust source
export RUSTDIR=~/source/rust
git clone https://github.com/rust-lang/rust.git -b 1.88.0 --depth 1 $RUSTDIR

# Configure the build
echo -e "[build]\nextended = true" > $RUSTDIR/config.toml

# Build the compiler (x86_64 and aarch64 targets)
cd $RUSTDIR

export build_env='
    CC_x86_64_pc_nto_qnx80=qcc
    CFLAGS_x86_64_pc_nto_qnx800=-Vgcc_ntox86_64_cxx
    CXX_x86_64_pc_nto_qnx800=qcc
    AR_x86_64_pc_nto_qnx800=ntox86_64-ar
    CC_aarch64_unknown_nto_qnx800=qcc
    CFLAGS_aarch64_unknown_nto_qnx800=-Vgcc_ntoaarch64le_cxx
    CXX_aarch64_unknown_nto_qnx800=qcc
    AR_aarch64_unknown_nto_qnx800=ntoaarch64-ar
    '

./x.py build --target aarch64-unknown-nto-qnx800,x86_64-pc-nto-qnx800,x86_64-unknown-linux-gnu rustc library/core library/alloc library/std library tools/rustfmt

# Create a symlink for easier use
rustup toolchain link qnx-custom $RUSTDIR/build/host/stage1
```

### Build `iceoryx2` for QNX

> [!WARNING]
> A `nostd` flavor of `iceoryx2` is required for use with QNX 8.0.

Use the custom-built compiler to build for QNX targets:

#### x86_64

##### QNX 7.1

```bash
export QNX_TOOLCHAIN="$HOME/qnx710"
source $QNX_TOOLCHAIN/qnxsdp-env.sh

cargo +qnx-custom build --target x86_64-pc-nto-qnx710 --package iceoryx2
```

##### QNX 8.0

```bash
export QNX_TOOLCHAIN=$HOME/qnx800
source $QNX_TOOLCHAIN/qnxsdp-env.sh

cargo +qnx-custom build --target x86_64-pc-nto-qnx800 --package iceoryx2
```

#### Aarch64

##### QNX 7.1

```bash
export QNX_TOOLCHAIN="$HOME/qnx710"
source $QNX_TOOLCHAIN/qnxsdp-env.sh

cargo +qnx-custom build --target aarch64-unknown-nto-qnx710 --package iceoryx2
```

##### QNX 8.0

```bash
export QNX_TOOLCHAIN=$HOME/qnx800
source $QNX_TOOLCHAIN/qnxsdp-env.sh

cargo +qnx8-custom build --target aarch64-unknown-nto-qnx800 --package iceoryx2
```

### Remote Debugging with GDB

The GNU debugger `gdb` can be used to transfer binaries to QNX running on QEMU.

First, start the [remote debug agent](https://www.qnx.com/developers/docs/8.0/com.qnx.doc.neutrino.user_guide/topic/security_pdebug.html?hl=pdebug)
in the QNX VM:

```sh
pdebug 1234
```

Then on the development host, connect to the target via `gdb`:

#### x86_64

##### QNX 7.1

```bash
export QNX_TOOLCHAIN="$HOME/qnx710"
source ${QNX_TOOLCHAIN}/qnxsdp-env.sh

ntox86_64-gdb
file path/to/binary
target qnx 172.31.1.11:1234 # If using same image as above
upload path/to/binary data/home/root/binary
run
```

##### QNX 8.0

```bash
export QNX_TOOLCHAIN=$HOME/qnx800
source ${QNX_TOOLCHAIN}/qnxsdp-env.sh

ntox86_64-gdb
file path/to/binary
target qnx 172.31.1.11:1234 # If using same image as above
upload path/to/binary data/home/root/binary
run
```

### Running Benchmarks

#### x86_64

##### QNX 7.1

First build the benchmarks:

```bash
export QNX_TOOLCHAIN="$HOME/qnx710"
source $QNX_TOOLCHAIN/qnxsdp-env.sh

cargo +qnx-custom build --release --target x86_64-pc-nto-qnx710 --package benchmark-publish-subscribe --package benchmark-event --package benchmark-request-response --package benchmark-queue
```

Then transfer the binaries to the target e.g. via `gdb` by first starting `pdebug`:

```sh
pdebug 1234
```

Then uploading the binaries:

```sh
ntox86_64-gdb
target qnx 172.31.1.11:1234 # If using same image as above
upload target/x86_64-pc-nto-qnx710/release/benchmark-publish-subscribe /data/home/root/benchmark-publish-subscribe
upload target/x86_64-pc-nto-qnx710/release/benchmark-event /data/home/root/benchmark-event
upload target/x86_64-pc-nto-qnx710/release/benchmark-request-response /data/home/root/benchmark-request-response
upload target/x86_64-pc-nto-qnx710/release/benchmark-queue /data/home/root/benchmark-queue
```

The benchmarks can then be executed from the target:

```sh
cd /data/home/root

./benchmark-publish-subscribe --bench-all --iterations 1000000
./benchmark-event --bench-all --iterations 1000
./benchmark-request-response --iterations 1000000
./benchmark-queue --iterations 1000000
```
