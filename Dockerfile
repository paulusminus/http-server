FROM alpine:3.18

EXPOSE 3001

COPY target/x86_64-unknown-linux-musl/release/serve-paulmin-nl /usr/bin/

RUN wget -O lipl-book.tar https://github.com/paulusminus/lipl-book/releases/latest/download/lipl-book.tar \
 && mkdir lipl-book \
 && tar -xf lipl-book.tar -C lipl-book \
 && rm lipl-book.tar

ADD paulmin-nl.tar paulmin-nl

ENTRYPOINT [ "serve-paulmin-nl" ]
