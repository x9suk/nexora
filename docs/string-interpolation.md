# String Interpolation

## Table of Contents

- [Basic Interpolation](#basic-interpolation)
- [Expression Interpolation](#expression-interpolation)
- [Variable Interpolation](#variable-interpolation)
- [Nested Interpolation](#nested-interpolation)
- [Examples](#examples)

## Basic Interpolation

Use `${expression}` to embed expressions inside strings:

```nexora
let name = "Nexora"
let version = 4
print "Welcome to ${name} v${version}!"
```

Output:
```
Welcome to Nexora v4!
```

## Expression Interpolation

Any valid expression can be used inside `${}`:

### Arithmetic

```nexora
print "2 + 3 = ${2 + 3}"
print "10 * 5 = ${10 * 5}"
print "100 / 3 = ${100 / 3}"
```

Output:
```
2 + 3 = 5
10 * 5 = 50
100 / 3 = 33
```

### Function Calls

```nexora
func add(a, b) {
    return a + b
}

print "5 + 3 = ${add(5, 3)}"
```

Output:
```
5 + 3 = 8
```

### Method Calls

```nexora
let name = "nexora"
print "Upper: ${name.toUpperCase()}"
print "Length: ${name.length}"
```

Output:
```
Upper: NEXORA
Length: 6
```

### Ternary Operator

```nexora
let age = 20
print "You are ${age >= 18 ? 'an adult' : 'a minor'}"
```

## Variable Interpolation

Embed variables directly:

```nexora
let lang = "Nexora"
let ver = 4
print "Welcome to ${lang} v${ver}!"
print "Language: ${lang}"
print "Version: ${ver}"
```

## Nested Interpolation

Expressions can contain strings with their own interpolation:

```nexora
let outer = "outer"
print "The ${outer} string contains ${'inner'} text"
```

## Examples

### Greeting

```nexora
let name = "World"
print "Hello, ${name}!"
```

### Date/Time Display

```nexora
let day = "Monday"
let date = "2026-01-15"
print "Today is ${day}, ${date}"
```

### Calculation Display

```nexora
let price = 29.99
let quantity = 3
print "Total: $${price * quantity}"
print "Item: $${price} x ${quantity}"
```

### Status Message

```nexora
let items = [1, 2, 3, 4, 5]
let count = len(items)
print "You have ${count} item${count != 1 ? 's' : ''}"
```

### Debug Output

```nexora
func debugPrint(variable, value) {
    print "[DEBUG] ${variable} = ${value}"
}

let x = 42
debugPrint("x", x)  // [DEBUG] x = 42
```

### Class Output

```nexora
class User {
    init(name, age) {
        this.name = name
        this.age = age
    }

    toString() {
        return "User(${this.name}, age ${this.age})"
    }
}

let user = new User("Alice", 25)
print "Created ${user}"
```

### Multi-line with Interpolation

```nexora
let name = "Nexora"
let features = ["fast", "simple", "clean"]

print """
Welcome to ${name}!

Features:
- ${features[0]}
- ${features[1]}
- ${features[2]}
"""
```

### JSON-like Output

```nexora
let config = {
    host: "localhost",
    port: 8080,
    debug: true
}

print '{"host": "${config.host}", "port": ${config.port}, "debug": ${config.debug}}'
```

## Tips

1. **Keep it simple** — Use interpolation for simple embedding, not complex logic
2. **Use parentheses** — For complex expressions, consider computing the value first:

```nexora
let result = 2 + 3 * 4
print "Result: ${result}"  // Cleaner than print "${2 + 3 * 4}"
```

3. **Escape characters** — Use `\$` to include a literal dollar sign:

```nexora
print "Price: \$50"  // "Price: $50"
```
