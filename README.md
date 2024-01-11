# rhole: Privacy as a whole

<p align="center" style="text-align: center">
  <img src="https://raw.githubusercontent.com/cocool97/rhole/main/rhole-web/public/favicon.png" width="33%"><br/>
</p>

[![dependency status](https://deps.rs/repo/github/cocool97/rhole/status.svg)](https://deps.rs/repo/github/cocool97/rhole)
[![codecov](https://codecov.io/gh/cocool97/rhole/branch/master/graph/badge.svg)](https://codecov.io/gh/cocool97/rhole)
[![docker pull](https://img.shields.io/docker/pulls/cocool97/rhole.svg)](https://hub.docker.com/repository/docker/cocool97/rhole/tags)

rhole is a fully open-source and community Rust / ReactJS project aiming to empower your privacy, by providing an easy-to-setup DNS server.

It is focused on **performance** and **usability** to be as transparent as possible on your network.

With rhole you will be able to :

- Block all kind of DNS records like advertising, pornography, malware, fake news...
- Monitor your DNS trafic in real-time, configure the server or unblock domains using the provided web interface.
- Serve your own DNS records.
- Increasing your privacy on Internet by providing a DOH (DNS-Over-HTTPS) recursor, aiming to reduce clear DNS traffic.
- [**TODO**] Sign DNS records according to DNSSEC specification
- [**TODO**] Setup a HTTP(s) proxy being able to inspect requests and block as you wish.
- [**TODO**] Setup a DOH (DNS-Over-HTTPS) server.
- [**TODO**] Setup a DOT (DNS-Over-TLS) server.
- [**TODO**] Use an Android/iOS application to monitor your trafic and block domains locally, without ask the server.

## How does it work ?

`rhole` reads blacklist entries from various remote sources, inserts them in a database and checks against it for every DNS requests received on its server. Domains are cached to improve performances.

On a DNS level, many projects are providing up-to-date blacklists, I personally recommend using [StevenBlack's](https://github.com/StevenBlack/hosts) as it is configurable in what topics can be blocked. To maximize the blocking scope, many lists can be inserted in parallel.

## Setup

All these methods require using `rhole` as a primary DNS server. This setup will depend on your distribution.

It is therefore recommended to set the server IP directly in your DHCP server configuration as `option domain-name-servers` to automatically inject this server to all clients configured via DHCP.

### Configuration variables

Many things may be configured with `rhole`. Some of these things are configurable using environment variables, other using a yaml configuration file.

You can find here the available environment variables:

| Environment variable |  What can be configured  |
|:--------------------:|:------------------------:|
|        DEBUG         |     Enable debug mode    |
|      CONFIG_PATH     |Path to configuration file|
|     DATABASE_PATH    |  Path to rhole database  |
|        HTML_DIR      |Path to HTML code directory|
|      CACHE_SIZE      | Increase DNS cache size  |
|       DNS_ADDR       |    Listen DNS address    |
|       WEB_ADDR       |    Listen WEB address    |

### Configuration file

Currently a configuration file is needed as well as the previous environment variables to configure `rhole`. The path to this configuration file can be configured using the `CONFIG_PATH` environment variable.

This will be temporary and will be replace by a configuration directly in the WEB UI.

Here is the required configuration format under YAML :

```yaml
# Configure TLS directly on rhole server
# tls:
#   certificate_path: /etc/rhole/ssl/crt.pem
#   pkey_path: /etc/rhole/ssl/key.pem

# Server on which to recurse for DNS queries (with DOH)
proxy_server:
  ip: "1.1.1.1"
  port: 853
  tls_dns_name: cloudflare-dns.com

# Blacklist sources
sources:
  update_interval: 5
  entries:
    # - source_type: !File
    #   location: hosts.txt
    #   comment: Global hosts file
    - source_type: !Network
      location: http://sbc.io/hosts/alternates/fakenews-gambling-porn/hosts
      comment: Remote hosts file
```

## Deployment

A Docker image is compiled at every release and is available on Docker Hub: <https://hub.docker.com/repository/docker/cocool97/rhole/tags>

### Using containers: Docker / Podman

A `Dockerfile` is provided to ease setup.

The previous environment variables can be mounted in Docker image using `-e VAR=VALUE` flag (many times if needed).

```bash
docker run \
  -it \
  --rm \
  -v config.yml:/etc/rhole/config.yml:ro,z \ # rhole configuration file
  -v data:/etc/rhole/data:rw,z \ # rhole data directory, containing database
  -p 53:53/udp \
  -p 443:443/tcp \
  -e DNS_ADDR="0.0.0.0:53" \
  -e WEB_ADDR="0.0.0.0:443" \
  cocool97/rhole:latest
```

### Host it behind a reverse proxy

This may be a good security practice to host `rhole` behind a reverse proxy like Nginx, Apache...

Configuration is classic, except the fact that websockets must be forwarded to allow for real-time traffic monitoring. The configuration may differ depending on which reverse proxy you choose, you may need to have a look at documentation.

## How to use it ?

### Access WEB UI

`rhole` hosts an administration interface useful to follow global or per-client live trafic, monitor blacklisted domains or display server logs. This interface can be accessed by default on 0.0.0.0:443, but this can be configured using the `WEB_ADDR` environment variable.

I'm definitely not a web developer, so this interface is currently in its beta form. Pull requests are welcome to improve UI or UX, or make the ReactJS code closer to best practices !
