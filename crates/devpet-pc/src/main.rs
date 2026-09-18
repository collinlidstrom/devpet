use devpet_core::{Action, Mood, PetState};
use macroquad::prelude::*;

const W:f32=160.; const H:f32=144.;
const BG:Color=Color::new(0.82,0.87,0.68,1.0);
const INK:Color=Color::new(0.10,0.16,0.12,1.0);

fn conf()->Conf { Conf{window_title:"DevPet".into(),window_width:640,window_height:576,high_dpi:true,..Default::default()} }

fn txt(s:&str,x:f32,y:f32,size:u16){ draw_text_ex(s,x,y,TextParams{font_size:size,color:INK,..Default::default()}); }
fn bar(x:f32,y:f32,v:u8){ draw_rectangle_lines(x,y,27.,4.,1.,INK); draw_rectangle(x+1.,y+1.,25.*v as f32/100.,2.,INK); }

fn byte(x:f32,y:f32,p:&PetState,t:f32){
    let bob=if (t*2.) as i32%2==0 {0.} else {1.};
    // Orb Byte: original 32x32 silhouette with central Core.
    draw_circle(x+16.,y+15.+bob,12.,INK);
    draw_circle(x+16.,y+15.+bob,9.,BG);
    draw_rectangle(x+10.,y+12.+bob,3.,3.,INK); draw_rectangle(x+20.,y+12.+bob,3.,3.,INK);
    let mouth_y=y+21.+bob;
    match p.mood { Mood::Happy=>{draw_line(x+13.,mouth_y,x+16.,mouth_y+2.,1.,INK);draw_line(x+16.,mouth_y+2.,x+20.,mouth_y,1.,INK);}
        Mood::Sick=>draw_line(x+13.,mouth_y+1.,x+20.,mouth_y+1.,1.,INK),
        _=>draw_line(x+14.,mouth_y,x+19.,mouth_y,1.,INK) }
    // Core
    let pulse=if (t*4.) as i32%2==0 {2.} else {1.};
    draw_rectangle(x+15.-pulse/2.,y+25.+bob,pulse+1.,pulse+1.,INK);
    draw_rectangle(x+8.,y+27.+bob,6.,3.,INK); draw_rectangle(x+19.,y+27.+bob,6.,3.,INK);
}

#[macroquad::main(conf)]
async fn main(){
    let mut pet=PetState::default(); let mut selected=0usize; let actions=["CODE","CARE","PLAY","MENU"];
    let mut last=get_time();
    loop {
        if get_time()-last>=1.0 { pet.advance_minutes(1); last=get_time(); }
        if is_key_pressed(KeyCode::Right)||is_key_pressed(KeyCode::D){selected=(selected+1)%4;}
        if is_key_pressed(KeyCode::Left)||is_key_pressed(KeyCode::A){selected=(selected+3)%4;}
        if is_key_pressed(KeyCode::Key1){selected=0;} if is_key_pressed(KeyCode::Key2){selected=1;}
        if is_key_pressed(KeyCode::Key3){selected=2;} if is_key_pressed(KeyCode::Key4){selected=3;}
        if is_key_pressed(KeyCode::Enter)||is_key_pressed(KeyCode::Z){
            match selected {0=>pet.apply(Action::Code),1=>pet.apply(Action::Feed),2=>pet.apply(Action::Play),_=>pet.apply(Action::Rest)}
        }
        clear_background(BLACK);
        let scale=(screen_width()/W).min(screen_height()/H).floor().max(1.);
        let ox=(screen_width()-W*scale)/2.; let oy=(screen_height()-H*scale)/2.;
        set_camera(&Camera2D{zoom:vec2(2./(W*scale),-2./(H*scale)),target:vec2(screen_width()/2.,screen_height()/2.),..Default::default()});
        draw_rectangle(ox,oy,W*scale,H*scale,BG);
        // Draw logical UI using scaled coordinates via a temporary camera transform.
        set_default_camera();
        let sx=|v:f32| ox+v*scale; let sy=|v:f32| oy+v*scale;
        draw_rectangle(ox,oy,W*scale,H*scale,BG);
        // render primitives scaled manually
        let fs=(6.*scale) as u16;
        draw_text_ex("DEVPET",sx(5.),sy(10.),TextParams{font_size:fs,color:INK,..Default::default()});
        draw_text_ex(&format!("LV {}",pet.level),sx(137.),sy(10.),TextParams{font_size:fs,color:INK,..Default::default()});
        // pet rendered in logical coordinates by camera
        set_camera(&Camera2D{zoom:vec2(2./W,-2./H),target:vec2(W/2.,H/2.),viewport:Some((ox as i32,oy as i32,(W*scale) as i32,(H*scale) as i32)),..Default::default()});
        byte(64.,30.,&pet,get_time() as f32);
        txt(match pet.mood {Mood::Coding=>"Byte is coding...",Mood::Happy=>"Byte is happy!",Mood::Sleeping=>"Byte is resting.",Mood::Sick=>"Byte needs care.",_=>"Byte is vibing."},42.,70.,7);
        txt("HP",5.,82.,6);bar(17.,79.,pet.health); txt("EN",48.,82.,6);bar(60.,79.,pet.energy);
        txt("FO",91.,82.,6);bar(103.,79.,pet.focus);
        draw_line(3.,91.,157.,91.,1.,INK);
        for (i,a) in actions.iter().enumerate(){ let x=5.+i as f32*39.; if i==selected{draw_rectangle(x-2.,96.,36.,13.,INK); draw_text_ex(a,x,105.,TextParams{font_size:6,color:BG,..Default::default()});}else{txt(a,x,105.,6);} }
        txt(&format!("XP {:03}  PROJECTS {:02}",pet.xp,pet.projects),5.,122.,6);
        txt("ARROWS/1-4  ENTER/Z",5.,136.,6);
        set_default_camera();
        next_frame().await;
    }
}
