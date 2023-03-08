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

|        | blocking loop  | cooperative loop |  **busy loop**      |
|--------|----------------|------------------|---------------------|
| mean   |   67.17µs      | 17.46µs          |   **14.74µs**       |
| std    |   36.56µs      |  7.53µs          |    **4.87µs**       |
| min    |    8.52µs      |  3.70µs          |    **3.28**         |
| max    |  974.84µs      | 86.88µs          |   **47.77µs**       |

</p>
