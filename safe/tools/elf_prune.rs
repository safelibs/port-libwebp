use std::collections::BTreeSet;
use std::env;
use std::fs;
use std::io;
use std::path::Path;

const ELFCLASS64: u8 = 2;
const ELFDATA2LSB: u8 = 1;
const STB_GLOBAL: u8 = 1;
const STB_WEAK: u8 = 2;
const STV_HIDDEN: u8 = 2;
const SHN_UNDEF: u16 = 0;

#[derive(Clone, Copy)]
struct SectionHeader {
    name_offset: usize,
    offset: usize,
    size: usize,
    entry_size: usize,
}

fn main() -> Result<(), String> {
    let mut args = env::args().skip(1);
    let mode = args
        .next()
        .ok_or_else(|| "usage: elf_prune <hide|localize|undefine> <elf> <exports>".to_owned())?;
    let elf_path = args.next().ok_or_else(|| "missing ELF path".to_owned())?;
    let exports_path = args
        .next()
        .ok_or_else(|| "missing exports path".to_owned())?;
    if args.next().is_some() {
        return Err("usage: elf_prune <hide|localize|undefine> <elf> <exports>".to_owned());
    }

    let keep = load_exports(Path::new(&exports_path)).map_err(|err| err.to_string())?;
    let mut bytes = fs::read(&elf_path).map_err(|err| err.to_string())?;
    patch_elf(&mut bytes, &keep, &mode)?;
    fs::write(&elf_path, bytes).map_err(|err| err.to_string())
}

fn load_exports(path: &Path) -> io::Result<BTreeSet<String>> {
    Ok(fs::read_to_string(path)?
        .lines()
        .map(str::trim)
        .filter(|line| !line.is_empty())
        .map(ToOwned::to_owned)
        .collect())
}

fn patch_elf(bytes: &mut [u8], keep: &BTreeSet<String>, mode: &str) -> Result<(), String> {
    if bytes.len() < 0x40 || &bytes[..4] != b"\x7fELF" {
        return Err("not an ELF file".to_owned());
    }
    if bytes[4] != ELFCLASS64 || bytes[5] != ELFDATA2LSB {
        return Err("only ELF64 little-endian is supported".to_owned());
    }

    let section_headers_offset = read_u64(bytes, 0x28)? as usize;
    let section_header_size = read_u16(bytes, 0x3a)? as usize;
    let section_count = read_u16(bytes, 0x3c)? as usize;
    let section_name_index = read_u16(bytes, 0x3e)? as usize;

    let sections = read_sections(bytes, section_headers_offset, section_header_size, section_count)?;
    let shstr = section_data(bytes, &sections, section_name_index)?;

    let dynsym_index = find_section(&sections, shstr, ".dynsym")?;
    let dynstr_index = find_section(&sections, shstr, ".dynstr")?;
    let versym_index = find_optional_section(&sections, shstr, ".gnu.version");

    let dynsym = sections[dynsym_index];
    let dynstr = sections[dynstr_index];
    let versym = versym_index.map(|index| sections[index]);

    if dynsym.entry_size == 0 || dynsym.entry_size < 24 {
        return Err("invalid .dynsym entry size".to_owned());
    }

    let symbol_count = dynsym.size / dynsym.entry_size;
    for index in 0..symbol_count {
        let entry_offset = dynsym.offset + index * dynsym.entry_size;
        let name_offset = read_u32(bytes, entry_offset)? as usize;
        let info_offset = entry_offset + 4;
        let other_offset = entry_offset + 5;
        let shndx_offset = entry_offset + 6;
        let value_offset = entry_offset + 8;
        let size_offset = entry_offset + 16;
        let binding = bytes[info_offset] >> 4;
        let shndx = read_u16(bytes, shndx_offset)?;
        let name = read_c_string(bytes, dynstr.offset + name_offset)?;
        if name.is_empty() || keep.contains(name) {
            continue;
        }
        if shndx == SHN_UNDEF || (binding != STB_GLOBAL && binding != STB_WEAK) {
            continue;
        }

        match mode {
            "hide" => {
                bytes[other_offset] = (bytes[other_offset] & !0x3) | STV_HIDDEN;
            }
            "localize" => {
                bytes[info_offset] &= 0x0f;
                if let Some(section) = versym {
                    write_u16(bytes, section.offset + index * 2, 0)?;
                }
            }
            "undefine" => {
                write_u16(bytes, shndx_offset, SHN_UNDEF)?;
                write_u64(bytes, value_offset, 0)?;
                write_u64(bytes, size_offset, 0)?;
                if let Some(section) = versym {
                    write_u16(bytes, section.offset + index * 2, 0)?;
                }
            }
            other => {
                return Err(format!("unknown mode `{other}`"));
            }
        }
    }

    Ok(())
}

fn read_sections(
    bytes: &[u8],
    section_headers_offset: usize,
    section_header_size: usize,
    section_count: usize,
) -> Result<Vec<SectionHeader>, String> {
    let mut sections = Vec::with_capacity(section_count);
    for index in 0..section_count {
        let offset = section_headers_offset
            .checked_add(index.saturating_mul(section_header_size))
            .ok_or_else(|| "section header overflow".to_owned())?;
        let name_offset = read_u32(bytes, offset)? as usize;
        let section_offset = read_u64(bytes, offset + 0x18)? as usize;
        let size = read_u64(bytes, offset + 0x20)? as usize;
        let entry_size = read_u64(bytes, offset + 0x38)? as usize;
        sections.push(SectionHeader {
            name_offset,
            offset: section_offset,
            size,
            entry_size,
        });
    }
    Ok(sections)
}

fn section_data<'a>(
    bytes: &'a [u8],
    sections: &[SectionHeader],
    index: usize,
) -> Result<&'a [u8], String> {
    let section = sections
        .get(index)
        .ok_or_else(|| format!("invalid section index {index}"))?;
    let end = section
        .offset
        .checked_add(section.size)
        .ok_or_else(|| "section size overflow".to_owned())?;
    bytes
        .get(section.offset..end)
        .ok_or_else(|| "section out of bounds".to_owned())
}

fn find_section(sections: &[SectionHeader], shstr: &[u8], name: &str) -> Result<usize, String> {
    find_optional_section(sections, shstr, name).ok_or_else(|| format!("missing section {name}"))
}

fn find_optional_section(sections: &[SectionHeader], shstr: &[u8], name: &str) -> Option<usize> {
    sections.iter().position(|section| {
        read_c_string(shstr, section.name_offset)
            .map(|candidate| candidate == name)
            .unwrap_or(false)
    })
}

fn read_c_string<'a>(bytes: &'a [u8], offset: usize) -> Result<&'a str, String> {
    if offset >= bytes.len() {
        return Err("string offset out of bounds".to_owned());
    }
    let tail = &bytes[offset..];
    let end = tail
        .iter()
        .position(|byte| *byte == 0)
        .ok_or_else(|| "unterminated string".to_owned())?;
    std::str::from_utf8(&tail[..end]).map_err(|err| err.to_string())
}

fn read_u16(bytes: &[u8], offset: usize) -> Result<u16, String> {
    let end = offset
        .checked_add(2)
        .ok_or_else(|| "u16 offset overflow".to_owned())?;
    let slice = bytes
        .get(offset..end)
        .ok_or_else(|| "u16 out of bounds".to_owned())?;
    Ok(u16::from_le_bytes([slice[0], slice[1]]))
}

fn read_u32(bytes: &[u8], offset: usize) -> Result<u32, String> {
    let end = offset
        .checked_add(4)
        .ok_or_else(|| "u32 offset overflow".to_owned())?;
    let slice = bytes
        .get(offset..end)
        .ok_or_else(|| "u32 out of bounds".to_owned())?;
    Ok(u32::from_le_bytes([slice[0], slice[1], slice[2], slice[3]]))
}

fn read_u64(bytes: &[u8], offset: usize) -> Result<u64, String> {
    let end = offset
        .checked_add(8)
        .ok_or_else(|| "u64 offset overflow".to_owned())?;
    let slice = bytes
        .get(offset..end)
        .ok_or_else(|| "u64 out of bounds".to_owned())?;
    Ok(u64::from_le_bytes([
        slice[0], slice[1], slice[2], slice[3], slice[4], slice[5], slice[6], slice[7],
    ]))
}

fn write_u16(bytes: &mut [u8], offset: usize, value: u16) -> Result<(), String> {
    let end = offset
        .checked_add(2)
        .ok_or_else(|| "u16 offset overflow".to_owned())?;
    let slice = bytes
        .get_mut(offset..end)
        .ok_or_else(|| "u16 out of bounds".to_owned())?;
    slice.copy_from_slice(&value.to_le_bytes());
    Ok(())
}

fn write_u64(bytes: &mut [u8], offset: usize, value: u64) -> Result<(), String> {
    let end = offset
        .checked_add(8)
        .ok_or_else(|| "u64 offset overflow".to_owned())?;
    let slice = bytes
        .get_mut(offset..end)
        .ok_or_else(|| "u64 out of bounds".to_owned())?;
    slice.copy_from_slice(&value.to_le_bytes());
    Ok(())
}
