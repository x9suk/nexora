# Nexora v1.0 — Complete Language Specification & Architecture Blueprint

> This document is the authoritative reference for building Nexora v1.0.
> Every feature, every syntax form, every compiler phase — defined here.

---

## Table of Contents

1. [Language Specification](#1-language-specification)
2. [Type System](#2-type-system)
3. [Error Handling](#3-error-handling)
4. [Compiler Architecture](#4-compiler-architecture)
5. [Standard Library](#5-standard-library)
6. [Package Manager (nxm)](#6-package-manager-nxm)
7. [CLI Design](#7-cli-design)
8. [Developer Tooling](#8-developer-tooling)
9. [Package Registry](#9-package-registry)
10. [Rust Implementation Folder Structure](#10-rust-implementation-folder-structure)
11. [Phased Roadmap](#11-phased-roadmap)

---

## 1. Language Specification

### 1.1 Design Philosophy

- **Readable**: Code reads like English. No cryptic symbols.
- **Safe**: Gradual types, no null, explicit errors, no undefined behavior.
- **Fast**: Compile to bytecode via SSA IR, JIT-ready.
- **Simple**: Few orthogonal features that compose well.
- **Practical**: Batteries-included stdlib, first-class async, package manager.

### 1.2 Lexical Structure

**File extension**: `.nx`

**Encoding**: UTF-8 (required)

**Line endings**: LF (`\n`) or CRLF (`\r\n`)

**Identifiers**: `[a-zA-Z_][a-zA-Z0-9_]*`
- Snake case for variables/functions: `my_var`, `calculate_total`
- PascalCase for types/classes: `MyClass`, `Result`
- SCREAMING_SNAKE for constants: `MAX_SIZE`, `PI`
- Leading underscore = intentionally unused: `_tmp`

**Literals**:
```
42          // integer (i64)
3.14        // float (f64)
"hello"     // string (UTF-8)
'c'         // char (single-quoted, stored as String)
true        // boolean
false       // boolean
null        // null (only for Option<T> = None)
```

### 1.3 Keywords (Final Set — 35 keywords)

```
// Declarations
let         const       func        class       struct
enum        trait       impl        type        module
export      import      as          from        pub

// Control Flow
if          else        elif        while       for
in          loop        break       continue    return
match       when        yield

// Error Handling
try         catch       finally     throw       panic

// Async
async       await

// Other
new         this        super       self        true
false       null        as          is          in
```

**Reserved for future use**: `defer`, `var`, `static`, `unsafe`, `ref`, `mut`

### 1.4 Operators & Precedence

| Precedence | Operator | Associativity | Description |
|:----------:|:--------:|:-------------:|:------------|
| 1 (lowest) | `\|\|` | Left | Logical OR (short-circuit) |
| 2 | `&&` | Left | Logical AND (short-circuit) |
| 3 | `\|` | Left | Bitwise OR |
| 4 | `^` | Left | Bitwise XOR |
| 5 | `&` | Left | Bitwise AND |
| 6 | `==` `!=` | Left | Equality |
| 7 | `<` `>` `<=` `>=` | Left | Comparison |
| 8 | `<<` `>>` | Left | Bitwise shift |
| 9 | `+` `-` | Left | Addition, subtraction |
| 10 | `*` `/` `%` | Left | Multiplication, division, modulo |
| 11 | `**` | Right | Exponentiation |
| 12 | `!` `-` `~` `@` | Right | Unary (NOT, negate, bitwise NOT, typeof) |
| 13 | `?.` `?.[` | Left | Optional chaining |
| 14 | `.` `[` `(` | Left | Property access, indexing, call |
| 15 (highest) | `??` | Right | Null coalescing |

**Additional operators**:
```
=           +=  -=  *=  /=  %=  **=  &=  |=  ^=  <<=  >>=
=>          ->          // arrow functions / return types
? :         // ternary
..  ..=     // range (exclusive .., inclusive ..=)
..          // spread (... in other contexts)
```

### 1.5 Comments

```nx
// Single-line comment

/* Multi-line
   comment */

/// Doc comment (for documentation generator)
fn hello() {
    // ...
}
```

### 1.6 String Interpolation

```nx
let name = "World"
let greeting = "Hello, ${name}!"
let expr = "2 + 3 = ${2 + 3}"
let escaped = "No interpolation: $${name}"
```

Triple-quoted strings (no escape processing):
```nx
let path = """
C:\Users\name\file.txt
Multiple lines preserved exactly
"""
```

Raw strings (no interpolation, no escapes):
```nx
let regex = r"\d+\.\d+"
let path = r"C:\Users\name"
```

### 1.7 Variables

```nx
// Mutable binding
let x = 10
x = 20  // OK

// Immutable binding
const PI = 3.14159
// PI = 3.0  // ERROR: cannot reassign const

// Type annotation (optional — inferred by HM type checker)
let name: String = "Alice"
let age: Int = 30
let items: Array<Int> = [1, 2, 3]

// Destructuring
let [a, b, ..rest] = [1, 2, 3, 4, 5]
let { x, y, z: w } = { x: 1, y: 2, z: 3 }

// Tuple destructuring
let (x, y) = (10, 20)
let point: (Int, Int) = (10, 20)

// Underscore = unused
let [first, _, third] = [1, 2, 3]
```

### 1.8 Types

```nx
// Primitive types
let i: Int = 42                    // 64-bit signed integer
let f: Float = 3.14                // 64-bit float
let s: String = "hello"            // UTF-8 string
let b: Bool = true                 // boolean
let n: Null = null                 // null (only for Option<T>)
let c: Char = 'x'                  // single character

// Compound types
let arr: Array<Int> = [1, 2, 3]    // dynamic array
let m: Map<String, Int> = {"a": 1} // hash map
let t: Tuple<Int, String> = (1, "hello")  // tuple
let p: (Int, Float, Bool) = (1, 2.0, true) // unnamed tuple

// Custom types
struct Point {
    x: Float
    y: Float
}

class Animal {
    name: String

    func init(name: String) {
        this.name = name
    }

    func speak() -> String {
        return "..."
    }
}

enum Color {
    Red
    Green
    Blue
    Rgb(Int, Int, Int)
}

// Type aliases
type UserId = Int
type Callback = func(Int) -> Bool
type Matrix = Array<Array<Float>>

// Opaque types (hide implementation)
opaque type Token = String
```

### 1.9 Functions

```nx
// Named function
func add(a: Int, b: Int) -> Int {
    return a + b
}

// Expression body (implicit return)
func add(a: Int, b: Int) -> Int = a + b

// Default parameters
func greet(name: String, greeting: String = "Hello") {
    print("${greeting}, ${name}!")
}
greet("Alice")                    // "Hello, Alice!"
greet("Bob", "Hi")                // "Hi, Bob!"

// Variadic parameters
func sum(..numbers: Array<Int>) -> Int {
    let total = 0
    for n in numbers {
        total += n
    }
    return total
}
sum(1, 2, 3, 4)  // 10

// Named parameters (call-site)
func create_user(name: String, age: Int, email: String) { ... }
create_user(name: "Alice", age: 30, email: "alice@example.com")

// Anonymous function
let square = func(x: Int) -> Int {
    return x * x
}

// Lambda (single expression)
let double = (x: Int) -> Int = x * 2
let add = (a, b) => a + b           // inferred types
let noop = () => {}                  // void lambda
let print_hello = () => print("Hello!")

// Closure captures by reference for mutable, by value for immutable
func counter() -> func() -> Int {
    let count = 0                    // captured by reference (mutable)
    return func() -> Int {
        count += 1
        return count
    }
}
let c = counter()
c()  // 1
c()  // 2
c()  // 3

// Higher-order functions
func apply(f: func(Int) -> Int, x: Int) -> Int = f(x)
apply(double, 5)  // 10

// Generics
func first<T>(arr: Array<T>) -> T {
    return arr[0]
}
first([1, 2, 3])       // 1
first(["a", "b"])      // "a"

// Named returns
func divide(a: Float, b: Float) -> (result: Float, error: String) {
    if b == 0.0 {
        return (0.0, "division by zero")
    }
    return (a / b, "")
}
let (result, err) = divide(10.0, 3.0)
```

### 1.10 Control Flow

```nx
// if / else if / else
if x > 0 {
    print("positive")
} elif x < 0 {
    print("negative")
} else {
    print("zero")
}

// if as expression
let label = if x > 0 { "positive" } else { "non-positive" }

// while
while condition {
    do_something()
}

// for-in (arrays, strings, maps)
for item in [1, 2, 3] {
    print(item)
}

for ch in "hello" {
    print(ch)
}

for key in my_map {
    print("${key}: ${my_map[key]}")
}

// for-range
for i in 0..10 {
    print(i)          // 0, 1, 2, ..., 9
}

for i in 0..=10 {
    print(i)          // 0, 1, 2, ..., 10
}

for i in range(0, 100, 5) {
    print(i)          // 0, 5, 10, ..., 95
}

// for with index
for i, item in ["a", "b", "c"] {
    print("${i}: ${item}")
}

// for with destructuring
for (name, age) in [("Alice", 30), ("Bob", 25)] {
    print("${name} is ${age}")
}

// loop (infinite, use break to exit)
loop {
    let input = input("Enter command: ")
    if input == "quit" {
        break
    }
    process(input)
}

// break/continue with labels
outer: for i in 0..10 {
    inner: for j in 0..10 {
        if i * j > 20 {
            break outer
        }
        print("${i},${j}")
    }
}

// match (exhaustive pattern matching)
match command {
    "quit" => exit(),
    "help" => show_help(),
    "status" => show_status(),
    _ => print("Unknown command"),
}

// Pattern matching with values
match number {
    0 => "zero",
    1 | 2 | 3 => "small",
    n if n < 0 => "negative",
    n if n > 100 => "large",
    _ => "other",
}

// Destructuring patterns
match point {
    (0, 0) => "origin",
    (x, 0) => "on x-axis at ${x}",
    (0, y) => "on y-axis at ${y}",
    (x, y) => "at (${x}, ${y})",
}

// Enum pattern matching
match color {
    Color::Red => "red",
    Color::Green => "green",
    Color::Blue => "blue",
    Color::Rgb(r, g, b) => "rgb(${r}, ${g}, ${b})",
}

// Array pattern matching
match arr {
    [] => "empty",
    [x] => "single: ${x}",
    [x, y] => "pair: ${x}, ${y}",
    [first, ..rest] => "first: ${first}, rest has ${len(rest)} items",
}

// with guard
match value {
    n if n > 0 && n < 100 => "small positive",
    n if n >= 100 => "large",
    _ => "other",
}
```

### 1.11 Classes & OOP

```nx
// Basic class
class Animal {
    name: String
    sound: String

    func init(name: String, sound: String) {
        this.name = name
        this.sound = sound
    }

    func speak() -> String {
        return "${this.name} says ${this.sound}!"
    }

    // Static method
    static func create(name: String) -> Animal {
        return new Animal(name, "...")
    }
}

// Inheritance
class Dog extends Animal {
    func init(name: String) {
        super(name, "Woof")
    }

    func fetch(item: String) -> String {
        return "${this.name} fetches the ${item}"
    }
}

// Polymorphism
let animals: Array<Animal> = [
    new Dog("Rex"),
    new Animal("Cat", "Meow"),
]

for animal in animals {
    print(animal.speak())
}

// Get type at runtime
let dog = new Dog("Rex")
print(type_of(dog))           // "Dog"
print(dog is Animal)          // true
print(dog is Dog)             // true

// toString override (convention, not magic method)
class Point {
    x: Float
    y: Float

    func init(x: Float, y: Float) {
        this.x = x
        this.y = y
    }

    func to_string() -> String {
        return "(${this.x}, ${this.y})"
    }
}
```

### 1.12 Structs (Value Semantics)

```nx
struct Point {
    x: Float
    y: Float
}

let p1 = Point { x: 1.0, y: 2.0 }
let p2 = p1                     // COPY, not reference
p2.x = 5.0
print(p1.x)                    // 1.0 (unchanged)

// Struct with methods
impl Point {
    func distance_to(other: Point) -> Float {
        let dx = this.x - other.x
        let dy = this.y - other.y
        return sqrt(dx * dx + dy * dy)
    }
}

// Tuple struct
struct Color(Int, Int, Int)
let red = Color(255, 0, 0)
```

### 1.13 Traits (Interfaces)

```nx
// Trait definition
trait Drawable {
    func draw()

    // Default implementation
    func describe() -> String {
        return "A drawable object"
    }
}

// Implementation
impl Drawable for Circle {
    func draw() {
        print("Drawing circle at ${this.center}")
    }
}

// Generic trait bounds
func render<T: Drawable>(item: T) {
    item.draw()
}

// Multiple trait bounds
func process<T: Drawable + Comparable>(item: T) {
    item.draw()
}

// Trait as parameter type
func draw_all(items: Array<dyn Drawable>) {
    for item in items {
        item.draw()
    }
}

// Operator overloading via traits
trait Addable {
    func add(other: Self) -> Self
}

impl Addable for Vector {
    func add(other: Vector) -> Vector {
        return new Vector(this.x + other.x, this.y + other.y)
    }
}
```

### 1.14 Enums (Algebraic Data Types)

```nx
// Simple enum
enum Direction {
    North
    South
    East
    West
}

// Enum with data
enum Shape {
    Circle(Float)
    Rectangle(Float, Float)
    Triangle(Float, Float, Float)
}

// Named fields enum
enum Message {
    Quit
    Move { x: Int, y: Int }
    Write(String)
    Color(Int, Int, Int)
}

// Usage
let shape = Shape::Circle(5.0)
match shape {
    Shape::Circle(r) => "Circle with radius ${r}",
    Shape::Rectangle(w, h) => "Rectangle ${w}x${h}",
    Shape::Triangle(a, b, c) => "Triangle with sides ${a}, ${b}, ${c}",
}

// Result<T, E> — built-in
enum Result<T, E> {
    Ok(T)
    Err(E)
}

// Option<T> — built-in
enum Option<T> {
    Some(T)
    None
}
```

### 1.15 Modules & Imports

```nx
// File: math_utils.nx
export func add(a: Int, b: Int) -> Int = a + b
export func subtract(a: Int, b: Int) -> Int = a - b
export const PI = 3.14159
export struct Vector { x: Float, y: Float }

// Import styles
import "math_utils"                    // import all exports into scope
import math_utils                      // same as above

import { add, subtract } from "math_utils"  // named imports
import { PI as PI_CONST } from "math_utils"  // aliased import
import math_utils as math              // namespace import

// Module declaration (alternative to file-based)
module geometry {
    pub func area(shape: Shape) -> Float {
        // ...
    }
}

// Nested module
module game {
    pub module physics {
        pub func gravity() -> Float = 9.8
    }
}
import game.physics.gravity
```

### 1.16 Error Handling

```nx
// Result<T, E> is the primary error handling mechanism
func read_config(path: String) -> Result<Config, String> {
    let content = read_file(path)?
    let parsed = json_parse(content)?
    return Ok(Config::from_json(parsed))
}

// ? operator for propagation
func process() -> Result<String, String> {
    let data = read_file("data.txt")?    // returns Err if fails
    let parsed = json_parse(data)?        // returns Err if fails
    let result = transform(parsed)?       // returns Err if fails
    return Ok(result)
}

// Custom error types
enum AppError {
    NotFound(String)
    PermissionDenied
    NetworkError(String)
    ParseError { line: Int, col: Int, message: String }
}

func fetch(url: String) -> Result<Response, AppError> {
    let response = http_get(url).map_err(
        |e| AppError::NetworkError(e)
    )?
    return Ok(response)
}

// try/catch only for external/unrecoverable errors
try {
    let file = open_file("critical.dat")
    process(file)
} catch (e: IoError) {
    log("IO error: ${e.message}")
    recover()
} finally {
    cleanup()
}

// panic for unrecoverable errors
func divide(a: Int, b: Int) -> Int {
    if b == 0 {
        panic("Division by zero")
    }
    return a / b
}
```

### 1.17 Generics

```nx
// Generic function
func first<T>(arr: Array<T>) -> Option<T> {
    if arr.is_empty() {
        return None
    }
    return Some(arr[0])
}

// Multiple type parameters
func zip<A, B>(a: Array<A>, b: Array<B>) -> Array<(A, B)> {
    let result = []
    for i in 0..min(len(a), len(b)) {
        result.push((a[i], b[i]))
    }
    return result
}

// Type constraints
func sort<T: Comparable>(arr: Array<T>) -> Array<T> {
    // ...
}

// Generic class
class Stack<T> {
    items: Array<T>

    func init() {
        this.items = []
    }

    func push(item: T) {
        this.items.push(item)
    }

    func pop() -> Option<T> {
        return this.items.pop()
    }

    func peek() -> Option<T> {
        if this.items.is_empty() {
            return None
        }
        return Some(this.items[this.items.len() - 1])
    }
}

let stack = new Stack<Int>()
stack.push(1)
stack.push(2)
print(stack.pop())  // Some(2)

// Generic enum (already built-in)
// Result<T, E>, Option<T>

// Where clauses
func process<T, U>(items: Array<T>) -> Array<U>
where
    T: Convertible<U>
{
    // ...
}
```

### 1.18 Comprehensions

```nx
// Array comprehension
let squares = [x * x for x in 0..10]
let evens = [x for x in range if x % 2 == 0]
let names = [user.name for user in users if user.age >= 18]

// Map comprehension
let indexed = {item.name: item for item in items}
let scores = {name: 0 for name in ["Alice", "Bob", "Charlie"]}

// Set comprehension
let unique = {x for x in arr}
```

### 1.19 Spread & Rest

```nx
// Spread operator
let arr1 = [1, 2, 3]
let arr2 = [4, 5, 6]
let combined = [...arr1, ...arr2]          // [1, 2, 3, 4, 5, 6]

let obj1 = {a: 1, b: 2}
let obj2 = {c: 3, d: 4}
let merged = {...obj1, ...obj2}           // {a: 1, b: 2, c: 3, d: 4}

// Function call spread
let args = [1, 2, 3]
print(*args)                               // print(1, 2, 3)

// Rest parameters
func sum(..numbers: Array<Int>) -> Int {
    let total = 0
    for n in numbers {
        total += n
    }
    return total
}

// Rest in destructuring
let [first, ..rest] = [1, 2, 3, 4]
let {name, ..rest} = {name: "Alice", age: 30, city: "NYC"}
```

### 1.20 Optional Chaining & Null Coalescing

```nx
// Optional chaining (safe navigation)
let city = user?.address?.city
let first_char = name?.chars?.[0]

// Null coalescing
let name = user?.name ?? "Anonymous"
let value = config?.get("key") ?? "default"

// Combined
let x = a?.b?.c ?? "fallback"
```

### 1.21 Ternary Operator

```nx
let label = if x > 0 { "positive" } else { "non-positive" }
// or
let label = x > 0 ? "positive" : "non-positive"
```

### 1.22 Async / Await

```nx
// Async function
async func fetch_data(url: String) -> Result<String, String> {
    let response = await http_get_async(url)
    return Ok(response)
}

// Async main
async func main() {
    let data = await fetch_data("https://api.example.com/data")
    match data {
        Ok(body) => print(body),
        Err(e) => print("Error: ${e}"),
    }
}

// Concurrent execution
async func main() {
    let results = await all([
        fetch_data("https://api.example.com/users"),
        fetch_data("https://api.example.com/posts"),
        fetch_data("https://api.example.com/comments"),
    ])
    print(results)
}

// Channel-based concurrency
let channel = Channel<Int>()
spawn func producer() {
    for i in 0..10 {
        channel.send(i)
    }
    channel.close()
}
spawn func consumer() {
    while let value = channel.recv() {
        print("Got: ${value}")
    }
}

// Join handles
let handle = spawn func() {
    return expensive_computation()
}
let result = handle.join()
```

### 1.23 Range Expressions

```nx
// Exclusive range
0..10        // 0, 1, 2, ..., 9
0.0..1.0     // floating point range (exclusive)

// Inclusive range
0..=10       // 0, 1, 2, ..., 10

// Custom step
range(0, 100, 5)   // 0, 5, 10, ..., 95
range(10, 0, -1)   // 10, 9, 8, ..., 1

// Range in for loop
for i in 0..5 {
    print(i)
}

// Range as array
let arr = [0..5]   // [0, 1, 2, 3, 4]
```

### 1.24 Test Syntax

```nx
test "addition works" {
    assert(add(2, 3) == 5)
    assert(add(-1, 1) == 0)
}

test "string interpolation" {
    let name = "World"
    assert("Hello, ${name}!" == "Hello, World!")
}

test "async operations" async {
    let result = await fetch_data("https://api.example.com")
    assert(result.is_ok())
}

// Test groups
describe("math operations") {
    test("addition") {
        assert(add(1, 1) == 2)
    }

    test("subtraction") {
        assert(subtract(5, 3) == 2)
    }
}
```

---

## 2. Type System

### 2.1 Gradual Typing

Nexora uses **gradual typing** — optional static type annotations with Hindley-Milner type inference.

```nx
// Fully inferred (no annotations needed)
let x = 10              // inferred: Int
let arr = [1, 2, 3]    // inferred: Array<Int>
func add(a, b) = a + b // inferred: func(Int, Int) -> Int

// Explicit annotations (checked at compile time)
let name: String = "Alice"
func greet(name: String) -> String = "Hello, ${name}!"

// Mix: some annotated, some inferred
let x: Int = 10
let y = x + 5          // y inferred as Int
```

### 2.2 Type Rules

**Primitive types** (inferred from literals):
| Type | Width | Range | Literal Example |
|------|-------|-------|-----------------|
| `Int` | 64-bit | ±9.2×10¹⁸ | `42` |
| `Float` | 64-bit | ±1.8×10³⁰⁸ | `3.14` |
| `String` | heap | arbitrary | `"hello"` |
| `Bool` | 1-bit | true/false | `true` |
| `Char` | 4 bytes | Unicode | `'a'` |
| `Null` | 0 bytes | null | `null` |

**Compound types**:
| Type | Syntax | Semantics |
|------|--------|-----------|
| `Array<T>` | `[1, 2, 3]` | Dynamic array, reference |
| `Map<K, V>` | `{"a": 1}` | Hash map, reference |
| `(T, U)` | `(1, "x")` | Tuple, value |
| `func(T) -> U` | `(x) => x * 2` | Function, reference |

**User types**:
| Type | Semantics |
|------|-----------|
| `struct` | Named fields, value semantics |
| `class` | Named fields, reference semantics, inheritance |
| `enum` | Sum type (algebraic data type) |
| `trait` | Interface with optional default implementations |
| `type` | Type alias |
| `opaque type` | Hidden implementation type |

### 2.3 Type Inference Algorithm (Hindley-Milner)

The type checker uses Algorithm W:

```
1. Assign fresh type variables to all unannotated bindings
2. Generate constraints from expressions
3. Unify constraints using Robinson's algorithm
4. Substitute solved types back
5. Report any unsolved constraints as type errors
```

**Examples**:
```nx
// Constraint generation
let x = 10                    // x: α, α = Int
let y = x + 5                 // α = Int, y: Int
let z = y > 0                 // z: Bool
let f = (a) => a + 1          // f: Int -> Int
let g = (a, b) => a + b       // g: Int -> Int -> Int
let h = (a, b) => a == b      // h: α -> α -> Bool (polymorphic)
```

### 2.4 Generic Type Parameters

```nx
// Function with generic type parameter
func identity<T>(x: T) -> T = x

// Multiple type parameters
func pair<A, B>(a: A, b: B) -> (A, B) = (a, b)

// Type constraints
func sort<T: Comparable>(arr: Array<T>) -> Array<T> { ... }
func serialize<T: Serializable>(item: T) -> String { ... }
func clone<T: Clone>(item: T) -> T { ... }

// Multiple bounds
func process<T: Clone + Debug>(item: T) { ... }

// Where clauses (complex bounds)
func transform<T, U>(items: Array<T>) -> Array<U>
where
    T: Convertible<U>
    U: Default
{ ... }
```

### 2.5 Algebraic Data Types

```nx
// Sum type
enum Shape {
    Circle(Float)
    Rectangle(Float, Float)
    Polygon(Array<(Float, Float)>)
}

// With named fields
enum Expr {
    Literal(Int)
    Add { left: Box<Expr>, right: Box<Expr> }
    Multiply { left: Box<Expr>, right: Box<Expr> }
}

// Product type (struct)
struct Point {
    x: Float
    y: Float
}

// Nested types
type Matrix = Array<Array<Float>>
type Graph = Map<String, Array<String>>
type Callback = func(Int, String) -> Bool
```

### 2.6 Standard Generic Types

```nx
// Option<T> — nullable value wrapper
enum Option<T> {
    Some(T)
    None
}

let maybe_name: Option<String> = Some("Alice")
let no_name: Option<String> = None

// Unwrapping
match maybe_name {
    Some(name) => print(name),
    None => print("no name"),
}

// Chaining
let upper = maybe_name
    .map(|name| name.to_upper())
    .unwrap_or("UNKNOWN")

// Result<T, E> — error handling
enum Result<T, E> {
    Ok(T)
    Err(E)
}

func parse_int(s: String) -> Result<Int, String> {
    match s.parse() {
        Ok(n) => Ok(n),
        Err(_) => Err("Invalid integer"),
    }
}

// Propagation with ?
func process() -> Result<Int, String> {
    let n = parse_int("42")?
    return Ok(n * 2)
}
```

### 2.7 Type Aliases

```nx
type UserId = Int
type Timestamp = Int
type JsonValue = Map<String, Any>
type Handler = func(Request) -> Response

// Opaque types (hide implementation)
opaque type Secret = String
opaque type Hash = Array<Int>

// Opaque types prevent mixing
let s1: Secret = "password"
let s2: Hash = [1, 2, 3]
// s1 == s2  // ERROR: different types
```

---

## 3. Error Handling

### 3.1 Philosophy

- **No exceptions for control flow** — use `Result<T, E>` for recoverable errors
- **`?` operator** for ergonomic propagation
- **`try/catch`** only for external errors (IO, network, FFI)
- **`panic`** for unrecoverable errors (like Rust's `panic!`)
- **No null** — use `Option<T>` instead

### 3.2 Error Types

```nx
// Built-in error enum
enum Error {
    Type(String)
    Undefined(String)
    Argument { expected: Int, found: Int }
    IndexOutOfBounds(String)
    DivisionByZero
    FileNotFound(String)
    IoError(String)
    NetworkError(String)
    ParseError(String)
    Custom(String, Map<String, Any>)
}

// Custom error types (extend enum)
enum AppError {
    NotFound(String)
    Unauthorized
    RateLimited { retry_after: Int }
    ValidationError { field: String, message: String }
}
```

### 3.3 Propagation

```nx
// ? operator — unwrap Ok or return Err
func read_config(path: String) -> Result<Config, AppError> {
    let content = read_file(path).map_err(
        |e| AppError::NotFound(path)
    )?
    let data = json_parse(content).map_err(
        |e| AppError::ValidationError {
            field: "config".to_string(),
            message: e.to_string(),
        }
    )?
    return Ok(Config::from(data))
}

// Map errors
func process() -> Result<Int, String> {
    let n = parse_int("42")
        .map_err(|e| "Failed to parse: ${e}")?
    return Ok(n + 1)
}

// or_else — try alternative
func get_config() -> Result<Config, AppError> {
    read_config("config.json")
        .or_else(|_| read_config("config.default.json"))
}
```

### 3.4 try/catch

```nx
// Only for external errors
try {
    let file = open_file("data.txt")
    let content = file.read_all()
    process(content)
} catch (e: IoError) {
    log("IO error: ${e.message}")
    recover()
} catch (e) {
    log("Unexpected error: ${e}")
    panic("Cannot continue")
} finally {
    cleanup()
}
```

### 3.5 Error Chains

```nx
// Wrap errors with context
func fetch(url: String) -> Result<Response, AppError> {
    http_get(url).map_err(|e| {
        AppError::NetworkError("Failed to fetch ${url}: ${e}")
    })?
}

// Source tracking (automatic)
// When errors propagate, the language tracks the chain:
//   AppError::NetworkError("Failed to fetch...")
//     → caused by: reqwest::Error("connection refused")
//       → caused by: std::io::Error(ErrorKind::ConnectionRefused)
```

### 3.6 Panic

```nx
// For unrecoverable errors
func divide(a: Int, b: Int) -> Int {
    if b == 0 {
        panic("Division by zero")
    }
    a / b
}

// assert! macro (panics on failure)
assert!(x > 0, "x must be positive, got ${x}")

// unwrap or panic
let value = maybe_value.expect("Expected value to be present")
```

---

## 4. Compiler Architecture

### 4.1 Pipeline Overview

```
Source Code (.nx)
   ↓
Lexer → Tokens
   ↓
Parser → AST (untyped)
   ↓
Semantic Analyzer → Annotated AST
   ↓
Type Checker → Typed AST
   ↓
IR Generator → SSA-based IR (Nexora IR / NIR)
   ↓
Optimizer → Optimized NIR
   ↓
Bytecode Compiler → Bytecode (.nxc)
   ↓
Nexora Virtual Machine → Execution
```

### 4.2 Lexer

**Token format**:
```rust
pub struct Token {
    pub kind: TokenKind,
    pub span: Span,
}

pub struct Span {
    pub line: u32,
    pub col: u32,
    pub start: u32,
    pub end: u32,
}
```

**Key behaviors**:
- Whitespace-independent (significant newlines only in REPL mode)
- String interpolation handled at lex time: `"hello ${name}"` → `[Text("hello "), InterpStart, Ident("name"), InterpEnd]`
- Unicode support in identifiers and strings
- Error recovery: replace bad chars with `\0` token, continue lexing

### 4.3 Parser

**Strategy**: Pratt parser (top-down operator precedence) for expressions, recursive descent for statements.

**AST structure**:
```rust
pub enum Expr {
    Literal(Literal),
    Ident(Ident),
    Binary { op: BinOp, left: Box<Expr>, right: Box<Expr> },
    Unary { op: UnaryOp, expr: Box<Expr> },
    Call { callee: Box<Expr>, args: Vec<Expr> },
    Index { object: Box<Expr>, index: Box<Expr> },
    Property { object: Box<Expr>, prop: Ident },
    Lambda { params: Vec<Param>, body: Box<Expr> },
    Match { scrutinee: Box<Expr>, arms: Vec<MatchArm> },
    // ... etc
}

pub enum Stmt {
    Let { name: Pat, type_ann: Option<TypeExpr>, value: Expr },
    Func { name: Ident, params: Vec<Param>, return_type: Option<TypeExpr>, body: Block },
    If { condition: Expr, then_body: Block, elifs: Vec<(Expr, Block)>, else_body: Option<Block> },
    While { condition: Expr, body: Block },
    For { pat: Pat, iterable: Expr, body: Block },
    Return(Option<Expr>),
    // ... etc
}

pub enum Pat {
    Wildcard,
    Ident(Ident),
    Literal(Literal),
    Tuple(Vec<Pat>),
    Array(Vec<Pat>),
    Object(Vec<(Ident, Pat)>),
    Or(Vec<Pat>),
    Guard { pattern: Box<Pat>, guard: Expr },
}
```

**Error recovery**:
- On unexpected token, synthesize missing tokens
- Skip to next statement boundary (`;` or `}`)
- Report all errors, don't stop at first

### 4.4 Semantic Analyzer

**Responsibilities**:
1. Name resolution (scope resolution, `use` checking)
2. Label resolution (`break`/`continue` targets)
3. Const evaluation (compile-time constant checking)
4. Export/import validation

**Output**: `AnnotatedAST` — AST with resolved names and scope information.

### 4.5 Type Checker

**Algorithm**: Hindley-Milner with extensions for:
- Row polymorphism (for records/objects)
- Subtyping (for class inheritance)
- Higher-kinded types (for traits)
- GADTs (future)

**Type environment**:
```rust
pub struct TypeEnv {
    parent: Option<Box<TypeEnv>>,
    bindings: HashMap<Ident, Type>,
    type_bindings: HashMap<Ident, TypeDef>,
}
```

**Output**: `TypedAST` — AST with type information on every node.

### 4.6 Intermediate Representation (SSA-based NIR)

**Design**: Three-address code in SSA form.

```
// Source: let x = a + b * c
// NIR:
  %1 = Load a
  %2 = Load b
  %3 = Load c
  %4 = Mul %2, %3
  %5 = Add %1, %4
  Store x, %5
```

**Key properties**:
- Every variable defined exactly once (SSA property)
- φ-functions for control flow joins
- No nested expressions
- Easy to optimize

**NIR Instructions**:
```
// Arithmetic
%result = Add %a, %b
%result = Sub %a, %b
%result = Mul %a, %b
%result = Div %a, %b
%result = Mod %a, %b
%result = Pow %a, %b
%result = Neg %a

// Memory
%val = Load %slot
Store %slot, %val
%val = LoadGlobal "name"
StoreGlobal "name", %val

// Control flow
Branch %cond, %true_label, %false_label
Jump %label
%label:  (phi function)
%result = Phi [(val1, block1), (val2, block2)]

// Functions
%result = Call %func, [%arg1, %arg2]
Return %val

// Objects
%field = GetField %obj, "name"
%obj = SetField %obj, "name", %val
%elem = GetIndex %arr, %idx
%arr = SetIndex %arr, %idx, %val

// Allocation
%obj = AllocObject "ClassName"
%arr = AllocArray %len
```

### 4.7 Optimizer

**Passes** (in order):

1. **Constant Folding** — evaluate `2 + 3` → `5` at compile time
2. **Dead Code Elimination** — remove unreachable code
3. **Constant Propagation** — substitute known constant values
4. **Copy Propagation** — eliminate redundant copies
5. **Common Subexpression Elimination** — avoid recomputing `a + b`
6. **Strength Reduction** — `x * 2` → `x << 1`
7. **Inlining** — inline small functions
8. **Tail Call Optimization** — optimize tail-recursive calls
9. **Loop Invariant Code Motion** — hoist loop-invariant computations
10. **Dead Store Elimination** — remove unused stores

### 4.8 Bytecode Format

**Recommendation**: Stack-based bytecode (simpler compiler, adequate performance).

**Bytecode header**:
```rust
pub struct BytecodeModule {
    pub magic: u32,           // 0x4E585243 ("NXRC")
    pub version: u16,
    pub constants: Vec<Value>,
    pub functions: Vec<Function>,
    pub globals: Vec<String>,
    pub classes: Vec<ClassDef>,
}

pub struct Function {
    pub name: String,
    pub arity: u8,
    pub upvalue_count: u8,
    pub bytecode: Vec<u8>,
    pub lines: Vec<(u32, u32)>,  // debug line info
}
```

**Instruction format**:
```
[opcode: u8] [operand1: u8] [operand2: u8]
```
Or for larger operands:
```
[opcode: u8] [EXTEND] [operand: u16]
```

### 4.9 Virtual Machine

**Architecture**: Stack-based register VM

**Components**:
- **Stack**: Fixed-size value stack (1024 entries)
- **Call frames**: Frame pointer + return address + closure
- **Globals**: Global variable table
- **Heap**: Object allocation (mark-sweep GC)

```rust
pub struct VM {
    stack: Vec<Value>,
    frames: Vec<CallFrame>,
    globals: HashMap<String, Value>,
    heap: Heap,
    ip: usize,
}

pub struct CallFrame {
    closure: Closure,
    ip: usize,
    slots: usize,  // stack pointer at frame start
}
```

**Garbage Collection**: Mark-sweep with generational collection (young gen for short-lived objects, old gen for long-lived).

### 4.10 Incremental Compilation for REPL

```
1. Parse new input as expression or statement
2. If incomplete, prompt for more input
3. Type-check against previous environment
4. Compile to bytecode
5. Execute in existing VM state
6. Update environment with new bindings
```

---

## 5. Standard Library

### 5.1 Module Structure

```
nexora.core       — print, input, type_of, assert, len, range, str, int, float
nexora.math       — sqrt, pow, abs, floor, ceil, round, sin, cos, tan, log, ln, PI, E, random
nexora.string     — upper, lower, trim, split, join, contains, replace, starts_with, ends_with, 
                     repeat, char_at, to_chars, lines, chars, bytes, parse_int, parse_float
nexora.collection — map, filter, reduce, sort, reverse, unique, flatten, zip, 
                     keys, values, entries, find, some, every, includes, concat
nexora.fs         — read_file, write_file, append_file, file_exists, read_dir, create_dir,
                     remove_file, copy_file, rename, metadata
nexora.http       — get, post, put, delete, patch, serve, request, Response, Request
nexora.json       — parse, stringify, pretty_print
nexora.net        — TcpListener, TcpStream, UdpSocket, connect, bind
nexora.os         — env, set_env, exec_command, current_dir, home_dir, platform,
                     cpu_count, memory_info
nexora.time       — now, timestamp, sleep, duration, Instant, DateTime, Timer
nexora.regex      — Regex::new, is_match, find, find_all, replace, split
nexora.testing    — test, describe, it, expect, assert_eq, assert_ne, assert_throws
nexora.crypto     — hash_md5, hash_sha256, hash_sha512, hmac, encrypt_aes, decrypt_aes
nexora.datetime   — DateTime::now, DateTime::parse, add_days, add_hours, format, to_iso
```

### 5.2 Module: `nexora.core`

```nx
module nexora.core {
    // Print to stdout with newline
    pub func print(..args: Array<Any>)

    // Print to stdout without newline
    pub func print_raw(..args: Array<Any>)

    // Read line from stdin
    pub func input(prompt: String = "") -> String

    // Type of value (runtime)
    pub func type_of(value: Any) -> String

    // Assert condition (panics on failure)
    pub func assert(condition: Bool, message: String = "Assertion failed")

    // Length of collection/string
    pub func len(collection: Any) -> Int

    // Generate range
    pub func range(..args: Array<Int>) -> Array<Int>

    // Type conversions
    pub func str(value: Any) -> String
    pub func int(value: Any) -> Int
    pub func float(value: Any) -> Float
    pub func bool(value: Any) -> Bool

    // Iteration
    pub func enumerate<T>(arr: Array<T>) -> Array<(Int, T)>
    pub func reversed<T>(arr: Array<T>) -> Array<T>

    // Comparison
    pub func min(a: Any, b: Any) -> Any
    pub func max(a: Any, b: Any) -> Any

    // Identity check
    pub func is_null(value: Any) -> Bool
    pub func is_int(value: Any) -> Bool
    pub func is_float(value: Any) -> Bool
    pub func is_string(value: Any) -> Bool
    pub func is_bool(value: Any) -> Bool
    pub func is_array(value: Any) -> Bool
    pub func is_map(value: Any) -> Bool
    pub func is_function(value: Any) -> Bool
}
```

### 5.3 Module: `nexora.math`

```nx
module nexora.math {
    pub const PI: Float = 3.141592653589793
    pub const E: Float = 2.718281828459045
    pub const TAU: Float = 6.283185307179586
    pub const INFINITY: Float = Infinity
    pub const NAN: Float = NaN

    pub func sqrt(x: Float) -> Float
    pub func pow(base: Float, exp: Float) -> Float
    pub func abs(x: Any) -> Any           // works for Int and Float
    pub func floor(x: Float) -> Int
    pub func ceil(x: Float) -> Int
    pub func round(x: Float) -> Int
    pub func sin(x: Float) -> Float
    pub func cos(x: Float) -> Float
    pub func tan(x: Float) -> Float
    pub func asin(x: Float) -> Float
    pub func acos(x: Float) -> Float
    pub func atan(x: Float) -> Float
    pub func atan2(y: Float, x: Float) -> Float
    pub func log(x: Float) -> Float       // natural log
    pub func log2(x: Float) -> Float
    pub func log10(x: Float) -> Float
    pub func exp(x: Float) -> Float
    pub func sign(x: Any) -> Int          // -1, 0, or 1
    pub func gcd(a: Int, b: Int) -> Int
    pub func lcm(a: Int, b: Int) -> Int
    pub func factorial(n: Int) -> Int
    pub func fibonacci(n: Int) -> Int

    // Random
    pub func random() -> Float                          // 0.0..1.0
    pub func random_int(min: Int, max: Int) -> Int      // min..max
    pub func random_choice<T>(arr: Array<T>) -> T
    pub func shuffle<T>(arr: Array<T>) -> Array<T>

    // Clamping
    pub func clamp(value: Any, min: Any, max: Any) -> Any

    // Linear interpolation
    pub func lerp(a: Float, b: Float, t: Float) -> Float
}
```

### 5.4 Module: `nexora.string`

```nx
module nexora.string {
    pub func length(s: String) -> Int
    pub func upper(s: String) -> String
    pub func lower(s: String) -> String
    pub func trim(s: String) -> String
    pub func trim_start(s: String) -> String
    pub func trim_end(s: String) -> String
    pub func contains(s: String, needle: String) -> Bool
    pub func starts_with(s: String, prefix: String) -> Bool
    pub func ends_with(s: String, suffix: String) -> Bool
    pub func replace(s: String, from: String, to: String) -> String
    pub func replace_all(s: String, from: String, to: String) -> String
    pub func split(s: String, delimiter: String) -> Array<String>
    pub func join(arr: Array<String>, delimiter: String) -> String
    pub func char_at(s: String, index: Int) -> String
    pub func chars(s: String) -> Array<String>
    pub func bytes(s: String) -> Array<Int>
    pub func lines(s: String) -> Array<String>
    pub func repeat(s: String, count: Int) -> String
    pub func reverse(s: String) -> String
    pub func index_of(s: String, needle: String) -> Int   // -1 if not found
    pub func last_index_of(s: String, needle: String) -> Int
    pub func substring(s: String, start: Int, end: Int) -> String
    pub func pad_start(s: String, length: Int, char: String = " ") -> String
    pub func pad_end(s: String, length: Int, char: String = " ") -> String
    pub func matches(s: String, pattern: String) -> Bool
    pub func parse_int(s: String) -> Result<Int, String>
    pub func parse_float(s: String) -> Result<Float, String>
    pub func is_empty(s: String) -> Bool
    pub func to_bytes(s: String) -> Array<Int>
    pub func from_bytes(bytes: Array<Int>) -> String
}
```

### 5.5 Module: `nexora.collection`

```nx
module nexora.collection {
    pub func map<T, U>(arr: Array<T>, f: func(T) -> U) -> Array<U>
    pub func filter<T>(arr: Array<T>, f: func(T) -> Bool) -> Array<T>
    pub func reduce<T, U>(arr: Array<T>, f: func(U, T) -> U, init: U) -> U
    pub func sort<T: Comparable>(arr: Array<T>) -> Array<T>
    pub func sort_by<T>(arr: Array<T>, key: func(T) -> Comparable) -> Array<T>
    pub func reverse<T>(arr: Array<T>) -> Array<T>
    pub func unique<T>(arr: Array<T>) -> Array<T>
    pub func unique_by<T>(arr: Array<T>, key: func(T) -> Any) -> Array<T>
    pub func flatten<T>(arr: Array<Array<T>>) -> Array<T>
    pub fn flat_map<T, U>(arr: Array<T>, f: func(T) -> Array<U>) -> Array<U>
    pub func zip<T, U>(a: Array<T>, b: Array<U>) -> Array<(T, U)>
    pub func unzip<T, U>(arr: Array<(T, U)>) -> (Array<T>, Array<U>)
    pub func find<T>(arr: Array<T>, f: func(T) -> Bool) -> Option<T>
    pub func find_index<T>(arr: Array<T>, f: func(T) -> Bool) -> Int
    pub func some<T>(arr: Array<T>, f: func(T) -> Bool) -> Bool
    pub func every<T>(arr: Array<T>, f: func(T) -> Bool) -> Bool
    pub func includes<T>(arr: Array<T>, item: T) -> Bool
    pub func concat<T>(a: Array<T>, b: Array<T>) -> Array<T>
    pub func take<T>(arr: Array<T>, n: Int) -> Array<T>
    pub func drop<T>(arr: Array<T>, n: Int) -> Array<T>
    pub func chunk<T>(arr: Array<T>, size: Int) -> Array<Array<T>>
    pub func partition<T>(arr: Array<T>, f: func(T) -> Bool) -> (Array<T>, Array<T>)
    pub func group_by<T, K>(arr: Array<T>, key: func(T) -> K) -> Map<K, Array<T>>

    // Map operations
    pub func map_keys<K, V>(m: Map<K, V>, f: func(K) -> K2) -> Map<K2, V>
    pub func map_values<K, V, V2>(m: Map<K, V>, f: func(V) -> V2) -> Map<K, V2>
    pub func merge<K, V>(a: Map<K, V>, b: Map<K, V>) -> Map<K, V>
}
```

### 5.6 Module: `nexora.fs`

```nx
module nexora.fs {
    pub func read(path: String) -> Result<String, IoError>
    pub func write(path: String, content: String) -> Result<Bool, IoError>
    pub func append(path: String, content: String) -> Result<Bool, IoError>
    pub func exists(path: String) -> Bool
    pub func read_dir(path: String) -> Result<Array<String>, IoError>
    pub func create_dir(path: String) -> Result<Bool, IoError>
    pub func create_dir_all(path: String) -> Result<Bool, IoError>
    pub func remove_file(path: String) -> Result<Bool, IoError>
    pub func remove_dir(path: String) -> Result<Bool, IoError>
    pub func copy(from: String, to: String) -> Result<Bool, IoError>
    pub func rename(from: String, to: String) -> Result<Bool, IoError>
    pub func metadata(path: String) -> Result<Metadata, IoError>
    pub func read_bytes(path: String) -> Result<Array<Int>, IoError>
    pub func write_bytes(path: String, bytes: Array<Int>) -> Result<Bool, IoError>

    struct Metadata {
        size: Int
        is_file: Bool
        is_dir: Bool
        modified: DateTime
    }
}
```

### 5.7 Module: `nexora.http`

```nx
module nexora.http {
    pub func get(url: String, headers: Map<String, String> = {}) -> Result<Response, HttpError>
    pub func post(url: String, body: String, headers: Map<String, String> = {}) -> Result<Response, HttpError>
    pub func put(url: String, body: String, headers: Map<String, String> = {}) -> Result<Response, HttpError>
    pub func delete(url: String, headers: Map<String, String> = {}) -> Result<Response, HttpError>
    pub func patch(url: String, body: String, headers: Map<String, String> = {}) -> Result<Response, HttpError>

    pub func request(config: RequestConfig) -> Result<Response, HttpError>

    pub func serve(port: Int, handler: func(Request) -> Response)

    struct Response {
        status: Int
        headers: Map<String, String>
        body: String
    }

    struct Request {
        method: String
        path: String
        headers: Map<String, String>
        body: String
    }

    struct RequestConfig {
        method: String
        url: String
        headers: Map<String, String>
        body: Option<String>
        timeout: Option<Int>
    }
}
```

### 5.8 Module: `nexora.json`

```nx
module nexora.json {
    pub func parse(s: String) -> Result<Any, JsonError>
    pub fn stringify(value: Any, pretty: Bool = false) -> String
    pub func pretty(value: Any) -> String
    pub func validate(s: String) -> Bool

    // Type-safe access
    pub func get<T>(json: Any, path: String) -> Option<T>
    pub func set(json: Any, path: String, value: Any) -> Any
}
```

### 5.9 Module: `nexora.time`

```nx
module nexora.time {
    pub func now() -> Int                         // Unix timestamp in seconds
    pub func now_ms() -> Int                      // Unix timestamp in milliseconds
    pub func now_us() -> Int                      // Unix timestamp in microseconds
    pub func timestamp() -> Float                 // High-precision timestamp
    pub func sleep(ms: Int)                       // Sleep for milliseconds
    pub func sleep_ms(ms: Int)                    // Sleep for milliseconds
    pub func sleep_secs(secs: Float)              // Sleep for seconds

    struct Instant {
        pub func elapsed() -> Duration
        pub func duration_since(other: Instant) -> Duration
    }

    struct Duration {
        secs: Int
        nanos: Int

        pub func from_secs(secs: Int) -> Duration
        pub func from_millis(ms: Int) -> Duration
        pub func from_micros(us: Int) -> Duration
        pub func as_secs() -> Int
        pub func as_millis() -> Int
        pub func add(other: Duration) -> Duration
        pub func subtract(other: Duration) -> Duration
    }

    struct DateTime {
        year: Int
        month: Int
        day: Int
        hour: Int
        minute: Int
        second: Int
        timezone: String

        pub func now() -> DateTime
        pub func parse(s: String, format: String) -> Result<DateTime, String>
        pub func format(template: String) -> String
        pub func to_iso() -> String
        pub fn add_days(n: Int) -> DateTime
        pub fn add_hours(n: Int) -> DateTime
        pub fn add_minutes(n: Int) -> DateTime
        pub fn diff(other: DateTime) -> Duration
    }
}
```

### 5.10 Module: `nexora.crypto`

```nx
module nexora.crypto {
    pub func hash_md5(data: String) -> String
    pub func hash_sha256(data: String) -> String
    pub func hash_sha512(data: String) -> String
    pub func hmac_sha256(key: String, data: String) -> String
    pub func encrypt_aes(data: String, key: String) -> Result<String, CryptoError>
    pub func decrypt_aes(data: String, key: String) -> Result<String, CryptoError>
    pub func random_bytes(n: Int) -> Array<Int>
    pub func random_hex(n: Int) -> String
    pub func base64_encode(data: String) -> String
    pub func base64_decode(data: String) -> Result<String, CryptoError>
}
```

### 5.11 Module: `nexora.testing`

```nx
module nexora.testing {
    pub func test(name: String, body: func() -> Any)
    pub func describe(name: String, body: func() -> Any)
    pub func it(name: String, body: func() -> Any)

    // Assertions
    pub func assert_eq<T: Equal>(actual: T, expected: T)
    pub func assert_ne<T: Equal>(actual: T, expected: T)
    pub func assert_throws(body: func() -> Any, error_type: String = "")
    pub func assert_includes(collection: Any, item: Any)
    pub func assert_match(value: Any, pattern: Any)

    // Custom matchers
    pub struct Expect<T> {
        value: T
    }

    pub func expect<T>(value: T) -> Expect<T>
}
```

---

## 6. Package Manager (nxm)

### 6.1 Project Structure

```
my-project/
├── nexora.json           # Project manifest (like Cargo.toml)
├── nxm-lock.json         # Lockfile (pinned versions)
├── src/
│   └── main.nx           # Entry point
├── lib/
│   └── utils.nx          # Library modules
├── tests/
│   └── test_main.nx      # Test files
└── nexora_modules/       # Local dependencies
    └── nexora-http/
        └── ...
```

### 6.2 `nexora.json` (Manifest)

```json
{
  "name": "my-project",
  "version": "1.0.0",
  "description": "A Nexora project",
  "author": "Alice <alice@example.com>",
  "license": "MIT",
  "repository": "https://github.com/alice/my-project",

  "entry": "src/main.nx",

  "dependencies": {
    "nexora-http": "^1.0.0",
    "nexora-json": "^2.1.0",
    "nexora-db": "github:user/repo"
  },

  "dev_dependencies": {
    "nexora-test": "^1.0.0"
  },

  "scripts": {
    "dev": "nx run src/main.nx --watch",
    "test": "nx test tests/",
    "build": "nx build --release",
    "lint": "nx lint src/",
    "fmt": "nx fmt src/"
  },

  "workspaces": [
    "packages/*"
  ],

  "engines": {
    "nexora": ">=0.8.0"
  }
}
```

### 6.3 `nxm-lock.json` (Lockfile)

```json
{
  "nexora": "0.8.0",
  "packages": {
    "nexora-http": {
      "version": "1.2.3",
      "resolved": "https://registry.nexora.dev/nexora-http-1.2.3.nxz",
      "integrity": "sha256:abc123...",
      "dependencies": {
        "nexora-net": "^1.0.0"
      }
    }
  }
}
```

### 6.4 Dependency Resolution

**Algorithm**: SAT-solver with version constraints

1. Parse all `nexora.json` manifests
2. Build dependency graph
3. Check for cycles
4. Resolve version constraints using semver ranges
5. Pick highest compatible versions
6. Write `nxm-lock.json`
7. Download and extract packages

**Version ranges**:
```
^1.0.0    → >=1.0.0, <2.0.0
~1.2.0    → >=1.2.0, <1.3.0
1.2.3     → exact version
>=1.0.0   → at least 1.0.0
*         → any version
```

### 6.5 Commands

```bash
nxm init                    # Create nexora.json interactively
nxm add <package>           # Add dependency
nxm add <package>@<version> # Add specific version
nxm add <package>@<tag>     # Add by tag (latest, beta, etc.)
nxm remove <package>        # Remove dependency
nxm install                 # Install all dependencies
nxm update                  # Update all dependencies
nxm update <package>        # Update specific package
nxm search <query>          # Search registry
nxm publish                 # Publish to registry
nxm info <package>          # Show package info
nxm list                    # List installed packages
nxm outdated                # Show outdated packages
nxm run <script>            # Run package script
```

---

## 7. CLI Design

### 7.1 Command Reference

```
nx new <name>               # Create new project
nx init                     # Initialize project in current dir
nx run <file.nx>            # Run a Nexora file
nx run --watch <file.nx>    # Run with file watching
nx build                    # Build project
nx build --release          # Build optimized release
nx test                     # Run all tests
nx test <file.nx>           # Run specific test file
nx fmt                      # Format all source files
nx fmt --check              # Check formatting without modifying
nx lint                     # Lint all source files
nx lint --fix               # Auto-fix lint issues
nx doc                      # Generate documentation
nx doc --serve              # Serve docs locally
nx add <package>            # Add dependency
nx remove <package>         # Remove dependency
nx update                   # Update all dependencies
nx repl                     # Start interactive REPL
nx repl --history <file>    # REPL with history file
nx doctor                   # Diagnose environment issues
nx publish                  # Publish package
nx search <query>           # Search packages
nx info <package>           # Package info
nx fmt --diff               # Show formatting diff
nx version                  # Show Nexora version
nx version --bump <part>    # Bump version (major/minor/patch)
```

### 7.2 REPL

```
nexora v0.8.0
Type .help for commands, .exit to quit

nx> let x = 10
nx> x + 5
15
nx> func add(a, b) = a + b
nx> add(3, 4)
7
nx> .help
Available commands:
  .help     Show this help
  .exit     Exit REPL
  .history  Show command history
  .clear    Clear screen
  .reset    Reset REPL state
  .load     Load file into REPL
  .save     Save session to file
  .type     Show type of expression
  .debug    Toggle debug mode

nx> .type [1, 2, 3]
Array<Int>

nx> .load examples/hello.nx
Loaded examples/hello.nx

nx> .save session.nx
Session saved to session.nx
```

---

## 8. Developer Tooling

### 8.1 Formatter

**Rules**:
- 4-space indentation (configurable)
- Trailing commas in multi-line collections
- No trailing whitespace
- Single blank line between functions
- Consistent brace placement (opening brace on same line)
- Max line length: 100 characters (configurable)

**Configuration** (`.nexora-format`):
```json
{
  "indent_size": 4,
  "indent_style": "space",
  "max_line_length": 100,
  "trailing_commas": true,
  "brace_style": "same_line"
}
```

### 8.2 Linter

**Rules** (grouped by category):

**Style**:
- `no-unused-vars` — Warn on unused variables
- `no-unused-imports` — Warn on unused imports
- `consistent-naming` — Enforce snake_case for vars/funcs
- `max-line-length` — Error on lines over limit
- `no-trailing-whitespace` — Error on trailing whitespace

**Errors**:
- `unused-result` — Warn on unused return values
- `unused-mutable` — Warn on mutable vars that aren't mutated
- `infinite-recursion` — Detect direct recursion without base case
- `type-mismatch` — Report type errors (from type checker)

**Best practices**:
- `prefer-const` — Suggest `let` → `const` when never reassigned
- `no-shadowing` — Warn when variable shadows outer scope
- `explicit-return` — Require explicit return in functions
- `no-empty-catch` — Warn on empty catch blocks

**Configuration** (`.nexora-lint`):
```json
{
  "rules": {
    "no-unused-vars": "warn",
    "consistent-naming": "error",
    "max-line-length": ["error", 100],
    "prefer-const": "info"
  },
  "ignore": ["nexora_modules/", "target/"]
}
```

### 8.3 Language Server Protocol (LSP)

**Features**:
- **Autocomplete** — Context-aware suggestions
- **Go to definition** — Jump to symbol definition
- **Find references** — Find all uses of a symbol
- **Hover** — Show type/value on hover
- **Signature help** — Show function signature while typing
- **Diagnostics** — Real-time errors and warnings
- **Rename** — Rename symbol across project
- **Code actions** — Quick fixes, refactorings
- **Formatting** — On-save formatting
- **Document symbols** — Outline of file
- **Workspace symbols** — Search project symbols
- **Call hierarchy** — Who calls this / what does this call
- **Inlay hints** — Show inferred types inline
- **Semantic tokens** — Enhanced syntax highlighting

### 8.4 Debugger

**Features**:
- **Breakpoints** — Line and conditional breakpoints
- **Step over/into/out** — Step through code
- **Variable inspection** — View local/global variables
- **Watch expressions** — Monitor expressions
- **Call stack** — View execution stack
- **REPL integration** — Evaluate in debug REPL
- **Exception breakpoints** — Break on specific error types
- **Memory inspection** — View heap objects

### 8.5 Profiler

**Features**:
- **CPU profiling** — Function-level timing
- **Memory profiling** — Allocation tracking
- **GC profiling** — Garbage collection stats
- **Call graph** — Visual call graph
- **Flame graph** — Generate flame graphs
- **Benchmark mode** — Run benchmarks with stats

### 8.6 Documentation Generator

**Features**:
- Parse doc comments (`///`)
- Generate HTML docs
- API reference
- Module index
- Type hierarchy
- Example code extraction
- Cross-references

---

## 9. Package Registry

### 9.1 REST API

```
GET    /api/v1/packages                  # List packages
GET    /api/v1/packages/:name            # Get package info
GET    /api/v1/packages/:name/versions   # List versions
GET    /api/v1/packages/:name/:version   # Get version info
GET    /api/v1/packages/:name/:version/download  # Download package
POST   /api/v1/packages                  # Create package
PUT    /api/v1/packages/:name/:version   # Update version
DELETE /api/v1/packages/:name/:version   # Yank version

GET    /api/v1/search?q=:query           # Search packages
GET    /api/v1/categories                # List categories
GET    /api/v1/packages/:name/reviews    # Package reviews
POST   /api/v1/packages/:name/reviews    # Add review

POST   /api/v1/auth/register             # Register
POST   /api/v1/auth/login                # Login
POST   /api/v1/auth/token                # Generate API token

GET    /api/v1/users/:username           # User profile
GET    /api/v1/users/:username/packages  # User's packages
```

### 9.2 Package Format

```
package-name-version.nxz (compressed archive)
├── nexora.json
├── src/
│   └── index.nx
├── README.md
└── LICENSE
```

### 9.3 Security

- Package signing with GPG
- SHA-256 integrity checksums
- Automated malware scanning
- Vulnerability database (CVE-like)
- Dependency audit
- Trusted publishers

### 9.4 Statistics

- Download counts (per version, per day)
- Dependency count
- Health score (based on tests, docs, activity)
- Maintenance status
- License compliance

---

## 10. Rust Implementation Folder Structure

```
nexora/
├── Cargo.toml                    # Workspace root
├── BLUEPRINT.md                  # This document
├── README.md
│
├── crates/
│   ├── nexora-lexer/             # Lexer
│   │   ├── Cargo.toml
│   │   └── src/
│   │       ├── lib.rs
│   │       ├── token.rs          # Token types
│   │       ├── lexer.rs          # Lexer implementation
│   │       ├── span.rs           # Source span tracking
│   │       └── tests/
│   │           └── lexer_tests.rs
│   │
│   ├── nexora-parser/            # Parser (Pratt + recursive descent)
│   │   ├── Cargo.toml
│   │   └── src/
│   │       ├── lib.rs
│   │       ├── ast.rs            # AST node definitions
│   │       ├── parser.rs         # Main parser
│   │       ├── expr.rs           # Expression parsing (Pratt)
│   │       ├── stmt.rs           # Statement parsing
│   │       ├── pattern.rs        # Pattern parsing
│   │       ├── type_expr.rs      # Type expression parsing
│   │       ├── recovery.rs       # Error recovery
│   │       └── tests/
│   │           └── parser_tests.rs
│   │
│   ├── nexora-semantic/          # Semantic analyzer
│   │   ├── Cargo.toml
│   │   └── src/
│   │       ├── lib.rs
│   │       ├── scope.rs          # Scope management
│   │       ├── resolver.rs       # Name resolution
│   │       ├── label.rs          # Label resolution
│   │       └── tests/
│   │           └── semantic_tests.rs
│   │
│   ├── nexora-types/             # Type system
│   │   ├── Cargo.toml
│   │   └── src/
│   │       ├── lib.rs
│   │       ├── types.rs          # Type definitions
│   │       ├── type_env.rs       # Type environment
│   │       ├── inference.rs      # HM type inference
│   │       ├── unify.rs          # Unification algorithm
│   │       ├── constraints.rs    # Constraint generation
│   │       ├── traits.rs         # Trait resolution
│   │       └── tests/
│   │           └── type_tests.rs
│   │
│   ├── nexora-ir/                # SSA intermediate representation
│   │   ├── Cargo.toml
│   │   └── src/
│   │       ├── lib.rs
│   │       ├── ir.rs             # IR instruction definitions
│   │       ├── builder.rs        # IR builder
│   │       ├── function.rs       # IR function representation
│   │       ├── module.rs         # IR module representation
│   │       ├── from_ast.rs       # AST → IR lowering
│   │       └── tests/
│   │           └── ir_tests.rs
│   │
│   ├── nexora-optimizer/         # Optimization passes
│   │   ├── Cargo.toml
│   │   └── src/
│   │       ├── lib.rs
│   │       ├── pass.rs           # Pass trait
│   │       ├── constant_fold.rs
│   │       ├── dead_code.rs
│   │       ├── const_prop.rs
│   │       ├── copy_prop.rs
│   │       ├── cse.rs            # Common subexpression elimination
│   │       ├── strength.rs       # Strength reduction
│   │       ├── inline.rs         # Inlining
│   │       ├── tail_call.rs      # TCO
│   │       ├── licm.rs           # Loop invariant code motion
│   │       └── pipeline.rs       # Pass pipeline
│   │
│   ├── nexora-codegen/           # Bytecode generation
│   │   ├── Cargo.toml
│   │   └── src/
│   │       ├── lib.rs
│   │       ├── bytecode.rs       # Bytecode format definitions
│   │       ├── compiler.rs       # IR → bytecode compiler
│   │       ├── constants.rs      # Constant pool
│   │       ├── function.rs       # Function compilation
│   │       └── tests/
│   │           └── codegen_tests.rs
│   │
│   ├── nexora-vm/                # Virtual machine
│   │   ├── Cargo.toml
│   │   └── src/
│   │       ├── lib.rs
│   │       ├── vm.rs             # VM main loop
│   │       ├── opcodes.rs        # Opcode definitions
│   │       ├── stack.rs          # Value stack
│   │       ├── frame.rs          # Call frames
│   │       ├── gc.rs             # Garbage collector
│   │       ├── heap.rs           # Object heap
│   │       ├── value.rs          # Runtime value representation
│   │       ├── builtins.rs       # Built-in functions
│   │       └── tests/
│   │           └── vm_tests.rs
│   │
│   ├── nexora-compiler/          # Orchestrator (ties all crates together)
│   │   ├── Cargo.toml
│   │   └── src/
│   │       ├── lib.rs
│   │       ├── config.rs         # Compiler configuration
│   │       ├── pipeline.rs       # Full compilation pipeline
│   │       ├── incremental.rs    # Incremental compilation for REPL
│   │       └── error.rs          # Compiler error types
│   │
│   └── nexora-stdlib/            # Standard library (implemented in Rust)
│       ├── Cargo.toml
│       └── src/
│           ├── lib.rs
│           ├── core.rs
│           ├── math.rs
│           ├── string.rs
│           ├── collection.rs
│           ├── fs.rs
│           ├── http.rs
│           ├── json.rs
│           ├── net.rs
│           ├── os.rs
│           ├── time.rs
│           ├── regex.rs
│           ├── testing.rs
│           ├── crypto.rs
│           └── datetime.rs
│
├── cli/
│   ├── Cargo.toml
│   └── src/
│       ├── main.rs               # CLI entry point
│       ├── commands/
│       │   ├── mod.rs
│       │   ├── new.rs
│       │   ├── init.rs
│       │   ├── run.rs
│       │   ├── build.rs
│       │   ├── test.rs
│       │   ├── fmt.rs
│       │   ├── lint.rs
│       │   ├── doc.rs
│       │   ├── repl.rs
│       │   ├── add.rs
│       │   ├── remove.rs
│       │   ├── update.rs
│       │   ├── publish.rs
│       │   ├── search.rs
│       │   ├── info.rs
│       │   ├── doctor.rs
│       │   └── version.rs
│       └── util.rs
│
├── nxm/
│   ├── Cargo.toml
│   └── src/
│       ├── main.rs               # nxm entry point
│       ├── config.rs             # nexora.json parser
│       ├── lockfile.rs           # nxm-lock.json parser
│       ├── resolver.rs           # Dependency resolver
│       ├── registry.rs           # Registry client
│       ├── installer.rs          # Package installer
│       ├── commands/
│       │   ├── mod.rs
│       │   ├── init.rs
│       │   ├── install.rs
│       │   ├── add.rs
│       │   ├── remove.rs
│       │   ├── update.rs
│       │   ├── search.rs
│       │   ├── publish.rs
│       │   └── list.rs
│       └── cache.rs              # Package cache
│
├── tools/
│   ├── formatter/
│   │   ├── Cargo.toml
│   │   └── src/
│   │       ├── main.rs
│   │       ├── formatter.rs      # Formatting rules
│   │       └── config.rs
│   │
│   ├── linter/
│   │   ├── Cargo.toml
│   │   └── src/
│   │       ├── main.rs
│   │       ├── linter.rs
│   │       ├── rules/
│   │       │   ├── mod.rs
│   │       │   ├── style.rs
│   │       │   ├── errors.rs
│   │       │   └── best_practices.rs
│   │       └── config.rs
│   │
│   └── docgen/
│       ├── Cargo.toml
│       └── src/
│           ├── main.rs
│           ├── generator.rs
│           ├── templates/
│           │   ├── html.rs
│           │   └── markdown.rs
│           └── extractor.rs
│
├── language-server/
│   ├── Cargo.toml
│   └── src/
│       ├── main.rs
│       ├── server.rs
│       ├── handler.rs
│       ├── completion.rs
│       ├── hover.rs
│       ├── goto.rs
│       ├── references.rs
│       ├── rename.rs
│       ├── symbols.rs
│       ├── diagnostics.rs
│       ├── document.rs
│       └── signature.rs
│
├── registry/
│   ├── Cargo.toml
│   └── src/
│       ├── main.rs
│       ├── api/
│       │   ├── mod.rs
│       │   ├── packages.rs
│       │   ├── auth.rs
│       │   ├── search.rs
│       │   └── users.rs
│       ├── db/
│       │   ├── mod.rs
│       │   ├── models.rs
│       │   └── schema.rs
│       ├── storage/
│       │   ├── mod.rs
│       │   └── s3.rs
│       └── scanner/
│           ├── mod.rs
│           └── malware.rs
│
├── extensions/
│   ├── vscode/
│   │   ├── package.json
│   │   ├── syntaxes/
│   │   │   └── nexora.tmLanguage.json
│   │   ├── language-configuration.json
│   │   ├── snippets/
│   │   │   └── nexora.json
│   │   └── src/
│   │       └── extension.ts
│   └── zed/
│       └── src/
│           └── lib.rs
│
├── stdlib/                        # Nexora source stdlib (thin wrappers)
│   ├── nexora/
│   │   ├── core.nx
│   │   ├── math.nx
│   │   ├── string.nx
│   │   ├── collection.nx
│   │   ├── fs.nx
│   │   ├── http.nx
│   │   ├── json.nx
│   │   ├── net.nx
│   │   ├── os.nx
│   │   ├── time.nx
│   │   ├── regex.nx
│   │   ├── testing.nx
│   │   ├── crypto.nx
│   │   └── datetime.nx
│
├── examples/
│   ├── hello.nx
│   ├── fibonacci.nx
│   ├── web_server.nx
│   ├── chat_app.nx
│   └── ...
│
├── tests/
│   ├── language/
│   │   ├── variables.nx
│   │   ├── functions.nx
│   │   ├── control_flow.nx
│   │   ├── classes.nx
│   │   ├── generics.nx
│   │   ├── error_handling.nx
│   │   └── ...
│   └── integration/
│       └── ...
│
└── docs/
    ├── spec.md
    ├── stdlib.md
    └── contributing.md
```

---

## 11. Phased Roadmap

### v0.1 — Core Language (Lexer, Parser, Interpreter Basics)

**Timeline**: 4-6 weeks

**Lexer**:
- [ ] All tokens: keywords (35), operators, delimiters, literals
- [ ] String interpolation at lex time
- [ ] Comments (`//`, `/* */`, `///`)
- [ ] Error recovery (bad characters → error token, continue)
- [ ] Unicode support in identifiers and strings
- [ ] Triple-quoted strings
- [ ] Raw strings

**Parser**:
- [ ] Pratt parser for all expressions with correct precedence
- [ ] Recursive descent for all statements
- [ ] All expression types: literals, binary, unary, call, index, property, lambda, match, ternary
- [ ] All statement types: let, const, func, if/elif/else, while, for, for-range, loop, break, continue, return
- [ ] Destructuring in let and for
- [ ] String interpolation parsing
- [ ] Error recovery (skip to statement boundary)
- [ ] Span tracking (line/col for every node)

**Interpreter**:
- [ ] Tree-walking evaluation
- [ ] Variables: let, const (immutability check)
- [ ] Functions: named, anonymous, arrow, default params
- [ ] Control flow: if/elif/else, while, for-in, for-range, loop, break, continue
- [ ] Closures (capture by reference)
- [ ] Numeric coercion (Int ↔ Float in comparisons)
- [ ] All operators with correct precedence
- [ ] Short-circuit `&&` and `||`

**Builtins** (20):
- `print`, `input`, `type_of`, `len`, `str`, `int`, `float`, `range`
- `assert`, `panic`

**AST changes**:
```rust
// Add const
enum Stmt {
    Let { name: Pat, value: Expr },
    Const { name: Pat, value: Expr },  // NEW
    // ...
}

// Add ternary
enum Expr {
    Ternary { condition: Box<Expr>, then_expr: Box<Expr>, else_expr: Box<Expr> },
}

// Add for-range
enum Stmt {
    ForRange { var: Ident, start: Expr, end: Expr, step: Option<Expr>, body: Block },
}

// Add loop
enum Stmt {
    Loop { body: Block },
}
```

**Lexer changes**:
```rust
// New tokens
Token::Const,
Token::Elif,
Token::Loop,
Token::StarStar,       // **
Token::Question,       // ?
Token::QuestionDot,    // ?.
Token::DoubleQuestion, // ??
Token::Spread,         // ...
Token::DotDot,         // ..
Token::DotDotEq,       // ..=
Token::Ampersand,      // &
Token::Pipe,           // |
Token::Caret,          // ^
Token::Tilde,          // ~
Token::At,             // @
Token::Backtick,       // `
```

**Parser changes**:
```rust
// Pratt precedence levels updated for **, ??, ?., &&, ||
// New parse functions:
parse_ternary()
parse_const()
parse_loop()
parse_for_range()
parse_destructuring_pat()
```

**Breaking changes**: None (fresh start)

---

### v0.2 — Functions, Closures, Modules

**Timeline**: 4-6 weeks

**Features**:
- [ ] Module system (import/export)
- [ ] Three import forms: `import "mod"`, `import { x } from "mod"`, `import x from "mod"`
- [ ] Export declarations
- [ ] Named parameters (call-site: `func(a: 1, b: 2)`)
- [ ] Variadic parameters (`func sum(..args)`)
- [ ] Named returns (`func divide() -> (result: Int, error: String)`)
- [ ] Higher-order functions
- [ ] Proper closure capture analysis (only capture referenced vars)
- [ ] Recursion
- [ ] Recursive closures

**Builtins** (+15):
- `push`, `pop`, `index_of`, `slice`, `split`, `join`, `contains`
- `upper`, `lower`, `trim`, `starts_with`, `ends_with`, `replace`, `repeat`, `char_at`

**AST changes**:
```rust
// Export statement
enum Stmt {
    Export(Box<Stmt>),
}

// Module declaration
enum Stmt {
    Module { name: Ident, body: Block },
}

// Named parameters
struct Param {
    name: Ident,
    type_ann: Option<TypeExpr>,
    default: Option<Expr>,
    named: bool,  // NEW
}

// Variadic
struct Param {
    // ...
    variadic: bool,  // NEW
}

// Named returns
struct FuncDef {
    // ...
    named_returns: Vec<(Ident, TypeExpr)>,  // NEW
}
```

**Lexer changes**:
```rust
Token::Export,
Token::Module,
Token::DotDot,  // ... for variadic (reuse)
```

**Breaking changes**: None

---

### v0.3 — Classes, Error Handling, Stdlib

**Timeline**: 6-8 weeks

**Features**:
- [ ] Classes with single inheritance
- [ ] Constructors (init method)
- [ ] Methods (instance and static)
- [ ] `this` and `super` keywords
- [ ] `new` keyword for instantiation
- [ ] `is` type checking
- [ ] `as` type casting
- [ ] try/catch/finally
- [ ] throw
- [ ] panic!
- [ ] Result<T, E> type
- [ ] Option<T> type
- [ ] ? operator (propagation)
- [ ] match with patterns (literals, variables, wildcards, destructuring, guards)
- [ ] Built-in Result and Option types

**Builtins** (+20):
- `map`, `filter`, `reduce`, `sort`, `reverse`, `unique`, `flatten`
- `keys`, `values`, `entries`
- `read_file`, `write_file`, `append_file`, `file_exists`, `read_dir`
- `json_parse`, `json_stringify`
- `http_get`, `http_post`

**AST changes**:
```rust
// Class declaration
enum Stmt {
    Class {
        name: Ident,
        parent: Option<Ident>,
        body: ClassBody,
    },
    Impl {
        type_name: Ident,
        body: Block,
    },
}

struct ClassBody {
    properties: Vec<Property>,
    methods: Vec<FuncDef>,
    constructor: Option<FuncDef>,
    static_methods: Vec<FuncDef>,
}

// New expression
enum Expr {
    New { class: Box<Expr>, args: Vec<Expr> },
    This,
    Super,
    Is { expr: Box<Expr>, type_name: Ident },
    As { expr: Box<Expr>, type_name: Ident },
}

// Match arm patterns
enum Pattern {
    Wildcard,
    Ident(Ident),
    Literal(Literal),
    Tuple(Vec<Pattern>),
    Array(Vec<Pattern>),
    Object(Vec<(Ident, Pattern)>),
    Or(Vec<Pattern>),
    Guard { pattern: Box<Pat>, guard: Expr },
}

// Try/catch
enum Stmt {
    Try {
        body: Block,
        catch: Option<CatchClause>,
        finally: Option<Block>,
    },
}

struct CatchClause {
    pattern: Pattern,
    body: Block,
}

// Throw
enum Stmt {
    Throw(Expr),
}
```

**Lexer changes**:
```rust
Token::Class,
Token::New,
Token::This,
Token::Super,
Token::Extends,
Token::Impl,
Token::Try,
Token::Catch,
Token::Finally,
Token::Throw,
Token::Panic,
Token::Is,
Token::As,
Token::Match,
Token::When,
Token::Result,
Token::Option,
Token::Some,
Token::None,
```

**Breaking changes**:
- `and`/`or` → `&&`/`||` (keywords removed, use operators)

---

### v0.4 — Type System, Generics

**Timeline**: 8-10 weeks

**Features**:
- [ ] Hindley-Milner type inference
- [ ] Type annotations in function signatures and let bindings
- [ ] Generic type parameters (`func first<T>(arr: Array<T>) -> T`)
- [ ] Generic classes (`class Stack<T>`)
- [ ] Type constraints (`T: Comparable`)
- [ ] Structs (value semantics)
- [ ] Traits (interfaces with default implementations)
- [ ] Enums (algebraic data types)
- [ ] Enum variants with data
- [ ] Type aliases
- [ ] Opaque types
- [ ] Compile-time type checking (optional — graceful degradation)
- [ ] `type_of` returns accurate runtime type name

**Builtins** (+5):
- `clone`, `equals`, `hash`, `to_string`, `debug`

**AST changes**:
```rust
// Type expressions
enum TypeExpr {
    Simple(Ident),
    Generic { name: Ident, args: Vec<TypeExpr> },
    Array(Box<TypeExpr>),
    Optional(Box<TypeExpr>),
    Function { params: Vec<TypeExpr>, return_type: Box<TypeExpr> },
    Tuple(Vec<TypeExpr>),
    Union(Vec<TypeExpr>),
    Intersection(Vec<TypeExpr>),
}

// Struct declaration
enum Stmt {
    Struct {
        name: Ident,
        fields: Vec<Field>,
        impls: Vec<ImplBlock>,
    },
}

// Trait declaration
enum Stmt {
    Trait {
        name: Ident,
        type_params: Vec<Ident>,
        methods: Vec<TraitMethod>,
    },
}

// Enum declaration
enum Stmt {
    Enum {
        name: Ident,
        type_params: Vec<Ident>,
        variants: Vec<EnumVariant>,
    },
}

struct EnumVariant {
    name: Ident,
    data: Option<Vec<TypeExpr>>,
}

// Generic parameters
struct FuncDef {
    type_params: Vec<Ident>,  // NEW
    // ...
}
```

**Lexer changes**:
```rust
Token::Struct,
Token::Trait,
Token::Enum,
Token::Impl,
Token::Type,
Token::Opaque,
Token::Where,
Token::Self_,  // as type
```

**Breaking changes**:
- Function signatures now checked at compile time (type annotations required)
- `type` keyword becomes reserved for type aliases

---

### v0.5 — Bytecode Compiler, VM

**Timeline**: 10-12 weeks

**Features**:
- [ ] SSA-based IR (Nexora IR)
- [ ] IR builder
- [ ] AST → IR lowering
- [ ] Bytecode compiler (IR → bytecode)
- [ ] Stack-based VM
- [ ] Call frames with closures and upvalues
- [ ] Mark-sweep garbage collector
- [ ] All opcodes (arithmetic, comparison, logical, control flow, functions, objects)
- [ ] Constant folding pass
- [ ] Dead code elimination pass
- [ ] Constant propagation pass
- [ ] Inlining pass
- [ ] Tail call optimization pass
- [ ] `nx build` command (compiles to .nxc bytecode)
- [ ] `nx run` with VM execution

**VM architecture**:
```
Stack: Vec<Value> (1024 entries max)
Frames: Vec<CallFrame> (256 max)
Globals: HashMap<String, Value>
Heap: ObjectAllocator
IP: usize
```

**Bytecode format**:
```
Header: magic (4 bytes) + version (2 bytes)
Functions: count + function definitions
Constants: count + constant pool
Globals: count + global name strings
Main: function index
```

**Breaking changes**:
- None (interpreter still available as fallback)

---

### v0.6 — Async/Await, Concurrency

**Timeline**: 8-10 weeks

**Features**:
- [ ] Event loop (tokio integration)
- [ ] async/await keywords (real implementation)
- [ ] Spawn tasks
- [ ] Join handles
- [ ] Channels (send/recv)
- [ ] Mutex, RwLock
- [ ] async fn, async blocks
- [ ] `.await` expressions
- [ ] `all()`, `race()` combinators
- [ ] async iteration (`for await`)
- [ ] Non-blocking I/O (file, network)
- [ ] Select statement

**Builtins** (+5):
- `spawn`, `join`, `channel`, `mutex`, `sleep_async`

**Lexer changes**:
```rust
Token::Spawn,
Token::Select,
Token::Channel,
Token::Mutex,
Token::RwLock,
```

**Breaking changes**:
- `async`/`await` become real keywords (no longer no-ops)

---

### v0.7 — Package Manager, Registry

**Timeline**: 6-8 weeks

**Features**:
- [ ] `nxm init`, `nxm install`, `nxm add`, `nxm remove`
- [ ] `nexora.json` manifest
- [ ] `nxm-lock.json` lockfile
- [ ] Dependency resolution (SAT solver)
- [ ] Package cache (`~/.nexora/packages/`)
- [ ] Semantic versioning
- [ ] Registry client
- [ ] Package download and extraction
- [ ] Workspace support
- [ ] Scripts system
- [ ] `nxm publish` (basic)
- [ ] `nxm search` (basic)

**Builtins** (0 new, stdlib handles this)

**Breaking changes**: None

---

### v0.8 — Tooling (Formatter, Linter, LSP)

**Timeline**: 8-10 weeks

**Features**:
- [ ] Formatter (4-space indent, consistent style)
- [ ] `nx fmt` command
- [ ] Linter with rule system
- [ ] `nx lint` command
- [ ] LSP server
- [ ] Autocomplete (context-aware)
- [ ] Go to definition
- [ ] Find references
- [ ] Hover information
- [ ] Signature help
- [ ] Diagnostics (real-time errors)
- [ ] Rename symbol
- [ ] Code actions
- [ ] VS Code extension
- [ ] Zed extension

**Breaking changes**: None

---

### v0.9 — Standard Library Completion

**Timeline**: 6-8 weeks

**Features**:
- [ ] Complete `nexora.core` module
- [ ] Complete `nexora.math` module
- [ ] Complete `nexora.string` module
- [ ] Complete `nexora.collection` module
- [ ] Complete `nexora.fs` module
- [ ] Complete `nexora.http` module
- [ ] Complete `nexora.json` module
- [ ] Complete `nexora.net` module (TCP/UDP)
- [ ] Complete `nexora.os` module
- [ ] Complete `nexora.time` module
- [ ] Complete `nexora.regex` module
- [ ] Complete `nexora.testing` module (test framework)
- [ ] Complete `nexora.crypto` module
- [ ] Complete `nexora.datetime` module
- [ ] `nx test` command (test runner)
- [ ] `nx doc` command (documentation generator)

**Builtins** (+50):
- All stdlib functions moved to modules

**Breaking changes**:
- Built-in functions moved to modules (e.g., `read_file` → `nexora.fs.read`)

---

### v1.0 — Production Ready

**Timeline**: 8-10 weeks

**Features**:
- [ ] All bugs from codebase analysis fixed
- [ ] Full test suite (>90% coverage)
- [ ] Documentation complete
- [ ] Package registry launched
- [ ] REPL with multi-line, history, autocomplete
- [ ] Profiler
- [ ] Debugger (basic breakpoints, step, variables)
- [ ] Cross-platform (Linux, macOS, Windows)
- [ ] Performance benchmarks
- [ ] Migration guide from v0.x
- [ ] Error messages quality pass
- [ ] Security audit

**Breaking changes**:
- All v0.x breaking changes consolidated
- REPL behavior finalized
- Module system finalized

---

## Appendix A: Known Bugs to Fix (from codebase analysis)

| Bug | Location | Fix |
|-----|----------|-----|
| `**` operator missing | `lexer.rs` | Add `StarStar` token for exponentiation |
| Constructor args leak as `arg0`, `arg1` | `interpreter.rs:549-552` | Don't store raw args; let `init` bind params |
| `slice` uses `args[3]` instead of `args[2]` | `interpreter.rs:795` | Fix index to `args[2]` |
| `assert` message never read (off-by-one) | `interpreter.rs:1555` | Fix: check `args.len() > 1` not `> 2` |
| `Integer(1) == Float(1.0)` returns false | `interpreter.rs:1847` | Add numeric coercion in equality |
| `ObjectInstance` type_name = "Object" | `value.rs:81` | Return `class_name` for instances |
| `and`/`or` don't short-circuit | `interpreter.rs:631-634` | Implement short-circuit evaluation |
| `async`/`await` are no-ops | `interpreter.rs:137-144` | Implement real async (v0.6) |
| `sort` compares by string repr | `interpreter.rs:1310` | Implement proper comparison |
| `panic!` for bad chars | `lexer.rs:311,334` | Return error token, continue |
| stdlib `module`/`export` unsupported | `lib/*.nx` | Add tokens and parser support (v0.2) |
| Two function types confusing | `value.rs:16-26` | Unify to single Closure type with captures |

---

## Appendix B: Example Programs

### Hello World
```nx
print("Hello, World!")
```

### Fibonacci
```nx
func fibonacci(n: Int) -> Int {
    if n <= 1 {
        return n
    }
    return fibonacci(n - 1) + fibonacci(n - 2)
}

for i in 0..20 {
    print("fib(${i}) = ${fibonacci(i)}")
}
```

### Web Server
```nx
import { serve } from "nexora/http"
import { json_stringify } from "nexora/json"

func handler(method: String, path: String) -> String {
    if path == "/api/hello" {
        return json_stringify({
            message: "Hello from Nexora!",
            timestamp: now(),
        })
    }
    return "<html><body><h1>Welcome to Nexora</h1></body></html>"
}

serve(8080, handler)
```

### File Processor
```nx
import { read, write, read_dir } from "nexora/fs"
import { parse, stringify } from "nexora/json"

func process_data(input_dir: String, output_dir: String) {
    let files = read_dir(input_dir).expect("Cannot read directory")
    
    for file in files {
        let content = read("${input_dir}/${file}").expect("Cannot read file")
        let data = parse(content).expect("Invalid JSON")
        
        // Process data
        let result = transform(data)
        
        write("${output_dir}/${file}", stringify(result, true))
            .expect("Cannot write file")
    }
}

process_data("data/input", "data/output")
```

### Async HTTP Client
```nx
import { get, post } from "nexora/http"

async func fetch_all(urls: Array<String>) -> Array<String> {
    let tasks = urls.map(|url| async {
        let response = await get(url)
        return response.body
    })
    
    return await all(tasks)
}

async func main() {
    let urls = [
        "https://api.example.com/users",
        "https://api.example.com/posts",
        "https://api.example.com/comments",
    ]
    
    let results = await fetch_all(urls)
    print("Fetched ${len(results)} responses")
}
```

### Test Suite
```nx
import { test, describe, assert_eq } from "nexora/testing"

describe("math operations") {
    test("addition") {
        assert_eq(2 + 3, 5)
        assert_eq(-1 + 1, 0)
        assert_eq(0 + 0, 0)
    }
    
    test("multiplication") {
        assert_eq(2 * 3, 6)
        assert_eq(-2 * 3, -6)
        assert_eq(0 * 100, 0)
    }
    
    test("division") {
        assert_eq(10 / 2, 5)
        assert_eq(7 / 2, 3)  // integer division
    }
}

describe("string operations") {
    test("concatenation") {
        assert_eq("Hello" + " " + "World", "Hello World")
    }
    
    test("interpolation") {
        let name = "Nexora"
        assert_eq("Hello, ${name}!", "Hello, Nexora!")
    }
}
```

---

*This document is the complete blueprint for Nexora v1.0. Every feature, every syntax form, every compiler phase — defined here. Build the future, one `.nx` file at a time.*
