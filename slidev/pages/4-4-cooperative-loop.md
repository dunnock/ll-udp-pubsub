---
title: Cooperative event-loop
---

```rust {all|}
let sock: std::net::UdpSocket;
sock.set_nonblocking(true);
loop {
    match channel.recv(&mut buf) {
        Ok(len) => handle_message(&buf[..len]),
        Err(err) if err.kind() == ErrorKind::WouldBlock || 
            err.kind() == ErrorKind::TimedOut => { }
        Err(err) => handle_error(err),
    }
    std::thread::yield_now();
}
```

[sched_yield](https://man7.org/linux/man-pages/man2/sched_yield.2.html)

<!--
Спробуємо варіант кооперативного шедулінга, оскільки його рекомендують розробники лінукса

1 - зробимо нашу операцію не блокуючою

2 - Треба обробити випадок коли повертається помилка що немає повідомлення

3 - Передаємо контроль операційній системі на короткий проміжок, 
але ми лишимось на цьому ж процесорі та якщо у системи немає приорітетніших задач
то контроль одразу поверне
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
