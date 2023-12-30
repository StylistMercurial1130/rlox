use std::fmt; 
use std::collections::HashMap;

#[allow(dead_code)]
#[derive(Debug,Clone)]
pub enum TokenType {
    // single character token
    LeftBrace , RightBrace, LeftParen, RightParen,
    Comma, Dot, Minus, Plus, Semicolon, Slash, Star,
    //one or two character tokens
    Bang, BangEqual, Equal, EqualEqual, Greater, GreaterEqual,
    Less, LessEqual,
    //literals
    Number, String, Identifier, Comments,
    //keywords
    And, Class, Else, False, Fun, For, If, Nil, Or,
    Print, Return, Super, This, True, Var, While,
    Eof
}

impl fmt::Display for TokenType {
    fn fmt(&self, f : &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f,"{:?}",self)
    }
}


#[allow(dead_code)]
pub struct Token {
    token_type : TokenType,
    line_number : u32,
    literal : String,
    column_number : u32
}

#[allow(dead_code)]
impl Token {

    pub fn new(token_type : TokenType,line_number : u32,literal : String,column_number: u32) -> Self {
       Token {
           token_type,
           line_number,
           literal : literal.clone(),
           column_number
       } 
    }

    pub fn to_string(&self) -> String {
        let tokentype = self.token_type.to_string();
        format!("< {0} => [ value : {1}  , line_number : {2} ] >",tokentype,self.literal,self.line_number) 
    }

}

#[allow(dead_code)]
pub struct Tokenizer {
    source : Vec<char>,
    start : usize,
    end : usize,
    current : usize,
}

#[allow(dead_code)]
impl Tokenizer {

    pub fn new(src : String) -> Self {
        let mut source : Vec<char> = Vec::new();
        let start = 0;
        let end = src.len() - 1;
        let current = 0;
        src.chars().for_each(|character| {
            source.push(character.clone());
        });
        Tokenizer {source, start, end, current} 
    }

    pub fn current(&self) -> Option<char> {
        if self.current <= self.end {
            return Some(self.source[self.current])
        }
        return None
    }

    pub fn peek(&self) -> Option<char> {
        if self.current <= self.end - 1 {
            return Some(self.source[self.current + 1])
        }
        return None;
    }

    pub fn advance(&mut self) {
       self.current += 1; 
    }

    fn push_tokens(&self,tokens : &mut Vec<Token>,token_type : TokenType,column : u32,literal : String,line : u32) {
        let token = Token::new(token_type,line,literal,column); 
        tokens.push(token);
    }

    fn check_if_other_and_add(&self,tokens : &mut Vec<Token>,current_character : char,current_token_type : TokenType,line_number : u32) {
        let mut literal = current_character.to_string(); 
        let mut current_token_type = current_token_type;
        if let Some(next_character) = self.peek() {
            if next_character == '=' {
                match current_token_type {
                    TokenType::Bang => current_token_type = TokenType::BangEqual,
                    TokenType::Greater => current_token_type = TokenType::GreaterEqual,
                    TokenType::Less => current_token_type = TokenType::LessEqual,
                    TokenType::Equal => current_token_type = TokenType::EqualEqual,
                    _ => {}
                }
                literal.push(next_character);
            } else {
                println!("unknown expresssion ! , {0}{1}",literal,next_character);
            } 
        } 
        self.push_tokens(tokens,current_token_type,0,literal,line_number);
    }

    fn string_literal(&mut self,tokens : &mut Vec<Token>,line_number : u32) {
        let mut literal = String::from(self.current().unwrap());
        self.advance();
        while let Some(character) = self.current() {
            if character != '"' {
                literal.push(character);
            } else {
                literal.push(character);
                break;
            }
            self.advance();
        }
        self.push_tokens(tokens,TokenType::String,0,literal,line_number);
    }

    fn number(&mut self,tokens : &mut Vec<Token>,line_number : u32) {
        let mut literal = String::from(self.current().unwrap());
        self.advance();
        let mut is_float = false;
        while let Some(character) = self.current() {
            if character.is_numeric() || character == '.' {
                if character == '.' {
                    if is_float == true {
                        println!("invalid float expression !");
                    } else { is_float = true }
                }
                literal.push(character); 
            } else {
                //handle error here !
                break;         
            }
            self.advance();
        }
        self.push_tokens(tokens,TokenType::Number,0,literal,line_number);
        return;
    }

    pub fn literals(&mut self,tokens : &mut Vec<Token>,line_number : u32) {
        let keyword_set : HashMap<&str,TokenType> = 
            HashMap::from([
                ("and",TokenType::And),
                ("class",TokenType::Class),
                ("else",TokenType::Else),
                ("false",TokenType::False),
                ("fun",TokenType::Fun),
                ("for",TokenType::For),
                ("if",TokenType::If),
                ("nil",TokenType::Nil),
                ("or",TokenType::Or),
                ("print",TokenType::Print),
                ("return",TokenType::Return),
                ("super",TokenType::Super),
                ("this",TokenType::This),
                ("true",TokenType::True),
                ("var",TokenType::Var),
                ("while",TokenType::While)
            ]);
        let mut literal = String::from(self.current().unwrap());
        self.advance();
        while let Some(character) = self.current() {
            if character.is_alphanumeric() || character == '_' || character == '-' {
                literal.push(character); 
            } else {
                if keyword_set.contains_key(literal.clone().as_str()) {
                    let token_type = keyword_set.get(literal.clone().as_str()).cloned().unwrap();
                    self.push_tokens(tokens,token_type,0,literal,line_number);
                } else {
                    self.push_tokens(tokens,TokenType::Identifier,0,literal,line_number);
                }         
                return
            }
            self.advance();
        }
    }

    pub fn comments(&mut self,tokens : &mut Vec<Token>,line_number : &mut u32) {
        let mut literal = String::new();
        while let Some(character) = self.current() {
            if character != '\n' {
                literal.push(character);
            } else { 
                *line_number += 1;
                break;
            }
            self.advance();
        }
        self.push_tokens(tokens,TokenType::Comments,0,literal,*line_number);
    }

    pub fn generate_tokens(&mut self) -> Option<Vec<Token>> {
        let mut tokens : Vec<Token> = Vec::new();
        let mut line = 1;
        while self.current <= self.end {
            match self.current() {
                Some(character) => {
                    match character {
                        '[' | '{' => {
                            self.push_tokens(&mut tokens,TokenType::LeftBrace,0,character.to_string(),line);
                            self.advance();
                        }
                        ']' | '}' => {
                            self.push_tokens(&mut tokens,TokenType::RightBrace,0,character.to_string(),line);
                            self.advance();
                        },
                        '(' => {
                            self.push_tokens(&mut tokens,TokenType::LeftParen,0,character.to_string(),line);
                            self.advance();
                        },
                        ')' => {
                            self.push_tokens(&mut tokens,TokenType::RightParen,0,character.to_string(),line);
                            self.advance();
                        }
                        ',' => {
                            self.push_tokens(&mut tokens,TokenType::Comma,0,character.to_string(),line);
                            self.advance();
                        }
                        '.' => {
                            self.push_tokens(&mut tokens,TokenType::Dot,0,character.to_string(),line);
                            self.advance();
                        }
                        '+' => {
                            self.push_tokens(&mut tokens,TokenType::Plus,0,character.to_string(),line);
                            self.advance();
                        }
                        '-' => {
                            self.push_tokens(&mut tokens,TokenType::Minus,0,character.to_string(),line);
                            self.advance();
                        }
                        ';' => {
                            self.push_tokens(&mut tokens,TokenType::Semicolon,0,character.to_string(),line);
                            self.advance();
                        }
                        '*' => {
                            self.push_tokens(&mut tokens,TokenType::Star,0,character.to_string(),line);
                            self.advance();
                        }
                        '!' => {
                            self.check_if_other_and_add(&mut tokens,character,TokenType::Equal,line);
                            self.advance();
                        }
                        '>' => {
                            self.check_if_other_and_add(&mut tokens,character,TokenType::Less,line);
                            self.advance();
                        }
                        '<'=> {
                            self.check_if_other_and_add(&mut tokens,character,TokenType::Greater,line);
                            self.advance();
                        }
                        '='=> { 
                            self.check_if_other_and_add(&mut tokens,character,TokenType::Equal,line);
                            self.advance();
                        }
                        '"' => { 
                            self.string_literal(&mut tokens,line);
                            self.advance();
                        },
                        '/' => {
                            if let Some(next_character) = self.peek() {
                                if next_character == '/' {
                                    self.comments(&mut tokens,&mut line);
                                } else {
                                    self.push_tokens(&mut tokens,TokenType::Slash,0,character.to_string(),line);
                                }
                            }
                            self.advance();
                        }
                        ' ' => self.advance(),
                        '\n' | '\t' => {
                            if character == '\n' {
                                line += 1;
                            }
                            self.advance();
                        }
                        _ => {
                            
                            if character.is_numeric() {
                                self.number(&mut tokens,line);
                            }
                            if character.is_alphabetic() {
                                self.literals(&mut tokens,line);
                            }
                        }
                    } 
                },
                None => {
                    break;
                }
            }
        }
       Some(tokens)
    }
}







