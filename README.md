# AssemblerDisassembler
An assembler and disassembler for: x86, x64, ARM32, ARM64, MIPS32, MIPS64, PPC32, PPC64, AVR32, RISCV32, RISCV64, Sparc32, Sparc64

QVLx Labs
Authors: Matzr3lla, m0nZSt3r, $t@$h

asmb-f writes output to files while asmb-s prints to std out

You can always use this as it was designed to be used in Salvum:
https://www.qvlx.com/downloads

To run this properly, you'll need to download these:
  gcc-9-powerpc-linux-gnu
  gcc-10-powerpc64-linux-gnu
  gcc-10-mips-linux-gnu
  gcc-10-mips64-linux-gnuabi64
  gcc-arm-none-eabi
  gcc-9-aarch64-linux-gnu
  gcc-10-riscv64-linux-gnu
  gcc-10-sparc64-linux-gnu
  gcc-avr

And make sure you don't have this installed because it will conflict:
  gcc-multilib

Then it will work like so:

![image](https://github.com/STashakkori/AssemblerDisassembler/assets/4257899/4123a8aa-d02c-44e6-a663-91a70bc65b12)
