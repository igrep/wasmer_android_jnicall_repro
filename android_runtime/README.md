1. Install Android SDK: https://medium.com/better-programming/install-android-studio-in-ubuntu-b8aed675849f
1. Install Android NDK: https://developer.android.com/studio/projects/install-ndk we use 21.0.6113669

Then do the following steps:
```bash
rustup install nightly
rustup target add --toolchain nightly aarch64-linux-android x86_64-linux-android

ln -s ~/Android/Sdk/ndk/21.0.6113669/toolchains/llvm/prebuilt/linux-x86_64/bin/aarch64-linux-android21-clang ~/Android/Sdk/ndk/21.0.6113669/toolchains/llvm/prebuilt/linux-x86_64/bin/aarch64-linux-android-clang
ln -s ~/Android/Sdk/ndk/21.0.6113669/toolchains/llvm/prebuilt/linux-x86_64/bin/x86_64-linux-android21-clang ~/Android/Sdk/ndk/21.0.6113669/toolchains/llvm/prebuilt/linux-x86_64/bin/x86_64-linux-android-clang

ln -s ~/Android/Sdk/ndk/21.0.6113669/toolchains/llvm/prebuilt/linux-x86_64/bin/aarch64-linux-android21-clang++ ~/Android/Sdk/ndk/21.0.6113669/toolchains/llvm/prebuilt/linux-x86_64/bin/aarch64-linux-android-clang++
ln -s ~/Android/Sdk/ndk/21.0.6113669/toolchains/llvm/prebuilt/linux-x86_64/bin/x86_64-linux-android21-clang++ ~/Android/Sdk/ndk/21.0.6113669/toolchains/llvm/prebuilt/linux-x86_64/bin/x86_64-linux-android-clang++

export PATH=~/Android/Sdk/ndk/21.0.6113669/toolchains/llvm/prebuilt/linux-x86_64/bin:$PATH

cargo +nightly build --target aarch64-linux-android --release --features singlepass
cargo +nightly build --target x86_64-linux-android --release --features singlepass
```
