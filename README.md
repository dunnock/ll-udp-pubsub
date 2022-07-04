# 1. Вимоги та особливості low latency

min(time) != max(throughput)  =>  low latency != high load

## Перешкоди:

- Системний планувальник (wait, park/unpark)
- Забезпечення гарантій (tcp, транзакції ..)
- Загальні бібліотеки

## Що допомагає:

- Busy loop
- CPU cache
- Протоколи з мінімальними гарантіями
- Hacks

# 2. Задача

Канал зв'язку між розподіленими процесами для швидкої передачі невеликих повідомлень в AWS.

## Вибір технологій

- Мінімально зручний протокол без гарантій - UDP
** Не використовуємо multicast через специфіку multicast в облаці
- Стандартна бібліотека задовільняє

# 3. Код

## 3.1. Busy loop
## 3.2. Non blocking UDP receive
## 3.3. UDP send (blocking vs non blocking)
## 3.4. Registry of clients via sync HashMap (ssc)

# 4. Run / measure
