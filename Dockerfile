FROM rustlang/rust:nightly

WORKDIR /app
COPY . .

RUN cargo build --release

CMD ./target/release/rocket