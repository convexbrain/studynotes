ASPATH='/c/Program Files (x86)/GNU Tools ARM Embedded/8 2019-q3-update/bin/arm-none-eabi-as'

"$ASPATH" -march=armv6s-m kernel_asm.s -o thumbv6m-none-eabi_kernel_asm.o
"$ASPATH" -march=armv7-m kernel_asm.s -o thumbv7m-none-eabi_kernel_asm.o
"$ASPATH" -march=armv7e-m kernel_asm.s -o thumbv7em-none-eabi_kernel_asm.o
"$ASPATH" -march=armv7e-m kernel_asm.s -o thumbv7em-none-eabihf_kernel_asm.o
"$ASPATH" -march=armv8-m.base kernel_asm.s -o thumbv8m.base-none-eabi_kernel_asm.o
"$ASPATH" -march=armv8-m.main kernel_asm.s -o thumbv8m.main-none-eabi_kernel_asm.o
