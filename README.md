# The Myrtle Programming Language

![Logo](./assets/img/logo.png)

*State-machine based reactive programming for embedded systems.*

<a href="https://www.buymeacoffee.com/aossola" target="_blank"><img src="https://cdn.buymeacoffee.com/buttons/default-orange.png" alt="Buy Me A Coffee" height="41" width="174"></a>

## Supported Platforms

- [X] RP2040 (Raspberry Pi Pico)
- [ ] ESP32
- [ ] STM32F407 (Black Pill)
- [ ] STM32F103 (Blue Pill)
- [ ] Renesas RA4M1 (Arduino Uno R4)

## Examples

### Blink

```
device {
    pin = push_pull(pin=5);
}

machine main {
    state entry {
        timer(ms=1000) >> emit(values=[0, 1]) >> setvar(var="pin");
    }
}
```

### States

```
device {
    pin = push_pull(pin=5);
}

machine main {
    state entry {
        once() >> setstate(state="on");
    }

    state on {
        once() >> literal(value=1) >> setvar(var="pin");
        delay(ms=1000) >> setstate(state="off");
    }

    state off {
        once() >> literal(value=0) >> setvar(var="pin");
        delay(ms=1000) >> setstate(state="on");
    }
}
```
