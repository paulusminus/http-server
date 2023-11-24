FROM alpine:3.18

EXPOSE 3001

COPY target/x86_64-unknown-linux-musl/release/serve-paulmin-nl /usr/bin/

RUN mkdir paulmin-nl
RUN mkdir lipl-book

ENTRYPOINT [ "serve-paulmin-nl" ]
