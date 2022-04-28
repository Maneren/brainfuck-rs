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
   */

#include <stdlib.h>
#include <stdio.h>
#include <string.h>

typedef struct _Instruction
{
  int shift, offset;
  int *d, d_length;
  struct _Instruction *go;
  int c;
  int index_go, linear;
  int *db, db_length;
} Instruction;

void *resize_array(void *pointer, int new_size, int original_size)
{
  pointer = realloc(pointer, new_size);
  memset((char *)pointer + original_size, 0, new_size - original_size);
  return pointer;
}
#define resize_int_array(pointer, new_size, original_size) resize_array(pointer, (new_size) * sizeof(int), (original_size) * sizeof(int));

void printop(Instruction *z)
{
  printf("op: c=%c, d='", z->c);

  /* if (!strchr("<>+-", z->c))
     printf("%c", (char)z->c); */

  for (int i = 0; i < z->db_length; i++)
    printf("%c", (char)z->db[i]);

  printf("' shift=%d offset=%d index_go=%d db=[ ", z->shift, z->offset, z->index_go);

  for (int i = 0; i < z->d_length; i++)
    printf("%d ", z->d[i]);

  printf("]\n");
}

int load_next_char()
{
  int current_char;
next:
  current_char = getchar();

  if (current_char == -1)
    return -1;

  if (!strchr(",.[]+-<>!", current_char))
    goto next;

  return current_char;
}

int consume(Instruction *instruction)
{
  int memory_pointer = 0;
  int current_char = instruction->c;

  if (strchr("[]", current_char))
    current_char = load_next_char();

  instruction->d_length = 1;
  instruction->d = resize_int_array(0, 1, 0);
  instruction->offset = 0;

  instruction->db_length = 0;
  instruction->db = 0;

  for (;; current_char = load_next_char())
  {
    if (current_char == -1 || current_char == '!')
      break;
    if (strchr(",.[]", current_char))
      break;

    instruction->db = resize_int_array(instruction->db, instruction->db_length + 1, instruction->db_length);
    instruction->db[instruction->db_length++] = current_char;

    if (current_char == '+')
      instruction->d[memory_pointer]++;

    else if (current_char == '-')
      instruction->d[memory_pointer]--;

    else if (current_char == '>')
    {
      memory_pointer++;
      if (memory_pointer >= instruction->d_length)
      {
        instruction->d = resize_int_array(instruction->d, instruction->d_length + 1, instruction->d_length);
        instruction->d_length++;
      }
    }
    else if (current_char == '<')
    {
      if (memory_pointer > 0)
        memory_pointer--;
      else
      {
        instruction->offset--;
        instruction->d = resize_int_array(instruction->d, instruction->d_length + 1, instruction->d_length);
        for (int i = instruction->d_length; i > 0; i--)
          instruction->d[i] = instruction->d[i - 1];
        instruction->d[0] = 0;
        instruction->d_length++;
      }
    }
  }
  instruction->shift = memory_pointer + instruction->offset;

  /* cut corners */
  while (instruction->d_length && instruction->d[instruction->d_length - 1] == 0)
    instruction->d_length--;
  while (instruction->d_length && instruction->d[0] == 0)
  {
    instruction->d_length--;
    for (int i = 0; i < instruction->d_length; i++)
      instruction->d[i] = instruction->d[i + 1];
    instruction->offset++;
  }

  return current_char;
}

int main()
{
  Instruction *instruction_array = 0, *instruction_array_end;
  int instruction_array_length = 0, i;
  int current_char = load_next_char();

  for (;; instruction_array_length++)
  {
    instruction_array = resize_array(
        instruction_array,
        (instruction_array_length + 1) * sizeof(Instruction),
        instruction_array_length * sizeof(Instruction));

    if (current_char == -1 || current_char == '!')
      break;

    instruction_array[instruction_array_length].c = current_char;

    if (strchr(",.", current_char))
    {
      // don't do anything special
      current_char = load_next_char();
      continue;
    }

    if (current_char == ']')
    {
      int depth = 1, index = instruction_array_length;

      // find matching opening bracket
      while (depth && index >= 0)
        if (index--)
        {
          int ch = instruction_array[index].c;
          depth += (ch == ']') - (ch == '[');
        }

      if (index < 0)
      {
        printf("unbalanced ']'\n");
        exit(1);
      }

      instruction_array[index].index_go = instruction_array_length;
      instruction_array[instruction_array_length].index_go = index;
    }
    current_char = consume(instruction_array + instruction_array_length);
  }

  for (i = 0; i < instruction_array_length; i++)
  {
    Instruction *current = instruction_array + i;
    // link jumps together
    current->go = &instruction_array[current->index_go];

    if (current->c == '[' && current->index_go == i + 1 && current->shift == 0 && current->offset <= 0)
    {
      current->linear = -current->d[-current->offset];
      printf("linear: %d\n", current->linear);
      if (current->linear < 0)
      {
        printf("Warning: infinite loop ");
        printop(current);
        printf("linear=%d\n", current->linear);
        current->linear = 0;
      }
    }
  }

  /*  for (size_t i = 0; i < instruction_array_length; i++)
      {
      printop(instruction_array + i);
      } */

  int memory_size = 1000; /* any number */
  int *memory = resize_int_array(0, memory_size, 0);
  int memory_pointer = 0;

  Instruction *current_instruction = instruction_array;
  instruction_array_end = instruction_array + instruction_array_length;

  // run
  for (; current_instruction < instruction_array_end; ++current_instruction)
  {
    if (current_instruction->c == ']')
    {
      if (memory[memory_pointer] != 0)
        current_instruction = current_instruction->go;
    }

    else if (current_instruction->c == '[')
    {
      if (memory[memory_pointer] == 0)
        current_instruction = current_instruction->go;
    }

    else if (current_instruction->c == ',')
    {
      memory[memory_pointer] = getchar();
      continue;
    }

    else if (current_instruction->c == '.')
    {
      putchar(memory[memory_pointer]);
      continue;
    }

    /* apply */
    if (current_instruction->d_length)
    {
      int nmsz = memory_pointer + current_instruction->d_length + current_instruction->offset;
      if (nmsz > memory_size)
      {
        memory = resize_int_array(memory, nmsz, memory_size);
        memory_size = nmsz;
      }

      if (current_instruction->linear)
      {
        int del = memory[memory_pointer] / current_instruction->linear;
        for (i = 0; i < current_instruction->d_length; i++)
        {
          memory[memory_pointer + current_instruction->offset + i] += del * current_instruction->d[i];
        }
      }
      else
      {
        for (i = 0; i < current_instruction->d_length; i++)
          memory[memory_pointer + current_instruction->offset + i] += current_instruction->d[i];
      }
    }

    if (current_instruction->shift > 0)
    {
      int nmsz = memory_pointer + current_instruction->shift + 1;
      if (nmsz > memory_size)
      {
        memory = resize_int_array(memory, nmsz, memory_size);
        memory_size = nmsz;
      }
    }
    memory_pointer += current_instruction->shift;
  }
  return 0;
}
