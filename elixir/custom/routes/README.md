# Routes

Routes is an Elixir library that automatically generates JavaScript and TypeScript route helpers directly from your Phoenix router. It provides type-safe route handling in your frontend code, ensuring that your client-side routing stays in sync with your Phoenix routes.

By integrating Routes into your application, you can effortlessly access your Phoenix routes within your JavaScript or TypeScript codebase, allowing for seamless and type-safe navigation and API calls.

## Table of Contents

- [Features](#features)
- [Installation](#installation)
- [Configuration](#configuration)
- [Usage](#usage)
  - [JavaScript and TypeScript](#javascript-and-typescript)
  - [Inertia.js Integration](#inertiajs-integration)
  - [Type Safety](#type-safety)
- [API Reference](#api-reference)
- [Development](#development)
- [Contributing](#contributing)
- [License](#license)

## Features

- 🔄 **Automatic route generation**: Generates route helpers directly from your Phoenix router.
- 📝 **TypeScript declarations**: Provides TypeScript definitions for type-safe routing.
- 🔍 **Live reloading**: Automatically regenerates routes during development when your router changes.
- 🎯 **Path parameters and query strings**: Supports dynamic path parameters and query string handling.
- 🚦 **HTTP method access**: Access the HTTP methods associated with your routes.
- 💪 **Type-safe parameter validation**: Ensures that required parameters are provided and correctly typed.

## Installation

Add `routes` to your list of dependencies in `mix.exs`:

```elixir
def deps do
  [
    {:routes, "~> 0.1.0"}
  ]
end
```

Then, run:

```bash
mix deps.get
```

## Configuration

1. **Add Routes to your Phoenix Router**:

   In your router file (e.g., `lib/your_app_web/router.ex`), add `use Routes`:

   ```elixir
   defmodule YourAppWeb.Router do
     use Phoenix.Router
     use Routes  # Add this line

     # Your routes...
   end
   ```

2. **Configure Routes in `config/config.exs`**:

   Specify your router module and optional settings:

   ```elixir
   config :routes,
     router: YourAppWeb.Router,
     typescript: true,         # Enable TypeScript output, defaults to false
     routes_path: "assets/js/routes"     # Optional, defaults to "assets/js"
   ```

3. **[Optional] Enable live reloading of routes**:

   To automatically regenerate routes when your router file changes during development, add the `Routes.Watcher` to your application's supervision tree in `lib/your_app/application.ex`:

   ```elixir
   def start(_type, _args) do
     children = [
       # ... other children
     ]

     # Add the Routes.Watcher in development environment
     children = if Mix.env() == :dev do
       children ++ [{Routes.Watcher, []}]
     else
       children
     end

     opts = [strategy: :one_for_one, name: YourApp.Supervisor]
     Supervisor.start_link(children, opts)
   end
   ```

## Usage

### JavaScript and TypeScript

Once configured, Routes will generate two files in your configured `routes_path`:

- `routes.js`: The JavaScript route helper implementation.
- `routes.d.ts`: TypeScript type definitions for full type safety.

You can import and use Routes in your JavaScript or TypeScript code as follows:

```typescript
// Import Routes
import Routes from './routes';

// Generate a URL with parameters
const url = Routes.path('user.show', { id: 123 });
// => "/users/123"

// Add query parameters
const searchUrl = Routes.path('user.index', {
  _query: { search: 'john', filter: 'active' }
});
// => "/users?search=john&filter=active"

// Get the HTTP method for a route
const method = Routes.method('user.create');
// => "POST"

// Check if a route exists
if (Routes.hasRoute('user.edit')) {
  // Route exists, proceed...
}
```

#### Replacing Path Parameters Directly

You can also use the `replaceParams` function to replace parameters in a given path:

```typescript
const customUrl = Routes.replaceParams('/posts/:postId/comments/:commentId', {
  postId: 42,
  commentId: 10,
  _query: { highlight: true }
});
// => "/posts/42/comments/10?highlight=true"
```

### Inertia.js Integration

If you're using [Inertia.js](https://inertiajs.com/), you can create a type-safe `Link` component that integrates seamlessly with Routes:

```tsx
// components/Link.tsx
import { Link as InertiaLink, InertiaLinkProps } from "@inertiajs/react";
import React from "react";
import type { PathParamsWithQuery, RoutePath } from "./routes";
import Routes from "./routes";

type LinkProps<T extends RoutePath> = Omit<InertiaLinkProps, "href"> & {
  to: T;
  params?: PathParamsWithQuery<T>;
  children: React.ReactNode;
};

export const Link = <T extends RoutePath>({
  to,
  params,
  children,
  ...props
}: LinkProps<T>) => {
  const href = Routes.replaceParams(to, params);

  return (
    <InertiaLink href={href} {...props}>
      {children}
    </InertiaLink>
  );
};
```

**Usage of the Inertia Link component:**

```tsx
import { Link } from "./components/Link";

// In your component
<Link
  to="/users/:id"
  params={{ id: 123, _query: { tab: "profile" } }}
>
  View Profile
</Link>
```

### Type Safety

Routes provides full TypeScript support with:

- **Route name autocompletion**: Get suggestions for route names as you type.
- **Parameter validation**: TypeScript will enforce that you provide all required parameters with correct types.
- **HTTP method types**: Methods like `Routes.method` return typed HTTP methods.
- **Query parameter support**: Supports type-checked query parameters.

**Example:**

```typescript
// Correct usage
const url = Routes.path('user.show', { id: 123 });

// TypeScript Error: Missing required 'id' parameter
const url = Routes.path('user.show');

// TypeScript Error: Route 'user.nonexistent' does not exist
const url = Routes.path('user.nonexistent');
```

## API Reference

### `Routes.path(name, params?)`

Generates a path (URL) for the given route name with optional parameters.

- **name**: The name of the route (string).
- **params**: An object containing route parameters and an optional `_query` object for query parameters.

**Example:**

```typescript
const url = Routes.path('post.show', { id: 42 });
// => "/posts/42"

const urlWithQuery = Routes.path('post.index', {
  _query: { page: 2, sort: 'desc' }
});
// => "/posts?page=2&sort=desc"
```

### `Routes.route(name, params?)`

Alias for `Routes.path`.

### `Routes.method(name)`

Returns the HTTP method associated with the given route name.

- **name**: The name of the route (string).

**Example:**

```typescript
const method = Routes.method('post.create');
// => "POST"
```

### `Routes.hasRoute(name)`

Checks if a route with the given name exists.

- **name**: The name of the route (string).

**Example:**

```typescript
if (Routes.hasRoute('post.edit')) {
  // Route exists
} else {
  // Route does not exist
}
```

### `Routes.replaceParams(path, params?)`

Replaces the path parameters in the given path string with the provided values.

- **path**: The path string containing parameters (e.g., `"/users/:id"`).
- **params**: An object containing parameter values and an optional `_query` object.

**Example:**

```typescript
const url = Routes.replaceParams('/users/:id', { id: '123' });
// => "/users/123"
```

## Development

Routes automatically watches for changes in your Phoenix router file during development and regenerates the route helpers accordingly. This ensures that your frontend code stays up-to-date with your backend routes without manual intervention.

To enable live reloading, make sure you've added the `Routes.Watcher` to your application's supervision tree as described in the [Configuration](#configuration) section.

## Contributing

We welcome contributions to Routes! To contribute:

1. **Fork the repository** on GitHub.
2. **Create a new branch** for your feature or bugfix.
   ```bash
   git checkout -b my-new-feature
   ```
3. **Commit your changes** with clear commit messages.
   ```bash
   git commit -am 'Add new feature'
   ```
4. **Push to your branch** on GitHub.
   ```bash
   git push origin my-new-feature
   ```
5. **Create a Pull Request** explaining your changes.

Please make sure to write tests for your changes and follow the existing code style.
