FROM rust:alpine3.20

WORKDIR /workspaces/ferchat/

RUN apk update && apk upgrade \
    && apk add --no-cache alpine-sdk \
    && apk add --no-cache python3 py3-pip \
    && apk add --no-cache openssl-dev \
    && apk add --no-cache openssl-libs-static
