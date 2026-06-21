# package-name

Package description for the Nexora runtime.

## Installation

```bash
nxm install package-name
```

## Usage

```nx
import { hello } from "package-name";

console.log(hello("Nexora"));
// Output: Hello, Nexora!
```

## API

### `hello(name?: string): string`

Returns a greeting string.

| Param   | Type     | Default   | Description          |
|---------|----------|-----------|----------------------|
| `name`  | `string` | `"World"` | Name to greet        |

## License

MIT
