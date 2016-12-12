FROM buildbot/buildbot-worker:master

RUN curl https://sh.rustup.rs -sSf | sh -s -- -y --default-toolchain nightly
RUN echo 'PATH=$PATH:$HOME/.cargo/bin' >> $HOME/.bashrc

USER root
RUN apt update -y
RUN apt install -y cmake pkg-config

USER buildbot
