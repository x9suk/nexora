# @opennexora/test

The official testing framework for the Nexora programming language. Provides unit testing, integration testing, and end-to-end testing capabilities with powerful assertion libraries and mocking utilities.

## Installation

```bash
npm install @opennexora/test
# or
nxm add @opennexora/test
```

## Features

- **Describe/It syntax**: Familiar test structure
- **Rich assertions**: Comprehensive assertion library
- **Mocking**: Built-in mocking and stubbing
- **Async support**: Native async/await testing
- **Code coverage**: Built-in coverage reporting
- **Watch mode**: Re-run tests on file changes
- **Parallel execution**: Run tests in parallel for speed

## Quick Start

```nx
import { describe, it, expect } from '@opennexora/test';

describe('Calculator', () => {
  it('should add two numbers', () => {
    expect(1 + 1).toBe(2);
  });

  it('should subtract two numbers', () => {
    expect(5 - 3).toBe(2);
  });
});
```

## Test Structure

### Basic Structure

```nx
import { describe, it, expect } from '@opennexora/test';

describe('StringUtils', () => {
  describe('capitalize', () => {
    it('should capitalize first letter', () => {
      expect(capitalize('hello')).toBe('Hello');
    });

    it('should handle empty string', () => {
      expect(capitalize('')).toBe('');
    });

    it('should handle single character', () => {
      expect(capitalize('a')).toBe('A');
    });
  });
});
```

### Nested Describes

```nx
describe('UserService', () => {
  describe('createUser', () => {
    describe('with valid data', () => {
      it('should create user', () => {
        // Test implementation
      });

      it('should return user object', () => {
        // Test implementation
      });
    });

    describe('with invalid data', () => {
      it('should throw error', () => {
        // Test implementation
      });
    });
  });
});
```

## Assertions

### Equality

```nx
expect(value).toBe(expected);           // Strict equality (===)
expect(value).toEqual(expected);        // Deep equality
expect(value).not.toBe(expected);       // Not equal
```

### Truthiness

```nx
expect(value).toBeTruthy();             // Boolean true
expect(value).toBeFalsy();              // Boolean false
expect(value).toBeDefined();            // Not undefined
expect(value).toBeUndefined();          // Undefined
expect(value).toBeNull();               // Null
expect(value).not.toBeNull();           // Not null
```

### Numbers

```nx
expect(value).toBeGreaterThan(3);       // Greater than
expect(value).toBeGreaterThanOrEqual(3);// Greater than or equal
expect(value).toBeLessThan(3);          // Less than
expect(value).toBeLessThanOrEqual(3);   // Less than or equal
expect(value).toBeCloseTo(0.1, 2);      // Floating point equality
```

### Strings

```nx
expect(string).toMatch('reg');          // Regex match
expect(string).toMatch(/reg/);          // Regex match
expect(string).toContain('substring');  // String contains
```

### Arrays

```nx
expect(array).toContain(item);          // Array contains item
expect(array).toHaveLength(3);          // Array length
expect(array).toEqual([1, 2, 3]);       // Array equality
```

### Objects

```nx
expect(object).toHaveProperty('key');              // Has property
expect(object).toHaveProperty('key', 'value');     // Has property with value
expect(object).toMatchObject({ key: 'value' });    // Partial match
expect(object).toEqual({ key: 'value', ... });     // Exact match
```

### Exceptions

```nx
expect(() => functionThatThrows()).toThrow();
expect(() => functionThatThrows()).toThrow('Error message');
expect(() => functionThatThrows()).toThrow(ErrorClass);
expect(async () => asyncFunctionThatThrows()).rejects.toThrow();
```

### Snapshots

```nx
expect(value).toMatchSnapshot();        // Snapshot match
expect(value).toMatchInlineSnapshot();  // Inline snapshot
```

## Async Testing

### Async/Await

```nx
describe('AsyncService', () => {
  it('should fetch data asynchronously', async () => {
    let data = await fetchData();
    expect(data).toBeDefined();
  });

  it('should handle async errors', async () => {
    await expect(asyncFunctionThatThrows()).rejects.toThrow();
  });
});
```

### Promises

```nx
describe('PromiseService', () => {
  it('should resolve promise', () => {
    return expect(promiseThatResolves()).resolves.toBe(expectedValue);
  });

  it('should reject promise', () => {
    return expect(promiseThatRejects()).rejects.toThrow();
  });
});
```

## Setup and Teardown

### Before/After Each

```nx
describe('UserService', () => {
  let service;

  beforeEach(() => {
    service = new UserService();
  });

  afterEach(() => {
    service = null;
  });

  it('should create user', () => {
    let user = service.createUser({ name: 'Alice' });
    expect(user).toBeDefined();
  });
});
```

### Before/After All

```nx
describe('DatabaseTests', () => {
  beforeAll(async () => {
    await database.connect();
  });

  afterAll(async () => {
    await database.disconnect();
  });

  it('should query users', async () => {
    let users = await database.query('SELECT * FROM users');
    expect(users).toBeDefined();
  });
});
```

## Mocking

### Mock Functions

```nx
import { describe, it, expect, mock } from '@opennexora/test';

describe('Mocking', () => {
  it('should mock function', () => {
    let mockFn = mock.fn();
    mockFn('arg1', 'arg2');
    
    expect(mockFn).toHaveBeenCalled();
    expect(mockFn).toHaveBeenCalledWith('arg1', 'arg2');
    expect(mockFn).toHaveBeenCalledTimes(1);
  });

  it('should mock return value', () => {
    let mockFn = mock.fn();
    mockFn.mockReturnValue('mocked value');
    
    let result = mockFn();
    expect(result).toBe('mocked value');
  });
});
```

### Mock Modules

```nx
import { describe, it, expect, mock } from '@opennexora/test';

// Mock entire module
mock.module('./database', () => ({
  query: mock.fn(),
  connect: mock.fn(),
  disconnect: mock.fn()
}));

describe('DatabaseService', () => {
  it('should use mocked database', async () => {
    let db = require('./database');
    db.query.mockReturnValue([{ id: 1, name: 'Alice' }]);
    
    let users = await databaseService.getUsers();
    expect(users).toEqual([{ id: 1, name: 'Alice' }]);
  });
});
```

### Spies

```nx
import { describe, it, expect, spy } from '@opennexora/test';

describe('Spying', () => {
  it('should spy on method', () => {
    let obj = {
      method: () => 'original'
    };
    
    let spyObj = spy.on(obj, 'method');
    obj.method();
    
    expect(spyObj).toHaveBeenCalled();
  });
});
```

## Test Coverage

### Running with Coverage

```bash
# Run tests with coverage
npx nexora-test --coverage

# Coverage report
# =========================
# File          | % Stmts | % Branch | % Funcs | % Lines
# ------------- | --------| -------- | ------- | -------
# src/utils.nx |   85.71  |   80.00  |  100.00 |   85.71
# src/api.nx   |   92.31  |   88.89  |  100.00 |   92.31
# ------------- | --------| -------- | ------- | -------
# All files     |   88.89  |   84.21  |  100.00 |   88.89
```

### Coverage Configuration

```nx
// nexora-test.config.nx
export default {
  coverage: {
    provider: 'v8',
    reporter: ['text', 'json', 'html'],
    exclude: [
      'node_modules/',
      'src/**/*.test.nx'
    ],
    thresholds: {
      statements: 80,
      branches: 80,
      functions: 80,
      lines: 80
    }
  }
};
```

## Watch Mode

```bash
# Run tests in watch mode
npx nexora-test --watch

# Watch specific files
npx nexora-test --watch src/**/*.test.nx
```

## Configuration

### nexora-test.config.nx

```nx
export default {
  testMatch: ['**/*.test.nx', '**/*.spec.nx'],
  testPathIgnorePatterns: ['/node_modules/', '/dist/'],
  transform: {
    '^.+\\.nx$': 'babel-jest'
  },
  moduleFileExtensions: ['nx', 'js', 'json'],
  testEnvironment: 'node',
  verbose: true,
  silent: false,
  maxWorkers: '50%',
  timeout: 10000
};
```

## Best Practices

### 1. Test Behavior, Not Implementation

```nx
// Bad - testing implementation
it('should call内部方法', () => {
  let spy = spy.on(service, '内部方法');
  service.createUser(data);
  expect(spy).toHaveBeenCalled();
});

// Good - testing behavior
it('should create user with valid data', () => {
  let user = service.createUser(data);
  expect(user).toBeDefined();
  expect(user.name).toBe(data.name);
});
```

### 2. Use Descriptive Test Names

```nx
// Bad
it('should work', () => {});

// Good
it('should return user object when given valid user data', () => {});
```

### 3. Keep Tests Independent

```nx
// Bad - tests depend on each other
it('should create user', () => {
  global.user = service.createUser(data);
});

it('should get user', () => {
  let user = service.getUser(global.user.id);
  expect(user).toBeDefined();
});

// Good - tests are independent
it('should create user', () => {
  let user = service.createUser(data);
  expect(user).toBeDefined();
});

it('should get user', () => {
  let createdUser = service.createUser(data);
  let user = service.getUser(createdUser.id);
  expect(user).toBeDefined();
});
```

### 4. Test Edge Cases

```nx
describe('divide', () => {
  it('should divide two numbers', () => {
    expect(divide(10, 2)).toBe(5);
  });

  it('should handle division by zero', () => {
    expect(() => divide(10, 0)).toThrow('Division by zero');
  });

  it('should handle negative numbers', () => {
    expect(divide(-10, 2)).toBe(-5);
  });

  it('should handle decimal numbers', () => {
    expect(divide(10, 3)).toBeCloseTo(3.333, 2);
  });
});
```

### 5. Use BeforeEach for Setup

```nx
describe('UserService', () => {
  let service;
  let testUser;

  beforeEach(() => {
    service = new UserService();
    testUser = {
      name: 'Test User',
      email: 'test@example.com'
    };
  });

  it('should create user', () => {
    let user = service.createUser(testUser);
    expect(user.name).toBe(testUser.name);
  });

  it('should update user', () => {
    let user = service.createUser(testUser);
    let updated = service.updateUser(user.id, { name: 'Updated' });
    expect(updated.name).toBe('Updated');
  });
});
```

## Migration from Other Frameworks

### From Jest

```nx
// Jest
describe('Calculator', () => {
  test('adds 1 + 2 to equal 3', () => {
    expect(add(1, 2)).toBe(3);
  });
});

// Nexora Test
describe('Calculator', () => {
  it('adds 1 + 2 to equal 3', () => {
    expect(add(1, 2)).toBe(3);
  });
});
```

### From Mocha

```nx
// Mocha
describe('Calculator', function() {
  it('adds 1 + 2 to equal 3', function() {
    expect(add(1, 2)).to.equal(3);
  });
});

// Nexora Test
describe('Calculator', () => {
  it('adds 1 + 2 to equal 3', () => {
    expect(add(1, 2)).toBe(3);
  });
});
```

## Contributing

We welcome contributions! Please see [CONTRIBUTING.md](../../CONTRIBUTING.md) for guidelines.

## License

MIT © OpenNexora Foundation