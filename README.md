- Go to `test_module` folder and build it according to instructions in `README.md`.
- Go to `android_runtime` folder and build it according to instructions in `README.md`.
- The `android` folder contains an android project that uses the built files through symlinks. Run it with for example by connecting your android ARM phone to your computer with developer usb debugging mode on, and afterwards open the `android` folder in android studio and press the play button at the top of android studio once your phone shows up.
- Any log should appear for example in the `Run` tab of android studio at the bottom.

Running on Pixel2 API 29 x86_64 emulator produces the following where last 3 lines show a successfull call:
```

04/14 17:15:52: Launching 'app' on Pixel 2 API 29.
$ adb shell am start -n "com.wasmer.android/com.wasmer.android.MainActivity" -a android.intent.action.MAIN -c android.intent.category.LAUNCHER
Connected to process 14168 on device 'Pixel_2_API_29 [emulator-5554]'.
Capturing and displaying logcat messages from application. This behavior can be disabled in the "Logcat output" section of the "Debugger" settings page.
D/libEGL: Emulator has host GPU support, qemu.gles is set to 1.
W/libc: Unable to set property "qemu.gles" to "1": connection failed; errno=13 (Permission denied)
D/libEGL: loaded /vendor/lib64/egl/libEGL_emulation.so
D/libEGL: loaded /vendor/lib64/egl/libGLESv1_CM_emulation.so
D/libEGL: loaded /vendor/lib64/egl/libGLESv2_emulation.so
W/RenderThread: type=1400 audit(0.0:227): avc: denied { write } for name="property_service" dev="tmpfs" ino=1667 scontext=u:r:untrusted_app:s0:c136,c256,c512,c768 tcontext=u:object_r:property_socket:s0 tclass=sock_file permissive=0
W/.wasmer.androi: Accessing hidden method Landroid/view/View;->computeFitSystemWindows(Landroid/graphics/Rect;Landroid/graphics/Rect;)Z (greylist, reflection, allowed)
W/.wasmer.androi: Accessing hidden method Landroid/view/ViewGroup;->makeOptionalFitsSystemWindows()V (greylist, reflection, allowed)
I/System.out: Calling JNIExecuteWasm!
I/System.out: Test was called from WASM!
I/chatty: uid=10136(com.wasmer.android) identical 1 line
I/System.out: Test was called from WASM!
    Finished calling JNIExecuteWasm!
    Finished program!
D/HostConnection: HostConnection::get() New Host Connection established 0x719255ae7fe0, tid 14199
D/HostConnection: HostComposition ext ANDROID_EMU_CHECKSUM_HELPER_v1 ANDROID_EMU_dma_v1 ANDROID_EMU_direct_mem ANDROID_EMU_host_composition_v1 ANDROID_EMU_host_composition_v2 ANDROID_EMU_YUV420_888_to_NV21 ANDROID_EMU_YUV_Cache ANDROID_EMU_async_unmap_buffer GL_OES_vertex_array_object GL_KHR_texture_compression_astc_ldr ANDROID_EMU_gles_max_version_2 
W/OpenGLRenderer: Failed to choose config with EGL_SWAP_BEHAVIOR_PRESERVED, retrying without...
D/eglCodecCommon: setVertexArrayObject: set vao to 0 (0) 0 0
D/EGL_emulation: eglCreateContext: 0x719255ae81c0: maj 2 min 0 rcv 2
D/EGL_emulation: eglMakeCurrent: 0x719255ae81c0: ver 2 0 (tinfo 0x7192e6fc2b20)
W/Gralloc3: mapper 3.x is not supported
D/HostConnection: createUnique: call
    HostConnection::get() New Host Connection established 0x719255ae8300, tid 14199
D/HostConnection: HostComposition ext ANDROID_EMU_CHECKSUM_HELPER_v1 ANDROID_EMU_dma_v1 ANDROID_EMU_direct_mem ANDROID_EMU_host_composition_v1 ANDROID_EMU_host_composition_v2 ANDROID_EMU_YUV420_888_to_NV21 ANDROID_EMU_YUV_Cache ANDROID_EMU_async_unmap_buffer GL_OES_vertex_array_object GL_KHR_texture_compression_astc_ldr ANDROID_EMU_gles_max_version_2 
D/eglCodecCommon: allocate: Ask for block of size 0x1000
    allocate: ioctl allocate returned offset 0x3ff803000 size 0x2000
D/EGL_emulation: eglMakeCurrent: 0x719255ae81c0: ver 2 0 (tinfo 0x7192e6fc2b20)
D/eglCodecCommon: setVertexArrayObject: set vao to 0 (0) 1 0

```


Running on Huawei P30 actual phone produces the following where last 3 lines show a failure of a JNI call:
```

04/14 17:16:44: Launching 'app' on HUAWEI ELE-L29.
$ adb shell am start -n "com.wasmer.android/com.wasmer.android.MainActivity" -a android.intent.action.MAIN -c android.intent.category.LAUNCHER
Connected to process 20190 on device 'huawei-ele_l29-XPH0219A31006737'.
Capturing and displaying logcat messages from application. This behavior can be disabled in the "Logcat output" section of the "Debugger" settings page.
I/.wasmer.androi: Late-enabling -Xcheck:jni
E/.wasmer.androi: Unknown bits set in runtime_flags: 0x8000
I/.wasmer.androi: Reinit property: dalvik.vm.checkjni= false
D/ActivityThread: Attach thread to application
I/.wasmer.androi: QarthPatchMonintor::Init
    QarthPatchMonintor::StartWatch
I/.wasmer.androi: QarthPatchMonintor::WatchPackage: /data/hotpatch/fwkhotpatch/
    QarthPatchMonintor::CheckAndWatchPatch: /data/hotpatch/fwkhotpatch/com.wasmer.android
    QarthPatchMonintor::CheckAndWatchPatch: /data/hotpatch/fwkhotpatch/all
    QarthPatchMonintor::Run
I/.wasmer.androi: QarthPatchMonintor::Reading
    QarthPatchMonintor::CheckNotifyEvent
    QarthPatchMonintor::CheckNotifyEvent before read
I/HwApiCacheMangerEx: apicache path=/storage/emulated/0 state=mounted key=com.wasmer.android#10233#256
I/HwApiCacheMangerEx: apicache path=/storage/emulated/0 state=mounted key=com.wasmer.android#10233#0
I/AwareBitmapCacher: init processName:com.wasmer.android pid=20190 uid=10233
E/AwareLog: AtomicFileUtils: readFileLines file not exist: android.util.AtomicFile@baf548f
E/AwareLog: AtomicFileUtils: readFileLines file not exist: android.util.AtomicFile@b04c31c
V/ActivityThread: callActivityOnCreate
V/HwWidgetFactory: : successes to get AllImpl object and return....
I/OverScrollerOptimization: start init SmartSlideOverScroller and get the overscroller config
    get the overscroller config
I/AwareAppScheduleManager: post cache drawable res id to aware, resId = 17302090, packagename = com.wasmer.android, cost time = 9693229
W/.wasmer.androi: Accessing hidden method Landroid/view/View;->computeFitSystemWindows(Landroid/graphics/Rect;Landroid/graphics/Rect;)Z (greylist, reflection, allowed)
W/.wasmer.androi: Accessing hidden method Landroid/view/ViewGroup;->makeOptionalFitsSystemWindows()V (greylist, reflection, allowed)
D/: [ZeroHung]zrhung_send_event: wp = 257, ret = 0
I/System.out: Calling JNIExecuteWasm!
I/System.out: Test was called from WASM!
A/libc: Fatal signal 6 (SIGABRT), code -1 (SI_QUEUE) in tid 20190 (.wasmer.android), pid 20190 (.wasmer.android)

```
