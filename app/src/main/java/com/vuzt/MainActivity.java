package com.vuzt;

import android.app.Activity;
import android.graphics.Color;
import android.os.Bundle;
import android.view.View;
import android.view.Window;
import android.view.WindowManager;
import android.webkit.*;
import java.io.File;

public class MainActivity extends Activity {
    WebView myWebView;
    String fileDir;

    static { System.loadLibrary("pemanasan_jni"); }

    // JNI Native Methods
    public native String getSystemInfoNative();
    public native void saveNoteNative(String path, String content);
    public native String readNoteNative(String path);
    public native void deleteNoteNative(String path);
    public native String listNotesNative(String dir);

    @Override
    protected void onCreate(Bundle savedInstanceState) {
        super.onCreate(savedInstanceState);
        
        // Buat Window Fullscreen, Transparan, dan Tanpa Title
        requestWindowFeature(Window.FEATURE_NO_TITLE);
        getWindow().setFlags(WindowManager.LayoutParams.FLAG_LAYOUT_NO_LIMITS, 
                             WindowManager.LayoutParams.FLAG_LAYOUT_NO_LIMITS);
        getWindow().setStatusBarColor(Color.TRANSPARENT);
        getWindow().setNavigationBarColor(Color.TRANSPARENT);

        fileDir = getFilesDir().getAbsolutePath();

        myWebView = new WebView(this);
        setContentView(myWebView);

        // Pengaturan WebView Mencegah Zoom & Native Feel
        WebSettings settings = myWebView.getSettings();
        settings.setJavaScriptEnabled(true);
        settings.setDomStorageEnabled(true);
        settings.setDatabaseEnabled(true);
        
        // KUNCI ZOOM: Mati total
        settings.setSupportZoom(false);
        settings.setBuiltInZoomControls(false);
        settings.setDisplayZoomControls(false);
        
        // Native feel settings
        settings.setLoadWithOverviewMode(true);
        settings.setUseWideViewPort(true);
        myWebView.setOverScrollMode(View.OVER_SCROLL_NEVER); // Matikan efek scroll membal web
        myWebView.setBackgroundColor(Color.TRANSPARENT); // WebView transparan

        myWebView.addJavascriptInterface(this, "Android");
        myWebView.loadUrl("file:///android_asset/index.html");
    }

    // Javascript Interfaces (Beda nama dengan Native agar tidak bentrok)
    @JavascriptInterface public String getRamData() { return getSystemInfoNative(); }
    
    @JavascriptInterface 
    public void saveNote(String filename, String content) { 
        saveNoteNative(new File(fileDir, filename).getAbsolutePath(), content); 
    }
    
    @JavascriptInterface 
    public String readNote(String filename) { 
        return readNoteNative(new File(fileDir, filename).getAbsolutePath()); 
    }

    @JavascriptInterface 
    public void deleteNote(String filename) { 
        deleteNoteNative(new File(fileDir, filename).getAbsolutePath()); 
    }

    @JavascriptInterface 
    public String getAllNotes() { 
        return listNotesNative(fileDir); 
    }
}
