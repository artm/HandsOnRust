FROM mcr.microsoft.com/devcontainers/rust:1-1-bullseye

# Install the necessary system libraries
RUN apt-get update && apt-get install -y \
    libwayland-dev \
    libxkbcommon-dev \
    libegl1-mesa \
    libgl1-mesa-glx \
    libgl1-mesa-dri \
    libglu1-mesa \
    && apt-get clean \
    && rm -rf /var/lib/apt/lists/*