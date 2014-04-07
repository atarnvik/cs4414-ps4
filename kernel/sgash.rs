/* kernel::sgash.rs */
#[allow(unused_imports)];

use core::*;
use core::str::*;
use core::option::{Some, Option, None}; 
use core::iter::Iterator;
use kernel::*;
use kernel::vec::Vec;
use super::super::platform::*;
use kernel::memory::Allocator;
use kernel::memory::BuddyAlloc;
use kernel::memory::Bitv;
use kernel::memory::BitvStorage;
use kernel::memory::Alloc;
use kernel::memory;
use kernel::fs;
use kernel::fs::FileNode;
use kernel::fs::DirNode;

pub static mut buffer: cstr = cstr {
	p: 0 as *mut u8,
	p_cstr_i: 0,
	max: 0
};

pub static mut s: cstr = cstr {
	p: 0 as *mut u8,
	p_cstr_i: 0,
	max: 256
};

pub static mut numberString: cstr = cstr {
    p: 0 as *mut u8,
    p_cstr_i: 0,
    max: 256
};

pub static mut count: uint = 0;

pub static mut root : DirNode = DirNode {
    name : cstr {
        p: 0 as *mut u8,
        p_cstr_i: 0,
        max: 256
    },
    dchildren : '\0' as *mut Vec<*mut DirNode>,
    fchildren : '\0' as *mut Vec<*mut FileNode>,
    parent : '\0' as *mut DirNode,
};
pub static mut pwd : *mut DirNode = '\0' as *mut DirNode;

pub fn putchar(key: char) {
    unsafe {
	/*
	 * We need to include a blank asm call to prevent rustc
	 * from optimizing this part out
	 */
	asm!("");
	io::write_char(key, io::UART0);
    }
}

pub fn putstr(msg: &str) {
    for c in slice::iter(as_bytes(msg)) {
	   putchar(*c as char);
    }	
}

pub unsafe fn drawstr(msg: &str) {
    let old_fg = super::super::io::FG_COLOR;
    let mut x: u32 = 0x6699AAFF;
    for c in slice::iter(as_bytes(msg)) {
    	x = (x << 8) + (x >> 24); 
    	super::super::io::set_fg(x);
    	drawchar(*c as char);
    }
    super::super::io::set_fg(old_fg);
}

unsafe fn drawchar(x: char)
{
    io::restore();
    if x == '\n' {
    	io::CURSOR_Y += io::CURSOR_HEIGHT;
    	io::CURSOR_X = 0u32;
    } 
    else {
    	io::draw_char(x);	
    	io::CURSOR_X += io::CURSOR_WIDTH;
    }
    io::backup();
    io::draw_cursor();
}

unsafe fn backspace()
{
    io::restore();
    if (io::CURSOR_X >= io::CURSOR_WIDTH) { 
    	io::CURSOR_X -= io::CURSOR_WIDTH;
    	io::draw_char(' ');
    }
    io::backup();
    io::draw_cursor();
}

pub unsafe fn parsekey(x: char) {
    let x = x as u8;
    // Set this to false to learn the keycodes of various keys!
    // Key codes are printed backwards because life is hard

    match x { 
	13		=>	{
		parse();
		putstr(&"\nsgash> ");
		drawstr(&"\nsgash> ");
        buffer.reset();
	}
	127		=>	{ 
	    putchar('');
	    putchar(' ');
	    putchar(''); 
	    backspace();
        buffer.delete_char();

	}
	_		=>	{ 
	    if io::CURSOR_X < io::SCREEN_WIDTH-io::CURSOR_WIDTH  && buffer.add_char(x) {
    		putchar(x as char);
    		drawchar(x as char);
	    }
	}
    }
}

unsafe fn parse(){
	// cd, rm, mkdir, pwd
	match buffer.getarg(' ', 0) {
	    Some(a) => {
	    	if(a.equals(&"echo")) {
                match buffer.getarg(' ', 1) {
                    Some(z) => {
                        drawchar('\n');
                        drawcstr(z);
                    }
                    None => {}
                }
			}
			if(a.equals(&"ls")) {
			    putstr(&"\nfile list");
			    drawstr(&"\nfile list");
			}
            if(a.equals(&"cat")) {
                putstr(&"\n");
                drawstr(&"\n");
                match buffer.getarg(' ', 1){
                    Some(b) =>{
                        (*pwd).read_file(b);
                    }
                    None => {}
                };
            }
            if(a.equals(&"cd")) {
                putstr(&"\nchange directory");
                drawstr(&"\nhange directory");
            }
            if(a.equals(&"rm")) {
                putstr(&"\nremove");
                drawstr(&"\nremove");
            }
			if(a.equals(&"mkdir")) {
                putstr(&"\n");
                drawstr(&"\n");
				match buffer.getarg(' ', 1){
					Some(b) =>{
						putcstr(b);
						drawcstr(b);
					}
					None => {}
				};
			}
            if(a.equals(&"pwd")) {
                // putcstr((*pwd).name);
                // drawcstr((*pwd).name);
                putstr(&"\nYou are here");
                drawstr(&"\nYou are here");
            }
            if(a.equals(&"wr")) {
                putstr(&"\nwrite file");
                drawstr(&"\nwrite file");
            }
            if(a.equals(&"highlight")) {
            	match buffer.getarg(' ', 1){
					Some(b) =>{
						if(b.equals("on")){
							io::set_bg(0x33ffff);
						}
						if(b.equals("off")){
							io::set_bg(0x660000);
						}
					}
					None => {}
				};
            }
            if(a.equals(&"clear")){
            	io::restart();
            }
	    }
	    None => { }
	};
	buffer.reset();
}

fn screen() {
    putstr(&"\n                                                               "); 
    putstr(&"\n                                                               ");
    putstr(&"\n                       7=..~$=..:7                             "); 
    putstr(&"\n                  +$: =$$$+$$$?$$$+ ,7?                        "); 
    putstr(&"\n                  $$$$$$$$$$$$$$$$$$Z$$                        ");
    putstr(&"\n              7$$$$$$$$$$$$. .Z$$$$$Z$$$$$$                    ");
    putstr(&"\n           ~..7$$Z$$$$$7+7$+.?Z7=7$$Z$$Z$$$..:                 ");
    putstr(&"\n          ~$$$$$$$$7:     :ZZZ,     :7ZZZZ$$$$=                ");
    putstr(&"\n           Z$$$$$?                    .+ZZZZ$$                 ");
    putstr(&"\n       +$ZZ$$$Z7                         7ZZZ$Z$$I.            "); 
    putstr(&"\n        $$$$ZZZZZZZZZZZZZZZZZZZZZZZZI,    ,ZZZ$$Z              "); 
    putstr(&"\n      :+$$$$ZZZZZZZZZZZZZZZZZZZZZZZZZZZ=    $ZZ$$+~,           "); 
    putstr(&"\n     ?$Z$$$$ZZZZZZZZZZZZZZZZZZZZZZZZZZZZI   7ZZZ$ZZI           "); 
    putstr(&"\n      =Z$$+7Z$$7ZZZZZZZZ$$$$$$$ZZZZZZZZZZ  ~Z$?$ZZ?            ");	 
    putstr(&"\n    :$Z$Z...$Z  $ZZZZZZZ~       ~ZZZZZZZZ,.ZZ...Z$Z$~          "); 
    putstr(&"\n    7ZZZZZI$ZZ  $ZZZZZZZ~       =ZZZZZZZ7..ZZ$?$ZZZZ$          "); 
    putstr(&"\n      ZZZZ$:    $ZZZZZZZZZZZZZZZZZZZZZZ=     ~$ZZZ$:           "); 
    putstr(&"\n    7Z$ZZ$,     $ZZZZZZZZZZZZZZZZZZZZ7         ZZZ$Z$          "); 
    putstr(&"\n   =ZZZZZZ,     $ZZZZZZZZZZZZZZZZZZZZZZ,       ZZZ$ZZ+         "); 
    putstr(&"\n     ,ZZZZ,     $ZZZZZZZ:     =ZZZZZZZZZ     ZZZZZ$:           "); 
    putstr(&"\n    =$ZZZZ+     ZZZZZZZZ~       ZZZZZZZZ~   =ZZZZZZZI          "); 
    putstr(&"\n    $ZZ$ZZZ$$Z$$ZZZZZZZZZ$$$$   IZZZZZZZZZ$ZZZZZZZZZ$          "); 
    putstr(&"\n      :ZZZZZZZZZZZZZZZZZZZZZZ   ~ZZZZZZZZZZZZZZZZZ~            "); 
    putstr(&"\n     ,Z$$ZZZZZZZZZZZZZZZZZZZZ    ZZZZZZZZZZZZZZZZZZ~           "); 
    putstr(&"\n     =$ZZZZZZZZZZZZZZZZZZZZZZ     $ZZZZZZZZZZZZZZZ$+           "); 
    putstr(&"\n        IZZZZZ:.                        . ,ZZZZZ$              "); 
    putstr(&"\n       ~$ZZZZZZZZZZZ                 ZZZZ$ZZZZZZZ+             "); 
    putstr(&"\n           Z$ZZZ. ,Z~               =Z:.,ZZZ$Z                 "); 
    putstr(&"\n          ,ZZZZZ..~Z$.             .7Z:..ZZZZZ:                ");
    putstr(&"\n          ~7+:$ZZZZZZZZI=:.   .,=IZZZZZZZ$Z:=7=                ");
    putstr(&"\n              $$ZZZZZZZZZZZZZZZZZZZZZZ$ZZZZ                    ");
    putstr(&"\n              ==..$ZZZ$ZZZZZZZZZZZ$ZZZZ .~+                    ");
    putstr(&"\n                  I$?.?ZZZ$ZZZ$ZZZI =$7                        ");
    putstr(&"\n                       $7..I$7..I$,                            ");
    putstr(&"\n"); 
    putstr(&"\n _                     _     _                         _  ");
    putstr(&"\n| |                   (_)   | |                       | | ");
    putstr(&"\n| | ____ ___  ____     _____| |_____  ____ ____  _____| | ");
    putstr(&"\n| |/ ___) _ \\|  _ \\   |  _   _) ___ |/ ___)  _ \\| ___ | | ");
    putstr(&"\n| | |  | |_| | | | |  | |  \\ \\| ____| |   | | | | ____| | ");
    putstr(&"\n|_|_|  \\____/|_| |_|  |_|   \\_\\_____)_|   |_| |_|_____)__)\n\n");
}

pub unsafe fn init() {
	buffer = cstr::new(256);
    screen();
   	putstr(&"\nsgash> ");

    root = fs::DirNode::new(from_str("Root"), '\0' as *mut DirNode);
    pwd = &mut root as *mut DirNode;
    (*pwd).name = from_str("Root");

	buffer.reset();
}

pub unsafe fn putcstr(s: cstr) {
    let mut p = s.p as uint;
    while *(p as *char) != '\0'
    {
        putchar(*(p as *char));
        p += 1;
    }
}

pub unsafe fn drawcstr(string : cstr) -> bool{
    let s = string.p as uint;
    let e = string.max;
    let mut i = 0;
    while i < e {
        let theChar : u8 = *((s+i) as *mut u8);
        if(theChar as char != '\0') {
            drawchar(theChar as char);
            i +=1;
        }
        else {
            return true;
        }
    }
    false
}

pub unsafe fn echo() -> bool{
    drawstr(&"\n");
    putstr(&"\n");
    let s = buffer.p as uint;
    let e = buffer.max;
    let mut i = 0;
    while i < e {
        let theChar : u8 = *((s+i) as *mut u8);
        if(theChar as char != '\0') {
            putchar(theChar as char);
            drawchar(theChar as char);
            i +=1;
        }
        else {
            drawstr(&"\n");
            putstr(&"\n");
            return true;
        }
    }
    false
}

pub unsafe fn from_str(s: &str) -> cstr {
    let mut this = cstr::new(256);
    for c in slice::iter(as_bytes(s)) {
        this.add_char(*c);
    };
    this
}

pub struct cstr {
    p: *mut u8,
    p_cstr_i: uint,
    max: uint 
}

impl cstr {

    pub unsafe fn new(size: uint) -> cstr {
        let (x,y) = heap.alloc(size);

        let temp = ((x as uint) + 256 * count) as *mut u8;

        count = count + 1;

        let this = cstr {
            p: temp,
            p_cstr_i: 0,
            max: y
        };
        *(((this.p as uint)+this.p_cstr_i) as *mut char) = '\0';
        this
    }

    fn len(&self) -> uint { 
        self.p_cstr_i
    }

    pub unsafe fn add_char(&mut self, x: u8) -> bool{
        if (self.p_cstr_i == self.max) { 
            putstr("not able to add");
            return false; 
        }
        *(((self.p as uint)+self.p_cstr_i) as *mut u8) = x;
        self.p_cstr_i += 1;
        *(((self.p as uint)+self.p_cstr_i) as *mut char) = '\0';
        true
    }

    unsafe fn delete_char(&mut self) -> bool {
        if (self.p_cstr_i == 0) { return false; }
        self.p_cstr_i -= 1;
        *(((self.p as uint)+self.p_cstr_i) as *mut char) = '\0';
        true
    }

    unsafe fn reset(&mut self) {
        self.p_cstr_i = 0; 
        *(self.p as *mut char) = '\0';
    }

    unsafe fn charAt(&self, n: u8) -> char {
        ((*self.p)  + n) as char
    }

    unsafe fn equals(&self, other: &str) -> bool {
    	// save val of self.p, which is u8, as a unit
    	let mut selfp: uint = self.p as uint;
    	// iterate through the str "other"
    	for c in slice::iter(as_bytes(other)){
    		// return false if any character does not match
    		if( *c != *(selfp as *u8) ) { 
                return false; 
            }
    		selfp += 1;
    	};
    	true
    }

    pub unsafe fn equals_cstr(&self, other: cstr) -> bool {
        let mut x: uint = 0;
        let mut selfp: uint = self.p as uint;
        let mut otherp: uint = other.p as uint;
        while x < self.len() {
            if (*(selfp as *char) != *(otherp as *char)) { 
               return false;
            }
            selfp += 1;
            otherp += 1;
            x += 1;
        }
        true
    }

	unsafe fn getarg(&self, delim: char, mut k: uint) -> Option<cstr> {
		let mut ind: uint = 0;
		let mut found = k == 0;
		let mut selfp: uint = self.p as uint;
        s.reset();
		loop {
			if (*(selfp as *char) == '\0') { 
				// End of string
                //return a copy of s (write copy method)
                // erased from next call
				if (found) { return Some(s); }
				else { return None; }
			};
			if (*(selfp as *u8) == delim as u8) { 
				if (found) { return Some(s); }
				k -= 1;
			};
			if (found) {
				s.add_char(*(selfp as *u8));
			};
			found = k == 0;
			selfp += 1;
			ind += 1;
			if (ind == self.max) { 
				putstr(&"\nSomething broke!");
				return None; 
			}
		}
	}
}