# UDP pubsub channel with minimum latency

Extremely low latency overhead on top of UDP (1-2 microseconds)

# Publisher and subscriber


```rust
impl ManagedPublisher {
    pub fn send<'r, Message: Serialize>(&mut self, msg: Message) -> Result<(), std::io::Error> { .. }
}
```

```rust
impl<MessageHandler: Handler + Send + 'static> UdpSubscriber<MessageHandler> {
    pub fn spawn(
        self,
        bind_to_core: Option<usize>,
    ) -> Result<UdpSubscriberHandle<MessageHandler>, std::io::Error> { ... }
}
```

# Usage example

## Implement publisher

```rust
let (ctl, mut publisher) = PublisherController::create(config).unwrap();
let handle = ctl.spawn().unwrap();

let timeout = Duration::from_micros(opts.timeout_micros);
for i in 0..i64::MAX {
    publisher.send(i).unwrap();
    std::thread::sleep(timeout);
}

handle.shutdown().unwrap();
```

## Implement subscriber

```rust
impl Handler for Receiver {
    type Message = i64;
    fn handle(&mut self, msg: Packet<Self::Message>, received_ts: i64) {
        self.count.fetch_add(1, Ordering::Relaxed);
        self.messages.push((msg.sent_ts, msg.data));
    }
}

let count = Arc::new(AtomicUsize::default());
let subscriber_config = UdpSubscriberConfig::new(opts.client_addr);
let receiver = Receiver { count: count.clone() };
let mut subscriber = UdpSubscriber::new(subscriber_config, receiver).unwrap();
subscriber.set_nonblocking(true).unwrap();
let controller_handle = subscriber.spawn_controller(opts.server_addr).unwrap();
let subscriber_handle = subscriber.spawn(core(1)).unwrap();

for i in 0..10 {
    println!("{}", count.load(Ordering::Relaxed));
    std::thread::sleep(Duration::from_secs(1)).unwrap();
}

controller_handle.shutdown();
subscriber_handle.shutdown();
let result = subscriber_handle.shutdown().unwrap();
dbg!(result);
```

## Other approaches

- eBPF, io_uring, linux mod, unikernel, open onload

