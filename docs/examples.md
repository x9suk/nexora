# Code Examples

## Table of Contents

- [Hello World](#hello-world)
- [Variables & Types](#variables--types)
- [Control Flow](#control-flow)
- [Functions](#functions)
- [Arrays & Objects](#arrays--objects)
- [Classes](#classes)
- [Error Handling](#error-handling)
- [Pattern Matching](#pattern-matching)
- [String Interpolation](#string-interpolation)
- [Higher-Order Functions](#higher-order-functions)
- [Web Server](#web-server)
- [Calculator](#calculator)
- [Todo List](#todo-list)
- [File Processing](#file-processing)

## Hello World

```nexora
print "Hello, World!"
print "Welcome to Nexora!"
```

## Variables & Types

```nexora
let name = "Nexora"
let version = 4
let pi = 3.14159
let isAwesome = true
let nothing = null

print "Language: " + name
print "Version: " + str(version)
print "Pi: " + str(pi)
print "Type: " + typeof(name)
```

## Control Flow

```nexora
let age = 20

if age >= 18 {
    print "Adult"
} elif age >= 13 {
    print "Teenager"
} else {
    print "Child"
}

// While loop
let i = 1
while i <= 5 {
    print "Count: " + str(i)
    i += 1
}

// For loop
for fruit in ["apple", "banana", "cherry"] {
    print fruit
}

// Break and continue
for num in range(1, 20) {
    if num % 2 != 0 {
        continue
    }
    if num > 10 {
        break
    }
    print num
}
```

## Functions

```nexora
func greet(name) {
    return "Hello, " + name + "!"
}

func factorial(n) {
    if n <= 1 { return 1 }
    return n * factorial(n - 1)
}

let multiply = (a, b) => a * b

print greet("Nexora")
print factorial(5)
print multiply(3, 4)
```

## Arrays & Objects

```nexora
let numbers = [1, 2, 3, 4, 5]
let doubled = map(numbers, x => x * 2)
let evens = filter(numbers, x => x % 2 == 0)
let sum = reduce(numbers, (acc, x) => acc + x, 0)

print "Doubled: " + str(doubled)
print "Evens: " + str(evens)
print "Sum: " + str(sum)

let person = {
    name: "Nexora",
    version: 4,
    features: ["fast", "simple", "clean"]
}

print person.name
print len(person.features)
```

## Classes

```nexora
class Animal {
    init(name) {
        this.name = name
    }

    speak() {
        return this.name + " makes a sound"
    }
}

class Dog extends Animal {
    init(name) {
        super(name)
    }

    bark() {
        return this.name + " barks!"
    }
}

let rex = new Dog("Rex")
print rex.bark()
print rex.speak()
```

## Error Handling

```nexora
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
} finally {
    print "Done"
}
```

## Pattern Matching

```nexora
let day = "Monday"
let type = match day {
    "Monday" => "Weekday"
    "Tuesday" => "Weekday"
    "Saturday" => "Weekend"
    "Sunday" => "Weekend"
    _ => "Unknown"
}

print day + " is a " + type
```

## String Interpolation

```nexora
let name = "Nexora"
let version = 4
print "Welcome to ${name} v${version}!"
print "2 + 3 = ${2 + 3}"
```

## Higher-Order Functions

```nexora
let numbers = [1, 2, 3, 4, 5]

// Map
let doubled = map(numbers, x => x * 2)

// Filter
let evens = filter(numbers, x => x % 2 == 0)

// Reduce
let sum = reduce(numbers, (acc, x) => acc + x, 0)

// Find
let found = find(numbers, x => x > 3)

// Every/Some
print every(numbers, x => x > 0)  // true
print some(numbers, x => x > 4)   // true

print "Doubled: " + str(doubled)
print "Evens: " + str(evens)
print "Sum: " + str(sum)
print "Found: " + str(found)
```

## Web Server

```nexora
func handler(method, path) {
    if path == "/" {
        return """
        <!DOCTYPE html>
        <html>
        <head><title>Nexora Web</title></head>
        <body>
            <h1>Welcome to Nexora!</h1>
            <p>A simple, clean programming language.</p>
            <ul>
                <li>Fast</li>
                <li>Simple</li>
                <li>Clean</li>
            </ul>
        </body>
        </html>
        """
    }
    if path == "/api/hello" {
        return '{"message": "Hello from Nexora!"}'
    }
    return "<h1>404 Not Found</h1>"
}

print "Server running at http://localhost:8080"
serve(8080, handler)
```

## Calculator

```nexora
func calculate(a, op, b) {
    return match op {
        "+" => a + b
        "-" => a - b
        "*" => a * b
        "/" => {
            if b == 0 {
                throw "Division by zero"
            }
            return a / b
        }
        "%" => a % b
        "**" => a ** b
        _ => throw "Unknown operator: " + op
    }
}

try {
    print "10 + 5 = " + str(calculate(10, "+", 5))
    print "10 - 3 = " + str(calculate(10, "-", 3))
    print "4 * 7 = " + str(calculate(4, "*", 7))
    print "10 / 2 = " + str(calculate(10, "/", 2))
    print "10 % 3 = " + str(calculate(10, "%", 3))
    print "2 ** 10 = " + str(calculate(2, "**", 10))
} catch (error) {
    print "Error: " + error
}
```

## Todo List

```nexora
let todos = []
let nextId = 1

func addTodo(text) {
    let todo = {
        id: nextId,
        text: text,
        done: false
    }
    push(todos, todo)
    nextId += 1
    return todo
}

func completeTodo(id) {
    for todo in todos {
        if todo.id == id {
            todo.done = true
            return todo
        }
    }
    return null
}

func listTodos() {
    for todo in todos {
        let status = todo.done ? "[x]" : "[ ]"
        print status + " " + str(todo.id) + ". " + todo.text
    }
}

// Usage
addTodo("Learn Nexora")
addTodo("Build a project")
addTodo("Share with others")

print "=== Todo List ==="
listTodos()

print ""
print "Completing task 1..."
completeTodo(1)

print ""
print "=== Updated List ==="
listTodos()
```

## File Processing

```nexora
// Read and process a file
func processFile(filename) {
    try {
        let content = read_file(filename)
        let lines = split(content, "\n")
        
        let wordCount = 0
        let lineCount = len(lines)
        
        for line in lines {
            let words = split(line, " ")
            wordCount += len(words)
        }
        
        print "File: " + filename
        print "Lines: " + str(lineCount)
        print "Words: " + str(wordCount)
    } catch (error) {
        print "Error reading file: " + error
    }
}

// Write to a file
func writeFile(filename, content) {
    try {
        write_file(filename, content)
        print "Written to " + filename
    } catch (error) {
        print "Error writing file: " + error
    }
}

// Usage
writeFile("output.txt", "Hello from Nexora!\nThis is a test file.")
processFile("output.txt")
```
