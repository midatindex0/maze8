#include "../rules.asm"

start:
    ld a, 0
    out a
    ld b, 1
    out b
    .loop:
        sta [0], b
        add a, b
        jc end
        out a
        ld b, a
        ld a, [0]
        jmp .loop
    
end:
    hlt
