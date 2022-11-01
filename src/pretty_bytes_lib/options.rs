#[derive(Clone)]
pub struct PrettyBytesOptions {
  pub use_1024_instead_of_1000: Option<bool>,
  pub number_of_decimal: Option<usize>,
  pub remove_zero_decimal: Option<bool>
}


#[derive(Clone)]
pub struct PrettyBytesOptionWithDefault {
  pub use_1024_instead_of_1000: bool,
  pub number_of_decimal: usize,
  pub remove_zero_decimal: bool
}
