use nom::IResult;
use nom::character::complete::{alphanumeric1, multispace1};
    use nom::sequence::preceded;

fn space(input: &str) -> IResult<&str, &str> {
    multispace1(input)
}

fn word(input: &str) -> IResult<&str, &str> {
    alphanumeric1(input)
}

fn whole_word(input: &str) -> IResult<&str, &str> {
    preceded(space, word)(input)
}

pub mod command {
    use nom::IResult;
    use nom::branch::alt;
    use nom::bytes::complete::tag;
    use nom::sequence::{preceded, pair, terminated};
    use nom::combinator::{cut, map, opt, success};
    use nom::multi::{many1, many_till};
    use crate::command::Command;
    use crate::item::ItemString;
    use crate::parser::{space, whole_word};
    pub fn parse_command(input: &str) -> IResult<&str, Command> {
        alt((
            parse_take_from,
            parse_take,
            parse_put,
            parse_inventory,
            parse_look,
            parse_unknown
        ))(input)
    }
    
    fn parse_take(input: &str) -> IResult<&str, Command> {
        map(
            preceded(
                take,
                cut(
                    many1(
                        whole_word
                    )
                )
            ),
            |words| Command::Take(ItemString::from_refs(words))
        )(input)
    }

    fn parse_take_from(input: &str) -> IResult<&str, Command> {
        map(
            preceded(
                take,
                pair(
                    map(
                        many_till(
                            whole_word,
                            preceded(
                                space,
                                from
                            )
                        ),
                        |(words, _)| words
                    ),
                    cut(
                        many1(
                            whole_word
                        )
                    )
                )
            ),
            |(item, source)| Command::TakeFrom {item: ItemString::from_refs(item), source: ItemString::from_refs(source)}
        )(input)
    }

    fn take(input: &str) -> IResult<&str, &str> {
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
        ))(input)
    }

    fn from(input: &str) -> IResult<&str, &str> {
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
        ))(input)
    }

    fn parse_put(input: &str) -> IResult<&str, Command> {
        map(
            preceded(
                put,
                pair(
                    map(
                        many_till(
                            whole_word,
                            preceded(
                                space,
                                into
                            )
                        ),
                        |(words, _)| words
                    ),
                    cut(
                        many1(
                            whole_word
                        )
                    )
                )
            ),
            |(item, destination)| Command::Put {item: ItemString::from_refs(item), destination: ItemString::from_refs(destination)}
        )(input)
    }

    fn put(input: &str) -> IResult<&str, &str> {
        alt((
            tag("put"),
            tag("move"),
            tag("add")
        ))(input)
    }

    fn into(input: &str) -> IResult<&str, &str> {
        alt((
            tag("into"),
            tag("onto"),
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
            )
        ))(input)
    }

    fn parse_inventory(input: &str) -> IResult<&str, Command> {
        map(
            inventory,
            |_| Command::Inventory
        )(input)
    }

    fn inventory(input: &str) -> IResult<&str, &str> {
        alt((
            tag("inventory"),
            tag("items")
        ))(input)
    }

    fn parse_look(input: &str) -> IResult<&str, Command> {
        preceded(
            look,
            map(
                alt((
                    map(
                        preceded(
                            opt(
                                preceded(
                                    space,
                                    in_at_on
                                )
                            ),
                            many1(
                                whole_word
                            )
                        ),
                        |words| Some(ItemString::from_refs(words))
                    ),
                    success(None)
                )),
                |option| Command::Look(option)
            )
        )(input)
    }

    fn look(input: &str) -> IResult<&str, &str> {
        alt((
            tag("look"),
            tag("l"),
            tag("inspect")
        ))(input)
    }

    fn in_at_on(input: &str) -> IResult<&str, &str> {
        alt((
            tag("at"),
            tag("in"),
            tag("on")
        ))(input)
    }
    
    fn parse_unknown(_input: &str) -> IResult<&str, Command> {
        Ok(("", Command::Unknown))
    }
}