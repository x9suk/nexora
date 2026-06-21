# Error Handling

## Table of Contents

- [Try-Catch](#try-catch)
- [Try-Catch-Finally](#try-catch-finally)
- [Throwing Errors](#throwing-errors)
- [Nested Try-Catch](#nested-try-catch)
- [Common Errors](#common-errors)
- [Best Practices](#best-practices)

## Try-Catch

Catch runtime errors with `try` and `catch`:

```nexora
try {
    let result = 10 / 0
    print "Result: " + str(result)
} catch (error) {
    print "Error caught: " + error
}
```

Output:
```
Error caught: Division by zero
```

## Try-Catch-Finally

Add a `finally` block to run code regardless of success or failure:

```nexora
try {
    let data = parseJSON("{ invalid json }")
    print "Data: " + str(data)
} catch (error) {
    print "JSON parse error: " + error
} finally {
    print "Finally block executed"
}
```

Output:
```
JSON parse error: ...
Finally block executed
```

## Throwing Errors

Use `throw` to raise custom errors:

```nexora
func divide(a, b) {
    if b == 0 {
        throw "Division by zero error"
    }
    return a / b
}

try {
    let result = divide(10, 0)
    print "Result: " + str(result)
} catch (error) {
    print "Caught: " + error
}
// Output: Caught: Division by zero error
```

### Throwing Objects

```nexora
func validateAge(age) {
    if age < 0 {
        throw "Age cannot be negative"
    }
    if age > 150 {
        throw "Age seems unrealistic"
    }
    return true
}

try {
    validateAge(-5)
} catch (error) {
    print "Validation error: " + error
}
```

## Nested Try-Catch

```nexora
try {
    try {
        throw "Inner error"
    } catch (innerError) {
        print "Inner catch: " + innerError
        throw "Outer error"
    }
} catch (outerError) {
    print "Outer catch: " + outerError
}
```

Output:
```
Inner catch: Inner error
Outer catch: Outer error
```

## Common Errors

### Division by Zero

```nexora
try {
    let result = 10 / 0
} catch (error) {
    print "Error: " + error  // "Division by zero"
}
```

### Index Out of Bounds

```nexora
try {
    let arr = [1, 2, 3]
    let value = arr[10]
} catch (error) {
    print "Error: " + error  // "Index out of bounds"
}
```

### Undefined Variable

```nexora
try {
    print unknownVariable
} catch (error) {
    print "Error: " + error  // "Undefined variable: unknownVariable"
}
```

### Type Errors

```nexora
try {
    let result = "hello" - 5
} catch (error) {
    print "Error: " + error  // "Type error: Cannot subtract integer from string"
}
```

### Stack Overflow (Infinite Recursion)

```nexora
func infinite() {
    return infinite()
}

try {
    infinite()
} catch (error) {
    print "Error: " + error  // "Maximum call stack size exceeded"
}
```

## Best Practices

### Validate Inputs

```nexora
func divide(a, b) {
    if typeof(a) != "integer" && typeof(a) != "float" {
        throw "First argument must be a number"
    }
    if typeof(b) != "integer" && typeof(b) != "float" {
        throw "Second argument must be a number"
    }
    if b == 0 {
        throw "Division by zero"
    }
    return a / b
}
```

### Use Finally for Cleanup

```nexora
func processFile(filename) {
    let file = null
    try {
        file = read_file(filename)
        // process file
    } catch (error) {
        print "Error processing file: " + error
    } finally {
        if file != null {
            close_file(file)
        }
    }
}
```

### Propagate Errors

```nexora
func fetchData() {
    try {
        let response = await http.get("https://api.example.com")
        return response.json()
    } catch (error) {
        print "Network error: " + error
        throw error  // Re-throw for caller to handle
    }
}

try {
    let data = await fetchData()
} catch (error) {
    print "Failed to fetch: " + error
}
```

### Error Messages

Provide descriptive error messages:

```nexora
func transferMoney(from, to, amount) {
    if from.balance < amount {
        throw "Insufficient funds: " + str(from.balance) + " available, " + str(amount) + " requested"
    }
    if amount <= 0 {
        throw "Transfer amount must be positive, got: " + str(amount)
    }
    // ... perform transfer
}
```

### Graceful Degradation

```nexora
func loadConfig(filename) {
    try {
        let content = read_file(filename)
        return json_parse(content)
    } catch (error) {
        print "Warning: Could not load config, using defaults"
        return {
            host: "localhost",
            port: 8080,
            debug: false
        }
    }
}
```
