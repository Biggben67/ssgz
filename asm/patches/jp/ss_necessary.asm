.open "main.dol"

.org @NextFreeSpace
.global custom_main_additions
.global handle_instant_text
.global finish_instant_text
.global hijack_rng
.global use_game_rng
.global handle_hide_ui
.global finish_ui

handle_instant_text:
lis r9, INSTANT_TEXT_ACTIVE@ha
li r4, 0
lbz r9, INSTANT_TEXT_ACTIVE@l(r9)
cmpwi r9, 0
beq finish_instant_text
li r4, 1
b finish_instant_text
finish_instant_text:
b returnForInstantText

hijack_rng:
lis r3, USE_RNG@ha
lbz r3, USE_RNG@l(r3)
cmpwi r3, 0
bne use_game_rng
lis r3, HARDCODED_RNG_FLOAT@ha
lfs f1, HARDCODED_RNG_FLOAT@l(r3)
blr

use_game_rng:
b RELOCATE_RAND

handle_hide_ui:
lis r9, UI_HIDDEN@ha
li r4, 0
lbz r9, UI_HIDDEN@l(r9)
cmpwi r9, 0
beq finish_ui
li r3, 1
blr
finish_ui:
stwu r1, -0x10(r1)
b dLytMeterMain__draw + 0x4

; 0x80062f40 in JP 1.0
; 0x80062e60 in US 1.0
.org @MainInjection
bl custom_main_additions

.org 0x80053838 ; end of callback after rel initialization
b load_custom_rel

;.org 0x80064660
;lis r3, 0x16

;.org 0x80064690
;lis r3, 0x60

;.org 0x800646a0
;lis r3, 0xD0

.org 0x801160b4 ; instant text patch
b handle_instant_text
;bl is_instant_text

.org 0x802e1110
b hijack_rng
.org 0x802e1118
subi r3, r13, 0x38A0
b REST_OF_RNG_FUNC

.org 0x800d80a0 ; dLytMeterMain__draw
b handle_hide_ui

.close