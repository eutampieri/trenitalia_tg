use std::io::{self, BufRead, Write};

mod telegram;

fn tinfo(args: &Vec<String>, treni: &trenitalia::Trenitalia, bot: &telegram::Telegram, chat_id: i64) {
    let mut output = "".to_string();
    let calling_at: &trenitalia::TrainStation;
    if args.len() == 3 {
        calling_at = treni.get_train_station("ILA").unwrap();
    } else if args.len() == 4 {
        if let Some(x) = treni.find_train_station(&args[3]) {
            calling_at = x;
        } else {
            let message = telegram::Message{chat: telegram::Chat{id: chat_id}, message_id: 0, text: Some(format!("Error: The station was not found."))};
        bot.send_message(&message);
            return;
        }
    } else {
        let message = telegram::Message{chat: telegram::Chat{id: chat_id}, message_id: 0, text: Some(format!("Usage: {} {} train_number [calling at]", args[0], args[1]))};
        bot.send_message(&message);
        return;
    }
    if let Ok(tinfo) = treni.train_info_calling_at(args[2].parse::<u32>().unwrap(), calling_at) {
        output = output + format!("=== Train {} ===\n", args[2]).as_str();
        output = output + format!("- Delay: {} minutes\n", tinfo.current_delay).as_str();
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
            output = output + format!("- Status: Arrived in {} at {}\n", tinfo.current_station.get_name(), current_stop.unwrap().arrival.map(|x| x.format("%H:%M").to_string()).unwrap_or("?".to_string())).as_str();
        } else {
            output = output + format!("- Status: Departed from {} at {}\n", tinfo.current_station.get_name(), current_stop.unwrap().departure.map(|x| x.format("%H:%M").to_string()).unwrap_or("?".to_string())).as_str();
        }
        if let Some(ns) = next_stop {
            output = output + format!(
                "- Next stop: {}\n    - Expected arrival: {}\n    - Expected platform: {}\n",
                ns.station.get_name(),
                ns.expected_arrival.map(|x| x.format("%H:%M").to_string()).unwrap_or("?".to_string()),
                ns.platform
            ).as_str();
        }
    } else {
        let message = telegram::Message{chat: telegram::Chat{id: chat_id}, message_id: 0, text: Some(format!("Usage: {} {} train_number calling_at\nError: multiple trains with the same number were found, please specify a station where the train calls at.", args[0], args[1]))};
        bot.send_message(&message);
        return;
    }
    let message = telegram::Message{chat: telegram::Chat{id: chat_id}, message_id: 0, text: Some(output)};
    bot.send_message(&message);
}

fn tft(args: &Vec<String>, treni: &trenitalia::Trenitalia, bot: &telegram::Telegram, chat_id: i64, fare: bool) {
    if args.len()!=4 {
        let message = telegram::Message{chat: telegram::Chat{id: chat_id}, message_id: 0, text: Some(format!("Usage: {} {} start destination", args[0], args[1]))};
        bot.send_message(&message);
        return;
    }
    let s1 = match treni.find_train_station(&args[2]) {
        None => {
            let message = telegram::Message{chat: telegram::Chat{id: chat_id}, message_id: 0, text: Some(format!("could not find station {}", args[2]))};
            return;
        }
        Some(x) => x
    };
    let s2 = match treni.find_train_station(&args[3]) {
        None => {
            let message = telegram::Message{chat: telegram::Chat{id: chat_id}, message_id: 0, text: Some(format!("could not find station {}", args[3]))};
            return;
        }
        Some(x) => x
    };
    let trips = treni.find_trips(s1, s2, &chrono::Local::now());
    let mut output = "".to_string();
    output = output + format!("Solutions for {} -> {}\n",s1.get_name(),s2.get_name()).as_str();
    for trip in &trips {
        let mut total_fare = 0.0;
        output = output + "\n";
        for i in 0..trip.len() {
            if i==0 {
                output = output + format!("{} ",trip[i].departure.0.get_name()).as_str();
            }
            output = output + format!("{} =={}==>> {} {}", trip[i].departure.1.format("%H:%M"), trip[i].train_number.to_string(), trip[i].arrival.1.format("%H:%M"), trip[i].arrival.0.get_name()).as_str();
			if i!=trip.len()-1 {
				output = output + " ";
            }
            if fare {
                total_fare = total_fare + trip[i].get_fare().unwrap_or(0.0);
            }
        }
        if fare {
            output = output + format!(" {} €", total_fare).as_str();
        }
        output = output + "\n";
    }
    let message = telegram::Message{chat: telegram::Chat{id: chat_id}, message_id: 0, text: Some(output)};
    bot.send_message(&message);
}

fn interactive(treni: &trenitalia::Trenitalia, telegram: &mut telegram::Telegram){
    let messages = telegram.read_messages();
    println!("{:?}", messages);
    if let Ok(msgs) = messages {
        for msg in msgs {
            if let Some(text) = msg.text {
                let mut new_args = vec![String::from(" ")];
                for arg in text.split_whitespace() {
                    new_args.push(String::from(arg));
                }
                new_args[1] = new_args[1].split("@").collect::<Vec<&str>>()[0].replace("/", "");
                println!("{:?}", new_args);
                exec(&new_args,&treni, telegram, msg.chat.id, false);
            }
        }
    }
}

fn help(args : &Vec<String>, bot: &telegram::Telegram, chat_id: i64){
    let mut output = "".to_string();
    output = output + format!("Usage: {}",args[0]).as_str();
    output = output + "\n    /tft [start] [destination]    find train from start to destination";
    output = output + "\n    /tftf [start] [destination]    find train from start to destination, display fares";
    output = output + "\n    /tinfo [train number] <[calling at]>    find train info";
    output = output + "\n    /interactive    enter interactive mode";
    output = output + "\n    /help    show this message";
    let message = telegram::Message{chat: telegram::Chat{id: chat_id}, message_id: 0, text: Some(output)};
    bot.send_message(&message);
}

fn exec(args : &Vec<String>, treni: &trenitalia::Trenitalia, bot: &telegram::Telegram, chat_id: i64, allow_interactive: bool){
    if args.len()<2 {
        help(&args, bot, chat_id);
        return;
    }
    if args[1]=="tft" {
        tft(&args,&treni, bot, chat_id, false);
    } else if args[1]=="tftf" {
        tft(&args,&treni, bot, chat_id, true);
    } else if args[1]=="tinfo" {
        tinfo(&args,&treni, bot, chat_id);
    } else if args[1]=="help"{
        help(&args, bot, chat_id);
    } else {
        help(&args, bot, chat_id);
    }
}

fn main() {
    let args : Vec<String> = std::env::args().collect();
    let treni = trenitalia::Trenitalia::new();
    let mut bot = telegram::Telegram::from(&args[1]);
    loop {
        interactive(&treni, &mut bot);
    }
}
