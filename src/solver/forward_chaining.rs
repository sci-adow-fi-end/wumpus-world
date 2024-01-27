
    use std::collections::{HashMap, VecDeque};
    use crate::solver::knowledge_base::definite_clause::DefiniteClause;
    use crate::solver::knowledge_base::KnowledgeBase;

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

