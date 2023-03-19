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


|        |  blocking  | **cooperative**|
|--------|------------|------------|
| mean   |    9.41µs  |    1.92µs  |
| std    |    1.42µs  |    0.19µs  |
| p99    |   12.36µs  |    2.47µs  |
| min    |    3.54µs  |    1.53µs🦄|
| max    |   22.63µs  |   10.62µs🐌|

> Less is better


</p>
