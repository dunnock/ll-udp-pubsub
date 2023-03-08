---
title: Assumptions
layout: center
---

# Assumptions

- Sequence is less important than time
- Allowance for losing messages

<!--
Якщо прибрати гарантії доставки та/чи послідовної доставки швидкість
доставки може збільшитись у рази.
-->

---
title: Building low latency system
layout: center
---

Nothing comes for free 
> except zero cost abstractions

---
title: Building low latency system
layout: center
---

# Implementation strategy

- Use protocol with minimum required guarantees
- Obtain full control over the execution
- Work around system scheduler

