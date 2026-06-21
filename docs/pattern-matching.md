# Pattern Matching

## Table of Contents

- [Match Expression](#match-expression)
- [Literal Patterns](#literal-patterns)
- [Wildcard Pattern](#wildcard-pattern)
- [Variable Patterns](#variable-patterns)
- [Guard Clauses](#guard-clauses)
- [String Matching](#string-matching)
- [Number Matching](#number-matching)
- [Examples](#examples)

## Match Expression

The `match` expression evaluates a value against multiple patterns:

```nexora
let day = "Monday"
let type = match day {
    "Monday" => "Weekday"
    "Tuesday" => "Weekday"
    "Saturday" => "Weekend"
    "Sunday" => "Weekend"
    _ => "Unknown"
}

print type  // "Weekday"
```

### Syntax

```nexora
let result = match value {
    pattern1 => expression1
    pattern2 => expression2
    _ => defaultExpression
}
```

## Literal Patterns

Match against specific values:

```nexora
let num = 42
let label = match num {
    0 => "zero"
    1 => "one"
    42 => "the answer"
    _ => "something else"
}

print label  // "the answer"
```

### String Literals

```nexora
let color = "red"
let hex = match color {
    "red" => "#FF0000"
    "green" => "#00FF00"
    "blue" => "#0000FF"
    _ => "#000000"
}

print hex  // "#FF0000"
```

### Boolean Literals

```nexora
let enabled = true
let status = match enabled {
    true => "Enabled"
    false => "Disabled"
}

print status  // "Enabled"
```

## Wildcard Pattern

Use `_` to match any value:

```nexora
let day = "Wednesday"
let type = match day {
    "Monday" => "Start of week"
    "Friday" => "End of work week"
    _ => "Some other day"
}

print type  // "Some other day"
```

### Wildcard as Default

Always include a wildcard pattern as a fallback:

```nexora
let num = 99
let label = match num {
    1 => "one"
    2 => "two"
    _ => "many"
}

print label  // "many"
```

## Variable Patterns

Capture the matched value in a variable:

```nexora
let value = 5
let description = match value {
    0 => "nothing"
    1 => "just one"
    n => str(n) + " items"
}

print description  // "5 items"
```

## Guard Clauses

Add conditions to patterns (not yet implemented):

```nexora
// Planned syntax
let num = 15
let label = match num {
    n if n < 0 => "negative"
    0 => "zero"
    n if n > 0 => "positive"
    _ => "unknown"
}
```

## String Matching

### Day of Week

```nexora
func getDayType(day) {
    return match day {
        "Monday" => "Weekday"
        "Tuesday" => "Weekday"
        "Wednesday" => "Weekday"
        "Thursday" => "Weekday"
        "Friday" => "Weekday"
        "Saturday" => "Weekend"
        "Sunday" => "Weekend"
        _ => "Invalid day"
    }
}

print getDayType("Monday")    // "Weekday"
print getDayType("Saturday")  // "Weekend"
print getDayType("Funday")    // "Invalid day"
```

### HTTP Status Codes

```nexora
func getStatusMessage(code) {
    return match code {
        200 => "OK"
        201 => "Created"
        400 => "Bad Request"
        401 => "Unauthorized"
        403 => "Forbidden"
        404 => "Not Found"
        500 => "Internal Server Error"
        _ => "Unknown status"
    }
}

print getStatusMessage(200)  // "OK"
print getStatusMessage(404)  // "Not Found"
```

## Number Matching

### Grade Calculator

```nexora
func getGrade(score) {
    return match score {
        s if s >= 90 => "A"
        s if s >= 80 => "B"
        s if s >= 70 => "C"
        s if s >= 60 => "D"
        _ => "F"
    }
}

print getGrade(95)  // "A"
print getGrade(72)  // "C"
```

### Season Detector

```nexora
func getSeason(month) {
    return match month {
        12 => "Winter"
        1 => "Winter"
        2 => "Winter"
        3 => "Spring"
        4 => "Spring"
        5 => "Spring"
        6 => "Summer"
        7 => "Summer"
        8 => "Summer"
        9 => "Fall"
        10 => "Fall"
        11 => "Fall"
        _ => "Invalid month"
    }
}

print getSeason(7)   // "Summer"
print getSeason(11)  // "Fall"
```

## Examples

### Traffic Light

```nexora
func getAction(light) {
    return match light {
        "red" => "Stop"
        "yellow" => "Caution"
        "green" => "Go"
        _ => "Invalid light"
    }
}

print getAction("red")     // "Stop"
print getAction("green")   // "Go"
```

### Calculator with Match

```nexora
func calculate(a, op, b) {
    return match op {
        "+" => a + b
        "-" => a - b
        "*" => a * b
        "/" => a / b
        "%" => a % b
        "**" => a ** b
        _ => "Unknown operator"
    }
}

print calculate(10, "+", 5)   // 15
print calculate(10, "*", 3)   // 30
print calculate(10, "/", 2)   // 5
```

### Type-Based Logic

```nexora
func describe(value) {
    return match typeof(value) {
        "integer" => "Whole number: " + str(value)
        "float" => "Decimal number: " + str(value)
        "string" => "Text: " + value
        "boolean" => "Boolean: " + str(value)
        "null" => "Nothing"
        "array" => "Array with " + str(len(value)) + " elements"
        "object" => "Object with " + str(len(keys(value))) + " keys"
        _ => "Unknown type"
    }
}

print describe(42)           // "Whole number: 42"
print describe("hello")      // "Text: hello"
print describe([1, 2, 3])    // "Array with 3 elements"
```

### Menu System

```nexora
func handleMenuChoice(choice) {
    let action = match choice {
        "1" => "View profile"
        "2" => "Edit settings"
        "3" => "View messages"
        "4" => "Logout"
        "q" => "Quit"
        _ => "Invalid choice"
    }
    print "Action: " + action
}

handleMenuChoice("2")  // "Action: Edit settings"
handleMenuChoice("q")  // "Action: Quit"
```
