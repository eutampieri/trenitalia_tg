use std::io::{self, BufRead, Write};

const TRAIN_TYPE_STR: [&str;10] = ["R","RV","IC","FR","FA","FB","ICN","EN","EC","?"];

fn tft(args: &Vec<String>, treni: &trenitalia::Trenitalia) {
    if args.len()!=4 {
        println!("Usage: {} {} start destination", args[0], args[1]);
        return;
    }
    let s1 = match treni.find_train_station(&args[2]) {
        None => {
            println!("could not find station {}", args[2]);
            return;
        }
        Some(x) => x
    };
    let s2 = match treni.find_train_station(&args[3]) {
        None => {
            println!("could not find station {}", args[3]);
            return;
        }
        Some(x) => x
    };
    let trips = treni.find_trips(s1, s2, &chrono::Local::now());
    println!("Solutions for {} -> {}",s1.get_name(),s2.get_name());
    for trip in &trips {
        print!("\n");
        for i in 0..trip.len() {
            if i==0 {
                print!("{} ",trip[i].departure.0.get_name());
            }
            print!("{} =={}{}==>> {} {}", trip[i].departure.1.format("%H:%M"), if trip[i].train_type as i32 as usize >= TRAIN_TYPE_STR.len() {"?"} else {TRAIN_TYPE_STR[trip[i].train_type as i32 as usize]}, trip[i].train_number, trip[i].arrival.1.format("%H:%M"), trip[i].arrival.0.get_name());
			if i!=trip.len()-1 {
				print!(" ");
			}
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
