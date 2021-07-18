use std::env;
use std::process;
use std::convert::TryInto;
use std::error::Error;
use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::vec::Vec;
use std::any::type_name;

pub mod instruction;
pub mod function;
pub mod modrm;
use instruction::*;
use function::*;
use modrm::*;


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

pub struct Emulator {
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

fn dump_registers(emu: &mut Emulator) {
    for i in 0..REGISTERS_COUNT {
        println!("{0} = {1:x}", REGISTERS_NAME[i], emu.regs[i])
        //println!("{0} = {1:x}", REGISTERS_NAME[i], emu.regs[i])
    }
    println!("EIP = {}", emu.eip)
}

fn read_binary(emu: &mut Emulator, filename: &String) -> u64 {
    let path = Path::new(&filename);
	let display = path.display();

	let mut file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", display,
                                                   why.description()),
        Ok(file) => file,
    };
    let file_len = file.metadata().unwrap().len();

	let mut binary = Vec::<u8>::new();
    match file.read_to_end(&mut binary) {
        Err(why) => panic!("couldn't read {}: {}", display,
                                                   why.description()),
        Ok(_) => println!("read file from {}\n", display),
    }

    emu.mem = vec![0; 0x7c00];
    emu.mem.extend(binary);

    return file_len;
}

fn main() {
    let args: Vec<String> = env::args().collect();


    if args.len() != 2 {
        println!("usage: x86emu filename");
        process::exit(1);
    }

    let mut emu = create_emu(0x7c00, 0x7c00);

    let len = read_binary(&mut emu, &args[1]);


    let mut instructions: Insts = [nop; 256];
    println!("{:?}",instructions.len());
    println!("{:?}",instructions[0](&mut emu));
    println!("{:p}", instructions[0] as *const());
    println!("{:p}", instructions[1] as *const());
    println!("{:p}", instructions[0xE8] as *const());
    init_instructions(&mut instructions);
    //↓ここにshort jumpがきていることを確認できる.
    println!("{:p}", instructions[0xEB] as *const());

    while emu.eip < MEMORY_SIZE {
        let code = get_code8(&mut emu, 0) as usize;
        println!("EIP = {}, Code = {}", emu.eip, code);

        if instructions[code] as usize == nop as usize {
            println!("Not implemented: {0}", code);
            break;
        }

        // Execute an instruction.
        instructions[code](&mut emu);

        // TODO: when does a program finish?
        if emu.eip >= len as usize {
            println!("\nEnd of program.\n");
            break;
        }
    }
    dump_registers(&mut emu);


    println!("finish!! main()");
}

