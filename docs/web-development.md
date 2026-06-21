# Web Development with Nexora

## Table of Contents

- [HTTP Server](#http-server)
- [HTML Generation](#html-generation)
- [Request Handling](#request-handling)
- [JSON API](#json-api)
- [Static Files](#static-files)
- [Full Example](#full-example)

## HTTP Server

Use `serve()` to start an HTTP server:

```nexora
func handler(method, path) {
    return "<h1>Hello, World!</h1>"
}

serve(8080, handler)
```

### Handler Function

The handler function receives:
- `method` — HTTP method (GET, POST, etc.)
- `path` — Request path

Return an HTML string as the response.

### Multiple Routes

```nexora
func handler(method, path) {
    if path == "/" {
        return "<h1>Home Page</h1>"
    }
    if path == "/about" {
        return "<h1>About Us</h1>"
    }
    if path == "/contact" {
        return "<h1>Contact Page</h1>"
    }
    return "<h1>404 Not Found</h1>"
}

serve(8080, handler)
```

## HTML Generation

Nexora includes built-in HTML helper functions:

### Basic Elements

```nexora
print div("Hello, World!")           // <div>Hello, World!</div>
print h1("Title")                     // <h1>Title</h1>
print h2("Subtitle")                  // <h2>Subtitle</h2>
print p("Paragraph text")             // <p>Paragraph text</p>
print span("Inline text")             // <span>Inline text</span>
```

### Lists

```nexora
let items = ["Item 1", "Item 2", "Item 3"]
print ul(items)    // <ul><li>Item 1</li><li>Item 2</li><li>Item 3</li></ul>
print li("Item")   // <li>Item</li>
```

### Links and Images

```nexora
print a("Click me", "https://example.com")
// <a href="https://example.com">Click me</a>

print img("photo.jpg", "A photo")
// <img src="photo.jpg" alt="A photo">
```

### Forms

```nexora
print form("POST", "/submit", [
    input("text", "name", "Your name"),
    input("submit", "", "Submit")
])
```

### Full HTML Page

```nexora
func page(title, content) {
    return """
    <!DOCTYPE html>
    <html>
    <head>
        <title>${title}</title>
        <style>
            body { font-family: sans-serif; margin: 40px; }
            h1 { color: #333; }
        </style>
    </head>
    <body>
        ${content}
    </body>
    </html>
    """
}

func handler(method, path) {
    let content = div([
        h1("Welcome to Nexora!"),
        p("A simple, clean programming language."),
        ul(["Fast", "Simple", "Clean"])
    ])
    return page("Nexora Website", content)
}

serve(8080, handler)
```

## Request Handling

### GET Requests

```nexora
func handler(method, path) {
    if method == "GET" && path == "/" {
        return "<h1>Home Page</h1>"
    }
    if method == "GET" && path == "/api/data" {
        let data = { message: "Hello", status: "ok" }
        return json_stringify(data)
    }
    return "<h1>404 Not Found</h1>"
}
```

### POST Requests

```nexora
func handler(method, path) {
    if method == "POST" && path == "/submit" {
        // Process form submission
        return "<h1>Submitted!</h1>"
    }
    return "<h1>Not Found</h1>"
}
```

## JSON API

### Serving JSON

```nexora
func handler(method, path) {
    if path == "/api/users" {
        let users = [
            { name: "Alice", age: 25 },
            { name: "Bob", age: 30 }
        ]
        return json_stringify(users)
    }
    return json_stringify({ error: "Not found" })
}
```

### Parsing JSON

```nexora
func handler(method, path) {
    if method == "POST" {
        let jsonStr = '{"name": "Nexora"}'
        let data = json_parse(jsonStr)
        return "<h1>Hello, " + data.name + "!</h1>"
    }
}
```

## Static Files

### Serving HTML Templates

```nexora
func layout(title, body) {
    return """
    <!DOCTYPE html>
    <html>
    <head><title>${title}</title></head>
    <body>${body}</body>
    </html>
    """
}

func homePage() {
    return layout("Home", div([
        h1("Welcome"),
        p("This is the home page.")
    ]))
}

func aboutPage() {
    return layout("About", div([
        h1("About Us"),
        p("We build Nexora.")
    ]))
}

func handler(method, path) {
    if path == "/" { return homePage() }
    if path == "/about" { return aboutPage() }
    return layout("404", h1("Page Not Found"))
}

serve(8080, handler)
```

## Full Example

### Blog Application

```nexora
import { json_parse, json_stringify } from "json"

let posts = [
    { id: 1, title: "First Post", content: "Hello World!" },
    { id: 2, title: "Second Post", content: "Nexora is awesome!" }
]

func renderPage(title, content) {
    return """
    <!DOCTYPE html>
    <html>
    <head>
        <title>${title}</title>
        <style>
            body { font-family: sans-serif; max-width: 800px; margin: 0 auto; padding: 20px; }
            .post { border: 1px solid #ddd; padding: 15px; margin: 10px 0; }
            h1 { color: #333; }
            a { color: #0066cc; }
        </style>
    </head>
    <body>
        <h1><a href="/">My Blog</a></h1>
        ${content}
    </body>
    </html>
    """
}

func homePage() {
    let items = []
    for post in posts {
        push(items, div({ class: "post" }, [
            h2(a(post.title, "/post/" + str(post.id))),
            p(post.content)
        ]))
    }
    return renderPage("Home", div(items))
}

func postPage(id) {
    for post in posts {
        if post.id == id {
            return renderPage(post.title, div([
                h1(post.title),
                p(post.content)
            ]))
        }
    }
    return renderPage("Not Found", h1("Post not found"))
}

func handler(method, path) {
    if path == "/" {
        return homePage()
    }
    if starts_with(path, "/post/") {
        let id = parseInt(slice(path, 6, len(path)))
        return postPage(id)
    }
    return renderPage("404", h1("Page Not Found"))
}

print "Blog running at http://localhost:8080"
serve(8080, handler)
```

### REST API

```nexora
import { json_parse, json_stringify } from "json"

let todos = [
    { id: 1, text: "Learn Nexora", done: false },
    { id: 2, text: "Build something", done: false }
]
let nextId = 3

func handler(method, path) {
    // CORS headers
    if method == "OPTIONS" {
        return ""
    }
    
    // GET /todos
    if method == "GET" && path == "/todos" {
        return json_stringify(todos)
    }
    
    // GET /todos/:id
    if method == "GET" && starts_with(path, "/todos/") {
        let id = parseInt(slice(path, 7, len(path)))
        for todo in todos {
            if todo.id == id {
                return json_stringify(todo)
            }
        }
        return json_stringify({ error: "Not found" })
    }
    
    // POST /todos
    if method == "POST" && path == "/todos" {
        let todo = { id: nextId, text: "New todo", done: false }
        nextId += 1
        push(todos, todo)
        return json_stringify(todo)
    }
    
    return json_stringify({ error: "Not found" })
}

print "API running at http://localhost:3000"
serve(3000, handler)
```
