use ramen_common::error::Diagnostic;

use crate::lex::{Token, TokenInfo};

/// Error originating from invalid syntax.
/// This should not be used after parsing stage.
#[derive(Debug, Clone)]
pub enum SyntaxError {
    UnexpectedToken {
        expected: Vec<Token>,
        found: TokenInfo,
    },
    ExpectedItem {
        found: TokenInfo
    },
    ExpectedExpression {
        found: TokenInfo
    },
    ExpectedType {
        found: TokenInfo
    }
}

impl Diagnostic for SyntaxError {
    fn is_fatal(&self) -> bool { true }

    fn get_location(&self) -> ramen_common::Loc {
        match self {
            Self::UnexpectedToken { found, .. } => found.location(),
            Self::ExpectedItem { found } => found.location(),
            Self::ExpectedExpression { found } => found.location(),
            Self::ExpectedType { found } => found.location(),
        }
    }

    fn build_report(&self, _session: &ramen_common::session::Session) -> ramen_common::ariadne::Report<'static, ramen_common::Loc> { 
        use ramen_common::ariadne::{Report, ReportKind, Config, Label};

        let loc = self.get_location();
        let mut report = Report::build(
            ReportKind::Error,
            loc.file,
            loc.span.start
        ).with_config(Config::default().with_cross_gap(true));

        report = match self {
            Self::UnexpectedToken { expected, .. } => {
                report.with_code("S01").with_message("Encountered unexpected token during parsing.")
                .with_label(
                    Label::new(loc)
                    .with_message(format!("Expected one of {expected:?} but got this."))
                    .with_priority(4)
                )
            }
            Self::ExpectedItem { .. } => {
                report.with_code("S02").with_message("Expected top-level item.")
                .with_label(
                    Label::new(loc)
                    .with_message(format!("Expected top-level item but got this."))
                    .with_priority(4)
                )
            }
            Self::ExpectedExpression { .. } => {
                report.with_code("S03").with_message("Expected expression.")
                .with_label(
                    Label::new(loc)
                    .with_message(format!("Expected expression but got this."))
                    .with_priority(4)
                )
            }
            Self::ExpectedType { .. } => {
                report.with_code("S04").with_message("Expected type literal.")
                .with_label(
                    Label::new(loc)
                    .with_message(format!("Expected type literal like int32 or unit, but found this."))
                    .with_priority(4)
                )
            }
        };

        report.finish()
    }
}