## RumOS

A hobby 64 bit operating system written in Rust.

It currently boots the qemu to 64-bit long mode and vga console also works. That's all:

![][rumos_boot]

The plan is to make virtual memory and kernel thread work and boot a NetBSD rump kernel in it. Then fun will begin from there since various components can be written and replaced at will while the kernel remains fully functional.

### Prerequisite

+ A rust compiler, of course. 0.9-pre should do.

+ A cross compiler tool-chain for your platform. The makefile currently expects ``x86_64-pc-netbsd-{gcc,ld,objcopy}``. There's a script ``misc/build-holy-triad.sh`` to help build them for you.

[rumos_boot]: http://i.imgur.com/AM2MWDP.jpg
