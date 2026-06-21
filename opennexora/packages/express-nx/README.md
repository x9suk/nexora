# @opennexora/express-nx

A fast, unopinionated, minimalist web framework for the Nexora programming language. Built on top of Nexora's async/await primitives and optimized for performance.

## Installation

```bash
npm install @opennexora/express-nx
# or
nxm add @opennexora/express-nx
```

## Features

- **Express-compatible API**: Easy migration from Express.js
- **Async/await native**: First-class support for async handlers
- **Type-safe**: Full Nexora type definitions
- **High performance**: Optimized for Nexora's runtime
- **Middleware support**: Extensible middleware system
- **Built-in utilities**: Common functionality included

## Quick Start

```nx
import express from '@opennexora/express-nx';

let app = express();

// Simple route
app.get('/', (req, res) => {
  res.json({ message: 'Hello, Nexora!' });
});

// Start server
app.listen(3000, () => {
  console.log('Server running on port 3000');
});
```

## Core Concepts

### Routing

```nx
import express from '@opennexora/express-nx';

let app = express();

// Basic routes
app.get('/users', (req, res) => {
  res.json({ users: [] });
});

app.post('/users', (req, res) => {
  let user = req.body;
  res.status(201).json(user);
});

app.get('/users/:id', (req, res) => {
  let { id } = req.params;
  res.json({ user: { id } });
});

// Route handlers
app.route('/books')
  .get((req, res) => {
    res.json({ books: [] });
  })
  .post((req, res) => {
    let book = req.body;
    res.status(201).json(book);
  });
```

### Middleware

```nx
import express from '@opennexora/express-nx';

let app = express();

// Built-in middleware
app.use(express.json());
app.use(express.urlencoded({ extended: true }));

// Custom middleware
let logger = (req, res, next) => {
  console.log(`${req.method} ${req.path}`);
  next();
};

app.use(logger);

// Error handling middleware
app.use((err, req, res, next) => {
  console.error(err.stack);
  res.status(500).json({ error: 'Something went wrong!' });
});
```

### Async/Await Support

```nx
import express from '@opennexora/express-nx';

let app = express();

// Async route handlers work natively
app.get('/users', async (req, res) => {
  let users = await database.getUsers();
  res.json({ users });
});

app.post('/users', async (req, res) => {
  try {
    let user = await database.createUser(req.body);
    res.status(201).json(user);
  } catch (error) {
    res.status(400).json({ error: error.message });
  }
});
```

### Request/Response

```nx
import express from '@opennexora/express-nx';

let app = express();

app.get('/example', (req, res) => {
  // Request properties
  console.log(req.method);      // 'GET'
  console.log(req.path);        // '/example'
  console.log(req.query);       // URL query parameters
  console.log(req.body);        // Request body
  console.log(req.headers);     // Request headers
  console.log(req.params);      // Route parameters

  // Response methods
  res.status(200).json({ success: true });
  res.send('Hello World');
  res.render('template', { data: {} });
  res.redirect('/other');
  res.cookie('name', 'value');
  res.clearCookie('name');
});
```

## Advanced Features

### Router

```nx
import express, { Router } from '@opennexora/express-nx';

let app = express();
let apiRouter = Router();

// Nested routes
apiRouter.get('/users', (req, res) => {
  res.json({ users: [] });
});

apiRouter.get('/posts', (req, res) => {
  res.json({ posts: [] });
});

// Mount router
app.use('/api', apiRouter);
```

### Template Engines

```nx
import express from '@opennexora/express-nx';

let app = express();

// Set template engine
app.set('view engine', 'ejs');

// Render templates
app.get('/profile', (req, res) => {
  res.render('profile', {
    name: 'Alice',
    age: 25
  });
});
```

### Static Files

```nx
import express from '@opennexora/express-nx';

let app = express();

// Serve static files
app.use(express.static('public'));

// With options
app.use(express.static('public', {
  maxAge: '1d',
  etag: true
}));
```

### Error Handling

```nx
import express from '@opennexora/express-nx';

let app = express();

// Async error handling
app.get('/async-route', async (req, res, next) => {
  try {
    let result = await someAsyncOperation();
    res.json(result);
  } catch (error) {
    next(error);
  }
});

// Global error handler
app.use((err, req, res, next) => {
  console.error(err.stack);
  
  if (err.type === 'validation') {
    return res.status(400).json({ error: err.message });
  }
  
  res.status(500).json({ error: 'Internal server error' });
});
```

## Built-in Middleware

### Body Parsing

```nx
import express from '@opennexora/express-nx';

let app = express();

// JSON parsing
app.use(express.json({ limit: '10mb' }));

// URL-encoded parsing
app.use(express.urlencoded({ extended: true }));

// Raw body parsing
app.use(express.raw({ type: 'application/octet-stream' }));
```

### Security

```nx
import express from '@opennexora/express-nx';

let app = express();

// CORS
app.use((req, res, next) => {
  res.header('Access-Control-Allow-Origin', '*');
  res.header('Access-Control-Allow-Methods', 'GET, POST, PUT, DELETE');
  res.header('Access-Control-Allow-Headers', 'Content-Type, Authorization');
  next();
});

// Helmet-style security headers
app.use((req, res, next) => {
  res.header('X-Content-Type-Options', 'nosniff');
  res.header('X-Frame-Options', 'DENY');
  res.header('X-XSS-Protection', '1; mode=block');
  next();
});
```

## TypeScript Integration

Full type definitions are included:

```nx
import express, { Request, Response, NextFunction } from '@opennexora/express-nx';

interface User {
  id: Number;
  name: String;
  email: String;
}

let app = express();

// Typed route handlers
app.get('/users/:id', (req: Request, res: Response) => {
  let { id } = req.params;
  // TypeScript knows id is a string
  res.json({ user: { id, name: 'Alice' } });
});

// Typed middleware
let authMiddleware = (req: Request, res: Response, next: NextFunction) => {
  let token = req.headers.authorization;
  if (!token) {
    return res.status(401).json({ error: 'Unauthorized' });
  }
  next();
};

app.use(authMiddleware);
```

## Performance

express-nx is optimized for Nexora's runtime:

- **Connection pooling**: Efficient database connections
- **Compression**: Built-in gzip/deflate support
- **Caching**: HTTP caching headers
- **Keep-alive**: Persistent connections

```nx
import express from '@opennexora/express-nx';

let app = express();

// Enable compression
app.use((req, res, next) => {
  res.header('Content-Encoding', 'gzip');
  next();
});

// Enable keep-alive
app.use((req, res, next) => {
  res.header('Connection', 'keep-alive');
  next();
});
```

## Migration from Express.js

### Step 1: Update Imports

```nx
// Before (JavaScript)
const express = require('express');

// After (Nexora)
import express from '@opennexora/express-nx';
```

### Step 2: Update Route Handlers

```nx
// Before (JavaScript)
app.get('/users', function(req, res) {
  res.json({ users: [] });
});

// After (Nexora)
app.get('/users', (req, res) => {
  res.json({ users: [] });
});
```

### Step 3: Add Types (Optional)

```nx
import express, { Request, Response } from '@opennexora/express-nx';

app.get('/users', (req: Request, res: Response) => {
  res.json({ users: [] });
});
```

## Examples

### REST API

```nx
import express from '@opennexora/express-nx';

let app = express();
app.use(express.json());

let users = [];

// CRUD operations
app.get('/users', (req, res) => {
  res.json({ users });
});

app.post('/users', (req, res) => {
  let user = { id: users.length + 1, ...req.body };
  users.push(user);
  res.status(201).json(user);
});

app.put('/users/:id', (req, res) => {
  let { id } = req.params;
  let index = users.findIndex(u => u.id === parseInt(id));
  if (index === -1) {
    return res.status(404).json({ error: 'User not found' });
  }
  users[index] = { ...users[index], ...req.body };
  res.json(users[index]);
});

app.delete('/users/:id', (req, res) => {
  let { id } = req.params;
  users = users.filter(u => u.id !== parseInt(id));
  res.status(204).send();
});

app.listen(3000);
```

### WebSocket Server

```nx
import express from '@opennexora/express-nx';
import { WebSocketServer } from 'ws';

let app = express();
let server = app.listen(3000);
let wss = new WebSocketServer({ server });

wss.on('connection', (ws) => {
  console.log('Client connected');
  
  ws.on('message', (message) => {
    console.log('Received:', message);
    ws.send(`Echo: ${message}`);
  });
  
  ws.on('close', () => {
    console.log('Client disconnected');
  });
});
```

## Contributing

We welcome contributions! Please see [CONTRIBUTING.md](../../CONTRIBUTING.md) for guidelines.

## License

MIT © OpenNexora Foundation