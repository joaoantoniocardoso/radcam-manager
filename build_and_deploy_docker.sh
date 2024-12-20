#!/usr/bin/env bash
set -e

# Define the architectures and corresponding targets
declare -A TARGETS
TARGETS=(
  ["linux/amd64"]="x86_64-unknown-linux-musl"
  ["linux/arm64"]="aarch64-unknown-linux-musl"
  ["linux/arm/v7"]="armv7-unknown-linux-musleabihf"
)

# Create a comma-separated list of platforms
PLATFORMS=""
BUILD_ARGS=()
for PLATFORM in "${!TARGETS[@]}"; do
    PLATFORMS+="$PLATFORM,"
    BUILD_ARGS+=(--build-arg TARGET="${TARGETS[$PLATFORM]}")
done
PLATFORMS="${PLATFORMS%,}"  # Remove the trailing comma

echo "Building multi-platform image for: $PLATFORMS"

# Run Docker buildx for all platforms, including dynamically generated build arguments
docker buildx build \
    --platform "$PLATFORMS" \
    "${BUILD_ARGS[@]}" \
    -t joaoantoniocardoso/radcam-manager:latest \
    --push \
    --output type=registry .

echo "Multi-architecture build and push completed!"
