# Using Docker

* `docker build -t name:tag .` to build the image
* `docker run -p HOSTPOST:CONTAINERPORT` to run the image and publish ports

# Rust things

* Use tokio's mutex instead of `std::sync`'s because `std::sync`'s doesn't play well with async
* We need a mutex because the extensions are shared globally, so we can't get a mutable reference directly
