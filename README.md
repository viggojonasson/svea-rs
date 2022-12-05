# [![Svea](assets/branding/logo-wide.svg)](https://svea.rs)

## Features

- Blazingly slow
- Unoptimized

## Examples:

---

### **Hello World Example**

Simple hello world example.

```rs
Server::new()
    .address("localhost".to_string())
    .port(3000)
    .router(
        Router::new().route(
            Route::new()
                .path("/")
                .handler(|_, _| async move { ("Hello, World!", Status::ImATeapot) }),
        ),
    )
    .run()
    .await;
```

---

### **Filter Example**

Filters allows a handler to only be executed if some conditions meet. In this example we execute the handler if the request has at least two queries, with them being `pi` and `random-number`. Pi needs to be 3.14 and random-number can be any parsable number.

This will map to: `localhost:3000?pi=3.14&random-number=<any random number>`

```rs
Server::new()
    .address("localhost".to_string())
    .port(3000)
    .router(
        Router::new().route(
            Route::new()
                .filter(
                    Filter::new("/")
                        .query("pi", QueryFilter::NumberExact(3.14))
                        .query("random-number", QueryFilter::Number),
                )
                .handler(|_, _| async move { ("Pi is 3.14!", Status::Ok) }),
        ),
    )
    .run()
    .await;
```

## More comprehensive examples:

### **`file-serving`**

Serves files in a static folder.

To run: `cargo run --bin file-serving`

### **`example-app`**

Simple application with endpoint testing and "states".

To run: `cargo run --bin example-app`

### **`readme-example`**

The first example in the readme.

To run: `cargo run --bin readme-example`
