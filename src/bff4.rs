use std::io::{stdin, stdout, Read, Write};

#[derive(Debug, PartialEq, Eq)]
struct Instruction {
  shift: i32,
  offset: i32,
  run: Vec<i32>,
  go: Option<*const Instruction>,
  go_index: usize,
  linear: bool,
  char: char,
}

impl Instruction {
  fn new(char: char) -> Self {
    Self {
      shift: 0,
      offset: 0,
      run: vec![],
      go: None,
      go_index: 0,
      linear: false,
      char,
    }
  }
}

fn strchr(s: &str, c: char) -> bool {
  s.find(c).is_some()
}

fn getchar() -> Option<char> {
  let mut buffer = [0u8; 1];

  match stdin().read_exact(&mut buffer) {
    Ok(_) => Some(buffer[0] as char),
    Err(_) => None,
  }
}

fn putchar(c: i32) {
  stdout().write_all(&[c as u8]).unwrap();
}

fn offset_ptr<T>(ptr: *const T, index: usize) -> *const T {
  unsafe { ptr.add(index) }
}

fn load_next_char() -> Option<char> {
  while let Some(current_char) = getchar() {
    if strchr(",.[]+-<>!", current_char) {
      return Some(current_char);
    }
  }

  None
}

fn consume(instruction: &mut Instruction) -> Option<char> {
  let mut memory_pointer = 0;
  let mut current_char = Some(instruction.char);

  if strchr("[]", current_char.unwrap()) {
    current_char = load_next_char();
    current_char?;
  }

  instruction.run = vec![0];
  instruction.offset = 0;

  while let Some(char) = current_char {
    if strchr("!,.[]", char) {
      break;
    }

    match char {
      '+' => instruction.run[memory_pointer] += 1,
      '-' => instruction.run[memory_pointer] -= 1,
      '>' => {
        memory_pointer += 1;

        if memory_pointer >= instruction.run.len() {
          instruction.run.push(0)
        }
      }
      '<' => {
        if memory_pointer > 0 {
          memory_pointer -= 1;
        } else {
          instruction.offset -= 1;

          instruction.run.insert(0, 0);
        }
      }
      _ => unreachable!(),
    }

    current_char = load_next_char();
  }

  // offset from the beggining of the run
  instruction.shift = memory_pointer as i32 + instruction.offset;

  while let Some(0) = instruction.run.last() {
    instruction.run.pop();
  }

  while let Some(0) = instruction.run.first() {
    instruction.run.remove(0);
    instruction.offset += 1;
  }

  current_char
}

fn find_matching_opening(instructions: &[Instruction]) -> usize {
  let mut depth = 1;
  let mut opening_index = instructions.len() - 1;

  // find matching opening bracket
  loop {
    if opening_index == 0 {
      panic!("No matching opening bracket found!");
    }

    let ch = instructions[opening_index].char;
    depth += (ch == ']') as i32 - (ch == '[') as i32;

    if depth == 0 {
      break;
    }

    opening_index -= 1;
  }

  opening_index
}

fn load_instructions() -> Vec<Instruction> {
  let mut instructions = Vec::new();

  let mut current_char = load_next_char();

  while let Some(char) = current_char {
    if strchr("!", char) {
      break;
    }

    let mut current = Instruction::new(char);

    if strchr(",.", char) {
      // don't do anything special
      current_char = load_next_char();
      instructions.push(current);
      continue;
    }

    if char == ']' {
      let opening_index = find_matching_opening(&instructions);

      current.go_index = opening_index;
      instructions[opening_index].go_index = instructions.len();
    }

    current_char = consume(&mut current);

    instructions.push(current);
  }

  instructions
}

fn link_jumps(instructions: &mut [Instruction]) {
  let ptr = instructions.as_ptr();

  for (i, instruction) in instructions.iter_mut().enumerate() {
    instruction.go = Some(offset_ptr(ptr, instruction.go_index));

    if instruction.char == '['
      && instruction.go_index == i + 1
      && instruction.shift == 0
      && instruction.offset <= 0
    {
      let linear_factor = -instruction
        .run
        .get(-instruction.offset as usize)
        .unwrap_or(&0);

      match linear_factor.cmp(&0) {
        std::cmp::Ordering::Less => {
          println!("Warning: infinite loop");
          println!("{:?}", instruction);

          instruction.linear = false;
        }
        std::cmp::Ordering::Equal => instruction.linear = false,
        std::cmp::Ordering::Greater => instruction.linear = true,
      }
    }
  }
}

fn interpret(instructions: Vec<Instruction>) {
  let mut memory = vec![0; 1024];
  let mut memory_pointer = 0;

  let mut current_ptr = instructions.as_ptr();
  let mut current = unsafe { &*current_ptr };
  let end = offset_ptr(current_ptr, instructions.len());

  while current_ptr < end {
    match current.char {
      ']' => {
        if memory[memory_pointer] != 0 {
          current_ptr = current.go.unwrap();
          current = unsafe { &*current_ptr };
        }
      }

      '[' => {
        if memory[memory_pointer] == 0 {
          current_ptr = current.go.unwrap();
          current = unsafe { &*current_ptr };
        }
      }
      '.' => putchar(memory[memory_pointer]),
      ',' => memory[memory_pointer] = getchar().unwrap() as i32,
      _ => {}
    }

    if !current.run.is_empty() {
      let new_memory_size = memory_pointer + current.run.len() + current.offset as usize;
      if new_memory_size > memory.len() {
        memory.resize(new_memory_size, 0);
      }

      if current.linear {
        while memory[memory_pointer] != 0 {
          for (i, value) in current.run.iter().enumerate() {
            memory[memory_pointer + current.offset as usize + i] += value;
          }
        }
      } else {
        for i in 0..current.run.len() {
          memory[memory_pointer + current.offset as usize + i] += current.run[i];
        }
      }
    }

    if current.shift > 0 {
      let new_memory_size = memory_pointer + current.shift as usize + 1;
      if new_memory_size > memory.len() {
        memory.resize(new_memory_size, 0);
      }
    }

    memory_pointer = memory_pointer.wrapping_add(current.shift as usize);

    current_ptr = offset_ptr(current_ptr, 1);
    current = unsafe { &*current_ptr };
  }
}

fn main() {
  let mut instructions = load_instructions();

  link_jumps(&mut instructions);

  interpret(instructions);
}
