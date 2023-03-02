---
title: Busy loop
---

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
-->

---
title: Busy loop
---

## Measurement results

<p>

|        | previous       |  **time**      |
|--------|----------------|----------------|
| mean   |   67.17>17.46  |   **14.74µs**  |
| std    |   36.56>7.53   |    **4.87µs**  |
| min    |    8.52>3.70   |    **3.28**    |
| max    |  974.84>86.88  |   **47.77µs**  |

</p>
