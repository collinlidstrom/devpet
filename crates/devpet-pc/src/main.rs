mod storage;
use devpet_core::{Action,Mood,PetState,Project,Species};
use macroquad::prelude::*;
const W:f32=160.;const H:f32=144.;const BG:Color=Color::new(.82,.87,.68,1.);const INK:Color=Color::new(.10,.16,.12,1.);
#[derive(Clone,Copy,PartialEq)]enum Screen{Home,Care,Code,Play,BugSquash,Profile,Evolution}
fn conf()->Conf{Conf{window_title:"DevPet".into(),window_width:640,window_height:576,high_dpi:true,..Default::default()}}
fn txt(s:&str,x:f32,y:f32,n:u16){draw_text_ex(s,x,y,TextParams{font_size:n,color:INK,..Default::default()});}
fn bar(x:f32,y:f32,v:u8){draw_rectangle_lines(x,y,42.,5.,1.,INK);draw_rectangle(x+1.,y+1.,40.*v as f32/100.,3.,INK);}
fn choice(label:&str,y:f32,on:bool){if on{draw_rectangle(10.,y-8.,140.,11.,INK);draw_text_ex(label,14.,y,TextParams{font_size:7,color:BG,..Default::default()});}else{txt(label,14.,y,7);}}
fn byte(x:f32,y:f32,p:&PetState,t:f32){let b=if(t*2.)as i32%2==0{0.}else{1.};draw_circle(x+16.,y+15.+b,12.,INK);draw_circle(x+16.,y+15.+b,9.,BG);draw_rectangle(x+10.,y+12.+b,3.,3.,INK);draw_rectangle(x+20.,y+12.+b,3.,3.,INK);let m=y+21.+b;match p.mood{Mood::Happy=>{draw_line(x+13.,m,x+16.,m+2.,1.,INK);draw_line(x+16.,m+2.,x+20.,m,1.,INK);}Mood::Sick=>draw_line(x+13.,m+1.,x+20.,m+1.,1.,INK),_=>draw_line(x+14.,m,x+19.,m,1.,INK)}let q=if(t*4.)as i32%2==0{3.}else{2.};draw_rectangle(x+15.,y+25.+b,q,q,INK);draw_rectangle(x+8.,y+27.+b,6.,3.,INK);draw_rectangle(x+19.,y+27.+b,6.,3.,INK);}
fn creature(x:f32,y:f32,p:&PetState,t:f32){
 if p.species==Species::Egg{draw_ellipse(x+16.,y+17.,11.,14.,0.,INK);draw_ellipse(x+16.,y+17.,8.,11.,0.,BG);let crack=if(t*3.)as i32%2==0{0.}else{1.};draw_line(x+11.,y+16.,x+15.,y+19.+crack,1.,INK);draw_line(x+15.,y+19.+crack,x+20.,y+15.,1.,INK);return}
 byte(x,y,p,t);
 match p.species{
  Species::Bot=>{draw_rectangle(x+5.,y+5.,4.,4.,INK);draw_line(x+7.,y+5.,x+7.,y+1.,1.,INK);draw_circle(x+7.,y,1.5,INK);draw_rectangle(x+25.,y+5.,3.,8.,INK);}
  Species::Beast=>{draw_triangle(vec2(x+5.,y+9.),vec2(x+10.,y+1.),vec2(x+14.,y+9.),INK);draw_triangle(vec2(x+20.,y+9.),vec2(x+25.,y+1.),vec2(x+29.,y+9.),INK);draw_line(x+5.,y+22.,x+1.,y+26.,2.,INK);draw_line(x+28.,y+22.,x+32.,y+26.,2.,INK);}
  Species::Ghost=>{draw_line(x+7.,y+27.,x+11.,y+31.,1.,INK);draw_line(x+11.,y+31.,x+16.,y+27.,1.,INK);draw_line(x+16.,y+27.,x+21.,y+31.,1.,INK);draw_line(x+21.,y+31.,x+26.,y+27.,1.,INK);}
  _=>{}
 }
}
fn nav(sel:&mut usize,n:usize){if is_key_pressed(KeyCode::Down)||is_key_pressed(KeyCode::S){*sel=(*sel+1)%n}if is_key_pressed(KeyCode::Up)||is_key_pressed(KeyCode::W){*sel=(*sel+n-1)%n}}
#[macroquad::main(conf)]async fn main(){
 let mut pet=storage::load();let mut screen=Screen::Home;let mut sel=0usize;let mut last=get_time();
 let mut game_start=0.;let mut hits=0u8;let mut bug_x=30.;let mut bug_y=50.;let mut last_save=get_time();
 loop{
  if get_time()-last>=1.{pet.advance_minutes(1);last=get_time();}if get_time()-last_save>=15.{let _=storage::save(&pet);last_save=get_time();}
  let ok=is_key_pressed(KeyCode::Enter)||is_key_pressed(KeyCode::Z);let back=is_key_pressed(KeyCode::Escape)||is_key_pressed(KeyCode::X);
  if back&&screen!=Screen::Home{screen=Screen::Home;sel=0;}
  match screen{
   Screen::Home=>{if is_key_pressed(KeyCode::Right)||is_key_pressed(KeyCode::D){sel=(sel+1)%4}if is_key_pressed(KeyCode::Left)||is_key_pressed(KeyCode::A){sel=(sel+3)%4}if ok{screen=match sel{0=>Screen::Code,1=>Screen::Care,2=>Screen::Play,_=>Screen::Profile};sel=0;}}
   Screen::Care=>{nav(&mut sel,4);if ok{pet.apply(match sel{0=>Action::Feed,1=>Action::Coffee,2=>Action::Rest,_=>Action::Medicine});}}
   Screen::Code=>{nav(&mut sel,4);if ok{pet.run_project(match sel{0=>Project::FixBug,1=>Project::BuildFeature,2=>Project::Refactor,_=>Project::ShipRelease});}}
   Screen::Profile=>{nav(&mut sel,2);if ok{screen=if sel==0{Screen::Evolution}else{Screen::Home};sel=0;}}
   Screen::Evolution=>{if ok||back{screen=Screen::Profile;sel=0;}}
   Screen::Play=>{if ok{screen=Screen::BugSquash;game_start=get_time();hits=0;bug_x=30.;bug_y=50.;}}
   Screen::BugSquash=>{let elapsed=get_time()-game_start;if elapsed>=20.{pet.bug_squash_reward(hits);screen=Screen::Home;sel=0;}else{let(mx,my)=mouse_position();let scale=(screen_width()/W).min(screen_height()/H).floor().max(1.);let ox=(screen_width()-W*scale)/2.;let oy=(screen_height()-H*scale)/2.;let lx=(mx-ox)/scale;let ly=(my-oy)/scale;if is_mouse_button_pressed(MouseButton::Left)&&(lx-bug_x).abs()<8.&&(ly-bug_y).abs()<8.{hits=hits.saturating_add(1);bug_x=15.+((hits as f32*37.)%130.);bug_y=30.+((hits as f32*23.)%70.);}}}
  }
  clear_background(BLACK);let scale=(screen_width()/W).min(screen_height()/H).floor().max(1.);let ox=(screen_width()-W*scale)/2.;let oy=(screen_height()-H*scale)/2.;
  set_camera(&Camera2D{zoom:vec2(2./W,-2./H),target:vec2(W/2.,H/2.),viewport:Some((ox as i32,oy as i32,(W*scale)as i32,(H*scale)as i32)),..Default::default()});clear_background(BG);
  match screen{
   Screen::Home=>{txt("DEVPET",5.,10.,7);txt(&format!("LV {}",pet.level),135.,10.,7);creature(64.,25.,&pet,get_time()as f32);txt(match pet.mood{Mood::Coding=>"Byte shipped code.",Mood::Happy=>"Byte is happy!",Mood::Sleeping=>"Byte is resting.",Mood::Sick=>"Byte needs care.",_=>"Byte is vibing."},42.,68.,7);txt("HP",5.,80.,6);bar(17.,77.,pet.health);txt("EN",65.,80.,6);bar(77.,77.,pet.energy);draw_line(3.,90.,157.,90.,1.,INK);for(i,a)in["CODE","CARE","PLAY","MENU"].iter().enumerate(){let x=5.+i as f32*39.;if i==sel{draw_rectangle(x-2.,96.,36.,13.,INK);draw_text_ex(a,x,105.,TextParams{font_size:6,color:BG,..Default::default()});}else{txt(a,x,105.,6)}}txt(&format!("XP {}  PROJECTS {}  BUGS {}",pet.xp,pet.projects,pet.bugs_fixed),5.,123.,6);txt("ARROWS + ENTER/Z",5.,137.,6);}
   Screen::Care=>{txt("< CARE",5.,10.,7);creature(64.,14.,&pet,get_time()as f32);txt(&format!("HEALTH {:3}",pet.health),5.,57.,6);bar(55.,53.,pet.health);txt(&format!("ENERGY {:3}",pet.energy),5.,66.,6);bar(55.,62.,pet.energy);txt(&format!("HAPPY  {:3}",pet.happiness),5.,75.,6);bar(55.,71.,pet.happiness);txt(&format!("FOCUS  {:3}",pet.focus),5.,84.,6);bar(55.,80.,pet.focus);for(i,a)in["FOOD +FULLNESS","COFFEE +FOCUS","SLEEP +ENERGY","MEDICINE +HEALTH"].iter().enumerate(){choice(a,99.+i as f32*10.,i==sel)}}
   Screen::Code=>{txt("< CODE / CHOOSE PROJECT",5.,10.,7);let labels=["FIX A BUG      +15 XP","BUILD FEATURE  +30 XP","REFACTOR       +20 XP","SHIP RELEASE   +50 XP"];for(i,a)in labels.iter().enumerate(){choice(a,35.+i as f32*18.,i==sel)}txt(&format!("ENERGY {}  FOCUS {}",pet.energy,pet.focus),10.,116.,7);txt("UP/DOWN  ENTER   X=BACK",10.,135.,6);}
   Screen::Profile=>{txt("< BYTE PROFILE",5.,10.,7);creature(64.,14.,&pet,get_time()as f32);txt(&format!("FORM {:?}",pet.species),8.,58.,7);txt(&format!("PERSONALITY {:?}",pet.personality()),8.,70.,7);txt(&format!("AGE {}m  LV {}",pet.age_minutes,pet.level),8.,82.,7);txt(&format!("XP {}  PROJECTS {}",pet.xp,pet.projects),8.,94.,7);txt(&format!("BUGS {}  CARE MISS {}",pet.bugs_fixed,pet.care_mistakes),8.,106.,7);choice("EVOLUTION",122.,sel==0);choice("BACK",134.,sel==1);}
   Screen::Evolution=>{txt("< EVOLUTION",5.,10.,7);creature(64.,18.,&pet,get_time()as f32);if pet.species==Species::Egg{txt("A SIGNAL IS FORMING...",27.,72.,7);txt("BYTE HATCHES AT AGE 3m",22.,87.,7);}else if pet.is_evolved(){txt(&format!("BYTE EVOLVED: {:?}",pet.species),25.,72.,8);txt(match pet.species{Species::Bot=>"Built through focused creation.",Species::Beast=>"Raised through play and care.",Species::Ghost=>"Adapted through adversity.",_=>""},15.,90.,6);}else{txt("HOW YOU RAISE BYTE",28.,69.,7);txt("DETERMINES ITS FUTURE.",22.,81.,7);txt(" ?       ?       ? ",30.,103.,10);txt("EVOLUTION AT AGE 240m",20.,121.,7);}txt("ENTER/X = BACK",42.,137.,6);}
   Screen::Play=>{txt("< PLAY",5.,10.,7);txt("BUG SQUASH",45.,45.,10);txt("20 SECOND MOUSE GAME",30.,63.,7);txt("CLICK BUGS BEFORE THEY MOVE!",17.,80.,6);choice("START",110.,true);txt("X = BACK",55.,135.,6);}
   Screen::BugSquash=>{let left=(20.-(get_time()-game_start)).max(0.);txt(&format!("BUG SQUASH   {:02}s",left.ceil()as i32),5.,10.,7);txt(&format!("HITS {}",hits),120.,10.,7);draw_circle(bug_x,bug_y,6.,INK);draw_line(bug_x-8.,bug_y-6.,bug_x+8.,bug_y+6.,1.,INK);draw_line(bug_x+8.,bug_y-6.,bug_x-8.,bug_y+6.,1.,INK);txt("CLICK THE BUG!",48.,132.,7);}
  }set_default_camera();if is_key_pressed(KeyCode::Q){let _=storage::save(&pet);break;}next_frame().await;
 }
}