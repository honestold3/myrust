//pub fn kankan(){
//    struct Context(&str);
//
//    struct Parser {
//        context: &Context,
//    }
//
//    impl Parser {
//        fn parse(&self) -> Result<(),&str> {
//            Err(&self.context.0[1..])
//        }
//    }
//}

pub fn kankan1(){

}

//struct Context<'a>(&'a str);
//
//struct Parser<'a> {
//    context: &'a Context<'a>,
//}
//
//impl<'a> Parser<'a> {
//    //fn parse(&self) -> Result<(), &str> {
//    fn parse<'a>(&'a self) -> Result<(), &'a str> {
//        Err(&self.context.0[1..])
//    }
//}

struct Context<'s>(&'s str);

struct Parser<'c, 's: 'c> { //'s的lifetime 长于 'c的lifetime
    context: &'c Context<'s>,
}

impl<'c, 's> Parser<'c, 's> {
    fn parse(&self) -> Result<(), &'s str> {
        Err(&self.context.0[1..])
    }
}

fn parse_context(context: Context) -> Result<(), &str> {
    Parser { context: &context }.parse()
}