FROM rustlang/rust:nightly

WORKDIR /build
COPY src /build/src
COPY templates /build/templates 
COPY Cargo.toml /build/
COPY Cargo.lock /build/
RUN cargo build --release

WORKDIR /app/
RUN cp /build/target/release/rr_screensharing /app/
COPY Rocket.toml /app
COPY static /app/static

CMD /app/rr_screensharing