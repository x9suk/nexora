# Classes

## Table of Contents

- [Class Declaration](#class-declaration)
- [Constructor](#constructor)
- [Methods](#methods)
- [Properties](#properties)
- [Inheritance](#inheritance)
- [Super](#super)
- [Static Properties](#static-properties)
- [Encapsulation](#encapsulation)
- [Method Chaining](#method-chaining)
- [Examples](#examples)

## Class Declaration

```nexora
class Animal {
    init(name) {
        this.name = name
    }

    speak() {
        return this.name + " makes a sound"
    }
}
```

## Constructor

The `init` method is the constructor, called when creating a new instance:

```nexora
class Person {
    init(name, age) {
        this.name = name
        this.age = age
    }
}

let person = new Person("Nexora", 1)
print person.name  // "Nexora"
print person.age   // 1
```

## Methods

Methods are functions defined inside a class:

```nexora
class Calculator {
    init() {
        this.result = 0
    }

    add(value) {
        this.result += value
        return this
    }

    subtract(value) {
        this.result -= value
        return this
    }

    getResult() {
        return this.result
    }
}

let calc = new Calculator()
let result = calc.add(10).add(5).subtract(3).getResult()
print result  // 12
```

## Properties

### Instance Properties

Defined in the constructor:

```nexora
class Dog {
    init(name, breed) {
        this.name = name
        this.breed = breed
    }
}
```

### Default Properties

```nexora
class Config {
    debug = false
    verbose = true
    maxRetries = 3
}
```

## Inheritance

Use `extends` to create a subclass:

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
print rex.bark()   // "Rex barks!"
print rex.speak()  // "Rex makes a sound"
```

## Super

Use `super()` to call the parent class constructor:

```nexora
class Vehicle {
    init(make, model, year) {
        this.make = make
        this.model = model
        this.year = year
    }

    getInfo() {
        return str(this.year) + " " + this.make + " " + this.model
    }
}

class Car extends Vehicle {
    init(make, model, year, doors) {
        super(make, model, year)
        this.doors = doors
    }

    getInfo() {
        return super.getInfo() + " with " + str(this.doors) + " doors"
    }
}

let car = new Car("Toyota", "Camry", 2024, 4)
print car.getInfo()  // "2024 Toyota Camry with 4 doors"
```

### Calling Parent Methods

```nexora
class Shape {
    area() {
        return 0
    }

    describe() {
        return "Area: " + str(this.area())
    }
}

class Circle extends Shape {
    init(radius) {
        this.radius = radius
    }

    area() {
        return 3.14159 * this.radius ** 2
    }

    describe() {
        return "Circle with radius " + str(this.radius) + " - " + super.describe()
    }
}

let c = new Circle(5)
print c.describe()  // "Circle with radius 5 - Area: 78.53975"
```

## Static Properties

Define class-level properties:

```nexora
class MathUtils {
    PI = 3.14159
    E = 2.71828
}

print MathUtils.PI  // 3.14159
print MathUtils.E   // 2.71828
```

## Encapsulation

Use naming conventions to indicate private members:

```nexora
class BankAccount {
    init(owner, balance) {
        this.owner = owner
        this._balance = balance  // Convention: _ means private
    }

    getBalance() {
        return this._balance
    }

    deposit(amount) {
        if amount > 0 {
            this._balance += amount
            print "Deposited " + str(amount)
        }
    }

    withdraw(amount) {
        if amount > 0 && amount <= this._balance {
            this._balance -= amount
            print "Withdrew " + str(amount)
        } else {
            print "Invalid withdrawal"
        }
    }
}

let account = new BankAccount("Alice", 1000)
account.deposit(500)
print account.getBalance()  // 1500
account.withdraw(200)
print account.getBalance()  // 1300
```

## Method Chaining

Return `this` to enable method chaining:

```nexora
class QueryBuilder {
    init() {
        this.table = ""
        this.conditions = []
        this.orderByField = null
    }

    from(table) {
        this.table = table
        return this
    }

    where(condition) {
        push(this.conditions, condition)
        return this
    }

    orderBy(field) {
        this.orderByField = field
        return this
    }

    build() {
        let sql = "SELECT * FROM " + this.table
        if len(this.conditions) > 0 {
            sql = sql + " WHERE " + join(this.conditions, " AND ")
        }
        if this.orderByField != null {
            sql = sql + " ORDER BY " + this.orderByField
        }
        return sql
    }
}

let query = new QueryBuilder()
    .from("users")
    .where("age > 18")
    .where("active = true")
    .orderBy("name")
    .build()

print query
// "SELECT * FROM users WHERE age > 18 AND active = true ORDER BY name"
```

## Examples

### Stack Data Structure

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
}

let stack = new Stack()
stack.push(1)
stack.push(2)
stack.push(3)
print stack.pop()    // 3
print stack.peek()   // 2
print stack.size()   // 2
```

### Inheritance Hierarchy

```nexora
class Animal {
    init(name) {
        this.name = name
    }

    speak() {
        return "..."
    }

    toString() {
        return this.name + ": " + this.speak()
    }
}

class Cat extends Animal {
    init(name) {
        super(name)
    }

    speak() {
        return "Meow!"
    }
}

class Dog extends Animal {
    init(name) {
        super(name)
    }

    speak() {
        return "Woof!"
    }
}

let animals = [new Cat("Whiskers"), new Dog("Rex")]
for animal in animals {
    print animal.toString()
}
// Whiskers: Meow!
// Rex: Woof!
```
