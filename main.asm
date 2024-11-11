add %r3, %r2 ; comment
and %r7, %r2 ; comment
add %r4, #-127
or %r3, %r5
ld %r2, $500
st $500, %r2
jmp $300
jz $399
shl %r3, #2
shr %r6, #4
int #2
mov %r4, #40
hlt
ret
call @function
cmp %r3, %r2
