# Android Development Environment

> [!IMPORTANT]
> The MVP work only with the `local` and `local_threadsafe` service variant.
> The `ipc` and `ipc_threadsafe` service variants need more work to circumvent
> the Android sandbox limitations.

## Get and Setup Android

Install the Android Rust target:

```bash
rustup target add aarch64-linux-android x86_64-linux-android
```

This tool simplifies building Rust for Android but is not required:

```bash
cargo install cargo-ndk
```

The Android NDK is required in order to build Rust Android applications:

```bash
cd /opt
sudo mkdir android
sudo chown $USER:$USER android
cd android
wget https://dl.google.com/android/repository/android-ndk-r29-linux.zip
unizp android-ndk-r29-linux.zip     # unzips to 'android-ndk-r29'
```

```bash
export ANDROID_NDK_HOME=/opt/android/android-ndk-r29
```


Add this to `~/.cargo/config.toml`:
```toml
[target.x86_64-linux-android]
linker = "/opt/android/android-ndk-r29/toolchains/llvm/prebuilt/linux-x86_64/bin/x86_64-linux-android35-clang"
[target.aarch64-linux-android]
linker = "/opt/android/android-ndk-r29/toolchains/llvm/prebuilt/linux-x86_64/bin/aarch64-linux-android35-clang"
```

TODO: check if this can be made work
```toml
[target.aarch64-linux-android]
linker = "$ANDROID_NDK_HOME/toolchains/llvm/prebuilt/linux-x86_64/bin/aarch64-linux-android35-clang"
```

## ADB and Waydroid

Arch Linux
```bash
sudo pacman -S android-tools
```

Ubuntu Linux
```bash
sudo apt install android-tools-adb android-tools-fastboot
```

See: https://wiki.archlinux.org/title/Waydroid
```bash
pacman -S waydroid
```
Init Waydroid. This will download the Android image.
```bash
sudo waydroid init
```

Run the Waydroid session in a separate terminal:
```bash
waydroid prop set persist.waydroid.width 576
waydroid prop set persist.waydroid.height 1024
waydroid session start
# potentially this needs to be done in order to activate the props
systemctl restart waydroid-container.service
```

List the available Android application and run the calculator:
```bash
waydroid app list
waydroid app launch com.android.calculator2
```

## Build and run a Rust hello-world application on Android

Create a hello world application
```bash
cargo new --bin hello-world-android
cd hello-world-android
cargo build --target x86_64-linux-android --release
```

Copy the binary to Waydroid (if `adb` hangs, restart the `waydroid session start`):
```bash
adb push target/x86_64-linux-android/release/hello-world-android /data/local/tmp
adb shell chmod +x /data/local/tmp/hello-world-android # it seems this is not required
adb shell /data/local/tmp/hello-world-android
```

Alternatively, after `adb push ...` run `waydroid shell` in a terminal:
```bash
sudo waydroid shell
cd /data/local/tmp
./hello-world-android
```

## Build iceoryx2

```bash
cargo build --target x86_64-linux-android --package iceoryx2-pal-posix --features libc_platform
cargo build --target aarch64-linux-android --package iceoryx2-pal-posix --features libc_platform

cargo build --example publish_subscribe_publisher --target x86_64-linux-android --features iceoryx2/android_platform
cargo build --example publish_subscribe_publisher --target aarch64-linux-android --features iceoryx2/android_platform

cargo build --example event_based_comm_publisher --target x86_64-linux-android --features iceoryx2/android_platform
cargo build --example event_based_comm_publisher --target aarch64-linux-android --features iceoryx2/android_platform

cargo build --target x86_64-linux-android --package iceoryx2-tunnel-zenoh --features iceoryx2/android_platform
cargo build --target aarch64-linux-android --package iceoryx2-tunnel-zenoh --features iceoryx2/android_platform
```

```bash
adb push target/x86_64-linux-android/debug/examples/publish_subscribe_publisher /data/local/tmp
adb shell /data/local/tmp/publish_subscribe_publisher

adb push target/x86_64-linux-android/debug/examples/event_based_comm_publisher /data/local/tmp
adb shell /data/local/tmp/event_based_comm_publisher
```


```bash
cargo ndk -t arm64-v8a -t x86_64 build --package iceoryx2-pal-posix --features libc_platform
```

TODO: Build C++ Bindings
```bash
export CC=/opt/android/android-ndk-r29/toolchains/llvm/prebuilt/linux-x86_64/bin/x86_64-linux-android35-clang
export CXX=/opt/android/android-ndk-r29/toolchains/llvm/prebuilt/linux-x86_64/bin/x86_64-linux-android35-clang++
```
