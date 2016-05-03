/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use unicode_segmentation::{ UnicodeSegmentation, UWordBoundIndices };

#[derive(Debug, PartialEq, Eq)]
pub enum TokenType<'a> {
    StartOfText,
    EndOfText,
    Newline,
    Other(&'a str),
}

#[derive(Debug, PartialEq, Eq)]
pub struct Token<'a> {
    tok: TokenType<'a>,
    byte_index: usize,
}

pub struct StringReader<'a> {
    eof: bool,
    start_emitted: bool,

    original_string: &'a str,

    word_bound_indices: UWordBoundIndices<'a>
}

impl<'a> Iterator for StringReader<'a> {
    type Item = Token<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        // Emit a start token (if start)
        if !self.start_emitted {
            self.start_emitted = true;
            Some(Token {
                tok: TokenType::StartOfText,
                byte_index: 0
            })
        } else {
            let next = self.word_bound_indices.next().map(|(index, tok)| {
                Token {
                    tok: TokenType::Other(tok),
                    byte_index: index,
                }
            });

            // If no EndOfText token was emitted at the end of the word bounds, emit one only once
            if next.is_none() && !self.eof {
                self.eof = true;
                Some(Token {
                    tok: TokenType::EndOfText,
                    byte_index: self.original_string.len(),
                })
            } else {
                next
            }
        }
    }
}

impl<'a> StringReader<'a> {
    pub fn new(source_string: &'a str) -> Self {
        let word_bounds = source_string.split_word_bound_indices();

        StringReader {
            eof: false,
            start_emitted: false,
            original_string: source_string,
            word_bound_indices: word_bounds
        }
    }
}

#[test]
fn test_simple() {
    let reader = StringReader::new("The quick brown fox");

    let actual = reader.collect::<Vec<Token>>();
    let expected: &[_] = &[
        Token { tok: TokenType::StartOfText, byte_index: 0 },
        Token { tok: TokenType::Other("The"),   byte_index: 0 },
        Token { tok: TokenType::Other(" "),     byte_index: 3 },
        Token { tok: TokenType::Other("quick"), byte_index: 4 },
        Token { tok: TokenType::Other(" "),     byte_index: 9 },
        Token { tok: TokenType::Other("brown"), byte_index: 10 },
        Token { tok: TokenType::Other(" "),     byte_index: 15 },
        Token { tok: TokenType::Other("fox"),   byte_index: 16 },
        Token { tok: TokenType::EndOfText,   byte_index: 19 },
    ];

    assert_eq!(actual, expected);
}

#[test]
fn test_simple_unicode() {
    let reader = StringReader::new("Ajuste la temperatura a 23 grados centígrados por favor");

    let actual = reader.collect::<Vec<Token>>();
    let expected: &[_] = &[
        Token { tok: TokenType::StartOfText,         byte_index: 0 },
        Token { tok: TokenType::Other("Ajuste"),      byte_index: 0 },
        Token { tok: TokenType::Other(" "),           byte_index: 6 },
        Token { tok: TokenType::Other("la"),          byte_index: 7 },
        Token { tok: TokenType::Other(" "),           byte_index: 9 },
        Token { tok: TokenType::Other("temperatura"), byte_index: 10 },
        Token { tok: TokenType::Other(" "),           byte_index: 21 },
        Token { tok: TokenType::Other("a"),           byte_index: 22 },
        Token { tok: TokenType::Other(" "),           byte_index: 23 },
        Token { tok: TokenType::Other("23"),          byte_index: 24 },
        Token { tok: TokenType::Other(" "),           byte_index: 26 },
        Token { tok: TokenType::Other("grados"),      byte_index: 27 },
        Token { tok: TokenType::Other(" "),           byte_index: 33 },
        Token { tok: TokenType::Other("centígrados"), byte_index: 34 },
        Token { tok: TokenType::Other(" "),           byte_index: 46 },
        Token { tok: TokenType::Other("por"),         byte_index: 47 },
        Token { tok: TokenType::Other(" "),           byte_index: 50 },
        Token { tok: TokenType::Other("favor"),       byte_index: 51 },
        Token { tok: TokenType::EndOfText,           byte_index: 56 },
    ];

    assert_eq!(actual, expected);
}

#[test]
fn test_complex_unicode() {
    let reader = StringReader::new("现在几点？");

    let actual = reader.collect::<Vec<Token>>();
    let expected: &[_] = &[
        Token { tok: TokenType::StartOfText, byte_index: 0 },
        Token { tok: TokenType::Other("\u{73b0}"), byte_index: 0 },
        Token { tok: TokenType::Other("\u{5728}"), byte_index: 3 },
        Token { tok: TokenType::Other("\u{51e0}"), byte_index: 6 },
        Token { tok: TokenType::Other("\u{70b9}"), byte_index: 9 },
        Token { tok: TokenType::Other("\u{ff1f}"), byte_index: 12 },
        Token { tok: TokenType::EndOfText, byte_index: 15 }
    ];
    assert_eq!(actual, expected);
}
