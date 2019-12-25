## Relay

A relay server written in Rust for the following scenario:

```
+-----+ Telemetry(UDP) +-------+ Telemetry(UDP) +-----+
|     +--------------->+       +--------------->|     |
|  A  |                | Relay |                |  B  |
|     +<---------------+       +<---------------+     |
+-----+  Commands(TCP) +-------+ Commands(HTTP) +-----+
```

Relay relays data between servers A and B. A can be anything, but is probably running on a microcontroller. B can also be anything, but is probably a website.

Relay takes in telemetry data from A and relays it to B via UDP. It may be adjusted to send that data elsewhere or limit the rate of data transfer.

Relay takes in a HTTP post from B to the root address and relays the body's bytes via TCP to A.
