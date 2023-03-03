FROM rust:1.67.1 as builder
RUN USER=root

RUN mkdir karting_groningen_analytics
WORKDIR /karting_groningen_analytics
ADD . ./

RUN rustup default nightly

RUN cargo clean && cargo build --release

FROM debian:bullseye as runner
ARG APP=/user/src/app
RUN mkdir -p {$APP}

RUN apt update -y
RUN apt install -y libpq-dev

# Copy the compiled binaries into the new container.
COPY --from=builder /karting_groningen_analytics/target/release/karting_groningen_analytics ${APP}/karting_groningen_analytics
COPY --from=builder /karting_groningen_analytics/Rocket.toml ${APP}/Rocket.toml
COPY --from=builder /karting_groningen_analytics/.env ${APP}/.env

WORKDIR ${APP}

EXPOSE 8089

CMD ["./karting_groningen_analytics"]