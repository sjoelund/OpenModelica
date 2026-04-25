# Socket Module Assumptions and Notes

## Assumptions

1. **C Library Availability**: All functions depend on the `omcruntime` C library being linked. The extern declarations assume the following C functions exist:
   - `int Socket_waitforconnect(int)` - waits for a connection, returns a file descriptor
   - `char* Socket_handlerequest(int)` - handles a request, returns a heap-allocated C string
   - `void Socket_sendreply(int, const char*)` - sends a reply string over a connection
   - `void Socket_close(int)` - closes a connection descriptor
   - `void Socket_cleanup()` - cleans up all socket resources

2. **String Memory Management**: `Socket_handlerequest` returns a heap-allocated C string. The current implementation creates a Rust `String` from it, but there is no matching free function exposed. This is a potential memory leak.

3. **Blocking Behavior**: `Socket_waitforconnect` is assumed to be a blocking call that waits for an incoming connection on the given socket/file descriptor.

4. **Unix-only**: The original Moduleica source states this is "Not implemented in Win32 builds." These wrappers are expected to work only on Unix-like systems.

5. **Integer Type**: MetaModelica `Integer` maps to the C `int` type, which is `i32` on all platforms we target.

6. **Socket Descriptor Convention**: The `inInteger` parameter in all functions is assumed to be a POSIX socket file descriptor.

## Things That Might Not Work As Expected

1. **Memory Leak**: The string returned by `Socket_handlerequest` is never freed. This could be fixed if the C library exposes a `Socket_freeString` or similar function.

2. **No Error Handling**: The current translation does not add any error handling for null pointer returns from `Socket_handlerequest` beyond returning an empty string. Other C functions may also fail silently.

3. **No Async Support**: All socket operations are blocking. If non-blocking I/O is needed, the C library would need to expose non-blocking variants.

4. **No Connection Lifecycle Management**: The module exposes low-level primitives but does not provide a higher-level connection lifecycle (accept, read, write, close) as a single cohesive API.

5. **Thread Safety**: It is unclear whether the C functions are thread-safe. If multiple threads need to use sockets concurrently, additional synchronization may be needed.
