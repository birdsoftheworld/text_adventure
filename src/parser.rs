use nom::{IResult};
use nom::character::complete::{alphanumeric1, multispace1};
use nom::sequence::preceded;
use nom::combinator::{peek, eof};
use nom::branch::alt;

fn space(input: &str) -> IResult<&str, &str> {
    multispace1(input)
}

fn word(input: &str) -> IResult<&str, &str> {
    alphanumeric1(input)
}

fn whole_word(input: &str) -> IResult<&str, &str> {
    preceded(space, word)(input)
}

fn word_ending(input: &str) -> IResult<&str, &str> {
    peek(alt((space, eof)))(input)
}

pub mod command {
    use nom::{Err, IResult};
    use nom::branch::alt;
    use nom::bytes::complete::tag;
    use nom::sequence::{preceded, terminated, tuple};
    use nom::combinator::{cut, map, opt, peek};
    use nom::multi::{many1, many_till};
    use crate::command::Command;
    use crate::item::NameString;
    use crate::parser::{space, whole_word, word_ending};
    use crate::parser::command::error::{CommandParseError, CommandParseType};

    pub mod error {
        use nom::error::{Error, ErrorKind, ParseError};

        #[derive(Debug)]
        pub enum CommandParseType {
            Take,
            TakeFrom,
            Put,
            Inventory,
            Look,
            Unknown
        }

        #[derive(Debug)]
        pub enum CommandParseError<I> {
            MissingArg(CommandParseType, usize),
            Unknown,
            Nom(I, ErrorKind)
        }

        impl<I> ParseError<I> for CommandParseError<I> {
            fn from_error_kind(input: I, kind: ErrorKind) -> Self {
                CommandParseError::Nom(input, kind)
            }

            fn append(_: I, _: ErrorKind, other: Self) -> Self {
                other
            }
        }

        impl<'a> From<Error<&'a str>> for CommandParseError<&'a str> {
            fn from(e: Error<&'a str>) -> Self {
                CommandParseError::Nom(e.input, e.code)
            }
        }
    }

    pub fn parse_full_command(input: &str) -> Result<Command, CommandParseError<&str>> {
        let res = parse_command(input);
        match res {
            Ok((_, command)) => {
                Ok(command)
            }
            Err(Err::Failure(a)) => {
                Err(a)
            }
            _ => {
                unimplemented!()
            }
        }
    }

    fn parse_command<'a>(input: &'a str) -> IResult<&str, Command, CommandParseError<&'a str>> {
        alt((
            parse_take_from,
            parse_take,
            parse_put,
            parse_inventory,
            parse_look,
            parse_unknown
        ))(input)
    }
    
    fn parse_take(input: &str) -> IResult<&str, Command, CommandParseError<&str>> {
        let (i, _) = take(input).map_err(Err::convert)?;
        let res = many1(whole_word)(i);
        if let Err(_) = res {
            return Err(
                Err::Failure(CommandParseError::MissingArg(CommandParseType::Take, 0))
            )
        }
        let (i, name) = res.unwrap();
        let item = NameString::from_refs(name);
        Ok((i, Command::Take(item)))
    }

    fn parse_take_from(input: &str) -> IResult<&str, Command, CommandParseError<&str>> {
        let (i, _) = take(input).map_err(Err::convert)?;
        let (i, (first, _)) = many_till(
            whole_word,
            preceded(
                space,
                from
            )
        )(i).map_err(Err::convert)?;
        peek(space)(i).map_err(Err::convert)?;
        let res = many1(whole_word)(i);
        if let Err(_) = res {
            return Err(Err::Failure(CommandParseError::MissingArg(CommandParseType::TakeFrom, 1)))
        }
        let (i, second) = res.unwrap();
        let item = NameString::from_refs(first);
        let source = NameString::from_refs(second);
        
        Ok((i, Command::TakeFrom {item, source}))
    }

    fn take(input: &str) -> IResult<&str, &str> {
        terminated(
            alt((
                tag("take"),
                tag("grab"),
                tag("t"),
                terminated(
                    tag("pick"),
                    preceded(
                        space,
                        tag("up")
                    )
                )
            )),
            word_ending
        )(input)
    }

    fn from(input: &str) -> IResult<&str, &str> {
        terminated(
            alt((
                tag("from"),
                tag("outta"),
                terminated(
                    tag("out"),
                    cut(preceded(
                        space,
                        alt((
                            tag("of"),
                            tag("from")
                        ))
                    ))
                )
            )),
            word_ending
        )(input)
    }

    fn parse_put(input: &str) -> IResult<&str, Command, CommandParseError<&str>> {
        let (i, _) = put(input).map_err(Err::convert)?;
        match peek(whole_word)(i) {
            Err(_) => return Err(
                Err::Failure(CommandParseError::MissingArg(CommandParseType::Put, 0))
            ),
            _ => ()
        };
    
        let res = many_till(
            whole_word,
            preceded(
                space,
                into
            )
        )(i);
    
        let (i, (first, _)) = match res {
            Err(_) => return Err(
                Err::Failure(CommandParseError::MissingArg(CommandParseType::Put, 1))
            ),
            Ok(v) => v
        };
        
        let res = many1(whole_word)(i);
        let (i, second) = match res {
            Err(_) => return Err(
                Err::Failure(CommandParseError::MissingArg(CommandParseType::Put, 1))
            ),
            Ok(v) => v
        };

        let item = NameString::from_refs(first);
        let destination = NameString::from_refs(second);
        
        Ok((i, Command::Put {item, destination}))
    }

    fn put(input: &str) -> IResult<&str, &str> {
        terminated(
            alt((
                tag("put"),
                tag("move"),
                tag("add")
            )),
            word_ending
        )(input)
    }

    fn into(input: &str) -> IResult<&str, &str> {
        terminated(
            alt((
                tag("into"),
                tag("onto"),
                terminated(
                    tag("inside"),
                    opt(
                        preceded(
                            space,
                            tag("of")
                        )
                    )
                ),
                terminated(
                    alt((
                        tag("in"),
                        tag("on")
                    )),
                    opt(
                        preceded(
                            space,
                            tag("to")
                        )
                    )
                ),
            )),
            word_ending
        )(input)
    }

    fn parse_inventory(input: &str) -> IResult<&str, Command, CommandParseError<&str>> {
        map(
            inventory,
            |_| Command::Inventory
        )(input).map_err(Err::convert)
    }

    fn inventory(input: &str) -> IResult<&str, &str> {
        terminated(
            alt((
                tag("inventory"),
                tag("items")
            )),
            word_ending
        )(input)
    }

    fn parse_look(input: &str) -> IResult<&str, Command, CommandParseError<&str>> {
        let (i, _) = look(input).map_err(Err::convert)?;

        let (i, in_at_on) = opt(
            preceded(
                space,
                in_at_on
            )
        )(i).unwrap();
        
        let res = opt(many1(whole_word))(i).unwrap();
        
        let target = match res {
            (i, None) => match in_at_on {
                None => (i, Command::Look(None)),
                Some(_) => return Err(Err::Failure(CommandParseError::MissingArg(CommandParseType::Look, 0)))
            },
            (i, Some(words)) => (i, Command::Look(Some(NameString::from_refs(words))))
        };

        Ok(target)
    }

    fn look(input: &str) -> IResult<&str, &str> {
        terminated(
            alt((
                tag("look"),
                tag("l"),
                tag("inspect")
            )),
            word_ending
        )(input)
    }

    fn in_at_on(input: &str) -> IResult<&str, &str> {
        terminated(
            alt((
                terminated(
                    tag("inside"),
                    opt(
                        preceded(
                            space,
                            tag("of")
                        )
                    )
                ),
                terminated(
                    tag("on"),
                    opt(
                        tuple((
                            space,
                            tag("top"),
                            space,
                            tag("of")
                        ))
                    )
                ),
                tag("at"),
                tag("in"),
            )),
            word_ending
        )(input)
    }
    
    fn parse_unknown(_: &str) -> IResult<&str, Command, CommandParseError<&str>> {
        Err(Err::Failure(CommandParseError::Unknown))
    }
}