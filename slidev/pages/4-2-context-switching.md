---
title: Context switching
---

# Context switching

```mermaid { scale: 0.7 }
sequenceDiagram
  activate Receiver
  Receiver -->> Linux: recv(&mut buf)
  deactivate Receiver
  Linux -->> CPU0: Save registers and wipe some caches
  loop wait for
    Note over Linux: packet
  end
  Linux --> Linux: Write packet into buf
  Linux --> Linux: Scheduler: run Receiver
  Linux -->> CPU1: Restore registers and stack
  Linux -->> Receiver: return from recv
  activate Receiver
  Receiver -->> CPU1: deser
  deactivate Receiver
```

---
title: Context switching impact
---

# CS impact

- ~8-14µs - In our case

- ~8µs - [Linux perf event Features and Overhead | Vincent Weaver | University of Maine](https://web.eece.maine.edu/~vweaver/projects/perf_events/overhead/fastpath2013_perfevents.pdf#page=4)

- ~3µs - [How long does it take to make a context switch? | Tsuna's blog](https://blog.tsunanet.net/2010/11/how-long-does-it-take-to-make-context.html)

---
title: Pin to CPU
---

# Let's pin our receiver to CPU Core

```rust {all|2}
let channel: std::net::UdpSocket;
core_affinity::set_for_current(core_id);
loop {
    match channel.recv(&mut buf) /* .await */ {
        Ok(len) => handle_message(&buf[..len]),
        Err(err) => handle_error(err),
    }
}
```

[core_affinity](https://docs.rs/core_affinity/latest/core_affinity/)

> Use isolated cores via `isolcpus=1-7` kernel setting

---
title: Measure
---

# Measure and compare

![Blocking loop pinned to core performance results](/static/1_blocking_affinity.png)

