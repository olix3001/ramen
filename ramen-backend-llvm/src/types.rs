use inkwell::{context::Context, types::{AnyTypeEnum, BasicMetadataTypeEnum, BasicType, BasicTypeEnum, FunctionType, VoidType}};
use ramen_common::types::{CallableType, RamenType};

use crate::error::CodegenError;

pub trait AsLLType {
    type Error;

    fn as_llvm_type<'ctx>(&self, context: &'ctx Context) -> Result<AnyTypeEnum<'ctx>, Self::Error>;
}

impl AsLLType for RamenType {
    type Error = CodegenError;
    
    fn as_llvm_type<'ctx>(&self, context: &'ctx Context) -> Result<AnyTypeEnum<'ctx>, Self::Error> {
        match self {
            Self::Unit => Ok(AnyTypeEnum::VoidType(context.void_type())),
            Self::Integer(width) => Ok(AnyTypeEnum::IntType(context.custom_width_int_type(*width as _))),
            Self::Callable(callable) => callable.as_llvm_type(context),
            _ => todo!("Throw apropriate error")
        }
    }
}

impl AsLLType for CallableType {
    type Error = CodegenError;

    fn as_llvm_type<'ctx>(&self, context: &'ctx Context) -> Result<AnyTypeEnum<'ctx>, Self::Error> {
        println!("{:?}", self.return_type);
        let return_type = self.return_type.as_llvm_type(context)?;

        let fn_type = build_fn_type_from_any_type(
            return_type,
            self.parameter_types.iter().map(|ty| {
                let basic_ty: BasicTypeEnum = ty.as_llvm_type(context)?.try_into().map_err::<CodegenError, _>(|_| todo!())?;
                Ok(basic_ty.into())
            }).collect::<Result<Vec<BasicMetadataTypeEnum>, CodegenError>>()?.as_slice(),
            self.is_vararg
        )?;

        Ok(AnyTypeEnum::FunctionType(fn_type))
    }
}

fn build_fn_type_from_any_type<'ctx>(
    return_type: AnyTypeEnum<'ctx>, 
    argument_types: &[BasicMetadataTypeEnum<'ctx>], 
    is_vararg: bool
) -> Result<FunctionType<'ctx>, CodegenError> {
    match return_type {
        AnyTypeEnum::VoidType(void_type) =>
            Ok(void_type.fn_type(argument_types, is_vararg)),
        _ => {
            let return_type: BasicTypeEnum = return_type.try_into().map_err::<CodegenError, _>(|_| todo!("Yes"))?;
            Ok(return_type.fn_type(argument_types, is_vararg))
        }
    }
}