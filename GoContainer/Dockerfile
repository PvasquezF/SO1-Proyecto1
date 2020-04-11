FROM golang:1.13-buster as build-env
#FROM golang:1.8.7-alpine3.6 as build-env
#RUN apk update
#RUN apk add git

#WORKDIR /go/src/github.com/go-redis/redis/
#RUN git clone https://github.com/go-redis/redis.git .
#RUN git checkout v6

#WORKDIR /go/src/github.com/mackerelio/
#RUN git clone https://github.com/mackerelio/go-osstat

WORKDIR /go/src/app
ADD . /go/src/app

RUN go get -d -v ./...

RUN go build -o /go/bin/app

#FROM gcr.io/distroless/base
#COPY --from=build-env /go/bin/app /
#CMD ["/go/bin/app"]

FROM gcr.io/distroless/base-debian10
COPY --from=build-env /go/bin/app /
COPY --from=build-env /go/src/app /
CMD ["/app"]