---
title: Agenda
layout: center
---

- Low latency domain
- Assumptions and considerations
- Implementation and measurement
- Using ll-udp-pubsub library

---
title: Why do we use Rust
layout: fact
---

Rust is ideal choice for low latency, it handles technical risks without sacrificing performance and provides access to native low level control over execution.

---
title: Low latency application specifics
layout: fact
---

Low latency refers to the short amount of time it takes for a signal or data to travel from one point to another in a system

`Low latency` ≠ ` High load `

<table>
<tr><td align="left" width="60%">
↓time
</td>
<td align="right">
↑throughput
</td>
</tr>
</table>

---
title: Event value
layout: fact
---

When low latency matters?

![Event value decline with time](static/event-time-value.png)

<!-- швидко  -->

---
title: Applications
layout: center
---

# Applications

- HFT
- RTB
- IoT
- Other realtime applications


<!-- не коментувати чьому -->

