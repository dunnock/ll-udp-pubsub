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
title: Busy loop performance
---

## Measurement results


![Busy loop performance results](static/3_busy_loop_affinity.png)

---
title: Busy loop performance
---

## Compare measurement results

![Busy loop performance results comparison](static/3_busy_loop_vs_all.png)


---
title: Busy loop profiling
---

## Profiling summary

```sh
$ perf stat taskset -c 1-4 bin/receive -c ${LOCAL_IP}:3000 -n 100000 --non-blocking --core 1
```

```sh
         106103.94 msec task-clock                #    1.000 CPUs utilized          
               253      context-switches          #    2.384 /sec                   
                 2      cpu-migrations            #    0.019 /sec                   
               789      page-faults               #    7.436 /sec                   
```

### previous result:

```sh
            339.31 msec task-clock                #    0.003 CPUs utilized          
            100329      context-switches          #  295.683 K/sec                  
                 2      cpu-migrations            #    5.894 /sec                   
               787      page-faults               #    2.319 K/sec                  
```
