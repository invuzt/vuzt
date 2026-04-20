#!/bin/bash
# Skrip Intip Proyek Vuzt - Jalur Absolut
FILES=(
  ".github/workflows/build.yml"
  "app/src/main/assets/index.html"
  "app/src/main/assets/sw.js"
  "app/src/main/assets/lib.rs"
  "app/src/main/AndroidManifest.xml"
  "app/src/main/res/layout/activity_main.xml"
  "app/src/main/res/values/themes.xml"
  "app/src/main/java/com/vuzt/MainActivity.java"
  "app/build.gradle"
  "build.gradle"
)

for f in "${FILES[@]}"; do
  if [ -f "$f" ]; then
    echo "================================================================================"
    echo " FILE: $f"
    echo "================================================================================"
    cat "$f"
    echo -e "\n"
  else
    echo "--------------------------------------------------------------------------------"
    echo " [!] File $f tidak ditemukan"
    echo "--------------------------------------------------------------------------------"
    echo -e "\n"
  fi
done
