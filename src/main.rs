use std::{
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
    env
};

const HTMLFILE: &str = "
<!DOCTYPE html>
<html>
    <head>
        <title>Presentation</title>
    </head>
    <body>
        <div style=\"
            position: relative; 
            width: 100%; 
            height: 0; 
            padding-top: 56.2500%;
            padding-bottom: 0; 
            box-shadow: 0 2px 8px 0 rgba(63,69,81,0.16); 
            margin-top: 1.6em; margin-bottom: 0.9em; 
            overflow: 
            hidden;
            border-radius: 8px; 
            will-change: transform;\">
            <iframe 
                loading=\"lazy\" style=\"position: absolute; width: 100%; height: 100%; top: 0; left: 0; border: none; padding: 0;margin: 0;\"
                src=\"https:&#x2F;&#x2F;www.canva.com&#x2F;design&#x2F;DAFSO1x1b7g&#x2F;view?embed\" allowfullscreen=\"allowfullscreen\" allow=\"fullscreen\">
            </iframe>
        </div>
        <a href=\"https:&#x2F;&#x2F;www.canva.com&#x2F;design&#x2F;DAFSO1x1b7g&#x2F;view?utm_content=DAFSO1x1b7g&amp;utm_campaign=designshare&amp;utm_medium=embeds&amp;utm_source=link\" target=\"_blank\" rel=\"noopener\">ดีไซน์</a>โดย Saravut Nakglom
    </body>
</html>
";

fn main() {
    let port: String = if let Ok(env_port) = env::var("PORT") {
        env_port
    } else {
        panic!("No port specified")
    };
    let listener = TcpListener::bind(format!("0.0.0.0:{}", port)).unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let _http_request: Vec<_> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();

    let status_line = "HTTP/1.1 200 OK";
    let length = HTMLFILE.len();

    let response =
        format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{HTMLFILE}");

    stream.write_all(response.as_bytes()).unwrap();
}
