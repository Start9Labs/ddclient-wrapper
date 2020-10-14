ASSETS := $(shell yq r manifest.yaml assets.*.src)
ASSET_PATHS := $(addprefix assets/,$(ASSETS))
VERSION := $(shell yq r manifest.yaml version)
CONFIGURATOR_SRC := $(shell find ./configurator -name '*.rs') configurator/Cargo.toml configurator/Cargo.lock

.DELETE_ON_ERROR:

all: ddclient.s9pk

install: ddclient.s9pk
	appmgr install ddclient.s9pk

ddclient.s9pk: manifest.yaml config_spec.yaml config_rules.yaml image.tar instructions.md $(ASSET_PATHS)
	appmgr -vv pack $(shell pwd) -o ddclient.s9pk
	appmgr -vv verify ddclient.s9pk

image.tar: Dockerfile docker_entrypoint.sh configurator/target/armv7-unknown-linux-musleabihf/release/configurator
	DOCKER_CLI_EXPERIMENTAL=enabled docker buildx build --tag start9/ddclient --build-arg DDCLIENT_VERSION=$(VERSION) --platform=linux/arm/v7 -o type=docker,dest=image.tar .

configurator/target/armv7-unknown-linux-musleabihf/release/configurator: $(CONFIGURATOR_SRC)
	docker run --rm -it -v ~/.cargo/registry:/root/.cargo/registry -v "$(shell pwd)"/configurator:/home/rust/src start9/rust-musl-cross:armv7-musleabihf cargo build --release
	docker run --rm -it -v ~/.cargo/registry:/root/.cargo/registry -v "$(shell pwd)"/configurator:/home/rust/src start9/rust-musl-cross:armv7-musleabihf musl-strip target/armv7-unknown-linux-musleabihf/release/configurator
