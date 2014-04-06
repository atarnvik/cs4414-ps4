/* kernel::filesystem.rs */
use core::*;
use core::str::*;
use core::option::{Some, Option, None}; // Match statement
use core::iter::Iterator;
use kernel::vec::Vec;
use super::super::platform::*;
use kernel::sgash::cstr;
use kernel::sgash::drawcstr;
use kernel::sgash::putcstr;
use core::iter::Iterator;
use core::slice::iter;

pub struct DirNode {
    name: cstr,
    parent: *mut DirNode,
    dchildren: *mut Vec<*mut DirNode>,
    fchildren : *mut Vec<*mut FileNode>,
}
 
impl DirNode {
    pub unsafe fn new(file_name: cstr, dad: *mut DirNode) -> DirNode {
        let this = DirNode {
            name: file_name,
            parent: dad,

            dchildren: &mut Vec::new() as *mut Vec<*mut DirNode>,
            fchildren : &mut Vec::new() as *mut Vec<*mut FileNode>,
        };
        this
    }

    pub unsafe fn list_directory (&mut self){
        for d in iter((self.dchildren).as_mut_slice()){
        	putcstr(d.name);
        	drawcstr(d.name);
        }

    }
 
    pub unsafe fn create_directory(&mut self, d : cstr) {
        let newNode : *mut DirNode = &mut DirNode::new(d, self); 
       (*self.dchildren).push(newNode);
    }

    pub unsafe fn create_file(&mut self, d : cstr) {
    	let empty = cstr::new(0);
        let newNode : *mut FileNode = &mut FileNode::new(d, self, empty); 
       (*self.fchildren).push(newNode);
    }

    pub unsafe fn write_file(&mut self, d : cstr, contents : cstr) {
        let newNode : *mut FileNode = &mut FileNode::new(d, self, contents); 
       (*self.fchildren).push(newNode);
    }

    // pub unsafe fn get_file(&mut self, name : cstr){

    // }
}
 
pub struct FileNode {
    name: cstr,
    parent: *mut DirNode,
    contents: cstr,
}

impl FileNode {
    pub unsafe fn new (file_name: cstr, dad: *mut DirNode, words: cstr) -> FileNode {
        let this = FileNode {
            name: file_name,
            parent: dad,
            contents: words,
        };
        this
    }

    pub unsafe fn getName(&self) -> cstr {
        self.name
    }

    //cat <file>
    pub fn read_file(&self) -> cstr {
        self.contents
    }
}

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


