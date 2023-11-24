FROM alpine:3.18

EXPOSE 3001

COPY target/x86_64-unknown-linux-musl/release/serve-paulmin-nl /usr/bin/

ADD https://github.com/paulusminus/lipl-book/releases/download/v1.2.0/lipl-book.tar lipl-book
ADD paulmin-nl.tar paulmin-nl

ENTRYPOINT [ "serve-paulmin-nl" ]
