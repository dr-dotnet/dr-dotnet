## Considerations When Rewriting IL

- Expand from a tiny method header to a fat method header if any of the following conditions are true:
  - A local variable exists
  - An exception exists
  - An extra data section exists
  - The operand stack > 8 entries
  - code_size (not counting 1 byte header) >= 64 bytes (This isn't explicitly stated, but the method size in bytes must be able to be encoded in 6 bits, so max is 2^6 -1)
- The CodeSize will need updated whether we have a fat or tiny header.
- If a fat method header, we may need to update the MaxStack.
- Expand short form opcode to long form opcode if target offset can no longer be stored in i8.
  - E.g. if operand > i8::MAX or operand < i8::MIN then expand
- Exception Handler Clause offsets and lengths will need updated (length only if we added/removed instructions in a try/catch/finally block).
- Exception Handler Section Headers and Clauses need to be expanded from the small form to the fat form if any of the following conditions are true:
  - The try length  > u8::MAX
  - The handler length > u8::MAX
  - The try offset > u16::MAX
  - The handler offset > u16::MAX