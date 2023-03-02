---
# try also 'default' to start simple
theme: seriph
# random image from a curated Unsplash collection by Anthony
# like them? see https://unsplash.com/collections/94734566/slidev
background: https://source.unsplash.com/collection/94734566/1920x1080
# apply any windi css classes to the current slide
class: 'text-center'
# https://sli.dev/custom/highlighters.html
highlighter: shiki
# show line numbers in code blocks
lineNumbers: false
# some information about the slides, markdown enabled
info: |
  ## Побудова мережевого каналу з мінімальними затримками

  Побудуємо відкритий канал передачі швидких невеличких повідомлень
  за допомогою стандартної бібліотеки Rust.

  Такий канал може використовуватись в застосунках де важлива
  реакція на події, які відбуваються в локальному мережевому середовищі,
  в межах до 50 мікросекунд.
# persist drawings in exports and build
drawings:
  persist: false
# page transition
transition: slide-left
# use UnoCSS
css: unocss
---

# Побудова мережевого каналу з мінімальними затримками

<!--
The last comment block of each slide will be treated as slide notes. It will be visible and editable in Presenter Mode along with the slide. [Read more in the docs](https://sli.dev/guide/syntax.html#notes)
-->


---
src: ./pages/1-motivation.md
---
---
src: ./pages/2-low-latency.md
---
---
src: ./pages/3-why-udp.md
---
---
src: ./pages/4-1-event-loop.md
---
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
