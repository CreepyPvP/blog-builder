use std::io::{self, Seek, Read};

use std::fs::File;

#[derive(PartialEq, Debug)]
enum Token 
{
    Heading(u8),
    NewLine,
    WhiteSpace,
    Text(usize, usize),
    Eof,
}

struct Lexer<'a> 
{
    buf: &'a Vec<u8>,
    pos: usize,
}

impl<'a> Lexer<'a>
{

    fn of(buf: &'a Vec<u8>) -> Self {
        Lexer {buf, pos: 0}
    }

    fn next_token(&mut self) -> Token
    {
        if self.pos >= self.buf.len() {
            return Token::Eof;
        }

        let c = self.buf[self.pos] as char;
        let token = match c {
            '#' => {
                let mut counter: u8 = 0;
                loop {
                    if self.pos >= self.buf.len() || 
                        self.buf[self.pos] as char != '#' {
                        break;
                    }
                    self.pos += 1;
                    counter += 1;
                }
                Token::Heading(counter)
            },
            ' ' => {
                self.pos += 1;
                Token::WhiteSpace
            },
            '\n' => {
                self.pos += 1;
                Token::NewLine
            },
            _ => {
                let start = self.pos;
                let mut len = 0;
                loop {
                    if self.pos >= self.buf.len() {
                        break;
                    }
                    let c = self.buf[self.pos] as char;
                    if c.is_alphanumeric() || c.is_ascii_punctuation() {
                        self.pos += 1;
                        len += 1;
                    } else  {
                        self.pos += 1;
                        break;
                    }
                }
                Token::Text(start, len)
            },
        };
        token
    }

}


fn process_file(path: &str) -> Result<(), io::Error>
{
    let mut file = File::open(path)?;
    let len = file.seek(io::SeekFrom::End(0))?;
    let mut buf: Vec<u8> = Vec::with_capacity(len as usize);
    file.seek(io::SeekFrom::Start(0))?;
    file.read_to_end(&mut buf)?;

    let mut lexer = Lexer::of(&buf);
    loop {
        let token = lexer.next_token();
        println!("{:?}", token);

        if token == Token::Eof {
            break;
        }
    }
    
    Ok(())
}

fn main()
{
    if let Err(err) = process_file("test.md") {
        println!("Failed to process file, {:?}", err);
    }
}
