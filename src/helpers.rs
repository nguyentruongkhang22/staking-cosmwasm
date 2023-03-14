use crate::state::ContractState;

impl ContractState {
  pub fn get_total_supply(&self) -> u64 {
    self.total_supply
  }
}
