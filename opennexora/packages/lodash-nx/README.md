# @opennexora/lodash-nx

A comprehensive utility library for the Nexora programming language, inspired by lodash but optimized for Nexora's type system and performance characteristics.

## Installation

```bash
npm install @opennexora/lodash-nx
# or
nxm add @opennexora/lodash-nx
```

## Features

- **Type-safe**: Full Nexora type definitions for all functions
- **Tree-shakeable**: Import only what you need
- **Performance optimized**: Written in pure Nexora for maximum performance
- **Functional**: Immutable operations and pure functions
- **Comprehensive**: 200+ utility functions

## Quick Start

```nx
import { chunk, flatten, uniq } from '@opennexora/lodash-nx';

// Chunk an array into groups
let numbers = [1, 2, 3, 4, 5, 6, 7, 8];
let chunks = chunk(numbers, 3);
// Result: [[1, 2, 3], [4, 5, 6], [7, 8]]

// Flatten nested arrays
let nested = [[1, 2], [3, 4], [5, 6]];
let flat = flatten(nested);
// Result: [1, 2, 3, 4, 5, 6]

// Remove duplicates
let withDuplicates = [1, 2, 2, 3, 3, 3, 4];
let unique = uniq(withDuplicates);
// Result: [1, 2, 3, 4]
```

## API Reference

### Array Functions

#### `chunk(array: Array<T>, size: Number) -> Array<Array<T>>`
Creates an array of elements split into groups the length of size.

```nx
let result = chunk(['a', 'b', 'c', 'd', 'e'], 2);
// Result: [['a', 'b'], ['c', 'd'], ['e']]
```

#### `compact(array: Array<T>) -> Array<T>`
Creates an array with all falsy values removed.

```nx
let result = compact([0, 1, false, 2, '', 3, null, undefined, NaN]);
// Result: [1, 2, 3]
```

#### `flatten(array: Array<T|Array<T>>) -> Array<T>`
Flattens array a single level deep.

```nx
let result = flatten([1, [2, [3, [4]], 5]]);
// Result: [1, 2, [3, [4]], 5]
```

#### `flattenDeep(array: Array<any>) -> Array<any>`
Recursively flattens array.

```nx
let result = flattenDeep([1, [2, [3, [4]], 5]]);
// Result: [1, 2, 3, 4, 5]
```

#### `uniq(array: Array<T>) -> Array<T>`
Creates a duplicate-free version of an array.

```nx
let result = uniq([1, 2, 1, 4, 1, 3]);
// Result: [1, 2, 4, 3]
```

### Collection Functions

#### `filter(collection: Array<T>, predicate: Function) -> Array<T>`
Iterates over elements of collection, returning an array of all elements predicate returns truthy for.

```nx
let users = [
  { name: 'Alice', age: 25 },
  { name: 'Bob', age: 30 },
  { name: 'Charlie', age: 35 }
];

let result = filter(users, (user) => user.age > 28);
// Result: [{ name: 'Bob', age: 30 }, { name: 'Charlie', age: 35 }]
```

#### `map(collection: Array<T>, iteratee: Function) -> Array<U>`
Creates an array of values by running each element through iteratee.

```nx
let numbers = [1, 2, 3, 4];
let result = map(numbers, (n) => n * 2);
// Result: [2, 4, 6, 8]
```

#### `reduce(collection: Array<T>, iteratee: Function, accumulator: U) -> U`
Reduces collection to a value which is the accumulated result of running each element through iteratee.

```nx
let numbers = [1, 2, 3, 4];
let result = reduce(numbers, (sum, n) => sum + n, 0);
// Result: 10
```

### Object Functions

#### `pick(object: Object, paths: Array<String>) -> Object`
Creates an object composed of the picked properties.

```nx
let user = { name: 'Alice', age: 25, email: 'alice@example.com' };
let result = pick(user, ['name', 'email']);
// Result: { name: 'Alice', email: 'alice@example.com' }
```

#### `omit(object: Object, paths: Array<String>) -> Object`
Creates an object composed of the omitted properties.

```nx
let user = { name: 'Alice', age: 25, email: 'alice@example.com' };
let result = omit(user, ['age']);
// Result: { name: 'Alice', email: 'alice@example.com' }
```

#### `merge(object: Object, sources: Object...) -> Object`
This method recursively merges own and inherited enumerable string keyed properties of source objects into the destination object.

```nx
let defaults = { color: 'red', size: 'medium' };
let custom = { color: 'blue' };
let result = merge({}, defaults, custom);
// Result: { color: 'blue', size: 'medium' }
```

### String Functions

#### `capitalize(string: String) -> String`
Converts the first character of string to upper case and the remaining to lower case.

```nx
let result = capitalize('HELLO');
// Result: 'Hello'
```

#### `camelCase(string: String) -> String`
Converts string to camelCase.

```nx
let result = camelCase('Foo Bar');
// Result: 'fooBar'
```

#### `kebabCase(string: String) -> String`
Converts string to kebab-case.

```nx
let result = kebabCase('Foo Bar');
// Result: 'foo-bar'
```

### Function Functions

#### `debounce(func: Function, wait: Number) -> Function`
Creates a debounced version of the function.

```nx
let debounceSearch = debounce((query) => {
  console.log('Searching for:', query);
}, 300);

// Call multiple times
debounceSearch('a');
debounceSearch('ab');
debounceSearch('abc');
// Only the last call executes after 300ms
```

#### `throttle(func: Function, limit: Number) -> Function`
Creates a throttled version of the function.

```nx
let throttleScroll = throttle((event) => {
  console.log('Scroll position:', event.target.scrollTop);
}, 100);

window.addEventListener('scroll', throttleScroll);
```

## Type Safety

All functions are fully typed with Nexora's type system:

```nx
import { map, filter } from '@opennexora/lodash-nx';

interface User {
  id: Number;
  name: String;
  email: String;
  age: Number;
}

let users: Array<User> = [
  { id: 1, name: 'Alice', email: 'alice@example.com', age: 25 },
  { id: 2, name: 'Bob', email: 'bob@example.com', age: 30 }
];

// TypeScript knows the return type
let names: Array<String> = map(users, (user) => user.name);
// Result: ['Alice', 'Bob']

let adults: Array<User> = filter(users, (user) => user.age >= 18);
// Result: all users
```

## Performance

lodash-nx is optimized for Nexora's runtime:

- **Lazy evaluation**: Operations are only computed when needed
- **Short-circuit evaluation**: Stops processing when result is determined
- **Memoization**: Caches expensive computations
- **Batch processing**: Processes large datasets in chunks

```nx
import { chain } from '@opennexora/lodash-nx';

// Lazy evaluation - only processes until condition is met
let result = chain([1, 2, 3, 4, 5, 6, 7, 8, 9, 10])
  .filter((n) => n > 3)
  .map((n) => n * 2)
  .take(3)
  .value();
// Result: [8, 10, 12]
// Only processes first 4 elements that satisfy filter
```

## Migration from lodash

If you're migrating from JavaScript's lodash:

1. Replace `require` with `import` statements
2. Update function names to match Nexora conventions
3. Add type annotations where needed
4. Remove any polyfills (Nexora has built-in support)

```nx
// Before (JavaScript)
const _ = require('lodash');
const result = _.map([1, 2, 3], function(n) {
  return n * 2;
});

// After (Nexora)
import { map } from '@opennexora/lodash-nx';
let result = map([1, 2, 3], (n) => n * 2);
```

## Contributing

We welcome contributions! Please see [CONTRIBUTING.md](../../CONTRIBUTING.md) for guidelines.

## License

MIT © OpenNexora Foundation