/*
  Persona innerSpace is modeled with InnerAgent (Agnt2) interactions with OuterAgent (Agent1)
  via [ Activities, Relationships, and Places ]. We first use Fibonacci real numbers to be 
  later replaced with a complex variable according to Binet formula to model its right 
  evolution or wrong degeneration according to underlying natural laws to be DISCOVERED and 
  SHARED. The InnerSpace is cultivable via reversed engineering observable #Traits [ #Truth, 
  #Honesty,  #Care, #Intuition - #Balanced in qualified persona - #KindnessEmpathy, 
  #Influenced, #Veiled, #Indoctrinated ] and their managements via suggested custom 
  functions from reversed engineering in cultivating desired #SmartPointers 
  [ #EmptyTheContent, #DhyanaSamadhi, #Samadhi, #Awareness, #Prajna, #AwarenessPrajna, 
  #SamadhiPrajna, #PrajnaTIP1, PrajnaTIP2 ]. OuterAgent and InnerAgent are the gateways to 
  specialized agentic Services in current specialized domains such as Health, Law, Finance, 
  Governance, Social, Commerce, Education, Inter-Realms, etc.

  We generalize the persona for big corporation important person to (1) clean up tainted 
  senses and (2) cultivate required #Traits for its What-Count culture. The function of 
  "cleaning" and required "cultivating" are the core of the persona HR strategies. In this 
  sense, the required #Traits are its core Values (e.g. #Truth >< #Honesty >< #Care ><) in
  execution. This part will be standardized and open sources for possible value-added
  solutions in x_traits, y_pointers, [f x y]-dimension : vectors instead of i32.
*/
#[derive(Debug)]
pub struct InnerSpace {
  /* 
   x and f dimensions require deeper Fibonacci levels empirically observed in Vietnamese
   saying "Tu Nhà, Tu Chợ, Tu Chùa"
  */
  // Kp Signed Posts or Gotama Jhanas or Right #Samadhi
  pub y_dimension: Vec<i32>, // = vec![0, 1, 2, 3, 4, 5, 6, -1, -2, -3, -4, -5, -6],
  // HuiNeng #WuNien or KP #Awareness
  pub x_dimension: Vec<i32>, // = vec![0, 1, 2, 3, 4, 5, 6, 7, -1, -2, -3, -4, -5, -6, -7],
  // KP KP processes of #EmptyTheContent from HuiNeng three Nots through KP three Haves
  pub f_dimension: Vec<i32>,//vec![0, 1, 2, 3, 4, 5, 6, 7, 8, -1, -2, -3, -4, -5, -6, -7, -8] 
  
  // The rated observable #Traits and #SmartPointers from attached to balanced to detached
  // HashMap<i32, String> visible traits observable in x_dimension
  pub x_traits: Vec<i32>, // vec![0, 1, 2, 3, 4, -1, -2, -3, -4]
  
  pub y_pointers: Vec<i32>, // vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9]
  
  // HashMap<i32, String> rated pointer levels from outcomes of y_dimension
  pub p1_empty_the_content: Vec<i32>, // vec![0, 1, 2, 3, 4, 5]
  pub p2_dhyana_samadhi: Vec<i32>, // vec![0, 1, 2, 3, 4, 5]
  pub p3_samadhi: Vec<i32>, // vec![0, 1, 2, 3, 4, 5]
  pub p4_awareness: Vec<i32>, // vec![0, 1, 2, 3, 4, 5]
  pub p5_prajna: Vec<i32>, // vec![0, 1, 2, 3, 4, 5]
  pub p6_awareness_prajna: Vec<i32>, // vec![0, 1, 2, 3, 4, 5]
  pub p7_samadhi_prajna: Vec<i32>, // vec![0, 1, 2, 3, 4, 5]
  pub p8_prajna_tip1: Vec<i32>, // vec![0, 1, 2, 3, 4, 5]
  pub p9_prajna_tip2: Vec<i32>, // vec![0, 1, 2, 3, 4, 5]
  
  // each persona has lists of cultivable traits, pointers, depths of x_, y_, and f_ dimension
  // for both Inner cultivations and Outer suggestions in acquired ability to handle
  // required tasks in complex Fibonacci sequences of f_dimension from the balanced persona 
  // vec![-1, -2, -3, -4, -5, -6, -7, -8, 0, 1, 2, 3, 4, 5, 6, 7, 8]
}

// Implementations branched to different mod
pub mod ydimension; // states of consciousness driving observable Activities and Relationships

pub mod xdimension; // states of consciousness varying from attachment to detachment

pub mod fdimension; // other relevant factors making up the states and stock of the persona

pub mod x_traits;   // observable visible traits of one's #Awareness in engaged living

pub mod y_pointers; // cultivations to make the description closed to the described

pub mod p1_empty_the_content;

pub mod p2_dhyana_samadhi;

pub mod p3_samadhi;

pub mod p4_awareness;

pub mod p5_prajna;

pub mod p6_awareness_prajna;

pub mod p7_samadhi_prajna;

pub mod p8_prajna_tip1;
pub mod p9_prajna_tip2;

/*
  Dynamic interactions between normative and positive AI of custom modeling the
  objective function #Prajna = f(x, y) in "complex-valued data" to foster innovations /
  breakthroughs in the Persona's war room and strategies for various types: a Latin humanitas,
  next qualified realm, an organization, foreign affairs, national development, etc.
  
  Modeling the InnerSpace is the systematic building internal power for different stages of
  the persona developments. Academia research + LLM statistical significance. These methods
  will be used as a part of f_evaluation depending on the type of persona and required service

impl InnerSpace { // different methods for different InnerSpace persona types

  fn f_evaluation<'a>(_x: &'a x_dimension, _y: &'a y_dimension) -> &'a str { // f(_x,_y)
    return "EmpathyAwareness".to_string()
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
}
*/
  

/* 
  HashMap will be replaced with db Map, a bare bone K-V store with namespace of unique Id and
  cgroups rated at 8 security levels for private container-to-container communications and
  collaborations at 8 levels of security: (1): KP container, (2): KP container-to-container, 
  (3): certified source, (4): auditted legal requirements, (5): hardware Id, (6): trusted, 
  (7): validated after minimum of 8 years, (8): worthy capable of reasoning
  https://www.youtube.com/watch?v=LOF6fvV5XUc (LLM + KnowledgeGraph)
  https://www.youtube.com/watch?v=cyGZPF_RMNE (reasoning LLM for "subjective experience")
  https://www.youtube.com/watch?v=Es6yuMlyfPw (causal reasoning PLUS scientifically 
  cultivable #Prajna for a breakthrough in any conflicting issue.
  
  The db Map belongs the owner having its unique relationships KP Ids and the firmware code
  standardization of #EmptyTheContent toward Non-Thingness / Nothingness in the owner
  scientifically cultivable #Prajna in the No-Conflict Consciousness orthogonal to the 
  duality plane of conflicting consciousness. The value of key "owner / स्वामी" is Vec v of 
  unique Array of bytes: &v[0] = onwer id, &v[1] = onwer hardware id, &v[2] = onwer KP 
  certified security level. Its relationship key id has &v[0] = the relationship with the 
  owner, &v[1] = the persona hardware id, &v[2] = persona KP certified security level. 
  Additional values are two possible related realms the "owner / स्वामी" may be naturally 
  reincarnated where the distributed records are securely maintained, waiting for its 
  legitimate activation based on our empirically Know-How to avoid past mistakes hard-learned 
  from 60,000+ human-years of esoteric degenerated feudal systems. Security features can be 
  physically custom designed and implemented at chip level. Only containers belong to those 
  having RoT (Root of Trust) can communicate and collaborate within a certified boundary.
*/
use std::collections::HashMap; // to be replaced by db Map

// given the past [ x-y-f ] dimensions, we re-evaluate pointers and traits for suggestions
pub fn build_inner_space(_x: Vec<i32>, _y: Vec<i32>, _f: Vec<i32>) -> InnerSpace  {
  // qualified persona at balanced traits, make it as observable list

  // to be evaluated in blockchain of user self-evaluation and AI
  let y_dimension = _y; // move to y_dimension
  // opinions from fact-base tracked records in engaged living
  let x_dimension = _x; // move to x_dimension
   // qualified person
  let f_dimension = _f; // move to f_dimension
  
  let x_traits: Vec<i32> = Vec::new(); 
  // qualified persona at Awareness, make it as observable list
  
  let y_pointers: Vec<i32> = Vec::new();
  
  let p1_empty_the_content: Vec<i32> = Vec::new();
  let p2_dhyana_samadhi: Vec<i32> = Vec::new();
  let p3_samadhi: Vec<i32> = Vec::new();
  let p4_awareness: Vec<i32> = Vec::new();
  let p5_prajna: Vec<i32> = Vec::new();
  let p6_awareness_prajna: Vec<i32> = Vec::new();
  let p7_samadhi_prajna: Vec<i32> = Vec::new();
  let p8_prajna_tip1: Vec<i32> = Vec::new();
  let p9_prajna_tip2: Vec<i32> = Vec::new();
  
  // new evaluation  of InnerSpace
  
  let mut _y = y_dimension;
  let mut _x = x_dimension;
  let mut _f = f_dimension;
  
  let mut _t = x_traits;
  
  let mut _p = y_pointers;

  let mut _p1 = p1_empty_the_content;
  let mut _p2 = p2_dhyana_samadhi;
  let mut _p3 = p3_samadhi;
  let mut _p4 = p4_awareness;
  let mut _p5 = p5_prajna;
  let mut _p6 = p6_awareness_prajna;
  let mut _p7 = p7_samadhi_prajna;
  let mut _p8 = p8_prajna_tip1;
  let mut _p9 = p9_prajna_tip2;

 // rebuild the relevant dimensions according to open standarded proven researches
 
  let mut y = HashMap::new(); // Taxonomy of y_dimension
  y.insert(0, String::from("Y 0: Peace"));         // =  0
  
  // y(-1) = 1 = y(1) Empathy to Tranquility enforced in y(2) = 1 of Equanimity
  y.insert(1, String::from("Y+1: Tranquility"));   // = 1
  y.insert(2, String::from("Y+2: Equanimity"));    // = 1
  y.insert(3, String::from("Y+3: Purity"));        // = 2
  y.insert(4, String::from("Y+4: Selfless"));      // = 3
  y.insert(5, String::from("Y+5: NonThingness"));  // = 5  Gotama's impass - SelfSelfless Act.
  y.insert(6, String::from("Y+6: Unmoving"));      // = 8  Gotama's impass - #SamadhiPrajna
  
  y.insert(-1, String::from("Y-1: Empathy"));      // =  1
  y.insert(-2, String::from("Y-2: Kindness"));     // = -1
  y.insert(-3, String::from("Y-3: Conscience"));   // =  2
  y.insert(-4, String::from("Y-4: NoConscience"));  // = -3
  y.insert(-5, String::from("Y-5: InflictedFear"));  // =  5 capable for hegemony
  y.insert(-6, String::from("Y-6: Vampire"));  // = -8
  
  let mut x = HashMap::new(); // Taxonomy of x_dimension for internal evaluation
  x.insert(0, String::from("X 0: Blank"));             // =  0
  
  // x(-1) = 1 = x(1) HonNhien enforced @ x(2)=1 of proper management in Equanimity
  x.insert(1, String::from("X+1: HonNhien"));              // = 1
  x.insert(2, String::from("X+2: ManagingFreshness"));     // = 1
  x.insert(3, String::from("X+3: KnowingFreshness"));      // = 2
  x.insert(4, String::from("X+4: ProcessDiscovered"));     // = 3
  x.insert(5, String::from("X+5: KnowingThought"));        // = 5     breakout = 5
  x.insert(6, String::from("X+6: CareViaCosmicEnergy"));   // = 8
  x.insert(7, String::from("X+7: HelpViaCosmicEnergy"));   // = 13    consciousness technology
  
  x.insert(-1, String::from("X-1: CulturalInfluenced"));   // =  1
  x.insert(-2, String::from("X-2: RegionalInfluenced"));   // = -1
  x.insert(-3, String::from("X-3: NationalInfluenced"));   // =  2
  x.insert(-4, String::from("X-4: VeiledType"));           // = -3
  x.insert(-5, String::from("X-5: BindingWord"));          // =  5    breakout
  x.insert(-6, String::from("X-6: BindingImage"));         // = -8
  x.insert(-7, String::from("X-7: ClingingThought"));      // = 13    indoctrinated
  
  let mut f = HashMap::new(); // Taxonomy (types) of f_dimension
  f.insert(0, String::from("F 0: Persona")); //  Person sub f as the key

  // f(-1) = 1 = f(1) Empathy Awareness to be qualified as humanitas upward, enforced f(2) = 1
  f.insert(1, String::from("F+1: EquanimityAwareness"));       // = 1
  f.insert(2, String::from("F+2: PurityAwareness"));           // = 1
  f.insert(3, String::from("F+3: SignedPosts"));               // = 2
  f.insert(4, String::from("F+4: SelflessAwareness"));         // = 3
  f.insert(5, String::from("F+5: VisibleAwarenessPrajna"));    // = 5
  f.insert(6, String::from("F+6: EngagedAwarenessPrajna"));    // = 8
  f.insert(7, String::from("F+7: ForecastingSimulation"));     // = 13
  f.insert(8, String::from("F+8: SamadhiPrajna"));             // = 21
  
  f.insert(-1, String::from("F-1: EmpathyAwareness"));         // =  1
  f.insert(-2, String::from("F-2: KindnessAwareness"));        // = -1
  f.insert(-3, String::from("F-3: AnimalEnergy"));             // =  2
  f.insert(-4, String::from("F-4: ExtremeDesire"));            // = -3
  f.insert(-5, String::from("F-5: AnimalConsciousness"));      // =  5    in living person
  // --------- observable only from spirit world
  f.insert(-6, String::from("F-6: Smelly1"));                  // = -8    Vampire
  f.insert(-7, String::from("F-7: Smelly2"));                  // =  13 
  f.insert(-8, String::from("F-8: Smelly3"));                  // = -34 
  
  // Only from humanity upward, one can have strong enough No-Conflict Consciousness to be
  // at the based line from duality plane to ascend the no-conflict plane having y_pointers
  // for detoxification of tainted senses at different depths glimpsed by past explorers 
  
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
  
  let mut p1 = HashMap::new(); // EmptyTheContent toward #NonThingness
  p1.insert(0, String::from("R0: Qualified")); 
  p1.insert(1, String::from("R1: Tranquility"));    // = 1 Swiss
  p1.insert(2, String::from("R2: Equanimity"));     // = 2 HOC
  p1.insert(3, String::from("R3: Purity"));         // = 3 Resort
  p1.insert(4, String::from("R4: Selfless"));       // = 4 Selfless
  p1.insert(5, String::from("R5: NonThingness"));   // = 5 NonThingness

  let mut p2 = HashMap::new(); // DhyanaSamadhi toward #NonThingness
  p2.insert(0, String::from("R0: Qualified")); 
  p2.insert(1, String::from("R1: Tranquility"));    // = 1 Swiss
  p2.insert(2, String::from("R2: Equanimity"));     // = 2 HOC
  p2.insert(3, String::from("R3: Purity"));         // = 3 Resort
  p2.insert(4, String::from("R4: Selfless"));       // = 4 Selfless
  p2.insert(5, String::from("R5: NonThingness"));   // = 5 NonThingness

  let mut p3 = HashMap::new(); // Samadhi toward #NonThingness
  p3.insert(0, String::from("R0: Qualified")); 
  p3.insert(1, String::from("R1: Tranquility"));    // = 1 Swiss
  p3.insert(2, String::from("R2: Equanimity"));     // = 2 HOC
  p3.insert(3, String::from("R3: Purity"));         // = 3 Resort
  p3.insert(4, String::from("R4: Selfless"));       // = 4 Selfless
  p3.insert(5, String::from("R5: NonThingness"));   // = 5 NonThingness

  let mut p4 = HashMap::new(); // Awareness toward #NonThingness
  p4.insert(0, String::from("R0: Qualified")); 
  p4.insert(1, String::from("R1: Tranquility"));    // = 1 Swiss
  p4.insert(2, String::from("R2: Equanimity"));     // = 2 HOC
  p4.insert(3, String::from("R3: Purity"));         // = 3 Resort
  p4.insert(4, String::from("R4: Selfless"));       // = 4 Selfless
  p4.insert(5, String::from("R5: NonThingness"));   // = 5 NonThingness

  let mut p5 = HashMap::new(); // Prajna toward #NonThingness
  p5.insert(0, String::from("R0: Qualified")); 
  p5.insert(1, String::from("R1: Tranquility"));    // = 1 Swiss
  p5.insert(2, String::from("R2: Equanimity"));     // = 2 HOC
  p5.insert(3, String::from("R3: Purity"));         // = 3 Resort
  p5.insert(4, String::from("R4: Selfless"));       // = 4 Selfless
  p5.insert(5, String::from("R5: NonThingness"));   // = 5 NonThingness

  let mut p6 = HashMap::new(); // AwarenessPrajna toward #NonThingness
  p6.insert(0, String::from("R0: Qualified")); 
  p6.insert(1, String::from("R1: Tranquility"));    // = 1 Swiss
  p6.insert(2, String::from("R2: Equanimity"));     // = 2 HOC
  p6.insert(3, String::from("R3: Purity"));         // = 3 Resort
  p6.insert(4, String::from("R4: Selfless"));       // = 4 Selfless
  p6.insert(5, String::from("R5: NonThingness"));   // = 5 NonThingness

  let mut p7 = HashMap::new(); // EmptyTheContent toward #NonThingness
  p7.insert(0, String::from("R0: Qualified")); 
  p7.insert(1, String::from("R1: Tranquility"));    // = 1 Swiss
  p7.insert(2, String::from("R2: Equanimity"));     // = 2 HOC
  p7.insert(3, String::from("R3: Purity"));         // = 3 Resort
  p7.insert(4, String::from("R4: Selfless"));       // = 4 Selfless
  p7.insert(5, String::from("R5: NonThingness"));   // = 5 NonThingness

  let mut p8 = HashMap::new(); // DhyanaSamadhi toward #NonThingness
  p8.insert(0, String::from("R0: Qualified")); 
  p8.insert(1, String::from("R1: Tranquility"));    // = 1 Swiss
  p8.insert(2, String::from("R2: Equanimity"));     // = 2 HOC
  p8.insert(3, String::from("R3: Purity"));         // = 3 Resort
  p8.insert(4, String::from("R4: Selfless"));       // = 4 Selfless
  p8.insert(5, String::from("R5: NonThingness"));   // = 5 NonThingness

  let mut p9 = HashMap::new(); // Samadhi toward #NonThingness
  p9.insert(0, String::from("R0: Qualified")); 
  p9.insert(1, String::from("R1: Tranquility"));    // = 1 Swiss
  p9.insert(2, String::from("R2: Equanimity"));     // = 2 HOC
  p9.insert(3, String::from("R3: Purity"));         // = 3 Resort
  p9.insert(4, String::from("R4: Selfless"));       // = 4 Selfless
  p9.insert(5, String::from("R5: NonThingness"));   // = 5 NonThingness

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
  
  // starting from qualified persona with digital Id and chip Id for minimum secure
  // container-2-container private communications and collaboration, there are custom 8
  // custom security levels. In each IamX, higher security can access lower levels but Not 
  // reversed https://www.youtube.com/watch?v=HmE0hUifX7Q
  let mut namespace = HashMap::new(); // the namespace and its cgroups of 8 security levels
  namespace.insert(String::from("Owner"), vec![String::from("0_123456789"),
   String::from("chip_0_987654321")]); // each relation is 0_Id and vector of joined data at
  println!("{namespace:?}"); 
  

  // dynamic between one's avaluation and the KP's prediction  
  _y = y_evaluation(y); // transcendental Inner Peace based on the outcomes in engaged living
  _x = x_evaluation(x); // transcendental Awareness based on self evaluations and assessments
  _f = f_evaluation(f); // system evaluation of the persona states and stock of #Prajna
  
  _t = t_evaluation(t); // list of visible x_traits based on the outcomes in engaged living
  _p = p_evaluation(p); // list of visible y_pointers based on InnerPeace of Samadhi
  
  _p1 = p1_evaluation(p1); // EmptyTheContent qualified at level 0 to 5 of NonThingness
  _p2 = p2_evaluation(p2); // DhyanaSamadhi qualified at level 0 to 5 of NonThingness
  _p3 = p3_evaluation(p3); // Samadhi qualified at level 0 to 5 of NonThingness
  _p4 = p4_evaluation(p4); // Awareness qualified at level 0 to 5 of NonThingness
  _p5 = p5_evaluation(p5); // Prajna qualified at level 0 to 5 of NonThingness
  _p6 = p6_evaluation(p6); // AwarenessPrajna qualified at level 0 to 5 of NonThingness
  _p7 = p7_evaluation(p7); // SamadhiPrajna qualified at level 0 to 5 of NonThingness
  _p8 = p8_evaluation(p8); // PrajnaTIP1 qualified at level 0 to 5 of NonThingness
  _p9 = p9_evaluation(p9); // PrajnaTIP2 qualified at level 0 to 5 of NonThingness
  
  // evaluation of InnerSpace
  let me = InnerSpace { // _t and _p as lists of traits and pointer evaluated from -3 to +3
    x_traits: _t,  
    y_pointers: _p,
    p1_empty_the_content: _p1,
    p2_dhyana_samadhi: _p2,    
    p3_samadhi: _p3,
    p4_awareness: _p4,
    p5_prajna: _p5,    
    p6_awareness_prajna: _p6,
    p7_samadhi_prajna: _p7,
    p8_prajna_tip1: _p8,    
    p9_prajna_tip2: _p9,
    
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
  custom data of the client. Fact-based evaluation via Activities and Relationships of
  pointers may be general. But when combined with rated traits revealed in Activities and
  Relationships, we can statistically evaluate the persona rated Fibonacci level.
*/
fn p_evaluation(p: HashMap<i32, String>) -> Vec<i32> { // return evaluated value to the blockchain

   println!("Valid SmartPointers"); // all claimed meditations must meet #DhyanaSamadhi
   for (key, value) in &p {
        println!("{key}: {value}");
    }
    // evaluate the transient value of p in circular processes of 8 hashtag and record
    // it somewhere for tracking records in different cases of claimed #
    
    return vec![0]; // qualified person starting the cultivation in #EmptyTheContent
}

fn p1_evaluation(p1: HashMap<i32, String>) -> Vec<i32> { // return evaluated value to the blockchain

   println!("Valid EmptyTheContent pointer"); // all claims must be evaluated
   for (key, value) in &p1 {
        println!("{key}: {value}");
    }
    // evaluate the transient value of p1 in circular processes at 5 levels and record
    // it somewhere for tracking records in different cases of claimed #
    
    return vec![0]; // qualified person starting the cultivation in #EmptyTheContent
}

fn p2_evaluation(p2: HashMap<i32, String>) -> Vec<i32> { // return evaluated value to the blockchain

   println!("Valid DhyanaSamadhi pointer"); // all claims must be evaluated
   for (key, value) in &p2 {
        println!("{key}: {value}");
    }
    // evaluate the transient value of p2 in circular processes at 5 levels and record
    // it somewhere for tracking records in different cases of claimed #
    
    return vec![0]; // qualified person starting the cultivation in #DhyanaSamadhi
}

fn p3_evaluation(p3: HashMap<i32, String>) -> Vec<i32> { // return evaluated value to the blockchain

   println!("Valid Samadhi pointer"); // all claims must be evaluated
   for (key, value) in &p3 {
        println!("{key}: {value}");
    }
    // evaluate the transient value of p3 in circular processes at 5 levels and record
    // it somewhere for tracking records in different cases of claimed #
    
    return vec![0]; // qualified person starting the cultivation in #Samadhi
}

fn p4_evaluation(p4: HashMap<i32, String>) -> Vec<i32> { // return evaluated value to the blockchain

   println!("Valid Awareness pointer"); // all claims must be evaluated
   for (key, value) in &p4 {
        println!("{key}: {value}");
    }
    // evaluate the transient value of p4 in circular processes at 5 levels and record
    // it somewhere for tracking records in different cases of claimed #
    
    return vec![0]; // qualified person starting the cultivation in #Awareness
}

fn p5_evaluation(p5: HashMap<i32, String>) -> Vec<i32> { // return evaluated value to the blockchain

   println!("Valid Prajna pointer"); // all claims must be evaluated
   for (key, value) in &p5 {
        println!("{key}: {value}");
    }
    // evaluate the transient value of p5 in circular processes at 5 levels and record
    // it somewhere for tracking records in different cases of claimed #
    
    return vec![0]; // qualified person starting the cultivation in #Prajna
}

fn p6_evaluation(p6: HashMap<i32, String>) -> Vec<i32> { // return evaluated value to the blockchain

   println!("Valid AwarenessPrajna pointer"); // all claims must be evaluated
   for (key, value) in &p6 {
        println!("{key}: {value}");
    }
    // evaluate the transient value of p6 in circular processes at 5 levels and record
    // it somewhere for tracking records in different cases of claimed #
    
    return vec![0]; // qualified person starting the cultivation in #AwarenessPrajna
}

fn p7_evaluation(p7: HashMap<i32, String>) -> Vec<i32> { // return evaluated value to the blockchain

   println!("Valid SamadhiPrajna pointer"); // all claims must be evaluated
   for (key, value) in &p7 {
        println!("{key}: {value}");
    }
    // evaluate the transient value of p7 in circular processes at 5 levels and record
    // it somewhere for tracking records in different cases of claimed #
    
    return vec![0]; // qualified person starting the cultivation in #SamadhiPrajna
}

fn p8_evaluation(p8: HashMap<i32, String>) -> Vec<i32> { // return evaluated value to the blockchain

   println!("Valid PrajnaTIP1 pointer"); // all claims must be evaluated
   for (key, value) in &p8 {
        println!("{key}: {value}");
    }
    // evaluate the transient value of p8 in circular processes at 5 levels and record
    // it somewhere for tracking records in different cases of claimed #
    
    return vec![0]; // qualified person starting the cultivation in #PrajnaTIP1
}

fn p9_evaluation(p9: HashMap<i32, String>) -> Vec<i32> { // return evaluated to blockchain

   println!("Valid PrajnaTIP2 pointer"); // all claims must be evaluated
   for (key, value) in &p9 {
        println!("{key}: {value}");
   }
    // evaluate the transient value of p9 in circular processes at 5 levels and record
    // it somewhere for tracking records in different cases of claimed #
    
    return vec![0]; // qualified person starting the cultivation in #PrajnaTIP2
}


/*
  t_evaluation return evaluated value of one's trait ranging from attachment to detachment
  to be explored which are tags for people connected to learn and share in the # (hashtag) 
  community. We can use the generic evaluation type to be implemented for identified trait 
  which can question from LLM models then fine-tuned for KpPlatform community data and the 
  custom data of the client. The observable traits are rated from Attachments to Detached
  which can be used to clean up negative traits and enforce positive traits to be 
  statistically evaluated at the rated Fibonacci level.
*/
fn t_evaluation(t: HashMap<i32, String>) -> Vec<i32> { // return evaluated value to the blockchain

   println!("Valid traits");
   for (key, value) in &t {
        println!("{key}: {value}");
    }
    // evaluate the transient value of i in circular processes of 8 hashtag and record it
    // somewhere for tracking records in different cased of claimed #
    
    return vec![0]; // rated level of Intuition #Balanced
}

/* 
  List of traits ranging from #Indoctrinated to #Truth. This part exposes the cheated 
  Descriptions such as Faith from all religions for testing and discovering the underlying 
  natural laws of their practices to be statistically evalutated by the world. For example, 
  we will prove that "Faith" does not need to be on the conflicting plan of duality due to 
  binding word/image and clinging thought leading to many religious crusades, and can be used 
  as a mean to ride on one's past binding forces as a form of Aspiration in transcending the 
  "Known" to reach the Y-dimension recorded by Gotama as Right Meditation. The proof is at 
  both theoretical level using math and at empirical levels as SHARED by different faith 
  practitioners of How to use "Faith / Trust" Aspiration in transcending the Known for 
  detoxifications of the tainted senses, realizing GodKingdom within. The "Hoax of Jesus 
  Redeemer" will be naturally exposed to scientifically point out the Right way of evolution 
  versus the Wrong way of degeneration as evidenced in the total collapse of the past
  esoteric feudal systems ruled by the degenerated King of Gods. This can be done with or 
  without the revelation of the real Jesus in his Continuity of the consciousness and major 
  Lesson Learned amongst Intelligent Beings.
  
  Similarly, the practices of Falun Dafa and all forms of Buddhist meditations are contrasted
  with claimed "Transcendental Meditation", Chinese "Qi", KhaiPhong's discovered
  underlying natural laws of Verifiable Processes from PrajnaTIPs, and other transcendental 
  technologies via Arts (singing, dancing, playing music, bonsai, merging with nature, etc) 
  to clean up cheaters "cooking sand and selling / hallucinated as rice". Make X, Y, F 
  levels as struts having methods to use trait bound to a generic type of certain behavior.
  
  To have html note and links in sticky note for explanation in implementations of "Trait" 
  definitions to group method signatures together to define a set of behaviors necessary to 
  accomplish some purpose. Tweet is an instance people can expose their views on the rating
  of X, Y, F or Tweet-on-the-Tweet for reply or retweet:
  
  The "traits" will be academically studied of observable attributes in Activities and 
  Relationships, then exposed as # for people tweet and tweet-on-tweet to be avaluated and
  rated at community conscientious level. The trait function to be redefined to Rust trait
  with fn explore which is an implementation of iterator plus additional function to verify
  if the recorded activities, relationships at all places have identified traits at
  identified "being" level.
  
  pub enum Xtraits {Truth, Honesty,} struct Traits {kind: Xtraits, note: String, rating: i32}
  let _truth = Traits {kind: Xtraits::#Truth, rightclick: String::from("The trait of two-ways
  communications between Oneness >< Diversities"),};
  println!("I'm connecting to node {:?}!", _truth.kind);
  
*/

/// Explore trait - extension of iterator - in activities and relationships.
///
/// pub trait Truth 
///	{
///		fn truth<T, E>(self) -> std::result::Result<Vec<T>, E>
/// 	where
/// 		Self: Iterator<Item = std::result::Result<Vec<T>, E>> + Sized;
/// }
///
/// 	impl<It> Truth for It
/// 	where
///		    It: Iterator + Sized,
/// 	{
///		    fn truth<T, E>(mut self) -> std::result::Result<Vec<T>, E>
///		    where
///		        Self: Iterator<Item = std::result::Result<Vec<T>, E>> + Sized,
///		    {
///		        let mut xs = Vec::new();
///		        loop {
///		            match self.next() {
///		                Some(Ok(x)) => xs.extend(x),
///		                Some(e) => {
///		                    return e; // propagate error
///		                }
///		                None => {
///		                    break;
///		                }
///		            }
///		        }
///		        Ok(xs {
///					explore rated value of the trait and add to the node
/// 			})
///		    }
///		}
///
///     type Item;
/// 	answer i32; // rated value of the trait
///     fn explore(&mut self) -> answer, Option<self::Item>

/// assert_eq!(0, answer);
/// ```

/*
pub enum Xtraits {
  Truth, Honesty, Care, Intuition, Balanced,
  KindnessEmpathy, Influenced, Veiled, Indoctrinated,
}
*/

#[derive(Debug)]
pub enum Xtraits {
  Truth(String),			// use cosmic energy
  Honesty(String), 			// KnowingThought
  Care(String),				// investigation, inquiries
  Intuition(String),		// Intuition Trust, Qi, Art
  Balanced(String),			// qualified person
  KindnessEmpathy(String),	// HonNhien
  Influenced(String),		// cultural, regional, national
  Veiled(String),			// hoax, fooled
  Indoctrinated(String),	// visible in BindingWord, BindingImage, ClingingThought
}
// for outside contributions
#[derive(Debug)]
pub enum Xlevels { // type behavior based on its traits and its rated major bounded one
  HonNhien(String),				// #KindnessEmpathy f<sub>1</sub> = 1
  ManagingFreshness(String),	// #Balanced f<sub>2</sub> = 1
  KnowingFreshness(String),		// #Intuition f<sub>3</sub> = 2
  ProcessDiscovered(String),	// #Care f<sub>4</sub> = 3
  KnowingThought(String),		// #Honesty f<sub>5</sub> = 5
  CareViaCosmicEnergy(String),	// #Truth f<sub>6</sub> = 8
  HelpViaCosmicEnergy(String),	// #Truth f<sub>7</sub> = 13
  CulturalInfluenced(String),	// #Influenced
  RegionalInfluenced(String),	// #Influenced
  NationalInfluenced(String),	// #Influenced
  VeiledType(String),			// #Veiled
  BindingWord(String),			// #Indoctrinated
  BindingImage(String),			// #Indoctrinated
  ClingingThought(String),		// #Indoctrinated
}

/*
 Based on empirical observation of a naturally qualified person, one has innate ability to be
 outside-the-box due to sufficient compassion and #Prajna in visible trait #Balanced above
 the visible trait of #KindnessEmpathy defined in Latin humanitas. This is Vietnamese KienTanh
 to (1) claim one's Dignity of Human Rights protected by modern society, then (2) DISCOVER
 and SHARE underlying natural laws to personally verify statistically significant epistemic
 objectives such as Gotama's #FourFoldTruth of Dukkha, Jesus's #GodKingdom within, Latin
 humanitas, #TamingTheOx; so are following Ypointers.
 
 All manifestations are conditioned and subjected to changes at Planck time. It has been
 proven in Generative AI that an optimal process can be engineered for a desired manifestation
 happened. Based on recorded experiences of LaoTzu, Gotama, Jesus, Bodhidharma, HuiNeng, etc,
 their states of manifestations Ylevels naturally produce smart pointers pointing to
 different observable traits in their qualities. "#Truth" is the identified "trait" in their
 qualities of [ Diversities >< Oneness / Hửu Không Vô Ngại / Self-Selfless Actualization ].
 Observable smart pointers are reported pointers pointing to the required traits in their
 observable evolution or negative traits leading to degeneration. Starting from achievable
 qualities of these front-line soldiers, we identify smart pointers ready for academia 
 researches the structure and attributes of these smart pointers, then expose them in # for
 public tweet and tweet-on-tweet further contribution in LLM models.


//! Enforce QualifiedHumanitas for natural detachment of dhyana, then Samadhi for
//! DhyanaSamadhi in activities and relationships, pointing to traits for suggested
//! attributes visibly appeared in engaged living.
//!
/// pub pointer EmptyTheContent {
///     type Item;
/// 	answer i32; // rated value of the trait
///     fn explore(&mut self) -> answer, Option<self::Item>
///		fn truth<T, E>(self) -> std::result::Result<Vec<T>, E>
/// 	where
/// 		Self: Iterator<Item = std::result::Result<Vec<T>, E>> + Sized;
/// }
///
/// 

/// assert_eq!(0, answer);
/// ```


struct EmptyTheContent {
	type item;
}

pub enum Ypointers { QualifiedHumanitas, 
	EmptyTheContent, DhyanaSamadhi, Samadhi, Awareness,
	Prajna, AwarenessPrajna, SamadhiPrajna, PrajnaTIP1, PrajnaTIP2,
}
*/


#[derive(Debug)]
pub enum Ypointers {
  QualifiedHumanitas(String),	// P0: QualifiedHumanitas has natural Peace
  
  EmptyTheContent(String),		// General smart pointers in natural Detachments
  DhyanaSamadhi(String),		// General smart pointers in all visible meditations
  Samadhi(String),    			// Right #Samadhi of visible outcomes known by Gotama
  Awareness(String), 			// #Awareness to explicitly qualify Gotama's Eightfold Path
  Prajna(String), 				// Visible manifestation of outside-the-box or breakthrough
  AwarenessPrajna(String),   	// related to karma forcing all Buddhists delivering outcomes
  SamadhiPrajna(String),		// known by HuiNeng
  PrajnaTIP1(String),			// from Kp in Sound technologies
  PrajnaTIP2(String),			// from Kp in Empathy
}
// for outside contributions
#[derive(Debug)]
pub enum Ylevels { // type behavior based on its traits and its rated major bounded one
  Tranquility(String),		// #KindnessEmpathy
  Equanimity(String),		// #Balanced
  Purity(String), 			// #Intuition
  Selfless(String),			// #Care
  NonThingness(String),		// #Honesty
  Unmoving(String), 		// #Truth
  Empathy(String),			// #Influenced
  Kindness(String),			// #Influenced
  Conscience(String),		// #Veiled of Right and Wromg
  NoConscience(String),	    // #Indoctrinated
  InflictedFear(String),    // #Indoctrinated
  Vampire(String),		    // #Indoctrinated
}

#[derive(Debug)]
pub enum OneToDependent { 
  Oneness(String),          // Oneness >< Diversities
  Diversities(String),
  
  AtHome(String),			
  RightEffort(String),
  TamingTheOx(String),		// TamingTheOx - Epistemic Objective
  RidingTheOx(String),
  NonThingness(String),
  TheSource(String),
  AwakeningBudh(String),
  
  FourFoldTruth(String),	// of Dukkha - Epistemic Objective
  GodKingdom(String),		// within - Epistemic Objective
  
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
  
  // Vedic system "the hubs to receive, assimilate, and distribute life energies”
  RootChakra(String),		// Muladhara - 		red
  SacralChakra(String),		// Svadhishthana - 	orange
  ThirdChakra(String),		// Mapipura -		yellow
  HeartChakra(String),		// Anahata -		green
  ThroatChakra(String),		// Vishuddha - 		blue
  ThirdEye(String),			// Ajna - 			purple
  CrownChakra(String),		// Sahasrata - 		white
}

// for outside contributions
#[derive(Debug)]
pub enum Flevels { // type behavior based on its traits and its rated major bounded one
  EquanimityAwareness(String), 			// #KindnessEmpathy
  PurityAwareness(String),				// #Balanced
  SignedPosts(String),    				// #Balanced
  SelflessAwareness(String), 			// #Intuition
  VisibleAwarenessPrajna(String), 		// #Care
  EngagedAwarenessPrajna(String),  		// #Honesty
  ForecastingAwarenessPrajna(String),	// #Truth
  VisibleSamadhiPrajna(String), 		// #Truth
  EmpathyAwareness(String),				// #Influenced
  KindnessAwareness(String),			// #Influenced
  AnimalEnergy(String),					// #Veiled
  ExtremeDesire(String),				// #Indoctrinated
  AnimalConsciousness(String),			// #Indoctrinated
  Smelly1(String),						// esoteric Degenerated
  Smelly2(String),						// esoteric Vampire
  Smelly3(String),						// esoteric X-of-Prey
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

/*
 Pointer is a general concept for a variable that contains an address in memory. Smart 
 pointers are data structures that act like a pointer but also have additional metadata and
 capabilities. Reference smart pointer type enables you to allow data to have multiple owners 
 by keeping track of the number of owners and, when no owners remain, cleaning up the data.
 This is a Kp tool for one-on-one consultation with dedicated front-line soldiers.
*/
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
     
  fn f_evaluation<'a>(_x: &'a x_dimension, _y: &'a y_dimension) -> &'a str { // f(_x,_y)
    return "EmpathyAwareness".to_string()     
*/    
fn f_evaluation(f: HashMap<i32, String>) -> Vec<i32> {

    println!("Suggested Inner Space for cultivation");
       for (key, value) in &f {
        println!("{key}: {value}");
    }
    
    // evaluations based on hard evidences in social relationships and proven experts
    
    return vec![0]; // rated complex Fibinacci level
}

