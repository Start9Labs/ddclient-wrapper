FROM alpine:3.12 as builder

ARG DDCLIENT_VERSION
RUN test -n "$DDCLIENT_VERSION"

RUN apk update
RUN apk add git make autoconf automake

WORKDIR /root
RUN git clone https://github.com/ddclient/ddclient.git
WORKDIR /root/ddclient
#RUN git checkout v${DDCLIENT_VERSION}
RUN ./autogen
RUN ./configure \
    --prefix=/usr \
    --sysconfdir=/etc/ddclient \
    --localstatedir=/var
RUN make
RUN make VERBOSE=1 check

FROM alpine:3.12

RUN apk update
RUN apk add tini

RUN apk add perl perl-utils perl-test-taint perl-netaddr-ip perl-net-ip perl-yaml perl-log-log4perl perl-io-socket-ssl

COPY --from=builder /root/ddclient/ddclient /usr/local/bin/ddclient

ARG DDCLIENT_VERSION
RUN test -n "$DDCLIENT_VERSION"

ADD ./configurator/target/armv7-unknown-linux-musleabihf/release/configurator /usr/local/bin/configurator
RUN chmod a+x /usr/local/bin/configurator
ADD ./docker_entrypoint.sh /usr/local/bin/docker_entrypoint.sh
RUN chmod a+x /usr/local/bin/docker_entrypoint.sh

ENTRYPOINT ["/usr/local/bin/docker_entrypoint.sh"]
