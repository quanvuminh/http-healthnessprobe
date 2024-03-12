# HTTP healthnessProbe

## Table of Contents
+ [About](#about)
+ [Getting Started](#getting_started)
+ [Usage](#usage)

## About <a name = "about"></a>

This project aims to convert Kubernetes container liveness/readiness probe command to HTTP request. When the host having high disk i/o usage, probe command left a lot of zombie process that can cause process leak. We discoverd this issue while investigating https://github.com/containerd/containerd/issues/7496.

## Getting Started <a name = "getting_started"></a>

These instructions will get you a copy of the project up and running on your local machine for development and testing purposes. See [usage](#usage) for notes on how to deploy the project on a live system.

### Prerequisites

Rust installed for local development without Docker.

### Build

#### Development

```bash
$ git clone https://github.com/quanvuminh/http-healthnessprobe.git
$ cd http-healthnessprobe
$ cargo build
```

#### Docker

```bash
$ git clone https://github.com/quanvuminh/http-healthnessprobe.git
$ cd http-healthnessprobe
$ docker build . -t your-registry.example.com/http-healthnessprobe
```

## Usage <a name = "usage"></a>

#### Docker

```bash
$ docker run --rm -it -p 8080:8080 ghcr.io/quanvuminh/http-healthnessprobe
```
