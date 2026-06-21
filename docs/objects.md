# Objects

## Table of Contents

- [Creating Objects](#creating-objects)
- [Accessing Properties](#accessing-properties)
- [Modifying Properties](#modifying-properties)
- [Nested Objects](#nested-objects)
- [Object Methods](#object-methods)
- [Object Functions](#object-functions)
- [Iterating Objects](#iterating-objects)
- [Object Patterns](#object-patterns)

## Creating Objects

Objects are key-value pairs:

```nexora
let person = {
    name: "Nexora",
    version: 4,
    isAwesome: true
}
```

### Empty Object

```nexora
let empty = {}
```

### String Keys

Keys can be strings:

```nexora
let config = {
    "api-url": "https://api.example.com",
    "max-retries": 3,
    "debug-mode": false
}
```

## Accessing Properties

### Dot Notation

```nexora
let person = {
    name: "Nexora",
    age: 1
}

print person.name   // "Nexora"
print person.age    // 1
```

### Bracket Notation

```nexora
let person = {
    name: "Nexora",
    age: 1
}

let key = "name"
print person[key]   // "Nexora"
```

### Property Access

```nexora
let obj = { a: 1, b: 2, c: 3 }
print obj.a  // 1
print obj.b  // 2
print obj.c  // 3
```

## Modifying Properties

```nexora
let person = {
    name: "Nexora",
    age: 1
}

// Add new property
person.version = 4

// Modify existing property
person.age = 2

print person  // {name: "Nexora", age: 2, version: 4}
```

## Nested Objects

```nexora
let company = {
    name: "TechCorp",
    address: {
        street: "123 Main St",
        city: "San Francisco",
        state: "CA"
    },
    employees: [
        { name: "Alice", role: "Developer" },
        { name: "Bob", role: "Designer" }
    ]
}

print company.name                  // "TechCorp"
print company.address.city          // "San Francisco"
print company.employees[0].name     // "Alice"
```

## Object Methods

Functions stored as object properties become methods:

```nexora
let calculator = {
    add: func(a, b) { return a + b },
    subtract: func(a, b) { return a - b },
    multiply: func(a, b) { return a * b }
}

print calculator.add(10, 5)       // 15
print calculator.multiply(3, 4)   // 12
```

### Arrow Function Methods

```nexora
let math = {
    square: x => x * x,
    cube: x => x ** 3
}

print math.square(5)  // 25
print math.cube(3)    // 27
```

## Object Functions

### keys() — Get All Keys

```nexora
let person = { name: "Nexora", age: 1 }
let k = keys(person)
print k  // ["name", "age"]
```

### values() — Get All Values

```nexora
let person = { name: "Nexora", age: 1 }
let v = values(person)
print v  // ["Nexora", 1]
```

### entries() — Get Key-Value Pairs

```nexora
let person = { name: "Nexora", age: 1 }
let e = entries(person)
print e  // [["name", "Nexora"], ["age", 1]]
```

### len() — Get Number of Properties

```nexora
let person = { name: "Nexora", age: 1 }
print len(person)  // 2
```

## Iterating Objects

### For Loop with Keys

```nexora
let person = { name: "Nexora", age: 1, city: "Mumbai" }

for key in keys(person) {
    print key + ": " + str(person[key])
}
```

### For Loop with Entries

```nexora
let config = { host: "localhost", port: 8080 }

for entry in entries(config) {
    let key = entry[0]
    let value = entry[1]
    print key + " = " + str(value)
}
```

## Object Patterns

### Object as Dictionary

```nexora
let colors = {
    red: "#FF0000",
    green: "#00FF00",
    blue: "#0000FF"
}

print colors.red  // "#FF0000"
```

### Object as Namespace

```nexora
module MathUtils {
    export func square(x) {
        return x * x
    }
    
    export func cube(x) {
        return x ** 3
    }
}

print MathUtils.square(5)  // 25
print MathUtils.cube(3)    // 27
```

### Object with Methods

```nexora
let user = {
    name: "Nexora",
    age: 1,
    greet() {
        return "Hello, I'm " + this.name
    },
    getInfo() {
        return this.name + " (age " + str(this.age) + ")"
    }
}

print user.greet()     // "Hello, I'm Nexora"
print user.getInfo()   // "Nexora (age 1)"
```

### Array of Objects

```nexora
let users = [
    { name: "Alice", age: 25, active: true },
    { name: "Bob", age: 30, active: false },
    { name: "Charlie", age: 35, active: true }
]

// Filter active users
for user in users {
    if user.active {
        print user.name
    }
}

// Find user by name
let bob = find(users, u => u.name == "Bob")
print bob.age  // 30
```

## Example: Data Processing

```nexora
let employees = [
    { name: "Alice", department: "Engineering", salary: 95000 },
    { name: "Bob", department: "Design", salary: 85000 },
    { name: "Charlie", department: "Engineering", salary: 105000 },
    { name: "Diana", department: "Design", salary: 90000 }
]

// Group by department
let byDept = group_by(employees, e => e.department)
print keys(byDept)  // ["Engineering", "Design"]

// Average salary per department
for dept in keys(byDept) {
    let avg = average(map(byDept[dept], e => e.salary))
    print dept + ": $" + str(avg)
}
```
