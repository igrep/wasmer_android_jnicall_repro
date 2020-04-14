package com.wasmer.android;

import androidx.annotation.Keep;
import androidx.appcompat.app.AppCompatActivity;

import android.content.res.AssetManager;
import android.os.Bundle;

import java.io.ByteArrayOutputStream;
import java.io.InputStream;

public class MainActivity extends AppCompatActivity {
    private static native void JNIExecuteWasm(MainActivity self, byte[] module_bytes) throws Exception;

    @Keep
    public void Test() {
        System.out.println("Test was called from WASM!");
    }

    @Override
    protected void onCreate(Bundle savedInstanceState) {
        try {
            // Wait for debugger to attach so
            // we can see all output
            Thread.sleep(2000);

            // Load runtime code
            System.loadLibrary("wasmer_android");

            // default code of the empty activity example
            super.onCreate(savedInstanceState);
            setContentView(R.layout.activity_main);

            // Read file into byte array
            AssetManager am = this.getAssets();
            InputStream inputStream = am.open("main.wasm");
            ByteArrayOutputStream baos = new ByteArrayOutputStream();
            int reads = inputStream.read();
            while(reads != -1) {
                baos.write(reads);
                reads = inputStream.read();
            }
            byte[] module_bytes = baos.toByteArray();

            // Run the file
            System.out.println("Calling JNIExecuteWasm!");
            JNIExecuteWasm(this, module_bytes);
            System.out.println("Finished calling JNIExecuteWasm!");
        } catch (Exception e) {
            e.printStackTrace();
        }
        System.out.println("Finished program!");
    }
}
