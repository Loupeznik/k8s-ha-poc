# ha-tester

A minimalist web service built in Rust that provides basic information about the host machine it's running on.

Can server in HA setup testing to see to which machine/container is the traffic getting routed to.

## Run via Docker

```shell
docker run -d -p 3000:3000 loupeznik/ha-tester:1.0.1
```

## Install via Helm

```shell
helm repo add loupeznik https://helm.dzarsky.eu
helm upgrade --install ha-tester loupeznik/ha-tester
```
