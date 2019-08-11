extern crate rayon;
extern crate console;
#[macro_use]
extern crate lazy_static;

use rayon::prelude::*;
use std::sync::Mutex;
use console::style;
use std::fs;

use std::time::Instant;
use std::fs::File;
use std::io::Write;

use std::env;



mod terminal_render;

#[warn(unused_must_use)]
fn main() {
    if env::args().len() == 2 {
        let now1 = Instant::now();
        match fs::read_to_string("MODEL.txt") {
            Ok(e) => {
                let mut i = true;
                let mut w = String::new();
                for argument in env::args() {
                    if i {
                        i = false;
                        continue;
                    }
                    w = argument.to_string();
                }
                println!("  {} {}",style("Intialized in").green().bold(),style(format!("{}ms",now1.elapsed().as_nanos() as f32 / 1_000_000.0)).yellow().bold());
                let now1 = Instant::now();
                w = clear(w);
                for i in e.replace("[\"","").replace("\"]","").split("\",\"") {
                    if w.contains(i) {
                        println!("  {} {}",style("Search done in").green().bold(),style(format!("{}ms",now1.elapsed().as_nanos() as f32 / 1_000_000.0)).yellow().bold());
                        println!("  {} {}",style("Legit").yellow().bold(),style("False").red().bold());
                        return;
                    }
                }
                println!("  {} {}",style("Search done in").green().bold(),style(format!("{}ms",now1.elapsed().as_nanos() as f32 / 1_000_000.0)).yellow().bold());
                println!("  {} {}",style("Legit").yellow().bold(),style("True").green().bold());
            },
            _ => {
                println!("{} {}",style("Failed to open file").red().bold(),style("MODEL.txt").yellow().bold().italic())
            }
        }
        return;
    }
    println!();
    let now1 = Instant::now();
    let mut vpn: Vec<String> = Vec::new();
    terminal_render::set_loading(1,33.3,None);
    let vpn_string = fs::read_to_string("VPN.txt").unwrap();
    let mut normal: Vec<String> = Vec::new();
    terminal_render::set_loading(1,66.6,None);
    let normal_string = fs::read_to_string("NORMAL.txt").unwrap();

    for x in vpn_string.split('\n') {
        if x.len() >= 3 {
            vpn.push(clear(x.trim().to_string()));
        }
    }
    for x in normal_string.split('\n') {
        if x.len() >= 3 {
            normal.push(clear(x.trim().to_string()));
        }
    }
    terminal_render::set_end(1,now1.elapsed().as_millis());
    let now = Instant::now();
    terminal_render::set_loading(0,0.0,None);
    let finished: Mutex<terminal_render::Type> = Mutex::new(terminal_render::Type::new(0));
    let ln = vpn.len() as f32;

    let lastp: Mutex<terminal_render::Type> = Mutex::new(terminal_render::Type::new(0));
    let u: Vec<String> = vpn.par_iter().flat_map(|p| {
        let hu = input(p.to_string(),&normal_string);
        let dat = filter(&hu,&vpn);
        if !dat.is_empty() {
            println!("{:?}: {:?}", p, dat);
        }

/*        if let Ok(mut e) = finished.lock() {
            e.tp += 1;
            let y = (e.tp  as f32)/ln * 100.0;
            if let Ok(mut i) = lastp.lock() {
                if i.tp < y as u32 {
                    terminal_render::set_loading(0,y,Some(temps_restant(&now,y)));
                    i.tp = y as u32;
                }
            }
        }
*/
        dat
    }).collect();
    terminal_render::set_end(0,now.elapsed().as_millis());

    let now = Instant::now();
    terminal_render::set_loading(2,0.0,None);
    let o = format!("{:?}",filter(&u,&vpn));
    terminal_render::set_loading(2,50.0,None);
    let mut file = File::create("MODEL.txt").unwrap();
    file.write_all(o.as_bytes()).unwrap();
    terminal_render::set_end(2,now.elapsed().as_millis());
}

fn temps_restant(i: &Instant,p: f32) -> u32 {
    let ela = (i.elapsed().as_millis()/100) as f32 / 10.0;
    let time_pp = ela/p;
    (time_pp*(100.0-p)) as u32
}

#[inline(never)]
fn input(b: String, normal: &str) -> Vec<String> {
    let mut dat: Vec<String> = Vec::with_capacity(b.len());
    for x in 0..(b.len() - 3) {
      for y in 3..b.len()-x {
          let str = &b[x..(x+y)];
          if normal.contains(str) {
              continue;
            }
          dat.push(str.to_string());
        }
    }
    dat
}

#[inline(never)]
fn clear(e: String) -> String {
    let mut i = String::new();
    let p = "abcdefghijklmnopqrstuvwxyz0123456789. ";
    for n in e.to_lowercase().split("") {
        if p.contains(n) {
            i.push_str(n);
        }
    }
    i
}

fn filter(dat: &[String],vpn: &[String]) -> Vec<String> {
  filter_lua(filter_gdat(dat, vpn))
}

#[inline(never)]
fn filter_gdat(dat: &[String], vpn: &[String]) -> Vec<String> {
  dat.iter()
    .filter(|q| vpn.iter().filter(|d| d.contains(&q[..])).count() > 1)
    .map(|s| s.to_string())
    .collect()
}

#[inline(never)]
fn filter_lua(gdat: Vec<String>) -> Vec<String> {
  let mut lua: Vec<String> = Vec::new();
  'loop3: for x in &gdat {
    for y in &gdat {
      if y.to_string().contains(x) && y.len() > x.len() {
        continue 'loop3;
      }
    }
    if !lua.contains(&x) {
      lua.push(x.to_string());
    }
  }
  lua
}

#[test]
fn gdat() {
    let strings = |strs: &[&str]| strs.iter().map(|s| s.to_string()).collect::<Vec<_>>();
    assert_eq!(
        filter_gdat(
            &strings(&[
                "a", // appears in three inputs
                "aaaabbaa", // never appears
                "lol", // appears once at the end and once in the middle
                "wards", // only appears in one input, but twice
                "copter", // only appears once at the end
                "be", // appears twice in the middle
                "n't", // one of the occurences is the complete string
            ]),
            &strings(&["abea", "roflolacopter", "don't wardsy wards be happy lol", "n't"]),
        ),
        strings(&["a", "lol", "be", "n't"]),
    );
}
