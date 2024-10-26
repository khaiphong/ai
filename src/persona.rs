/*
 The result of KP 50+ years of field researches PLUS exoteric / esoteric observations of
 qualified candidates as first front-line soldiers while waiting for verifications and/or
 modifications / enhancements of later academia contributions. We start from actual
 experiences of a Vietnamese student since 1970 all Gotama's recorded Jhanas, further 
 investigated leading to Bodhidharma's exchange with Emperor Wu (Lương Vũ Đế) on “What is the 
 highest meaning of noble Truth" and HuiNeng's specialized contributions about (1) the 
 Three-Not (Wu-Nien, No-Mark, Detachment / Vô Niệm, Vô Vết, Vô Trụ), (2) #DhyanaSamadhi / 
 Thiền Định, (3) #SamadhiPrajna / Định-Tuệ" leading to KP (1) Three-Haves ("#Awareness, 
 Dependent, #Prajna / Kiến Tánh, Duyên Khởi, Bát Nhã"), via (2) observable X-dimension in 
 both exoteric and esoteric realms from Figure 11.1 that forms with personally observable 
 Inner Peace of Y-dimension from Gotama's recorded Jhanas a No-Conflict plan at any focused 
 event and (3) scientifically cultivable #Prajna / Wisdom with discovered underlying natural 
 laws of PrajnaTIPs from one's scienrifically cultivable InnerSpace F-dimension for 
 detoxification of tainted senses. Introducing the #Awareness of a persona F-dimension with 
 Right evolution or Wrong potential degeneration of complex Fibonacci sequence - testable 
 with Vietnamese KienTanh / HuiNeng WuNien / Gotama's Inner Peace / Bodhidharma's 
 #EmptyTheContent - together with Lucas Numbers applicable to innate Intellligence of 
 Illuminati / Communist elites, we theoretically and empirically prove that the Right 
 evolution must come through (type) EquanimityAwareness f+1 = 1 enhanced and materialized in 
 Equanimity Communities where Nature and Science toward What Count join hand. It is then 
 enforced with (type) PurityAwareness f+2 = 1 for the possibility of Transcending the 
 Observed and Observer to see Thing-As-It-Is and #Prajna of breakthrough.
*/


/*
  We generalize the persona to big corporation HR and important person to (1) clean up 
  tainted senses and (2) cultivate required traits for its What-Count culture. The function 
  of "clean-up" and required "cultivation" are the core of the persona HR strategies. In this 
  sense, the required traits are its core Values (e.g. #Honesty & #Care) in execution. This
  part must be standardized and open sources for possible value-added solutions
  x_traits, y_pointers, [f x y]-dimension : vectors instead of i32
*/
#[derive(Debug)]
pub struct InnerSpace {
  // observable traits and pointers from attached to balanced to detached in one's InnerSpace
  // HashMap<i32, String> visible traits observable in x_dimension
  pub x_traits: Vec<i32>, // vec![-1, -2, -3, -4, 0, 1, 2, 3, 4]
  // HashMap<i32, String> smart pointers from outcomes of y_dimension
  pub y_pointers: Vec<i32>, // vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9]
  // each persona must have lists of traits, pointers, depths of X, Y, and F for both
  // internal and/or externam acquired ability to handle required tasks
  // lumped other factors observable in complex Fibonacci sequences
  //vec![-1, -2, -3, -4, -5, -6, -7, -8, 0, 1, 2, 3, 4, 5, 6, 7, 8],
  pub f_dimension: Vec<i32>, 
  // KP Signed Posts or Gotama Jhanas or Right Samadhi
  pub y_dimension: Vec<i32>, // = vec![-1, -2, -3, -4, -5, -6, 0, 1, 2, 3, 4, 5, 6],
  // HuiNeng #WuNien or KP #Awareness
  pub x_dimension: Vec<i32>, // = vec![-1, -2, -3, -4, -5, -6, -7, 0, 1, 2, 3, 4, 5, 6, 7],   
}
// Implementations
pub mod y_pointers; // implementations to make the description closed to the described
pub mod ydimension; // states of consciousness driving observable Activities and Relationships

pub mod x_traits;   // observable visible traits of one's #Awareness in engaged living
pub mod xdimension; // states of consciousness varying from attachment to detachment

pub mod fdimension; // other relevant factors making up the states and stock of the persona

/*
  Dynamic interactions between normative and positive AI intelligence to custom modeling the
  objective function #Prajna = F(X, Y) of "complex-valued data" to foster innovations /
  breakthroughs in the Persona's war room and strategies for various types: a Latin humanitas,
  next qualified realm, an organization, foreign affairs, national development, etc.
*/
impl InnerSpace { // different methods for different InnerSpace persona types

/*
Modeling the InnerSpace is the systematic building internal power for different stages of
the persona developments

  fn f_evaluation(&self) -> i32 { // f(_x,_y) for the persona
    return self.f_dimension
  }
  fn qualified_realm(&self) -> String { // f(_x,_y) for Inter-Realm
    return "human".to_string()
  }
  fn maturity_level(&self) -> String { // f(_x,_y) for organization
    return "self sustainable".to_string()
  }
  fn change_management(&self) -> String { // f(_x,_y) for foreign affairs
    return "decisive_battle".to_string()
  }
  fn nation_happiness(&self) -> String { // f(_x,_y) for a national development
    return "in_operation".to_string()
  }
*/
}

// HashMap will be replaced with DbHashMap, a bare bone K-V store with namespace and cgroup
use std::collections::HashMap; 

// given the past [ x-y-f ] dimensions, we re-evaluate pointers and traits for suggestions
pub fn build_inner_space(_x: Vec<i32>, _y: Vec<i32>, _f: Vec<i32>) -> InnerSpace  {
  // qualified persona at balanced traits, make it as observable list
  let x_traits: Vec<i32> = Vec::new(); 
  // qualified persona at Awareness, make it as observable list
  let y_pointers: Vec<i32> = Vec::new();
  // to be evaluated in blockchain of user self-evaluation and AI
  let y_dimension = _y; // move to y_dimension
  // opinions from fact-base tracked records in engaged living
  let x_dimension = _x; // move to x_dimension
   // qualified person
  let f_dimension = _f; // move to f_dimension
  
  // new evaluation  of InnerSpace
  let mut _t = x_traits;
  let mut _p = y_pointers;
  let mut _y = y_dimension;
  let mut _x = x_dimension;
  let mut _f = f_dimension;

  // rebuild the relevant dimensions according to open standarded proven researches
  let mut x = HashMap::new(); // Taxonomy of x_dimension for internal evaluation
  x.insert(0, String::from("X 0: Awareness"));             // =  0
  
  // x(-1) = 1 = x(1) Culture & HonNhien enforced @ x(2)=1 of proper management in Equanimity
  x.insert(-1, String::from("X-1: CulturalInfluenced"));   // =  1
  x.insert(-2, String::from("X-2: RegionalInfluenced"));   // = -1
  x.insert(-3, String::from("X-3: NationalInfluenced"));   // =  2
  x.insert(-4, String::from("X-4: VeiledType"));           // = -3
  x.insert(-5, String::from("X-5: BindingWord"));          // =  5    breakout
  x.insert(-6, String::from("X-6: BindingImage"));         // = -8
  x.insert(-7, String::from("X-7: ClingingThought"));      // = 13    indoctrinated
  
  x.insert(1, String::from("X+1: HonNhien"));              // = 1
  x.insert(2, String::from("X+2: ManagingFreshness"));     // = 1
  x.insert(3, String::from("X+3: KnowingFreshness"));      // = 2
  x.insert(4, String::from("X+4: ProcessDiscovered"));     // = 3
  x.insert(5, String::from("X+5: KnowingThought"));        // = 5     breakout = 5
  x.insert(6, String::from("X+6: CareViaCosmicEnergy"));   // = 8
  x.insert(7, String::from("X+7: HelpViaCosmicEnergy"));   // = 13    consciousness technology

  let mut y = HashMap::new(); // Taxonomy of y_dimension
  y.insert(0, String::from("Y 0: Peace"));         // =  0
  
  // y(-1) = 1 = y(1) Empathy to Tranquility enforced in y(2) = 1 of Equanimity
  y.insert(-1, String::from("Y-1: Empathy"));      // =  1
  y.insert(-2, String::from("Y-2: Kindness"));     // = -1
  y.insert(-3, String::from("Y-3: Conscience"));   // =  2
  y.insert(-4, String::from("Y-4: Conscience1")); // = -3
  y.insert(-5, String::from("Y-5: Conscience2")); // =  5 capable for hegemony
  y.insert(-6, String::from("Y-6: Conscience3")); // = -8
  
  y.insert(1, String::from("Y+1: Tranquility"));   // = 1
  y.insert(2, String::from("Y+2: Equanimity"));    // = 1
  y.insert(3, String::from("Y+3: Purity"));        // = 2
  y.insert(4, String::from("Y+4: Selfless"));      // = 3
  y.insert(5, String::from("Y+5: NonThingness"));  // = 5  Gotama's impass - SelfSelfless Act.
  y.insert(6, String::from("Y+6: Unmoving"));      // = 8  Gotama's impass - #SamadhiPrajna
  
  // SmartPointers y_pointers starting from ability to #EmptyTheContent for detoxification 
  let mut p = HashMap::new(); // pointers (p) toward #SamadhiPrajna
  p.insert(0, String::from("P0: Qualified")); 
  p.insert(1, String::from("P1: EmptyTheContent"));   // = 1 Bodhidharma
  p.insert(2, String::from("P2: DhyanaSamadhi"));     // = 2 HuiNeng
  p.insert(3, String::from("P3: Samadhi"));           // = 3 Meditation from different angles
  p.insert(4, String::from("P4: Awareness"));         // = 4 Transcendental #Awareness
  p.insert(5, String::from("P5: Prajna"));            // = 5 #Prajna
  p.insert(6, String::from("P6: AwarenessPrajna"));   // = 6 #AwarenessPrajna
  p.insert(7, String::from("P7: SanadhiPrajna"));     // = 7 #SamadhiPrajna
  p.insert(8, String::from("P8: PrajnaTIP1"));        // = 8 #PrajnaTIP1
  p.insert(9, String::from("P9: PrajnaTIP2"));        // = 9 #PrajnaTIP2

  // x_traits driven by #Intuition (t) 
  let mut t = HashMap::new(); // observable traits from x-dimension
  t.insert(0, String::from("T-0: Balanced"));                // =  0, qualified person
  
  // the x_traits balanced by KindnessEmpathy from binding/clinging forces to total detachment
  t.insert(-1, String::from("T-1: KindnessEmpathy"));        // = -1
  t.insert(-2, String::from("T-2: Influenced"));             // = -2
  t.insert(-3, String::from("T-3: Veiled"));                 // = -3
  t.insert(-4, String::from("T-4: Indoctrinated"));          // = -4
  
  t.insert(1, String::from("T+1: Intuition"));               // = 1
  t.insert(2, String::from("T+2: Care"));                    // = 2
  t.insert(3, String::from("T+3: Honesty"));                 // = 3
  t.insert(4, String::from("T+4: Truth"));                   // = 4
  
  let mut f = HashMap::new(); // Taxonomy (types) of f_dimension
  f.insert(0, String::from("F 0: Persona")); //  Person sub f as the key
  
  // f(-1) = 1 = f(1) Empathy Awareness to be qualified as humanitas upward, enforced f(2) = 1
  f.insert(-1, String::from("F-1: EmpathyAwareness"));         // =  1
  f.insert(-2, String::from("F-2: KindnessAwareness"));        // = -1
  f.insert(-3, String::from("F-3: AnimalEnergy"));             // =  2
  f.insert(-4, String::from("F-4: ExtremeDesire"));            // = -3
  f.insert(-5, String::from("F-5: AnimalConsciousness"));      // =  5    in living person
  // --------- observable only from spirit world
  f.insert(-6, String::from("F-6: Smelly1"));                  // = -8    Degenerated spirits
  f.insert(-7, String::from("F-7: Smelly2"));                  // =  13   Vampire 
  f.insert(-8, String::from("F-8: Smelly3"));                  // = -34   X-of-prey
  
  f.insert(1, String::from("F+1: EquanimityAwareness"));       // = 1
  f.insert(2, String::from("F+2: PurityAwareness"));           // = 1
  f.insert(3, String::from("F+3: SignedPosts"));               // = 2
  f.insert(4, String::from("F+4: SelflessAwareness"));         // = 3
  f.insert(5, String::from("F+5: VisibleAwarenessPrajna"));    // = 5
  f.insert(6, String::from("F+6: EngagedAwarenessPrajna"));    // = 8
  f.insert(7, String::from("F+7: ForecastingSimulation"));     // = 13
  f.insert(8, String::from("F+8: SamadhiPrajna"));             // = 21
  
  // dynamic between one's avaluation and the KP's prediction
  _t = t_evaluation(t); // list of visible x_traits based on the outcomes in engaged living
  _p = p_evaluation(p); // list of visible y_pointers based on InnerPeace of Samadhi
  
  _y = y_evaluation(y); // transcendental Inner Peace based on the outcomes in engaged living
  _x = x_evaluation(x); // transcendental Awareness based on self evaluations and assessments
  _f = f_evaluation(f); // system evaluation of the persona states and stock of #Prajna
  
  // evaluation of InnerSpace
  let me = InnerSpace { // _t and _p as lists of traits and pointer evaluated from -3 to +3
    x_traits: _t,  
    y_pointers: _p,
    
    y_dimension: _y,
    x_dimension: _x,
    f_dimension: _f, 
  };

  return me;

} // end of build_InnerSpace from traits t, SignedPosts y, Awareness x, InnerSpace f

/*
  p_evaluation return evaluated value of y_pointer #EmptyTheContent toward #SanadhiPrajna
  and its reversed engineering for people connected to learn and share in the # (hashtag)
  community. We can use the generic evaluation type to be implemented for identified pointer
  which can question from LLM models then fine-tuned from KpPlatform community data and the
  custom data of the client.
*/
fn p_evaluation(p: HashMap<i32, String>) -> Vec<i32> { // return evaluated value to the blockchain

   println!("Valid Samadhi pointers"); // all claimed meditations must meet #DhyanaSamadhi
   for (key, value) in &p {
        println!("{key}: {value}");
    }
    // evaluate the transient value of p in circular processes of 8 hashtag and record
    // it somewhere for tracking records in different cases of claimed #
    
    return vec![0]; // qualified person starting the cultivation in #EmptyTheContent
}

/*
  t_evaluation return evaluated value of one's trait ranging from attachment to detachment
  to be explored which are tags for people connected to learn and share in the # (hashtag) 
  community. We can use the generic evaluation type to be implemented for identified trait 
  which can question from LLM models then fine-tuned for KpPlatform community data and the 
  custom data of the client.
*/
fn t_evaluation(t: HashMap<i32, String>) -> Vec<i32> { // return evaluated value to the blockchain

   println!("Valid traits");
   for (key, value) in &t {
        println!("{key}: {value}");
    }
    // evaluate the transient value of i in circular processes of 8 hashtag and record it
    // somewhere for tracking records in different cased of claimed #
    
    return vec![4]; // rated level of Intuition #Balanced
}

/* 
  List of traits ranging from #Indoctrinatd to #Truth. This part exposes the cheated 
  Descriptions such as Faith from all religions for testing and discovering the underlying 
  natural laws of their practices to be statistically evalutated by the world. For example, 
  we will prove that "Faith" does not need to be on the conflicting plan of duality due to 
  binding word/image and clinging thought leading to many religious crusades, and can be used 
  as a mean to ride on one's past binding forces as a form of Aspiration in transcending the 
  "Known" to reach the Y-dimension recorded by Gotama as Right Meditation. The proof is at 
  both theoretical level using math and at empirical levels as SHARED by different faith 
  practitioners of How to use "Faith / Trust" Aspiration in transcending the Known for 
  detoxifications of the tainted senses, realizing Kingdom of gods within. The "Hoax of Jesus 
  Redeemer" will be naturally exposed to scientifically point out the Right way of evolution 
  versus the Wrong way of degeneration as evidenced in the total collapse of the past
  esoteric feudal systems ruled by the degenerated King of Gods. This can be done with or 
  without the revelation of the real Jesus in his Continuity of the consciousness and major 
  Lesson Learned amongst Intelligent Beings.
  
  Similarly, the practices of Falun Dafa and all forms of Buddhist meditations are contrasted
  with claimed "Transcendental Meditation", Chinese "Qi", KhaiPhong's discovered
  underlying natural laws of Verifiable Processes from PrajnaTIPs, and other transcendental 
  technologies via Arts (singing, dancing, playing music, bonsai, merging with nature, etc) 
  to clean up cheaters "cooking sand and selling as rice".
  
  To have html note and links in sticky note for explanation in future implementations:
  pub enum Xtraits {Truth, Honesty,} struct Traits {kind: Xtraits, note: String, rating: i32}
  let _truth = Traits {kind: Xtraits::#Truth, rightclick: String::from("The trait of two-ways
  communications between Oneness >< Diversities"),};
  println!("I'm connecting to node {:?}!", _truth.kind);
*/
#[derive(Debug)]
pub enum Xtraits {
  Truth(String), 
  Honesty(String), 
  Care(String),
  Intuition(String), // Intuition Trust, Qi, Art
  Balanced(String), // qualified person
  KindnessEmpathy(String),  
  Influenced(String),
  Veiled(String),
  Indoctrinated(String), // visible in BindingWord, BindingImage, ClingingThought
}
// for outside contributions
#[derive(Debug)]
pub enum Xlevels {
  HonNhien(String), 
  ManagingFreshness(String),
  KnowingFreshness(String),    
  ProcessDiscovered(String), 
  KnowingThought(String), 
  CareViaCosmicEnergy(String),  
  HelpViaCosmicEnergy(String),  
  CulturalInfluenced(String),    
  RegionalInfluenced(String),
  NationalInfluenced(String),
  VeiledType(String),
  BindingWord(String),
  BindingImage(String),
  ClingingThought(String),
}

#[derive(Debug)]
pub enum Ypointers {
  QualifiedHumanitas(String),        // P0: QualifiedHumanitas has natural Peace
  EmptyTheContent(String), 
  DhyanaSamadhi(String),
  Samadhi(String),    // Samadhi must produce visible outcomes known by Gotama
  Awareness(String),  // Transcendental Awareness
  Prajna(String), 
  AwarenessPrajna(String),   // related to karma
  SamadhiPrajna(String),     // known by HuiNeng
  PrajnaTIP1(String),         // from KP
  PrajnaTIP2(String),
}
// for outside contributions
#[derive(Debug)]
pub enum Ylevels {
  Tranquility(String), 
  Equanimity(String),
  Purity(String),    
  Selfless(String), 
  NonThingness(String), 
  Unmoving(String),  
  Empathy(String),  
  Kindness(String),    
  Conscience(String),
  Conscience1(String),
  Conscience2(String),
  Conscience3(String),
}

#[derive(Debug)]
pub enum OneToDependent { 
  Oneness(String),          // Oneness >< Diversities
  Diversities(String),
  Heaven(String),           // I-Ching, LaoTzu
  Earth(String),  
  People(String), 
  Nhan(String),             // KungTzu
  Nghia(String),  
  Le(String),     
  Tri(String),
  Tin(String),  
  Morality(String),         // Falun  
  Patience(String), 
}

// for outside contributions
#[derive(Debug)]
pub enum Flevels {
  EquanimityAwareness(String), 
  PurityAwareness(String),
  SignedPosts(String),    
  SelflessAwareness(String), 
  VisibleAwarenessPrajna(String), 
  EngagedAwarenessPrajna(String),  
  ForecastingAwarenessPrajna(String),  
  VisibleSamadhiPrajna(String),    
  EmpathyAwareness(String),
  KindnessAwareness(String),
  AnimalEnergy(String),
  ExtremeDesire(String),
  AnimalConsciousness(String),
  Smelly1(String),
  Smelly2(String),
  Smelly3(String),
}


/*
 Traits are a way to group method signatures together to define a set of behaviors
 necessary to accomplish some purpose. By defining # x_trais varied from #Indoctrinated
 to verifiable #Truth in ChangeManagement toward SmartPointers for the manifestation of 
 #Prajna in required focused event, we force each type (claimed method) implementing the 
 traits such as Falun to provide its custom and measurable processes in the method's body 
 to be verified by the community and AI. Knowledge graph is a study the Described focused 
 event and best possible outcomes honestly evaluated by the persona of personal learned 
 lessons relevant to the norm of the mass.
*/
// x_traits
pub mod truth;    
pub mod honesty;
pub mod care;
pub mod intuition; 
pub mod balanced;
pub mod kindnessempathy;
pub mod influenced;
pub mod veiled;  
pub mod indoctrinated;   

// y_pointers
pub mod empty_the_content;  
pub mod dhyanasamadhi;  
pub mod samadhi;  
pub mod awareness; 
pub mod prajna;
pub mod awarenessprajna;
pub mod samadhiprajna;
pub mod prajnatip1;     
pub mod prajnatip2;


/*
  y_evaluation and x_evaluation return evaluated value of one's Y and X types to be further
  explored in these two dimensions that are inputs into the f_evaluation. We can make the
  function fn evaluation<T>(HashMap<i32, String>) -> &[T] {} over type T. The evaluation will
  return a reference to a value of the same type T which is the rated Fibonacci value of the
  T dimension.
*/
fn y_evaluation(y: HashMap<i32, String>) -> Vec<i32> { // return evaluated value to the blockchain

   println!("Valid SignedPost / TranscendentalInnerPeace");
   for (key, value) in &y {
        println!("{key}: {value}");
    }
    
    // model y Inner Peace based on one's observable pointers in circular processes of 9 
    //hashtags and record it somewhere for tracking records in different cased of claimed #
    
    return vec![0]; // rated level of SignedPost
}
fn x_evaluation(x: HashMap<i32, String>) -> Vec<i32> { // return evaluated value to the blockchain
   println!("Valid WuNien / Awareness");
   for (key, value) in &x {
        println!("{key}: {value}");
    }   
    
    // model x Perception based on one's observable traits in circular processes of 9 
    //hashtags and record it somewhere for tracking records in different cased of claimed #
    
    return vec![0]; // rated level of WuNien
    
}
/*
 let _wisdom = Ypointers::Prajna(String::from("#Prajna")); in the scale of rating system from
 1 - 5 for modeling the change management of f_evaluation and its preparation steps in
     Plan >< Execution >< Feedback of collected fact-based decision. 
*/
fn f_evaluation(f: HashMap<i32, String>) -> Vec<i32> {

    println!("Suggested Inner Space for cultivation");
       for (key, value) in &f {
        println!("{key}: {value}");
    }
    
    // evaluations based on hard evidences in community social networks and proven experts
    
    return vec![0]; // rated complex Fibinacci level
}

