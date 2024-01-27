
    mod forward_chaining;
    mod backward_chaining;

    pub(crate) mod knowledge_base;


    use ini::Ini;
    use knowledge_base::KnowledgeBase;
    use Algorithms::{BackwardChaining, ForwardChaining};
    use forward_chaining::forward_chaining;
    use backward_chaining::backward_chaining;

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
