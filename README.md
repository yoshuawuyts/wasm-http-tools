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

| Status          | Crate                                              | Description                                                       |
| --------------- | -------------------------------------------------- | ----------------------------------------------------------------- |
| complete        | [wasy-async-runtime](./crates/wasy-async-runtime/) | WASI-based async runtime library for Rust                         |
| in-progress     | [openapi-bindgen](./crates/openapi-bindgen/)       | Generate bidirectional bindings between OpenAPI (Swagger) and WIT |
| in-progress     | [wasi-http-client](./crates/wasi-http-client/)     | WASI-based HTTP client library for Rust                           |
| not yet started | [wasm-http-tools](./crates/wasm-http-tools/)       | Rust tooling to use Wasm Components over HTTP                     |
| not yet started | [wit-bindgen-http](./crates/wit-bindgen-http/)     | Automatically generate HTTP routing layers for WIT definitions    |

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
