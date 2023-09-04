mov r5, 249
cal .display_num
hlt

.display_num
    mov r3, 0
.DISPLAY_NUM_loop
    mod r4, r5, 10
    add r4, r4, 48
    pst r3, r4
    div r5, r5, 10
    add r3, r3, 1
    cmp 0, r5
    jgr .DISPLAY_NUM_loop
    ret

.reverse_arr
    mov r6, 0
    ret
    