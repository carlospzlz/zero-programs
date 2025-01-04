# Zero programs

A set of small programs targetting the Raspberry Pi Zero (v1 and v2) to learn
about various topics like cross-compilation, robotics, motor drivers and rust.

## Hello GPIO

This initial example shows how to interact with the GPIO interface using rust.
It can be cross-compiled for RPI Zero v2 with the command below:

```
~/.cargo/bin/cross build --target aarch64-unknown-linux-gnu --release
```

The RPI Zero v2 has a quad-core 64-bit ARM Cortex-A53, which implements a
`aarch64` architecture.

Remember that each of the sub-strings in the toolchain name refer to:
 - **arch64**: architecture
 - **unknown**: vendor, *unknown* is typically used for open-source toolchains
 - **linux**: The OS the platform will be running
 - **gnu**: ABI - Application Binary Interface (GNU C Library - `glibc` in this case)

You can simply deploy your cross-compiled binary using `scp`:

```
scp target/aarch64-unknown-linux-gnu/release/hello-gpio zero-v2:~/programs
```

At this point, you are all set up to run the program:

```
./hello-gpio
```

An GPIO hat with LEDs indicators was added to easily observe/debug the outcome
of the program.

<p align="middle">
    <img src="resources/hello_gpio_01.jpg" width="353" />&nbsp;&nbsp;
    <img src="resources/hello_gpio_02.jpg" width="450" />
</p>

## Interactive Bot

The hardware for this example adds a couple of wheeled motors and a motor driver.

The cross-compilation and deploy workflow is the same as exposed in the previous
example.

It implements a basic shell that allows to send commands to robot. You can
ssh to your RPI Zero v2 and launch shell, then request commands.

```
./interactive-bot
>>> left f
>>> left b
>>> forward
>>> backward
>>> spin l
>>> spin r
...
```

## Event controller

This is similar to the previous one, but it uses an event polling mechanism
so the robot can be controlled with the keyboard.

