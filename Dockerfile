FROM alpine:3.18

EXPOSE 3001

COPY target/x86_64-unknown-linux-musl/release/serve-paulmin-nl /usr/bin/

RUN wget -O lipl-book.tar https://github.com/paulusminus/lipl-book/releases/latest/download/lipl-book.tar \
 && mkdir lipl-book \
 && tar -xf lipl-book.tar --no-same-owner -C lipl-book \
 && rm lipl-book.tar

RUN wget -O picocss.tar.gz https://github.com/picocss/pico/archive/refs/tags/v1.5.10.tar.gz \
&& mkdir paulmin-nl \
&& tar -xzf picocss.tar.gz --no-same-owner -C paulmin-nl \
&& rm picocss.tar.gz

ADD paulmin-nl.tar paulmin-nl

ENTRYPOINT [ "serve-paulmin-nl" ]
