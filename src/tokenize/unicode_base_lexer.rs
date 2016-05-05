/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use std::fmt;
use unicode_segmentation::{ UnicodeSegmentation, UWordBoundIndices };

#[derive(Debug, PartialEq, Eq)]
pub enum TokenType<'a> {
    StartOfText,
    EndOfText,
    Str(&'a str),
}

#[derive(PartialEq, Eq)]
pub struct Token<'a> {
    pub token: TokenType<'a>,
    pub byte_index: usize,
}

impl<'a> fmt::Display for Token<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Tok({:?}, {})", self.token, self.byte_index)
    }
}

impl<'a> fmt::Debug for Token<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Tok({:?}, {})", self.token, self.byte_index)
    }
}

impl<'a> Token<'a> {
    pub const fn new(typ: TokenType<'a>, byte_index: usize) -> Self {
        Token {
            token: typ,
            byte_index: byte_index,
        }
    }
}

pub struct UnicodeLexer<'a> {
    eof: bool,
    start_emitted: bool,

    original_string: &'a str,

    word_bound_indices: UWordBoundIndices<'a>
}

impl<'a> Iterator for UnicodeLexer<'a> {
    type Item = Token<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        // Emit a start token (if start)
        if !self.start_emitted {
            self.start_emitted = true;
            Some(Token {
                token: TokenType::StartOfText,
                byte_index: 0
            })
        } else {
            let next = self.word_bound_indices.next().map(|(index, tok)| {
                Token {
                    token: TokenType::Str(tok),
                    byte_index: index,
                }
            });

            // If no EndOfText token was emitted at the end of the word bounds, emit one only once
            if next.is_none() && !self.eof {
                self.eof = true;
                Some(Token {
                    token: TokenType::EndOfText,
                    byte_index: self.original_string.len(),
                })
            } else {
                next
            }
        }
    }
}

pub fn filter_word_boundaries<'a>(tok: &Token<'a>) -> bool {
    // TODO: Use the actual unicode word boundary rules

    match tok.token {
        TokenType::Str(word) => {
            !(
                word == " " ||
                word == "\n"
            )
        }
        _ => true
    }
}

impl<'a> UnicodeLexer<'a> {
    pub fn new(source_string: &'a str) -> Self {
        let word_bounds = source_string.split_word_bound_indices();

        UnicodeLexer {
            eof: false,
            start_emitted: false,
            original_string: source_string,
            word_bound_indices: word_bounds
        }
    }
}

#[test]
fn test_simple() {
    let reader = UnicodeLexer::new("The quick brown fox");

    let actual = reader.filter(filter_word_boundaries)
                       .collect::<Vec<Token>>();

    let expected: &[_] = &[
        Token { token: TokenType::StartOfText, byte_index: 0 },
        Token { token: TokenType::Str("The"),   byte_index: 0 },
        Token { token: TokenType::Str("quick"), byte_index: 4 },
        Token { token: TokenType::Str("brown"), byte_index: 10 },
        Token { token: TokenType::Str("fox"),   byte_index: 16 },
        Token { token: TokenType::EndOfText,   byte_index: 19 },
    ];

    assert_eq!(actual, expected);
}

#[test]
fn test_simple_unicode() {
    let reader = UnicodeLexer::new("Ajuste la temperatura a 23 grados centígrados por favor");

    let actual = reader.filter(filter_word_boundaries)
                       .collect::<Vec<Token>>();

    let expected: &[_] = &[
        Token { token: TokenType::StartOfText,         byte_index: 0 },
        Token { token: TokenType::Str("Ajuste"),      byte_index: 0 },
        Token { token: TokenType::Str("la"),          byte_index: 7 },
        Token { token: TokenType::Str("temperatura"), byte_index: 10 },
        Token { token: TokenType::Str("a"),           byte_index: 22 },
        Token { token: TokenType::Str("23"),          byte_index: 24 },
        Token { token: TokenType::Str("grados"),      byte_index: 27 },
        Token { token: TokenType::Str("centígrados"), byte_index: 34 },
        Token { token: TokenType::Str("por"),         byte_index: 47 },
        Token { token: TokenType::Str("favor"),       byte_index: 51 },
        Token { token: TokenType::EndOfText,           byte_index: 56 },
    ];

    assert_eq!(actual, expected);
}

#[test]
fn test_complex_unicode() {
    let reader = UnicodeLexer::new("现在几点？");

    let actual = reader.collect::<Vec<Token>>();
    let expected: &[_] = &[
        Token { token: TokenType::StartOfText, byte_index: 0 },
        Token { token: TokenType::Str("\u{73b0}"), byte_index: 0 },
        Token { token: TokenType::Str("\u{5728}"), byte_index: 3 },
        Token { token: TokenType::Str("\u{51e0}"), byte_index: 6 },
        Token { token: TokenType::Str("\u{70b9}"), byte_index: 9 },
        Token { token: TokenType::Str("\u{ff1f}"), byte_index: 12 },
        Token { token: TokenType::EndOfText, byte_index: 15 }
    ];
    assert_eq!(actual, expected);
}
