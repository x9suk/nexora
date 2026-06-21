# nexora-http

HTTP client for the Nexora runtime. Supports interceptors, timeouts, and JSON parsing.

## Installation

```bash
nxm install nexora-http
```

## Quick Start

```nx
import http from "nexora-http";

const client = http("https://api.example.com");

const res = await client.get("/users");
console.log(res.data);
```

## Creating a Client

```nx
import { create, HttpClient } from "nexora-http";

// Simple
const api = create("https://api.example.com");

// With defaults
const api = create("https://api.example.com", {
  headers: { "X-API-Key": "secret" },
  timeout: 5000,
});

// Manual
const client = new HttpClient({
  baseURL: "https://api.example.com",
  headers: { Accept: "application/json" },
});
```

## Methods

```nx
// GET
const res = await client.get("/users");

// POST
const res = await client.post("/users", { name: "Alice" });

// PUT
const res = await client.put("/users/1", { name: "Bob" });

// DELETE
const res = await client.delete("/users/1");

// PATCH
const res = await client.patch("/users/1", { name: "Charlie" });
```

## Response Object

```nx
const res = await client.get("/users");

res.status;    // 200
res.ok;        // true (status 2xx)
res.data;      // parsed JSON or string
res.headers;   // response headers
```

## Headers

```nx
client.setHeader("X-Custom", "value");

client.setAuth("my-token");
// Sets Authorization: Bearer my-token
```

## Timeouts

```nx
// Global
client.setTimeout(5000);

// Per-request
await client.get("/slow", { timeout: 10000 });
```

## Interceptors

### Request Interceptor

```nx
client.interceptRequest((config) => {
  config.headers["X-Request-Time"] = Date.now().toString();
  console.log(`${config.method} ${config.url}`);
  return config;
});
```

### Response Interceptor

```nx
client.interceptResponse((response) => {
  console.log(`Status: ${response.status}`);
  return response;
});
```

### Chaining Interceptors

```nx
client
  .interceptRequest((config) => {
    config.headers["X-Token"] = getToken();
    return config;
  })
  .interceptResponse((res) => {
    if (!res.ok) {
      throw new Error(`HTTP ${res.status}`);
    }
    return res;
  });
```

## Error Handling

```nx
try {
  const res = await client.get("/data");
} catch (err) {
  console.error("Request failed:", err.message);
}
```

## Full Example

```nx
import { create } from "nexora-http";

const api = create("https://jsonplaceholder.typicode.com");

// Add auth
api.setAuth("my-api-token");

// Log requests
api.interceptRequest((config) => {
  console.log(`[${config.method}] ${config.url}`);
  return config;
});

// Handle errors
api.interceptResponse((res) => {
  if (!res.ok) {
    console.error(`Error: ${res.status}`);
  }
  return res;
});

// Get posts
const posts = await api.get("/posts");
console.log(posts.data);

// Create post
const newPost = await api.post("/posts", {
  title: "Hello",
  body: "World",
  userId: 1,
});
console.log(newPost.data);

// Update post
await api.put("/posts/1", {
  title: "Updated",
  body: "Content",
  userId: 1,
});

// Delete post
await api.delete("/posts/1");
```

## License

MIT
