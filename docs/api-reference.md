# API Reference

## Table of Contents

- [Core Functions](#core-functions)
- [Math Functions](#math-functions)
- [String Functions](#string-functions)
- [Array Functions](#array-functions)
- [Object Functions](#object-functions)
- [Type Conversion](#type-conversion)
- [Date/Time Functions](#datetime-functions)

## Core Functions

### print

Print values to stdout.

```nexora
print "Hello, World!"
print "Name:", "Nexora", "Version:", 4
```

**Parameters:** `...values` — Any number of values to print
**Returns:** `null`

### typeof

Get the type name of a value.

```nexora
print typeof(42)        // "integer"
print typeof(3.14)      // "float"
print typeof("hello")   // "string"
print typeof(true)      // "boolean"
print typeof(null)      // "null"
print typeof([1, 2])    // "array"
print typeof({a: 1})    // "object"
print typeof(func() {})  // "function"
```

**Parameters:** `value` — Any value
**Returns:** `string` — Type name

### len

Get the length of a string, array, or object.

```nexora
print len("hello")      // 5
print len([1, 2, 3])    // 3
print len({a: 1, b: 2}) // 2
```

**Parameters:** `value` — String, array, or object
**Returns:** `integer`

### assert

Assert that a condition is true.

```nexora
assert(1 + 1 == 2, "Math is broken!")
assert("hello" == "hello", "String equality broken!")
```

**Parameters:**
- `condition` — Boolean condition to check
- `message` (optional) — Error message if assertion fails

**Returns:** `null`
**Throws:** Error if condition is false

## Math Functions

### sqrt

Square root of a number.

```nexora
print sqrt(16)    // 4
print sqrt(2)     // 1.4142135623730951
```

**Parameters:** `x` — Number
**Returns:** `float`

### pow

Raise base to the power of exponent.

```nexora
print pow(2, 10)   // 1024
print pow(3, 3)    // 27
```

**Parameters:**
- `base` — Number
- `exp` — Number

**Returns:** `float`

### abs

Absolute value.

```nexora
print abs(-5)      // 5
print abs(5)       // 5
```

**Parameters:** `x` — Number
**Returns:** `integer` or `float`

### floor

Round down to nearest integer.

```nexora
print floor(3.7)   // 3
print floor(3.2)   // 3
```

**Parameters:** `x` — Number
**Returns:** `integer`

### ceil

Round up to nearest integer.

```nexora
print ceil(3.2)    // 4
print ceil(3.7)    // 4
```

**Parameters:** `x` — Number
**Returns:** `integer`

### round

Round to nearest integer.

```nexora
print round(3.5)   // 4
print round(3.4)   // 3
```

**Parameters:** `x` — Number
**Returns:** `integer`

### min

Return the smaller of two numbers.

```nexora
print min(3, 7)    // 3
print min(7, 3)    // 3
```

**Parameters:**
- `a` — Number
- `b` — Number

**Returns:** `number`

### max

Return the larger of two numbers.

```nexora
print max(3, 7)    // 7
print max(7, 3)    // 7
```

**Parameters:**
- `a` — Number
- `b` — Number

**Returns:** `number`

### random

Generate a random float between 0 (inclusive) and 1 (exclusive).

```nexora
print random()     // 0.523416...
```

**Parameters:** None
**Returns:** `float`

### Math.PI

The mathematical constant pi.

```nexora
print Math.PI      // 3.14159265358979
```

### Math.E

The mathematical constant e.

```nexora
print Math.E       // 2.71828182845904
```

## String Functions

### split

Split a string by delimiter.

```nexora
let parts = split("a,b,c", ",")
print parts         // ["a", "b", "c"]

let words = split("Hello World", " ")
print words         // ["Hello", "World"]
```

**Parameters:**
- `str` — String to split
- `delimiter` — Delimiter string

**Returns:** `array`

### join

Join an array into a string.

```nexora
let result = join(["a", "b", "c"], "-")
print result         // "a-b-c"

let sentence = join(["Hello", "World"], " ")
print sentence       // "Hello World"
```

**Parameters:**
- `arr` — Array to join
- `separator` — Separator string

**Returns:** `string`

### upper

Convert string to uppercase.

```nexora
print upper("hello")    // "HELLO"
```

**Parameters:** `str` — String
**Returns:** `string`

### lower

Convert string to lowercase.

```nexora
print lower("HELLO")    // "hello"
```

**Parameters:** `str` — String
**Returns:** `string`

### trim

Remove whitespace from both ends.

```nexora
print trim("  hello  ")    // "hello"
```

**Parameters:** `str` — String
**Returns:** `string`

### contains

Check if string contains substring.

```nexora
print contains("hello world", "world")    // true
print contains("hello", "xyz")            // false
```

**Parameters:**
- `str` — String to search in
- `substr` — Substring to search for

**Returns:** `boolean`

### replace

Replace occurrences of a substring.

```nexora
print replace("hello world", "world", "nexora")    // "hello nexora"
```

**Parameters:**
- `str` — Original string
- `old` — Substring to replace
- `new` — Replacement string

**Returns:** `string`

### starts_with

Check if string starts with prefix.

```nexora
print starts_with("hello", "hel")    // true
print starts_with("hello", "xyz")    // false
```

**Parameters:**
- `str` — String
- `prefix` — Prefix to check

**Returns:** `boolean`

### ends_with

Check if string ends with suffix.

```nexora
print ends_with("hello", "llo")    // true
print ends_with("hello", "xyz")    // false
```

**Parameters:**
- `str` — String
- `suffix` — Suffix to check

**Returns:** `boolean`

### char_at

Get character at index.

```nexora
print char_at("hello", 0)    // "h"
print char_at("hello", 4)    // "o"
```

**Parameters:**
- `str` — String
- `index` — Integer index

**Returns:** `string`

### index_of

Find first occurrence of substring.

```nexora
print index_of("hello world", "world")    // 6
print index_of("hello", "xyz")            // -1
```

**Parameters:**
- `str` — String to search
- `substr` — Substring to find

**Returns:** `integer` (-1 if not found)

### slice

Extract a substring.

```nexora
print slice("hello", 1, 3)    // "el"
print slice("hello", 0, 5)    // "hello"
```

**Parameters:**
- `str` — String
- `start` — Start index (inclusive)
- `end` — End index (exclusive)

**Returns:** `string`

### repeat

Repeat a string n times.

```nexora
print repeat("ha", 3)    // "hahaha"
```

**Parameters:**
- `str` — String to repeat
- `n` — Number of times

**Returns:** `string`

### to_chars

Convert string to array of characters.

```nexora
print to_chars("abc")    // ["a", "b", "c"]
```

**Parameters:** `str` — String
**Returns:** `array`

## Array Functions

### push

Add element(s) to end of array (returns new array).

```nexora
let arr = [1, 2]
let extended = push(arr, 3)
print extended    // [1, 2, 3]
```

**Parameters:**
- `arr` — Array
- `...items` — Items to add

**Returns:** `array`

### pop

Remove and return last element.

```nexora
let arr = [1, 2, 3]
let item = pop(arr)
print item    // 3
```

**Parameters:** `arr` — Array
**Returns:** Last element or `null` if empty

### sort

Sort array elements.

```nexora
print sort([3, 1, 4, 1, 5])    // [1, 1, 3, 4, 5]
```

**Parameters:** `arr` — Array
**Returns:** `array`

### reverse

Reverse array order.

```nexora
print reverse([1, 2, 3])    // [3, 2, 1]
```

**Parameters:** `arr` — Array
**Returns:** `array`

### unique

Remove duplicate elements.

```nexora
print unique([1, 1, 2, 2, 3])    // [1, 2, 3]
```

**Parameters:** `arr` — Array
**Returns:** `array`

### flatten

Flatten nested arrays.

```nexora
print flatten([[1, 2], [3, 4]])    // [1, 2, 3, 4]
```

**Parameters:** `arr` — Nested array
**Returns:** `array`

### range

Generate a range of numbers.

```nexora
print range(1, 5)      // [1, 2, 3, 4]
print range(0, 10, 2)  // [0, 2, 4, 6, 8]
```

**Parameters:**
- `start` — Start value
- `end` — End value (exclusive)
- `step` (optional) — Step size (default: 1)

**Returns:** `array`

### zip

Combine multiple arrays.

```nexora
print zip([1, 2], ["a", "b"])    // [[1, "a"], [2, "b"]]
```

**Parameters:** `...arrays` — Arrays to combine
**Returns:** `array`

### map

Transform each element.

```nexora
let doubled = map([1, 2, 3], x => x * 2)
print doubled    // [2, 4, 6]
```

**Parameters:**
- `arr` — Source array
- `fn` — Transform function

**Returns:** `array`

### filter

Select matching elements.

```nexora
let evens = filter([1, 2, 3, 4], x => x % 2 == 0)
print evens    // [2, 4]
```

**Parameters:**
- `arr` — Source array
- `fn` — Filter function (returns boolean)

**Returns:** `array`

### reduce

Accumulate into single value.

```nexora
let sum = reduce([1, 2, 3], (acc, x) => acc + x, 0)
print sum    // 6
```

**Parameters:**
- `arr` — Source array
- `fn` — Reduce function
- `init` — Initial value

**Returns:** Accumulated value

### find

Find first matching element.

```nexora
let found = find([1, 2, 3, 4], x => x > 2)
print found    // 3
```

**Parameters:**
- `arr` — Source array
- `fn` — Predicate function

**Returns:** Matching element or `null`

### find_index

Find index of first matching element.

```nexora
let idx = find_index([1, 2, 3, 4], x => x > 2)
print idx    // 2
```

**Parameters:**
- `arr` — Source array
- `fn` — Predicate function

**Returns:** `integer` (-1 if not found)

### every

Test if all elements match.

```nexora
print every([2, 4, 6], x => x % 2 == 0)    // true
print every([2, 3, 6], x => x % 2 == 0)    // false
```

**Parameters:**
- `arr` — Source array
- `fn` — Predicate function

**Returns:** `boolean`

### some

Test if any element matches.

```nexora
print some([1, 2, 3], x => x > 2)    // true
print some([1, 2, 3], x => x > 5)    // false
```

**Parameters:**
- `arr` — Source array
- `fn` — Predicate function

**Returns:** `boolean`

### includes

Check if array contains value.

```nexora
print includes([1, 2, 3], 2)    // true
print includes([1, 2, 3], 5)    // false
```

**Parameters:**
- `arr` — Array
- `val` — Value to check

**Returns:** `boolean`

### index_of

Find index of value.

```nexora
print index_of([1, 2, 3], 2)    // 1
print index_of([1, 2, 3], 5)    // -1
```

**Parameters:**
- `arr` — Array
- `val` — Value to find

**Returns:** `integer` (-1 if not found)

### sum

Sum all elements.

```nexora
print sum([1, 2, 3, 4, 5])    // 15
```

**Parameters:** `arr` — Array of numbers
**Returns:** `number`

### product

Multiply all elements.

```nexora
print product([2, 3, 4])    // 24
```

**Parameters:** `arr` — Array of numbers
**Returns:** `number`

### min

Find minimum value.

```nexora
print min([3, 1, 4, 1, 5])    // 1
```

**Parameters:** `arr` — Array of numbers
**Returns:** `number`

### max

Find maximum value.

```nexora
print max([3, 1, 4, 1, 5])    // 5
```

**Parameters:** `arr` — Array of numbers
**Returns:** `number`

### average

Calculate average.

```nexora
print average([1, 2, 3, 4, 5])    // 3
```

**Parameters:** `arr` — Array of numbers
**Returns:** `number`

## Object Functions

### keys

Get all keys.

```nexora
print keys({a: 1, b: 2})    // ["a", "b"]
```

**Parameters:** `obj` — Object
**Returns:** `array`

### values

Get all values.

```nexora
print values({a: 1, b: 2})    // [1, 2]
```

**Parameters:** `obj` — Object
**Returns:** `array`

### entries

Get key-value pairs.

```nexora
print entries({a: 1, b: 2})    // [["a", 1], ["b", 2]]
```

**Parameters:** `obj` — Object
**Returns:** `array`

## Type Conversion

### str

Convert value to string.

```nexora
print str(42)        // "42"
print str(3.14)      // "3.14"
print str(true)      // "true"
print str([1, 2])    // "[1, 2]"
```

**Parameters:** `value` — Any value
**Returns:** `string`

### num

Convert string to number.

```nexora
print num("42")      // 42
print num("3.14")    // 3.14
```

**Parameters:** `value` — String or number
**Returns:** `integer` or `float`

### int

Convert to integer.

```nexora
print int(3.7)       // 3
print int("42")      // 42
```

**Parameters:** `value` — Number or string
**Returns:** `integer`

### float

Convert to float.

```nexora
print float(42)      // 42.0
print float("3.14")  // 3.14
```

**Parameters:** `value` — Number or string
**Returns:** `float`

### parseInt

Parse string as integer.

```nexora
print parseInt("42")      // 42
print parseInt("3.14")    // 3
```

**Parameters:** `value` — String
**Returns:** `integer`

### parseFloat

Parse string as float.

```nexora
print parseFloat("3.14")    // 3.14
print parseFloat("42")      // 42.0
```

**Parameters:** `value` — String
**Returns:** `float`

## Date/Time Functions

### now

Get current Unix timestamp (seconds).

```nexora
let ts = now()
print ts    // 1705312345
```

**Parameters:** None
**Returns:** `integer`

### timestamp

Get current precise timestamp (milliseconds).

```nexora
let ts = timestamp()
print ts    // 1705312345678
```

**Parameters:** None
**Returns:** `integer`
