use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "dashlang.pest"]
pub struct DashlangParser {}
