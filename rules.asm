#ruledef
{
    nop => 0x00
    ; ------ Output instructions ------
    out a => 0x01
    out b => 0x02

    ; ------ Load immediate instructions ------
    ld a, {value: u8} => 0x10 @ value
    ld b, {value: u8} => 0x11 @ value
    ld mx, {value: u8} => 0x12 @ value

    ; ------ Stote to and load from memory ------
    sta [{addr: u8}], a => 0x20 @ addr
    sta [{addr: u8}], b => 0x21 @ addr
    ld a, [{addr: u8}] => 0x22 @ addr
    ld b, [{addr: u8}] => 0x23 @ addr
    ld a, b => 0x24
    ld b, a => 0x25
    ;....
    ld mx, a => 0x2d
    ld mx, b => 0x2e
    ld mx, [{addr: u8}] => 0x2f @ addr

    ; ------ Load from flash ------
    ld a, [[{addr: u16}]] => 0x30 @ addr
    ld a, [[a]], [[b]] => 0x31
    ld b, [[a]], [[b]] => 0x32

    ; ------ ALU operations ------
    add a, b => 0x40
    adc a, b => 0x41

    ; ------ Jump ------
    jmp {addr: u16} => 0xf0 @ addr
    jmp a => 0xf1
    jmp a, b => 0xf2
    jc {addr: u16} => 0xf3 @ addr
    jc a => 0xf4
    jc a, b => 0xf5
    jz {addr: u16} => 0xf6 @ addr
    jz b => 0xf7
    
    ; ------ Halt ------
    hlt => 0xff
}