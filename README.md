Heartbeat
===============================================================================

This is an example Prometheus client written in C.
It generates a heatbeat message.

Build
-------------------------------------------------------------------------------

`docker build -f Containerfile -t heartbeat .`

Run
-------------------------------------------------------------------------------

To view the CLI output, use the `-i` flag as below. To free up the terminal
(e.g. only the metrics at the endpoint are of interest), dettach with the `-d`
flag.

`docker run --rm --name heartbeat -tip 8000:8000 heartbeat`

Stop
-------------------------------------------------------------------------------

If running interactively, send a SIGINT with Ctrl+c.
If running dettached, simply stop with: `docker stop heartbeat`

Test
-------------------------------------------------------------------------------

`curl http://localhost:8080/metrics` also available from web browsers near you.
On an additional command or a refresh, the heartbeat metric should increase.

Install
-------------------------------------------------------------------------------

On the install target, the container may be run, or the client may be
copied out of the container and run directly on the target host.

To run directly on a target host, the dynamically linked dependencies for a
Prometheus client will need to be available on the target host. For a dpkg-
based host (e.g. Debian, Ubuntu, etc.) follow the commands in the
`Containerfile`. For installing libraries in a non-Debian (but typical Linux
FHS), i.e. RHEL, use the below steps in place of the base stage of the
Containerfile.

```Dockerfile
FROM ${BASE_IMAGE}:${BASE_VERSION} as base
ADD "https://github.com/digitalocean/prometheus-client-c/releases/download/v0.1.3/libprom-dev-0.1.3-Linux.tar.gz" ./
RUN tar -xzf libprom*.tar.gz && \
    rm libprom*.tar.gz && \
    mv libprom*/lib/* /usr/lib/ && \
    mv libprom*/include/* /usr/include
```
