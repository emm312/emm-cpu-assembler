mov r1, 69
cal .function
cal .function
hlt
.function
pst 5, r1
add r1, r1, 1
ret