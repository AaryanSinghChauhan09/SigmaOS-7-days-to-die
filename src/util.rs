//! 7-days-to-die model — 7 Days to Die dedicated server.

#[derive(Debug, Clone)]
pub struct Player {
    pub name: String,
    pub id: String,
    pub ping: u32,
}

#[derive(Debug, Clone)]
pub struct Model {
    pub players: Vec<Player>,
    pub world: String,
    pub running: bool,
}

impl Default for Model {
    fn default() -> Self { Self { players: vec![], world: "Navezgane".into(), running: false } }
}

impl Model {
    pub fn new() -> Self { Self::default() }

    pub fn start(&mut self) -> bool {
        if !self.running { self.running = true; true } else { false }
    }

    pub fn join(&mut self, player: Player) -> bool {
        if self.running { self.players.push(player); true } else { false }
    }

    pub fn list_players(&self) -> &[Player] { &self.players }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn start_join() {
        let mut d = Model::new();
        assert!(d.start());
        assert!(d.join(Player { name: "P1".into(), id: "1".into(), ping: 25 }));
        assert_eq!(d.list_players().len(), 1);
    }
}
