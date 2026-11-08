//! 7 Days to Die model — dedicated server with advanced features.

use std::collections::HashMap;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

#[derive(Debug, Clone, PartialEq)]
pub enum GameState {
    Stopped,
    Starting,
    Running,
    Paused,
    ShuttingDown,
    Crashed,
}

#[derive(Debug, Clone, PartialEq)]
pub enum PlayerStatus {
    Connecting,
    Connected,
    InGame,
    Disconnected,
    Banned,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Difficulty {
    Beginner,
    Normal,
    Hard,
    NightStalker,
    Creative,
}

#[derive(Debug, Clone, PartialEq)]
pub enum GameMode {
    Survival,
    Creative,
    Viewer,
}

#[derive(Debug, Clone, PartialEq)]
pub enum BloodMoonFrequency {
    Never,
    Weekly,
    Monthly,
    FullMoon,
}

#[derive(Debug, Clone, PartialEq)]
pub enum LootAbundance {
    Scarcity,
    Poor,
    Normal,
    Abundant,
    Mega,
}

#[derive(Debug, Clone, PartialEq)]
pub enum BlockDamage {
    None,
    Low,
    Medium,
    High,
    VeryHigh,
}

#[derive(Debug, Clone, PartialEq)]
pub enum BlockDamagePlayer {
    None,
    Low,
    Medium,
    High,
    VeryHigh,
}

#[derive(Debug, Clone, PartialEq)]
pub enum EntityDamage {
    None,
    Low,
    Medium,
    High,
    VeryHigh,
}

#[derive(Debug, Clone, PartialEq)]
pub enum EntityDamagePlayer {
    None,
    Low,
    Medium,
    High,
    VeryHigh,
}

#[derive(Debug, Clone)]
pub struct Player {
    pub id: String,
    pub name: String,
    pub ip_address: String,
    pub ping: u32,
    pub status: PlayerStatus,
    pub joined_at: SystemTime,
    pub last_seen: SystemTime,
    pub playtime: Duration,
    pub kills: u32,
    pub deaths: u32,
    pub score: u64,
    pub position: (f32, f32, f32),
    pub health: f32,
    pub max_health: f32,
    pub stamina: f32,
    pub max_stamina: f32,
    pub experience: u64,
    pub level: u32,
    pub is_admin: bool,
    pub is_op: bool,
    pub is_vip: bool,
}

#[derive(Debug, Clone)]
pub struct World {
    pub name: String,
    pub seed: u64,
    pub size: (u32, u32),
    pub game_mode: GameMode,
    pub difficulty: Difficulty,
    pub day_night_cycle: bool,
    pub blood_moon_frequency: BloodMoonFrequency,
    pub max_players: u32,
    pub pvp_enabled: bool,
    pub pve_enabled: bool,
    pub friendly_fire: bool,
    pub drop_on_death: bool,
    pub loot_abundance: LootAbundance,
    pub block_damage: BlockDamage,
    pub block_damage_player: BlockDamagePlayer,
    pub block_value_percentages: HashMap<String, f32>,
    pub entity_damage: EntityDamage,
    pub entity_damage_player: EntityDamagePlayer,
    pub entity_class_damage: HashMap<String, f32>,
    pub player_safe_zone_hours: u32,
    pub player_safe_zone_level: u32,
    pub build_create: bool,
    pub build_destroy: BlockDamage,
    pub build_change_block: bool,
}

#[derive(Debug, Clone)]
pub struct ModelConfig {
    pub server_port: u16,
    pub control_port: u16,
    pub max_players: u32,
    pub max_ping: u32,
    pub kick_idle_players: bool,
    pub idle_timeout: Duration,
    pub save_interval: Duration,
    pub backup_interval: Duration,
    pub max_backups: usize,
    pub enable_rcon: bool,
    pub rcon_port: u16,
    pub rcon_password: Option<String>,
    pub enable_web_api: bool,
    pub web_api_port: u16,
    pub enable_stats: bool,
    pub stats_port: u16,
    pub enable_mods: bool,
    pub mods_dir: String,
    pub world_dir: String,
    pub save_dir: String,
    pub log_dir: String,
    pub server_name: String,
    pub server_description: String,
    pub server_password: Option<String>,
    pub visible: bool,
    pub pvp_enabled: bool,
    pub pve_enabled: bool,
    pub allow_crossplay: bool,
    pub game_name: String,
    pub game_version: String,
    pub map_name: String,
    pub map_seed: u64,
    pub game_mode: GameMode,
    pub difficulty: Difficulty,
    pub day_night_cycle: bool,
    pub blood_moon_frequency: BloodMoonFrequency,
    pub max_players_config: u32,
    pub pvp_enabled_config: bool,
    pub pve_enabled_config: bool,
    pub friendly_fire_config: bool,
    pub drop_on_death_config: bool,
    pub loot_abundance_config: LootAbundance,
    pub block_damage_config: BlockDamage,
    pub block_damage_player_config: BlockDamagePlayer,
    pub block_value_percentages_config: HashMap<String, f32>,
    pub entity_damage_config: EntityDamage,
    pub entity_damage_player_config: EntityDamagePlayer,
    pub entity_class_damage_config: HashMap<String, f32>,
    pub player_safe_zone_hours_config: u32,
    pub player_safe_zone_level_config: u32,
    pub build_create_config: bool,
    pub build_destroy_config: BlockDamage,
    pub build_change_block_config: bool,
}

#[derive(Debug, Clone)]
pub struct ServerStatistics {
    pub total_connections: u64,
    pub successful_connections: u64,
    pub failed_connections: u64,
    pub total_playtime: Duration,
    pub average_ping: u32,
    pub total_kills: u64,
    pub total_deaths: u64,
    pub uptime: Duration,
    pub started_at: Option<SystemTime>,
    pub last_backup: Option<SystemTime>,
}

#[derive(Debug, Clone)]
pub struct EventLog {
    pub timestamp: SystemTime,
    pub event_type: String,
    pub details: Option<String>,
}

#[derive(Debug, Clone)]
pub struct Model {
    pub players: HashMap<String, Player>,
    pub worlds: HashMap<String, World>,
    pub current_world: Option<String>,
    pub game_state: GameState,
    pub config: ModelConfig,
    pub statistics: ServerStatistics,
    pub next_player_id: u32,
    pub admin_list: Vec<String>,
    pub ban_list: Vec<String>,
    pub whitelist: Vec<String>,
    pub event_log: Vec<EventLog>,
    pub max_event_log: usize,
}

impl Default for ModelConfig {
    fn default() -> Self {
        Self {
            server_port: 26900,
            control_port: 8080,
            max_players: 8,
            max_ping: 500,
            kick_idle_players: true,
            idle_timeout: Duration::from_secs(600),
            save_interval: Duration::from_secs(900),
            backup_interval: Duration::from_secs(3600),
            max_backups: 10,
            enable_rcon: true,
            rcon_port: 8081,
            rcon_password: None,
            enable_web_api: true,
            web_api_port: 8082,
            enable_stats: true,
            stats_port: 8083,
            enable_mods: true,
            mods_dir: "/home/server/.config/7dtd/mods".into(),
            world_dir: "/home/server/.config/7dtd/worlds".into(),
            save_dir: "/home/server/.config/7dtd/saves".into(),
            log_dir: "/home/server/.config/7dtd/logs".into(),
            server_name: "7 Days to Die Server".into(),
            server_description: "A 7 Days to Die dedicated server".into(),
            server_password: None,
            visible: true,
            pvp_enabled: true,
            pve_enabled: true,
            allow_crossplay: false,
            game_name: "7 Days to Die".into(),
            game_version: "2.0".into(),
            map_name: "Navezgane".into(),
            map_seed: 0,
            game_mode: GameMode::Survival,
            difficulty: Difficulty::Normal,
            day_night_cycle: true,
            blood_moon_frequency: BloodMoonFrequency::Monthly,
            max_players_config: 8,
            pvp_enabled_config: true,
            pve_enabled_config: true,
            friendly_fire_config: false,
            drop_on_death_config: true,
            loot_abundance_config: LootAbundance::Normal,
            block_damage_config: BlockDamage::Medium,
            block_damage_player_config: BlockDamagePlayer::Medium,
            block_value_percentages_config: HashMap::new(),
            entity_damage_config: EntityDamage::Medium,
            entity_damage_player_config: EntityDamagePlayer::Medium,
            entity_class_damage_config: HashMap::new(),
            player_safe_zone_hours_config: 24,
            player_safe_zone_level_config: 10,
            build_create_config: true,
            build_destroy_config: BlockDamage::Medium,
            build_change_block_config: true,
        }
    }
}

impl Default for Model {
    fn default() -> Self {
        Self {
            players: HashMap::new(),
            worlds: HashMap::new(),
            current_world: None,
            game_state: GameState::Stopped,
            config: ModelConfig::default(),
            statistics: ServerStatistics {
                total_connections: 0,
                successful_connections: 0,
                failed_connections: 0,
                total_playtime: Duration::ZERO,
                average_ping: 0,
                total_kills: 0,
                total_deaths: 0,
                uptime: Duration::ZERO,
                started_at: None,
                last_backup: None,
            },
            next_player_id: 1,
            admin_list: vec![],
            ban_list: vec![],
            whitelist: vec![],
            event_log: Vec::new(),
            max_event_log: 1000,
        }
    }
}

impl Model {
    pub fn new() -> Self { Self::default() }

    pub fn create_world(&mut self, name: &str, seed: u64, size: (u32, u32), game_mode: GameMode, difficulty: Difficulty) -> bool {
        if self.worlds.contains_key(name) {
            return false;
        }
        let world = World {
            name: name.into(),
            seed,
            size,
            game_mode,
            difficulty,
            day_night_cycle: true,
            blood_moon_frequency: BloodMoonFrequency::Monthly,
            max_players: self.config.max_players,
            pvp_enabled: true,
            pve_enabled: true,
            friendly_fire: false,
            drop_on_death: true,
            loot_abundance: LootAbundance::Normal,
            block_damage: BlockDamage::Medium,
            block_damage_player: BlockDamagePlayer::Medium,
            block_value_percentages: HashMap::new(),
            entity_damage: EntityDamage::Medium,
            entity_damage_player: EntityDamagePlayer::Medium,
            entity_class_damage: HashMap::new(),
            player_safe_zone_hours: 24,
            player_safe_zone_level: 10,
            build_create: true,
            build_destroy: BlockDamage::Medium,
            build_change_block: true,
        };
        self.worlds.insert(name.into(), world);
        self.log_event("World created", Some(format!("World '{}' created with seed {}", name, seed)));
        true
    }

    pub fn delete_world(&mut self, name: &str) -> bool {
        if self.worlds.remove(name).is_some() {
            self.log_event("World deleted", Some(format!("World '{}' deleted", name)));
            return true;
        }
        false
    }

    pub fn start_server(&mut self, world_name: &str) -> bool {
        if self.game_state != GameState::Stopped {
            return false;
        }
        if !self.worlds.contains_key(world_name) {
            return false;
        }
        self.current_world = Some(world_name.into());
        self.game_state = GameState::Starting;
        self.statistics.started_at = Some(SystemTime::now());
        self.game_state = GameState::Running;
        self.log_event("Server started", Some(format!("Server started with world '{}'", world_name)));
        true
    }

    pub fn stop_server(&mut self) -> bool {
        if self.game_state != GameState::Running {
            return false;
        }
        self.game_state = GameState::ShuttingDown;
        for player in self.players.values_mut() {
            player.status = PlayerStatus::Disconnected;
        }
        self.game_state = GameState::Stopped;
        self.current_world = None;
        if let Some(started) = self.statistics.started_at {
            self.statistics.uptime = SystemTime::now().duration_since(started).unwrap_or(Duration::ZERO);
        }
        self.log_event("Server stopped", None);
        true
    }

    pub fn pause_server(&mut self) -> bool {
        if self.game_state == GameState::Running {
            self.game_state = GameState::Paused;
            self.log_event("Server paused", None);
            return true;
        }
        false
    }

    pub fn resume_server(&mut self) -> bool {
        if self.game_state == GameState::Paused {
            self.game_state = GameState::Running;
            self.log_event("Server resumed", None);
            return true;
        }
        false
    }

    pub fn player_join(&mut self, name: &str, ip_address: &str) -> Option<String> {
        if self.game_state != GameState::Running {
            return None;
        }
        let world = self.current_world.as_ref()
            .and_then(|w| self.worlds.get(w))?;
        if self.players.len() >= world.max_players as usize {
            self.statistics.failed_connections += 1;
            return None;
        }
        if !self.whitelist.is_empty() && !self.whitelist.contains(&name.to_string()) {
            self.statistics.failed_connections += 1;
            return None;
        }
        if self.ban_list.contains(&name.to_string()) {
            self.statistics.failed_connections += 1;
            return None;
        }
        let player_id = format!("player-{}", self.next_player_id);
        self.next_player_id += 1;
        let player = Player {
            id: player_id.clone(),
            name: name.into(),
            ip_address: ip_address.into(),
            ping: 0,
            status: PlayerStatus::Connected,
            joined_at: SystemTime::now(),
            last_seen: SystemTime::now(),
            playtime: Duration::ZERO,
            kills: 0,
            deaths: 0,
            score: 0,
            position: (0.0, 0.0, 0.0),
            health: 100.0,
            max_health: 100.0,
            stamina: 100.0,
            max_stamina: 100.0,
            experience: 0,
            level: 1,
            is_admin: self.admin_list.contains(&name.to_string()),
            is_op: false,
            is_vip: false,
        };
        self.players.insert(player_id.clone(), player);
        self.statistics.total_connections += 1;
        self.statistics.successful_connections += 1;
        self.log_event("Player joined", Some(format!("Player '{}' joined", name)));
        Some(player_id)
    }

    pub fn player_leave(&mut self, player_id: &str) -> bool {
        if let Some(player) = self.players.remove(player_id) {
            self.statistics.total_playtime += player.playtime;
            self.log_event("Player left", Some(format!("Player '{}' left", player.name)));
            return true;
        }
        false
    }

    pub fn player_disconnect(&mut self, player_id: &str) -> bool {
        let player_name = self.players.get(player_id).map(|p| p.name.clone());
        if let Some(player) = self.players.get_mut(player_id) {
            player.status = PlayerStatus::Disconnected;
            if let Some(name) = player_name {
                self.log_event("Player disconnected", Some(format!("Player '{}' disconnected", name)));
            }
            return true;
        }
        false
    }

    pub fn player_reconnect(&mut self, player_id: &str) -> bool {
        let player_name = self.players.get(player_id).map(|p| p.name.clone());
        if let Some(player) = self.players.get_mut(player_id) {
            player.status = PlayerStatus::Connected;
            player.last_seen = SystemTime::now();
            if let Some(name) = player_name {
                self.log_event("Player reconnected", Some(format!("Player '{}' reconnected", name)));
            }
            return true;
        }
        false
    }

    pub fn update_player_ping(&mut self, player_id: &str, ping: u32) -> bool {
        if let Some(player) = self.players.get_mut(player_id) {
            player.ping = ping;
            player.last_seen = SystemTime::now();
            return true;
        }
        false
    }

    pub fn update_player_position(&mut self, player_id: &str, position: (f32, f32, f32)) -> bool {
        if let Some(player) = self.players.get_mut(player_id) {
            player.position = position;
            player.last_seen = SystemTime::now();
            return true;
        }
        false
    }

    pub fn update_player_health(&mut self, player_id: &str, health: f32) -> bool {
        if let Some(player) = self.players.get_mut(player_id) {
            player.health = health.min(player.max_health);
            return true;
        }
        false
    }

    pub fn player_kill(&mut self, player_id: &str) -> bool {
        if let Some(player) = self.players.get_mut(player_id) {
            player.kills += 1;
            player.score += 100;
            self.statistics.total_kills += 1;
            return true;
        }
        false
    }

    pub fn player_death(&mut self, player_id: &str) -> bool {
        if let Some(player) = self.players.get_mut(player_id) {
            player.deaths += 1;
            player.health = 0.0;
            self.statistics.total_deaths += 1;
            return true;
        }
        false
    }

    pub fn player_gain_experience(&mut self, player_id: &str, exp: u64) -> bool {
        if let Some(player) = self.players.get_mut(player_id) {
            player.experience += exp;
            let new_level = ((player.experience / 1000) + 1) as u32;
            if new_level > player.level {
                player.level = new_level;
                player.max_health = 100.0 + (player.level as f32 * 10.0);
                player.health = player.max_health;
            }
            return true;
        }
        false
    }

    pub fn add_admin(&mut self, player_name: &str) -> bool {
        if !self.admin_list.contains(&player_name.to_string()) {
            self.admin_list.push(player_name.into());
            for player in self.players.values_mut() {
                if player.name == player_name {
                    player.is_admin = true;
                }
            }
            return true;
        }
        false
    }

    pub fn remove_admin(&mut self, player_name: &str) -> bool {
        if let Some(index) = self.admin_list.iter().position(|n| n == player_name) {
            self.admin_list.remove(index);
            for player in self.players.values_mut() {
                if player.name == player_name {
                    player.is_admin = false;
                }
            }
            return true;
        }
        false
    }

    pub fn ban_player(&mut self, player_name: &str) -> bool {
        if !self.ban_list.contains(&player_name.to_string()) {
            self.ban_list.push(player_name.into());
            let player_ids_to_disconnect: Vec<String> = self.players.iter()
                .filter(|(_, p)| p.name == player_name)
                .map(|(id, _)| id.clone())
                .collect();
            for player_id in player_ids_to_disconnect {
                self.player_disconnect(&player_id);
            }
            self.log_event("Player banned", Some(format!("Player '{}' banned", player_name)));
            return true;
        }
        false
    }

    pub fn unban_player(&mut self, player_name: &str) -> bool {
        if let Some(index) = self.ban_list.iter().position(|n| n == player_name) {
            self.ban_list.remove(index);
            return true;
        }
        false
    }

    pub fn add_whitelist(&mut self, player_name: &str) -> bool {
        if !self.whitelist.contains(&player_name.to_string()) {
            self.whitelist.push(player_name.into());
            return true;
        }
        false
    }

    pub fn remove_whitelist(&mut self, player_name: &str) -> bool {
        if let Some(index) = self.whitelist.iter().position(|n| n == player_name) {
            self.whitelist.remove(index);
            return true;
        }
        false
    }

    pub fn kick_player(&mut self, player_id: &str, reason: &str) -> bool {
        let player_name = self.players.get(player_id).map(|p| p.name.clone());
        if let Some(name) = player_name {
            self.player_disconnect(player_id);
            self.log_event("Player kicked", Some(format!("Player '{}' kicked: {}", name, reason)));
            return true;
        }
        false
    }

    pub fn backup_world(&mut self) -> bool {
        if let Some(world_name) = self.current_world.clone() {
            let backup_name = format!("{}_{}", world_name, SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs());
            self.log_event("World backup", Some(format!("World '{}' backed up as '{}'", world_name, backup_name)));
            self.statistics.last_backup = Some(SystemTime::now());
            return true;
        }
        false
    }

    pub fn log_event(&mut self, event_type: &str, details: Option<String>) {
        let event = EventLog {
            timestamp: SystemTime::now(),
            event_type: event_type.into(),
            details,
        };
        self.event_log.push(event);
        if self.event_log.len() > self.max_event_log {
            self.event_log.remove(0);
        }
    }

    // Configuration setters
    pub fn set_server_name(&mut self, name: &str) {
        self.config.server_name = name.into();
    }

    pub fn set_server_description(&mut self, description: &str) {
        self.config.server_description = description.into();
    }

    pub fn set_max_players(&mut self, max_players: u32) {
        self.config.max_players = max_players;
    }

    pub fn set_password(&mut self, password: Option<String>) {
        self.config.server_password = password;
    }

    pub fn set_visible(&mut self, visible: bool) {
        self.config.visible = visible;
    }

    pub fn set_pvp_enabled(&mut self, enabled: bool) {
        self.config.pvp_enabled = enabled;
    }

    pub fn set_pve_enabled(&mut self, enabled: bool) {
        self.config.pve_enabled = enabled;
    }

    pub fn set_allow_crossplay(&mut self, enabled: bool) {
        self.config.allow_crossplay = enabled;
    }

    pub fn set_server_port(&mut self, port: u16) {
        self.config.server_port = port;
    }

    pub fn set_control_port(&mut self, port: u16) {
        self.config.control_port = port;
    }

    pub fn set_max_ping(&mut self, ping: u32) {
        self.config.max_ping = ping;
    }

    pub fn set_kick_idle_players(&mut self, enabled: bool) {
        self.config.kick_idle_players = enabled;
    }

    pub fn set_idle_timeout(&mut self, timeout: Duration) {
        self.config.idle_timeout = timeout;
    }

    pub fn set_save_interval(&mut self, interval: Duration) {
        self.config.save_interval = interval;
    }

    pub fn set_backup_interval(&mut self, interval: Duration) {
        self.config.backup_interval = interval;
    }

    pub fn set_max_backups(&mut self, max_backups: usize) {
        self.config.max_backups = max_backups;
    }

    pub fn set_enable_rcon(&mut self, enabled: bool) {
        self.config.enable_rcon = enabled;
    }

    pub fn set_rcon_port(&mut self, port: u16) {
        self.config.rcon_port = port;
    }

    pub fn set_rcon_password(&mut self, password: Option<String>) {
        self.config.rcon_password = password;
    }

    pub fn set_enable_web_api(&mut self, enabled: bool) {
        self.config.enable_web_api = enabled;
    }

    pub fn set_web_api_port(&mut self, port: u16) {
        self.config.web_api_port = port;
    }

    pub fn set_enable_stats(&mut self, enabled: bool) {
        self.config.enable_stats = enabled;
    }

    pub fn set_stats_port(&mut self, port: u16) {
        self.config.stats_port = port;
    }

    pub fn set_enable_mods(&mut self, enabled: bool) {
        self.config.enable_mods = enabled;
    }

    pub fn set_mods_dir(&mut self, dir: &str) {
        self.config.mods_dir = dir.into();
    }

    pub fn set_world_dir(&mut self, dir: &str) {
        self.config.world_dir = dir.into();
    }

    pub fn set_save_dir(&mut self, dir: &str) {
        self.config.save_dir = dir.into();
    }

    pub fn set_log_dir(&mut self, dir: &str) {
        self.config.log_dir = dir.into();
    }

    // Accessors
    pub fn get_players(&self) -> Vec<&Player> {
        self.players.values().collect()
    }

    pub fn get_player(&self, player_id: &str) -> Option<&Player> {
        self.players.get(player_id)
    }

    pub fn get_mut_player(&mut self, player_id: &str) -> Option<&mut Player> {
        self.players.get_mut(player_id)
    }

    pub fn get_worlds(&self) -> Vec<&World> {
        self.worlds.values().collect()
    }

    pub fn get_world(&self, name: &str) -> Option<&World> {
        self.worlds.get(name)
    }

    pub fn get_current_world(&self) -> Option<&World> {
        self.current_world.as_ref()
            .and_then(|name| self.worlds.get(name))
    }

    pub fn get_game_state(&self) -> &GameState {
        &self.game_state
    }

    pub fn get_statistics(&self) -> &ServerStatistics {
        &self.statistics
    }

    pub fn get_event_log(&self) -> &[EventLog] {
        &self.event_log
    }

    pub fn get_admin_list(&self) -> &[String] {
        &self.admin_list
    }

    pub fn get_ban_list(&self) -> &[String] {
        &self.ban_list
    }

    pub fn get_whitelist(&self) -> &[String] {
        &self.whitelist
    }

    pub fn players(&self) -> &HashMap<String, Player> { &self.players }
    pub fn worlds(&self) -> &HashMap<String, World> { &self.worlds }
    pub fn current_world(&self) -> Option<&str> { self.current_world.as_deref() }
    pub fn game_state(&self) -> &GameState { &self.game_state }
    pub fn config(&self) -> &ModelConfig { &self.config }
    pub fn statistics(&self) -> &ServerStatistics { &self.statistics }
    pub fn next_player_id(&self) -> u32 { self.next_player_id }
    pub fn admin_list(&self) -> &[String] { &self.admin_list }
    pub fn ban_list(&self) -> &[String] { &self.ban_list }
    pub fn whitelist(&self) -> &[String] { &self.whitelist }
    pub fn event_log(&self) -> &[EventLog] { &self.event_log }
    pub fn max_event_log(&self) -> usize { self.max_event_log }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn seven_days_basic_operations() {
        let mut m = Model::new();
        assert!(m.players.is_empty());
        assert!(m.worlds.is_empty());
        assert_eq!(m.game_state(), &GameState::Stopped);
    }

    #[test]
    fn seven_days_world_management() {
        let mut m = Model::new();
        assert!(m.create_world("TestWorld", 12345, (4096, 4096), GameMode::Survival, Difficulty::Normal));
        assert!(m.worlds.contains_key("TestWorld"));
        assert!(m.get_world("TestWorld").is_some());
        assert!(m.delete_world("TestWorld"));
        assert!(!m.worlds.contains_key("TestWorld"));
    }

    #[test]
    fn seven_days_server_lifecycle() {
        let mut m = Model::new();
        m.create_world("TestWorld", 12345, (4096, 4096), GameMode::Survival, Difficulty::Normal);
        assert!(m.start_server("TestWorld"));
        assert_eq!(m.game_state(), &GameState::Running);
        assert!(m.pause_server());
        assert_eq!(m.game_state(), &GameState::Paused);
        assert!(m.resume_server());
        assert_eq!(m.game_state(), &GameState::Running);
        assert!(m.stop_server());
        assert_eq!(m.game_state(), &GameState::Stopped);
    }

    #[test]
    fn seven_days_player_management() {
        let mut m = Model::new();
        m.create_world("TestWorld", 12345, (4096, 4096), GameMode::Survival, Difficulty::Normal);
        m.start_server("TestWorld");
        let player_id = m.player_join("Player1", "192.168.1.100");
        assert!(player_id.is_some());
        let pid = player_id.as_ref().unwrap();
        assert!(m.players.contains_key(pid));
        assert!(m.player_leave(pid));
        assert!(!m.players.contains_key(pid));
    }

    #[test]
    fn seven_days_player_stats() {
        let mut m = Model::new();
        m.create_world("TestWorld", 12345, (4096, 4096), GameMode::Survival, Difficulty::Normal);
        m.start_server("TestWorld");
        let player_id = m.player_join("Player1", "192.168.1.100").unwrap();
        assert!(m.update_player_ping(&player_id, 50));
        assert!(m.update_player_position(&player_id, (100.0, 200.0, 50.0)));
        assert!(m.update_player_health(&player_id, 75.0));
        assert!(m.player_kill(&player_id));
        assert!(m.player_death(&player_id));
        assert!(m.player_gain_experience(&player_id, 500));
        let player = m.get_player(&player_id).unwrap();
        assert_eq!(player.ping, 50);
        assert_eq!(player.kills, 1);
        assert_eq!(player.deaths, 1);
        assert_eq!(player.level, 1);
    }

    #[test]
    fn seven_days_admin_management() {
        let mut m = Model::new();
        m.create_world("TestWorld", 12345, (4096, 4096), GameMode::Survival, Difficulty::Normal);
        m.start_server("TestWorld");
        let player_id = m.player_join("Player1", "192.168.1.100").unwrap();
        assert!(m.add_admin("Player1"));
        assert!(m.get_player(&player_id).unwrap().is_admin);
        assert!(m.remove_admin("Player1"));
        assert!(!m.get_player(&player_id).unwrap().is_admin);
    }

    #[test]
    fn seven_days_ban_management() {
        let mut m = Model::new();
        m.create_world("TestWorld", 12345, (4096, 4096), GameMode::Survival, Difficulty::Normal);
        m.start_server("TestWorld");
        let player_id = m.player_join("Player1", "192.168.1.100").unwrap();
        assert!(m.ban_player("Player1"));
        assert_eq!(m.get_player(&player_id).unwrap().status, PlayerStatus::Disconnected);
        assert!(m.ban_list.contains(&"Player1".to_string()));
        assert!(m.unban_player("Player1"));
        assert!(!m.ban_list.contains(&"Player1".to_string()));
    }

    #[test]
    fn seven_days_backup() {
        let mut m = Model::new();
        m.create_world("TestWorld", 12345, (4096, 4096), GameMode::Survival, Difficulty::Normal);
        m.start_server("TestWorld");
        assert!(m.backup_world());
        assert!(m.statistics.last_backup.is_some());
    }

    #[test]
    fn seven_days_settings() {
        let mut m = Model::new();
        m.set_server_name("My Server");
        m.set_server_description("A 7 Days to Die server");
        m.set_max_players(16);
        m.set_password(Some("secret123".into()));
        m.set_visible(false);
        m.set_pvp_enabled(false);
        m.set_pve_enabled(true);
        assert_eq!(m.config().server_name, "My Server");
        assert_eq!(m.config().server_description, "A 7 Days to Die server");
        assert_eq!(m.config().max_players, 16);
        assert_eq!(m.config().server_password, Some("secret123".into()));
        assert!(!m.config().visible);
        assert!(!m.config().pvp_enabled);
        assert!(m.config().pve_enabled);
    }
}
