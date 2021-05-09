use std::env;
use std::process;
use std::convert::TryInto;
use std::error::Error;
use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::vec::Vec;
use std::any::type_name;

// Memory size is 1MiB.
const MEMORY_SIZE: usize = 1024 * 1024;

const EAX: usize = 0;
const ECX: usize = 1;
const EDX: usize = 2;
const EBX: usize = 3;
const ESP: usize = 4;
const EBP: usize = 5;
const ESI: usize = 6;
const EDI: usize = 7;
const REGISTERS_COUNT: usize = 8;
const REGISTERS_NAME: [&str; 8] = ["EAX", "ECX", "EDX", "EBX", "ESP", "EBP", "ESI", "EDI"];

type InstFunc = fn(&mut Emulator);

type Insts = [InstFunc; 256];

fn type_of<T>(_: T) -> String{
  let a = std::any::type_name::<T>();
  return a.to_string();
}

struct Emulator {
    regs: [u32; REGISTERS_COUNT],
    eflags: u32,
    mem: Vec<u8>,
    eip: usize,
}

fn create_emu(eip: usize, esp: u32) -> Emulator {
    let memory = Vec::new();
    let mut registers = [0; REGISTERS_COUNT]; 
    registers[ESP] = esp;
    //println!("here!");
    //println!("{:?}", type_of(registers));
    //println!("{:?}", registers);
    return Emulator {
        regs: registers,
        eflags: 0,
        mem: memory,
        eip: eip,
    };
} 

fn nop(_emu: &mut Emulator) {
}

// MOV r32, imm32: Move imm32 to r32.
fn mov_r32_imm32(emu: &mut Emulator) {
    let reg: usize = (get_code8(emu, 0) - 0xB8).try_into().unwrap();
    let value = get_code32(emu, 1);
    emu.regs[reg] = value;
    emu.eip += 5;
}

fn init_instructions(instructions: &mut Insts) {
	for i in 0..8 {
        instructions[0xB8 + i] = mov_r32_imm32;
	}
    instructions[0xEB] = short_jump;
}


fn main() {
    let args: Vec<String> = env::args().collect();
    let mut emu = create_emu(0x0000, 0x7c00);

    if args.len() != 2 {
        println!("usage: x86emu filename");
        process::exit(1);
    }

    let path = Path::new(&args[1]);
	let display = path.display();

	let mut file = match File::open(&path) {
        Err(why) => {
            panic!("couldn't open {}: {}", display, why.description())
        },
        Ok(file) => {
            file
        },
    };
    let file_len = file.metadata().unwrap().len();
	let mut binary = Vec::<u8>::new();

    match file.read_to_end(&mut binary) {
        Err(why) => panic!("couldn't read {}: {}", display,
                                                   why.description()),
        Ok(_) => println!("read file from {}\n", display),
    }
    emu.mem = binary;
    let mut instructions: Insts = [nop; 256];
    println!("{:?}",instructions.len());
    println!("{:?}",instructions[0](&mut emu));
    println!("{:p}", instructions[0] as *const());
    init_instructions(emu);


    println!("finish!! main()");
}

