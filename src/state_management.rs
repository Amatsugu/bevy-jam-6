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
pub struct GameplaySystems;

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub struct GameStartSystems;

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub struct GameWaitingSystems;

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub struct GameCleanupSystems;

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub struct GameOverSystems;

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub struct ResetSystems;
