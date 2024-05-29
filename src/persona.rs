/*
 F-dimention of one's scientifically cultivable InnerSpace - expressed in the form of complex
 Fibonacci sequences - forms with Y-dimension of Transcendental Inner Peace known by Gotama
 for the persona's Activities and Relationships in the direction of Right or Wrong toward
 What Count, and X-dimension of one's perceptions of outside events in Places (Figure 11.1).
 One can evaluate at the bottom line of the event outcome to normatively and honestly rate
 oneself as Prajna/Ignorance for possible ChangeManagement of the traits called Intuition,
 contributing and causing one's final decision. The dynamic feed back and feed forward effects
 of InnerSpace and changing outer event can be statistically modeled via different agents 
 responsible and custom made for the each trait for eventually overall simple rating to
 understand the underlying natural laws leading to expected outcomes.
 
 The function build_inner_space initially and periodically rate the persona for user custom 
 services. The ranges for evaluation of i (Intuition), y, x are updated according to academia 
 researches for predicted outcome of Prajna or Ignorance in ChangeManagement. Same thing is 
 applicable to the modeling and rating the function outcomes: _y, _x, _f making up a new "me".
 
 The implementation from self to selfless is via _x Detachment, _y InnerPeace / RightWrong,
 and _f positive InnerSpace observable in the manifestations of complex Fibonacci sequence. 
 Evaluation of x_dimension is trained via empirical observations and contributions by those 
 connected to [ WuNien / #Awareness ]. Evaluation of y_dimension is trained via empirical 
 observations and contributions by those connected to #Samadhi of Transcendental Inner Peace
 known by Gotama and verifiable by practitioners in different claimed meditations and 
 reflections to weed out garbages blah blah blah by past claimed masters and religious 
 leaders. The observable point on X-Y plane is connected to stable f_evaluation for custom
 trainings and treatments.
*/
#[derive(Debug)]
pub struct InnerSpace {
  // observable traits from attached to neutral balance to detached in one's InnerSpace
  pub traits: i32,   // to perceive event via Intuition
  pub f_dimension: i32,   // lumped other factors observable in complex Fibonacci sequences 
  pub y_dimension: i32,   // KP Signed Posts or Gotama Jhanas or Right Samadhi  
  pub x_dimension: i32,   // HuiNeng #WuNien or KP #Awareness or Gotama Mindfulness
}
pub mod traits;     // implementations to make the description as closed to the described
pub mod ydimension;
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
  let mut _y = 0;  // to be evaluated in blockchain of user self-evaluation and expert
  let mut _x = 0;  // opinions from fact-base tracked records in engaged living.
  let mut _f = 0;  // qualified person

  let mut i = HashMap::new(); // Taxonomy of intuition at different depths in InnerSpace
  i.insert(0, String::from("I 0: Balanced"));           // =  0
  
  // the trait qualities vary from neutral balance to degrees if attached and/or detached
  // balanced at 1 word from binding/clinging forces to total detachment
  i.insert(-1, String::from("I-1: Influenced"));        // = -1
  i.insert(-2, String::from("I-2: Veiled"));            // = -2
  i.insert(-3, String::from("I-3: BindingWord"));       // = -3
  i.insert(-4, String::from("I-4: BindingImage"));      // = -4
  i.insert(-5, String::from("I-5: ClingingThought"));   // = -5
  
  i.insert(1, String::from("I+1: KindnessEmpathy"));    // = 1
  i.insert(2, String::from("I+2: TrustQiArt"));         // = 2 [ OutsideTheBox ]
  i.insert(3, String::from("I+3: HonestyBudhPatience"));// = 3
  i.insert(4, String::from("I+4: Care"));               // = 4 [ Enganged Living ]
  i.insert(5, String::from("I+5: Truth"));              // = 5

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
  
  let mut x = HashMap::new(); // Taxonomy of x_dimension
  x.insert(0, String::from("X 0: Awareness"));             // =  0
  
  // x(-1) = 1 = x(1) Culture & HonNhien enforced @ x(2)=1 of proper management in Equanimity
  x.insert(-1, String::from("X-1: CulturalInfluenced"));   // =  1
  x.insert(-2, String::from("X-2: RegionalInfluenced"));   // = -1
  x.insert(-3, String::from("X-3: NationalInfluenced"));   // =  2
  x.insert(-4, String::from("X-4: Veiled"));               // = -3
  x.insert(-5, String::from("X-5: BindingWord"));          // =  5
  x.insert(-6, String::from("X-6: BindingImage"));         // = -8
  x.insert(-7, String::from("X-7: ClingingThought"));      // = 13     vampire 
  x.insert(1, String::from("X+1: HonNhien"));              // = 1
  x.insert(2, String::from("X+2: ManagingFreshness"));     // = 1
  x.insert(3, String::from("X+3: KnowingFreshness"));      // = 2
  x.insert(4, String::from("X+4: ProcessDiscovered"));     // = 3
  x.insert(5, String::from("X+5: KnowingThought"));        // = 5      breakout = 5
  x.insert(6, String::from("X+6: CareViaCosmicEnergy"));   // = 8
  x.insert(7, String::from("X+7: HelpViaCosmicEnergy"));   // = 13
  
  let mut f = HashMap::new(); // Taxonomy of f_dimension
  f.insert(0, String::from("F 0")); //  Person sub f as the key
  
  // f(-1) = 1 = f(1) Empathy Awareness to be qualified as humanitas upward, enforced f(2) = 1
  f.insert(-1, String::from("F-1: EmpathyAwareness"));         // =  1
  f.insert(-2, String::from("F-2: KindnessAwareness"));        // = -1
  f.insert(-3, String::from("F-3: AnimalEnergy"));             // =  2
  f.insert(-4, String::from("F-4: ExtremeDesire"));            // = -3
  f.insert(-5, String::from("F-5: AnimalConsciousness"));      // =  5    Smelly-1
  f.insert(-6, String::from("F-6: Smelly-1"));                 // = -8    in spirits
  f.insert(-7, String::from("F-7: Smelly-2"));                 // =  13   vampire 
  f.insert(-8, String::from("F-8: Smelly-3"));                 // = -34   natural laws
  
  f.insert(1, String::from("F+1: EmpathyAwareness"));          // = 1
  f.insert(2, String::from("F+2: PurityAwareness"));           // = 1
  f.insert(3, String::from("F+3: SignedPosts"));               // = 2
  f.insert(4, String::from("F+4: SelflessAwareness"));         // = 3
  f.insert(5, String::from("F+5: VisibleAwarenessPrajna"));    // = 5
  f.insert(6, String::from("F+6: EngagedAwarenessPrajna"));    // = 8
  f.insert(7, String::from("F+7: ForecastingSimulation"));     // = 13
  f.insert(8, String::from("F+8: SamadhiPrajna"));             // = 21
  
    
  // dynamic between one's avaluation and the KP's prediction
  _i = i_evaluation(i); // visible traits based on the outcomes of engaged living 
  _y = y_evaluation(y); // transcendental Inner Peace based on the outcomes in engaged living
  _x = x_evaluation(x); // transcendental Awareness based on self evaluations and assessments
  _f = f_evaluation(f);
  
  // evaluation of InnerSpace
  let me = InnerSpace {
    traits: _i,
    y_dimension: _y,
    x_dimension: _x,
    f_dimension: _f, 
  };

  return me;

} // end of build_InnerSpace from Intuition traits i, SignedPosts y, Awareness x, InnerSpace f

/*
  i_evaluation return evaluated value of one's trait type ranging from attachment to
  detachment to be explored in one's transcendental #Awareness and #Samadhi which are tags
  for people connected to learn and share in the hashtag community. We can use the generic
  evaluation type to be implemented for identified trait which can question from LLM models
  then fine-tuned for KP platform community data and the custom data of the client.
  
*/
fn i_evaluation(i: HashMap<i32, String>) -> i32 { // return evaluated value to the blockchain

   println!("Valid Intuition / Traits");
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
  empirical levels as SHARED by different faith practitioners How to use "Faith" Aspiration in 
  transcending the Known for detoxifications of the tainted senses, realizing Kingdom of gods
  within. The "Hoax of Jesus Redeemer" will be naturally exposed to scientifically point out
  the Right way of evolution versus the Wrong way of degeneration as evidenced in the total
  collapse of the past esoteric feudal systems ruled by the degenerated King og Gods.
  
  Similarly, the practices of Falun Dafa and all forms of Buddhist meditations are contrasted
  with claimed "Transcendental Meditation", Chinese ""Qi", KhaiPhong's discovered
  underlying natural laws of Verifiable Processes from PrajnaTIP in Sound 
  (https://blog.khaiphong.io/2023/09/nature-of-things.html#Section_2.1), and other 
  transcendental technologies via Arts (singing, dancing, playing music, bonsai, merging with 
  nature, etc) to wipe out cheaters "cooking sand and sell as rice".
*/
pub enum Traits {
  Truth(String), 
  Care(String), 
  Honesty(String), // "Honesty / TRUTH / Chân"
  Budh(String),    // "Budh / Morality / Thiện"
  Patience(String), // "Patience / Nhẩn"
  Trust(String),
  Qi(String),
  Art(String),
  Empathy(String), 
  Kindness(String), 
  
  Balanced(String),
  
  Influenced(String),
  Veiled(String),
  BindingWord(String),
  BindingImage(String),
  ClingingThought(String)
}

// Knowledge graph to study the Described focused event and best possible outcomes honestly 
// evaluated by the persona of personal learned lessons relevant to the norm of the mass
pub mod truth;  
pub mod care;  
pub mod honesty;  
pub mod budh; 
pub mod patience;
pub mod trust;
pub mod qi;
pub mod art;
pub mod empathy;
pub mod kindness;

pub mod balanced;

pub mod influenced;
pub mod veiled;  
pub mod binding_word;  
pub mod binding_image;  
pub mod clinging_thought; 

/*
 Trait definitions are a way to group method signatures together to define a set of behaviors
 necessary to accomplish some purpose. By defining Intuition, Meditation, Awareness and
 ChangeManagement, we force each type (claimed method) implementing the trait to provide its
 own custom behavior in the method's body to be verified by the community at.
*/
pub trait Intuition { 
  fn perceive(&self) -> i32; // ranging from -5 to 5
}
pub trait Meditation {       // ability to be on x-y plan, ranging from -6 to +6
  fn meditate(&self) -> i32;
}
pub trait Awareness {        // ability to be on x-y plan, ranging from -7 to +7 
  fn aware(&self) -> i32;
}
pub trait ChangeManagement { //self avaluated and lesson learned of a formal project to
 fn evaluate(&self) -> String; // eip as a prompt to LLM
}

impl Intuition for InnerSpace {
  fn perceive(&self) -> i32 {
    // concrete model of InnerSpace Intuition level
    return 0;
  }
}

impl Meditation for InnerSpace {
  fn meditate(&self) -> i32 {
    // concrete model of InnerSpace Meditation according to different traditions
    return 0;
  }
}

impl Awareness for InnerSpace { // for ferifications of HuiNeng's WuNien
  fn aware(&self) -> i32 {
    // concrete model of InnerSpace Intuition level
    return 0;
  }
}

impl ChangeManagement for InnerSpace { // a feed back and feed forward stream to LLM ML
 fn evaluate(&self) -> String {
    // concrete model of InnerSpace ChangeManagement
    return String::from("(Prompt...)");
 }
}

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
    
    // model y Inner Peace based on one's observable traits in circular processes of 9 
    //hashtags and record it somewhere for tracking records in different cased of claimed #
    
    return 0; // rated level of SignedPost
}
fn x_evaluation(x: HashMap<i32, String>) -> i32 { // return evaluated value to the blockchain
   println!("Valid WuNien / Awareness / TranscendentalAwareness");
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

