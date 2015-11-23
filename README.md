rust-leecher
============

A leecher server which accepts urls by a GET parameter `url`, and downloads it to `path` on server.
Leecher supports YouTube urls (using `youtube-dl`) and optinally accepts a `quality` parameter, too.

This is an experimental project I wrote to learn Rust's HTTP servers, you probably shouldn't use it in production.
