FROM ubuntu:20.04
WORKDIR /app
COPY target/release/with-baby-auth .
RUN apt update && apt install -y libpq-dev
ENTRYPOINT ["/app/with-baby-auth"]