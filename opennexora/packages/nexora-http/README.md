# @opennexora/http

A modern, feature-rich HTTP client for the Nexora programming language. Built with async/await support, interceptors, and comprehensive request/response handling.

## Installation

```bash
npm install @opennexora/http
# or
nxm add @opennexora/http
```

## Features

- **Promise-based**: Full async/await support
- **Request/Response interceptors**: Transform requests and responses
- **Automatic transforms**: JSON, FormData, and more
- **Error handling**: Comprehensive error types
- **Timeout support**: Configurable request timeouts
- **Cancel tokens**: Cancel requests in flight
- **Browser & Node.js**: Works in both environments

## Quick Start

```nx
import http from '@opennexora/http';

// Basic GET request
let response = await http.get('https://api.example.com/users');
console.log(response.data);

// POST request with data
let user = await http.post('https://api.example.com/users', {
  name: 'Alice',
  email: 'alice@example.com'
});
console.log(user.data);
```

## HTTP Methods

### GET Request

```nx
import http from '@opennexora/http';

// Simple GET
let response = await http.get('/api/users');

// GET with query parameters
let response = await http.get('/api/users', {
  params: {
    page: 1,
    limit: 10
  }
});

// GET with headers
let response = await http.get('/api/users', {
  headers: {
    'Authorization': 'Bearer token123'
  }
});
```

### POST Request

```nx
import http from '@opennexora/http';

// Simple POST
let response = await http.post('/api/users', {
  name: 'Alice',
  email: 'alice@example.com'
});

// POST with custom headers
let response = await http.post('/api/users', userData, {
  headers: {
    'Content-Type': 'application/json'
  }
});

// POST with FormData
let formData = new FormData();
formData.append('file', fileBlob);
let response = await http.post('/api/upload', formData);
```

### PUT Request

```nx
import http from '@opennexora/http';

let response = await http.put('/api/users/1', {
  name: 'Alice Updated'
});
```

### PATCH Request

```nx
import http from '@opennexora/http';

let response = await http.patch('/api/users/1', {
  email: 'newemail@example.com'
});
```

### DELETE Request

```nx
import http from '@opennexora/http';

let response = await http.delete('/api/users/1');
```

## Configuration

### Creating an Instance

```nx
import http from '@opennexora/http';

// Create configured instance
let api = http.create({
  baseURL: 'https://api.example.com',
  timeout: 5000,
  headers: {
    'Authorization': 'Bearer token123'
  }
});

// Use configured instance
let response = await api.get('/users');
```

### Request Config

```nx
let config = {
  url: '/users',
  method: 'get',
  baseURL: 'https://api.example.com',
  params: { page: 1 },
  data: { name: 'Alice' },
  headers: { 'Content-Type': 'application/json' },
  timeout: 5000,
  responseType: 'json'
};

let response = await http.request(config);
```

## Interceptors

### Request Interceptors

```nx
import http from '@opennexora/http';

// Add request interceptor
http.interceptors.request.use(
  (config) => {
    // Add auth token to all requests
    let token = getAuthToken();
    config.headers.Authorization = `Bearer ${token}`;
    return config;
  },
  (error) => {
    return Promise.reject(error);
  }
);
```

### Response Interceptors

```nx
import http from '@opennexora/http';

// Add response interceptor
http.interceptors.response.use(
  (response) => {
    // Transform response data
    if (response.data && response.data.pagination) {
      response.data.items = response.data.data;
      delete response.data.data;
    }
    return response;
  },
  (error) => {
    // Handle errors globally
    if (error.response?.status === 401) {
      // Redirect to login
      window.location.href = '/login';
    }
    return Promise.reject(error);
  }
);
```

### Ejecting Interceptors

```nx
let interceptor = http.interceptors.request.use((config) => {
  // Do something
  return config;
});

// Remove interceptor
http.interceptors.request.eject(interceptor);
```

## Error Handling

### Basic Error Handling

```nx
import http from '@opennexora/http';

try {
  let response = await http.get('/api/users');
} catch (error) {
  if (error.response) {
    // Server responded with error status
    console.error('Status:', error.response.status);
    console.error('Data:', error.response.data);
  } else if (error.request) {
    // Request made but no response
    console.error('No response received');
  } else {
    // Error setting up request
    console.error('Error:', error.message);
  }
}
```

### Custom Error Handler

```nx
import http from '@opennexora/http';

function handleApiError(error) {
  if (error.response) {
    switch (error.response.status) {
      case 400:
        return { error: 'Bad Request', details: error.response.data };
      case 401:
        return { error: 'Unauthorized' };
      case 403:
        return { error: 'Forbidden' };
      case 404:
        return { error: 'Not Found' };
      case 500:
        return { error: 'Server Error' };
      default:
        return { error: 'Unknown Error' };
    }
  }
  return { error: 'Network Error' };
}

try {
  let response = await http.get('/api/users');
} catch (error) {
  let result = handleApiError(error);
  console.error(result);
}
```

## Timeouts

```nx
import http from '@opennexora/http';

// Set timeout for specific request
let response = await http.get('/api/slow-endpoint', {
  timeout: 10000 // 10 seconds
});

// Set default timeout
http.defaults.timeout = 5000;

// Handle timeout errors
try {
  let response = await http.get('/api/slow-endpoint');
} catch (error) {
  if (error.code === 'ECONNABORTED') {
    console.error('Request timeout');
  }
}
```

## Cancel Tokens

```nx
import http, { CancelToken } from '@opennexora/http';

// Create cancel token
let cancelToken = CancelToken.source();

try {
  let response = await http.get('/api/users', {
    cancelToken: cancelToken.token
  });
} catch (error) {
  if (http.isCancel(error)) {
    console.log('Request cancelled');
  }
}

// Cancel the request
cancelToken.cancel('Operation cancelled by user');
```

## Request/Response Transforms

### Transform Request Data

```nx
import http from '@opennexora/http';

// Transform request data
http.interceptors.request.use((config) => {
  if (config.data && typeof config.data === 'object') {
    // Convert to snake_case for API
    config.data = camelToSnake(config.data);
  }
  return config;
});
```

### Transform Response Data

```nx
import http from '@opennexora/http';

// Transform response data
http.interceptors.response.use((response) => {
  if (response.data && typeof response.data === 'object') {
    // Convert to camelCase for frontend
    response.data = snakeToCamel(response.data);
  }
  return response;
});
```

## File Upload

```nx
import http from '@opennexora/http';

// Upload file
let formData = new FormData();
formData.append('file', fileBlob, 'filename.jpg');
formData.append('description', 'My photo');

let response = await http.post('/api/upload', formData, {
  headers: {
    'Content-Type': 'multipart/form-data'
  },
  onUploadProgress: (progressEvent) => {
    let percentCompleted = Math.round(
      (progressEvent.loaded * 100) / progressEvent.total
    );
    console.log(`Upload progress: ${percentCompleted}%`);
  }
});
```

## File Download

```nx
import http from '@opennexora/http';

// Download file
let response = await http.get('/api/download/file.pdf', {
  responseType: 'blob'
});

// Create download link
let url = window.URL.createObjectURL(new Blob([response.data]));
let link = document.createElement('a');
link.href = url;
link.setAttribute('download', 'file.pdf');
document.body.appendChild(link);
link.click();
link.remove();
```

## Browser Support

nexora-http works in all modern browsers:

- Chrome 60+
- Firefox 55+
- Safari 11+
- Edge 15+

## Node.js Support

nexora-http also works in Node.js:

```nx
import http from '@opennexora/http';

// Node.js specific options
let response = await http.get('https://api.example.com/users', {
  // Use Node.js http agent
  httpAgent: new http.Agent({ keepAlive: true }),
  httpsAgent: new https.Agent({ keepAlive: true })
});
```

## TypeScript Integration

Full type definitions are included:

```nx
import http, { AxiosResponse, AxiosError } from '@opennexora/http';

interface User {
  id: Number;
  name: String;
  email: String;
}

// Typed response
let response: AxiosResponse<Array<User>> = await http.get('/api/users');
console.log(response.data[0].name);

// Typed error handling
try {
  let response = await http.get<User>('/api/users/1');
} catch (error) {
  let axiosError = error as AxiosError;
  console.error(axiosError.response?.status);
}
```

## Performance Tips

1. **Use cancel tokens** to avoid unnecessary requests
2. **Enable keep-alive** for multiple requests to same host
3. **Use response streaming** for large responses
4. **Implement request caching** for repeated requests

```nx
import http from '@opennexora/http';

// Enable keep-alive
let api = http.create({
  baseURL: 'https://api.example.com',
  httpAgent: new http.Agent({ keepAlive: true })
});

// Implement simple cache
let cache = new Map();

async function cachedGet(url) {
  if (cache.has(url)) {
    return cache.get(url);
  }
  
  let response = await api.get(url);
  cache.set(url, response.data);
  return response.data;
}
```

## Contributing

We welcome contributions! Please see [CONTRIBUTING.md](../../CONTRIBUTING.md) for guidelines.

## License

MIT © OpenNexora Foundation