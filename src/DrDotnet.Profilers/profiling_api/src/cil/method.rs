#![allow(non_upper_case_globals)]
use crate::cil::{nearest_multiple, Error, Instruction, MethodHeader, Section};
use std::convert::TryFrom;
use std::slice;

#[derive(Debug)]
pub struct Method {
    pub method_header: MethodHeader,
    pub instructions: Vec<Instruction>,
    pub sections: Vec<Section>,
}
impl Method {
    pub fn new(method_header: *const u8, method_size: u32) -> Result<Self, Error> {
        let body = unsafe { slice::from_raw_parts(method_header, method_size as usize) };
        let method_header = MethodHeader::from_bytes(&body)?;
        let (instructions_start, instructions_end): (usize, usize) = match &method_header {
            MethodHeader::Fat(header) => (12, (12 + header.code_size - 1) as usize),
            MethodHeader::Tiny(header) => (1, header.code_size as usize),
        };
        let instruction_bytes = &body[instructions_start..=instructions_end];
        let instructions = Self::instructions_from_bytes(instruction_bytes)?;
        let sections = match &method_header {
            MethodHeader::Fat(header) if header.more_sects => {
                let sections_start = nearest_multiple(4, instructions_end + 1); // Sections must be DWORD aligned
                let sections_bytes = &body[sections_start..];
                Self::sections_from_bytes(sections_bytes)?
            }
            _ => Vec::new(), // only fat headers with the more sections flag set have additional sections
        };
        Ok(Method {
            method_header,
            instructions,
            sections,
        })
    }
    pub fn into_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.append(&mut self.method_header.into_bytes());
        bytes.append(&mut self.instructions_to_bytes());
        bytes.append(&mut self.sections_to_bytes());
        bytes
    }
    pub fn insert_prelude(&mut self, prelude: Vec<Instruction>) -> Result<(), Error> {
        // For now ignore the operand stack. Assume we aren't exceeding the previous max stack size.
        // Also assume we aren't adding any new exceptions or new method data sections.
        // Also assume we aren't adding any local variables (I think this would require modifying the metadata)
        // Also assume we don't need to expand any short branches, tiny headers, or small sections.

        let prelude_length: usize = prelude.iter().map(|i| i.into_bytes().len()).sum();
        // update code_size in method_header
        match &mut self.method_header {
            MethodHeader::Fat(header) => {
                let size = header.code_size as u128 + prelude_length as u128;
                header.code_size = u32::try_from(size).or(Err(Error::PreludeTooBig))?;
            }
            MethodHeader::Tiny(header) => {
                let size = header.code_size as u128 + prelude_length as u128;
                header.code_size = u8::try_from(size)
                    .or_else(|err| todo!("Expand into fat header!, {:?}", err))?;
            }
        }
        // update try offset
        // update handler offset
        for section in &mut self.sections {
            match section {
                Section::FatSection(_, clauses) => {
                    for clause in clauses {
                        let try_offset = clause.try_offset as u128 + prelude_length as u128;
                        clause.try_offset =
                            u32::try_from(try_offset).or(Err(Error::PreludeTooBig))?;
                        let handler_offset = clause.handler_offset as u128 + prelude_length as u128;
                        clause.handler_offset =
                            u32::try_from(handler_offset).or(Err(Error::PreludeTooBig))?;
                    }
                }
                Section::SmallSection(_, clauses) => {
                    for clause in clauses {
                        let try_offset = clause.try_offset as u128 + prelude_length as u128;
                        clause.try_offset = u16::try_from(try_offset)
                            .or_else(|err| todo!("Expand into fat section!, {:?}", err))?;
                        let handler_offset = clause.handler_offset as u128 + prelude_length as u128;
                        clause.handler_offset = u16::try_from(handler_offset)
                            .or_else(|err| todo!("Expand into fat section!, {:?}", err))?;
                    }
                }
            }
        }
        // Insert the instructions
        self.instructions.splice(0..0, prelude);
        Ok(())
    }
    fn instructions_from_bytes(il: &[u8]) -> Result<Vec<Instruction>, Error> {
        let mut index = 0;
        let mut instructions = Vec::new();
        while index < il.len() {
            let il = &il[index..];
            let instruction = Instruction::from_bytes(il)?;
            index += instruction.length();
            instructions.push(instruction);
        }
        Ok(instructions)
    }
    fn sections_from_bytes(il: &[u8]) -> Result<Vec<Section>, Error> {
        let mut index = 0;
        let mut sections = Vec::new();
        while index < il.len() {
            let il = &il[index..];
            let section = Section::from_bytes(il)?;
            index += section.data_size();
            sections.push(section);
        }
        Ok(sections)
    }
    fn instructions_to_bytes(&self) -> Vec<u8> {
        self.instructions
            .iter()
            .flat_map(|i| i.into_bytes())
            .collect()
    }
    fn sections_to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        match &self.method_header {
            MethodHeader::Fat(header) if header.more_sects => {
                // Sections must be DWORD aligned. Add zero padding at the end to achieve alignment.
                let padding_byte_size = 4 - bytes.len() % 4;
                for _ in 0..padding_byte_size {
                    bytes.push(0);
                }
                let mut section_bytes = self.sections.iter().flat_map(|s| s.into_bytes()).collect();
                bytes.append(&mut section_bytes);
            }
            _ => (),
        }
        bytes
    }
}
