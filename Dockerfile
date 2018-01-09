FROM fedora:latest

WORKDIR /usr/src/rice_bot2
COPY . .
RUN dnf install -y https://download1.rpmfusion.org/free/fedora/rpmfusion-free-release-$(rpm -E %fedora).noarch.rpm \
	 && dnf install -y cargo rust pkg-config libsodium-devel opus-devel compat-openssl10-devel youtube-dl ffmpeg \
	&& cargo update && cargo build --release

ARG discord_token
ENV DISCORD_TOKEN=${discord_token}

CMD DISCORD_TOKEN=${discord_token} cargo run --release
