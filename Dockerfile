FROM jimmycuadra/rust:1.1.0

ADD . .
RUN ["/bin/sh", "-c", "cargo build"]
ENTRYPOINT ["/bin/sh", "-c", "cargo run"]
EXPOSE 8333
