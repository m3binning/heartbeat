ARG BASE_IMAGE=debian
ARG BASE_VERSION=12.1
FROM ${BASE_IMAGE}:${BASE_VERSION} as base
USER root
WORKDIR /usr/local/src
ADD "https://github.com/digitalocean/prometheus-client-c/releases/download/v0.1.3/libprom-dev-0.1.3-Linux.deb" ./
ADD "https://github.com/digitalocean/prometheus-client-c/releases/download/v0.1.3/libpromhttp-dev-0.1.3-Linux.deb" ./
RUN chown _apt ./*.deb && \
    apt update && \
    apt install -y libmicrohttpd-dev ./*.deb
FROM base as builder
RUN apt update && apt install -y gcc make
COPY *.c include/ Makefile ./
RUN make
FROM base as runner
COPY --from=builder /usr/local/src/heartbeat /bin
ENTRYPOINT [ "heartbeat" ]
CMD [ "--help" ]
