# ESP32 C3

## Hardware Setup

```
RJ45 breakout | M5Stamp C3 pinout
-------------:+------------------
              | GND             |
      GND (7) | G4              |
       RX (6) | G5          3V3 |
       TX (5) | G6          G21 |
              | G7          G20 |
              | G8           EN |
              | G10          G9 |
              | 5V          GND |
              | GND         G18 |
              | 5V          G19 |
              | G1           5V |
              | G0  (USB-C) GND |
```

## Installation

See [DEVELOPMENT.md](DEVELOPMENT.md) for setup related instructions.

```console
$ cargo x run --release
```
