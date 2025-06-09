# Drees-OS

An experimental, toy OS to learn how about building operating systems.

## Getting Started

This project is split into multiple parts, with each subdirectory of the root being a full cargo root. 

### os

This is the main output project, and will automatically build the dependencies as needed. However, it is
configured for working on your host machine and booting into QEMU, so things like rust-analyzer may not 
work as expected when looking in the other directories. 

### kernel

The core kernel code. This project is configured to only build in the relevant targets, including having
rust-analyzer check these targets. When working on the kernel, you should be working from this folder.