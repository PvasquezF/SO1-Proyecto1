FROM golang:1.13-buster as build-env
WORKDIR /go/src/app
ADD . /go/src/app
RUN go get -d -v ./...
RUN go build -o /go/bin/app
FROM gcr.io/distroless/base-debian10
COPY --from=build-env /go/bin/app /
COPY --from=build-env /go/src/app /
CMD ["/app"]