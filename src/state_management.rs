use bevy::prelude::*;

#[derive(States, Debug, Clone, PartialEq, Eq, Hash)]
pub enum GameplayState {
	Reset,
	Waiting,
	Startup,
	Playing,
	GameOver,
	Cleanup,
}

#[derive(States, Debug, Clone, PartialEq, Eq, Hash)]
pub enum GameOverState {
	Init,
	Wait,
}

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub struct GameplaySet;

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub struct GameStartSet;

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub struct GameWaitingSet;

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub struct GameCleanupSet;

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub struct GameOverSet;

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub struct ResetSet;
