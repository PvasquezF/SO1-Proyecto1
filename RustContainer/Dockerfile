FROM rust:1.41.0 as build-env

WORKDIR /app
ADD . /app

RUN rustup default nightly
RUN cargo build --release
FROM gcr.io/distroless/cc
COPY --from=build-env /app/target/release/pavel /
COPY --from=build-env /app/Template/ /Template/
EXPOSE 8888
CMD ["./pavel"]