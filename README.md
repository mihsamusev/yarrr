# YARRR - Yet Another Rust Raytracer Rewrite

[![](https://github.com/mihsamusev/yarrr/actions/workflows/build.yml/badge.svg)](https://github.com/mihsamusev/yarrr/actions/workflows/build.yml)

Study exercise in rewriting the famous [Ray Tracing in One Weekend by Peter Shirley](https://raytracing.github.io/books/RayTracingInOneWeekend.html#thevec3class/vec3utilityfunctions) from C++ to Rust to practice the language.

Goals:

- not looking at other numerous Rust implementations until im done with first complete implementaton
- not using 3rd party linear algebra libraries until im done with first complete implementaton
- practice writting organizing rust modules, writing tests and using CI like it was a production ready tool
- have fun and show it to my mum when im done

Progress:

- [x] Vector math in 3D
- [x] Basic PPM image format writer
- [ ] Rays
  - [x] intersection with sphere
  - [ ] bounce
- [x] Camera & Scene
- [x] Adding sphere objects

Milestone 1: 1 sphere with simple normal based lighting
![](doc/level1.ppm)

Todos:

- how to avoid mistakes where Vector3D / Point3D / Ray can be any float but color either 0.0-1.0 or 0-255 without rewritting too much code, reusing Vector3D
- how to make sure if `vec_a` _ b is implemented we get b _ `vec_a` automatically?
