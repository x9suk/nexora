# Standard Library

## Table of Contents

- [Core Functions](#core-functions)
- [Math Functions](#math-functions)
- [String Functions](#string-functions)
- [Collection Functions](#collection-functions)
- [Object Functions](#object-functions)
- [Type Functions](#type-functions)
- [Date/Time Functions](#datetime-functions)

## Core Functions

| Function | Description | Example |
|----------|-------------|---------|
| `print(...)` | Print values to stdout | `print "Hello"` |
| `typeof(value)` | Get type name | `typeof(42)` → `"integer"` |
| `len(value)` | Get length | `len("hello")` → `5` |
| `str(value)` | Convert to string | `str(42)` → `"42"` |
| `num(value)` | Convert to number | `num("42")` → `42` |
| `int(value)` | Convert to integer | `int(3.7)` → `3` |
| `float(value)` | Convert to float | `float(42)` → `42.0` |
| `parseInt(value)` | Parse string as integer | `parseInt("42")` → `42` |
| `parseFloat(value)` | Parse string as float | `parseFloat("3.14")` → `3.14` |
| `assert(condition, msg)` | Assert condition | `assert(1+1==2, "Math broken")` |

## Math Functions

### Global Functions

| Function | Description | Example |
|----------|-------------|---------|
| `sqrt(x)` | Square root | `sqrt(16)` → `4` |
| `pow(base, exp)` | Power | `pow(2, 10)` → `1024` |
| `abs(x)` | Absolute value | `abs(-5)` → `5` |
| `floor(x)` | Round down | `floor(3.7)` → `3` |
| `ceil(x)` | Round up | `ceil(3.2)` → `4` |
| `round(x)` | Round to nearest | `round(3.5)` → `4` |
| `min(a, b)` | Minimum | `min(3, 7)` → `3` |
| `max(a, b)` | Maximum | `max(3, 7)` → `7` |
| `random()` | Random float [0, 1) | `random()` → `0.5234` |

### Math Object

```nexora
print Math.PI  // 3.14159265358979
print Math.E   // 2.71828182845904
```

### Advanced Math (from lib/math.nx)

```nexora
import { sin, cos, tan, log, ln } from "math"

print sin(3.14159 / 2)  // ~1
print cos(0)             // ~1
print log(2.71828)       // ~1
```

## String Functions

| Function | Description | Example |
|----------|-------------|---------|
| `split(str, delim)` | Split string | `split("a,b,c", ",")` → `["a","b","c"]` |
| `join(arr, delim)` | Join array | `join(["a","b"], "-")` → `"a-b"` |
| `upper(str)` | Uppercase | `upper("hello")` → `"HELLO"` |
| `lower(str)` | Lowercase | `lower("HELLO")` → `"hello"` |
| `trim(str)` | Trim whitespace | `trim(" hello ")` → `"hello"` |
| `contains(str, sub)` | Check substring | `contains("hello", "ell")` → `true` |
| `replace(str, old, new)` | Replace | `replace("hello", "l", "r")` → `"herro"` |
| `starts_with(str, prefix)` | Check prefix | `starts_with("hello", "hel")` → `true` |
| `ends_with(str, suffix)` | Check suffix | `ends_with("hello", "llo")` → `true` |
| `char_at(str, i)` | Get character | `char_at("hello", 1)` → `"e"` |
| `to_chars(str)` | Convert to array | `to_chars("abc")` → `["a","b","c"]` |
| `repeat(str, n)` | Repeat string | `repeat("ha", 3)` → `"hahaha"` |
| `slice(str, start, end)` | Substring | `slice("hello", 1, 3)` → `"el"` |
| `index_of(str, sub)` | Find index | `index_of("hello", "l")` → `2` |

### String Methods

```nexora
let s = "Hello, World!"

s.length         // 13
s.toUpperCase()  // "HELLO, WORLD!"
s.toLowerCase()  // "hello, world!"
s.trim()         // "Hello, World!"
s.includes("World")    // true
s.startsWith("Hello")  // true
s.endsWith("!")        // true
s.indexOf("World")     // 7
s.charAt(0)            // "H"
```

### Advanced String Functions (from lib/string.nx)

```nexora
import { pad_left, pad_right, center, count, is_alpha, is_digit } from "string"

pad_left("42", 5, "0")  // "00042"
pad_right("hi", 5, ".")  // "hi..."
center("hi", 5, "-")     // "-hi--"
count("hello", "l")      // 2
is_alpha("hello")        // true
is_digit("123")          // true
```

## Collection Functions

| Function | Description | Example |
|----------|-------------|---------|
| `push(arr, item)` | Add to end | `push([1,2], 3)` → `[1,2,3]` |
| `pop(arr)` | Remove last | `pop([1,2,3])` → `3` |
| `sort(arr)` | Sort array | `sort([3,1,2])` → `[1,2,3]` |
| `reverse(arr)` | Reverse | `reverse([1,2,3])` → `[3,2,1]` |
| `unique(arr)` | Remove dupes | `unique([1,1,2])` → `[1,2]` |
| `flatten(arr)` | Flatten | `flatten([[1,2],[3]])` → `[1,2,3]` |
| `range(start, end)` | Number range | `range(1, 5)` → `[1,2,3,4]` |
| `zip(arr1, arr2)` | Combine | `zip([1,2], ["a","b"])` → `[[1,"a"],[2,"b"]]` |
| `map(arr, fn)` | Transform | `map([1,2], x => x*2)` → `[2,4]` |
| `filter(arr, fn)` | Select | `filter([1,2,3], x => x>1)` → `[2,3]` |
| `reduce(arr, fn, init)` | Accumulate | `reduce([1,2,3], (a,x) => a+x, 0)` → `6` |
| `find(arr, fn)` | Find first | `find([1,2,3], x => x>1)` → `2` |
| `find_index(arr, fn)` | Find index | `find_index([1,2,3], x => x>1)` → `1` |
| `every(arr, fn)` | Test all | `every([2,4], x => x%2==0)` → `true` |
| `some(arr, fn)` | Test any | `some([1,2], x => x>1)` → `true` |
| `includes(arr, val)` | Contains | `includes([1,2], 2)` → `true` |
| `index_of(arr, val)` | Find index | `index_of([1,2], 2)` → `1` |
| `last_index_of(arr, val)` | Last index | `last_index_of([1,2,1], 1)` → `2` |
| `concat(arr1, arr2)` | Merge | `concat([1,2], [3,4])` → `[1,2,3,4]` |
| `flat_map(arr, fn)` | Map+flatten | `flat_map([1,2], x => [x,x*2])` → `[1,2,2,4]` |
| `group_by(arr, fn)` | Group | `group_by([1,2,3], x => x%2==0?"e":"o")` → `{e:[2],o:[1,3]}` |
| `count_by(arr, fn)` | Count by | `count_by([1,2,3], x => x%2==0?"e":"o")` → `{e:1,o:2}` |
| `partition(arr, fn)` | Split | `partition([1,2,3], x => x>1)` → `[[2,3],[1]]` |
| `shuffle(arr)` | Randomize | `shuffle([1,2,3])` → `[2,1,3]` (random) |
| `sample(arr, n)` | Random n | `sample([1,2,3,4], 2)` → random 2 items |
| `sum(arr)` | Sum all | `sum([1,2,3])` → `6` |
| `product(arr)` | Multiply all | `product([2,3,4])` → `24` |
| `min(arr)` | Minimum | `min([3,1,2])` → `1` |
| `max(arr)` | Maximum | `max([3,1,2])` → `3` |
| `average(arr)` | Average | `average([1,2,3])` → `2` |

## Object Functions

| Function | Description | Example |
|----------|-------------|---------|
| `keys(obj)` | Get keys | `keys({a:1})` → `["a"]` |
| `values(obj)` | Get values | `values({a:1})` → `[1]` |
| `entries(obj)` | Key-value pairs | `entries({a:1})` → `[["a",1]]` |
| `exists(obj, key)` | Check key | `exists({a:1}, "a")` → `true` |

### Object Methods

```nexora
let obj = { name: "Nexora", version: 4 }
print obj.name      // "Nexora"
print obj.version   // 4

obj.newKey = "value"  // Add property
```

## Type Functions

| Function | Description | Example |
|----------|-------------|---------|
| `typeof(value)` | Type name | `typeof(42)` → `"integer"` |
| `type_of(value)` | Type name (alias) | `type_of(42)` → `"integer"` |
| `str(value)` | To string | `str(42)` → `"42"` |
| `num(value)` | To number | `num("42")` → `42` |
| `int(value)` | To integer | `int(3.7)` → `3` |
| `float(value)` | To float | `float(42)` → `42.0` |

### Type Names

```
"integer"     - int values
"float"       - float values
"string"      - string values
"boolean"     - true/false
"null"        - null
"array"       - arrays
"object"      - objects
"function"    - user functions
"native_function" - built-in functions
"class"       - class definitions
"instance"    - class instances
"closure"     - closures
```

## Date/Time Functions

| Function | Description | Example |
|----------|-------------|---------|
| `now()` | Current timestamp | `now()` → `1705312345` |
| `timestamp()` | Precise timestamp | `timestamp()` → `1705312345678` |

### Example

```nexora
let current = now()
print "Unix timestamp: " + str(current)

let ts = timestamp()
print "Precise time: " + str(ts)
```
