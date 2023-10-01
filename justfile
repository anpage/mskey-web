set windows-shell := ["powershell.exe", "-NoLogo", "-Command"]

alias b := build
alias s := serve

clean:
    trunk clean

build:
    trunk build --release

serve:
    trunk serve --release --open

deploy: clean build
    surge .\dist\ https://mskt.surge.sh

npm-install:
    npm install

tauri-build: npm-install
    npm run tauri build

tauri-serve: npm-install
    npm run tauri dev
