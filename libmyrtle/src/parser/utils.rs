use nom::character::complete::multispace0;
use nom::error::ParseError;
use nom::IResult;
use nom::sequence::delimited;

pub fn ws<'a, F, O, E: ParseError<&'a str>>(inner: F) -> impl FnMut(&'a str) -> IResult<&'a str, O, E>
    where
        F: FnMut(&'a str) -> IResult<&'a str, O, E>,
{
    delimited(multispace0, inner, multispace0)
}
