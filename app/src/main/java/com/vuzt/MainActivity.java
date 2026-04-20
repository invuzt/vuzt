package com.vuzt;
import android.app.Activity;
import android.os.Bundle;
import android.webkit.*;
import java.io.File;

public class MainActivity extends Activity {
    WebView myWebView;
    String filePath;

    static { System.loadLibrary("pemanasan_jni"); }

    public native String getSystemInfoNative();
    public native void saveNoteNative(String path, String content);
    public native String readNoteNative(String path);

    @Override
    protected void onCreate(Bundle savedInstanceState) {
        super.onCreate(savedInstanceState);
        filePath = new File(getFilesDir(), "note.txt").getAbsolutePath();
        myWebView = new WebView(this);
        setContentView(myWebView);
        myWebView.getSettings().setJavaScriptEnabled(true);
        myWebView.addJavascriptInterface(this, "Android");
        myWebView.loadUrl("file:///android_asset/index.html");
    }

    @JavascriptInterface public String getSystemInfoNative() { return getSystemInfoNative(); }
    @JavascriptInterface public void saveNote(String t) { saveNoteNative(filePath, t); }
    @JavascriptInterface public String readNote() { return readNoteNative(filePath); }
}
