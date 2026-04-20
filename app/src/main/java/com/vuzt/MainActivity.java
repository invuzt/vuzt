package com.vuzt;

import android.app.Activity;
import android.os.Bundle;
import android.webkit.*;

public class MainActivity extends Activity {
    WebView myWebView;

    // Load library .so yang akan dibuat oleh GitHub Actions
    static {
        System.loadLibrary("pemanasan_jni");
    }

    // Deklarasi fungsi dari Rust
    public native String cekRamNative();

    @Override
    protected void onCreate(Bundle savedInstanceState) {
        super.onCreate(savedInstanceState);
        myWebView = new WebView(this);
        setContentView(myWebView);

        WebSettings ws = myWebView.getSettings();
        ws.setJavaScriptEnabled(true);
        myWebView.addJavascriptInterface(this, "Android");

        myWebView.loadUrl("file:///android_asset/index.html");
    }

    @JavascriptInterface
    public void jalankanBinary() {
        // Memanggil Rust langsung tanpa sub-process/chmod
        final String hasil = cekRamNative();
        
        runOnUiThread(new Runnable() {
            @Override
            public void run() {
                // Mengirim hasil ke tampilan HTML
                myWebView.evaluateJavascript("tampilkanData('" + hasil.replace("\n", "\\n") + "')", null);
            }
        });
    }
}
