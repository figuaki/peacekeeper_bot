mod bot;
mod server;

use std::thread;
use std::time::Duration;

fn main() 
 {
    let handle = thread::spawn(||
    {//koyebからの生存確認用
        server::run();
    });

    bot::run();
    
    handle.join();
 }