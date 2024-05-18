FROM rust:alpine
WORKDIR /usr/src/desperates_everywhere
COPY . .

RUN apk --no-cache add musl-dev
RUN cargo install --path .

EXPOSE 8080

CMD ["desperates_everywhere"]
