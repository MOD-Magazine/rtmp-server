# MOD! Magazine RTMP Tools

This repository holds tools used by MOD! Magazine to stream (to YouTube and
Twitch) and record simultaneously while only opening one network connection from
the host computer (instead of connecting to Twitch, to YouTube, and recording all at once).

## Usage

1. Clone this repository.
2. Fill out the `TODO`s in `docker-compose.yml`.
3. Remove the relevant commented lines in `rtmp/nginx.conf`
4. `docker-compose build`
5. `docker-compose up`

## Architecture

### Nginx

Nginx runs the RTMP server that forwards to both YouTube and Twitch as well as recording. It is accessible via `rtmp://server_ip:1935/stream`.

### Authentication Server

The authentication server is a simple Rust/Rocket.rs web server that authenticates clients via stream keys.

