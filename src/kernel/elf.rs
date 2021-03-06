/* automatically generated by rust-bindgen */
/* This file defines standard ELF types, structures, and macros.
   Copyright (C) 1995, 1996, 1997, 1998, 1999 Free Software Foundation, Inc.
   This file is part of the GNU C Library.
   Contributed by Ian Lance Taylor <ian@cygnus.com>.
 
   The GNU C Library is free software; you can redistribute it and/or
   modify it under the terms of the GNU Library General Public License as
   published by the Free Software Foundation; either version 2 of the
   License, or (at your option) any later version.
 
   The GNU C Library is distributed in the hope that it will be useful,
   but WITHOUT ANY WARRANTY; without even the implied warranty of
   MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
   Library General Public License for more details.
 
   You should have received a copy of the GNU Library General Public
   License along with the GNU C Library; see the file COPYING.LIB.  If not,
   write to the Free Software Foundation, Inc., 59 Temple Place - Suite 330,
   Boston, MA 02111-1307, USA.  */

use core;

pub type uint16_t = u16;
pub type uint32_t = u32;
pub type int32_t = i32;
pub type uint64_t = u64;
pub type int64_t = i64;
pub type c_uchar = u8;
pub type c_int = i32;
pub type c_long = u64;
pub type c_void = uint;
pub type Elf32_Half = uint16_t;
pub type Elf64_Half = uint16_t;
pub type Elf32_Word = uint32_t;
pub type Elf32_Sword = int32_t;
pub type Elf64_Word = uint32_t;
pub type Elf64_Sword = int32_t;
pub type Elf32_Xword = uint64_t;
pub type Elf32_Sxword = int64_t;
pub type Elf64_Xword = uint64_t;
pub type Elf64_Sxword = int64_t;
pub type Elf32_Addr = uint32_t;
pub type Elf64_Addr = uint64_t;
pub type Elf32_Off = uint32_t;
pub type Elf64_Off = uint64_t;
pub type Elf32_Section = uint16_t;
pub type Elf64_Section = uint16_t;
pub type Elf32_Symndx = uint32_t;
pub type Elf64_Symndx = uint64_t;
#[packed]
pub struct Elf32_Ehdr {
    e_ident: [c_uchar, ..16u],
    e_type: Elf32_Half,
    e_machine: Elf32_Half,
    e_version: Elf32_Word,
    e_entry: Elf32_Addr,
    e_phoff: Elf32_Off,
    e_shoff: Elf32_Off,
    e_flags: Elf32_Word,
    e_ehsize: Elf32_Half,
    e_phentsize: Elf32_Half,
    e_phnum: Elf32_Half,
    e_shentsize: Elf32_Half,
    e_shnum: Elf32_Half,
    e_shstrndx: Elf32_Half,
}
pub struct Elf64_Ehdr {
    e_ident: [c_uchar, ..16u],
    e_type: Elf64_Half,
    e_machine: Elf64_Half,
    e_version: Elf64_Word,
    e_entry: Elf64_Addr,
    e_phoff: Elf64_Off,
    e_shoff: Elf64_Off,
    e_flags: Elf64_Word,
    e_ehsize: Elf64_Half,
    e_phentsize: Elf64_Half,
    e_phnum: Elf64_Half,
    e_shentsize: Elf64_Half,
    e_shnum: Elf64_Half,
    e_shstrndx: Elf64_Half,
}
pub struct Elf32_Shdr {
    sh_name: Elf32_Word,
    sh_type: Elf32_Word,
    sh_flags: Elf32_Word,
    sh_addr: Elf32_Addr,
    sh_offset: Elf32_Off,
    sh_size: Elf32_Word,
    sh_link: Elf32_Word,
    sh_info: Elf32_Word,
    sh_addralign: Elf32_Word,
    sh_entsize: Elf32_Word,
}
pub struct Elf64_Shdr {
    sh_name: Elf64_Word,
    sh_type: Elf64_Word,
    sh_flags: Elf64_Xword,
    sh_addr: Elf64_Addr,
    sh_offset: Elf64_Off,
    sh_size: Elf64_Xword,
    sh_link: Elf64_Word,
    sh_info: Elf64_Word,
    sh_addralign: Elf64_Xword,
    sh_entsize: Elf64_Xword,
}
pub struct Elf32_Sym {
    st_name: Elf32_Word,
    st_value: Elf32_Addr,
    st_size: Elf32_Word,
    st_info: c_uchar,
    st_other: c_uchar,
    st_shndx: Elf32_Section,
}
pub struct Elf64_Sym {
    st_name: Elf64_Word,
    st_info: c_uchar,
    st_other: c_uchar,
    st_shndx: Elf64_Section,
    st_value: Elf64_Addr,
    st_size: Elf64_Xword,
}
pub struct Elf32_Syminfo {
    si_boundto: Elf32_Half,
    si_flags: Elf32_Half,
}
pub struct Elf64_Syminfo {
    si_boundto: Elf64_Half,
    si_flags: Elf64_Half,
}
pub struct Elf32_Rel {
    r_offset: Elf32_Addr,
    r_info: Elf32_Word,
}
pub struct Elf64_Rel {
    r_offset: Elf64_Addr,
    r_info: Elf64_Xword,
}
pub struct Elf32_Rela {
    r_offset: Elf32_Addr,
    r_info: Elf32_Word,
    r_addend: Elf32_Sword,
}
pub struct Elf64_Rela {
    r_offset: Elf64_Addr,
    r_info: Elf64_Xword,
    r_addend: Elf64_Sxword,
}
#[packed]
pub struct Elf32_Phdr {
    p_type: Elf32_Word,
    p_offset: Elf32_Off,
    p_vaddr: Elf32_Addr,
    p_paddr: Elf32_Addr,
    p_filesz: Elf32_Word,
    p_memsz: Elf32_Word,
    p_flags: Elf32_Word,
    p_align: Elf32_Word,
}
pub struct Elf64_Phdr {
    p_type: Elf64_Word,
    p_flags: Elf64_Word,
    p_offset: Elf64_Off,
    p_vaddr: Elf64_Addr,
    p_paddr: Elf64_Addr,
    p_filesz: Elf64_Xword,
    p_memsz: Elf64_Xword,
    p_align: Elf64_Xword,
}
pub struct Union_Unnamed1 {
    data: [c_uchar, ..4u],
}
impl Union_Unnamed1 {
    pub fn d_val(&mut self) -> *mut Elf32_Word {
        unsafe { core::mem::transmute(self) }
    }
    pub fn d_ptr(&mut self) -> *mut Elf32_Addr {
        unsafe { core::mem::transmute(self) }
    }
}
pub struct Elf32_Dyn {
    d_tag: Elf32_Sword,
    d_un: Union_Unnamed1,
}
pub struct Union_Unnamed2 {
    data: [c_uchar, ..8u],
}
impl Union_Unnamed2 {
    pub fn d_val(&mut self) -> *mut Elf64_Xword {
        unsafe { core::mem::transmute(self) }
    }
    pub fn d_ptr(&mut self) -> *mut Elf64_Addr {
        unsafe { core::mem::transmute(self) }
    }
}
pub struct Elf64_Dyn {
    d_tag: Elf64_Sxword,
    d_un: Union_Unnamed2,
}
pub struct Elf32_Verdef {
    vd_version: Elf32_Half,
    vd_flags: Elf32_Half,
    vd_ndx: Elf32_Half,
    vd_cnt: Elf32_Half,
    vd_hash: Elf32_Word,
    vd_aux: Elf32_Word,
    vd_next: Elf32_Word,
}
pub struct Elf64_Verdef {
    vd_version: Elf64_Half,
    vd_flags: Elf64_Half,
    vd_ndx: Elf64_Half,
    vd_cnt: Elf64_Half,
    vd_hash: Elf64_Word,
    vd_aux: Elf64_Word,
    vd_next: Elf64_Word,
}
pub struct Elf32_Verdaux {
    vda_name: Elf32_Word,
    vda_next: Elf32_Word,
}
pub struct Elf64_Verdaux {
    vda_name: Elf64_Word,
    vda_next: Elf64_Word,
}
pub struct Elf32_Verneed {
    vn_version: Elf32_Half,
    vn_cnt: Elf32_Half,
    vn_file: Elf32_Word,
    vn_aux: Elf32_Word,
    vn_next: Elf32_Word,
}
pub struct Elf64_Verneed {
    vn_version: Elf64_Half,
    vn_cnt: Elf64_Half,
    vn_file: Elf64_Word,
    vn_aux: Elf64_Word,
    vn_next: Elf64_Word,
}
pub struct Elf32_Vernaux {
    vna_hash: Elf32_Word,
    vna_flags: Elf32_Half,
    vna_other: Elf32_Half,
    vna_name: Elf32_Word,
    vna_next: Elf32_Word,
}
pub struct Elf64_Vernaux {
    vna_hash: Elf64_Word,
    vna_flags: Elf64_Half,
    vna_other: Elf64_Half,
    vna_name: Elf64_Word,
    vna_next: Elf64_Word,
}
pub struct Union_Unnamed3 {
    data: [c_uchar, ..8u],
}
impl Union_Unnamed3 {
    pub fn a_val(&mut self) -> *mut c_long {
        unsafe { core::mem::transmute(self) }
    }
    pub fn a_ptr(&mut self) -> *mut *mut c_void {
        unsafe { core::mem::transmute(self) }
    }
    pub fn a_fcn(&mut self) -> *mut extern "C" fn() {
        unsafe { core::mem::transmute(self) }
    }
}
pub struct Elf32_auxv_t {
    a_type: c_int,
    a_un: Union_Unnamed3,
}
pub struct Union_Unnamed4 {
    data: [c_uchar, ..8u],
}
impl Union_Unnamed4 {
    pub fn a_val(&mut self) -> *mut c_long {
        unsafe { core::mem::transmute(self) }
    }
    pub fn a_ptr(&mut self) -> *mut *mut c_void {
        unsafe { core::mem::transmute(self) }
    }
    pub fn a_fcn(&mut self) -> *mut extern "C" fn() {
        unsafe { core::mem::transmute(self) }
    }
}
pub struct Elf64_auxv_t {
    a_type: c_long,
    a_un: Union_Unnamed4,
}
pub struct Elf32_Nhdr {
    n_namesz: Elf32_Word,
    n_descsz: Elf32_Word,
    n_type: Elf32_Word,
}
pub struct Elf64_Nhdr {
    n_namesz: Elf64_Word,
    n_descsz: Elf64_Word,
    n_type: Elf64_Word,
}
pub struct Struct_Unnamed5 {
    gt_current_g_value: Elf32_Word,
    gt_unused: Elf32_Word,
}
pub struct Struct_Unnamed6 {
    gt_g_value: Elf32_Word,
    gt_bytes: Elf32_Word,
}
pub struct Elf32_gptab {
    data: [c_uchar, ..8u],
}
impl Elf32_gptab {
    pub fn gt_header(&mut self) -> *mut Struct_Unnamed5 {
        unsafe { core::mem::transmute(self) }
    }
    pub fn gt_entry(&mut self) -> *mut Struct_Unnamed6 {
        unsafe { core::mem::transmute(self) }
    }
}
pub struct Elf32_RegInfo {
    ri_gprmask: Elf32_Word,
    ri_cprmask: [Elf32_Word, ..4u],
    ri_gp_value: Elf32_Sword,
}
pub struct Elf_Options {
    kind: c_uchar,
    size: c_uchar,
    section: Elf32_Section,
    info: Elf32_Word,
}
pub struct Elf_Options_Hw {
    hwp_flags1: Elf32_Word,
    hwp_flags2: Elf32_Word,
}
pub struct Elf32_Lib {
    l_name: Elf32_Word,
    l_time_stamp: Elf32_Word,
    l_checksum: Elf32_Word,
    l_version: Elf32_Word,
    l_flags: Elf32_Word,
}
pub struct Elf64_Lib {
    l_name: Elf64_Word,
    l_time_stamp: Elf64_Word,
    l_checksum: Elf64_Word,
    l_version: Elf64_Word,
    l_flags: Elf64_Word,
}
pub type Elf32_Conflict = Elf32_Addr;
extern "C" { }
