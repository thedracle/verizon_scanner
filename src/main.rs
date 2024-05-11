use std::sync::Arc;

use tokio::{sync::Mutex, task::JoinSet};
use verizon_api::{lookup_verizon_service, VerizonResponse};

const REQUEST_DISPATCH_GROUP_SIZE : usize = 10;

mod verizon_api;

#[tokio::main]
async fn main() {
    // Read each line in input_addresses.csv
    let rdr = csv::Reader::from_path("input_addresses.csv");
    let mut rdr = match rdr {
        Ok(rdr) => rdr,
        Err(_) => {
            println!("Error: you must create an input_addresses.csv document, containing the addresses you want to check.\nThe format should be: name,address,city,state,zippcode");
            return;
        }
    };
    let output = Arc::new(Mutex::new(csv::Writer::from_path("output_addresses.csv").unwrap()));
    output.lock().await.write_record(["Name", "Address", "City", "State", "Zip", "Qualified 4G", "Qualified", "Qualified C-Band"]).unwrap();
    let mut set = JoinSet::new();
    for result in rdr.records() {
        let record = result.unwrap();
        let name = record[0].to_string();
        let address = record[1].to_string();
        let city = record[2].to_string();
        let state = record[3].to_string();
        let zip = record[4].to_string();

        if set.len() >= REQUEST_DISPATCH_GROUP_SIZE {
            set.join_next().await;
            while set.try_join_next().is_some() {}
        }

        let output_clone = output.clone();
        set.spawn(async move {
            let response = lookup_verizon_service(&address, &city, &state, &zip).await;
            match response {
                Ok(response) => {
                    // Parse the response
                    let verizon_response = response.json::<VerizonResponse>().await;
                    match verizon_response {
                        Ok(verizon_response) => {
                            println!("Name: {}, Address: {} {} {} {}, 4G: {}, 5G: {}, C-Band: {}", name, address, city, state, zip, verizon_response.output.qualified4_ghome, verizon_response.output.qualified, verizon_response.output.qualified_cband);
                            output_clone.lock().await.write_record(&[name, address, city, state, zip, verizon_response.output.qualified4_ghome.to_string(), verizon_response.output.qualified.to_string(), verizon_response.output.qualified_cband.to_string()]).unwrap();

                        }
                        Err(_) => {
                            println!("Unable to lookup Verizon service for address: {} {} {} {}", address, city, state, zip);
                        }
                    }
                }
                Err(e) => {
                    println!("Error: {:?}", e);
                }
            }
        });
    }

    // Wait for all tasks to complete
    for _ in 0..set.len() {
        set.join_next().await;
    }
}
