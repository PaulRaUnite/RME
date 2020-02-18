mod machine;
use machine::Environment;
use machine::Instruction::*;

fn main() {
    let instructions = vec![
        Increase(1),
        Jump {
            first: 1,
            second: 1,
            goto: 1,
        },
    ];
    println!("{:?}", instructions);

    let mut env = Environment::new(instructions);
    for (line, regs) in &mut env {
        println!("line: {}", line);
        println!("{:?}", regs)
    }
    println!("{:?}", env.regs());
}
