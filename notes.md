# Using Docker

* `docker build -t name:tag .` to build the image
* `docker run -p HOSTPOST:CONTAINERPORT` to run the image and publish ports

# Database

* `docker pull postgres` to pull the postgres image
* `docker run --name todont-postgres -e POSTGRES_PASSWORD=password -d -p 5432:5432 postgres` to create and the container
* `psql -h 127.0.0.1 -p 5432 -U postgres` to connect to the container from outside
* [Diesel Getting Started](https://diesel.rs/guides/getting-started)

# Rust things

* Use tokio's mutex instead of `std::sync`'s because `std::sync`'s doesn't play well with async
* We need a mutex because the extensions are shared globally, so we can't get a mutable reference directly
