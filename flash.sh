#!/bin/bash
#SRC=target/thumbv7em-none-eabihf/debug/oui
SRC=oui.bin
openocd -c 'source [find interface/jlink.cfg]' -c 'transport select swd' -c 'source [find target/nrf52.cfg]' -c 'tcl_port 0' -c 'telnet_port 0' -c 'gdb_port 3333' -c 'init' -c 'targets' -c 'reset halt' -c "flash write_image erase "pip+root.bin" 0" -c "verify_image "pip+root.bin" 0" -c 'reset halt'