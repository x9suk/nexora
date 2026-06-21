# Migration Guide

## Table of Contents

- [Overview](#overview)
- [From JavaScript](#from-javascript)
- [From Python](#from-python)
- [From Rust](#from-rust)
- [From Go](#from-go)
- [Key Differences](#key-differences)
- [Common Patterns](#common-patterns)

## Overview

This guide helps developers familiar with other languages transition to Nexora.

## From JavaScript

### Variables

```javascript
// JavaScript
let name = "Nexora"
const version = 4
var old = "avoid"
```

```nexora
// Nexora
let name = "Nexora"
const version = 4
// No 'var' equivalent
```

### Functions

```javascript
// JavaScript
function greet(name) {
    return `Hello, ${name}!`
}

const add = (a, b) => a + b
```

```nexora
// Nexora
func greet(name) {
    return "Hello, " + name + "!"
}

let add = (a, b) => a + b
```

### Arrays

```javascript
// JavaScript
let arr = [1, 2, 3]
arr.push(4)
arr.pop()
arr.map(x => x * 2)
arr.filter(x => x > 1)
arr.reduce((acc, x) => acc + x, 0)
```

```nexora
// Nexora
let arr = [1, 2, 3]
let extended = push(arr, 4)
let item = pop(arr)
let doubled = map(arr, x => x * 2)
let filtered = filter(arr, x => x > 1)
let sum = reduce(arr, (acc, x) => acc + x, 0)
```

### Classes

```javascript
// JavaScript
class Animal {
    constructor(name) {
        this.name = name
    }
    speak() {
        return `${this.name} makes a sound`
    }
}

class Dog extends Animal {
    bark() {
        return `${this.name} barks!`
    }
}
```

```nexora
// Nexora
class Animal {
    init(name) {
        this.name = name
    }
    speak() {
        return "${this.name} makes a sound"
    }
}

class Dog extends Animal {
    init(name) {
        super(name)
    }
    bark() {
        return "${this.name} barks!"
    }
}
```

### Strings

```javascript
// JavaScript
let upper = str.toUpperCase()
let lower = str.toLowerCase()
let parts = str.split(",")
let joined = arr.join("-")
```

```nexora
// Nexora
let upper = str.toUpperCase()
let lower = str.toLowerCase()
let parts = split(str, ",")
let joined = join(arr, "-")
```

## From Python

### Variables

```python
# Python
name = "Nexora"
version = 4
```

```nexora
// Nexora
let name = "Nexora"
let version = 4
```

### Functions

```python
# Python
def greet(name):
    return f"Hello, {name}!"

add = lambda a, b: a + b
```

```nexora
// Nexora
func greet(name) {
    return "Hello, " + name + "!"
}

let add = (a, b) => a + b
```

### Lists

```python
# Python
arr = [1, 2, 3]
arr.append(4)
arr.pop()
[x * 2 for x in arr]
[x for x in arr if x > 1]
sum(arr)
```

```nexora
// Nexora
let arr = [1, 2, 3]
let extended = push(arr, 4)
let item = pop(arr)
let doubled = map(arr, x => x * 2)
let filtered = filter(arr, x => x > 1)
let total = sum(arr)
```

### Dictionaries

```python
# Python
person = {"name": "Nexora", "version": 4}
person["name"]
person.keys()
person.values()
```

```nexora
// Nexora
let person = { name: "Nexora", version: 4 }
person.name
keys(person)
values(person)
```

### Classes

```python
# Python
class Animal:
    def __init__(self, name):
        self.name = name
    
    def speak(self):
        return f"{self.name} makes a sound"

class Dog(Animal):
    def bark(self):
        return f"{self.name} barks!"
```

```nexora
// Nexora
class Animal {
    init(name) {
        this.name = name
    }
    
    speak() {
        return "${this.name} makes a sound"
    }
}

class Dog extends Animal {
    init(name) {
        super(name)
    }
    
    bark() {
        return "${this.name} barks!"
    }
}
```

## From Rust

### Variables

```rust
// Rust
let name = String::from("Nexora");
let version = 4;
let mut counter = 0;
```

```nexora
// Nexora
let name = "Nexora"
let version = 4
let counter = 0
counter += 1  // Reassignment works
```

### Functions

```rust
// Rust
fn greet(name: &str) -> String {
    format!("Hello, {}!", name)
}

let add = |a: i32, b: i32| -> i32 { a + b };
```

```nexora
// Nexora
func greet(name) {
    return "Hello, " + name + "!"
}

let add = (a, b) => a + b
```

### Error Handling

```rust
// Rust
fn divide(a: f64, b: f64) -> Result<f64, String> {
    if b == 0.0 {
        return Err("Division by zero".to_string());
    }
    Ok(a / b)
}

match divide(10.0, 0.0) {
    Ok(result) => println!("{}", result),
    Err(e) => println!("Error: {}", e),
}
```

```nexora
// Nexora
func divide(a, b) {
    if b == 0 {
        throw "Division by zero"
    }
    return a / b
}

try {
    let result = divide(10, 0)
    print result
} catch (error) {
    print "Error: " + error
}
```

## From Go

### Variables

```go
// Go
name := "Nexora"
version := 4
var counter int = 0
```

```nexora
// Nexora
let name = "Nexora"
let version = 4
let counter = 0
```

### Functions

```go
// Go
func greet(name string) string {
    return fmt.Sprintf("Hello, %s!", name)
}
```

```nexora
// Nexora
func greet(name) {
    return "Hello, " + name + "!"
}
```

### Structs → Classes

```go
// Go
type Animal struct {
    Name string
}

func (a Animal) Speak() string {
    return a.Name + " makes a sound"
}
```

```nexora
// Nexora
class Animal {
    init(name) {
        this.name = name
    }
    
    speak() {
        return this.name + " makes a sound"
    }
}
```

## Key Differences

| Feature | Other Languages | Nexora |
|---------|----------------|--------|
| Variable declaration | `var`, `const`, `let` varies | `let` (mutable), `const` (immutable) |
| Function declaration | `function`, `def`, `fn` | `func` |
| String interpolation | `` `${var}` ``, `f"{var}"` | `${var}` |
| Array methods | Methods on arrays | Global functions (`map`, `filter`, etc.) |
| Classes | `constructor`, `__init__`, `new` | `init` |
| Null | `null`, `None`, `nil` | `null` |
| Boolean | `true`, `True` | `true` |
| Comments | `//`, `#`, `/* */` | `//`, `/* */` |

## Common Patterns

### Loop with Index

```nexora
let arr = [1, 2, 3]
let i = 0
for item in arr {
    print str(i) + ": " + str(item)
    i += 1
}
```

### Object Iteration

```nexora
let obj = { a: 1, b: 2, c: 3 }
for key in keys(obj) {
    print key + ": " + str(obj[key])
}
```

### Error Handling Pattern

```nexora
func safeDivide(a, b) {
    try {
        return a / b
    } catch (error) {
        print "Error: " + error
        return null
    }
}
```

### Functional Composition

```nexora
func compose(f, g) {
    return x => f(g(x))
}

let double = x => x * 2
let addTen = x => x + 10
let doubleThenAddTen = compose(addTen, double)

print doubleThenAddTen(5)  // 20
```
