set architecture arm
target extended-remote :3333
monitor halt
monitor reset
monitor arm semihosting enable