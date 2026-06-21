# Arrays

## Table of Contents

- [Creating Arrays](#creating-arrays)
- [Accessing Elements](#accessing-elements)
- [Modifying Arrays](#modifying-arrays)
- [Array Methods](#array-methods)
- [Higher-Order Array Methods](#higher-order-array-methods)
- [Array Properties](#array-properties)
- [Nested Arrays](#nested-arrays)
- [Iterating Arrays](#iterating-arrays)

## Creating Arrays

```nexora
let numbers = [1, 2, 3, 4, 5]
let fruits = ["apple", "banana", "cherry"]
let mixed = [1, "hello", true, null, 3.14]
let empty = []
```

## Accessing Elements

Arrays are zero-indexed:

```nexora
let fruits = ["apple", "banana", "cherry"]

print fruits[0]   // "apple"
print fruits[1]   // "banana"
print fruits[2]   // "cherry"
```

### Last Element

```nexora
let arr = [1, 2, 3, 4, 5]
print arr[len(arr) - 1]  // 5
```

### Negative Indexing (not supported)

Use `len()` to access from the end:

```nexora
let arr = [1, 2, 3, 4, 5]
let last = arr[len(arr) - 1]  // 5
let secondLast = arr[len(arr) - 2]  // 4
```

## Modifying Arrays

```nexora
let arr = [1, 2, 3]

// push - add element(s) to end
let extended = push(arr, 4)
print extended  // [1, 2, 3, 4]

// pop - remove last element
let item = pop(arr)
print item      // 3
print arr       // [1, 2]
```

## Array Methods

### Built-in Functions

| Function | Description | Example |
|----------|-------------|---------|
| `push(arr, item)` | Add element(s) to end | `push([1,2], 3)` → `[1,2,3]` |
| `pop(arr)` | Remove last element | `pop([1,2,3])` → `3` |
| `len(arr)` | Get length | `len([1,2,3])` → `3` |
| `sort(arr)` | Sort elements | `sort([3,1,2])` → `[1,2,3]` |
| `reverse(arr)` | Reverse order | `reverse([1,2,3])` → `[3,2,1]` |
| `unique(arr)` | Remove duplicates | `unique([1,1,2,2])` → `[1,2]` |
| `flatten(arr)` | Flatten nested arrays | `flatten([[1,2],[3]])` → `[1,2,3]` |
| `range(start, end)` | Generate number range | `range(1, 5)` → `[1,2,3,4]` |
| `zip(arr1, arr2)` | Combine arrays | `zip([1,2], ["a","b"])` → `[[1,"a"],[2,"b"]]` |

### Object Key-Value Functions

| Function | Description | Example |
|----------|-------------|---------|
| `keys(obj)` | Get object keys | `keys({a:1, b:2})` → `["a","b"]` |
| `values(obj)` | Get object values | `values({a:1, b:2})` → `[1,2]` |
| `entries(obj)` | Get key-value pairs | `entries({a:1})` → `[["a",1]]` |

## Higher-Order Array Methods

### map

Transform each element:

```nexora
let numbers = [1, 2, 3, 4, 5]
let doubled = map(numbers, x => x * 2)
print doubled  // [2, 4, 6, 8, 10]
```

### filter

Select matching elements:

```nexora
let numbers = [1, 2, 3, 4, 5, 6]
let evens = filter(numbers, x => x % 2 == 0)
print evens  // [2, 4, 6]
```

### reduce

Accumulate into single value:

```nexora
let numbers = [1, 2, 3, 4, 5]
let sum = reduce(numbers, (acc, x) => acc + x, 0)
print sum  // 15
```

### find

Find first matching element:

```nexora
let numbers = [1, 2, 3, 4, 5]
let found = find(numbers, x => x > 3)
print found  // 4
```

### findIndex

Find index of first matching element:

```nexora
let numbers = [1, 2, 3, 4, 5]
let idx = find_index(numbers, x => x > 3)
print idx  // 3
```

### every / some

Test all or any elements:

```nexora
let numbers = [2, 4, 6, 8]
print every(numbers, x => x % 2 == 0)  // true
print some(numbers, x => x > 5)        // true
```

### includes / indexOf

Check membership and position:

```nexora
let arr = [1, 2, 3, 4, 5]
print includes(arr, 3)     // true
print index_of(arr, 3)     // 2
print last_index_of(arr, 3) // 2
```

### flatMap

Map and flatten:

```nexora
let arr = [1, 2, 3]
let result = flat_map(arr, x => [x, x * 2])
print result  // [1, 2, 2, 4, 3, 6]
```

### groupBy / countBy

Group and count elements:

```nexora
let numbers = [1, 2, 3, 4, 5, 6]
let grouped = group_by(numbers, x => x % 2 == 0 ? "even" : "odd")
print grouped  // {even: [2, 4, 6], odd: [1, 3, 5]}

let counts = count_by(numbers, x => x % 2 == 0 ? "even" : "odd")
print counts  // {even: 3, odd: 3}
```

### partition

Split into two groups:

```nexora
let numbers = [1, 2, 3, 4, 5, 6]
let [evens, odds] = partition(numbers, x => x % 2 == 0)
print evens  // [2, 4, 6]
print odds   // [1, 3, 5]
```

### shuffle / sample

Randomize and pick:

```nexora
let arr = [1, 2, 3, 4, 5]
let shuffled = shuffle(arr)
let sampled = sample(arr, 3)
```

### sum / product / min / max / average

Aggregation:

```nexora
let arr = [1, 2, 3, 4, 5]
print sum(arr)       // 15
print product(arr)   // 120
print min(arr)       // 1
print max(arr)       // 5
print average(arr)   // 3
```

## Array Properties

```nexora
let arr = [1, 2, 3, 4, 5]
print arr.length  // 5
```

## Nested Arrays

```nexora
let matrix = [
    [1, 2, 3],
    [4, 5, 6],
    [7, 8, 9]
]

print matrix[0][0]  // 1
print matrix[1][2]  // 6

// Flatten nested arrays
let flat = flatten(matrix)
print flat  // [1, 2, 3, 4, 5, 6, 7, 8, 9]
```

## Iterating Arrays

### For Loop

```nexora
let fruits = ["apple", "banana", "cherry"]
for fruit in fruits {
    print fruit
}
```

### With Index

```nexora
let arr = ["a", "b", "c"]
let i = 0
for item in arr {
    print str(i) + ": " + item
    i += 1
}
```

### Functional Iteration

```nexora
let numbers = [1, 2, 3, 4, 5]

// Print each element
map(numbers, x => {
    print x
    return x
})

// Sum all elements
let total = reduce(numbers, (acc, x) => acc + x, 0)
print "Total: " + str(total)
```
