# express-nx

Web framework for the Nexora runtime. Provides routing, middleware, and HTTP server functionality.

## Installation

```bash
nxm install express-nx
```

## Quick Start

```nx
import express from "express-nx";

const app = express();

app.get("/", (req, res) => {
  res.json({ message: "Hello World" });
});

app.listen(3000, () => {
  console.log("Server running on port 3000");
});
```

## Routing

### Basic Routes

```nx
app.get("/users", (req, res) => {
  res.json({ users: [] });
});

app.post("/users", (req, res) => {
  res.status(201).json({ id: 1, name: req.body.name });
});

app.put("/users/:id", (req, res) => {
  res.json({ id: req.params.id, ...req.body });
});

app.delete("/users/:id", (req, res) => {
  res.status(204).end();
});
```

### Route Parameters

```nx
app.get("/users/:id", (req, res) => {
  console.log(req.params.id); // "42"
  res.json({ id: req.params.id });
});

app.get("/posts/:postId/comments/:commentId", (req, res) => {
  const { postId, commentId } = req.params;
  res.json({ postId, commentId });
});
```

### Query Parameters

```nx
app.get("/search", (req, res) => {
  const { q, page, limit } = req.query;
  res.json({ query: q, page, limit });
});
// GET /search?q=nexora&page=1&limit=10
```

### Route Groups

```nx
const router = express.Router();

router.get("/", (req, res) => { /* list */ });
router.get("/:id", (req, res) => { /* get */ });
router.post("/", (req, res) => { /* create */ });

app.use("/api/users", router);
```

## Middleware

### Built-in Middleware

```nx
import express, { json, staticFiles, cors } from "express-nx";

app.use(json());           // Parse JSON bodies
app.use(cors());           // Enable CORS
app.use(staticFiles("./public")); // Serve static files
```

### Custom Middleware

```nx
app.use((req, res, next) => {
  console.log(`${req.method} ${req.path}`);
  next();
});

app.use((req, res, next) => {
  req.startTime = Date.now();
  next();
  const duration = Date.now() - req.startTime;
  console.log(`Response time: ${duration}ms`);
});
```

### Route-specific Middleware

```nx
function auth(req, res, next) {
  const token = req.headers.authorization;
  if (!token) {
    return res.status(401).json({ error: "Unauthorized" });
  }
  next();
}

app.get("/admin", auth, (req, res) => {
  res.json({ message: "Admin panel" });
});
```

## Request Properties

```nx
app.get("/example", (req, res) => {
  req.method;     // "GET"
  req.url;        // "/example?q=1"
  req.path;       // "/example"
  req.params;     // { id: "123" }
  req.query;      // { q: "1" }
  req.body;       // parsed body
  req.headers;    // { host: "...", ... }
  res.json({ ok: true });
});
```

## Response Methods

```nx
app.get("/demo", (req, res) => {
  // JSON response
  res.json({ data: "hello" });

  // Status + JSON
  res.status(201).json({ created: true });

  // HTML response
  res.send("<h1>Hello</h1>");

  // Redirect
  res.redirect("/login");

  // Set headers
  res.set("X-Custom", "value").json({ ok: true });

  // End with status
  res.status(204).end();
});
```

## Error Handling

```nx
app.use((err, req, res, next) => {
  console.error(err.stack);
  res.status(500).json({ error: "Something went wrong" });
});
```

## Static Files

```nx
app.use(staticFiles("./public"));
// Serves ./public/index.html at GET /
// Serves ./public/style.css at GET /style.css
```

## CORS

```nx
app.use(cors());
// Allow all origins

app.use(cors({ origin: "https://example.com" }));
// Allow specific origin
```

## Full Example

```nx
import express, { json, cors, staticFiles } from "express-nx";

const app = express();

app.use(cors());
app.use(json());
app.use(staticFiles("./public"));

let users = [
  { id: 1, name: "Alice" },
  { id: 2, name: "Bob" },
];

app.get("/api/users", (req, res) => {
  res.json(users);
});

app.get("/api/users/:id", (req, res) => {
  const user = users.find((u) => u.id === parseInt(req.params.id));
  if (!user) return res.status(404).json({ error: "Not found" });
  res.json(user);
});

app.post("/api/users", (req, res) => {
  const user = { id: users.length + 1, ...req.body };
  users.push(user);
  res.status(201).json(user);
});

app.listen(3000, () => {
  console.log("API running on http://localhost:3000");
});
```

## License

MIT
