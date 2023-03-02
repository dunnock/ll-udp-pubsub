---
title: Low latency application specifics
---

Low latency: optimize min(time)
Usual application: max(throughput)

Low latency != high load

- Events represent remote system state update
- Message value significantly reduces with time
- Sequence is not very important
- Allowance for losing messages

<!--
Якщо прибрати гарантії доставки та/чи послідовної доставки швидкість
доставки може збільшитись у рази.
-->

---
title: Building low latency system
---

1. Використовувати протокол з мінімальними гарантіями
2. Використовувати мінімум рівней, меньше коду, більше контролю
- Стандартна бібліотека по можливості
3. Обійти системний планувальник
4. Trade-off із мінімальним часом затримки
- Загальні бібліотеки орієнтуються на пропускну способність, не на затримки
5. Різні хаки на рівні ОС та коду:
> core affinity, memory alignment, process sched priority, less branching
6. Profiling
