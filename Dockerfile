FROM rustlang/rust:nightly

WORKDIR /
COPY . .

RUN cargo build --release

CMD ./target/release/rocket