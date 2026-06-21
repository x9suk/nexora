# nexora-auth

Authentication library for the Nexora runtime. JWT tokens, sessions, and user management.

## Installation

```bash
nxm install nexora-auth
```

## Quick Start

```nx
import { createAuth } from "nexora-auth";

const auth = createAuth({ secret: "your-secret-key" });

// Register
const user = await auth.register({
  email: "alice@example.com",
  password: "secret123",
  name: "Alice",
});

// Login
const { token, refreshToken } = await auth.login("alice@example.com", "secret123");

// Verify token
const payload = auth.verifyToken(token);
console.log(payload.sub); // user ID
```

## AuthManager

### `createAuth(options)`

```nx
const auth = createAuth({
  secret: "your-secret-key",    // JWT signing secret
  tokenExpiry: 3600,            // Access token expiry (seconds)
  refreshExpiry: 604800,        // Refresh token expiry (seconds)
});
```

### `register(data)`

```nx
const user = await auth.register({
  email: "bob@example.com",
  password: "securepass",
  name: "Bob",
  role: "admin", // optional, defaults to "user"
});

// Returns: { id, email, name, role, createdAt }
```

### `login(email, password)`

```nx
const result = await auth.login("bob@example.com", "securepass");

// Returns:
{
  user: { id, email, name, role },
  token: "eyJhbG...",
  refreshToken: "eyJhbG...",
  sessionId: "abc123",
}
```

### `logout(sessionId)`

```nx
await auth.logout(sessionId);
```

### `verifyToken(token)`

```nx
const payload = auth.verifyToken(token);
console.log(payload.sub);  // user ID
console.log(payload.email); // user email
console.log(payload.exp);   // expiry timestamp
```

### `refresh(refreshToken)`

```nx
const { token, refreshToken: newRefresh } = await auth.refresh(refreshToken);
```

### `getUser(userId)`

```nx
const user = auth.getUser(userId);
// { id, email, name, role, createdAt }
```

## Middleware

```nx
app.use("/protected", auth.middleware(), (req, res) => {
  res.json({ user: req.user });
});
```

## Hooks

```nx
auth.on("beforeRegister", async (data) => {
  console.log("Registering:", data.email);
});

auth.on("afterRegister", async (user) => {
  await sendWelcomeEmail(user.email);
});

auth.on("beforeLogin", async (user) => {
  console.log("Login attempt:", user.email);
});

auth.on("afterLogin", async (result) => {
  await logActivity(result.user.id, "login");
});
```

## JWT Functions

### `sign(payload, secret, options)`

```nx
import { sign } from "nexora-auth";

const token = sign(
  { sub: "user-123", role: "admin" },
  "secret",
  { expiresIn: 3600 }
);
```

### `verify(token, secret)`

```nx
import { verify } from "nexora-auth";

const payload = verify(token, "secret");
// { sub: "user-123", role: "admin", iat: ..., exp: ... }
```

### `decode(token)`

```nx
import { decode } from "nexora-auth";

const payload = decode(token);
// Without verifying signature
```

## Password Hashing

```nx
import { hashPassword, verifyPassword } from "nexora-auth";

// Hash
const { hash, salt } = hashPassword("mypassword");

// Verify
const valid = verifyPassword("mypassword", hash, salt);
// true
```

## Sessions

```nx
import { SessionStore } from "nexora-auth";

const store = new SessionStore();

// Create session
const session = store.create(userId, { ip: "127.0.0.1" });

// Get session
const s = store.get(sessionId);

// Delete session
store.delete(sessionId);

// Destroy all sessions for a user
store.destroy(userId);

// Clean expired sessions
store.clean();
```

## Full Example

```nx
import { createAuth } from "nexora-auth";
import express, { json } from "express-nx";

const app = express();
app.use(json());

const auth = createAuth({
  secret: "super-secret-key",
  tokenExpiry: 3600,
});

// Register
app.post("/register", async (req, res) => {
  try {
    const user = await auth.register(req.body);
    res.status(201).json(user);
  } catch (err) {
    res.status(400).json({ error: err.message });
  }
});

// Login
app.post("/login", async (req, res) => {
  try {
    const result = await auth.login(req.body.email, req.body.password);
    res.json(result);
  } catch (err) {
    res.status(401).json({ error: err.message });
  }
});

// Protected route
app.get("/profile", auth.middleware(), (req, res) => {
  res.json({ user: req.user });
});

// Refresh token
app.post("/refresh", async (req, res) => {
  try {
    const tokens = await auth.refresh(req.body.refreshToken);
    res.json(tokens);
  } catch (err) {
    res.status(401).json({ error: err.message });
  }
});

app.listen(3000);
```

## License

MIT
