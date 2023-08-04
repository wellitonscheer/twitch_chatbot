use tungstenite::Message;
use tungstenite::connect;
use url::Url;
use std::time::Duration;
use regex::Regex;
use std::thread;

fn main() {
    let (mut socket, response) = connect(Url::parse("ws://irc-ws.chat.twitch.tv:80").unwrap()).expect("Failed to connect");
    
    println!("Connected to the server");
    println!("Response HTTP code: {}", response.status());
    println!("Response contains the following headers:");
    for (ref header, value ) in response.headers().iter() {
        println!("* {}, {:?}", header, value);
    }

    socket.send(Message::Text("CAP REQ twitch.tv/commands".to_string())).unwrap();
    socket.send(Message::Text("PASS oauth:52l3h02pbei9idoy35giuv5qri90df".to_string())).unwrap();
    socket.send(Message::Text("NICK caieallant".to_string())).unwrap();
    socket.send(Message::Text("JOIN #camelul".to_string())).unwrap();

    loop {
        socket.send(Message::Text("PRIVMSG #camelul :$fish".to_string())).unwrap();

        match socket.read() {
            Ok(msg) => {
                if msg.to_string().contains(":supibot!supibot@supibot.tmi.twitch.tv") && msg.to_string().contains(":caieallant")/* && msg.to_string().contains("t") */{
                    println!("Received: {}", msg);

                    let re: Regex = Regex::new(r"\((.*?)\)").unwrap();
                    let cap: String = re.captures(&msg.to_string()).unwrap().get(1).unwrap().as_str().parse().unwrap();
                    println!("Cap: {}", cap);

                    let mut total_seconds: u16 = 0;

                    if cap.contains("cooldown") {
                        println!("Contains cooldown");

                        if cap.contains("m") && cap.contains("s") {
                            let minutes: u16 = get_minutes(&cap);
                            let seconds: u16 = get_seconds(&cap);
                            total_seconds = minutes * 60 + seconds;
                        }
                        else {
                            let seconds: u16 = get_seconds(&cap);
                            total_seconds = seconds;
                        }
                    }
                    else if cap.contains("You caught") {
                        total_seconds = 1800;
                    }

                    total_seconds += 2;

                    thread::sleep(Duration::from_secs(total_seconds.into()));
                }
            }
            Err(e) => {
                println!("Error reading message: {}", e);
            }
        }
    }
}

pub fn get_seconds(msg: &str) -> u16{
    let re = Regex::new(r"(\d+)s").unwrap();
    let seconds: u16 = re.captures(msg).unwrap().get(1).unwrap().as_str().parse().unwrap();

    return seconds;
}

pub fn get_minutes(msg: &str) -> u16{
    let re = Regex::new(r"(\d+)m").unwrap();
    let minutes: u16 = re.captures(msg).unwrap().get(1).unwrap().as_str().parse().unwrap();

    return minutes;
}

#[cfg(test)]
mod tests {
    use crate::get_minutes;
    #[test]
    fn test_get_minutes() {
        let msg = "1m, 9s cooldown";
        let minutes = get_minutes(msg);
        assert_eq!(minutes, 1);
    }

    #[test]
    fn test_get_minutes_with_space() {
        let msg = "1m , 9s cooldown";
        let minutes = get_minutes(msg);
        assert_eq!(minutes, 1);
    }

    #[test]
    fn test_get_30_minutes() {
        let msg = "30m , 9s cooldown";
        let minutes = get_minutes(msg);
        assert_eq!(minutes, 30);
    }

    #[test]
    fn test_get_23_minutes() {
        let msg = "23m, 9s cooldown";
        let minutes = get_minutes(msg);
        assert_eq!(minutes, 23);
    }

    use crate::get_seconds;
    #[test]
    fn test_get_seconds() {
        let msg = "1m, 9s cooldown";
        let seconds = get_seconds(msg);
        assert_eq!(seconds, 9);
    }

    #[test]
    fn test_get_seconds_with_22() {
        let msg = "1m , 22s cooldown";
        let seconds = get_seconds(msg);
        assert_eq!(seconds, 22);
    }

    #[test]
    fn test_get_single_second() {
        let msg = "30m , 1s cooldown";
        let seconds = get_seconds(msg);
        assert_eq!(seconds, 1);
    }

    #[test]
    fn test_get_332_seconds() {
        let msg = "23m, 233s cooldown";
        let seconds = get_seconds(msg);
        assert_eq!(seconds, 233);
    }
    
    #[test]
    fn test_wait(){
        use std::time::Duration;
        use std::thread;
        println!("Waiting 3 seconds");
        thread::sleep(Duration::from_secs(3));
        println!("Done waiting");
        assert_eq!(1, 1)
    }
}