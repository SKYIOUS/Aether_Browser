use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Doctype(DoctypeToken),
    StartTag(TagToken),
    EndTag(TagToken),
    Comment(String),
    Character(char),
    Whitespace(char),
    Eof,
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct DoctypeToken {
    pub name: Option<String>,
    pub public_id: Option<String>,
    pub system_id: Option<String>,
    pub force_quirks: bool,
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct TagToken {
    pub name: String,
    pub self_closing: bool,
    pub attrs: HashMap<String, String>,
}

// ── Tokenizer State Machine ──────────────────────────────────

pub struct Tokenizer {
    input: String,
    pos: usize,
    pub tokens: Vec<Token>,
    state: State,
    return_state: State,
    // Buffers
    temp: String,
    last_start_tag: Option<String>,
    pub is_foreign: bool,
    // Character reference state
    char_ref_code: Option<u32>,
}

#[derive(Debug, Clone, PartialEq)]
enum State {
    Data,
    RcData,
    RawText,
    ScriptData,
    PlainText,
    TagOpen,
    EndTagOpen,
    TagName,
    RcDataLessThanSign,
    RcDataEndTagOpen,
    RcDataEndTagName,
    ScriptDataLessThanSign,
    ScriptDataEndTagOpen,
    ScriptDataEndTagName,
    ScriptDataEscapeStart,
    ScriptDataEscapeStartDash,
    ScriptDataEscaped,
    ScriptDataEscapedDash,
    ScriptDataEscapedDashDash,
    ScriptDataEscapedLessThanSign,
    ScriptDataEscapedEndTagOpen,
    ScriptDataEscapedEndTagName,
    ScriptDataDoubleEscapeStart,
    ScriptDataDoubleEscaped,
    ScriptDataDoubleEscapedDash,
    ScriptDataDoubleEscapedDashDash,
    ScriptDataDoubleEscapedLessThanSign,
    ScriptDataDoubleEscapeEnd,
    BeforeAttributeName,
    AttributeName,
    AfterAttributeName,
    BeforeAttributeValue,
    AttributeValueDoubleQuoted,
    AttributeValueSingleQuoted,
    AttributeValueUnquoted,
    AfterAttributeValueQuoted,
    SelfClosingStartTag,
    BogusComment,
    MarkupDeclarationOpen,
    CommentStart,
    CommentStartDash,
    Comment,
    CommentLessThanSign,
    CommentLessThanSignBang,
    CommentLessThanSignBangDash,
    CommentLessThanSignBangDashDash,
    CommentEndDash,
    CommentEnd,
    CommentEndBang,
    Doctype,
    BeforeDoctypeName,
    DoctypeName,
    AfterDoctypeName,
    AfterDoctypePublicKeyword,
    BeforeDoctypePublicIdentifier,
    DoctypePublicIdentifierDoubleQuoted,
    DoctypePublicIdentifierSingleQuoted,
    AfterDoctypePublicIdentifier,
    BetweenDoctypePublicAndSystemIdentifiers,
    BeforeDoctypeSystemIdentifier,
    DoctypeSystemIdentifierDoubleQuoted,
    DoctypeSystemIdentifierSingleQuoted,
    AfterDoctypeSystemIdentifier,
    BogusDoctype,
    CdataSection,
    CdataSectionBracket,
    CdataSectionEnd,
    CharacterReference,
    NamedCharacterReference,
    AmbiguousAmpersand,
    NumericCharacterReference,
    HexademicalCharacterReferenceStart,
    DecimalCharacterReferenceStart,
    HexademicalCharacterReference,
    DecimalCharacterReference,
    NumericCharacterReferenceEnd,
}

impl Tokenizer {
    pub fn new(input: String) -> Self {
        Self {
            input,
            pos: 0,
            tokens: Vec::new(),
            state: State::Data,
            return_state: State::Data,
            temp: String::new(),
            last_start_tag: None,
            is_foreign: false,
            char_ref_code: None,
        }
    }

    fn next_char(&self) -> char {
        self.input[self.pos..].chars().next().unwrap_or('\0')
    }

    fn consume_char(&mut self) -> char {
        let c = self.next_char();
        self.pos += c.len_utf8();
        c
    }

    fn eof(&self) -> bool {
        self.pos >= self.input.len()
    }

    fn consume_while<F>(&mut self, test: F) -> String
    where
        F: Fn(char) -> bool,
    {
        let mut result = String::new();
        while !self.eof() && test(self.next_char()) {
            result.push(self.consume_char());
        }
        result
    }

    fn consume_ascii_alpha_while(&mut self) -> String {
        self.consume_while(|c| c.is_ascii_alphabetic())
    }

    fn reconsume(&mut self, state: State) {
        self.state = state;
    }

    fn emit_token(&mut self, token: Token) {
        self.tokens.push(token);
    }

    fn emit_char(&mut self, c: char) {
        if c.is_whitespace() && (c == ' ' || c == '\t' || c == '\n' || c == '\r' || c == '\x0C') {
            self.emit_token(Token::Whitespace(c));
        } else {
            self.emit_token(Token::Character(c));
        }
    }

    fn emit_string(&mut self, s: &str) {
        for c in s.chars() {
            self.emit_char(c);
        }
    }

    fn flush_temp(&mut self) -> String {
        let s = self.temp.clone();
        self.temp.clear();
        s
    }

    fn is_ascii_upper(&self, c: char) -> bool {
        c.is_ascii_uppercase()
    }

    fn to_lower(&self, c: char) -> char {
        c.to_ascii_lowercase()
    }

    fn consume_tag_name(&mut self) -> String {
        self.consume_while(|c| c.is_alphanumeric() || c == '-' || c == '_' || c == ':' || c == '.')
    }

    pub fn tokenize(&mut self) {
        loop {
            if self.eof() {
                self.emit_token(Token::Eof);
                return;
            }
            match self.state.clone() {
                State::Data => self.data_state(),
                State::TagOpen => self.tag_open_state(),
                State::EndTagOpen => self.end_tag_open_state(),
                State::TagName => self.tag_name_state(),
                State::BeforeAttributeName => self.before_attribute_name_state(),
                State::AttributeName => self.attribute_name_state(),
                State::AfterAttributeName => self.after_attribute_name_state(),
                State::BeforeAttributeValue => self.before_attribute_value_state(),
                State::AttributeValueDoubleQuoted => self.attribute_value_double_quoted_state(),
                State::AttributeValueSingleQuoted => self.attribute_value_single_quoted_state(),
                State::AttributeValueUnquoted => self.attribute_value_unquoted_state(),
                State::AfterAttributeValueQuoted => self.after_attribute_value_quoted_state(),
                State::SelfClosingStartTag => self.self_closing_start_tag_state(),
                State::BogusComment => self.bogus_comment_state(),
                State::MarkupDeclarationOpen => self.markup_declaration_open_state(),
                State::CommentStart => self.comment_start_state(),
                State::CommentStartDash => self.comment_start_dash_state(),
                State::Comment => self.comment_state(),
                State::CommentLessThanSign => self.comment_less_than_sign_state(),
                State::CommentLessThanSignBang => self.comment_less_than_sign_bang_state(),
                State::CommentLessThanSignBangDash => self.comment_less_than_sign_bang_dash_state(),
                State::CommentLessThanSignBangDashDash => self.comment_less_than_sign_bang_dash_dash_state(),
                State::CommentEndDash => self.comment_end_dash_state(),
                State::CommentEnd => self.comment_end_state(),
                State::CommentEndBang => self.comment_end_bang_state(),
                State::Doctype => self.doctype_state(),
                State::BeforeDoctypeName => self.before_doctype_name_state(),
                State::DoctypeName => self.doctype_name_state(),
                State::AfterDoctypeName => self.after_doctype_name_state(),
                State::AfterDoctypePublicKeyword => self.after_doctype_public_keyword_state(),
                State::BeforeDoctypePublicIdentifier => self.before_doctype_public_identifier_state(),
                State::DoctypePublicIdentifierDoubleQuoted => self.doctype_public_identifier_double_quoted_state(),
                State::DoctypePublicIdentifierSingleQuoted => self.doctype_public_identifier_single_quoted_state(),
                State::AfterDoctypePublicIdentifier => self.after_doctype_public_identifier_state(),
                State::BetweenDoctypePublicAndSystemIdentifiers => self.between_doctype_public_and_system_identifiers_state(),
                State::BeforeDoctypeSystemIdentifier => self.before_doctype_system_identifier_state(),
                State::DoctypeSystemIdentifierDoubleQuoted => self.doctype_system_identifier_double_quoted_state(),
                State::DoctypeSystemIdentifierSingleQuoted => self.doctype_system_identifier_single_quoted_state(),
                State::AfterDoctypeSystemIdentifier => self.after_doctype_system_identifier_state(),
                State::BogusDoctype => self.bogus_doctype_state(),
                State::CdataSection => self.cdata_section_state(),
                State::CdataSectionBracket => self.cdata_section_bracket_state(),
                State::CdataSectionEnd => self.cdata_section_end_state(),
                State::RcData => self.rcdata_state(),
                State::RawText => self.raw_text_state(),
                State::ScriptData => self.script_data_state(),
                State::PlainText => self.plain_text_state(),
                State::RcDataLessThanSign => self.rcdata_less_than_sign_state(),
                State::RcDataEndTagOpen => self.rcdata_end_tag_open_state(),
                State::RcDataEndTagName => self.rcdata_end_tag_name_state(),
                State::ScriptDataLessThanSign => self.script_data_less_than_sign_state(),
                State::ScriptDataEndTagOpen => self.script_data_end_tag_open_state(),
                State::ScriptDataEndTagName => self.script_data_end_tag_name_state(),
                State::ScriptDataEscapeStart => self.script_data_escape_start_state(),
                State::ScriptDataEscapeStartDash => self.script_data_escape_start_dash_state(),
                State::ScriptDataEscaped => self.script_data_escaped_state(),
                State::ScriptDataEscapedDash => self.script_data_escaped_dash_state(),
                State::ScriptDataEscapedDashDash => self.script_data_escaped_dash_dash_state(),
                State::ScriptDataEscapedLessThanSign => self.script_data_escaped_less_than_sign_state(),
                State::ScriptDataEscapedEndTagOpen => self.script_data_escaped_end_tag_open_state(),
                State::ScriptDataEscapedEndTagName => self.script_data_escaped_end_tag_name_state(),
                State::ScriptDataDoubleEscapeStart => self.script_data_double_escape_start_state(),
                State::ScriptDataDoubleEscaped => self.script_data_double_escaped_state(),
                State::ScriptDataDoubleEscapedDash => self.script_data_double_escaped_dash_state(),
                State::ScriptDataDoubleEscapedDashDash => self.script_data_double_escaped_dash_dash_state(),
                State::ScriptDataDoubleEscapedLessThanSign => self.script_data_double_escaped_less_than_sign_state(),
                State::ScriptDataDoubleEscapeEnd => self.script_data_double_escape_end_state(),
                State::CharacterReference => self.character_reference_state(),
                State::NamedCharacterReference => self.named_character_reference_state(),
                State::AmbiguousAmpersand => self.ambiguous_ampersand_state(),
                State::NumericCharacterReference => self.numeric_character_reference_state(),
                State::HexademicalCharacterReferenceStart => self.hexademical_character_reference_start_state(),
                State::DecimalCharacterReferenceStart => self.decimal_character_reference_start_state(),
                State::HexademicalCharacterReference => self.hexademical_character_reference_state(),
                State::DecimalCharacterReference => self.decimal_character_reference_state(),
                State::NumericCharacterReferenceEnd => self.numeric_character_reference_end_state(),
            }
        }
    }

    fn data_state(&mut self) {
        loop {
            let c = self.next_char();
            match c {
                '&' => { self.consume_char(); self.state = State::CharacterReference; self.char_ref_code = None; return; }
                '<' => { self.consume_char(); self.state = State::TagOpen; return; }
                '\0' => { if !self.eof() { self.consume_char(); self.emit_char('\0'); } else { return; } }
                _ => { self.consume_char(); self.emit_char(c); }
            }
        }
    }

    fn tag_open_state(&mut self) {
        let c = self.next_char();
        if c == '/' { self.consume_char(); self.state = State::EndTagOpen; return; }
        if c.is_ascii_alphabetic() { self.temp.clear(); self.state = State::TagName; return; }
        if c == '?' { self.consume_char(); self.state = State::BogusComment; return; }
        if c == '!' { self.consume_char(); self.state = State::MarkupDeclarationOpen; return; }
        self.emit_char('<');
        self.state = State::Data;
    }

    fn end_tag_open_state(&mut self) {
        let c = self.next_char();
        if c.is_ascii_alphabetic() { self.temp.clear(); self.state = State::TagName; return; }
        if c == '>' { self.consume_char(); self.state = State::Data; return; }
        self.emit_char('<');
        self.emit_char('/');
        self.state = State::BogusComment;
    }

    fn tag_name_state(&mut self) {
        loop {
            let c = self.next_char();
            if c.is_whitespace() { self.state = State::BeforeAttributeName; return; }
            if c == '/' { self.state = State::SelfClosingStartTag; return; }
            if c == '>' {
                self.consume_char();
                let name = self.flush_temp();
                self.emit_token(Token::StartTag(TagToken {
                    name,
                    self_closing: false,
                    attrs: HashMap::new(),
                }));
                self.state = State::Data;
                return;
            }
            if c.is_ascii_uppercase() { self.temp.push(c.to_ascii_lowercase()); } else { self.temp.push(c); }
            self.consume_char();
        }
    }

    fn before_attribute_name_state(&mut self) {
        loop {
            let c = self.next_char();
            if c.is_whitespace() { self.consume_char(); continue; }
            if c == '/' { self.state = State::SelfClosingStartTag; return; }
            if c == '>' {
                let name = self.flush_temp();
                self.emit_token(Token::StartTag(TagToken {
                    name,
                    self_closing: false,
                    attrs: HashMap::new(),
                }));
                self.state = State::Data;
                return;
            }
            self.temp.clear();
            self.state = State::AttributeName;
            return;
        }
    }

    fn attribute_name_state(&mut self) {
        loop {
            let c = self.next_char();
            if c.is_whitespace() || c == '/' || c == '>' {
                self.state = State::AfterAttributeName;
                return;
            }
            if c == '=' { self.consume_char(); self.state = State::BeforeAttributeValue; return; }
            if c.is_ascii_uppercase() { self.temp.push(c.to_ascii_lowercase()); } else if c != '"' && c != '\'' && c != '<' { self.temp.push(c); }
            self.consume_char();
        }
    }

    fn after_attribute_name_state(&mut self) {
        loop {
            let c = self.next_char();
            if c.is_whitespace() { self.consume_char(); continue; }
            if c == '/' { self.state = State::SelfClosingStartTag; return; }
            if c == '=' { self.consume_char(); self.state = State::BeforeAttributeValue; return; }
            if c == '>' {
                let name = self.flush_temp();
                self.emit_token(Token::StartTag(TagToken {
                    name,
                    self_closing: false,
                    attrs: HashMap::new(),
                }));
                self.state = State::Data;
                return;
            }
            self.temp.clear();
            self.state = State::AttributeName;
            return;
        }
    }

    fn before_attribute_value_state(&mut self) {
        let c = self.next_char();
        if c.is_whitespace() { self.consume_char(); return; }
        if c == '"' { self.consume_char(); self.temp.clear(); self.state = State::AttributeValueDoubleQuoted; return; }
        if c == '\'' { self.consume_char(); self.temp.clear(); self.state = State::AttributeValueSingleQuoted; return; }
        self.temp.clear(); self.state = State::AttributeValueUnquoted;
    }

    fn attribute_value_double_quoted_state(&mut self) {
        loop {
            let c = self.next_char();
            if c == '"' { self.consume_char(); self.state = State::AfterAttributeValueQuoted; return; }
            if c == '&' { self.consume_char(); self.state = State::CharacterReference; self.char_ref_code = None; return; }
            if c == '\0' { self.consume_char(); self.temp.push('\0'); }
            else { self.consume_char(); self.temp.push(c); }
        }
    }

    fn attribute_value_single_quoted_state(&mut self) {
        loop {
            let c = self.next_char();
            if c == '\'' { self.consume_char(); self.state = State::AfterAttributeValueQuoted; return; }
            if c == '&' { self.consume_char(); self.state = State::CharacterReference; self.char_ref_code = None; return; }
            if c == '\0' { self.consume_char(); self.temp.push('\0'); }
            else { self.consume_char(); self.temp.push(c); }
        }
    }

    fn attribute_value_unquoted_state(&mut self) {
        loop {
            let c = self.next_char();
            if c.is_whitespace() { self.state = State::BeforeAttributeName; return; }
            if c == '&' { self.consume_char(); self.state = State::CharacterReference; self.char_ref_code = None; return; }
            if c == '>' {
                self.consume_char();
                let name = self.flush_temp();
                self.emit_token(Token::StartTag(TagToken {
                    name: name.clone(),
                    self_closing: false,
                    attrs: HashMap::new(),
                }));
                self.state = State::Data;
                return;
            }
            if c == '"' || c == '\'' || c == '<' || c == '=' || c == '`' || c == '\0' { self.consume_char(); self.temp.push(c); }
            else { self.consume_char(); self.temp.push(c); }
        }
    }

    fn after_attribute_value_quoted_state(&mut self) {
        let c = self.next_char();
        if c.is_whitespace() { self.consume_char(); self.state = State::BeforeAttributeName; return; }
        if c == '/' { self.consume_char(); self.state = State::SelfClosingStartTag; return; }
        if c == '>' {
            self.consume_char();
            let name = self.flush_temp();
            self.emit_token(Token::StartTag(TagToken {
                name,
                self_closing: false,
                attrs: HashMap::new(),
            }));
            self.state = State::Data;
            return;
        }
        self.state = State::BeforeAttributeName;
    }

    fn self_closing_start_tag_state(&mut self) {
        let c = self.next_char();
        if c == '>' {
            self.consume_char();
            let n = self.flush_temp();
            self.emit_token(Token::StartTag(TagToken { name: n, self_closing: true, attrs: HashMap::new() }));
            self.state = State::Data;
            return;
        }
        if c.is_whitespace() { self.consume_char(); return; }
        self.state = State::BeforeAttributeName;
    }

    fn bogus_comment_state(&mut self) {
        let mut content = String::new();
        loop {
            if self.eof() { break; }
            let c = self.next_char();
            if c == '>' { self.consume_char(); break; }
            if c == '\0' { self.consume_char(); content.push('\0'); }
            else { content.push(c); self.consume_char(); }
        }
        self.emit_token(Token::Comment(content));
        self.state = State::Data;
    }

    fn markup_declaration_open_state(&mut self) {
        if self.input[self.pos..].starts_with("--") {
            self.pos += 2;
            self.state = State::CommentStart;
            return;
        }
        if self.input[self.pos..].to_uppercase().starts_with("DOCTYPE") {
            self.pos += 7;
            self.state = State::Doctype;
            return;
        }
        if self.input[self.pos..].starts_with("[CDATA[") {
            self.pos += 7;
            self.state = State::CdataSection;
            return;
        }
        self.state = State::BogusComment;
    }

    fn comment_start_state(&mut self) {
        let c = self.next_char();
        if c == '-' { self.consume_char(); self.state = State::CommentStartDash; return; }
        if c == '>' { self.consume_char(); self.state = State::Data; return; }
        self.state = State::Comment;
    }

    fn comment_start_dash_state(&mut self) {
        let c = self.next_char();
        if c == '-' { self.consume_char(); self.state = State::CommentEnd; return; }
        if c == '>' { self.consume_char(); self.state = State::Data; return; }
        self.state = State::Comment;
    }

    fn comment_state(&mut self) {
        loop {
            let c = self.next_char();
            if c == '-' { self.consume_char(); self.state = State::CommentEndDash; return; }
            if c == '\0' { self.consume_char(); self.temp.push('\0'); }
            else if !self.eof() { self.consume_char(); self.temp.push(c); }
            else { return; }
        }
    }

    fn comment_less_than_sign_state(&mut self) {
        let c = self.next_char();
        if c == '!' { self.consume_char(); self.temp.push_str("<!-"); self.state = State::CommentLessThanSignBang; return; }
        if c != '-' { self.temp.push('<'); }
        self.state = State::Comment;
    }

    fn comment_less_than_sign_bang_state(&mut self) {
        let c = self.next_char();
        if c == '-' { self.consume_char(); self.state = State::CommentLessThanSignBangDash; return; }
        self.state = State::Comment;
    }

    fn comment_less_than_sign_bang_dash_state(&mut self) {
        let c = self.next_char();
        if c == '-' { self.consume_char(); self.state = State::CommentLessThanSignBangDashDash; return; }
        self.state = State::CommentEndDash;
    }

    fn comment_less_than_sign_bang_dash_dash_state(&mut self) {
        self.state = State::CommentEnd;
    }

    fn comment_end_dash_state(&mut self) {
        let c = self.next_char();
        if c == '-' { self.consume_char(); self.state = State::CommentEnd; return; }
        self.state = State::Comment;
    }

    fn comment_end_state(&mut self) {
        let c = self.next_char();
        if c == '>' { self.consume_char(); /* emit comment */ self.flush_temp(); self.state = State::Data; return; }
        if c == '!' { self.consume_char(); self.state = State::CommentEndBang; return; }
        if c == '-' { self.consume_char(); self.temp.push('-'); return; }
        self.state = State::Comment;
    }

    fn comment_end_bang_state(&mut self) {
        let c = self.next_char();
        if c == '-' { self.consume_char(); self.temp.push_str("--!"); self.state = State::CommentEndDash; return; }
        if c == '>' { self.consume_char(); self.state = State::Data; return; }
        self.state = State::Comment;
    }

    fn doctype_state(&mut self) {
        let c = self.next_char();
        if c.is_whitespace() { self.consume_char(); self.state = State::BeforeDoctypeName; return; }
        if c == '>' { self.consume_char(); self.state = State::Data; return; }
        self.state = State::BeforeDoctypeName;
    }

    fn before_doctype_name_state(&mut self) {
        loop {
            let c = self.next_char();
            if c.is_whitespace() { self.consume_char(); continue; }
            if c == '>' { self.emit_token(Token::Doctype(DoctypeToken::default())); self.state = State::Data; return; }
            self.state = State::DoctypeName;
            return;
        }
    }

    fn doctype_name_state(&mut self) {
        loop {
            let c = self.next_char();
            if c.is_whitespace() { self.state = State::AfterDoctypeName; return; }
            if c == '>' { self.consume_char(); self.state = State::Data; return; }
            if c.is_ascii_uppercase() { self.temp.push(c.to_ascii_lowercase()); } else { self.temp.push(c); }
            self.consume_char();
        }
    }

    fn after_doctype_name_state(&mut self) {
        loop {
            let c = self.next_char();
            if c.is_whitespace() { self.consume_char(); continue; }
            if c == '>' { self.consume_char(); self.state = State::Data; return; }
            if self.input[self.pos..].to_uppercase().starts_with("PUBLIC") {
                self.pos += 6;
                self.state = State::AfterDoctypePublicKeyword;
                return;
            }
            if self.input[self.pos..].to_uppercase().starts_with("SYSTEM") {
                self.pos += 6;
                self.state = State::AfterDoctypePublicKeyword;
                return;
            }
            self.state = State::BogusDoctype;
            return;
        }
    }

    fn after_doctype_public_keyword_state(&mut self) {
        loop {
            let c = self.next_char();
            if c.is_whitespace() { self.consume_char(); continue; }
            if c == '"' { self.consume_char(); self.temp.clear(); self.state = State::DoctypePublicIdentifierDoubleQuoted; return; }
            if c == '\'' { self.consume_char(); self.temp.clear(); self.state = State::DoctypePublicIdentifierSingleQuoted; return; }
            if c == '>' { self.consume_char(); self.state = State::Data; return; }
            self.state = State::BogusDoctype;
            return;
        }
    }

    fn before_doctype_public_identifier_state(&mut self) {
        loop {
            let c = self.next_char();
            if c.is_whitespace() { self.consume_char(); continue; }
            if c == '"' { self.consume_char(); self.temp.clear(); self.state = State::DoctypePublicIdentifierDoubleQuoted; return; }
            if c == '\'' { self.consume_char(); self.temp.clear(); self.state = State::DoctypePublicIdentifierSingleQuoted; return; }
            if c == '>' { self.consume_char(); self.state = State::Data; return; }
            self.state = State::BogusDoctype;
            return;
        }
    }

    fn doctype_public_identifier_double_quoted_state(&mut self) {
        loop {
            let c = self.next_char();
            if c == '"' { self.consume_char(); self.state = State::AfterDoctypePublicIdentifier; return; }
            if c == '\0' { self.consume_char(); self.temp.push('\0'); }
            else if !self.eof() { self.consume_char(); self.temp.push(c); }
            else { return; }
        }
    }

    fn doctype_public_identifier_single_quoted_state(&mut self) {
        loop {
            let c = self.next_char();
            if c == '\'' { self.consume_char(); self.state = State::AfterDoctypePublicIdentifier; return; }
            if c == '\0' { self.consume_char(); self.temp.push('\0'); }
            else if !self.eof() { self.consume_char(); self.temp.push(c); }
            else { return; }
        }
    }

    fn after_doctype_public_identifier_state(&mut self) {
        loop {
            let c = self.next_char();
            if c.is_whitespace() { self.consume_char(); self.state = State::BetweenDoctypePublicAndSystemIdentifiers; return; }
            if c == '>' { self.consume_char(); self.state = State::Data; return; }
            if c == '"' { self.consume_char(); self.temp.clear(); self.state = State::DoctypeSystemIdentifierDoubleQuoted; return; }
            if c == '\'' { self.consume_char(); self.temp.clear(); self.state = State::DoctypeSystemIdentifierSingleQuoted; return; }
            self.state = State::BogusDoctype;
            return;
        }
    }

    fn between_doctype_public_and_system_identifiers_state(&mut self) {
        loop {
            let c = self.next_char();
            if c.is_whitespace() { self.consume_char(); continue; }
            if c == '>' { self.consume_char(); self.state = State::Data; return; }
            if c == '"' { self.consume_char(); self.temp.clear(); self.state = State::DoctypeSystemIdentifierDoubleQuoted; return; }
            if c == '\'' { self.consume_char(); self.temp.clear(); self.state = State::DoctypeSystemIdentifierSingleQuoted; return; }
            self.state = State::BogusDoctype;
            return;
        }
    }

    fn before_doctype_system_identifier_state(&mut self) {
        loop {
            let c = self.next_char();
            if c.is_whitespace() { self.consume_char(); continue; }
            if c == '"' { self.consume_char(); self.temp.clear(); self.state = State::DoctypeSystemIdentifierDoubleQuoted; return; }
            if c == '\'' { self.consume_char(); self.temp.clear(); self.state = State::DoctypeSystemIdentifierSingleQuoted; return; }
            if c == '>' { self.consume_char(); self.state = State::Data; return; }
            self.state = State::BogusDoctype;
            return;
        }
    }

    fn doctype_system_identifier_double_quoted_state(&mut self) {
        loop {
            let c = self.next_char();
            if c == '"' { self.consume_char(); self.state = State::AfterDoctypeSystemIdentifier; return; }
            if c == '\0' { self.consume_char(); self.temp.push('\0'); }
            else if !self.eof() { self.consume_char(); self.temp.push(c); }
            else { return; }
        }
    }

    fn doctype_system_identifier_single_quoted_state(&mut self) {
        loop {
            let c = self.next_char();
            if c == '\'' { self.consume_char(); self.state = State::AfterDoctypeSystemIdentifier; return; }
            if c == '\0' { self.consume_char(); self.temp.push('\0'); }
            else if !self.eof() { self.consume_char(); self.temp.push(c); }
            else { return; }
        }
    }

    fn after_doctype_system_identifier_state(&mut self) {
        loop {
            let c = self.next_char();
            if c.is_whitespace() { self.consume_char(); continue; }
            if c == '>' { self.consume_char(); self.state = State::Data; return; }
            self.state = State::BogusDoctype;
            return;
        }
    }

    fn bogus_doctype_state(&mut self) {
        loop {
            if self.eof() { return; }
            let c = self.next_char();
            if c == '>' { self.consume_char(); self.state = State::Data; return; }
            self.consume_char();
        }
    }

    fn cdata_section_state(&mut self) {
        loop {
            let c = self.next_char();
            if c == ']' { self.consume_char(); self.state = State::CdataSectionBracket; return; }
            if self.eof() { return; }
            self.consume_char(); self.emit_char(c);
        }
    }

    fn cdata_section_bracket_state(&mut self) {
        let c = self.next_char();
        if c == ']' { self.consume_char(); self.state = State::CdataSectionEnd; return; }
        self.emit_char(']');
        self.state = State::CdataSection;
    }

    fn cdata_section_end_state(&mut self) {
        let c = self.next_char();
        if c == ']' { self.consume_char(); self.emit_char(']'); return; }
        if c == '>' { self.consume_char(); self.state = State::Data; return; }
        self.emit_char(']');
        self.emit_char(']');
        self.state = State::CdataSection;
    }

    fn rcdata_state(&mut self) {
        loop {
            let c = self.next_char();
            if c == '&' { self.consume_char(); self.state = State::CharacterReference; return; }
            if c == '<' { self.consume_char(); self.state = State::RcDataLessThanSign; return; }
            if c == '\0' { self.consume_char(); self.emit_char('\0'); }
            else if !self.eof() { self.consume_char(); self.emit_char(c); }
            else { return; }
        }
    }

    fn raw_text_state(&mut self) {
        loop {
            let c = self.next_char();
            if c == '<' { self.consume_char(); self.state = State::RcDataLessThanSign; return; }
            if c == '\0' { self.consume_char(); self.emit_char('\0'); }
            else if !self.eof() { self.consume_char(); self.emit_char(c); }
            else { return; }
        }
    }

    fn script_data_state(&mut self) {
        loop {
            let c = self.next_char();
            if c == '<' { self.consume_char(); self.state = State::ScriptDataLessThanSign; return; }
            if c == '\0' { self.consume_char(); self.emit_char('\0'); }
            else if !self.eof() { self.consume_char(); self.emit_char(c); }
            else { return; }
        }
    }

    fn plain_text_state(&mut self) {
        loop {
            if self.eof() { return; }
            let c = self.next_char();
            if c == '\0' { self.consume_char(); self.emit_char('\0'); }
            else { self.consume_char(); self.emit_char(c); }
        }
    }

    fn rcdata_less_than_sign_state(&mut self) {
        let c = self.next_char();
        if c == '/' { self.consume_char(); self.temp.clear(); self.state = State::RcDataEndTagOpen; return; }
        self.emit_char('<'); self.state = State::RcData;
    }

    fn rcdata_end_tag_open_state(&mut self) {
        let c = self.next_char();
        if c.is_ascii_alphabetic() { self.temp.push(c.to_ascii_lowercase()); self.consume_char(); self.state = State::RcDataEndTagName; return; }
        self.emit_char('<'); self.emit_char('/'); self.state = State::RcData;
    }

    fn rcdata_end_tag_name_state(&mut self) {
        loop {
            let c = self.next_char();
            if c.is_whitespace() || c == '/' || c == '>' { /* tentative end tag */ self.state = State::Data; return; }
            if c.is_ascii_alphabetic() { self.temp.push(c.to_ascii_lowercase()); self.consume_char(); }
            else { self.state = State::RcData; return; }
        }
    }

    fn script_data_less_than_sign_state(&mut self) {
        let c = self.next_char();
        if c == '/' { self.consume_char(); self.temp.clear(); self.state = State::ScriptDataEndTagOpen; return; }
        if c == '!' { self.consume_char(); self.state = State::ScriptDataEscapeStart; return; }
        self.emit_char('<'); self.state = State::ScriptData;
    }

    fn script_data_end_tag_open_state(&mut self) {
        let c = self.next_char();
        if c.is_ascii_alphabetic() { self.temp.push(c.to_ascii_lowercase()); self.consume_char(); self.state = State::ScriptDataEndTagName; return; }
        self.emit_char('<'); self.emit_char('/'); self.state = State::ScriptData;
    }

    fn script_data_end_tag_name_state(&mut self) {
        loop {
            let c = self.next_char();
            if c.is_whitespace() || c == '/' || c == '>' { self.state = State::Data; return; }
            if c.is_ascii_alphabetic() { self.temp.push(c.to_ascii_lowercase()); self.consume_char(); }
            else { self.state = State::ScriptData; return; }
        }
    }

    fn script_data_escape_start_state(&mut self) {
        let c = self.next_char();
        if c == '-' { self.consume_char(); self.state = State::ScriptDataEscapeStartDash; return; }
        self.emit_char('<'); self.emit_char('!'); self.state = State::ScriptData;
    }

    fn script_data_escape_start_dash_state(&mut self) {
        let c = self.next_char();
        if c == '-' { self.consume_char(); self.state = State::ScriptDataEscapedDashDash; return; }
        self.emit_char('<'); self.emit_char('!'); self.emit_char('-'); self.state = State::ScriptData;
    }

    fn script_data_escaped_state(&mut self) {
        loop {
            let c = self.next_char();
            if c == '-' { self.consume_char(); self.emit_char('-'); self.state = State::ScriptDataEscapedDash; return; }
            if c == '<' { self.consume_char(); self.state = State::ScriptDataEscapedLessThanSign; return; }
            if c == '\0' { self.consume_char(); self.emit_char('\0'); }
            else if !self.eof() { self.consume_char(); self.emit_char(c); }
            else { return; }
        }
    }

    fn script_data_escaped_dash_state(&mut self) {
        let c = self.next_char();
        if c == '-' { self.consume_char(); self.state = State::ScriptDataEscapedDashDash; return; }
        if c == '<' { self.consume_char(); self.state = State::ScriptDataEscapedLessThanSign; return; }
        if c == '\0' { self.consume_char(); self.emit_char('\0'); }
        else if !self.eof() { self.consume_char(); self.emit_char(c); }
        else { return; }
    }

    fn script_data_escaped_dash_dash_state(&mut self) {
        loop {
            let c = self.next_char();
            if c == '-' { self.consume_char(); self.emit_char('-'); return; }
            if c == '<' { self.consume_char(); self.state = State::ScriptDataEscapedLessThanSign; return; }
            if c == '>' { self.consume_char(); self.emit_char('>'); self.state = State::ScriptData; return; }
            if c == '\0' { self.consume_char(); self.emit_char('\0'); }
            else if !self.eof() { self.consume_char(); self.emit_char(c); }
            else { return; }
        }
    }

    fn script_data_escaped_less_than_sign_state(&mut self) {
        let c = self.next_char();
        if c == '/' { self.consume_char(); self.temp.clear(); self.state = State::ScriptDataEscapedEndTagOpen; return; }
        if c.is_ascii_alphabetic() { self.emit_char('<'); self.temp.push(c.to_ascii_lowercase()); self.consume_char(); self.state = State::ScriptDataDoubleEscapeStart; return; }
        self.emit_char('<'); self.state = State::ScriptDataEscaped;
    }

    fn script_data_escaped_end_tag_open_state(&mut self) {
        let c = self.next_char();
        if c.is_ascii_alphabetic() { self.temp.push(c.to_ascii_lowercase()); self.consume_char(); self.state = State::ScriptDataEscapedEndTagName; return; }
        self.emit_char('<'); self.emit_char('/'); self.state = State::ScriptDataEscaped;
    }

    fn script_data_escaped_end_tag_name_state(&mut self) {
        loop {
            let c = self.next_char();
            if c.is_whitespace() || c == '/' || c == '>' { self.state = State::Data; return; }
            if c.is_ascii_alphabetic() { self.temp.push(c.to_ascii_lowercase()); self.consume_char(); }
            else { self.state = State::ScriptDataEscaped; return; }
        }
    }

    fn script_data_double_escape_start_state(&mut self) {
        loop {
            let c = self.next_char();
            if c.is_whitespace() || c == '/' || c == '>' { self.state = State::ScriptDataDoubleEscaped; return; }
            if c.is_ascii_alphabetic() { self.temp.push(c.to_ascii_lowercase()); self.consume_char(); }
            else { self.state = State::ScriptDataEscaped; return; }
        }
    }

    fn script_data_double_escaped_state(&mut self) {
        loop {
            let c = self.next_char();
            if c == '-' { self.consume_char(); self.emit_char('-'); self.state = State::ScriptDataDoubleEscapedDash; return; }
            if c == '<' { self.consume_char(); self.state = State::ScriptDataDoubleEscapedLessThanSign; return; }
            if c == '\0' { self.consume_char(); self.emit_char('\0'); }
            else if !self.eof() { self.consume_char(); self.emit_char(c); }
            else { return; }
        }
    }

    fn script_data_double_escaped_dash_state(&mut self) {
        let c = self.next_char();
        if c == '-' { self.consume_char(); self.state = State::ScriptDataDoubleEscapedDashDash; return; }
        if c == '<' { self.consume_char(); self.state = State::ScriptDataDoubleEscapedLessThanSign; return; }
        if c == '\0' { self.consume_char(); self.emit_char('\0'); }
        else if !self.eof() { self.consume_char(); self.emit_char(c); }
        else { return; }
    }

    fn script_data_double_escaped_dash_dash_state(&mut self) {
        loop {
            let c = self.next_char();
            if c == '-' { self.consume_char(); self.emit_char('-'); return; }
            if c == '<' { self.consume_char(); self.state = State::ScriptDataDoubleEscapedLessThanSign; return; }
            if c == '>' { self.consume_char(); self.emit_char('>'); self.state = State::ScriptData; return; }
            if c == '\0' { self.consume_char(); self.emit_char('\0'); }
            else if !self.eof() { self.consume_char(); self.emit_char(c); }
            else { return; }
        }
    }

    fn script_data_double_escaped_less_than_sign_state(&mut self) {
        let c = self.next_char();
        if c == '/' { self.consume_char(); self.temp.clear(); self.state = State::ScriptDataDoubleEscapeEnd; return; }
        self.emit_char('<'); self.state = State::ScriptDataDoubleEscaped;
    }

    fn script_data_double_escape_end_state(&mut self) {
        loop {
            let c = self.next_char();
            if c.is_whitespace() || c == '/' || c == '>' { self.state = State::ScriptDataEscaped; return; }
            if c.is_ascii_alphabetic() { self.temp.push(c.to_ascii_lowercase()); self.consume_char(); }
            else { self.state = State::ScriptDataDoubleEscaped; return; }
        }
    }

    fn character_reference_state(&mut self) {
        let c = self.next_char();
        if c.is_ascii_alphanumeric() { self.state = State::NamedCharacterReference; return; }
        if c == '#' { self.consume_char(); self.char_ref_code = Some(0); self.state = State::NumericCharacterReference; return; }
        self.emit_char('&'); self.state = self.return_state.clone();
    }

    fn named_character_reference_state(&mut self) {
        let name = self.consume_ascii_alpha_while();
        let code = match name.as_str() {
            "amp" => '&',
            "lt" => '<',
            "gt" => '>',
            "quot" => '"',
            "apos" => '\'',
            "nbsp" => '\u{00A0}',
            "copy" => '\u{00A9}',
            "reg" => '\u{00AE}',
            "trade" => '\u{2122}',
            _ => { self.emit_char('&'); for ch in name.chars() { self.emit_char(ch); } self.state = self.return_state.clone(); return; }
        };
        let next = self.next_char();
        if next == ';' {
            self.consume_char();
        }
        self.emit_char(code);
        self.state = self.return_state.clone();
    }

    fn ambiguous_ampersand_state(&mut self) {
        let c = self.next_char();
        if c.is_ascii_alphanumeric() { self.consume_char(); self.emit_char(c); return; }
        if c == ';' { self.consume_char(); return; }
        self.state = self.return_state.clone();
    }

    fn numeric_character_reference_state(&mut self) {
        let c = self.next_char();
        if c == 'x' || c == 'X' { self.consume_char(); self.char_ref_code = Some(0); self.state = State::HexademicalCharacterReferenceStart; return; }
        self.state = State::DecimalCharacterReferenceStart;
    }

    fn hexademical_character_reference_start_state(&mut self) {
        self.state = State::HexademicalCharacterReference;
    }

    fn decimal_character_reference_start_state(&mut self) {
        self.state = State::DecimalCharacterReference;
    }

    fn hexademical_character_reference_state(&mut self) {
        let digits = self.consume_while(|c| c.is_ascii_hexdigit());
        if let Ok(val) = u32::from_str_radix(&digits, 16) {
            self.char_ref_code = Some(val);
        }
        self.state = State::NumericCharacterReferenceEnd;
    }

    fn decimal_character_reference_state(&mut self) {
        let digits = self.consume_while(|c| c.is_ascii_digit());
        if let Ok(val) = u32::from_str_radix(&digits, 10) {
            self.char_ref_code = Some(val);
        }
        self.state = State::NumericCharacterReferenceEnd;
    }

    fn numeric_character_reference_end_state(&mut self) {
        if self.next_char() == ';' {
            self.consume_char();
        }
        if let Some(code) = self.char_ref_code {
            if let Some(c) = char::from_u32(code) {
                if c != '\0' && !(0xD800..=0xDFFF).contains(&code) && code <= 0x10FFFF {
                    self.emit_char(c);
                    self.state = self.return_state.clone();
                    return;
                }
            }
        }
        self.emit_char('\u{FFFD}');
        self.state = self.return_state.clone();
    }
}
