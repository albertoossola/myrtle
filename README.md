# The Myrtle Programming Language

![Logo](./assets/img/logo.png)

*State-machine based reactive programming for embedded systems.*

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