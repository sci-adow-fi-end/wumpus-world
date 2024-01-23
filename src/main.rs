
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
    use std::fs::File;
    use std::io::Read;
    use ini::Ini;
    use crate::definite_clauses::DefiniteClause;


    #[derive(Debug, Eq, PartialEq)]
    pub struct KnowledgeBase{
        symbols:HashSet<String>,
        implications:HashSet<DefiniteClause>
    }
    impl KnowledgeBase{
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

        pub fn load(&mut self, path_to_knowledge:String){



            let mut file = File::open(&path_to_knowledge).unwrap();
            let config = Ini::load_from_file(&path_to_knowledge).expect("Failed to load INI file");

            let mut contents = String::new();
            file.read_to_string(&mut contents).unwrap();
            for line in contents.lines(){


                if line.contains("->"){
                    let mut definite_clause = DefiniteClause::new();
                    let mut split_line = line.split("->");

                    if let Some(premise) = split_line.next() {
                        for element in premise.split(","){
                            definite_clause.add_premise(element.to_string())
                        }
                    } else {
                        panic!();
                    }

                    if let Some(conclusion) = split_line.next() {
                        definite_clause.set_conclusion(conclusion.to_string());
                    } else {
                        panic!();
                    }
                    self.add_implication(definite_clause);

                }
                else {
                    self.add_symbol(line.to_string());
                }

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

    pub fn forward_chaining(q:&String, kb: &KnowledgeBase) ->bool{

            let mut tb= Toolbox::generate(kb); //the queue initially contains all the knowledge base symbols
            while tb.is_queue_empty(){
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

    pub fn backward_chaining(q:&String, kb: &KnowledgeBase) ->bool{

        let mut dead_end =false;

        let mut tb= Toolbox::generate(kb,q); //the queue initially contains just the query
        while !dead_end && tb.is_queue_empty(){
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


fn main() {
    println!("Hello, world!");
}
