    pub(crate) mod implication;

    use std::collections::HashSet;
    use std::fs::File;
    use std::io::Read;
    use implication::Implication;


    #[derive(Debug, Eq, PartialEq)]
    pub struct KnowledgeBase{
        symbols:HashSet<String>,
        implications:HashSet<Implication>
    }
    impl KnowledgeBase{

        pub fn new()->Self{
            let symbols:HashSet<String>=HashSet::new();
            let implications:HashSet<Implication>=HashSet::new();
            return KnowledgeBase{symbols,implications};
        }
        pub fn iterate_symbols(&self) -> std::collections::hash_set::Iter<'_, String> {
            return self.symbols.iter();
        }
        pub fn iterate_implications(&self) -> std::collections::hash_set::Iter<'_, Implication> {
            return self.implications.iter();
        }

        fn add_symbol(&mut self, new_symbol:String){
            self.symbols.insert(new_symbol);
        }

        fn add_implication(&mut self, new_implication: Implication){
            self.implications.insert(new_implication);
        }

        pub fn symbols_contain(&self, symbol: &String)->bool{
            return self.symbols.contains(symbol);
        }

        pub fn load(&mut self, path_to_knowledge:&String){


            let mut file = File::open(path_to_knowledge).expect("knowledge base file not found");

            let mut content =String::new();
            file.read_to_string(&mut content).expect("error in file parsing");

                    for clause in content.lines(){


                        if clause.contains("->"){
                            let mut definite_clause = Implication::new();
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


