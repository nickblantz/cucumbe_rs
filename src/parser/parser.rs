use crate::parser::dialect::Dialect;
use crate::parser::io::InputStream;
// use nom::IResult;
// use nom::number::complete::be_u16;
// use nom::bytes::complete::take;


// pub fn length_value(input: &[u8]) -> IResult<&[u8],&[u8]> {
//     let (input, length) = be_u16(input)?;
//     take(length)(input)
// }

pub struct GherkinParser {
    dialect: Dialect,
    input_stream: InputStream,
}

impl GherkinParser {
    pub fn create(dialect: Dialect, input_stream: InputStream) -> GherkinParser {
        GherkinParser { dialect: dialect, input_stream: input_stream }
    }

    //pub fn parse(input_stream: InputStream)
}
