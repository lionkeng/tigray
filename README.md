# tigray-rust

A simple micro-service based on the awesome axum web framework
## Error Handling

With our use of axum (the web framework), tigray service is *infallible*. We will therefore, propagate error as a valid HTTP response with an appropriate StatusCode and error messsage in the body of the HTTP response.