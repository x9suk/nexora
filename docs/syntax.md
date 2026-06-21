# Nexora Syntax Reference

## Table of Contents

- [Comments](#comments)
- [Statements](#statements)
- [Expressions](#expressions)
- [Operators](#operators)
- [Literals](#literals)
- [Blocks](#blocks)

## Comments

```nexora
// Single-line comment

/*
  Multi-line comment
*/
```

## Statements

Statements are the basic units of execution in Nexora.

### Print Statement

```nexora
print "Hello, World!"
print "Name:", "Nexora", "Version:", 1
```

### Variable Declaration

```nexora
let name = "Nexora"
let version = 1
const MAX_SIZE = 100
```

See [Variables & Types](variables.md) for details.

### Function Declaration

```nexora
func greet(name) {
    print "Hello, " + name + "!"
}
```

See [Functions](functions.md) for details.

### Class Declaration

```nexora
class Animal {
    init(name) {
        this.name = name
    }
}
```

See [Classes](classes.md) for details.

### If-Else

```nexora
let age = 20

if age >= 18 {
    print "Adult"
} elif age >= 13 {
    print "Teenager"
} else {
    print "Child"
}
```

### While Loop

```nexora
let i = 1
while i <= 5 {
    print i
    i += 1
}
```

### For Loop

```nexora
let fruits = ["apple", "banana", "cherry"]
for fruit in fruits {
    print fruit
}
```

### Break and Continue

```nexora
for num in [1, 2, 3, 4, 5, 6, 7, 8, 9, 10] {
    if num % 2 != 0 {
        continue
    }
    print num
}

for i in range(1, 100) {
    if i > 10 {
        break
    }
    print i
}
```

### Try-Catch-Finally

```nexora
try {
    let result = 10 / 0
} catch (error) {
    print "Error:", error
} finally {
    print "Cleanup"
}
```

See [Error Handling](error-handling.md) for details.

### Return Statement

```nexora
func add(a, b) {
    return a + b
}
```

### Import Statement

```nexora
import "math"
import "os" as operatingSystem
from "utils" import { formatDate, parseNumber }
```

See [Modules](modules.md) for details.

### Export Statement

```nexora
export func publicFunction() {
    print "This is exported"
}
```

## Expressions

Expressions produce values.

### Ternary Operator

```nexora
let age = 20
let status = age >= 18 ? "adult" : "minor"
```

### Match Expression

```nexora
let day = "Monday"
let type = match day {
    "Monday" => "Weekday"
    "Tuesday" => "Weekday"
    _ => "Unknown"
}
```

See [Pattern Matching](pattern-matching.md) for details.

### Lambda Expression

```nexora
let double = x => x * 2
let add = (a, b) => a + b
```

See [Functions](functions.md) for details.

## Operators

### Arithmetic Operators

| Operator | Description | Example |
|----------|-------------|---------|
| `+` | Addition | `5 + 3` → `8` |
| `-` | Subtraction | `5 - 3` → `2` |
| `*` | Multiplication | `5 * 3` → `15` |
| `/` | Division | `6 / 3` → `2` |
| `%` | Modulo | `7 % 3` → `1` |
| `**` | Power | `2 ** 10` → `1024` |

### Comparison Operators

| Operator | Description | Example |
|----------|-------------|---------|
| `==` | Equal | `5 == 5` → `true` |
| `!=` | Not equal | `5 != 3` → `true` |
| `>` | Greater than | `5 > 3` → `true` |
| `<` | Less than | `5 < 3` → `false` |
| `>=` | Greater or equal | `5 >= 5` → `true` |
| `<=` | Less or equal | `3 <= 5` → `true` |

### Logical Operators

| Operator | Description | Example |
|----------|-------------|---------|
| `&&` | AND | `true && false` → `false` |
| `||` | OR | `true || false` → `true` |
| `!` | NOT | `!true` → `false` |

### Bitwise Operators

| Operator | Description | Example |
|----------|-------------|---------|
| `&` | AND | `5 & 3` → `1` |
| `\|` | OR | `5 \| 3` → `7` |
| `^` | XOR | `5 ^ 3` → `6` |
| `<<` | Left shift | `1 << 3` → `8` |
| `>>` | Right shift | `8 >> 2` → `2` |

### Assignment Operators

| Operator | Description | Example |
|----------|-------------|---------|
| `=` | Assign | `x = 5` |
| `+=` | Add and assign | `x += 3` |
| `-=` | Subtract and assign | `x -= 3` |
| `*=` | Multiply and assign | `x *= 3` |
| `/=` | Divide and assign | `x /= 3` |
| `%=` | Modulo and assign | `x %= 3` |
| `**=` | Power and assign | `x **= 3` |

### String Operators

| Operator | Description | Example |
|----------|-------------|---------|
| `+` | Concatenation | `"Hello" + " World"` → `"Hello World"` |
| `*` | Repeat | `"ha" * 3` → `"hahaha"` |

### Other Operators

| Operator | Description | Example |
|----------|-------------|---------|
| `in` | Membership | `"a" in ["a", "b"]` → `true` |
| `.` | Property access | `obj.name` |
| `[]` | Index access | `arr[0]` |
| `? :` | Ternary | `x > 0 ? "pos" : "non-pos"` |

## Literals

### Integer

```nexora
let a = 42
let b = -10
let c = 0
```

### Float

```nexora
let pi = 3.14159
let e = 2.71828
```

### String

```nexora
let name = "Nexora"
let greeting = 'Hello'
let path = "C:\\Users"
```

### Boolean

```nexora
let isTrue = true
let isFalse = false
```

### Null

```nexora
let nothing = null
```

### Array

```nexora
let numbers = [1, 2, 3, 4, 5]
let mixed = [1, "hello", true, null]
```

### Object

```nexora
let person = {
    name: "Nexora",
    version: 1,
    isAwesome: true
}
```

## Blocks

Blocks group statements together:

```nexora
if condition {
    // block of statements
    print "a"
    print "b"
}

func myFunc() {
    // function body
    let x = 1
    return x
}

// Scope
{
    let temp = 10
    print temp
}
```
