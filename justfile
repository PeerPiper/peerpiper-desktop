# use just.systems variable for the ollama version
version := "v0.1.33"

tauri:
  npm run tauri dev

# This is called from github/workflows, if you change this name, change that file too
install_ollama_linux:
  echo "Installing ollama on Linux"
  # wget https://github.com/jmorganca/ollama/releases/download/v0.1.20/ollama-darwin
  wget https://github.com/ollama/ollama/releases/download/{{version}}/ollama-linux-amd64
  chmod +x ollama-linux-amd64
  # Tauri needs this specific name
  mv ollama-linux-amd64 src-tauri/ollama-x86_64-unknown-linux-gnu 

install_ollama_macos:
  echo "Installing ollama on Mac"
  wget https://github.com/jmorganca/ollama/releases/download/{{version}}/ollama-darwin
  chmod +x ollama-darwin

  # Tauri needs this specific name
  mv ollama-darwin src-tauri/ollama-aarch64-apple-darwin

install_ollama_windows:
  echo "Installing ollama on Windows"
  curl -L -O -o . "https://github.com/ollama/ollama/releases/download/{{version}}/ollama-windows-amd64.zip"
  unzip ollama-windows-amd64.zip

  # Tauri needs this specific name
  mv ollama.exe src-tauri/ollama-x86_64-pc-windows-msvc.exe
  # mv all the *.dll files too, they can keep the same name
  mv *.dll src-tauri
