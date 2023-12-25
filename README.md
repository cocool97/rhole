# rhole: Privacy as a whole

[![dependency status](https://deps.rs/repo/github/cocool97/rhole/status.svg)](https://deps.rs/repo/github/cocool97/rhole)
[![codecov](https://codecov.io/gh/cocool97/rhole/branch/master/graph/badge.svg)](https://codecov.io/gh/cocool97/rhole)

rhole is a fully open-source and community Rust / ReactJS project aiming to empower your privacy, by providing an easy-to-setup DNS server.

It is focused on **<ins>performance</ins>** and **<ins>usability</ins>** to be as transparent as possible on your network.

With rhole you will be able to :

- Block all kind of DNS records like advertising, pornography, malware, fake news...
- Monitor your DNS trafic in real-time, configure the server or unblock domains using the provided web interface.
- Serve your own DNS records.
- [**TODO**] Increasing your privacy on Internet by providing a DOH (DNS-Over-HTTP) recursor, aiming to reduce clear DNS traffic.
- [**TODO**] Sign DNS records according to DNSSEC specification
- [**TODO**] Setup a HTTP(s) proxy being able to inspect requests and block as you wish.
- [**TODO**] Setup a DOH (DNS-Over-HTTP) server.
- [**TODO**] Setup a DOT (DNS-Over-TLS) server.
- [**TODO**] Use an Android/iOS application to monitor your trafic and block domains locally, without ask the server.

## How does it work ?

`rhole` reads blacklist entries from various remote sources, inserts them in a database and checks against it for every DNS requests received on its server. Domains are cached to improve performances.

On a DNS level, many projects are providing up-to-date blacklists, I personally recommend using [StevenBlack's](https://github.com/StevenBlack/hosts) as it is configurable in what topics can be blocked. To maximize the blocking scope, many lists can be inserted in parallel.

## Setup

All these methods require using `rhole` as a primary DNS server. This setup will depend on your distribution.

It is therefore recommended to set the server IP directly in your DHCP server configuration as `option domain-name-servers` to automatically inject this server to all clients configured via DHCP.

### Using Docker / Podman

A `Dockerfile` is provided to ease setup.

```bash
docker build -t rhole:latest -f Dockerfile .

docker run \
  -it \
  --rm \
  -v config.yml:/etc/rhole/config.yml:ro,z \ # rhole configuration file
  -v data:/etc/rhole/data:rw,z \ # rhole data directory, containing database
  -p 53:53/udp \
  -p 443:443/tcp \
  -e CONFIG_PATH="/etc/rhole/config.yml" \
  -e DNS_ADDR="0.0.0.0:53" \
  -e WEB_ADDR="0.0.0.0:443" \
  -e DEBUG=1 \
  rhole:latest
```

### Locally

TODO

## How to use it ?

### Write initial configuration file

- Will not be mandatory after

TODO

### Access WEB UI

TODO

- URL
- Screen(s)
- Add license
