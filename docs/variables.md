# Variables & Types

## Table of Contents

- [Declaring Variables](#declaring-variables)
- [Data Types](#data-types)
- [Type Checking](#type-checking)
- [Type Annotations](#type-annotations)
- [Constants](#constants)
- [Scope](#scope)
- [Type Conversion](#type-conversion)

## Declaring Variables

Use `let` to declare a variable:

```nexora
let name = "Nexora"
let version = 1
let pi = 3.14159
let isAwesome = true
let nothing = null
```

Variables are dynamically typed — the type is determined by the assigned value.

## Data Types

### Integer

Whole numbers (64-bit signed):

```nexora
let age = 25
let negative = -10
let zero = 0
```

### Float

Decimal numbers (64-bit floating point):

```nexora
let pi = 3.14159
let temperature = -40.0
```

### String

Text enclosed in double or single quotes:

```nexora
let name = "Nexora"
let greeting = 'Hello'
```

Strings support concatenation:

```nexora
let full = name + " is awesome"  // "Nexora is awesome"
```

### Boolean

Logical values:

```nexora
let isActive = true
let isDeleted = false
```

### Null

Represents absence of a value:

```nexora
let result = null
```

### Array

An ordered list of values:

```nexora
let numbers = [1, 2, 3, 4, 5]
let mixed = [1, "hello", true, null, [1, 2]]
```

### Object

A key-value map:

```nexora
let person = {
    name: "Nexora",
    version: 4,
    features: ["fast", "simple", "clean"]
}
```

### Function

Functions are first-class values:

```nexora
let add = func(a, b) => a + b
let double = x => x * 2
```

## Type Checking

Use `typeof` to get the type name of a value:

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

## Type Annotations

Nexora supports optional type annotations for clarity and documentation:

```nexora
let name: string = "Nexora"
let version: int = 4
let pi: float = 3.14159
```

Type annotations are advisory — the language remains dynamically typed at runtime.

## Constants

Use `const` to declare a value that cannot be reassigned:

```nexora
const MAX_SIZE = 100
const PI = 3.14159
const APP_NAME = "Nexora"
```

Attempting to reassign a constant causes a runtime error:

```nexora
const x = 5
x = 10  // Error: Cannot reassign constant
```

## Scope

Variables are scoped to the block in which they are defined:

```nexora
let global = "I'm global"

func myFunc() {
    let local = "I'm local"
    print global  // accessible
    print local   // accessible
}

// print local  // Error: local is not defined
```

### Block Scoping

Variables declared inside a block are only accessible within that block:

```nexora
let x = 10

if true {
    let y = 20
    print x  // 10
    print y  // 20
}

// print y  // Error: y is not defined
```

### Function Scoping

```nexora
func outer() {
    let a = 1
    
    func inner() {
        let b = 2
        print a  // accessible (closure)
        print b  // accessible
    }
    
    inner()
    // print b  // Error: b is not defined
}
```

### Closure Scope

Functions capture variables from their enclosing scope:

```nexora
let multiplier = 3
let multiply = x => x * multiplier

print multiply(5)  // 15
```

## Type Conversion

Nexora provides built-in functions for type conversion:

### To String

```nexora
let num = 42
let strNum = str(num)      // "42"
let piStr = str(3.14)      // "3.14"
let boolStr = str(true)     // "true"
```

### To Number

```nexora
let numStr = "42"
let num = num(numStr)       // 42
let intVal = parseInt("42")  // 42
let floatVal = parseFloat("3.14")  // 3.14
```

### To Integer

```nexora
let float = 3.7
let int = int(float)        // 3 (truncates)
```

## String Operations

Strings have several built-in methods:

```nexora
let s = "Hello, World!"

len(s)                    // 13
s.toUpperCase()           // "HELLO, WORLD!"
s.toLowerCase()           // "hello, world!"
s.trim()                  // "Hello, World!"
s.includes("World")       // true
s.startsWith("Hello")     // true
s.endsWith("!")           // true
s.indexOf("World")        // 7
s.charAt(0)               // "H"
```

See [Standard Library](standard-library.md) for the complete string API.

## Array Operations

```nexora
let arr = [1, 2, 3, 4, 5]

len(arr)          // 5
arr[0]            // 1
arr.length        // 5
```

See [Arrays](arrays.md) for the complete array API.
