/*
   Optimizing brainfuck implementation of dialect based on
   Daniel's dbfi (see "A very short self-interpreter")
   This interpreter has only one input: program and input to the
   program have to be separated with ! e.g. ",.!a" prints 'a'
   To use it in interactive mode paste your program as input.
   This program can be compiled with NOLNR macro defined.
   NOLNR disables optimization of linear loops (where '<>' balanced), e.g. [->+>++<<].
   Linear loop is then executed in one step.
   Oleg Mazonka 4 Dec 2006  http://mazonka.com/
   Updated by Maneren in 2022
   */

#include <stdlib.h>
#include <stdio.h>
#include <string.h>

typedef struct _Instruction
{
  int shift, offset;
  int *run, run_length;
  struct _Instruction *go;
  int go_index;
  int linear;
  int ch;
  int *db, db_length;
} Instruction;

void *resize_array(void *pointer, int new_size, int original_size)
{
  pointer = realloc(pointer, new_size);
  memset((char *)pointer + original_size, 0, new_size - original_size);
  return pointer;
}
#define resize_int_array(pointer, new_size, original_size) resize_array(pointer, (new_size) * sizeof(int), (original_size) * sizeof(int));

void print_instruction(Instruction *z)
{
  printf("op: c=%c, db='", z->ch);

  /* if (!strchr("<>+-", z->ch))
     printf("%c", (char)z->ch); */

  for (int i = 0; i < z->db_length; i++)
    printf("%c", (char)z->db[i]);

  printf("' shift=%d offset=%d go_index=%d ", z->shift, z->offset, z->go_index);

  if (z->linear)
    printf("linear=%d ", z->linear);

  printf("d=[ ");

  for (int i = 0; i < z->run_length; i++)
    printf("%d ", z->run[i]);

  printf("]\n");
}

int load_next_char()
{
  for (int current_char = getchar(); current_char != EOF; current_char = getchar())
    if (strchr(",.[]+-<>!", current_char))
      return current_char;

  return -1;
}

int consume(Instruction *instruction)
{
  int memory_pointer = 0;
  int current_char = instruction->ch;

  if (strchr("[]", current_char))
    current_char = load_next_char();

  // initialize the run array to one element
  instruction->run_length = 1;
  instruction->run = resize_int_array(0, 1, 0);

  instruction->offset = 0;

  instruction->db_length = 0;
  instruction->db = 0;

  for (; current_char != -1; current_char = load_next_char())
  {
    if (strchr("!,.[]", current_char))
      break;

    instruction->db = resize_int_array(instruction->db, instruction->db_length + 1, instruction->db_length);
    instruction->db[instruction->db_length++] = current_char;

    switch (current_char)
    {
    case '+':
      instruction->run[memory_pointer]++;
      break;

    case '-':
      instruction->run[memory_pointer]--;
      break;

    case '>':
      memory_pointer++;
      if (memory_pointer >= instruction->run_length)
      {
        instruction->run = resize_int_array(instruction->run, instruction->run_length + 1, instruction->run_length);
        instruction->run_length++;
      }
      break;

    case '<':
      if (memory_pointer > 0)
      {
        memory_pointer--;
      }
      else
      {
        instruction->offset--;

        // prepend 0 to the run array
        instruction->run = resize_int_array(instruction->run, instruction->run_length + 1, instruction->run_length);

        for (int i = instruction->run_length; i > 0; i--)
          instruction->run[i] = instruction->run[i - 1];

        instruction->run[0] = 0;

        instruction->run_length++;
      }
    }
  }

  // offset from the beggining of the run
  instruction->shift = memory_pointer + instruction->offset;

  // remove empty fields on the end
  while (instruction->run_length && instruction->run[instruction->run_length - 1] == 0)
    instruction->run_length--;

  // remove empty fields on the start
  while (instruction->run_length && instruction->run[0] == 0)
  {
    instruction->run_length--;

    // shift whole array one to the left
    for (int i = 0; i < instruction->run_length; i++)
    {
      instruction->run[i] = instruction->run[i + 1];
    }

    instruction->offset++;
  }

  return current_char;
}

int find_matching_opening(Instruction *instructions, int length)
{
  int depth = 1;
  int opening_index = length;

  // find matching opening bracket
  while (depth && opening_index-- > 0)
  {
    int ch = instructions[opening_index].ch;
    depth += (ch == ']') - (ch == '[');
  }

  if (opening_index < 0)
  {
    printf("unbalanced ']'\n");
    exit(1);
  }

  return opening_index;
}

int load_instructions(Instruction **instructions)
{
  int length = 0;

  for (int current_char = load_next_char(); current_char != -1 && current_char != '!'; length++)
  {
    *instructions = resize_array(
        *instructions,
        (length + 1) * sizeof(Instruction),
        length * sizeof(Instruction));

    Instruction *current = &(*instructions)[length];

    current->ch = current_char;

    if (strchr(",.", current_char))
    {
      // don't do anything special
      current_char = load_next_char();
      continue;
    }

    if (current_char == ']')
    {
      int opening_index = find_matching_opening(*instructions, length);

      current->go_index = opening_index;
      (*instructions)[opening_index].go_index = length;
    }

    current_char = consume(current);
  }

  return length;
}

void link_jumps(Instruction *instructions, int length)
{
  for (int i = 0; i < length; i++)
  {
    Instruction *current = &instructions[i];
    // link jumps together
    current->go = &instructions[current->go_index];

    if (current->ch == '[' && current->go_index == i + 1 && current->shift == 0 && current->offset <= 0)
    {
      current->linear = -current->run[-current->offset];
      if (current->linear < 0)
      {
        printf("Warning: infinite loop ");
        print_instruction(current);
        printf("linear=%d\n", current->linear);
        current->linear = 0;
      }
    }
  }
}

void interpret(Instruction *instructions, int length)
{
  int memory_size = 1024;
  int *memory = resize_int_array(0, memory_size, 0);
  int memory_pointer = 0;

  for (Instruction *current = &instructions[0]; current < instructions + length; current++)
  {
    switch (current->ch)
    {
    case ']':
      if (memory[memory_pointer] != 0)
        current = current->go;
      break;
    case '[':
      if (memory[memory_pointer] == 0)
        current = current->go;
      break;
    case '.':
      putchar(memory[memory_pointer]);
      break;
    case ',':
      memory[memory_pointer] = getchar();
      break;
    }

    if (current->run_length)
    {
      int new_memory_size = memory_pointer + current->run_length + current->offset;
      if (new_memory_size > memory_size)
      {
        memory = resize_int_array(memory, new_memory_size, memory_size);
        memory_size = new_memory_size;
      }

      if (current->linear)
      {
        // compute how many iteration we have to do
        int factor = memory[memory_pointer] / current->linear;
        int is_exact = memory[memory_pointer] % current->linear == 0;

        if (is_exact)
        {
          for (int i = 0; i < current->run_length; i++)
            memory[memory_pointer + current->offset + i] += factor * current->run[i];
        }
        else
        {
          while (memory[memory_pointer] != 0)
            for (int i = 0; i < current->run_length; i++)
              memory[memory_pointer + current->offset + i] += current->run[i];
        }
      }
      else
      {
        for (int i = 0; i < current->run_length; i++)
          memory[memory_pointer + current->offset + i] += current->run[i];
      }
    }

    if (current->shift > 0)
    {
      int new_memory_size = memory_pointer + current->shift + 1;
      if (new_memory_size > memory_size)
      {
        memory = resize_int_array(memory, new_memory_size, memory_size);
        memory_size = new_memory_size;
      }
    }

    memory_pointer += current->shift;
  }
}

int main()
{
  Instruction *instructions = 0;

  int instructions_length = load_instructions(&instructions);

  link_jumps(instructions, instructions_length);

  interpret(instructions, instructions_length);

  return 0;
}
