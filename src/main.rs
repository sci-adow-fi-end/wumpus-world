
mod definite_clauses{
    use std::collections::hash_set::Iter;
    use std::collections::HashSet;
    use std::hash::{Hash, Hasher};

    #[derive(Debug, Eq, PartialEq)]
    pub struct DefiniteClause{
        premise:HashSet<String>,
        conclusion:String
    }
    impl Hash for DefiniteClause {
        fn hash<H: Hasher>(&self, state: &mut H) {
            let premise_vec: Vec<_> = self.premise.iter().collect();
            premise_vec.hash(state);
            self.conclusion.hash(state);
        }
    }

    impl DefiniteClause{

        pub fn new()->Self{
            DefiniteClause{
                premise:HashSet::new(),
                conclusion:String::new()
            }
        }

        pub fn get_premises_number(&self)->u16{
            return self.premise.len() as u16;
        }

        pub fn iterate_premises(&self) -> Iter<'_, String> {
            return self.premise.iter();
        }

        pub fn add_premise(&mut self, new_premise:String){
            self.premise.insert(new_premise);
        }
        pub fn set_conclusion(&mut self, new_conclusion:String){
            self.conclusion = new_conclusion;
        }
        pub fn get_conclusion(&self)->String{
            return self.conclusion.clone();
        }
        pub fn premise_contains(&self, symbol:&String)->bool{
            return self.premise.contains(symbol);
        }
        pub fn is_conclusion(&self, symbol:&String)->bool{
            return self.conclusion==*symbol;
        }
    }

}


mod knowledge_base{
    use std::collections::HashSet;
    use ini::Ini;
    use crate::definite_clauses::DefiniteClause;


    #[derive(Debug, Eq, PartialEq)]
    pub struct KnowledgeBase{
        symbols:HashSet<String>,
        implications:HashSet<DefiniteClause>
    }
    impl KnowledgeBase{

        pub fn new()->Self{
            let symbols:HashSet<String>=HashSet::new();
            let implications:HashSet<DefiniteClause>=HashSet::new();
            return KnowledgeBase{symbols,implications};
        }
        pub fn iterate_symbols(&self) -> std::collections::hash_set::Iter<'_, String> {
            return self.symbols.iter();
        }
        pub fn iterate_implications(&self) -> std::collections::hash_set::Iter<'_, DefiniteClause> {
            return self.implications.iter();
        }

        fn add_symbol(&mut self, new_symbol:String){
            self.symbols.insert(new_symbol);
        }

        fn add_implication(&mut self, new_implication:DefiniteClause){
            self.implications.insert(new_implication);
        }

        pub fn symbols_contain(&self, symbol: &String)->bool{
            return self.symbols.contains(symbol);
        }

        pub fn load(&mut self, path_to_knowledge:&String){

            let config = Ini::load_from_file(path_to_knowledge).expect("Failed to load INI file");

            if let Some(section) = config.section(Some("inputs")) {
                if let Some(contents) = section.get("KB") {

                    for clause in contents.split(" ; "){


                        if clause.contains("->"){
                            let mut definite_clause = DefiniteClause::new();
                            let mut split_line = clause.split("->");

                            if let Some(premise) = split_line.next() {
                                for element in premise.split(","){
                                    definite_clause.add_premise(element.to_string())
                                }
                            } else {
                                panic!("error in the clauses' syntax");
                            }

                            if let Some(conclusion) = split_line.next() {
                                definite_clause.set_conclusion(conclusion.to_string());
                            } else {
                                panic!("error in the symbols' syntax");
                            }
                            self.add_implication(definite_clause);

                        }
                        else {
                            self.add_symbol(clause.to_string());
                        }

                    }
                }
                }
            else{
                panic!("error in the knowledge base's file structure");
            }
            }




    }

}


mod forward_chaining{

    use std::collections::{HashMap, VecDeque};
    use crate::definite_clauses::DefiniteClause;
    use crate::knowledge_base::KnowledgeBase;

    struct Toolbox<'a> {
        count: HashMap<&'a DefiniteClause, u16>,
        inferred: HashMap<&'a String, bool>,
        queue: VecDeque<String>
    }
    impl<'a> Toolbox<'a>{

        fn generate(kb: &'a KnowledgeBase)->Self{

            let mut count:HashMap<&'a DefiniteClause, u16> = HashMap::new();
            for implication in kb.iterate_implications(){
                count.insert(implication, implication.get_premises_number());
            }

            let mut inferred:HashMap<&'a String, bool> = HashMap::new();
            for symbol in kb.iterate_symbols(){
                inferred.insert(symbol, false);
            }

            let mut queue:VecDeque<String> = VecDeque::new();
            for symbol in kb.iterate_symbols(){
                queue.push_back(symbol.clone());
            }

            return Toolbox{count,inferred,queue};
        }


        pub fn is_queue_empty(&self)->bool {
            return self.queue.is_empty();
        }

        pub fn pop_symbol(&mut self)->Option<String> {
            return self.queue.pop_front();
        }

        pub fn print_queue(&self){
            println!("{:?}",self.queue);
        }

        pub fn push_symbol(&mut self, symbol: String){
            self.queue.push_back(symbol);
        }

        pub fn is_already_inferred(&self, symbol:&String)->bool{
            return match self.inferred.get(symbol) {
                Some(matching)=>*matching,
                None=>false
            }
        }

        pub fn set_inferred(&mut self, symbol:&String){
            match self.inferred.get_mut(symbol) {
                Some(matching)=> *matching=true,
                None=>()
            }
        }

        pub fn is_count_zero(&self, clause:&DefiniteClause)->bool{
            return match self.count.get(clause) {
                Some(matching)=>if *matching==0 {true} else { false },
                None=>false
            }
        }

         pub fn decrease_count(&mut self, clause:&DefiniteClause){
            match self.count.get_mut(clause) {
                Some(matching)=> *matching-=1,
                None=>()
            }
        }

    }

    pub fn forward_chaining(q:&String, kb: &KnowledgeBase, debug: bool) ->bool{


            let mut tb= Toolbox::generate(kb); //the queue initially contains all the knowledge base symbols
            let mut i =0; //index for debug
            while !tb.is_queue_empty(){
                i=i+1;
                if debug{
                    println!("Queue at step {}:",i);
                    tb.print_queue();
                    println!(" ");
                }

                let p = tb.pop_symbol().unwrap();
                if p==*q{
                    return true;
                }
                if !tb.is_already_inferred(&p){
                    tb.set_inferred(&p);
                    for clause in kb.iterate_implications(){
                        if clause.premise_contains(&p){
                            tb.decrease_count(clause);
                            if tb.is_count_zero(clause){

                                tb.push_symbol(clause.get_conclusion())
                            }
                        }
                    }
                }
            }

        return false;
    }



}

mod backward_chaining{
    use std::collections::{HashMap, VecDeque};
    use crate::knowledge_base::KnowledgeBase;

    struct Toolbox<'a> {
        reached: HashMap<&'a String, bool>,
        queue: VecDeque<String>
    }
    impl<'a> Toolbox<'a>{

        fn generate(kb: &'a KnowledgeBase, query:&String)->Self{


            let mut reached:HashMap<&'a String, bool> = HashMap::new();
            for symbol in kb.iterate_symbols(){
                reached.insert(symbol, false);
            }

            let mut queue:VecDeque<String> = VecDeque::new();

                queue.push_back(query.clone());

            return Toolbox{ reached,queue};
        }


        pub fn is_queue_empty(&self)->bool {
            return self.queue.is_empty();
        }

        pub fn print_queue(&self){
            println!("{:?}",self.queue);
        }

        pub fn pop_symbol(&mut self)->Option<String> {
            return self.queue.pop_front();
        }

        pub fn push_symbol(&mut self, symbol: String){
            self.queue.push_back(symbol);
        }

        pub fn is_already_reached(&self, symbol:&String) ->bool{
            return match self.reached.get(symbol) {
                Some(matching)=>*matching,
                None=>false
            }
        }

        pub fn set_reached(&mut self, symbol:&String){
            match self.reached.get_mut(symbol) {
                Some(matching)=> *matching=true,
                None=>()
            }
        }



    }

    pub fn backward_chaining(q:&String, kb: &KnowledgeBase, debug:bool) ->bool{

        let mut dead_end =false;

        let mut tb= Toolbox::generate(kb,q); //the queue initially contains just the query
        let mut i=0;//index for debug
        while !dead_end && !tb.is_queue_empty(){

            i=i+1;
            if debug{
                println!("Queue at step {}:", i);
                tb.print_queue();
                println!(" ");
            }


            let p = tb.pop_symbol().unwrap();
            if !kb.symbols_contain(&p) {
                dead_end = true;
                for clause in kb.iterate_implications() {
                    if clause.is_conclusion(&p) {
                        dead_end = false;
                        for premise in clause.iterate_premises() {

                            if !tb.is_already_reached(premise) {
                                tb.set_reached(premise);
                                tb.push_symbol(premise.clone());
                            }
                        }
                    }
                }
            }
        }
        return if dead_end {false} else {true};
    }
}



mod solver{
    use ini::Ini;
    use crate::knowledge_base::KnowledgeBase;
    use crate::solver::Algorithms::{BackwardChaining, ForwardChaining};
    use crate::forward_chaining::forward_chaining;
    use crate::backward_chaining::backward_chaining;

    #[derive(Debug)]
    enum Algorithms{
        ForwardChaining,
        BackwardChaining
    }


    #[derive(Debug)]
    pub struct Solver{
        kb:KnowledgeBase,
        query:String,
        algorithm_used:Algorithms,
        debug:bool

    }

    impl Solver{
        pub fn load_problem(file: String)->Self{

            let mut kb=KnowledgeBase::new();
            kb.load(&file);

            let config = Ini::load_from_file(&file).expect("Failed to load INI file");

            let mut structural_error= false;

            let mut query=String::new();

            if let Some(section) = config.section(Some("inputs")) {
                if let Some(q) = section.get("query") {
                    query= q.parse().unwrap();
                }
                else { structural_error=true; }
            }
            else { structural_error=true; }

            let mut algorithm_used= ForwardChaining;

            if let Some(section) = config.section(Some("algorithm")) {
                if let Some(a) = section.get("algorithm") {
                    algorithm_used= match a {
                        "forward chaining"=>ForwardChaining,
                        "backward chaining"=>BackwardChaining,
                        _ => panic!("the selected algorithm is not valid!")
                    }
                }
                else { structural_error=true; }
            }
            else { structural_error=true; }

            let mut debug=false;

            if let Some(section) = config.section(Some("algorithm")) {
                if let Some(d) = section.get("debug") {
                    debug= match d{
                        "1"=>true,
                        "0"=>false,
                        _ => panic!("the debug option is not valid")
                    }
                }
                else { structural_error=true; }
            }
            else { structural_error=true; }


            if structural_error{
                panic!("there is an error in the initialization file structure")
            }

            return Solver{kb,query,algorithm_used,debug};


        }
        pub fn solve(&self)->bool{
           return match self.algorithm_used {
               ForwardChaining=>forward_chaining(&self.query,&self.kb,self.debug),
               BackwardChaining=>backward_chaining(&self.query,&self.kb,self.debug)
           }
        }
    }
}

use crate::solver::Solver;

fn main() {

let s = Solver::load_problem("/home/alessio/RustroverProjects/wumpus_world/ini/knowledge_base.ini".to_string());
println!("{}", s.solve());
}
