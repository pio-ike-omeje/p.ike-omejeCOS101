use std::fs::File;
use std::io::Write; 

fn main() {
    // Create the 3 arrays representing the commissioners' names, ministries and respective zones
    let commissioners = [
        "Aigbogun Alamba Daudu ",
        "Murtala Afeez Bendu ",
        "Okorocha Calistus Ogbona ",
        "Adewale Jimoh Akanbi ",
        "Osazuwa Faith Etieye ",
    ];

    let ministries = [
        "Internal Affairs ",
        "Justice ",
        "Defense ",
        "Power & Steel ",
        "Petroleum ",
    ];

    let zones = [
        "South West",
        "North East",
        "South South",
        "South West",
        "South East",
    ];

    //  Create the file that'd house the 3 datasets
    let mut file = File::create("efcc_report.csv").expect("Failed to create file");

    //  Loop through 0 to 4
    for i in 0..5 {

        // collect the current name, ministry and zone from the arrays
        let name = commissioners[i];
        let ministry = ministries[i];
        let zone = zones[i];

        // Write the name, ministry and zone to the file
        file.write_all(name.as_bytes()).expect("Failed");
        file.write_all(ministry.as_bytes()).expect("Failed");
        file.write_all(zone.as_bytes()).expect("Failed");
    }

    println!("Successfully merged all 3 datasets");
}
