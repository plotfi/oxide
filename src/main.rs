
fn get_mime_type(filename: String) -> String {
  let filename_toks : Vec<&str> = filename.split(".").collect();

  let namestr = match filename_toks.len() {
      0 => { "" },
      _ => { filename_toks[filename_toks.len() - 1] }
  };

  String::from(
    match namestr {
      "html" => { "text/html; charset=iso-8859-1" },
      "midi" => { "audio/midi" },
      "jpg"  => { "image/jpeg" },
      "jpeg" => { "image/jpeg" },
      "mpeg" => { "video/mpeg" },
      "gif"  => { "image/gif" },
      "png"  => { "image/png" },
      "css"  => { "text/css" },
      "au"   => { "audio/basic" },
      "wav"  => { "audio/wav" },
      "avi"  => { "video/x-msvideo" },
      "mov"  => { "video/quicktime" },
      "mp3"  => { "audio/mpeg" },
      "m4a"  => { "audio/mp4" },
      "pdf"  => { "application/pdf" },
      "ogg"  => { "application/ogg" },
      _ => { "text/plain; charset=iso-8859-1" }
    }
  )
}

fn send_headers(status: i32, title: String, mime: String,
                socket: i32, len: u32) -> std::string::String {

  let mut HeaderStr =
    format!("HTTP/1.1 {} {} \r\nServer: swift-httpd\r\n", status, title) +
  match mime.as_str() {
    "" => { format!("Content-Type: {} \r\n", mime) },
    _  => { "" }
  };

  if len >= 0 {
    HeaderStr += format!("Content-Length: {}\r\n", len);
  }

  HeaderStr += "Connection: close\r\n\r\n";

  HeaderStr
}

fn main() {
    let a = get_mime_type(String::from("foo.mp3"));
    println!("MIME {}\n", a);
}
