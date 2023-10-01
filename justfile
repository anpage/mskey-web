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

tauri-build: clean
    trunk build --release

tauri-serve:
    trunk serve --release

