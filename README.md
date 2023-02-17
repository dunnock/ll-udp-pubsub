# UDP канал з мінімальними затримками

Застосунки із реакцію не більше декільков десятків мікросекунд, наприклад HFT

## 1. Вимоги та особливості low latency

1-2 min
min(time) != max(throughput)  =>  low latency != high load

- Застосування - HFT та IoT - реакція ~10-100us
- Цінність повідомлення суттєво знижується із затримкою
- Послідовність доставки опціональна
- Є дозвіл на пропуск повідомлень

Якщо прибрати гарантії доставки та/чи послідовної доставки швидкість
доставки може збільшитись у рази.

### Як будується система:

1. Використовувати протокол з мінімальними гарантіями

- Забезпечення гарантій (tcp, транзакції ..) може займати ~100us-1ms

- Мінімально зручний протокол без гарантій - UDP
** Не використовуємо multicast через специфіку multicast в облаці

2. Використовувати мінімум рівней

- Стандартна бібліотека задовільняє

3. Обійти системний планувальник 

 - Чому (wait, park/unpark) (~20-40us)
розповісти про park/unpark!

додати діаграму:
UDP packet -> OS kernel driver -> unpark thread | 20-40us | -> stdlib -> out code

Тут можно показати шматок коду - busy loop:
https://github.com/dunnock/ll-udp-pubsub/blob/main/src/subscriber.rs#L87-L110

Діаграму порівняння часу із blocking та без blocking

4. Trade-off із мінімальним часом затримки

- Загальні бібліотеки орієнтуються на пропускну способність, не на затримки

навести приклад коду з crossbeam:
https://github.com/crossbeam-rs/crossbeam/blob/23b10f2b737b6b6b66f5ca224cab2568350940b0/crossbeam-channel/src/flavors/array.rs#L410
https://github.com/crossbeam-rs/crossbeam/blob/23b10f2b737b6b6b66f5ca224cab2568350940b0/crossbeam-utils/src/backoff.rs#L210

5. Хак із cpu_dma_latency

6. Хак із cpu_pin

### Вибір технологій

- Цікаво було б поекспериментувати із eBPF, io_uring, linux mod, unikernel, open onload

## 3. Код

### 3.1. Send via UDP (./examples/send.rs)
### 3.2. Receive via UDP (./examples/receive.rs)
### 3.3. Блокуючий та неблокуючий режим
### 3.4. Вимірювання
### 3.5. Publisher and Subscriber (./examples/publisher.rs) (./examples/subscriber.rs)


Попередні зауваження

0. Хто я, чим займаємось - це повинно бути логічним підгрунтям для доповіді
1. В яких умовах ми працюємо - чьому облако, чьому UDP
2. Зменьшити об'єм
3. Пояснити деякі терміни (park/unpark), покращіти початок - чьому в HFT такі специфічні вимоги,
може ілюстративно?
4. Порізати код, показувати те що має цінність для доповіді
5. Презентація по результату випробовувань - по випрбовуванням меньше зайвого тексту, параметрів.. 
показувати тільки те що має цінність для доповіді

