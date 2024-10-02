

#[derive(PartialEq, Clone, Debug)]
enum Token {
    BTC(Option<f64>),
    USDT(Option<f64>),
    SOL(Option<f64>),
    BNB(Option<f64>),
    ETH(Option<f64>)
}

impl Token {
    fn amount(&self) -> Option<f64> {
        match self {
            Token::BTC(value) => *value,
            Token::USDT(value) => *value,
            Token::SOL(value) => *value,
            Token::BNB(value) => *value,
            Token::ETH(value) => *value,
        }
    }

    fn extract_amount(&self) -> f64 {
        match self.amount() {
            None => 0.0,
            Some(value) => value
        }
    }

    fn increase_amount(&self, inc_amount: f64) -> Token {
        match self {
            Token::BTC(_value) => Token::BTC(Some(inc_amount + self.extract_amount())),
            Token::USDT(_value) => Token::USDT(Some(inc_amount + self.extract_amount())),
            Token::SOL(_value) => Token::SOL(Some(inc_amount + self.extract_amount())),
            Token::BNB(_value) => Token::BNB(Some(inc_amount + self.extract_amount())),
            Token::ETH(_value) => Token::ETH(Some(inc_amount + self.extract_amount())),
        }
    }

    
    fn type_of(&self) -> u8 {
        match self {
            Token::BTC(_value) => 0,
            Token::USDT(_value) => 1,
            Token::SOL(_value) => 2,
            Token::BNB(_value) => 3,
            Token::ETH(_value) => 4,
        }
    }
}

#[derive(Clone)]
struct Wallet {
    content: Vec<Token>
}

impl Wallet {
    fn new() -> Self {
        Wallet {content: vec![]}
    }

    fn add(&mut self, token: Token) {
        match token.amount() {
            None => (),
            Some(new_amount) => {
                let old = self.content.clone();
                self.content = vec![];

                for existing_token in old {
                    if existing_token.type_of() == token.type_of() {
                        self.content.push(existing_token.increase_amount(new_amount));
                        return
                    } else {
                        self.content.push(existing_token);
                    }
                }
                self.content.push(token);
            }
        };

    }
}

fn main() {
    let mut wallet = Wallet::new();
    let token = Token::BTC(Some(0.001));
    wallet.add(token);
    let token = Token::BTC(Some(0.02));
    wallet.add(token);
    for existing_token in wallet.content {
        println!("{:?}", existing_token)
    }
}
