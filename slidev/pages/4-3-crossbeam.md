---
title: corssbeam recv implementation
---

[crossbeam_channel::array](https://github.com/crossbeam-rs/crossbeam/blob/master/crossbeam-channel/src/flavors/array.rs#L396)

```rust {all|3-6|7-10|13,14}
pub(crate) fn recv(&self, deadline: Option<Instant>) -> Result<T, RecvTimeoutError> {
    ...
    loop {
        if self.start_recv(token) {
            return unsafe { self.read(token) };
        }
        if backoff.is_completed() {
            break;
        } else {
            backoff.snooze();
        }
    }
    ...
    // Block the current thread.
    let sel = cx.wait_until(deadline);
    ...
}
```

---
title: corssbeam recv implementation
---

[crossbeam_utils::backoff](https://github.com/crossbeam-rs/crossbeam/blob/23b10f2b737b6b6b66f5ca224cab2568350940b0/crossbeam-utils/src/backoff.rs#L209)

```rust {all|2-5|6-7|8-10}
pub fn snooze(&self) {
    if self.step.get() <= 6 {
        for _ in 0..1 << self.step.get() {
            ::std::hint::spin_loop();
        }
    } else {
        ::std::thread::yield_now();
    }
    if self.step.get() <= 10 {
        self.step.set(self.step.get() + 1); // => backoff.is_completed()
    }
}
```

[sched_yield](https://man7.org/linux/man-pages/man2/sched_yield.2.html)
