/*
 The result of KP 50+ years of field researches PLUS exoteric / esoteric observations of
 qualified candidates as the first front-line soldiers while waiting for verifications and/or
 modifications / enhancements of later academia contributions. We start from actual
 experiences of a Vietnamese student since 1970 all Gotama's recorded Jhanas, further 
 investigated leading to Bodhidharma's exchange with Emperor Wu (Lương Vũ Đế) on “What is the 
 highest meaning of noble #Truth" which is fact-based epistemic objective and HuiNeng's specialized contributions about (1) the Three-Not (Wu-Nien, No-Mark, Detachment / Vô Niệm, Vô Vết, Vô Trụ), (2) #DhyanaSamadhi / Thiền Định, (3) #SamadhiPrajna / Định-Tuệ" leading to KP (1) Three-Haves ("#Awareness, Dependent, #Prajna / Kiến Tánh, Duyên Khởi, Bát Nhã"), via (2) observable X-dimension in both exoteric and esoteric realms from Figure 11.1 that forms with personally observable Inner Peace of Y-dimension from Gotama's recorded Jhanas on a No-Conflict plan at any focused event and (3) scientifically cultivable Prajna / Wisdom with discovered underlying natural laws of #PrajnaTIP from one's scientifically cultivable InnerSpace F-dimension for detoxification of tainted senses. Introducing the #Awareness of a persona F-dimension with Right evolution or Wrong potential degeneration of complex Fibonacci sequence - testable with KP KienTanh / HuiNeng WuNien / Gotama's InnerPeace / Bodhidharma's #EmptyTheContent - together with Lucas Numbers applicable to innate Intellligence of Illuminati / Communist elites, we theoretically and empirically prove that the Right evolution must come through (type) EquanimityAwareness f+1 = 1 enhanced and materialized in Equanimity Communities where Nature and Science toward What Count join hand. It is then enforced with (type) PurityAwareness f+2 = 1 for the possibility of Transcending the Observed and Observer to see Thing-As-It-Is. We ride on human open-source Rust standardization of #SmartPointer and #Trait to value add required functionalities of proven fact-based tracked records of #Truths such as #FourFoldTruth, #GodKingdom, #EmptyTheContent, #KindnessEmpathy, #Prajna, etc. The value-adds make the core distinction beteen "implementtion" which is KP custom suggestion and the persona final rating at the bottom line of its outcome and the "has" which is its innate quality, stable for the higher realm evolution according to underlying natural laws to be DSSCOVER & SHARED.
*/
#[derive(Debug)]
pub struct InnerSpace {
  // observable traits from attached to balanced to detached in one's InnerSpace
  pub x_traits: i32, // HashMap<i32, String>,  perceive event via measurable Intuition
  pub y_pointers: i32, // HashMap<i32, String>,  perceive event via measurable Sanadhi
  
  pub f_dimension: i32,   // lumped other factors observable in complex Fibonacci sequences 
  pub y_dimension: i32,   // KP Signed Posts or Gotama Jhanas or Right Samadhi  
  pub x_dimension: i32,   // HuiNeng #WuNien or KP #Awareness or Gotama Mindfulness
}
pub mod y_pointers;  // implementations to make the description as closed to the described
pub mod ydimension;

pub mod x_traits;  // implementations to make the description as closed to the described
pub mod xdimension;

pub mod fdimension;

impl InnerSpace { // we enable evaluation from LLM of the mass and from custom AGI
/*
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
  fn nation_happiness(&self) -> String { // f(_x,_y) for a nation
    return "in_operation".to_string()
  }
*/
}

use std::collections::HashMap;

pub fn build_inner_space(_x: i32, _y: i32, _f: i32) -> InnerSpace  {
  let mut _i = 0;  // qualified persona at balanced traits
  let mut _s = 0;  // qualified new Era persona at Awareness-Prajna
  
  let mut _y = 0;  // to be evaluated in blockchain of user self-evaluation and expert
  let mut _x = 0;  // opinions from fact-base tracked records in engaged living.
  let mut _f = 0;  // qualified person

  let mut x = HashMap::new(); // Taxonomy of x_dimension
  x.insert(0, String::from("X 0: Awareness"));             // =  0
  
  // x(-1) = 1 = x(1) Culture & HonNhien enforced @ x(2)=1 of proper management in Equanimity
  x.insert(-1, String::from("X-1: CulturalInfluenced"));   // =  1
  x.insert(-2, String::from("X-2: RegionalInfluenced"));   // = -1
  x.insert(-3, String::from("X-3: NationalInfluenced"));   // =  2
  x.insert(-4, String::from("X-4: Veiled"));               // = -3
  x.insert(-5, String::from("X-5: BindingWord"));          // =  5
  x.insert(-6, String::from("X-6: BindingImage"));         // = -8
  x.insert(-7, String::from("X-7: ClingingThought"));      // = 13
  
  x.insert(1, String::from("X+1: HonNhien"));              // = 1
  x.insert(2, String::from("X+2: ManagingFreshness"));     // = 1
  x.insert(3, String::from("X+3: KnowingFreshness"));      // = 2
  x.insert(4, String::from("X+4: ProcessDiscovered"));     // = 3
  x.insert(5, String::from("X+5: KnowingThought"));        // = 5      breakout = 5
  x.insert(6, String::from("X+6: CareViaCosmicEnergy"));   // = 8
  x.insert(7, String::from("X+7: HelpViaCosmicEnergy"));   // = 13

  let mut y = HashMap::new(); // Taxonomy of y_dimension
  y.insert(0, String::from("Y 0: Peace"));         // =  0
  
  // y(-1) = 1 = y(1) Empathy to Tranquility enforced in y(2) = 1 of Equanimity
  y.insert(-1, String::from("Y-1: Empathy"));      // =  1
  y.insert(-2, String::from("Y-2: Kindness"));     // = -1
  y.insert(-3, String::from("Y-3: Conscience"));   // =  2
  y.insert(-4, String::from("Y-4: Conscience-1")); // = -3
  y.insert(-5, String::from("Y-5: Conscience-2")); // =  5
  y.insert(-6, String::from("Y-6: Conscience-3")); // = -8
  
  y.insert(1, String::from("Y+1: Tranquility"));   // = 1
  y.insert(2, String::from("Y+2: Equanimity"));    // = 1
  y.insert(3, String::from("Y+3: Purity"));        // = 2
  y.insert(4, String::from("Y+4: Selfless"));      // = 3
  y.insert(5, String::from("Y+5: NonThingness"));  // = 5 Gotama's impass
  y.insert(6, String::from("Y+6: Unmoving"));      // = 8 Gotama's impass
  
  // SmartPointers y_traints starting from ability to #EmptyTheContent for detoxification 
  let mut s = HashMap::new(); // SmartPointers (s) toward SamadhiPrajna
  s.insert(1, String::from("S1: EmptyTheContent"));         // = 1
  s.insert(2, String::from("S2: DhyanaSamadhi"));           // = 2
  s.insert(3, String::from("S3: Samadhi"));                 // = 3 #Smadhi Meditation
  s.insert(4, String::from("S4: Awareness"));               // = 4 #Awareness KienTanh
  s.insert(5, String::from("S5: Prajna"));                  // = 5 #Prajna
  s.insert(6, String::from("S6: AwarenessPrajna"));         // = 6 #AwarenessPrajna
  s.insert(7, String::from("S7: SanadhiPrajna"));           // = 7 #SamadhiPrajna
  s.insert(8, String::from("S8: PrajnaTIP1"));              // = 8 #PrajnaTIP1
  s.insert(9, String::from("S9: PrajnaTIP2"));              // = 9 #PrajnaTIP2

  // x_traits driven by Intuition (i) 
  let mut i = HashMap::new(); // intuition from x-dimension via SmartPointers of Peace
  i.insert(0, String::from("I 0: Balanced"));           // =  0, qualified person
  
  // the x_traits vary from neutral balance to degrees if attached and/or detached
  // balanced via KindnessEmpathy from binding/clinging forces to total detachment
  i.insert(-1, String::from("I-1: KindnessEmpathy"));        // = -1
  i.insert(-2, String::from("I-2: Influenced"));             // = -2
  i.insert(-3, String::from("I-3: Veiled"));                 // = -3
  i.insert(-4, String::from("I-4: Indoctrinated"));          // = -4
  
  i.insert(1, String::from("I+1: Intuition"));               // = 1
  i.insert(2, String::from("I+2: Care"));                    // = 2
  i.insert(3, String::from("I+3: Honesty"));                 // = 3
  i.insert(4, String::from("I+4: Truth"));                   // = 4

  // => observable X-dimension from attachments to detachments, manifested in cosmic energy
  
  let mut f = HashMap::new(); // Taxonomy (types) of f_dimension
  f.insert(0, String::from("F 0: Persona")); //  Person sub f as the key
  
  // f(-1) = 1 = f(1) Empathy Awareness to be qualified as humanitas upward, enforced f(2) = 1
  f.insert(-1, String::from("F-1: EmpathyAwareness"));         // =  1
  f.insert(-2, String::from("F-2: KindnessAwareness"));        // = -1
  f.insert(-3, String::from("F-3: AnimalEnergy"));             // =  2
  f.insert(-4, String::from("F-4: ExtremeDesire"));            // = -3
  f.insert(-5, String::from("F-5: AnimalConsciousness"));      // =  5 
  // ---------
  f.insert(-6, String::from("F-6: Smelly-1"));                 // = -8    in spirits
  f.insert(-7, String::from("F-7: Smelly-2"));                 // =  13   vampire 
  f.insert(-8, String::from("F-8: Smelly-3"));                 // = -34   natural laws
  
  f.insert(1, String::from("F+1: EquanimityAwareness"));       // = 1
  f.insert(2, String::from("F+2: PurityAwareness"));           // = 1
  f.insert(3, String::from("F+3: SignedPosts"));               // = 2
  f.insert(4, String::from("F+4: SelflessAwareness"));         // = 3
  f.insert(5, String::from("F+5: VisibleAwarenessPrajna"));    // = 5
  f.insert(6, String::from("F+6: EngagedAwarenessPrajna"));    // = 8
  f.insert(7, String::from("F+7: ForecastingSimulation"));     // = 13
  f.insert(8, String::from("F+8: SamadhiPrajna"));             // = 21
  
    
  // dynamic between one's avaluation and the KP's prediction
  _i = i_evaluation(i); // visible x_traits based on the outcomes in engaged living 
  _s = s_evaluation(s); // visible y_pointers based on deeper InnerPeace of Samadhi
  
  _y = y_evaluation(y); // transcendental Inner Peace based on the outcomes in engaged living
  _x = x_evaluation(x); // transcendental Awareness based on self evaluations and assessments
  _f = f_evaluation(f);
  
  // evaluation of InnerSpace
  let me = InnerSpace {
    x_traits: _i,
    y_pointers: _s,
    
    y_dimension: _y,
    x_dimension: _x,
    f_dimension: _f, 
  };

  return me;

} // end of build_InnerSpace from Intuition traits i, SignedPosts y, Awareness x, InnerSpace f

/*
  s_evaluation return evaluated value of one's y_pointer type of SmartPointer InnerPeace from
  #EmptyTheContent toward SanadhiPrajna and its reversed engineering for people connected to
  learn and share in the hashtag community. We can use the generic evaluation type to be 
  implemented for identified pointer which can question from LLM models then fine-tuned from 
  KpPlatform community data and the custom data of the client.
*/
fn s_evaluation(s: HashMap<i32, String>) -> i32 { // return evaluated value to the blockchain

   println!("Valid Meditation pointers"); // all claimed meditations must meet DhyanaSamadhi
   for (key, value) in &s {
        println!("{key}: {value}");
    }
    // evaluate the transient value of s in circular processes of 7 or 8 hashtag and record
    // it somewhere for tracking records in different cases of claimed #
    
    return 0; // rated level of Samadhi
}

/*
  i_evaluation return evaluated value of one's trait (type) ranging from attachment to
  detachment to be explored in one's transcendental #Awareness and #Samadhi which are tags
  for people connected to learn and share in the hashtag community. We can use the generic
  evaluation type to be implemented for identified trait which can question from LLM models
  then fine-tuned for KpPlatform community data and the custom data of the client.
*/
fn i_evaluation(i: HashMap<i32, String>) -> i32 { // return evaluated value to the blockchain

   println!("Valid Intuition traits");
   for (key, value) in &i {
        println!("{key}: {value}");
    }
    // evaluate the transient value of i in circular processes of 8 hashtag and record it
    // somewhere for tracking records in different cased of claimed #
    
    return 0; // rated level of Intuition
}


/* 
  List of traits ranging from Detachments to practical outcomes Truth. This part exposes the
  cheated Descriptions such as Faith from all religions, [ Honesty, Budh, Patience / Chân, 
  Thiện, Nhẩn ] from Falun Dafa and all forms of Buddhist meditations or reflection (quán
  chiếu) for testing and discovering the underlying natural laws of their practices to be
  statistically evalutated by the world. For example, we will prove that "Faith" does not need
  to be on the conflicting plan of duality due to binding word/image and clinging thought 
  leading to many religious crusades, and can be used as a mean to ride on one's past binding 
  forces as a form of Aspiration in transcending the "Known" to reach the Y-dimension recorded 
  by Gotama as Right Meditation. The proof is at both theoretical level using math and at 
  empirical levels as SHARED by different faith practitioners How to use "Faith / Trust"
  Aspiration in transcending the Known for detoxifications of the tainted senses, realizing 
  Kingdom of gods within. The "Hoax of Jesus Redeemer" will be naturally exposed to 
  scientifically point out the Right way of evolution versus the Wrong way of degeneration as 
  evidenced in the total collapse of the past esoteric feudal systems ruled by the degenerated
  King og Gods. This can be done with or without the revelation of the real Jesus in his
  Continuity of the consciousness and major Lesson Learned amongst Intelligent Beings.
  
  Similarly, the practices of Falun Dafa and all forms of Buddhist meditations are contrasted
  with claimed "Transcendental Meditation", Chinese ""Qi", KhaiPhong's discovered
  underlying natural laws of Verifiable Processes from PrajnaTIP in Sound, and other 
  transcendental technologies via Arts (singing, dancing, playing music, bonsai, merging with 
  nature, etc) to wipe out cheaters "cooking sand and selling as rice".
*/
pub enum Xtraits {
  Truth(String), 
  Care(String),
  // Falun gong Honesty - Budh - Patience /  Chân - Thiện - Nhẩn
  Falun(String), 
  // Intuition Trust, Qi, Art
  Intuition(String),
  
  Balanced(String), // qualified person
  
  KindnessEmpathy(String),  
  Influenced(String),
  Veiled(String),
  // indovtrinated visible in BindingWord, BindingImage, ClingingThought
  Indoctrinated(String),
}

pub enum Ypointers {
  EmptyTheContent(String), 
  DhyanaSamadhi(String),
  // Mediatation or Samadhi must produce visible outcomes known by Gotama
  Meditation(String), 
  // Vietnamese KienTanh or Chinese WuNien
  KienTanh(String),
  AwarenessPrajna(String), // related to karma
  SamadhiPrajna(String),            // known by HuiNeng
  PrajnaTIP(String),  
}

/*
 Trait definitions are a way to group method signatures together to define a set of behaviors
 necessary to accomplish some purpose. By defining hashtag x_trais varied from indoctrinated
 to the Truth in ChangeManagement toward SmartPointers to absolute Silence, we force each type
 (claimed method) implementing the trait such as Falun gong to provide its custom and 
 measurable processes in the method's body to be verified by the community and AI.
 Knowledge graph is a study the Described focused event and best possible outcomes honestly 
 evaluated by the persona of personal learned lessons relevant to the norm of the mass.
*/
// x_traits
pub mod truth;  
pub mod care;  
pub mod falun;  
pub mod intuition; 
pub mod balanced;
pub mod kindness_empathy;
pub mod influenced;
pub mod veiled;  
pub mod indoctrinated;   

// y_pointers or SmartPointers
pub mod empty_the_content;  
pub mod dhyana_samadhi;  
pub mod meditation;  
pub mod kien_tanh; 
pub mod awareness_prajna;
pub mod samadhi_prajna;
pub mod prajna_tip;     

/*
  y_evaluation and x_evaluation return evaluated value of one's Y and X types to be further
  explored in these two dimensions that are inputs into the f_evaluation. We can make the
  function fn evaluation<T>(HashMap<i32, String>) -> &[T] {} over type T. The evaluation will
  return a reference to a value of the same type T which is the rated Fibonacci value of the
  T dimension.
*/
fn y_evaluation(y: HashMap<i32, String>) -> i32 { // return evaluated value to the blockchain

   println!("Valid SignedPost / TranscendentalInnerPeace");
   for (key, value) in &y {
        println!("{key}: {value}");
    }
    
    // model y Inner Peace based on one's observable pointers in circular processes of 9 
    //hashtags and record it somewhere for tracking records in different cased of claimed #
    
    return 0; // rated level of SignedPost
}
fn x_evaluation(x: HashMap<i32, String>) -> i32 { // return evaluated value to the blockchain
   println!("Valid WuNien / Awareness");
   for (key, value) in &x {
        println!("{key}: {value}");
    }   
    
    // model x Perception based on one's observable traits in circular processes of 9 
    //hashtags and record it somewhere for tracking records in different cased of claimed #
    
    return 0; // rated level of WuNien
    
}
fn f_evaluation(f: HashMap<i32, String>) -> i32 {

    println!("Suggested Inner Space for cultivation");
       for (key, value) in &f {
        println!("{key}: {value}");
    }
    
    // evaluations based on hard evidences in community social networks and proven experts
    
    return 0; // rated complex Fibinacci level
}

