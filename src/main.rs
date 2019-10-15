use std::io::{self, BufRead, Write};

fn tft(args: &Vec<String>, treni: &trenitalia::Trenitalia) {
    if args.len()!=4 {
        println!("Usage: {} {} start destination", args[0], args[1]);
        return;
    }
    let s1 = treni.find_train_station(&args[2])
        .or_else(||{println!("could not find station {}", args[2]);std::process::exit(1);}).unwrap();
    let s2 = treni.find_train_station(&args[3])
        .or_else(||{println!("could not find station {}", args[2]);std::process::exit(1);}).unwrap();
    let trips = treni.find_trips(s1, s2, &chrono::Local::now());
    println!("Solutions for {} -> {}",s1.name,s2.name);
    for trip in &trips {
        print!("\n");
        for train in trip {
            print!("{} {} --{:?}-{}--> {} {}\t",train.departure.0.name, train.departure.1.format("%H:%M"), train.train_type, train.train_number, train.arrival.0.name, train.arrival.1.format("%H:%M") );
        }
        print!("\n");
	}
}

fn interactive(args : &Vec<String>, treni: &trenitalia::Trenitalia){
    if args.len()>2 {
        help(args);
    }
    let stdin = io::stdin();
    print!("\n#");
    io::stdout().flush().unwrap();
    for line in stdin.lock().lines() {
        let mut new_args: Vec<String>=vec![String::from(" ")];
        for arg in line.unwrap().split_whitespace() {
            new_args.push(String::from(arg));
        }
        if new_args.len()>=2 && new_args[1]=="exit"{
			return;
        }
        exec(&new_args,&treni,false);
        print!("\n#");
        io::stdout().flush().unwrap();
    }
}

fn help(args : &Vec<String>){
    println!("Usage: {}",args[0]);
    println!("\ttft [start] [destination]\tfind train from start to destination");
    println!("\tinteractive\tenter interactive mode");
    println!("\texit\texit from interactive mode");
    println!("\thelp\tshow this message");
}

fn exec(args : &Vec<String>, treni: &trenitalia::Trenitalia, allow_interactive: bool){
    if args.len()<2 {
        help(&args);
        return;
    }
    if args[1]=="tft" {
        tft(&args,&treni);
    } else if args[1]=="interactive" && allow_interactive {
        interactive(&args, &treni);
    } else if args[1]=="help"{
        help(&args);
    } else {
        help(&args);
    }
}

fn main() {
    let args : Vec<String> = std::env::args().collect();
    let treni = trenitalia::Trenitalia::new();
    exec(&args,&treni,true);
}
