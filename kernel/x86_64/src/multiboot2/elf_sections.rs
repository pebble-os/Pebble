/*
 * Copyright (C) 2016, Philipp Oppermann.
 * Copyright (C) 2017, Pebble Developers.
 * See LICENCE.md
 */

use core::{str,slice};
use ::memory::paging::{PhysicalAddress,VirtualAddress};

#[derive(Clone,Copy,Debug)]
#[repr(packed)]
pub struct ElfSectionsTag
{
    typ                 : u32,
    size                : u32,
    number_of_sections  : u32,
    entry_size          : u32,
    shndx               : u32, // string table
    first_section       : ElfSection,
}

impl ElfSectionsTag
{
    pub fn sections(&'static self) -> ElfSectionIter
    {
        ElfSectionIter
        {
            current_section     : unsafe { &self.first_section },
            remaining_sections  : self.number_of_sections - 1,
            entry_size          : self.entry_size,
        }
    }

    pub fn string_table(&self) -> &'static StringTable
    {
        unsafe
        {
            let string_table_ptr = (&self.first_section as *const ElfSection).offset(self.shndx as isize);
            &*((*string_table_ptr).start_as_physical().in_kernel_space().ptr() as *const StringTable)
        }
    }
}

pub struct StringTable(u8);

impl StringTable
{
    pub fn section_name(&self, section: &ElfSection) -> &'static str
    {
        let name_ptr = unsafe
                       {
                           (&self.0 as *const u8).offset(section.name_index as isize)
                       };

        let length = {
                         let mut len = 0;

                         // TODO: This is a bug in clippy - see rust-clippy#2584
                         #[allow(while_immutable_condition)]
                         while unsafe { *name_ptr.offset(len) } != 0
                         {
                             len += 1;
                         }

                         len as usize
                     };

        str::from_utf8(unsafe { slice::from_raw_parts(name_ptr, length) }).unwrap()
    }
}

#[derive(Clone)]
pub struct ElfSectionIter
{
    current_section     : &'static ElfSection,
    remaining_sections  : u32,
    entry_size          : u32,
}

impl Iterator for ElfSectionIter
{
    type Item = &'static ElfSection;

    fn next(&mut self) -> Option<&'static ElfSection>
    {
        if self.remaining_sections == 0
        {
            None
        }
        else
        {
            let section = self.current_section;
            let next_section_addr = (self.current_section as *const _ as u64) + u64::from(self.entry_size);
            self.current_section = unsafe { &*(next_section_addr as *const ElfSection) };
            self.remaining_sections -= 1;

            if section.typ == ElfSectionType::Unused as u32
            {
                self.next()
            }
            else
            {
                Some(section)
            }
        }
    }
}

#[derive(Clone,Copy,Debug)]
#[repr(C)]
pub struct ElfSection
{
    name_index  : u32,
    typ         : u32,
    flags       : u64,
    addr        : u64,      // XXX: Depending on the section, this could be physical **or** virtual!
    offset      : u64,
    size        : u64,
    link        : u32,
    info        : u32,
    addralign   : u64,
    entry_size  : u64,
}

impl ElfSection {
    #[allow(unused)]
    pub fn section_type(&self) -> ElfSectionType
    {
        match self.typ
        {
            0                           => ElfSectionType::Unused,
            1                           => ElfSectionType::ProgramSection,
            2                           => ElfSectionType::LinkerSymbolTable,
            3                           => ElfSectionType::StringTable,
            4                           => ElfSectionType::RelaRelocation,
            5                           => ElfSectionType::SymbolHashTable,
            6                           => ElfSectionType::DynamicLinkingTable,
            7                           => ElfSectionType::Note,
            8                           => ElfSectionType::Uninitialized,
            9                           => ElfSectionType::RelRelocation,
            10                          => ElfSectionType::Reserved,
            11                          => ElfSectionType::DynamicLoaderSymbolTable,
            0x6000_0000...0x6FFF_FFFF   => ElfSectionType::EnvironmentSpecific,
            0x7000_0000...0x7FFF_FFFF   => ElfSectionType::ProcessorSpecific,
            _ => panic!(),
        }
    }

    pub fn start_as_physical(&self) -> PhysicalAddress
    {
        PhysicalAddress::new(self.addr as usize)
    }

    pub fn start_as_virtual(&self) -> VirtualAddress
    {
        VirtualAddress::new(self.addr as usize)
    }

    pub fn end_as_virtual(&self) -> VirtualAddress
    {
        self.start_as_virtual().offset(self.size as isize)
    }

    pub fn size(&self) -> usize
    {
        self.size as usize
    }

    pub fn flags(&self) -> ElfSectionFlags
    {
        ElfSectionFlags::from_bits_truncate(self.flags)
    }

    pub fn is_allocated(&self) -> bool
    {
        self.flags().contains(ElfSectionFlags::ALLOCATED)
    }
}

#[derive(Clone,Copy,Debug,PartialEq,Eq)]
#[repr(u32)]
#[allow(unused)]
pub enum ElfSectionType
{
    Unused                      = 0,
    ProgramSection              = 1,
    LinkerSymbolTable           = 2,
    StringTable                 = 3,
    RelaRelocation              = 4,
    SymbolHashTable             = 5,
    DynamicLinkingTable         = 6,
    Note                        = 7,
    Uninitialized               = 8,
    RelRelocation               = 9,
    Reserved                    = 10,
    DynamicLoaderSymbolTable    = 11,
    EnvironmentSpecific         = 0x6000_0000,
    ProcessorSpecific           = 0x7000_0000,
}

type ElfSectionFlagsType = u64;

bitflags!
{
    pub struct ElfSectionFlags : ElfSectionFlagsType
    {
        const WRITABLE      = 0x1;
        const ALLOCATED     = 0x2;
        const EXECUTABLE    = 0x4;
        // plus environment-specific use at 0x0F000000
        // plus processor-specific use at 0xF0000000
    }
}
