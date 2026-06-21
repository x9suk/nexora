# lodash-nx

Utility library for the Nexora runtime. Provides common operations for arrays, objects, functions, and strings.

## Installation

```bash
nxm install lodash-nx
```

## Usage

```nx
import { map, filter, debounce, merge } from "lodash-nx";

const doubled = map([1, 2, 3], (n) => n * 2);
// [2, 4, 6]

const evens = filter([1, 2, 3, 4], (n) => n % 2 === 0);
// [2, 4]
```

## Array Functions

### `map(arr, fn)`

Creates a new array by applying a function to each element.

```nx
map(["a", "b", "c"], (s) => s.toUpperCase());
// ["A", "B", "C"]
```

### `filter(arr, fn)`

Creates a new array with elements that pass a test.

```nx
filter([1, 2, 3, 4, 5], (n) => n > 3);
// [4, 5]
```

### `reduce(arr, fn, initial)`

Reduces an array to a single value.

```nx
reduce([1, 2, 3], (acc, n) => acc + n, 0);
// 6
```

### `find(arr, fn)`

Returns the first element that passes a test.

```nx
find([1, 2, 3], (n) => n > 1);
// 2
```

### `every(arr, fn)`

Returns `true` if all elements pass a test.

```nx
every([2, 4, 6], (n) => n % 2 === 0);
// true
```

### `some(arr, fn)`

Returns `true` if any element passes a test.

```nx
some([1, 2, 3], (n) => n > 2);
// true
```

### `flatten(arr)`

Flattens a nested array by one level.

```nx
flatten([[1, 2], [3, [4]]]);
// [1, 2, 3, [4]]
```

### `flatMap(arr, fn)`

Maps and then flattens by one level.

```nx
flatMap([[1, 2], [3, 4]], (arr) => arr.map((n) => n * 2));
// [2, 4, 6, 8]
```

### `unique(arr)`

Returns unique values.

```nx
unique([1, 2, 2, 3, 3, 3]);
// [1, 2, 3]
```

### `chunk(arr, size)`

Splits an array into chunks.

```nx
chunk([1, 2, 3, 4, 5], 2);
// [[1, 2], [3, 4], [5]]
```

### `range(start, end, step)`

Creates an array of numbers.

```nx
range(5);
// [0, 1, 2, 3, 4]

range(1, 10, 2);
// [1, 3, 5, 7, 9]
```

### `sortBy(arr, fn)`

Returns a sorted copy of the array.

```nx
sortBy([{ a: 2 }, { a: 1 }, { a: 3 }], "a");
// [{ a: 1 }, { a: 2 }, { a: 3 }]

sortBy([3, 1, 2], (n) => n);
// [1, 2, 3]
```

### `groupBy(arr, fn)`

Groups elements by a key.

```nx
groupBy(["one", "two", "three"], (s) => s.length);
// { 3: ["one", "two"], 5: ["three"] }
```

## Object Functions

### `pick(obj, keys)`

Picks specified keys from an object.

```nx
pick({ a: 1, b: 2, c: 3 }, ["a", "c"]);
// { a: 1, c: 3 }
```

### `omit(obj, keys)`

Omits specified keys from an object.

```nx
omit({ a: 1, b: 2, c: 3 }, ["b"]);
// { a: 1, c: 3 }
```

### `clone(obj)`

Deep clones an object.

```nx
const original = { a: { b: 1 } };
const copy = clone(original);
copy.a.b = 2;
original.a.b; // 1
```

### `merge(target, ...sources)`

Deep merges objects.

```nx
merge({ a: 1 }, { b: 2 }, { a: 3 });
// { a: 3, b: 2 }
```

### `keys(obj)` / `values(obj)` / `entries(obj)`

Object iteration helpers.

```nx
keys({ a: 1, b: 2 });
// ["a", "b"]
```

### `fromEntries(entries)`

Creates an object from key-value pairs.

```nx
fromEntries([["a", 1], ["b", 2]]);
// { a: 1, b: 2 }
```

### `isEmpty(obj)`

Checks if a value is empty.

```nx
isEmpty(null);     // true
isEmpty([]);       // true
isEmpty({});       // true
isEmpty("hello");  // false
```

### `deepEqual(a, b)`

Deep equality check.

```nx
deepEqual({ a: [1] }, { a: [1] });
// true
```

## Function Utilities

### `debounce(fn, delay)`

Delays function execution until after `delay` ms of inactivity.

```nx
const search = debounce((query) => {
  console.log("Searching:", query);
}, 300);

input.on("input", (e) => search(e.target.value));
```

### `throttle(fn, limit)`

Limits function calls to once per `limit` ms.

```nx
const onScroll = throttle(() => {
  console.log("Scrolled");
}, 100);

window.on("scroll", onScroll);
```

### `memoize(fn)`

Caches function results by arguments.

```nx
const expensive = memoize((n) => {
  console.log("Computing...");
  return n * n;
});

expensive(4); // Computing... 16
expensive(4); // 16 (cached)
```

### `once(fn)`

Ensures a function is called only once.

```nx
const init = once(() => {
  console.log("Initialized");
});

init(); // Initialized
init(); // (no output)
```

### `curry(fn)`

Converts a function to curried form.

```nx
const add = curry((a, b) => a + b);
const add5 = add(5);
add5(3); // 8
```

### `pipe(...fns)`

Composes functions left to right.

```nx
const process = pipe(
  (s) => s.trim(),
  (s) => s.toLowerCase(),
  (s) => s.replace(/\s+/g, "-")
);

process("  Hello World  ");
// "hello-world"
```

### `compose(...fns)`

Composes functions right to left.

## String Utilities

### `capitalize(str)`

Capitalizes the first letter.

```nx
capitalize("hello"); // "Hello"
```

### `camelCase(str)`

Converts to camelCase.

```nx
camelCase("hello_world"); // "helloWorld"
```

### `kebabCase(str)`

Converts to kebab-case.

```nx
kebabCase("helloWorld"); // "hello-world"
```

### `snakeCase(str)`

Converts to snake_case.

```nx
snakeCase("helloWorld"); // "hello_world"
```

### `truncate(str, length, suffix)`

Truncates a string.

```nx
truncate("Hello World", 5);       // "Hello..."
truncate("Hello World", 5, "…");  // "Hello…"
```

## License

MIT
