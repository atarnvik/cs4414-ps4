/* kernel::sgash.rs */
#[allow(unused_imports)];
use std::path::*;
use core::*;
use core::str::*;
use core::option::{Some, Option, None}; 
use core::iter::Iterator;
use kernel::*;
use super::super::platform::*;
use kernel::memory::Allocator;
use kernel::memory::BuddyAlloc;
use kernel::memory::Bitv;
use kernel::memory::BitvStorage;
use kernel::memory::Alloc;

pub static mut buffer: cstr = cstr {
				p: 0 as *mut u8,
				p_cstr_i: 0,
				max: 0
			      };

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

fn putstr(msg: &str) {
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
	if(buffer.equals(&"freeze")){
		putstr(&"\nTHIS IS A STICK UP!!");
		drawstr(&"\nTHIS IS A STICK UP!!");
	};
	// cd, rm, mkdir, pwd
	// match buffer.getarg(' ', 0) {
	//     Some(a) => {
	//     	if(a.equals(&"echo")) {
	//     		echo();
	// 		}
	// 		if(a.equals(&"ls")) {
	// 		    putstr(&"\nfile list");
	// 		    drawstr(&"\nfile list");
	// 		}
	// 		if(a.equals(&"pwd")) {
	// 		    putstr(&"\nmy directory");
	// 		    drawstr(&"\nmy directory");
	// 		}
	// 		if(a.equals(&"mkdir")) {
	// 			match buffer.getarg(' ', 1){
	// 				Some(b) =>{
	// 					putstr(&"");
	// 					drawstr(&a);
	// 				}
	// 				None => {}
	// 			};
	// 		}
	// 		f(a.equals(&"cat")) {
	// 			match buffer.getarg(' ', 1){
	// 				Some(b) =>{
	// 					putstr(&a);
	// 					drawstr(&a);
	// 				}
	// 				None => {}
	// 			};
	// 		}
	//     }
	//     None => { }
	// };
	// buffer.reset();
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
	//drawstr(&"\nsgash> ");
	buffer.reset();
}

pub unsafe fn echo() -> bool{
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
    return false;
}



struct cstr {
    p: *mut u8,
    p_cstr_i: uint,
    max: uint 
}

impl cstr {

    pub unsafe fn new(size: uint) -> cstr {
        // Sometimes this doesn't allocate enough memory and gets stuck..

        let (x,y) = heap.alloc(size);

        let this = cstr {
            p: x,
            p_cstr_i: 0,
            max: y
        };
        *(((this.p as uint)+this.p_cstr_i) as *mut char) = '\0';
        this
    }

    #[allow(dead_code)]
    fn len(&self) -> uint { 
        self.p_cstr_i 
    }

    unsafe fn add_char(&mut self, x: u8) -> bool{
        //putstr("1");
        if (self.p_cstr_i == self.max) { 
            return false; 
        }
        //putstr("2");
        *(((self.p as uint)+self.p_cstr_i) as *mut u8) = x;
        //putstr("3");
        self.p_cstr_i += 1;
        *(((self.p as uint)+self.p_cstr_i) as *mut char) = '\0';
        //putstr("4");
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

    unsafe fn equals(&self, other: &str) -> bool {
    	// save val of self.p, which is u8, as a unit
    	let mut selfp: uint = self.p as uint;
    	// iterate through the str "other"
    	for c in slice::iter(as_bytes(other)){
    		// return false if any character does not match
    		if( *c != *(selfp as *u8) ) { return false; }
    		selfp += 1;
    	};
    	return true;
    	*(selfp as *char) == '\0'
    }

	unsafe fn getarg(&self, delim: char, mut k: uint) -> Option<cstr> {
		let mut ind: uint = 0;
		let mut found = k == 0;
		let mut selfp: uint = self.p as uint;
		let mut s = cstr::new(256);
		loop {
			if (*(selfp as *char) == '\0') { 
				// End of string
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