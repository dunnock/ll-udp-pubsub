# Compile trader binary. This ensures that we enable all correct compile flags
# and allow compilation on non-compatible dev machines.

COMMIT_HASH := $(shell git show -s --format='%h' HEAD)
COMMIT_TAG := $(shell git tag --points-at HEAD | head -n1)
COMMIT_DATE := $(shell git show -s --format='%cs' HEAD | tr -d '-')

RELEASE_ID := $(if $(COMMIT_TAG),$(COMMIT_TAG),$(COMMIT_DATE)-$(COMMIT_HASH))

FORCE:
	mkdir -p build

version-info: FORCE
	mkdir -p build
	git show -s --format="%H%d" HEAD > build/version-info

dist: version-info
	rm -f ll-udp-pubsub-$(RELEASE_ID).tar
	sudo DOCKER_BUILDKIT=1 docker build \
		-f ./Dockerfile \
		--output type=tar,dest=ll-udp-pubsub-$(RELEASE_ID).tar .
	sudo chown ${USER} ll-udp-pubsub-$(RELEASE_ID).tar
	tar -rf ll-udp-pubsub-$(RELEASE_ID).tar build/version-info README.md
