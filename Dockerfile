FROM alpine:3.19

ENV PORT=3060
ENV WWW_ROOT=/var/www/html/
ENV RUST_LOG=info

EXPOSE ${PORT}

COPY target/x86_64-unknown-linux-musl/release/http-server /usr/bin/

ENTRYPOINT [ "http-server" ]
