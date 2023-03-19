---
title: Busy loop
---

# Busy loop

```rust {all|10}
let sock: std::net::UdpSocket;
sock.set_nonblocking(true);
loop {
    match channel.recv(&mut buf) {
        Ok(len) => handle_message(&buf[..len]),
        Err(err) if err.kind() == ErrorKind::WouldBlock || 
            err.kind() == ErrorKind::TimedOut => { }
        Err(err) => handle_error(err),
    }
    std::hint::spin_loop();
}
```

<!--
Спробуємо варіант кооперативного шедулінга, оскільки його рекомендують розробники лінукса


сто тисяч разів ми нічього не будемо робити
-->

---
title: Busy loop
---

## Measurement results

<p>

TODO: Measure once again

|        |  blocking  | cooperative| **busy loop** |
|--------|------------|------------|------------|
| mean   |    9.41µs  |    1.92µs  |    1.90µs  |
| std    |    1.42µs  |    0.19µs  |    0.21µs  |
| p99    |   12.36µs  |    2.47µs  |    2.53µs  |
| min    |    3.54µs  |    1.53µs  |    1.53µs🦄|
| max    |   22.63µs  |   10.62µs  |   13.13µs🐌|

> Less is better

</p>
