
    use std::collections::{HashMap, VecDeque};
    use crate::solver::knowledge_base::KnowledgeBase;

    struct Toolbox {
        reached: HashMap<String, bool>,
    }
    impl Toolbox{

        fn generate(kb: & KnowledgeBase, query:&String)->Self{


            let mut reached:HashMap<String, bool> = HashMap::new();
            for symbol in kb.iterate_symbols(){
                reached.insert(symbol.clone(), false);
            }


            let mut queue:VecDeque<String> = VecDeque::new();

            queue.push_back(query.clone());

            return Toolbox{ reached};
        }

        pub fn already_reached(&mut self, symbol:&String) ->bool{
            return match self.reached.get(symbol) {
                Some(matching)=>*matching,
                None=>{self.reached.insert(symbol.clone(),false); false}
            }
        }

        pub fn set_reached(&mut self, symbol:&String){
            match self.reached.get_mut(symbol) {
                Some(matching)=> *matching=true,
                None=>println!("{symbol} not found")
            }
        }
    }


    pub fn backward_chaining(q:&String, kb: &KnowledgeBase, debug:bool) ->bool{

        let mut tb= Toolbox::generate(kb,q); //the queue initially contains just the query
        let mut queue:VecDeque<String>=VecDeque::new();
        queue.push_back(q.clone());
        let i=0;//index for debug

        return or_search(q, kb, queue, &mut tb, i, debug)
    }

    fn and_search(q:&String, kb: &KnowledgeBase, queue:VecDeque<String>, tb:&mut Toolbox, i:i32, debug:bool) ->bool{

        if debug{
            println!("calling and_search on {:?} at depth {}", queue, i);
            println!("    ")
        }
        for symbol in queue{
            let mut new_queue:VecDeque<String> = VecDeque::new();
            new_queue.push_back(symbol);
            if !or_search(q, kb, new_queue, tb, i, debug){
                return false
            }
        }
        return true;
    }

    fn or_search(q:&String, kb: &KnowledgeBase, queue:VecDeque<String>, tb:&mut Toolbox,mut i:i32, debug:bool) ->bool{

        if debug{
            i=i+1;
            println!("calling or_search on {:?} at depth {}", queue, i);
            println!("    ")
        }
        for symbol in queue{
            if kb.symbols_contain(&symbol){
                return true;
            }
            else{
                for implication in kb.iterate_implications(){
                    if implication.is_conclusion(&symbol){
                        let mut new_queue:VecDeque<String> = VecDeque::new();
                        for premise in implication.iterate_premises(){
                            if !tb.already_reached(premise) {
                                tb.set_reached(premise);
                                new_queue.push_back(premise.clone());
                            }
                            else{
                                println!("blocked {}",premise);
                            }
                        }
                        if and_search(q, kb, new_queue, tb, i, debug){
                            return true;
                        }
                    }
                }
            }
        }
        return false;
    }