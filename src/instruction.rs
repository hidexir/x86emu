use crate::*;
use function::*;
use modrm::*;

// MOV r32, r/m32 (8B /r): Move r/m32 to r32.
pub fn mov_r32_rm32(emu: &mut Emulator) {
    emu.eip += 1;
    let modrm = parse_modrm(emu);
    let rm32 = get_rm32(emu, &modrm);
    set_r32(emu, &modrm, rm32);
}

// MOV r/m32, r32 (89 /r): Move r32 to r/m32.
pub fn mov_rm32_r32(emu: &mut Emulator) {
    emu.eip += 1;
    let modrm = parse_modrm(emu);
    let r32 = get_r32(emu, &modrm);
    set_rm32(emu, &modrm, r32);
}


pub fn code_83(emu: &mut Emulator) {
    emu.eip += 1;
    let modrm = parse_modrm(emu);

    match modrm.opecode {
        5 => { sub_rm32_imm8(emu, &modrm); }
        _ => {
            println!("not implemented 83 {}", modrm.opecode);
            process::exit(1);
        }
    }
}


pub fn code_ff(emu: &mut Emulator) {
    emu.eip += 1;
    let modrm = parse_modrm(emu);

    match modrm.opecode {
        0 => { inc_rm32(emu, &modrm); }
        _ => {
            println!("not implemented ff {}", modrm.opecode);
            process::exit(1);
        }
    }
}




// MOV r32, imm32: Move imm32 to r32.
pub fn mov_r32_imm32(emu: &mut Emulator) {
    let reg: usize = (get_code8(emu, 0) - 0xB8).try_into().unwrap();
    let value = get_code32(emu, 1);
    emu.regs[reg] = value;
    emu.eip += 5;
}

// MOV r/m32, imm32 (C7 /0 id): Move imm32 to r/m32.
pub fn mov_rm32_imm32(emu: &mut Emulator) {
    emu.eip += 1;
    let modrm = parse_modrm(emu);
    let value = get_code32(emu, 0);
    emu.eip += 4;
    set_rm32(emu, &modrm, value);
}


// JMP rel8: Jump short, relative, displacement relative to next instruction.
pub fn short_jump(emu: &mut Emulator) {
    let diff = get_sign_code8(emu, 1) as usize;
    emu.eip += diff + 2;
}

// JMP rel32: Jump near, relative, RIP = RIP + 32-bit displacement sign extended to 64-bits.
pub fn near_jump(emu: &mut Emulator) {
    let diff = get_sign_code32(emu, 1) as usize;
    emu.eip += diff + 5;
}

pub fn nop(_emu: &mut Emulator) {
}

pub fn init_instructions(instructions: &mut Insts) {
    for i in 0..8 {
        instructions[0x50 + i] = push_r32;
    }
    for i in 0..8 {
        instructions[0x58 + i] = pop_r32;
    }

    instructions[0x68] = push_imm32;
    instructions[0x6A] = push_imm8;


    instructions[0x01] = add_rm32_r32;
    instructions[0x83] = code_83;
    instructions[0x89] = mov_rm32_r32;
    instructions[0x8B] = mov_r32_rm32;
	for i in 0..8 {
        instructions[0xB8 + i] = mov_r32_imm32;
	}

    instructions[0xC7] = mov_rm32_imm32;
    instructions[0xE9] = near_jump;
    instructions[0xEB] = short_jump;
    instructions[0xFF] = code_ff;

    instructions[0xC3] = ret;
    instructions[0xE8] = call_rel32;

}

// ADD r/m32, r32 (01 /r): Add r32 to r/m32.
pub fn add_rm32_r32(emu: &mut Emulator) {
    emu.eip += 1;
    let modrm = parse_modrm(emu);
    let r32 = get_r32(emu, &modrm);
    let rm32 = get_rm32(emu, &modrm);
    set_rm32(emu, &modrm, rm32 + r32);
}

// SUB r/m32, imm8 (83 /5 ib): Subtract sign-extended imm8 from r/m32.
pub fn sub_rm32_imm8(emu: &mut Emulator, modrm: &ModRM) {
    let rm32 = get_rm32(emu, &modrm);
    let imm8 = get_sign_code8(emu, 0);
    emu.eip += 1;
    set_rm32(emu, &modrm, (rm32 as i32 - imm8) as u32);
}

// INC r/m32 (FF /0): Increment r/m doubleword by 1.
pub fn inc_rm32(emu: &mut Emulator, modrm: &ModRM) {
    let value = get_rm32(emu, &modrm);
    set_rm32(emu, &modrm, value + 1);
}

pub fn push_r32(emu: &mut Emulator) {
    let reg = (get_code8(emu, 0) - 0x50) as usize;
    let value = get_register32(emu, reg);
    push32(emu, value);
    emu.eip += 1;
}

pub fn push_imm32(emu: &mut Emulator) {
    let value = get_code32(emu, 1);
    push32(emu, value);
    emu.eip += 5;
}

pub fn push_imm8(emu: &mut Emulator) {
    println!("push_imm8");
    let value = get_code8(emu, 1);
    push32(emu, value.into());
    emu.eip += 2;
}

pub fn pop_r32(emu: &mut Emulator) {
    println!("call pop_r32");
    let reg = (get_code8(emu, 0) - 0x58) as usize;
    let value = pop32(emu);
    set_register32(emu, reg, value);
    emu.eip += 1;
}

pub fn call_rel32(emu: &mut Emulator) {
    let diff = get_sign_code32(emu, 1);
    push32(emu, (emu.eip + 5).try_into().unwrap());
    emu.eip = emu.eip.wrapping_add((diff + 5) as usize);
}

pub fn ret(emu: &mut Emulator) {
    emu.eip = pop32(emu).try_into().unwrap();
}

