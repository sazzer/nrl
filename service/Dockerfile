FROM golang:1.15.2-alpine3.12 AS builder

WORKDIR /nrl

COPY go.mod go.sum /nrl/
COPY cmd /nrl/cmd/
COPY internal /nrl/internal/
COPY tests /nrl/tests/

RUN go get -u github.com/gobuffalo/packr/v2/packr2
RUN go generate ./...
RUN go build -o output/nrl ./cmd/nrl/*


FROM alpine:3.12

ENV DOCKERIZE_VERSION v0.6.1
RUN wget https://github.com/jwilder/dockerize/releases/download/$DOCKERIZE_VERSION/dockerize-linux-amd64-$DOCKERIZE_VERSION.tar.gz \
  && tar -C /usr/local/bin -xzvf dockerize-linux-amd64-$DOCKERIZE_VERSION.tar.gz \
  && rm dockerize-linux-amd64-$DOCKERIZE_VERSION.tar.gz

WORKDIR /nrl

COPY --from=builder /nrl/output/nrl /nrl/nrl

CMD /nrl/nrl
