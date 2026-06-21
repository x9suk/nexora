# nexora-db

Database ORM for the Nexora runtime. Supports schema definition, queries, and transactions.

## Installation

```bash
nxm install nexora-db
```

## Quick Start

```nx
import { createDatabase } from "nexora-db";

const db = createDatabase({ type: "sqlite", file: "./data.db" });
await db.connect();
```

## Schema Definition

```nx
const users = db.schema("users");

users
  .integer("id").primary()
  .varchar("name", 255).notNull()
  .varchar("email", 255).unique().notNull()
  .boolean("active").default(true)
  .datetime("createdAt");

await users.createTable();
```

### Column Types

```nx
table.integer("count");
table.text("bio");
table.varchar("name", 100);
table.boolean("active");
table.datetime("createdAt");
table.decimal("price", 10, 2);
```

### Column Constraints

```nx
table.integer("id").primary();
table.text("name").notNull();
table.varchar("email").unique();
table.text("status").default("active");
```

## CRUD Operations

### Insert

```nx
const user = await users.create({
  name: "Alice",
  email: "alice@example.com",
});
console.log(user.id); // 1
```

### Find

```nx
// By ID
const user = await users.find(1);

// One record
const user = await users.findOne({ email: "alice@example.com" });

// Many records
const activeUsers = await users.findMany({ active: true });
```

### Update

```nx
await users.update({ name: "Bob" }, { id: 1 });
```

### Delete

```nx
await users.delete({ id: 1 });
```

## Query Builder

### Select

```nx
const results = await users
  .select("id", "name")
  .where({ active: true })
  .orderBy("name", "ASC")
  .limit(10)
  .offset(0)
  .findMany();
```

### Where Clauses

```nx
users.where({ active: true });
users.where("age > ?", [18]);
users.where("name LIKE ?", ["%alice%"]);
```

### Order and Pagination

```nx
users.orderBy("createdAt", "DESC");
users.limit(10);
users.offset(20);
```

### Joins

```nx
users.join("posts", "posts.userId = users.id").findMany();
```

## Aggregates

```nx
const count = await users.count({ active: true });
const exists = await users.exists({ email: "alice@example.com" });
```

## Raw Queries

```nx
const result = await db.query(
  "SELECT * FROM users WHERE age > ?",
  [18]
);
console.log(result.rows);
```

## Transactions

```nx
await db.begin();
try {
  await db.insert("accounts", { userId: 1, balance: 100 });
  await db.insert("accounts", { userId: 2, balance: -100 });
  await db.commit();
} catch (err) {
  await db.rollback();
  throw err;
}
```

## Full Example

```nx
import { createDatabase } from "nexora-db";

const db = createDatabase({ type: "sqlite", file: "./app.db" });
await db.connect();

// Define schema
const users = db.schema("users");
users
  .integer("id").primary()
  .varchar("name", 100).notNull()
  .varchar("email", 200).unique()
  .boolean("active").default(true);

const posts = db.schema("posts");
posts
  .integer("id").primary()
  .integer("userId").notNull()
  .text("title").notNull()
  .text("body");

await users.createTable();
await posts.createTable();

// Create user
const user = await users.create({
  name: "Alice",
  email: "alice@example.com",
});

// Create posts
await posts.create({
  userId: user.id,
  title: "Hello World",
  body: "My first post",
});

// Query posts with user
const results = await posts
  .select("posts.*", "users.name as author")
  .join("users", "posts.userId = users.id")
  .orderBy("createdAt", "DESC")
  .limit(10)
  .findMany();

console.log(results);

await db.disconnect();
```

## License

MIT
