/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use super::unicode_base_lexer::{ TokenType, Token };
use regex::Regex;

lazy_static!{
    static ref _ENG_ENCLITIC_REX: Regex = Regex::new(r"(.*)('s|'ll|'re|'ve|n't|'m|'ve|'d)$").unwrap();
}

pub fn english_clitic_expand<'a>(token: Token<'a>) -> Vec<Token<'a>> {

    match token.token {
        TokenType::Str(word) => {
            if let Some(captures) = _ENG_ENCLITIC_REX.captures(word) {
                if let Some(first_cap) = captures.at(1) {
                    if let Some(second_cap) = captures.at(2) {
                        return vec![
                            Token::new(
                                TokenType::Str(first_cap),
                                token.byte_index
                            ),
                            Token::new(
                                TokenType::Str(second_cap),
                                token.byte_index + captures.pos(2).unwrap().0
                            ),
                        ]
                    }
                }
            }

            vec![token]
        },
        _ => vec![token]
    }

}


#[test]
fn test_english_clitic_expansion() {
    use super::unicode_base_lexer::{ filter_word_boundaries, UnicodeLexer };

    let reader = UnicodeLexer::new("i'll we're i've couldn't ain't i'm could've they'd");

    let actual = reader.flat_map(english_clitic_expand)
                       .filter(filter_word_boundaries)
                       .collect::<Vec<Token>>();

    let expected: &[_] = &[
        Token::new(TokenType::StartOfText, 0),
        Token::new(TokenType::Str("i"), 0),
        Token::new(TokenType::Str("'ll"), 1),
        Token::new(TokenType::Str("we"), 5),
        Token::new(TokenType::Str("'re"), 7),
        Token::new(TokenType::Str("i"), 11),
        Token::new(TokenType::Str("'ve"), 12),
        Token::new(TokenType::Str("could"), 16),
        Token::new(TokenType::Str("n't"), 21),
        Token::new(TokenType::Str("ai"), 25),
        Token::new(TokenType::Str("n't"), 27),
        Token::new(TokenType::Str("i"), 31),
        Token::new(TokenType::Str("'m"), 32),
        Token::new(TokenType::Str("could"), 35),
        Token::new(TokenType::Str("'ve"), 40),
        Token::new(TokenType::Str("they"), 44),
        Token::new(TokenType::Str("'d"), 48),
        Token::new(TokenType::EndOfText, 50),
    ];

    assert_eq!(actual, expected);

}
