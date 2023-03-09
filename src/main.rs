use rand;
use rand::Rng;
use rand::seq::SliceRandom;
use std::collections::HashMap;
use std::io;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time;

use crate::StateStudent::Silence;
use crate::StateStudent::Talking;

#[derive(Copy, Clone)]
enum StateStudent {
    Talking,
    Silence
}

#[derive(Clone)]
struct Student {
    id: i64,
    state: StateStudent,
    patience: u64,
}

impl Student {
    fn new(id: i64, patience: u64) -> Self {
        Student {
            id: id,
            state: Talking,
            patience: patience
        }
    }
}

fn display_title() {
    println!("\n\n\n ■■■■■■■ It Took X Minutes For Everyone To Quiet Down ■■■■■■\n\n\n");
}

fn choose_difficulty() -> (u64, u64, usize) {
    loop {
        println!(" Choose the difficulty(easy / medium / hard / extream): ");
        let mut input = String::new();
    
        io::stdin().read_line(&mut input);
        let trimmed = input.trim();

        match trimmed {
            "easy" => {
                return (6, 30, 10)
            },
            "medium" => {
                return (6, 30, 100)
            },
            "hard" => {
                return (6, 20, 10)
            },
            "extream" => {
                return (6, 20, 100)
            },
            "oni" => {
                return (6, 10, 100)
            }
            _ => println!("正しい難易度を選んでください")
        }
    }
}

fn main() {
    display_title();
    loop {
        let (num_st, max_patience_sec, mut min_size_id) = choose_difficulty();
        let num_silence_student = Arc::new(Mutex::new(0));
        let mut rng = rand::thread_rng();

        let mut students: Vec<Arc<Mutex<Student>>> = Vec::new();
        let mut memo: HashMap<usize, usize> = HashMap::new();

        let mut id_lists: Vec<usize> = (min_size_id..min_size_id*10).collect::<Vec<usize>>();
        id_lists.shuffle(&mut rng);

        
        for (i, v) in id_lists.iter().enumerate() {
            if i >= num_st.try_into().unwrap() {break;}
            students.push(Arc::new(Mutex::new(Student::new(*v as i64, rng.gen_range(2..max_patience_sec)))));
            memo.insert(*v, i);
            println!("students group {v} is talking...");
        }

        let now = time::SystemTime::now();
        loop {
            let mut input = String::new();

            io::stdin().read_line(&mut input);

            let trimmed = input.trim();
            match trimmed.parse::<usize>() {
                Ok(i) => {
                    if min_size_id > i.try_into().unwrap() || min_size_id*10 < i.try_into().unwrap() {
                        println!("有効な範囲の数字を入力してください、{min_size_id}~{}", min_size_id*10);
                    }
                    else if !memo.contains_key(&i) {
                        println!("入力した生徒グループは存在しません");
                    }
                    else {
                        let idx = memo[&i];
                        let state = students[idx].clone().lock().unwrap().state;
                        match state {
                            Talking => {
                                *num_silence_student.lock().unwrap() += 1;
                                students[idx].lock().unwrap().state = Silence;  
                                println!("生徒{i}らを注意した");

                                let mut num1 = num_silence_student.clone();
                                let mut student = students[idx].clone();

                                thread::spawn(move || {
                                    let patience = student.clone().lock().unwrap().patience;
                                    let secs = time::Duration::from_secs(patience);
                                    thread::sleep(secs);
                                    println!("生徒{i}らは{patience}秒しか我慢できなかった");
                                    let mut num1_locked = num1.lock().unwrap();
                                    let mut st1 = student.lock().unwrap();
                                    *num1_locked -= 1;
                                    st1.state = Talking;
                                });
                            }
                            Silence => {
                                println!("生徒{i}ら は静かにしているようです");
                            }
                        }
                    }
                }
                Err(i) => {
                    println!("有効な数字を入力してください、{min_size_id}~{}", min_size_id*10);
                }
            }
        
            
            if *num_silence_student.lock().unwrap() >= num_st {
                println!("はい、みんなが静かになるまでに、'{} 秒' かかりました ", now.elapsed().expect("ごめんなさい、静かになるまでの時間を数えるのを忘れていました").as_secs());
                break
            }
        }

        // println!("retry? (y/n)");
        // let mut input = String::new();
        // io::stdin().read_line(&mut input);
        // let trimmed = input.trim();
        // if trimmed != "y" {
        //     break;
        // }
    }
}
