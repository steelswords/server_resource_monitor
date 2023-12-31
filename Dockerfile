FROM rust as builder

WORKDIR /app
RUN rustup target add x86_64-unknown-linux-musl
RUN apt update && apt install -y musl-tools musl-dev
#RUN git clone https://github.com/steelswords/server_resource_monitor.git
COPY . /app/server_resource_monitor
RUN cd server_resource_monitor &&\
    cargo build --target x86_64-unknown-linux-musl --release


#######################
# Final image
#######################
FROM alpine:latest
RUN apk add lm-sensors
COPY --from=builder /app /app
COPY --from=builder /app/server_resource_monitor/target/x86_64-unknown-linux-musl/release/resource_monitor /app/resource_monitor
COPY --from=builder /app/server_resource_monitor/Rocket.toml /app/Rocket.toml
CMD /app/resource_monitor
