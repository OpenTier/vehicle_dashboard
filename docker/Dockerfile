# build stage
FROM rust:1.81.0 AS builder

WORKDIR /app

# install curl and unzip
RUN apt-get update && apt-get install -y \
    protobuf-compiler

# copy dependency files and build dependencies
COPY ../Cargo.toml /app/Cargo.toml
COPY ../Cargo.lock /app/Cargo.lock

# copy app's files
COPY ../proto /app/proto
COPY ../ui /app/ui
COPY ../src /app/src
COPY ../build.rs /app/build.rs

# build app
RUN cargo build --release --bin vehicle-dashboard

# run stage
FROM ubuntu:22.04 AS runner

# create a non-root user and group
RUN groupadd -r appuser && useradd -r -g appuser appuser

# install dependencies needed to run dashboard
RUN DEBIAN_FRONTEND=noninteractive apt-get update && apt-get install -y \
    libx11-xcb1 \
    libxkbcommon-x11-0 \
    x11-apps \
    libxcursor1 \
    libxi6 \
    libxcb-glx0 \
    libxcb-keysyms1 \
    libxcb-image0 \
    libxcb-shm0 \
    libxcb-icccm4 \
    libxcb-sync1 \
    libxcb-xfixes0 \
    libxcb-shape0 \
    libxcb-randr0 \
    libxcb-render-util0 \
    libxrender1 \
    libxcb1 \
    libfontconfig1 \
    libfreetype6 \
    libqt5gui5 \
    libqt5core5a \
    libqt5widgets5 \
    && apt-get clean && rm -rf /var/lib/apt/lists/*

# copy the built binary to the new image and set permissions
COPY --from=builder /app/target/release/vehicle-dashboard /usr/local/bin/
RUN chmod +x /usr/local/bin/vehicle-dashboard

# switch to the non-root user
USER appuser

# set run script as entrypoint
ENTRYPOINT ["vehicle-dashboard"]
