FROM rust:latest as build

WORKDIR /usr/src/shaver-api
COPY . .

RUN cargo build --release

FROM gcr.io/distroless/cc-debian10

COPY --from=build /usr/src/shaver-api/target/release/shaver-api /usr/local/bin/shaver-api

WORKDIR /usr/local/bin
CMD ["shaver-api"]
