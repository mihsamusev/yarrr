# YARRR - Yet Another Rust Raytracer Rewrite

[![](https://github.com/mihsamusev/yarrr/actions/workflows/build.yml/badge.svg)](https://github.com/mihsamusev/yarrr/actions/workflows/build.yml)

Study exercise in rewriting the famous [Ray Tracing in One Weekend by Peter Shirley](https://raytracing.github.io/books/RayTracingInOneWeekend.html#thevec3class/vec3utilityfunctions) from C++ to Rust to practice the language.

Goals:

- not looking at other numerous Rust implementations until im done with first complete implementaton
- not using 3rd party linear algebra libraries until im done with first complete implementaton
- practice writting organizing rust modules, writing tests and using CI like it was a production ready tool
- have fun and show it to my mum when im done

Progress:

![](/doc/final_render_1200.jpeg)

- [x] Vector math in 3D
- [x] Basic PPM image format writer
- [x] Rays
  - [x] intersection with sphere
  - [x] antialiasing
  - [x] multiple bounce
  - [x] diffuse scattering
  - [x] reflection
  - [x] refraction
- [x] Scene with spheres
- [x] Adding sphere objects
- [x] Materials
- [x] Camera with arbitrary coordinate system
- [ ] Scene builder config from json or yaml with `serde`
  - [ ] example to generate scene `yaml` file
- [ ] refactor unit tests from `tests` to coresponding modules
- [ ] performance optimization
  - [ ] benches for hot paths
  - [ ] profiling with perf and flamegraph
  - [ ] figure out the artifact
