use std::fmt::Debug;
use crate::deck::*;
use crate::hand::*;
use crate::grade::*;

#[derive(Debug, Clone, Copy)]
enum Stage {
	PreFlop,
	Flop,
	Turn,
	River,
}

enum Showdown {
	Win,
	Lose,
	Draw,
}

#[derive(Debug)]
pub struct HoldemGame {
	deck: Deck,
	stage: Stage,
	community_cards: Vec<Card>,
	players_cnt: usize,
	players: Vec<Hand>,
}

impl Clone for HoldemGame {
	fn clone(&self) -> Self {
		let deck : Deck = self.deck.clone();
		let stage : Stage = self.stage;
		let community_cards : Vec<Card> = self.community_cards.clone();
		let players_cnt: usize = self.players_cnt;
		let players: Vec<Hand> = self.players.clone();
		Self {deck, stage, community_cards, players_cnt, players}
	}
}

impl HoldemGame {
	pub fn new(players_cnt: usize) -> Self {
		if players_cnt > 10 {panic!("Too many players on this game.");}
		let deck = Deck::new();
		let mut players = Vec::<Hand>::new();
		for _ in 0..players_cnt {players.push(Hand::new());}
		
		Self {	deck: deck,
			stage: Stage::PreFlop,
			community_cards: Vec::<Card>::new(),
			players_cnt: players_cnt,
			players: players
		}
	}
	
	pub fn add_community_card(&mut self) -> Result<(), &str> {
		let dealt_card : Card = self.deck.deal_cards(1)[0];
		if self.community_cards.len() < 5 {
			for i in 0..self.players_cnt {
				self.players[i].add_card(dealt_card);
			}
			self.community_cards.push(dealt_card);
		}
		else {return Err("Community Cards cannot be over 5 cards.");}
		return Ok(());
	}

	fn preflop(&mut self) -> () {
		self.players.iter_mut().for_each(
			|player|
			while player.len() < 2 { player.add_card(self.deck.deal_cards(1)[0]) }
		);
	}

	fn preflop_to_flop(&mut self) -> () {
	// preflop is the state in which every player gets 2 cards.
	// flop is the state in which 3 community cards are open.
		if let Stage::PreFlop = self.stage {
			for _ in 0..3 { 
				if let Err(msg) = self.add_community_card() {panic!("{}", msg);}
			}
			self.stage = Stage::Flop;
		} else {
			panic!("{}", "preflop_to_flop function must be called at preflop stage.");
		}
	}
	
	fn flop_to_turn(&mut self) -> () {
		if let Stage::Flop = self.stage {
			if let Err(msg) = self.add_community_card() {panic!("{}", msg);}
			self.stage = Stage::Turn;
		} else {
			panic!("{}", "flop_to_turn function must be called at flop stage.");
		}
	}

	fn turn_to_river(&mut self) -> () {
		if let Stage::Turn = self.stage {
			if let Err(msg) = self.add_community_card() {panic!("{}", msg);}
			self.stage = Stage::River;
		} else {
			panic!("{}", "turn_to_river function must be called at turn stage.");
		}
	}
	
	fn play_once(&mut self) -> Showdown {
		self.deck.partial_shuffle();
		// println!("{:?}", self.deck);
		self.preflop();
		//println!("{:?}", self);
		match self.stage {
			Stage::PreFlop => {
				self.preflop_to_flop();
				self.flop_to_turn();
				self.turn_to_river();
			},
			Stage::Flop => {
				self.flop_to_turn();
				self.turn_to_river();
			},
			Stage::Turn => {
				self.turn_to_river();
			},
			Stage::River => {},
		};
		//println!("{:?}", self);
		let my_grade: Grade = Grade::new(&self.players[0].cards);
		let other_grades : Vec<Grade> = self.players.iter()
						.skip(1).map(|player| Grade::new(&player.cards) )
						.collect::<Vec<_>>();
		//println!("{:?}", my_grade);
		//println!("{:?}", other_grades[0]);
		let mut draw_cnt : u8 = 0;
		let mut win_cnt : u8 = 0;
		let mut lose_cnt : u8 = 0;
		other_grades.into_iter().for_each(
			|og| 
			if 	my_grade == og 	{draw_cnt += 1} 
			else if my_grade > og 	{win_cnt += 1} 
			else 			{lose_cnt += 1});
		if lose_cnt > 0 	{Showdown::Lose}
		else if draw_cnt > 0 	{Showdown::Draw}
		else			{Showdown::Win}
	}

	pub fn calc_win_rate(&self) -> f32 {
		let mut win_cnt : usize = 0;
		let mut lose_cnt :usize = 0;
		let mut draw_cnt :usize = 0;
		for _ in 0..50000 {
			let mut cloned_self = self.clone();
			match cloned_self.play_once() {
				Showdown::Win => {win_cnt += 1;},
				Showdown::Lose => {lose_cnt += 1;},
				Showdown::Draw => {draw_cnt += 1;}
			};
		}
		return win_cnt as f32 / (win_cnt as f32 + lose_cnt as f32 + draw_cnt as f32);
	}

	pub fn set_front_cards(&mut self, front_cards: Vec<Card>) -> Result<(), &str> {
		self.players[0].add_cards(&front_cards);
		self.deck.set_front_cards(front_cards)
	}
}


#[cfg(test)]
mod montecarlo_test {
    use crate::holdem::*;
    #[test]
    fn motecarlo_test_00() {	
	let mut game = HoldemGame::new(3);
	let frontcards = vec![Card{shape:0, num:12}, Card{shape:1, num:12}];
	game.set_front_cards(frontcards);
	println!("{:?}{:?}", game.deck.cards[0], game.deck.cards[1]);
	let win_rate = game.calc_win_rate();
	println!("{}", win_rate);
	// assert!(0.8236-0.01 < win_rate && win_rate < 0.8236+0.01 );
	assert!(0.735-0.01 < win_rate && win_rate < 0.735+0.01 );
    }

}
