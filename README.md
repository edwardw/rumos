## RumOS

A hobby 64 bit operating system written in Rust.

It currently boots the qemu to 64 bit long mode and loop forever in a rust function. The plan is to make virtual memory and kernel thread work and boot the NetBSD rump kernel in it. Then fun will begin from there as various components can be written and replace at will while the kernel remains functional.

### Prerequisite

+ A rust compiler, of course. 0.9-pre should do.

+ A cross compiler tool-chain for your platform. The makefile currently expects ``x86_64-pc-netbsd-{gcc,ld,objcopy}``. There's a script ``misc/build-holy-triad.sh`` to help build them for you.
