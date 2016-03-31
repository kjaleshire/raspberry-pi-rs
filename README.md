##### A collection of Raspberry pi 2 bare metal programs. In Rust.

These examples are for Raspberry pi 2 (I don't have a pi or pi zero). It would be trivial to recompile these examples for pi, just change the first byte in GPIO registers from `0x3F` to `0x20` and the target architecture from `armv7` to `arm`.

Major credit to David Welch <dwelch@dwelch.com> for providing bare-metal examples in C at [http://github.com/dwelch67/raspberrypi](github.com/dwelch67/raspberrypi)
