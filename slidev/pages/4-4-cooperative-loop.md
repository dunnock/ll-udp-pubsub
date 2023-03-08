---
title: Cooperative event-loop
---

# Use cooperative scheduling

```rust {all|2-4|6-7|10}
let sock: std::net::UdpSocket;
sock.set_nonblocking(true);
loop {
    match sock.recv(&mut buf) {
        Ok(len) => handle_message(&buf[..len]),
        Err(err) if err.kind() == ErrorKind::WouldBlock || 
            err.kind() == ErrorKind::TimedOut => { }
        Err(err) => handle_error(err),
    }
    std::thread::yield_now();
}
```

<!--
Давайте перепишемо наш цикл із використанням кооперативного шедулінгу спочатку.
Бо цей підхід біль прийнятливий для ОС та його радять в лінукс комььюніті якщо вже використовується навантажений цикл.
-->

---
title: Cooperative event-loop
---

## Measurement results

<p>

|        | previous   |  **time**      |
|--------|------------|----------------|
| mean   |   67.17µs  |   **17.46µs**  |
| std    |   36.56µs  |    **7.53µs**  |
| min    |    8.52µs  |    **3.70µs**  |
| max    |  974.84µs  |   **86.88µs**  |


</p>
