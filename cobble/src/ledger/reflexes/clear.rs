use crate::ledger::resource::Ledger;

impl Ledger {
    pub fn clear(&mut self) {
        self.started = false;
        self.boards.clear();
    }
}
