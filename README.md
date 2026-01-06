# sort-wasm

Reusable WASM package for dynamic sorting algorithms supporting numbers, strings, and objects.

## Features

- ⚡ High-performance sorting powered by WebAssembly
- 🔢 Sort numbers and strings
- 📦 Sort arrays of objects by column/property
- 🔧 Custom key function support for complex sorting logic
- ↕️ Ascending and descending order support

## Installation

```sh
npm install sort-wasm
```

## Building from Source

```sh
wasm-pack build --release --target bundler
```

## Usage

### Basic Setup

```js
import init, { 
  sort_numbers, 
  sort_strings, 
  sort_objects_by_column, 
  sort_objects_by_key_fn 
} from 'sort-wasm';

await init();
```

### Sort Numbers

```js
const numbers = [5, 2, 9, 1, 5, 6];

// Ascending
const ascending = sort_numbers(numbers, true);  // [1, 2, 5, 5, 6, 9]

// Descending
const descending = sort_numbers(numbers, false); // [9, 6, 5, 5, 2, 1]
```

### Sort Strings

```js
const strings = ['banana', 'apple', 'cherry'];

// Ascending
const ascending = sort_strings(strings, true);  // ['apple', 'banana', 'cherry']

// Descending
const descending = sort_strings(strings, false); // ['cherry', 'banana', 'apple']
```

### Sort Objects by Column

Sort an array of objects by a specific property/column name:

```js
const users = [
  { name: 'Alice', age: 30 },
  { name: 'Bob', age: 25 },
  { name: 'Charlie', age: 35 }
];

// Sort by age (ascending)
const byAge = sort_objects_by_column(users, 'age', true);
// [{ name: 'Bob', age: 25 }, { name: 'Alice', age: 30 }, { name: 'Charlie', age: 35 }]

// Sort by name (descending)
const byName = sort_objects_by_column(users, 'name', false);
// [{ name: 'Charlie', age: 35 }, { name: 'Bob', age: 25 }, { name: 'Alice', age: 30 }]
```

### Sort Objects with Custom Key Function

Use a custom function to extract or transform values before sorting:

```js
const products = [
  { name: 'Widget', price: 25.99, discount: 0.1 },
  { name: 'Gadget', price: 15.50, discount: 0.2 },
  { name: 'Gizmo', price: 35.00, discount: 0.05 }
];

// Sort by discounted price
const byDiscountedPrice = sort_objects_by_key_fn(
  products, 
  (item) => item.price * (1 - item.discount), 
  true
);

// Access nested properties
const data = [
  { user: { profile: { score: 100 } } },
  { user: { profile: { score: 50 } } },
  { user: { profile: { score: 75 } } }
];

const byNestedScore = sort_objects_by_key_fn(
  data, 
  (item) => item.user.profile.score, 
  true
);
```

## API Reference

### `sort_numbers(numbers: number[], ascending: boolean): number[]`

Sorts an array of numbers.

| Parameter | Type | Description |
|-----------|------|-------------|
| `numbers` | `number[]` | Array of numbers to sort |
| `ascending` | `boolean` | `true` for ascending, `false` for descending |

### `sort_strings(strings: string[], ascending: boolean): string[]`

Sorts an array of strings.

| Parameter | Type | Description |
|-----------|------|-------------|
| `strings` | `string[]` | Array of strings to sort |
| `ascending` | `boolean` | `true` for ascending, `false` for descending |

### `sort_objects_by_column(objects: object[], column: string, ascending: boolean): object[]`

Sorts an array of objects by a specific column/property. Automatically detects if the column contains numbers or strings.

| Parameter | Type | Description |
|-----------|------|-------------|
| `objects` | `object[]` | Array of objects to sort |
| `column` | `string` | Property name to sort by |
| `ascending` | `boolean` | `true` for ascending, `false` for descending |

### `sort_objects_by_key_fn(objects: object[], keyFn: Function, ascending: boolean): object[]`

Sorts an array of objects using a custom key function. The function receives each object and should return a value (number or string) to be used for comparison.

| Parameter | Type | Description |
|-----------|------|-------------|
| `objects` | `object[]` | Array of objects to sort |
| `keyFn` | `Function` | Function that takes an object and returns the sort key |
| `ascending` | `boolean` | `true` for ascending, `false` for descending |

## Publishing

```sh
wasm-pack publish
```

## License

MIT

