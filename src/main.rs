use std::io::{self, BufRead, Write};

fn tinfo(args: &Vec<String>, treni: &trenitalia::Trenitalia) {
    let calling_at: &trenitalia::TrainStation;
    if args.len() == 3 {
        calling_at = treni.get_train_station("ILA").unwrap();
    } else if args.len() == 4 {
        if let Some(x) = treni.find_train_station(&args[3]) {
            calling_at = x;
        } else {
            println!("Error: The station was not found.");
            return;
        }
    } else {
        println!("Usage: {} {} train_number [calling at]", args[0], args[1]);
        return;
    }
    if let Ok(tinfo) = treni.train_info_calling_at(args[2].parse::<u32>().unwrap(), calling_at) {
        println!("=== Train {} ===", args[2]);
        println!("- Delay: {} minutes", tinfo.current_delay);
        let mut current_stop: Option<&trenitalia::DetailedTrainTripStop> = None;
        let mut next_stop: Option<&trenitalia::DetailedTrainTripStop> = None;
        for i in 0..tinfo.stops.len() {
            if tinfo.stops[i].station.id == tinfo.current_station.id {
                current_stop = Some(&tinfo.stops[i]);
                if i < tinfo.stops.len()-1 {
                    next_stop = Some(&tinfo.stops[i+1]);
                } else {
                    next_stop = None;
                }
            }
        }
        if tinfo.is_at_station {
            println!("- Status: Arrived in {} at {}", tinfo.current_station.get_name(), current_stop.unwrap().arrival.map(|x| x.format("%H:%M").to_string()).unwrap_or("?".to_string()));
        } else {
            println!("- Status: Departed from {} at {}", tinfo.current_station.get_name(), current_stop.unwrap().departure.map(|x| x.format("%H:%M").to_string()).unwrap_or("?".to_string()));
        }
        if let Some(ns) = next_stop {
            println!(
                "- Next stop: {}\n\t- Expected arrival: {}\n\t- Expected platform: {}",
                ns.station.get_name(),
                ns.expected_arrival.map(|x| x.format("%H:%M").to_string()).unwrap_or("?".to_string()),
                ns.platform
            );
        }
    } else {
        println!("Usage: {} {} train_number calling_at\nError: multiple trains with the same number were found, please specify a station where the train calls at.", args[0], args[1]);
        return;
    }
}

fn tft(args: &Vec<String>, treni: &trenitalia::Trenitalia, fare: bool) {
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
        let mut total_fare = 0.0;
        print!("\n");
        for i in 0..trip.len() {
            if i==0 {
                print!("{} ",trip[i].departure.0.get_name());
            }
            print!("{} =={}==>> {} {}", trip[i].departure.1.format("%H:%M"), trip[i].train_number.to_string(), trip[i].arrival.1.format("%H:%M"), trip[i].arrival.0.get_name());
			if i!=trip.len()-1 {
				print!(" ");
            }
            if fare {
                total_fare = total_fare + trip[i].get_fare().unwrap_or(0.0);
            }
        }
        if fare {
            print!(" {} €", total_fare);
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
    println!("\ttftf [start] [destination]\tfind train from start to destination, display fares");
    println!("\ttinfo [train number] <[calling at]>\tfind train info");
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
        tft(&args,&treni, false);
    } else if args[1]=="tftf" {
        tft(&args,&treni, true);
    } else if args[1]=="tinfo" {
        tinfo(&args,&treni);
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
