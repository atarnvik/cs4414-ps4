/* kernel::filesystem.rs */
use core::*;
use core::str::*;
use core::option::{Some, Option, None}; // Match statement
use core::iter::Iterator;
//use kernel::vec::Vec;
use super::super::platform::*;
use kernel::sgash;

pub struct node {
    name: sgash::cstr,
    parent: Option<*node>,
    contents: sgash::cstr,
    isDir: bool,
    dchildren: Option<*mut node>,
    fchildren : Option<*mut FileNode>,
}
 
impl node {
    pub unsafe fn new(file_name: sgash::cstr, dad: Option<*node>, words: sgash::cstr, dir: bool) -> node {
        let this = node {
            name: file_name,
            parent: dad,
            contents: words,
            isDir: dir,
            dchildren: None,
            fchildren : None,
        };
        this
    }
 
    // pub unsafe fn add_node(&mut self, d : &mut node) { 
    //    (*self.children).push(*d);
    // }
}
 
pub struct FileNode {
    name: sgash::cstr,
    parent: Option<*node>,
    contents: sgash::cstr,
    next: Option<*mut FileNode>,
}

impl FileNode {
    pub unsafe fn new (file_name: sgash::cstr, dad: Option<*node>, words: sgash::cstr) -> FileNode {
        let this = FileNode {
            name: file_name,
            parent: dad,
            contents: words,
            next : None,
        };
        this
    }
}

// cat <file>
// pub fn read_file(file){}

// // wr
// pub fn write_file(file, string){}

// // touch
// pub fn create_file(directory, name){}

// // rm
// pub fn delete_file(directory, name){}

// // ???
// pub fn get_file(directory, name){}

// // ls
// pub fn list_directory(directory){}

// // mkdir
// pub fn create_directory(parent, name){}

// // rm
// pub fn delete_directory(directory){}

// // pwd
// pub fn get_directory(parent, name){}


