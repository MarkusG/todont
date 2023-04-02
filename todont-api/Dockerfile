FROM rust:latest as build

WORKDIR /usr/src/todont-api
COPY . .

RUN cargo build --release

FROM gcr.io/distroless/cc-debian10

COPY --from=build /usr/src/todont-api/target/release/todont-api /usr/local/bin/todont-api

WORKDIR /usr/local/bin
CMD ["todont-api"]
