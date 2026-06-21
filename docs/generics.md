# Generics & Type Annotations

## Table of Contents

- [Type Annotations](#type-annotations)
- [Supported Types](#supported-types)
- [Type Inference](#type-inference)
- [Generic Functions](#generic-functions)
- [Generic Classes](#generic-classes)
- [Type Checking at Runtime](#type-checking-at-runtime)

## Type Annotations

Nexora supports optional type annotations for documentation and clarity:

```nexora
let name: string = "Nexora"
let version: int = 4
let pi: float = 3.14159
let active: bool = true
```

Type annotations are advisory — they don't enforce type safety at runtime.

## Supported Types

| Type | Description | Example |
|------|-------------|---------|
| `int` | Integer (64-bit signed) | `let x: int = 42` |
| `float` | Floating-point (64-bit) | `let x: float = 3.14` |
| `string` | Text | `let x: string = "hello"` |
| `bool` | Boolean | `let x: bool = true` |
| `null` | Null value | `let x: null = null` |
| `array` | Array | `let x: array = [1, 2, 3]` |
| `object` | Object/Map | `let x: object = {a: 1}` |

## Type Inference

Nexora automatically infers types from values:

```nexora
let x = 42          // inferred as integer
let y = 3.14        // inferred as float
let s = "hello"     // inferred as string
let b = true        // inferred as boolean
let arr = [1, 2, 3] // inferred as array
```

### Override Inference

Use type annotations to override inference:

```nexora
let x: float = 42   // Treat 42 as float
let y: int = 3.14   // Treat 3.14 as integer (truncates)
```

## Generic Functions

While Nexora doesn't have formal generics, you can write functions that work with any type:

```nexora
func identity(x) {
    return x
}

print identity(42)      // 42
print identity("hello") // "hello"
print identity([1, 2])  // [1, 2]
```

### Type-Checking Functions

Write functions that handle multiple types:

```nexora
func describe(value) {
    let t = typeof(value)
    if t == "integer" {
        return "Integer: " + str(value)
    }
    if t == "string" {
        return "String: " + value
    }
    if t == "array" {
        return "Array with " + str(len(value)) + " elements"
    }
    return "Unknown type: " + t
}

print describe(42)           // "Integer: 42"
print describe("hello")      // "String: hello"
print describe([1, 2, 3])    // "Array with 3 elements"
```

### Polymorphic Functions

```nexora
func first(arr) {
    if len(arr) > 0 {
        return arr[0]
    }
    return null
}

print first([1, 2, 3])     // 1
print first(["a", "b"])    // "a"
print first([true, false]) // true
```

## Generic Classes

Write classes that work with any type:

```nexora
class Container {
    init(value) {
        this.value = value
    }

    get() {
        return this.value
    }

    set(value) {
        this.value = value
    }

    type() {
        return typeof(this.value)
    }
}

let intContainer = new Container(42)
let strContainer = new Container("hello")
let arrContainer = new Container([1, 2, 3])

print intContainer.get()     // 42
print strContainer.get()     // "hello"
print arrContainer.type()    // "array"
```

### Stack Implementation

```nexora
class Stack {
    init() {
        this.items = []
    }

    push(item) {
        push(this.items, item)
    }

    pop() {
        return pop(this.items)
    }

    peek() {
        if len(this.items) == 0 {
            return null
        }
        return this.items[len(this.items) - 1]
    }

    isEmpty() {
        return len(this.items) == 0
    }

    size() {
        return len(this.items)
    }

    toArray() {
        return this.items
    }
}

// Works with any type
let intStack = new Stack()
intStack.push(1)
intStack.push(2)
intStack.push(3)
print intStack.pop()  // 3

let strStack = new Stack()
strStack.push("hello")
strStack.push("world")
print strStack.pop()  // "world"
```

## Type Checking at Runtime

Use `typeof` for runtime type checking:

```nexora
func add(a, b) {
    let typeA = typeof(a)
    let typeB = typeof(b)
    
    if typeA != "integer" && typeA != "float" {
        throw "First argument must be a number, got " + typeA
    }
    if typeB != "integer" && typeB != "float" {
        throw "Second argument must be a number, got " + typeB
    }
    
    return a + b
}

print add(5, 3)       // 8
print add(3.14, 2.0)  // 5.14

try {
    print add("hello", 5)
} catch (error) {
    print error  // "First argument must be a number, got string"
}
```

### Type Assertions

```nexora
func ensureString(value) {
    if typeof(value) != "string" {
        return str(value)
    }
    return value
}

print ensureString(42)       // "42"
print ensureString("hello")  // "hello"
```

## Best Practices

1. **Use type annotations for documentation** — Help readers understand expected types
2. **Use typeof for validation** — Check types at runtime when needed
3. **Write flexible functions** — Use typeof to handle multiple types
4. **Be defensive** — Validate inputs before processing
