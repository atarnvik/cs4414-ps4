/* kernel::filesystem.rs */
use core::*;
use core::str::*;
use core::option::{Some, Option, None}; // Match statement
use core::iter::Iterator;
use kernel::vec::Vec;
use super::super::platform::*;
use kernel::sgash;

pub struct DirNode {
    name: sgash::cstr,
    parent: *mut DirNode,
    dchildren: *mut Vec<*mut DirNode>,
    fchildren : *mut Vec<*mut FileNode>,
}
 
impl DirNode {
    pub unsafe fn new(file_name: sgash::cstr, dad: *mut DirNode) -> DirNode {
        let this = DirNode {
            name: file_name,
            parent: dad,

            dchildren: &mut Vec::new(),
            fchildren : &mut Vec::new(),
        };
        this
    }

    // pub unsafe fn list_directory (&mut self, dir : sgash::cstr) {
    //     if(dir.equals(self.name)) {

    //     }
    // }
 
    pub unsafe fn add_dir(&mut self, d : sgash::cstr) {
        let newNode : *mut DirNode = &mut DirNode::new(d, self); 
       (*self.dchildren).push(newNode);
    }

    pub unsafe fn add_file(&mut self, d : sgash::cstr, contents : sgash::cstr) {
        let newNode : *mut FileNode = &mut FileNode::new(d, self, contents); 
       (*self.fchildren).push(newNode);
    }
}
 
pub struct FileNode {
    name: sgash::cstr,
    parent: *mut DirNode,
    contents: sgash::cstr,
}

impl FileNode {
    pub unsafe fn new (file_name: sgash::cstr, dad: *mut DirNode, words: sgash::cstr) -> FileNode {
        let this = FileNode {
            name: file_name,
            parent: dad,
            contents: words,
        };
        this
    }

    pub unsafe fn getName(&self) -> sgash::cstr {
        self.name
    }

    //cat <file>
    pub fn read_file(&self) -> sgash::cstr {
        self.contents
    }

    // wr
    pub fn write_file(&self, string : sgash::cstr) {

    }
}



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


