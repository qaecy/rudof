# @qaecy/rudof

Native Node.js bindings for [rudof](https://github.com/rudof-project/rudof) — an RDF, ShEx, SHACL, and SPARQL toolkit written in Rust.

> **Fork notice**: This package is built from [qaecy/rudof](https://github.com/qaecy/rudof),
> a fork of [rudof-project/rudof](https://github.com/rudof-project/rudof) at commit
> [`6e770ec`](https://github.com/rudof-project/rudof/commit/6e770ec6) (v0.3.1).
> The only addition is this `node/` directory with [napi-rs](https://napi.rs) bindings.

## Installation

```bash
npm install @qaecy/rudof
```

## Usage

```ts
import { Rudof, RudofConfig } from '@qaecy/rudof';

const config = new RudofConfig();
const rudof = new Rudof(config);

// Load RDF data (string, file path, or URL)
rudof.readData('<http://example.org/s> <http://example.org/p> <http://example.org/o> .');

// Load SHACL shapes
rudof.readShacl(shapesString);

// Validate
rudof.validateShacl();

// Get results as compact JSON
const results = rudof.serializeShaclValidationResults('Json');
console.log(results);
```

## API

All methods are **synchronous**. SPARQL endpoint queries block the calling thread —
offload with [`node:worker_threads`](https://nodejs.org/api/worker_threads.html) or a thread pool for use in async services.

See [`index.d.ts`](./index.d.ts) for the full TypeScript API.

## Building from source

Requires Rust (stable) and Node.js ≥ 18.

```bash
cd node
npm install
npm run build          # release build
npm run build:debug    # debug build
```

## Publishing

```bash
npm run publish:npm    # builds and publishes to npm as @qaecy/rudof
```

## License

MIT OR Apache-2.0 — same as the upstream rudof project.
