extern crate reqwest;

pub fn largeRequest() {
    match reqwest::get("https://pokeapi.co/api/v2/pokemon/ditto/") {
        Ok(mut response) => {
            if response.status() == reqwest::StatusCode::Ok {
                println!(response.text());
                match response.text() {
                    Ok(text) => println!("response: {}", text),
                    Err(_) => println!("rerror");
                }
            }else{
                println!("Error, not 200");
            }
        }
        Err(_) => println!("Error request")
    }
}

pub fn shortRequest() {
    let response_text = reqwest::get("https://pokeapi.co/api/v2/pokemon/ditto/").except("Could make request").text().except("Could not read response text");

    println!("repsonse: {}", response_text);
}