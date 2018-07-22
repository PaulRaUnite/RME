mod machine;
use machine::Instruction::*;
use machine::Environment;

fn main() {
    let instr = vec![
        Increase(1),
        Jump {
            first: 1,
            second: 1,
            goto: 0,
        },
    ];
    let mut env = Environment::new(&instr);
    for (line, instructions, regs) in &mut env {
        println!("line: {}, {:?}", line, instructions);
        println!("{:?}", regs)
    }
    println!("{:?}", env.regs());
}
