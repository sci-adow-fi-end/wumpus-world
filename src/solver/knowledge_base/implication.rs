
    use std::collections::hash_set::Iter;
    use std::collections::HashSet;
    use std::hash::{Hash, Hasher};

    #[derive(Debug, Eq, PartialEq)]
    pub struct Implication {
        premise:HashSet<String>,
        conclusion:String
    }
    impl Hash for Implication {
        fn hash<H: Hasher>(&self, state: &mut H) {
            let premise_vec: Vec<_> = self.premise.iter().collect();
            premise_vec.hash(state);
            self.conclusion.hash(state);
        }
    }

    impl Implication {

        pub fn new()->Self{
            Implication {
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


