use std::collections::BTreeMap;

type AccountId = String;
type Balance = u128;

#[derive(Debug)]
pub struct Pallet {
    balances: BTreeMap<AccountId, Balance>,
}

impl Pallet {
    pub fn new() -> Self {
        Self {
            balances: BTreeMap::new()
        }
    }
    pub fn set_balance(&mut self, who: &AccountId, amount: Balance) {
        self.balances.insert(who.clone(), amount);
    }
    pub fn balance(&self, who: &AccountId) -> Balance {
        *self.balances.get(who).unwrap_or(&0)
    }

    pub fn transfer(
        &mut self,
        caller: AccountId,
        to: AccountId,
        amount: Balance,
    ) -> Result<(), &'static str> {
        let caller_ballance = self.balance(&caller);
        let to_ballance = self.balance(&to);

        let new_caller_balance = caller_ballance.checked_sub(amount).ok_or("Not enough funds")?;
        let new_to_balance = to_ballance.checked_add(amount).ok_or("Overflow")?;

        self.balances.insert(caller, new_caller_balance);
        self.balances.insert(to, new_to_balance);

        Ok(())

    }
}