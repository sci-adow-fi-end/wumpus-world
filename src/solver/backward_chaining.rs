
    use std::collections::{HashMap, VecDeque};
    use crate::solver::knowledge_base::KnowledgeBase;

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


