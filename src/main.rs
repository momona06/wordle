mod builtin_words;
use rand::seq::SliceRandom;
use rand::SeedableRng;
use rand::rngs::StdRng;
use std::io::{BufReader, BufRead, Read};
use std::*;
use colored::*;
use clap::{Arg,App};
use text_io::read;
use serde_json::json;
use serde_derive::*;

#[derive(PartialEq)]
enum GameMode {
    AnswerMode,//the mode without the -w/--word args
    AnswerConsoleMode,//the mode that uses the -w/--word args to specify the answer
    RandomMode,
}//confirm the mode of game

#[derive(Debug)]
enum GameDifficulty {
    Normal,
    Difficult,
}//confirm the difficulty of game

#[derive(Copy, Clone)]
enum StateofChar {
    G,
    Y,
    R,
    X,
}

impl StateofChar {
    fn to_char(self) -> char {
        match self 
        {
            StateofChar::G => 'G',
            StateofChar::Y => 'Y',
            StateofChar::R => 'R',
            StateofChar::X => 'X'
        }
    }

    fn to_string(self) -> String {
        match self 
        {
            StateofChar::G => 'G'.to_string(),
            StateofChar::Y => 'Y'.to_string(),
            StateofChar::R => 'R'.to_string(),
            StateofChar::X => 'X'.to_string()
        }
    }
}

struct Tcmdstate {
    num: i32,
    str: String,
}
struct GameStates {
    answer: String,
    days: u64,
    stateofword: Vec<StateofChar>,
    stateofal: Vec<StateofChar>,
    guessword: Vec<char>,
}

struct WordleGame {
    wordle_istty: bool,//if interactive then true if not then false
    gamemode: GameMode,
    gamedifficulty: GameDifficulty,
    seed: u64,
    stats: bool,
    gamestates:GameStates,
}

#[derive(Serialize, Deserialize)]
struct Statesjson {
    total_rounds: u64,
    games: Vec<GameRound>,

}

#[derive(Serialize, Deserialize, Clone)]
struct GameRound {
    answer: String,
    guesses: Vec<String>,
}


/// The main function for the Wordle game, implement your own logic here
fn main() -> Result<(), Box<dyn std::error::Error>> { 
    let mut stateofwordexam: Vec<StateofChar> = Vec::new();
    let mut stateofalexam: Vec<StateofChar> = Vec::new();
    let mut guesswords: Vec<char> = Vec::new();
    let mut randans: Vec<String> = Vec::new();
    let mut guesstcmd: Vec<Tcmdstate> = Vec::new();
    let mut randguess: Vec<Vec<String>> = Vec::new();
    for _ in 1..6 {
        stateofwordexam.push(StateofChar::X);
    }
    for _ in 1..27 {
        stateofalexam.push(StateofChar::X);
    }
    for _ in 1..6{
        guesswords.push('A');
    }
    let gamestatesexam:GameStates = GameStates{ answer: "CARGO".to_string(), days: 1, stateofword: stateofwordexam, stateofal: stateofalexam, guessword: guesswords };
    let mut wordlegame: WordleGame = WordleGame 
    { wordle_istty: false, gamemode: GameMode::AnswerMode, gamedifficulty: GameDifficulty::Normal, seed: 11111111, stats: false, gamestates: gamestatesexam };//init of game
    let games: Vec<GameRound> = Vec::new();
    let mut statejson: Statesjson = Statesjson { total_rounds: 0, games};

    let mut finaltext: Vec<String> = Vec::new();
    let mut accepttext: Vec<String> = Vec::new();
    let mut finaltxt: String = String::new();
    let mut accepttxt: String = String::new();
    let mut statetxt: String = String::new();
    let mut sjson: bool = false;
    let mut fjson: bool = false;
    let mut ajson: bool = false;
    let mut endcode: bool = false;
    let mut winwin: i32 = 0;
    let mut loselose:i32 = 0;
    let mut sumsumsum: i32 = 0;
    let mut sumsum: i32 = 0;
    //init of the variables

    let _is_tty = atty::is(atty::Stream::Stdout);

    let matched = App::new("Wordle")
                    .arg(Arg::with_name("word")
                            .short('w')
                            .long("word")
                            .value_name("ANSWER")
                            .help("word command")
                    )
                    .arg(Arg::with_name("random")
                            .short('r')
                            .long("random")
                            .required(false)
                            .help("random command")
                    )
                    .arg(Arg::with_name("difficulty")
                        .short('D')
                        .long("difficult")
                        .required(false)
                        .help("difficulty command")
                    )
                    .arg(Arg::with_name("stats")
                        .short('t')
                        .long("stats")
                        .required(false)
                        .help("states command")
                    )
                    .arg(Arg::with_name("days")
                        .short('d')
                        .long("day")
                        .value_name("DAYS")
                        .help("days command")
                    )
                    .arg(Arg::with_name("seeds")
                        .short('s')
                        .long("seed")
                        .value_name("SEEDS")
                        .help("seeds command")
                    ) 
                    .arg(Arg::with_name("finalset")
                        .short('f')
                        .long("final-set")
                        .value_name("FINAL")
                        .help("final command")
                    ) 
                    .arg(Arg::with_name("acceptset")
                        .short('a')
                        .long("acceptable-set")
                        .value_name("ACCEPT")
                        .help("accept command")
                    ) 
                    .arg(Arg::with_name("State")
                        .short('S')
                        .long("state")
                        .default_missing_value("Errorfile")
                        .help("State command")
                        .required(false)
                    )
                    .arg(Arg::with_name("config")
                        .short('c')
                        .long("config")
                        .value_name("CONFIG")
                        .help("config command")
                    )
                .get_matches();  
 
    if let Some(c) = matched.value_of("config") {
        let configtxt:String = c.to_string();
        let mut file = std::fs::File::open(configtxt)?;
        let mut contents: String = String::new();
        file.read_to_string(&mut contents).unwrap();
        let json: serde_json::Value = serde_json::from_str(&contents).expect("something wrong when change");
        match &json["random"] {
            json!(true) => {
                wordlegame.gamemode=GameMode::RandomMode;
            }
            json!(false) => {
            }
            _ => {
            }
        }
        match &json["difficult"] {
            json!(true) => {
                wordlegame.gamedifficulty=GameDifficulty::Difficult;
            }
            json!(false) => {
            }
            _ => {
            }
        }
        match &json["stats"] {
            json!(true) => {
                wordlegame.stats = true;
            }
            json!(false) => {
                wordlegame.stats = false;
            }
            _ => {
            }
        }
        match &json["seed"] {
            serde_json::Value::Number(s) => {
                wordlegame.seed= s.as_u64().unwrap();           
            }
            _ => {
            }
        }
        match &json["day"] {
            serde_json::Value::Number(s) => {
                wordlegame.gamestates.days= s.as_u64().unwrap();  
            }
            _ => {
            }
        }
        match &json["final_set"] {
            serde_json::Value::String(s) => {
                fjson = true;
                finaltxt = s.clone();
            }
            _ => {
            }
        }
        match &json["acceptable_set"] {
            serde_json::Value::String(s) => {
                ajson= true;
                accepttxt= s.clone();
            }
            _ => {
            }
        }
        match &json["state"] {
            serde_json::Value::String(s) => {
                sjson = true;
                statetxt = s.clone();
            }
            _ => {
            }
        }
        match &json["word"] {
            serde_json::Value::String(s) => {
                wordlegame.gamestates.answer = s.to_uppercase().clone();
            }
            _ => {
            }
        }
         
        
    }
    //parsing the info in config json
    if matched.is_present("State") || sjson == true {

        if let Some(c) = matched.value_of("State") {
            statetxt =c.to_string();
            if statetxt == "Errorfile".to_string() {
            }
            else {
        }

            let mut file = std::fs::File::open(&statetxt)?;
            let mut contents: String = String::new();
            file.read_to_string(&mut contents).unwrap();
            let json: serde_json::Value = serde_json::from_str(&contents)?;

            match &json["total_rounds"] {
                serde_json::Value::Number(s) => {
                    let uin = s.as_u64().unwrap();
                    statejson.total_rounds = uin;      
                    }
                    _ => {
                        //panic!("Error Scmd");
                }        
            }           

            match &json["games"] {
                serde_json::Value::Array(s) => {            
                    for its in s {
                        let mut gameround: GameRound = GameRound{ answer: String::new(), guesses: Vec::new()};
                        match &its["answer"] {
                            serde_json::Value::String(dp) =>{
                                gameround.answer = dp.to_string().clone();
                            }
                            _ => {
                               // panic!("Error Scmd");
                            }
                        }
                        match &its["guesses"] {
                            serde_json::Value::Array(v) => {
                                for itss in v {
                                    match itss {
                                            serde_json::Value::String(vs) => {
                                            gameround.guesses.push(vs.clone());

                                            let mut tcmd: bool =false;
                                            for ing in &mut guesstcmd {
                                                if &ing.str == vs {
                                                    ing.num += 1;
                                                    tcmd = true;
                                                    break;
                                                }
                                            }
                                            if tcmd == false {
                                                guesstcmd.push(Tcmdstate{num: 1, str: vs.to_string().clone()});
                                            }

                                        }
                                        _ =>{
                                        //panic!("Error Scmd");
                                        }
                                    }
                                }
                                sumsum = v.len() as i32;
                            }
                            _ => {
                                //panic!("Error Scmd");
                            }
                        }     
                        if gameround.answer == gameround.guesses[gameround.guesses.len()-1 as usize] {
                            winwin += 1;
                            sumsumsum += sumsum;
                        }             
                        else {
                            loselose += 1;
                        }
                        statejson.games.push(gameround.clone());
                    }
                }
                _ => {
                    //panic!("Error Scmd");
                }
            } 
        }         
    }
    //parsing the info in gamestate json
    if let Some(c) = matched.value_of("word") {
        wordlegame.gamestates.answer=c.to_string().to_uppercase().clone();
    }

    if let Some(c) = matched.value_of("seeds") {
        let seed: String = c.to_string();
        let seednum: u64 = seed.parse::<u64>().unwrap();
        wordlegame.seed = seednum;
    }
    
    if let Some(c) = matched.value_of("days") {
        let days: String = c.to_string();
        let daysnum: u64 = days.parse::<u64>().unwrap();
        wordlegame.gamestates.days = daysnum;
    }

    if matched.is_present("random") {
        wordlegame.gamemode = GameMode::RandomMode;
    }

    if matched.is_present("word") {
        wordlegame.gamemode = GameMode::AnswerConsoleMode;
    }

    if matched.is_present("difficulty") {
        wordlegame.gamedifficulty = GameDifficulty::Difficult;
    }

    if matched.is_present("stats") {
        wordlegame.stats = true;
    }

    if matched.is_present("random") {
        wordlegame.gamemode = GameMode::RandomMode;
    }

    if let Some(c) = matched.value_of("finalset") {
        finaltxt = c.to_string();
    }
       
    if matched.is_present("finalset") || fjson == true {
        let file = std::fs::File::open(&finaltxt)?;
        let buffered = BufReader::new(file);
        for line in buffered.lines() {
            finaltext.push(line.unwrap_or_default().to_uppercase());
            for i in finaltext[finaltext.len()-1].chars() {
                if !(i >= 'A' && i <='Z') {
                    panic!("invalid print");
                }
            }
            if finaltext[finaltext.len()-1].len() != 5 {
                panic!("Wrong");
            }
        }
    }
    else {
        for i in builtin_words::FINAL {
            finaltext.push(i.to_string().to_uppercase());
        }      
        endcode = true;
    }

    finaltext.sort_by(|a,b| a.cmp(&b));

    
    for i in 1..finaltext.len() {
        if &finaltext[i] == &finaltext[i-1] {
            panic!("Error Mult");
        }
    }
    
    
    if let Some(c) = matched.value_of("acceptset") {
        accepttxt = c.to_string();
    }

    if matched.is_present("acceptset") || ajson == true {
        let file = std::fs::File::open(&accepttxt)?;
        let buffered = BufReader::new(file);
        for line in buffered.lines() {
            accepttext.push(line.unwrap_or_default().to_uppercase());
            if finaltext[finaltext.len()-1].len() != 5 {
                panic!("Wrong");
            }
        }

    }
    else {
        for i in builtin_words::ACCEPTABLE {
            accepttext.push(i.to_string().to_uppercase());
        }      
        endcode = true;
    }

    accepttext.sort_by(|a,b| a.cmp(&b));
    
    
    for i in 1..accepttext.len() {
        if &accepttext[i] == &accepttext[i-1] {
            panic!("Error Mult");
        }
    }
    
    if !endcode {
        for i in 0..finaltext.len() {
        if !accepttext[i..accepttext.len()].contains(&finaltext[i]) {
            panic!("Rt");
        }
        } 
    }
    
    
    if _is_tty {
      wordlegame.wordle_istty=true;
    }
    //parsing the command in terminal and do the init work
    


    if (matched.is_present("seeds") || matched.is_present("days")) && 
    (matched.is_present("word") || wordlegame.gamemode == GameMode::AnswerMode) {
        panic!("Error sw");
    }
     
    if matched.is_present("random") && matched.is_present("word") {
        panic!("Error sw");
    }


    match wordlegame.gamemode {
        GameMode::AnswerMode | GameMode::AnswerConsoleMode => { 
            let mut wingame: i32 = 0;
            let mut losegame: i32 = 0;                         
            let mut sum: i32 = 0;
            let mut float: f32;
            let mut modevec: Vec<Vec<StateofChar>> = Vec::new();
            
            loop {

                //gameround += 1;
                match matched.is_present("word") {
                    true => {
                        let mut wordright: bool = false;                                                         
                            for it in &finaltext {
                                if *it == wordlegame.gamestates.answer {
                                    wordright = true;                                           
                                }
                            }                                                            

                        if wordright == false {
                            panic!("INVALID ANSWER");
                        }
                    }
                    
                    false => {
                        let mut wordright: bool = false;
                        while wordright == false {

                            let answeringame0: String = read_line::read_line();// the answer word 
                            let anslice: &str = &answeringame0[0..answeringame0.len()-1]; 
                            let answeringame: String = anslice.to_string().to_uppercase().clone();
                            
                                for it in &finaltext {
                                    if *it == answeringame {
                                        wordright =true;
                                        wordlegame.gamestates.answer = answeringame.clone();
                                        break; 
                                    }
                                }                                                                                       

                            if wordright == false {
                                println!("INVALID");
                            }
                        } 
                    }
                }  //confirm answer is valid or not     

                let mut endgameal: bool = true;   
                for it in &mut wordlegame.gamestates.stateofal {
                    *it = StateofChar::X;
                }                          

                for u in 1..7 {

                    let mut invec:Vec<char> = Vec::new();
                    let mut anvec:Vec<char> = Vec::new();
                    //some init
                                                        
                    let mut wordright2: bool = false;
                    let mut diff: bool = true;
                    while wordright2 == false || diff == false {

                        let inword0: String = read_line::read_line();
                        let inslice: &str = &inword0[0..inword0.len()-1]; 
                        let inword: String = inslice.to_string().to_uppercase().clone();

                        match wordlegame.gamedifficulty {
                            GameDifficulty::Difficult => {
                                if u > 1 {
                                    diff = difficultmode(&inword, &mut wordlegame);
                                }   
                            }
                            GameDifficulty::Normal => {
                            }
                        }
                        
                        if diff == true {                           
                            for it in &accepttext {
                                if *it == inword {
                                    wordright2 = true;
                                    for it in inword.chars() {
                                        invec.push(it);
                                    }
                                    let mut csum: usize = 0;
                                    for its in inword.chars() {                                               
                                        wordlegame.gamestates.guessword[csum] = its;
                                        csum += 1;
                                    }
                                    let mut tcmd: bool =false;
                                    for ing in &mut guesstcmd {
                                        if &ing.str == it {
                                            ing.num += 1;
                                            tcmd = true;
                                            break;
                                        }
                                    }
                                    if tcmd == false {
                                        guesstcmd.push(Tcmdstate{num: 1, str: it.to_string().clone()});
                                    }
                                    break; 
                                }
                            }                          
                        }

                        if wordright2 == false || diff == false {
                        println!("INVALID");
                        }

                    } //confirm guess word is valid or not
                                                        
          
                    for it in wordlegame.gamestates.answer.chars() {
                        anvec.push(it);
                    }
                    for it in &mut wordlegame.gamestates.stateofword {
                        *it = StateofChar::X;
                    }
                    

                    refresh(&invec, &anvec, &mut wordlegame);
                    //main operating function

                    match wordlegame.wordle_istty {
                        true =>{ 
                            modevec.push(Vec::new());
                            for oi in 0..5 {
                                modevec[u-1].push(wordlegame.gamestates.stateofword[oi]);                                   
                            }
                            
                            for oi in 0..26 {
                                modevec[u-1].push(wordlegame.gamestates.stateofal[oi]);                                   
                            }
                            for its in &modevec {
                                let mut ntime: i32 = 0;
                                for  itss in its {  
                                    ntime += 1;      
                                    if ntime != 6 {                      
                                        match itss {
                                            StateofChar::G => {
                                                print!("{}",itss.to_string().green());
                                            }
                                            StateofChar::Y => {
                                                print!("{}",itss.to_string().yellow());
                                            }
                                            StateofChar::R => {
                                                print!("{}",itss.to_string().red());
                                            }
                                            StateofChar::X => {
                                                print!("{}",itss.to_string().red());
                                            }
                                        }
                                    }
                                    else {
                                    print!(" ");
                                    }                                    
                                }
                                print!("\n");
                            }                                   
                            println!("");
                        }
                        false => {
                        for it in &wordlegame.gamestates.stateofword {
                            print!("{}",it.to_char());
                        }
                        print!(" ");
                        for it in &wordlegame.gamestates.stateofal {
                            print!("{}",it.to_char());
                        }
                        println!("");
                        }
                    }
                    //realize the function of output
                
                    let mut endgame: bool = true;
                    for it in &wordlegame.gamestates.stateofword {
                        match *it {
                            StateofChar::G => {
                            }
                            _ => { 
                            endgame=false;
                            }
                        }
                    }
                    endgameal = endgame;
                    if endgame == true {
                        println!("CORRECT {}",u);
                        wingame += 1;
                        sum += u as i32;
                        break;
                    }
                }

                if endgameal == false {
                    println!("FAILED {}",wordlegame.gamestates.answer);
                    losegame += 1;
                }

                if wordlegame.stats == true {
                    let sumf: f32 = sum as f32;
                    let wingamef: f32 = wingame as f32;
                    float = sumf / wingamef;
                 

                    guesstcmd.sort_by(|a,b| a.str.cmp(&b.str));
                    guesstcmd.sort_by(|a,b| b.num.cmp(&a.num));
                    let mut cou: i32 = 0;

                    if _is_tty {
                        if wingame != 0 {
                            println!("{} {} {:.2}", wingame.to_string().green(), losegame.to_string().red(), float.to_string().yellow());
                            }
                            else {
                            let d:f32=0.00;
                            println!("{} {} {:.2}", wingame.to_string().green(), losegame.to_string().red() ,d.to_string().yellow());
                            }
                        for id in &guesstcmd {
                            print!("{} {}",id.str.red(),id.num.to_string().green());
                            cou += 1; 
                            if cou>=5 || cou >= guesstcmd.len() as i32{
                                break;
                            }
                            print!(" ");
                        }
                        println!("");
                    }
                    else {
                        if wingame != 0 {
                            println!("{} {} {:.2}", wingame, losegame, float);
                            }
                            else {                           
                            println!("{} {} 0.00", wingame, losegame);
                            }
                        for id in &guesstcmd {
                            print!("{} {}",id.str,id.num);
                            cou += 1; 
                            if cou>=5 || cou >= guesstcmd.len() as i32{
                                break;
                            }
                            print!(" ");
                        }
                        print!("\n");
                    }
                }//realize the function of -t command


                if matched.is_present("word") {
                    break;
                }
                else {
                    let gameon: char = read!();
                    if gameon == 'N' {
                        break;
                    }              
                }

            }//the game end point
        }      

        GameMode::RandomMode => {

            let mut wingame: i32 = winwin;
            let mut losegame: i32 = loselose;                         
            let mut sum: i32 = sumsumsum;
            let mut float: f32;
            let mut modevec: Vec<Vec<StateofChar>> = Vec::new();
            let mut gameround: u64 = 0;

            let mut stdrng = StdRng::seed_from_u64(wordlegame.seed);
            finaltext.shuffle(&mut stdrng);
            
            loop {

                gameround += 1;
                statejson.total_rounds += 1;
                let gg: GameRound = GameRound { answer: String::new(), guesses: Vec::new() };
                statejson.games.push(gg.clone());

                wordlegame.gamestates.answer = finaltext[(wordlegame.gamestates.days) as usize -1].to_string();


                randans.push(wordlegame.gamestates.answer.clone());
                let len = &statejson.games.len()-1;
                statejson.games[len].answer = wordlegame.gamestates.answer.clone();
                                                           
                                                    
                let mut endgameal: bool = true;       
                randguess.push(Vec::new());                      

                for it in &mut wordlegame.gamestates.stateofal {
                    *it = StateofChar::X;
                }  
                for u in 1..7 {

                    let mut invec:Vec<char> = Vec::new();
                    let mut anvec:Vec<char> = Vec::new();
                                                        
                    let mut wordright2: bool = false;
                    let mut diff: bool = true;
                    while wordright2 == false ||  diff == false {

                        let inword0: String = read_line::read_line();
                        let inslice: &str = &inword0[0..inword0.len()-1]; 
                        let inword: String = inslice.to_string().to_uppercase().clone();// the guess word
                                                   
                        
                        match wordlegame.gamedifficulty {
                            GameDifficulty::Difficult => {
                                if u > 1 {
                                    diff = difficultmode(&inword, &mut wordlegame);
                                }   
                            }
                            GameDifficulty::Normal => {
                            }
                        }

                        if diff == true {
                            for it in &accepttext {
                                if *it == inword {
                                    wordright2 = true;
                                    for it in inword.chars() {
                                        invec.push(it);
                                    }
                                    let mut csum: usize = 0;
                                    for its in inword.chars() {                                               
                                        wordlegame.gamestates.guessword[csum] = its;
                                        csum += 1;
                                    }

                                    randguess[(gameround-1) as usize].push(inword.clone());
                                    let len = &statejson.games.len()-1;
                                    statejson.games[len].guesses.push(inword.clone());

                                    let mut tcmd: bool = false;
                                    for ing in &mut guesstcmd {
                                        if &ing.str == it {
                                            ing.num += 1;
                                            tcmd = true;
                                            break;
                                        }
                                    }
                                    if tcmd == false {
                                        guesstcmd.push(Tcmdstate{num: 1, str: it.to_string().clone()});
                                    }
                                    break; 
                                }
                            } 
                        } 
                            
                        if wordright2 == false || diff == false {
                            println!("INVALID");
                        }
                    }//check the guess answer is valid or not
                                                    

                    
                    for it in wordlegame.gamestates.answer.chars() {
                        anvec.push(it);
                    }//the init of in/an vec
                    for it in &mut wordlegame.gamestates.stateofword {
                        *it = StateofChar::X;
                    }
                    //the init mod

                    refresh(&invec, &anvec, &mut wordlegame);
                    //main operating function

                    match wordlegame.wordle_istty {
                        true =>{ 
                            modevec.push(Vec::new());
                            for oi in 0..5 {
                                modevec[u-1].push(wordlegame.gamestates.stateofword[oi]);                                   
                            }
                            
                            for oi in 0..26 {
                                modevec[u-1].push(wordlegame.gamestates.stateofal[oi]);                                   
                            }
                            for its in &modevec {
                                let mut ntime: i32 =0;
                                for  itss in its {  
                                    ntime += 1;      
                                    if ntime != 6 {                      
                                        match itss {
                                            StateofChar::G => {
                                                print!("{}",itss.to_string().green());
                                            }
                                            StateofChar::Y => {
                                                print!("{}",itss.to_string().yellow());
                                            }
                                            StateofChar::R => {
                                                print!("{}",itss.to_string().red());
                                            }
                                            StateofChar::X => {
                                                print!("{}",itss.to_string().red());
                                            }
                                        }
                                    }
                                    else {
                                    print!(" ");
                                    }                                    
                                }
                                print!("\n");
                            }                                   
                            println!("");
                        }
                        false => {
                        for it in &wordlegame.gamestates.stateofword {
                            print!("{}",it.to_char());
                        }
                        print!(" ");
                        for it in &wordlegame.gamestates.stateofal {
                            print!("{}",it.to_char());
                        }
                        println!("");
                        }
                    }
                    //realize the function of output




                    if matched.is_present("State") && statetxt != "Errorfile".to_string() {   
                        let mut file = std::fs::File::create(&statetxt)?;
                        serde_json::to_writer(&mut file, &statejson)?;
                    }
                    //realize the function of -S command of writing info in json
                
                    let mut endgame: bool = true;
                    for it in &wordlegame.gamestates.stateofword {
                        match *it {
                            StateofChar::G => {
                            }
                            _ => { 
                            endgame=false;
                            }
                        }
                    }
                    endgameal = endgame;
                    if endgame == true {
                        println!("CORRECT {}",u);
                        wingame += 1;
                        sum += u as i32;
                        break;
                    }
                    //here is the endgame point
                }

                if endgameal == false {
                    println!("FAILED {}",wordlegame.gamestates.answer);
                    losegame += 1;
                }

                if wordlegame.stats == true {
                    let sumf: f32 = sum as f32;
                    let wingamef: f32 = wingame as f32;
                    float = sumf / wingamef;

                    guesstcmd.sort_by(|a,b| a.str.cmp(&b.str));
                    guesstcmd.sort_by(|a,b| b.num.cmp(&a.num));
                    let mut cou: i32 = 0;

                    if _is_tty {
                        if wingame != 0 {
                            println!("{} {} {:.2}", wingame.to_string().green(), losegame.to_string().red(), float.to_string().yellow());
                            }
                            else {
                                let d:f32 = 0.00;
                                println!("{} {} {:.2}", wingame, losegame,d.to_string().yellow());
                            }
                        for id in &guesstcmd {
                            print!("{} {}",id.str.red(),id.num.to_string().green());
                            cou += 1; 
                            if cou>=5 || cou >= guesstcmd.len() as i32{
                                break;
                            }
                            print!(" ");
                        }
                        println!("");
                    }
                    else {
                        if wingame != 0 {
                            println!("{} {} {:.2}", wingame, losegame, float);
                            }
                            else {
                                println!("{} {} 0.00", wingame, losegame);
                            }
                        for id in &guesstcmd {
                            print!("{} {}",id.str,id.num);
                            cou += 1; 
                            if cou>=5 || cou >= guesstcmd.len() as i32{
                                break;
                            }
                            print!(" ");
                        }
                        print!("\n");
                    }
                }//realize the function of -t command 
                
                    let gameon: char = read!();
                    match &gameon {
                        'N' => {
                            break;
                        }
                        'Y' => {
                        }
                        _ => {
                            panic!("ERROR ENDCHAR");
                        }
                    }      

                wordlegame.gamestates.days += 1;
            }//the game end point 
        }       
    }

    Ok(())
}

fn countnum (cvec: &Vec<char>, char: char, wordleg: &WordleGame) -> i32 {
    let mut count: i32=0;
    for n1 in 0..5 {
        let n2 = n1 as usize;
       
        match wordleg.gamestates.stateofword[n2] {
            StateofChar::G => {  }
            _ => {
                if cvec[n2] == char {
                    count += 1;
                }
            }
        }
    }
    count
}
//count the number of a special char in one word with the para: cvec as word, char as the char, wordlegame as the basic info;
//return the i32 number which means the times of the char in this word
fn refreshword (invec: &Vec<char>, anvec: &Vec<char>, char: char, wordleg: &mut WordleGame) {
    for n1 in 0..5 {
        let n2 = n1 as usize;
        if anvec[n2] == invec[n2] {
            wordleg.gamestates.stateofword[n2] = StateofChar::G;
        }
    }//select the G word
    let innum = countnum(invec, char, wordleg);
    let annum = countnum(anvec, char, wordleg);
    if innum <= annum {
        for i in 0..5 {
            let ui = i as usize;
            match wordleg.gamestates.stateofword[ui] {
                StateofChar::G => { }
                StateofChar::X => {
                   if invec[ui] == char {
                    wordleg.gamestates.stateofword[ui] = StateofChar::Y;
                   } 
                }    
                _ => {  }           
            }
        }
    }
    else if  innum > annum { 
        let mut liscount : i32 =0;
        for i in 0..5 {
            let ui = i as usize;
            match wordleg.gamestates.stateofword[ui] {
                StateofChar::G => { }
                StateofChar::X => {
                    if invec[ui] == char {
                    liscount += 1;
                    if liscount <= annum {
                        wordleg.gamestates.stateofword[ui] = StateofChar::Y;
                        }
                    else { 
                        wordleg.gamestates.stateofword[ui] = StateofChar::R;
                        }
                    } 
                }    
                _ => {                     
                }                  
            }
        }
    }
}
//fresh the state of guessed word in one char with the para: invec as guess word, anvec as answer,char as the char,wordleg as the basic info
fn refresh (invec: &Vec<char>, anvec: &Vec<char>, wordleg: &mut WordleGame) {
    let wordvec: Vec<char> = vec!['A','B','C','D','E','F','G','H','I','J',
    'K','L','M','N','O','P','Q','R','S','T','U','V','W','X','Y','Z'];
    for i in 0..26 {
        let ui = i as usize;
        refreshword(invec, anvec, wordvec[ui], wordleg);
    }
    refreshal(invec, wordleg);
}
//fresh the state of guessed word with the para:invec as guess word, anvec as answer,wordleg as the basic info
fn refreshal(invec: &Vec<char>, wordleg: &mut WordleGame) {
    let wordvec: Vec<char> = vec!['A','B','C','D','E','F','G','H','I','J',
    'K','L','M','N','O','P','Q','R','S','T','U','V','W','X','Y','Z'];
    for it in 0..5 {
        let uit = it as usize;
        for i in 0..26 {
            let ui = i as usize; 
            if  invec[uit] == wordvec[ui] {
                changeal(ui, uit, wordleg);
                break;
            }
        }
    }
}
//fresh the state of all the 26 chars with the para:invec as guess word,wordleg as the basic info
fn changeal(n: usize, un: usize, wordleg: &mut WordleGame) {
    match wordleg.gamestates.stateofal[n] {
        StateofChar::X => {
            wordleg.gamestates.stateofal[n] = wordleg.gamestates.stateofword[un];
        }
        StateofChar::R => {
            wordleg.gamestates.stateofal[n] = wordleg.gamestates.stateofword[un];
        }
        StateofChar::Y => {
            let mid: &StateofChar = &wordleg.gamestates.stateofword[un];
            match mid {
                StateofChar::R => {
                }
                _ => {          
                    wordleg.gamestates.stateofal[n] = wordleg.gamestates.stateofword[un];
                }
            }
        }
        StateofChar::G => {
        }
    }
}
// realize the function of freshing the all 26 chars with the para:n and un as the iter para, wordleg as the basic info
fn difficultnumber_char (inword: &String, wordleg: &mut WordleGame, char: char) -> bool{
    let mut invec: Vec<char> = Vec::new();
    let mut gstates: Vec<bool> = Vec::new();
    let mut dnum: usize = 0;
    let mut snum: usize = 0;
    for it in inword.chars() {
        invec.push(it);
    }
    for _ in 0..5 {
        gstates.push(false);
    }
    for i in 0..5 {
        match wordleg.gamestates.stateofword[i] {
            StateofChar::G => {
                if char == invec[i] {
                    if invec[i] == wordleg.gamestates.guessword[i] {
                        gstates[i] = true;
                    }
                    else {
                        return false;
                    }
                }
            }
            StateofChar::Y => {
                if char == wordleg.gamestates.guessword[i] {
                    snum += 1;
                }
            }
            _ => {

            }
        }
    }

    for i in 0..5 {
        if invec[i] == char {
            match gstates[i] {
                true => {
                }
                false => {
                    dnum += 1;
                }
            }
        }
    }        
        

        if dnum >= snum {
            return true;
        }
        else {
            return false;
        }
    

}
//confirm if one char in the word is approved in difficult mode with the para:inword as the word, wordleg as the basic info, char as the char; 
//return the bool value which means the word in this char is valid or not

fn difficultmode (inword: &String, wordleg: &mut WordleGame)  -> bool {
    let wordvec: Vec<char> = vec!['A','B','C','D','E','F','G','H','I','J',
    'K','L','M','N','O','P','Q','R','S','T','U','V','W','X','Y','Z'];
   
    for i in 0..26 {
        let ui = i as usize;
        if !difficultnumber_char(inword, wordleg, wordvec[ui]) {
            return false;
        }
    }
    true
}
//confirm if the word is approved in difficult mode with the para:inword as guess word, wordleg as the basic info;
//return the bool value which means the word is valid or not



