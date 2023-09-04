mov r1, 3
mov r2, 3
mov r3, 3
mul r1, r2, r1
sub r2, r2, 1

.pow_loop
mul r3, r3, r3
sub r2, r2, 1
cmp r2, 1
jle .pow_loop

mul r3, r3, r1
hlt
