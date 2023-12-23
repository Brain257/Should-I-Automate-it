use std::env;

fn help() {
    println!("usage:
./should_i_automate.exe <years> {{years: how long it will have to be done manually before a new solution would make it obsolete}}");
}

fn invalid_number() {
    println!("Invalid number provided, number must be u16 and > 1");
}

fn compute_time_spent(days:u32, occurence_per_day: f32, time_for_event: u32) -> String{
    let result = ((time_for_event as f32) * occurence_per_day * (days as f32)) as i64;
    let seconds = result % 60;
    let minutes = (result / 60) % 60;
    let hours = (result / 60) / 60;
    let return_string = format!("{:0>2}:{:0>2}:{:0>2}", hours, minutes, seconds);
    return return_string
}

fn compute_automation_time(years: u16) {
    println!("Computing should I automate diagram for {} years", years);
    let days: u32 = (years * 365).into();

    let times_per_event = [5, 30, 60, 2*60, 5*60, 15*60, 30*60, 60*60, 2*60*60, 4*60*60];
    let occurences_per_day: [f32; 7] = [50.0, 10.0, 1.0, 2.0/7.0, 1.0/7.0, 2.0/(7.0 * 4.33), 1.0/(7.0 * 4.33)];
    println!("{:?}", times_per_event);
    println!("{:?}", occurences_per_day);

    println!("|{:>10}|{:>10}|{:>10}|{:>10}|{:>10}|{:>10}|{:>10}|", "50/day", "10/day", "daily", "bi-weekly", "weekly", "bi-monthly", "monthly");
    for time in times_per_event {
        let mut time_spent: Vec<String> = vec![String::new(); 7];
        for (i, occurencies) in occurences_per_day.iter().enumerate() {
            time_spent[i] = compute_time_spent(days, *occurencies, time);
        }
        println!("|{:>10}|{:>10}|{:>10}|{:>10}|{:>10}|{:>10}|{:>10}|", time_spent[0], time_spent[1], time_spent[2], time_spent[3], time_spent[4], time_spent[5], time_spent[6]);        
    }
    
}

fn main() {
    let args: Vec<String> = env::args().collect();

    match args.len(){
        1 => {
            println!("Compute automation time needed for default of 5 years");
            compute_automation_time(5)
        },
        2 => {
            match args[1].parse::<u16>() {
                Ok(n) => {
                    if n == 0 {
                        invalid_number();
                    }
                    else {
                        compute_automation_time(n);
                    }
                }
                Err(e) => {
                    println!("{}", e);
                    invalid_number();
                }
            }
        },
        _ => {
            help();
        }
    }
}
