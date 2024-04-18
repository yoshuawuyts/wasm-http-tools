<h1 align="center">wasm-http-tools</h1>
<div align="center">
  <strong>
    Rust tooling to use <a href="https://github.com/webassembly/component-model">Wasm Components</a> over HTTP
  </strong>
</div>

<div align="center">
  <h3>
    <a href="https://yoshuawuyts.github.io/wasm-http-tools">
      Documentation
    </a>
    <span> | </span>
    <a href="https://github.com/yoshuawuyts/wasm-http-tools/releases">
      Releases
    </a>
    <span> | </span>
    <a href="https://github.com/yoshuawuyts/wasm-http-tools/blob/master.github/CONTRIBUTING.md">
      Contributing
    </a>
  </h3>
</div>

## Explainer

This crate contains tools to enable Wasm Components to be called over HTTP. This
should make it possible to convert WIT definitions into working HTTP APIs with
little effort. Think: automatic SDK generation, and less [remote
objects](https://en.wikipedia.org/wiki/Distributed_object). This means there
will be some limitations: the only functions we'll be able to map over HTTP will
be fallible, asynchronous functions. When executed remotely these functions 
reflect the realities of networking. But when executed locally, the asynchronous
fallible interface can always be optimized out.

This project is a first attempt at defining what a bidirectional mapping of WIT
and HTTP can look like. This is key to ensuring Wasm Components and WIT is not
just restricted to intra-machine communication, but also inter-machine
communication. If we have types on both sides, it should be able to generate the
protocol to communicate between those types automatically.

## Tools Included

| Status          | Crate                                          | Description                                                       |
| --------------- | ---------------------------------------------- | ----------------------------------------------------------------- |
| not yet started | [wasi-http-client](./crates/wasi-http-client/) | Generate structured HTTP types from IETF specifications           |
| in-progress     | [openapi-bindgen](./crates/openapi-bindgen/)   | Generate bidirectional bindings between OpenAPI (Swagger) and WIT |
| not yet started | [wasm-http-tools](./crates/wasm-http-tools/)   | Rust tooling to use Wasm Components over HTTP                     |
| not yet started | [wit-bindgen-http](./crates/wit-bindgen-http/) | Automatically generate HTTP routing layers for WIT definitions    |

## Contributing
Want to join us? Check out our ["Contributing" guide][contributing] and take a
look at some of these issues:

- [Issues labeled "good first issue"][good-first-issue]
- [Issues labeled "help wanted"][help-wanted]

[contributing]: https://github.com/yoshuawuyts/wasm-http-tools/blob/master.github/CONTRIBUTING.md
[good-first-issue]: https://github.com/yoshuawuyts/wasm-http-tools/labels/good%20first%20issue
[help-wanted]: https://github.com/yoshuawuyts/wasm-http-tools/labels/help%20wanted

## License

<sup>
Licensed under <a href="LICENSE">Apache-2.0 WITH LLVM-exception</a> 
</sup>

<br/>

<sub>
Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in this crate by you, as defined in the Apache-2.0 license with LLVM-exception,
shall be licensed as above, without any additional terms or conditions.
</sub>

| effect name | context    | lowering   | terminate  | forward  |
| ----------- | ---------- | ---------- | ---------- | -------- |
| fallibility | `try {}`   | `Try`      | `match`    | `?`      |
| asynchrony  | `async {}` | `Future`   | `block_on` | `.await` |
| iteration   | `gen {}`   | `Iterator` | `for..in`  | *n/a*    |

> Filling in the [schedule from the MCP](https://github.com/rust-lang/compiler-team/issues/695) gives:
 
 | date	Rust Stable	Rust Beta	Rust Nightly	Notes
 | -----------| ----| ----| ---| ---|
 | 2024-02-08	| 1.76	|         1.77	|         1.78	|         add support for `wasm32-wasi-preview1` (1, 2) |
 | 2024-03-21	| 1.77	|         1.78	|         1.79	|
 | 2024-05-02	| 1.78	|         1.79	|         1.80	|
 | 2024-06-13	| 1.79	|         1.80	|         1.81	|         warn on `wasm32-wasi` (4)
 | 2024-07-25	| 1.80	|         1.81	|         1.82	|
 | 2024-09-05	| 1.81	|         1.82	|         1.83	|
 | 2024-10-17	| 1.82	|         1.83	|         1.84	|         remove `wasm32-wasi` (5)
 | 2024-11-28	| 1.83	|         1.84	|         1.85	|
 | 2025-01-09	| 1.84	|         1.85	|         1.86	|         `wasm32-wasi` is now gone on all release channels
