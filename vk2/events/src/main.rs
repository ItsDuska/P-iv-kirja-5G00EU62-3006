fn main() {
    let months: Vec<&str> = vec![
        "January",
        "February",
        "March",
        "April",
        "May",
        "June",
        "July",
        "August",
        "September",
        "October",
        "November",
        "December",
    ];

    let events = vec![
        (
            2009,
            15,
            1,
            "Chesley Sullenberger lands US Airways Flight 1549 on the Hudson River shortly after takeoff from LaGuardia Airport in NYC. All passengers and crew members survive in what becomes known as the (Miracle on the Hudson)",
        ),
        (
            1920,
            16,
            1,
            "First assembly of the League of Nations is held in Paris",
        ),
        (
            1873,
            17,
            1,
            "Modoc warriors defeats the United States Army in the First Battle of the Stronghold, a part of the Modoc War",
        ),
        (
            1817,
            18,
            1,
            "Argentine General José de San Martín leads a revolutionary army over the Andes to attack Spanish royalists in Chile",
        ),
        (
            1883,
            19,
            1,
            "The first electric lighting system employing overhead wires, built by Thomas Edison, begins service at Roselle, New Jersey",
        ),
        (
            1841,
            20,
            1,
            "China cedes Hong Kong Island to Britain during the First Opium War",
        ),
        (
            1952,
            21,
            1,
            "Jawaharlal Nehru's Indian National Congress wins India's first general election",
        ),
        (
            1905,
            22,
            1,
            "A large demonstration of workers in St Petersburg, Russia, led by Father Gapon, marches to the Winter Palace with a petition to the Tsar; troops fire on the protesters in what becomes known as Bloody Sunday",
        ),
        (
            1973,
            22,
            1,
            "In a landmark decision, the US Supreme Court legalizes most abortions (Roe v. Wade). Writing the majority opinion, Justice Harry Blackmun states that the criminalization of abortion does not have (roots in the English common law tradition.)",
        ),
    ];
    // testi yhtä päivää varten
    let test_day: i32 = 15;
    for &(year, day, month, text) in &events {
        if day == test_day {
            println!(
                "Events of {}: {}\n{}: {}\n-------\n",
                months[month], day, year, text
            )
        }
    }

    for current_date in 15..=22 {
        let mut printed_date = false;

        for &(year, day, month, text) in &events {
            if day == current_date {
                if !printed_date {
                    println!("Events of {} {}:", day, months[month - 1]);
                    printed_date = true;
                }

                println!("{}: {}\n", year, text);
            }
        }
    }
}
