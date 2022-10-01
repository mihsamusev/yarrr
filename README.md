# YARRR - Yet Another Rust Raytracer Rewrite

[![](https://github.com/mihsamusev/yarrr/actions/workflows/build.yml/badge.svg)](https://github.com/mihsamusev/yarrr/actions/workflows/build.yml)

Study exercise in rewriting the famous [Ray Tracing in One Weekend by Peter Shirley](https://raytracing.github.io/books/RayTracingInOneWeekend.html#thevec3class/vec3utilityfunctions) from C++ to Rust to practice the language. This is also my final project submission of the course [OTUS Rust Developer](https://otus.ru/lessons/rust-developer/).

## Goals

- not looking at other numerous Rust implementations until im done with first complete implementaton
- not using 3rd party linear algebra libraries until im done with first complete implementaton
- practice writting organizing rust modules, writing tests and using CI like it was a production ready tool

## Getting started

Run one of the examples

```sh
cargo run --release --example final_render
```

![](/doc/final_render_1600_1.jpeg)

## Todos

- [x] core
  - [x] vector math in 3D
  - [x] rays
    - [x] intersection with sphere
    - [x] antialiasing
    - [x] multiple bounce
    - [x] diffuse scattering
    - [x] reflection
    - [x] refraction
  - [x] scene with spheres
  - [x] adding sphere objects
  - [x] materials
  - [x] camera with arbitrary coordinate system
- [ ] infrastructure
  - [x] basic PPM image format writer
  - [x] loading bar
  - [x] custom buffer writter https://docs.rs/image/latest/image/fn.save_buffer.html
  - [ ] simple CLI to control output image parameters
  - [ ] scene builder config from json or yaml with `serde`
  - [ ] example to generate scene `json` file
- [x] examples
- [ ] performance
  - [x] profiling with perf and flamegraph
  - [x] benches for hot paths
  - [ ] parallelize the per pixel operations
