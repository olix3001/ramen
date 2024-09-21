use inkwell::builder::BuilderError;
use ramen_common::{error::Diagnostic, session::Session, Loc, ariadne};

#[derive(Debug, Clone)]
pub enum CodegenError {

}

impl Diagnostic for CodegenError {
    fn is_fatal(&self) -> bool { true }

    fn get_location(&self) -> Loc {
        todo!()
    }

    fn build_report(&self, session: &Session) -> ariadne::Report<'static, Loc> {
        todo!()
    }
}

impl From<BuilderError> for CodegenError {
    fn from(value: BuilderError) -> Self {
        todo!()
    }
}