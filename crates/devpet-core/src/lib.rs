use serde::{Deserialize,Serialize};
pub const MAX_STAT:u8=100;
pub const HATCH_MINUTES:u32=3;
pub const EVOLVE_MINUTES:u32=240;

#[derive(Debug,Clone,Copy,PartialEq,Eq,Serialize,Deserialize)]pub enum Mood{Idle,Happy,Coding,Sleeping,Sick}
#[derive(Debug,Clone,Copy,PartialEq,Eq,Serialize,Deserialize)]pub enum Action{Feed,Coffee,Rest,Play,Code,Medicine}
#[derive(Debug,Clone,Copy,PartialEq,Eq,Serialize,Deserialize)]pub enum Project{FixBug,BuildFeature,Refactor,ShipRelease}
#[derive(Debug,Clone,Copy,PartialEq,Eq,Serialize,Deserialize)]pub enum Species{Egg,Byte,Bot,Beast,Ghost}
#[derive(Debug,Clone,Copy,PartialEq,Eq,Serialize,Deserialize)]pub enum Personality{Curious,Focused,Playful,Resilient}

impl Project{
 pub fn xp(self)->u32{match self{Self::FixBug=>15,Self::BuildFeature=>30,Self::Refactor=>20,Self::ShipRelease=>50}}
 pub fn energy_cost(self)->u8{match self{Self::FixBug=>6,Self::BuildFeature=>14,Self::Refactor=>10,Self::ShipRelease=>22}}
 pub fn focus_cost(self)->u8{match self{Self::FixBug=>5,Self::BuildFeature=>12,Self::Refactor=>9,Self::ShipRelease=>18}}
}
#[derive(Debug,Clone,PartialEq,Eq,Serialize,Deserialize)]
pub struct PetState{
 pub name:String,pub species:Species,pub age_minutes:u32,pub health:u8,pub energy:u8,pub happiness:u8,pub focus:u8,pub fullness:u8,
 pub xp:u32,pub level:u16,pub care_mistakes:u16,pub projects:u16,pub bugs_fixed:u16,pub play_sessions:u16,pub feeds:u16,pub rests:u16,pub mood:Mood,
}
impl Default for PetState{fn default()->Self{Self{name:"Byte".into(),species:Species::Egg,age_minutes:0,health:100,energy:80,happiness:80,focus:60,fullness:75,xp:0,level:1,care_mistakes:0,projects:0,bugs_fixed:0,play_sessions:0,feeds:0,rests:0,mood:Mood::Idle}}}
fn add(v:u8,n:u8)->u8{v.saturating_add(n).min(MAX_STAT)}fn sub(v:u8,n:u8)->u8{v.saturating_sub(n)}
impl PetState{
 pub fn is_hatched(&self)->bool{self.species!=Species::Egg}
 pub fn is_evolved(&self)->bool{matches!(self.species,Species::Bot|Species::Beast|Species::Ghost)}
 pub fn personality(&self)->Personality{if self.projects>=self.play_sessions.saturating_add(3){Personality::Focused}else if self.play_sessions>self.projects{Personality::Playful}else if self.care_mistakes>=3{Personality::Resilient}else{Personality::Curious}}
 pub fn evolution_preview(&self)->Species{if self.care_mistakes>=4{Species::Ghost}else if self.play_sessions.saturating_add(self.feeds)>self.projects.saturating_add(self.rests){Species::Beast}else{Species::Bot}}
 pub fn maybe_evolve(&mut self)->bool{if self.species==Species::Byte&&self.age_minutes>=EVOLVE_MINUTES{self.species=self.evolution_preview();self.mood=Mood::Happy;return true}false}
 pub fn apply(&mut self,a:Action){if !self.is_hatched(){return}match a{
  Action::Feed=>{self.fullness=add(self.fullness,25);self.happiness=add(self.happiness,4);self.feeds=self.feeds.saturating_add(1);self.mood=Mood::Happy}
  Action::Coffee=>{self.focus=add(self.focus,22);self.energy=add(self.energy,8);self.health=sub(self.health,2);self.mood=Mood::Happy}
  Action::Rest=>{self.energy=add(self.energy,35);self.focus=add(self.focus,8);self.rests=self.rests.saturating_add(1);self.mood=Mood::Sleeping}
  Action::Play=>{self.happiness=add(self.happiness,18);self.energy=sub(self.energy,10);self.xp+=5;self.play_sessions=self.play_sessions.saturating_add(1);self.mood=Mood::Happy}
  Action::Code=>{self.run_project(Project::FixBug);return}Action::Medicine=>{self.health=add(self.health,30);self.mood=Mood::Idle}}self.update_level()}
 pub fn run_project(&mut self,p:Project)->bool{if !self.is_hatched()||self.energy<p.energy_cost()||self.focus<p.focus_cost(){return false}self.energy=sub(self.energy,p.energy_cost());self.focus=sub(self.focus,p.focus_cost());self.fullness=sub(self.fullness,4);self.xp+=p.xp();self.projects=self.projects.saturating_add(1);if p==Project::FixBug{self.bugs_fixed=self.bugs_fixed.saturating_add(1)}self.mood=Mood::Coding;self.update_level();true}
 pub fn bug_squash_reward(&mut self,h:u8){if !self.is_hatched(){return}self.bugs_fixed=self.bugs_fixed.saturating_add(h as u16);self.xp+=h as u32*3;self.happiness=add(self.happiness,5u8.saturating_add(h));self.energy=sub(self.energy,8);self.play_sessions=self.play_sessions.saturating_add(1);self.mood=Mood::Happy;self.update_level()}
 fn update_level(&mut self){self.level=1+(self.xp/100)as u16}
 pub fn advance_minutes(&mut self,m:u32){for _ in 0..m{self.age_minutes=self.age_minutes.saturating_add(1);if self.species==Species::Egg{if self.age_minutes>=HATCH_MINUTES{self.species=Species::Byte;self.mood=Mood::Happy}continue}if self.age_minutes%20==0{self.fullness=sub(self.fullness,1)}if self.age_minutes%25==0{self.energy=sub(self.energy,1)}if self.age_minutes%30==0{self.focus=sub(self.focus,1)}if self.fullness==0&&self.age_minutes%30==0{self.health=sub(self.health,2);self.care_mistakes=self.care_mistakes.saturating_add(1)}if self.health<25{self.mood=Mood::Sick}self.maybe_evolve();}}
}
#[cfg(test)]mod tests{use super::*;
 #[test]fn egg_hatches(){let mut p=PetState::default();p.advance_minutes(HATCH_MINUTES);assert_eq!(p.species,Species::Byte)}
 #[test]fn bot_path(){let mut p=PetState::default();p.species=Species::Byte;p.age_minutes=EVOLVE_MINUTES;p.projects=8;p.play_sessions=1;assert!(p.maybe_evolve());assert_eq!(p.species,Species::Bot)}
 #[test]fn beast_path(){let mut p=PetState::default();p.species=Species::Byte;p.age_minutes=EVOLVE_MINUTES;p.feeds=8;p.play_sessions=8;assert!(p.maybe_evolve());assert_eq!(p.species,Species::Beast)}
 #[test]fn ghost_path(){let mut p=PetState::default();p.species=Species::Byte;p.age_minutes=EVOLVE_MINUTES;p.care_mistakes=4;assert!(p.maybe_evolve());assert_eq!(p.species,Species::Ghost)}
 #[test]fn deterministic(){let mut a=PetState::default();let mut b=a.clone();a.advance_minutes(10);b.advance_minutes(10);a.run_project(Project::BuildFeature);b.run_project(Project::BuildFeature);assert_eq!(a,b)}
}