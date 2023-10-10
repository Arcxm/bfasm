use std::env::args;
use std::fs::File;
use std::io::{BufRead, BufReader, Error, ErrorKind, Result, Write};

/// A brainfuck instruction
enum Instruction {
    /// `>` : Increment data pointer
    Increment,
    /// `<` : Decrement data pointer
    Decrement,
    /// `+` : Add one to current cell
    Add,
    /// `-` : Subtract one from current cell
    Subtract,
    /// `.` : Write ascii value of current cell to stdout
    Write,
    /// `,` : Read ascii value from stdin to current cell
    Read,
    /// `[` : Beginning of loop with a `jmp_pc: i32`
    Jump(i32),
    /// `]` : End of loop with a `jmp_pc: i32`
    Return(i32),
}

/// The amount of `DWORD`s to reserve for the tape in the `.bss` segment
const DATA_SIZE: i32 = 256;

/// The program's entry point
fn main() {
    // The executable's arguments
    let args: Vec<_> = args().collect();

    if args.len() < 2 {
        // Print usage if no file was given
        println!("usage: bfasm FILE");
    } else {
        let file = File::open(&args[1]);

        // Return when it could not open the file
        if let Err(_) = file {
            eprintln!("error: could not find or open '{}'!", &args[1]);
            return;
        }

        // The parsed instructions
        let mut instructions: Vec<Instruction> = Vec::new();
        
        // The stack used to parse loops
        let mut stack: Vec<i32> = Vec::new();
        
        // The program counter
        let mut pc = 0;

        let f = BufReader::new(file.unwrap());
        for line in f.lines() {
            let l = line.unwrap();

            for c in l.chars() {
                match c {
                    '>' => instructions.push(Instruction::Increment),
                    '<' => instructions.push(Instruction::Decrement),
                    '+' => instructions.push(Instruction::Add),
                    '-' => instructions.push(Instruction::Subtract),
                    '.' => instructions.push(Instruction::Write),
                    ',' => instructions.push(Instruction::Read),
                    '[' => {
                        // The jump instruction is initialized with a jmp_pc of 0 but this will be overwritten by the corresponding Return instruction's pc later
                        instructions.push(Instruction::Jump(0));
                        stack.push(pc);
                    },
                    ']' => {
                        if let Some(stack_pc) = stack.pop() {
                            instructions.push(Instruction::Return(stack_pc));
                            instructions[stack_pc as usize] = Instruction::Jump(pc);
                        } else {
                            // Return when the opening and closing brackets do not match
                            eprintln!("error: unmatched ']'!");
                            return;
                        }
                    },
                    // Decrement program counter when the character is not an instruction (=> comment)
                    _ => pc -= 1,
                }

                // Increment program counter on each character (=> instruction)
                pc += 1;
            }
        }
    
        // Create the output filename from the input file's name
        let mut out_name = args[1].to_owned();
        out_name = out_name.replace(".bf", ".asm");

        // Try to write the assembly and log depending on its result
        let result = write_asm(&out_name, &instructions);
        if let Ok(()) = result {
            println!("info: successfully wrote to {}", &out_name);
        } else if let Err(err) = result {
            eprintln!("error: {}", err);
        }
    }
}

/// Writes the assembly corresponding to the given instructions to a file
/// 
/// # Arguments
/// 
/// * `filename` - The name of the file to create and write to
/// * `instructions` - A vec of instructions that contains the program
fn write_asm(filename: &str, instructions: &Vec<Instruction>) -> Result<()> {
    let file = File::create(filename);

    if let Ok(mut f) = file {
        // Write the "header"
        writeln!(f, "bits 64")?;
        writeln!(f, "default rel")?;
        writeln!(f)?;
        writeln!(f, "segment .data")?;
        writeln!(f, "\tdp dd 0")?;
        writeln!(f)?;
        writeln!(f, "segment .bss")?;
        writeln!(f, "\ttape resd {}", DATA_SIZE)?;
        writeln!(f)?;
        writeln!(f, "segment .text")?;
        writeln!(f, "global main")?;
        writeln!(f)?;
        writeln!(f, "extern _getch")?;
        writeln!(f, "extern putchar")?;
        writeln!(f)?;
        writeln!(f, "main:")?;
        writeln!(f, "\tpush rbp")?;
        writeln!(f, "\tmov rbp, rsp")?;
        writeln!(f, "\tsub rsp, 32")?;
        writeln!(f)?;

        // Append the instructions
        let mut pc = 0;
        for instr in instructions {
            match instr {
                Instruction::Increment => {
                    writeln!(f, "\tinc dword [dp]")?;
                },
                Instruction::Decrement => {
                    writeln!(f, "\tdec dword [dp]")?;
                },
                Instruction::Add => {
                    writeln!(f, "\tmov ebx, [dp]")?;
                    writeln!(f, "\tinc dword [tape + 4 * ebx]")?;
                },
                Instruction::Subtract => {
                    writeln!(f, "\tmov ebx, [dp]")?;
                    writeln!(f, "\tdec dword [tape + 4 * ebx]")?;
                },
                Instruction::Write => {
                    writeln!(f, "\tmov ebx, [dp]")?;
                    writeln!(f, "\tmov ecx, [tape + 4 * ebx]")?;
                    writeln!(f, "\tcall putchar")?;
                },
                Instruction::Read => {
                    writeln!(f, "\tcall _getch")?;
                    writeln!(f, "\tmov ebx, [dp]")?;
                    writeln!(f, "\tmov [tape + 4 * ebx], eax")?;
                },
                Instruction::Jump(jmp_pc) => {
                    writeln!(f, "JUMP_{}:", pc)?;
                    writeln!(f, "\tmov ebx, [dp]")?;
                    writeln!(f, "\tcmp dword [tape + 4 * ebx], 0")?;
                    writeln!(f, "\tje RETURN_{}", jmp_pc)?;
                },
                Instruction::Return(jmp_pc) => {
                    writeln!(f, "RETURN_{}:", pc)?;
                    writeln!(f, "\tmov ebx, [dp]")?;
                    writeln!(f, "\tcmp dword [tape + 4 * ebx], 0")?;
                    writeln!(f, "\tjne JUMP_{}", jmp_pc)?;
                },
            }

            pc += 1;
        }

        // Leave stack frame and return with 0
        writeln!(f)?;
        writeln!(f, "\tmov rsp, rbp")?;
        writeln!(f, "\tpop rbp")?;
        writeln!(f)?;
        writeln!(f, "\txor rax, rax")?;
        writeln!(f, "\tret")?;

        Ok(())
    } else {
        // Return error on failure
        Err(Error::new(ErrorKind::Other, "could not write to file!"))
    }
}