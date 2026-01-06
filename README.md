# sort-wasm

Reusable WASM package for dynamic sorting algorithms (numbers and strings).

## Usage

1. Build with wasm-pack:
   ```sh
   wasm-pack build --release --target bundler
   ```
2. Import in JS/TS:
   ```js
   import init, { sort_numbers, sort_strings } from 'sort-wasm';
   await init();
   const sorted = sort_numbers([3,1,2], true); // [1,2,3]
   ```

## API

- `sort_numbers(numbers: number[], ascending: boolean): number[]`
- `sort_strings(strings: string[], ascending: boolean): string[]`

## Publishing

- Use `wasm-pack publish` to publish to NPM.
- Add usage examples and TypeScript definitions for best developer experience.

## License
MIT

