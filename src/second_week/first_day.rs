struct Program {
    acc: i16,
    instructions: Vec<Instruction>,
    visited: Vec<bool>,
    fixed: Vec<i16>,
    run: bool,
    has_fix: bool,
}

impl Program {
    fn new(instructions: Vec<Instruction>) -> Self {
        let visited = (0..instructions.len()).map(|_| false).collect();
        Self {
            acc: 0,
            instructions,
            visited,
            fixed: Vec::new(),
            run: true,
            has_fix: true,
        }
    }

    fn reset(&mut self) {
        self.acc = 0;
        self.has_fix = false;
        self.visited = (0..self.instructions.len()).map(|_| false).collect();
    }
}

#[derive(Debug, Eq, PartialEq)]
enum Instruction {
    Acc(i16),
    Jmp(i16),
    Nop(i16),
}

impl Instruction {
    fn new(inp: String) -> Self {
        let mut split = inp.split_whitespace();
        let ins = split.next().unwrap();
        let mov: i16 = split.next().map(|s| s.parse().unwrap_or(0 as i16)).unwrap();
        if ins == "acc" {
            return Self::Acc(mov);
        } else if ins == "jmp" {
            return Self::Jmp(mov);
        }
        Self::Nop(mov)
    }

    fn fix_instruction(&self) -> Self {
        match self {
            Instruction::Jmp(i) => Instruction::Nop(*i),
            Instruction::Nop(i) => Instruction::Jmp(*i),
            // An acc instruction should not be fixed
            _ => Instruction::Acc(0),
        }
    }

    fn change_index(&self, index: i16) -> i16 {
        match self {
            Instruction::Acc(_) | Instruction::Nop(_) => index + 1,
            Instruction::Jmp(i) => index + i,
        }
    }
}

pub fn execute_bug_program(input: Vec<String>) -> i16 {
    let instructions = input.into_iter().map(Instruction::new).collect();
    let mut program = Program::new(instructions);
    let mut index: i16 = 0;
    while program.run {
        if let Some(ins) = program.instructions.get(index as usize) {
            //safe to search index as this vec mirrors instructions
            if program.visited[index as usize] {
                program.run = false;
                break;
            }
            program.visited[index as usize] = true;
            match ins {
                Instruction::Acc(i) => {
                    index += 1;
                    program.acc += i;
                }
                Instruction::Jmp(_) | Instruction::Nop(_) => index = ins.change_index(index),
            }
        } else {
            program.run = false;
        }
    }
    program.acc
}

pub fn execute_and_fix_program(input: Vec<String>) -> i16 {
    let instructions: Vec<Instruction> = input.into_iter().map(Instruction::new).collect();
    let mut program = Program::new(instructions);
    let mut index: i16 = 0;
    while program.run {
        if let Some(ins) = program.instructions.get(index as usize) {
            //safe to search index as this vec mirrors instructions
            if program.visited[index as usize] {
                program.reset();
                index = 0;
                continue;
            }
            program.visited[index as usize] = true;
            match ins {
                Instruction::Acc(i) => {
                    index += 1;
                    program.acc += i;
                }
                Instruction::Nop(0) => index += 1,
                Instruction::Jmp(_) | Instruction::Nop(_) => {
                    if !program.has_fix && !program.fixed.contains(&index) {
                        program.fixed.push(index);
                        program.has_fix = true;
                        index = ins.fix_instruction().change_index(index);
                    } else {
                        index = ins.change_index(index);
                    }
                }
            }
        } else {
            program.run = false;
        }
    }
    program.acc
}

#[test]
fn test_new_instruction() {
    let inp = "acc +16".to_string();
    assert_eq!(Instruction::Acc(16), Instruction::new(inp));
    let inp = "jmp -2".to_string();
    assert_eq!(Instruction::Jmp(-2), Instruction::new(inp));
    let inp = "nop +0".to_string();
    assert_eq!(Instruction::Nop(0), Instruction::new(inp));
}

#[test]
fn test_fix_instruction() {
    let ins = Instruction::Acc(2);
    assert_eq!(Instruction::Acc(0), ins.fix_instruction());
    let ins = Instruction::Jmp(-2);
    assert_eq!(Instruction::Nop(-2), ins.fix_instruction());
    let ins = Instruction::Nop(16);
    assert_eq!(Instruction::Jmp(16), ins.fix_instruction());
}

#[test]
fn test_execute_program() {
    let input = vec![
        "nop +0".to_string(),
        "acc +1".to_string(),
        "jmp +4".to_string(),
        "acc +3".to_string(),
        "jmp -3".to_string(),
        "acc -99".to_string(),
        "acc +1".to_string(),
        "jmp -4".to_string(),
        "acc +6".to_string(),
    ];
    assert_eq!(5, execute_bug_program(input))
}

#[test]
fn test_execute_and_fix_program() {
    let input = vec![
        "nop +0".to_string(),
        "acc +1".to_string(),
        "jmp +4".to_string(),
        "acc +3".to_string(),
        "jmp -3".to_string(),
        "acc -99".to_string(),
        "acc +1".to_string(),
        "jmp -4".to_string(),
        "acc +6".to_string(),
    ];
    assert_eq!(8, execute_and_fix_program(input))
}
