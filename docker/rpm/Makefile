.PHONY: all build

all: build
	docker run -it --rm -v ./out:/host rpm

build:
	docker build -t rpm .
