# Liuyan

A lightweight comment server written in Rust. Currently a work-in-progress.

TODO:

- [ ] Thread endpoint
- [ ] User endpoint
- [ ] Comment endpoint
- [ ] Basic posting functionality

## Building

First, make sure `diesel_cli` is installed with `cargo`. Then, run `diesel_cli
migration run` to generate the database. Finally, build with `cargo build`.

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
