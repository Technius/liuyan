# Liuyan

A lightweight comment server written in Rust. Currently a work-in-progress, not
ready for production use.

## Building

First, make sure `diesel_cli` is installed with `cargo`. Then, run `diesel_cli
migration run` to generate the database schema. Make sure you have an instance
of `postgres` running; a `docker-compose.yml` file is provided to quickly start
up `postgres`.Finally, build with `cargo build`.

## TODO

- [x] Thread endpoint
- [x] User endpoint
- [x] Comment endpoint
- [x] Basic posting functionality
- [ ] Refactor query code into separate modules
- [ ] Split up or modularize `routes.rs`
- [ ] CORS with configuration options
- [ ] Email integration: verification and reply-posting
- [ ] Optional voting/points system
- [ ] Implement a comment form for end-users
- [x] Use `r2d2` instead of a `Mutex`'d connection
- [ ] Offer redis option for sessions
- [ ] Packaging
- [ ] Write rustdocs and usage

## License
Copyright 2017 Bryan Tan

Licensed under the Apache License, Version 2.0 (the "License"); you may not use
this file except in compliance with the License. You may obtain a copy of the
License at

> http://www.apache.org/licenses/LICENSE-2.0

Unless required by applicable law or agreed to in writing, software distributed
under the License is distributed on an "AS IS" BASIS, WITHOUT WARRANTIES OR
CONDITIONS OF ANY KIND, either express or implied. See the License for the
specific language governing permissions and limitations under the License.
