# Peripheral Map

## GPIO Configuration

### **USART2**
 - USART 9600 8B 1bit Parity

| Peri      | GPIO | PinName | PIN# | Mode    |
| --------- | ---- | ------- | ---- | ------- |
| USART2_RX | PA2  | RX      | 9    | Pull-Up |
| USART2_TX | PA3  | TX      | 10   | Pull-Up |

### **EXTI** - Input Mode
| Peri  | GPIO | PinName     | PIN# | Mode    | EXTI   |
| ----- | ---- | ----------- | ---- | ------- | ------ |
| EXTI6 | PA7  | P_OUT_PULSE | 14   | Float   | EXTI7  |
| EXTI5 | PA8  | P_EMPTY     | 15   | Float   | EXTI8  |
| EXTI4 | PA11 | P_ERROR     | 16   | Float   | EXTI11 |

### **SWD** - Single Wire Debug
| Peri      | GPIO  | PinName | PIN# | Mode      |
| --------- | ----- | ------- | ---- | --------- |
| SYS_SWDIO | PA14  | SWDIO   | 18   | Dont Care |
| SYS_SWCLK | PA13  | SWCLK   | 19   | Dont Care |

### **GPIO OUTPUT**
| Peri | GPIO | PinName   | PIN# | Mode      |
| ---- | ---- | --------- | ---- | --------  |
| GPIO | PB8  | LED0      | 1    | Push-Pull |
| GPIO | PB9  | LED1      | 2    | Push-Pull |
| GPIO | PA4  | P_PULSE   | 11   | Push-Pull |
| GPIO | PA5  | P_RESET   | 12   | Push-Pull |
| GPIO | PA6  | P_INHIBIT | 13   | Push-Pull |

## Timer

### **TIM16**
 - Heratbeat - LED0 1Hz 