---
# try also 'default' to start simple
theme: apple-basic
# eloc
# apply any windi css classes to the current slide
class: 'text-center'
# https://sli.dev/custom/highlighters.html
highlighter: shiki
# show line numbers in code blocks
lineNumbers: true
# some information about the slides, markdown enabled
info: |
  ## Побудова мережевого каналу з мінімальними затримками

  Побудуємо відкритий канал передачі швидких невеличких повідомлень
  за допомогою стандартної бібліотеки Rust.

  Такий канал може використовуватись в застосунках де важлива
  реакція на події, які відбуваються в локальному мережевому середовищі,
  в межах до 50 мікросекунд.

titleTemplate: '%s'

defaults:
  layout: 'center'

# use UnoCSS
css: unocss
style: ./custom.css

layout: intro-image-right
image: /static/racing.jpeg
---

# Building low latency networking channel

__Maxim Vorobjov__

__Volition Technologies__


---
title: About me
layout: intro-image-right
image: /static/me.jpg
---

# About me

- 5+ years with Rust
- 3 years in HFT
- JavaScript, Python, C, C++ .. in background
- gh: [dunnock](https://github.com/dunnock)
- t: [maxsparr0w](https://twitter.com/maxsparr0w)

---
src: ./pages/1-motivation.md
---
---
src: ./pages/2-low-latency.md
---

---
layout: section
---
# Network protocol
---
src: ./pages/3-why-udp.md
---

---
layout: section
---
# Implementation

---
src: ./pages/4-1-event-loop.md
---
---
src: ./pages/4-2-context-switching.md
---
---
src: ./pages/4-3-crossbeam.md
---
---
src: ./pages/4-4-cooperative-loop.md
---
---
src: ./pages/4-5-busy-loop.md
---

---
title: ll-udp-pubsub
---

# [ll-udp-pubsub](https://github.com/dunnock/ll-udp-pubsub)

- Generic statically linked message type via serde
- Publisher maintains list of subscriptions
- Subscriptions expire after 60 seconds
- Subscriber actively maintains subscriptions

---
title: What every programmmer should know
---

# Want to know more?

- [What Every Programmer Should Know About Memory | Ulrich Drepper | Red Hat Inc](https://akkadia.org/drepper/cpumemory.pdf)

<QRCode href="https://akkadia.org/drepper/cpumemory.pdf"/>

- [How GPU Computing works | Stephen Jones | nVidia](https://www.nvidia.com/en-us/on-demand/session/gtcspring21-s31151/)

<QRCode href="https://www.nvidia.com/en-us/on-demand/session/gtcspring21-s31151/"/>

- Q & A
