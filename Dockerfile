FROM ubuntu:22.04 as base
ARG TARGETPLATFORM
COPY . /tmp
WORKDIR /tmp

RUN echo $TARGETPLATFORM
RUN ls -R /tmp/
# move the binary to root based on platform
RUN case $TARGETPLATFORM in \
        "linux/amd64")  BUILD=x86_64-unknown-linux-gnu  ;; \
        "linux/arm64")  BUILD=aarch64-unknown-linux-gnu  ;; \
        *) exit 1 ;; \
    esac; \
    mv /tmp/$BUILD/fast-pbf-server-$BUILD /fast-pbf-server; \
    chmod +x /fast-pbf-server;

FROM ubuntu:22.04

COPY --from=base /fast-pbf-server /fast-pbf-server

ENTRYPOINT ["/fast-pbf-server"]