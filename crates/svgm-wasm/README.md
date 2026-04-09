<p align="center">
  <br>
  <a href="https://github.com/madebyfrmwrk/svgm">
    <img alt="svgm" src="https://raw.githubusercontent.com/madebyfrmwrk/svgm/main/assets/svgm-light.svg" height="72">
  </a>
  <br>
  <br>
</p>

<p align="center">
  SVG optimization for the browser — WebAssembly build of <a href="https://github.com/madebyfrmwrk/svgm">svgm</a>.
</p>

<div align="center">

[![npm version][npm-badge]][npm-url]
[![MIT licensed][license-badge]][license-url]
[![Build Status][ci-badge]][ci-url]

</div>

---

## Install

```bash
npm install svgm-wasm
```

## Usage

```js
import init, { optimize, version } from 'svgm-wasm';

await init();

const result = optimize('<svg xmlns="http://www.w3.org/2000/svg">...</svg>');
console.log(result.data);       // optimized SVG string
console.log(result.iterations); // convergence count
console.log(version());         // e.g. "0.3.2"
```

### With options

```js
const result = optimize(svgString, {
  preset: 'safe',       // "safe" | "default"
  precision: 2,         // numeric precision (default: 3)
  passes: {
    removeDesc: true,    // enable opt-in passes
    mergePaths: false,   // disable specific passes
  },
});
```

## Presets

- **safe** — removal and normalization only (20 passes)
- **default** — full optimization (34 passes)

## Links

- [Website](https://svgm.dev)
- [Playground](https://svgm.dev/playground)
- [GitHub](https://github.com/madebyfrmwrk/svgm)
- [CLI on crates.io](https://crates.io/crates/svgm)
- [Node.js bindings](https://www.npmjs.com/package/svgm-node)

## License

Dual-licensed under [MIT](LICENSE-MIT) and [Apache 2.0](LICENSE-APACHE).

[npm-badge]: https://img.shields.io/npm/v/svgm-wasm.svg
[npm-url]: https://www.npmjs.com/package/svgm-wasm
[license-badge]: https://img.shields.io/badge/license-MIT%2FApache--2.0-blue.svg
[license-url]: https://github.com/madebyfrmwrk/svgm/blob/main/LICENSE-MIT
[ci-badge]: https://github.com/madebyfrmwrk/svgm/actions/workflows/ci.yml/badge.svg
[ci-url]: https://github.com/madebyfrmwrk/svgm/actions
