#[derive(Clone, Debug)]
enum Token {
    BTC(Option<f64>),
    USDT(Option<f64>),
    SOL(Option<f64>),
    BNB(Option<f64>),
    ETH(Option<f64>),
}

impl Token {
    fn probable_amount(&self) -> Option<f64> {
        match self {
            Token::BTC(value) => *value,
            Token::USDT(value) => *value,
            Token::SOL(value) => *value,
            Token::BNB(value) => *value,
            Token::ETH(value) => *value,
        }
    }

    fn amount(&self) -> f64 {
        match self.probable_amount() {
            None => 0.0,
            Some(value) => value,
        }
    }

    fn new(type_value: u8, amount: f64) -> Token {
        match type_value {
            0 => Token::BTC(Some(amount)),
            1 => Token::USDT(Some(amount)),
            2 => Token::SOL(Some(amount)),
            3 => Token::BNB(Some(amount)),
            _ => Token::ETH(Some(amount))
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
    content: Vec<Token>,
}

impl Wallet {
    fn new() -> Self {
        Wallet { content: vec![] }
    }

    fn add(&mut self, new_token: Token) {
        let token_type = new_token.type_of();
        for (index, token) in self.content.iter().enumerate() {
            if token.type_of() == token_type {
                self.content[index] = Token::new(token_type, token.amount() + new_token.amount());
                return;
            }
        }
        self.content.push(new_token);
    }

    fn _if_let_add(&mut self, new_token: Token) {
        let token_type = new_token.type_of();
        for (index, _) in self.content.iter().enumerate() {
            if self.content[index].type_of() == token_type {
                if let Some(element) = self.content.get_mut(index) {
                    *element = Token::new(token_type, element.amount() + new_token.amount())
                }
                return;
            }
        }
        self.content.push(new_token);
    }
}

fn main() {
    // Note: This example is for practicing with a mixture of enums, structs and special enums such as Option
    // It could be much simpler but the goal was something else.
    let mut wallet = Wallet::new();
    let token = Token::BTC(Some(0.001));
    wallet.add(token);
    let token = Token::BTC(Some(0.02));
    wallet.add(token);
    wallet.add(Token::ETH(Some(5.0)));
    for existing_token in wallet.content {
        println!("{:?}", existing_token)
    }
}
