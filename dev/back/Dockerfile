FROM rust:1.31.0

LABEL   maintainer="krondev@pm.me" \
        org.label-schema.build-date="n/a" \
        org.label-schema.name="molva-app" \
        org.label-schema.description="p2p messenger" \
        org.label-schema.vendor="Eliot Krone" \
        org.label-schema.vcs-url="github.com/krondev/molva" \
        org.label-schema.vcs-ref="n/a" \
        org.label-schema.version="n/a"

WORKDIR /usr/code
COPY . /usr/code

RUN apt-get update && apt-get install -y curl gcc clang

RUN curl https://sh.rustup.rs -sSf | sh -s -- -y --default-toolchain stable
ENV PATH /root/.cargo/bin:$PATH

# RUN rustup target add wasm32-unknown-unknown
# RUN rustup toolchain install nightly
# RUN rustup target add --toolchain nightly wasm32-unknown-unknown

RUN export LIBCLANG_PATH='/usr/lib64/llvm/6/lib64/libclang.so.6.0'

RUN rustc --version
RUN cargo build

EXPOSE 10333
CMD ["/usr/code/target/debug/molva", "10333"]
