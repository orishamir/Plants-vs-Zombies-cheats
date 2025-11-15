FROM rust:1.90.0-trixie

RUN apt update && apt upgrade -y
RUN apt install -y \
    mingw-w64 \
    neovim \
    jq \
    less

ENV CARGO_BUILD_TARGET=x86_64-pc-windows-gnu

RUN rustup target add x86_64-pc-windows-gnu
RUN rustup component add clippy

COPY <<EOF /root/.bashrc

source /usr/share/bash-completion/completions/git

alias gs="git status"
alias gcm="git commit -m"
alias gp="git push"
alias ga="git add"
EOF
