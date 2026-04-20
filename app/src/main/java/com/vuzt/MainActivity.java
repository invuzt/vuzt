package com.vuzt;

import android.app.Activity;
import android.os.Bundle;
import android.webkit.*;
import java.io.*;

public class MainActivity extends Activity {
    WebView myWebView;

    @Override
    protected void onCreate(Bundle savedInstanceState) {
        super.onCreate(savedInstanceState);
        myWebView = new WebView(this);
        setContentView(myWebView);

        WebSettings ws = myWebView.getSettings();
        ws.setJavaScriptEnabled(true);
        
        // Interface agar JavaScript bisa memanggil Java
        myWebView.addJavascriptInterface(this, "Android");

        myWebView.loadUrl("file:///android_asset/index.html");
    }

    @JavascriptInterface
    public void jalankanBinary() {
        try {
            // 1. Lokasi internal storage yang aman untuk eksekusi
            File binFile = new File(getFilesDir(), "pemanasan");

            // 2. Copy dari Assets ke Internal (hanya jika belum ada atau versi baru)
            InputStream in = getAssets().open("bin/pemanasan");
            OutputStream out = new FileOutputStream(binFile);
            byte[] buffer = new byte[1024];
            int read;
            while ((read = in.read(buffer)) != -1) { out.write(buffer, 0, read); }
            in.close(); out.close();

            // 3. CHMOD +X (Memberikan izin eksekusi secara native)
            binFile.setExecutable(true);

            // 4. JALANKAN BINARY & BACA OUTPUT
            Process process = Runtime.getRuntime().exec(binFile.getAbsolutePath());
            BufferedReader reader = new BufferedReader(new InputStreamReader(process.getInputStream()));
            StringBuilder output = new StringBuilder();
            String line;
            while ((line = reader.readLine()) != null) {
                output.append(line).append("\n");
            }

            // 5. KIRIM HASIL KE WEBVIEW
            final String hasil = output.toString();
            runOnUiThread(() -> myWebView.evaluateJavascript("tampilkanData('" + hasil.replace("\n", "\\n") + "')", null));

        } catch (Exception e) {
            final String err = e.getMessage();
            runOnUiThread(() -> myWebView.evaluateJavascript("tampilkanData('Error: " + err + "')", null));
        }
    }
}
