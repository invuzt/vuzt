package com.vuzt;

import android.app.Activity;
import android.os.Bundle;
import android.webkit.WebSettings;
import android.webkit.WebView;
import android.webkit.WebViewClient;

public class MainActivity extends Activity {

    @Override
    protected void onCreate(Bundle savedInstanceState) {
        super.onCreate(savedInstanceState);
        
        // Membuat WebView secara programmatik agar memenuhi layar
        WebView myWebView = new WebView(this);
        setContentView(myWebView);

        WebSettings webSettings = myWebView.getSettings();
        
        // PENTING: Aktifkan JavaScript untuk menjalankan logika kalkulator
        webSettings.setJavaScriptEnabled(true);
        
        // PENTING: Aktifkan DOM Storage agar Service Worker (PWA) bisa jalan
        webSettings.setDomStorageEnabled(true);
        
        // Mencegah browser eksternal terbuka saat ada link diklik
        myWebView.setWebViewClient(new WebViewClient());

        // Memuat file HTML dari folder assets
        myWebView.loadUrl("file:///android_asset/index.html");
    }
}

