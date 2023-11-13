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

struct Parser<'a>
{
    lexer: Lexer<'a>,
    curr: Token,
    peek: Token,
}

impl<'a> Parser<'a>
{

    fn of(mut lexer: Lexer<'a>) -> Self
    {
        let peek = lexer.next_token();
        let curr = lexer.next_token();
        Parser {
            lexer,
            peek,
            curr,
        }
    }

    fn next(&mut self)
    {
        self.curr = std::mem::replace(&mut self.peek, Token::Eof);
        self.peek = self.lexer.next_token();
    }

    fn parse_header(&mut self, value: u8, buf: &mut str) 
    {
        self.next();
    }

    fn parse(&mut self) -> String
    {
        let mut buf = String::new();
        loop {
            println!("{:?}", self.curr);
            match &self.curr {
                Token::Eof => break,
                Token::Heading(value) => self.parse_header(*value, &mut buf),
                Token::Text(start, size) => {
                    let slice = &self.lexer.buf[*start..*start + *size];
                    let str = std::str::from_utf8(slice).unwrap();
                    buf += str;
                    self.next();
                },
                Token::WhiteSpace => {
                    buf += " ";
                    self.next();
                },
                Token::NewLine => {
                    buf += "\n";
                    self.next();
                },
                _ => self.next(),
            }
        }

        buf
    }

}

fn read_file(path: &str) -> Result<Vec<u8>, io::Error>
{
    let mut file = File::open(path)?;
    let len = file.seek(io::SeekFrom::End(0))?;
    let mut buf: Vec<u8> = Vec::with_capacity(len as usize);
    file.seek(io::SeekFrom::Start(0))?;
    file.read_to_end(&mut buf)?;
    Ok(buf)
}

fn process_file(path: &str, template: &str) -> Result<(), io::Error>
{
    let buf = read_file(path)?;

    let lexer = Lexer::of(&buf);
    let mut parser = Parser::of(lexer);
    let content = parser.parse();

    let output = template.replace("<!-- Content -->", &content);
    println!("{}", output);

    Ok(())
}

fn main()
{
    let template = match read_file("template.html") {
        Ok(template) => String::from_utf8(template).unwrap(),
        Err(err) => {
            println!("Failed to load template: {:?}", err);
            return;
        }
    };

    if let Err(err) = process_file("test.md", &template) {
        println!("Failed to process file, {:?}", err);
    }
}
