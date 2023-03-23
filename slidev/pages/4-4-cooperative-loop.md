---
title: Cooperative event-loop
---

# Use cooperative scheduling

```rust {all|3-5|7-8|11}
let sock: std::net::UdpSocket;
core_affinity::set_for_current(core_id);
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

![Cooperative loop performance results](static/2_cooperative_affinity.png)

---
title: Cooperative event-loop
---

## Compare with blocking

![Blocking loop performance results](static/2_cooperative_vs_blocking.png)
