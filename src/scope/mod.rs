mod layer_map;

use crate::TypeIdent;
use layer_map::LayerMap;

// TODO there needs to be a proper package for this config
pub mod Config {
    pub type Config = ();

    // TODO there needs to be a method to build the base scope, based on a projects config File
    fn base_scope(config: &Config) -> super::Scope {
        unimplemented!();
    }
}

pub enum Scope {
    /// Module or Library
    Module {
        path_name: String,
        exports: Vec<Scope>,
    },

    /// Type Struct
    StructType {
        ty: TypeIdent,
        fields: Vec<StructField>,
    },

    /// Type Enum
    EnumType {
        // path_name can be derived from type
        ty: TypeIdent,
        Variants: Vec<EnumVariant>,
    },

    Func {
        name: String,
    },

    Ident {
        name: String,
        ty: Option<TypeIdent>,
    }
}
