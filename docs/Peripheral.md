# GPIO Configuration

## USART

 - **USART2**

| Peri      | GPIO | PinName | PIN# | Mode    |
| --------- | ---- | ------- | ---- | ------- |
| USART2_RX | PA2  | RX      | 9    | Pull-Up |
| USART2_TX | PA3  | TX      | 10   | Pull-Up |

## EXTI
 
 - **Input Mode**

| Peri  | GPIO | PinName     | PIN# | Mode    |
| ----- | ---- | ----------- | ---- | ------- |
| EXTI4 | PA4  | P_ERROR     | 11   | Float   |
| EXTI5 | PA5  | P_EMOTY     | 12   | Float   |
| EXTI6 | PA6  | P_OUT_PULSE | 13   | Float   |

## SWD

 - **Single Wire Debug**
 
| Peri      | GPIO  | PinName | PIN# | Mode      |
| --------- | ----- | ------- | ---- | --------- |
| SYS_SWDIO | PA14  | SWDIO   | 18   | Dont Care |
| SYS_SWCLK | PA13  | SWCLK   | 19   | Dont Care |

## GPIO OUTPUT
 
 - **Output Mode**

| Peri | GPIO | PinName   | PIN# | Mode      |
| ---- | ---- | --------- | ---- | --------  |
| GPIO | PB8  | LED0      | 1    | Push-Pull |
| GPIO | PB9  | LED1      | 2    | Push-Pull |
| GPIO | PA7  | P_PULSE   | 14   | Push-Pull |
| GPIO | PA8  | P_RESET   | 14   | Push-Pull |
| GPIO | PA11 | P_INHIBIT | 15   | Push-Pull |