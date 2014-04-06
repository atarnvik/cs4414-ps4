/* kernel::filesystem.rs */
use core::*;
use core::str::*;
use core::option::{Some, Option, None}; // Match statement
use core::iter::Iterator;
use kernel::vec::Vec;
use super::super::platform::*;

pub struct node {
    name: cstr,
    parent: *node,
    contents: cstr,
    isDir: bool,
    children: *Vec<node>,
}
 
impl node {
    pub unsafe fn new(file_name: cstr, dad: *node, words: cstr, dir: bool) -> node {
        let this = node {
            name: file_name,
            parent: dad,
            contents: words,
            isDir: dir,
            children: &Vec::new() as *Vec<node>,
        };
        this
    }
 
    pub unsafe fn add_node(&mut self, d : &mut node) { 
       (*self.children).push(*d);
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


