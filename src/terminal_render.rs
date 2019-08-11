use console::{Term,style};
use std::sync::Mutex;

lazy_static! {
    static ref DATA: Mutex<Type> = Mutex::new(Type::new(99));
}

pub struct Type {
    pub tp: u32
}

impl Type {
    pub fn new(i: u32) -> Self {
        Self{tp: i}
    }
}

pub fn set_loading(tp: u32,percentage: f32,tr: Option<u32>) {

    if let Ok(mut e) = DATA.lock() {

                let term = Term::stdout();
                if e.tp == tp {
                    term.clear_last_lines(1).unwrap();
                } else {
                    e.tp = tp;
                }
        }
        if tp == 0 {
            print!("  {} [",style("Learning").cyan().bold());
        } else if tp == 1 {
            print!("   {} [",style("Reading").green().bold());
        } else if tp == 2 {
            print!("   {} [",style("Writing").yellow().bold());
        }
        let ip = (percentage / 2.5) as u32;
        let number = ip;
        let n = 40;
        for _k in 1..number {
           print!("=");
        }
        if number > 1 {
            print!(">");
        }
        if number < n {
           for _k in 0..(n-number) {
               print!(" ");
           }
        }
        print!("] {}%",(f64::from(number)/f64::from(n) * 100.0) as i32);
        if let Some(e) = tr {

                if e > 60 {
                    print!(" {}m {}s restantes",e/60,e%60);
                } else {
                    print!(" {}s restantes",e);
                }
        }
        println!();
}

pub fn set_end(tp: u32,ms: u128) {

        if let Ok(mut e) = DATA.lock() {
                let term = Term::stdout();
                if e.tp == tp {
                    term.clear_last_lines(1).unwrap();
                } else {
                    e.tp = tp;
                }
        }
        if tp == 0 {
            print!("  {} ",style("Learning").cyan().bold());
        } else if tp == 1 {
            print!("   {} ",style("Reading").green().bold());
        } else if tp == 2 {
            print!("   {} ",style("Writing").yellow().bold());
        }
        if ms < 100 {
            print!("Done in {}ms",ms);
        } else {
            let o = (ms/100*100) as f32 / 1000.0;
            if o > 60.0 {
                print!("Done in {}m {}s",(o as u32)/60,o%60.0);
            } else {
                print!("Done in {}s",o);
            }
        }
        println!();
}
