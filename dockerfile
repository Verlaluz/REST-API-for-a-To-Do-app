FROM rust:latest
WORKDIR /usr/src/todo_api
COPY . .
RUN cargo build --release

# Expose port 3000 so we can access it from our Windows browser
EXPOSE 3000

ENTRYPOINT ["./target/release/todo_api"]