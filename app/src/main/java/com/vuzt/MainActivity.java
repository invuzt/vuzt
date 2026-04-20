package com.vuzt;

import android.app.Activity;
import android.os.Bundle;
import android.webkit.*;

public class MainActivity extends Activity {
    WebView myWebView;

    static {
        System.loadLibrary("pemanasan_jni");
    }

    // Fungsi native baru yang mencakup semua info
    public native String getSystemInfoNative();

    @Override
    protected void onCreate(Bundle savedInstanceState) {
        super.onCreate(savedInstanceState);
        myWebView = new WebView(this);
        setContentView(myWebView);
        myWebView.getSettings().setJavaScriptEnabled(true);
        myWebView.addJavascriptInterface(this, "Android");
        myWebView.loadUrl("file:///android_asset/index.html");
    }

    @JavascriptInterface
    public void jalankanBinary() {
        final String hasil = getSystemInfoNative();
        runOnUiThread(() -> {
            myWebView.evaluateJavascript("tampilkanData('" + hasil.replace("\n", "\\n") + "')", null);
        });
    }
}
