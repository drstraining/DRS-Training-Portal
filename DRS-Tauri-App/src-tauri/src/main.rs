
#![cfg_attr(all(not(debug_assertions), target_os = "windows"), windows_subsystem = "windows")]
use std::fs::{self,OpenOptions};
use std::io::{Read,Write};
use tauri::Manager;
use sha2::{Sha256,Digest};

fn main(){
 tauri::Builder::default()
 .setup(|app|{
    let dir=tauri::api::path::app_dir().unwrap();
    fs::create_dir_all(&dir).ok();
    let f=dir.join("license.dat");
    let mut launches=0u32;

    if f.exists(){
        let mut s=String::new();
        if let Ok(mut h)=OpenOptions::new().read(true).open(&f){
            h.read_to_string(&mut s).ok();
            launches=s.parse().unwrap_or(0);
        }
    }
    launches+=1;
    if launches>2 { std::process::exit(0); }
    let mut w=OpenOptions::new().write(true).create(true).truncate(true).open(&f).unwrap();
    w.write_all(launches.to_string().as_bytes()).ok();
    Ok(())
 })
 .run(tauri::generate_context!())
 .expect("error");
}
