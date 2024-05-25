/*
 F-dimention forms with Y-dimension for the persona's Activities and Realtionships and
 X-dimension for Places. Prajna/Ignorance is evaluated at the event's outcome in 
 ChangeManagement to expose the trait of one's Intuition. The function build_InnerSpace
 initially and periodically rate the persona for user custom services. The 
 ranges for evaluation of y, x, i (Intuition) are updated according to academia researches for
 predicted outcome of Prajna or Ignorance in ChangeManagement. Same thing is applicable to
 their rating the function outcomes: _y, _x, _f. The return is a new "me".
*/
pub trait Intuition { 
  fn perceive(&self) -> i32; // ranging from -5 to 5
}
pub trait Sanadhi {       // ability to be on x-y plan, ranging from -6 to +6
  fn meditate(&self) -> i32;
}
pub trait Awareness {     // ability to be on x-y plan, ranging from -7 to +7 
  fn aware(&self) -> i32;
}
pub trait ChangeManagement { //self avaluated and lesson learned of a formal project to
 fn evaluate(&self) -> String; // eip as a prompt to LLM
}

/*
 The implementation from self to selfless via _x Detachment, _y InnerPeace / RightWrong,
 and _f positive InnerSpace. Evaluation of x_dimension is trained via empirical observations 
 and contributions by those connected to [ #WuNien / #Awareness ]. Evaluation of y_dimension 
 is trained via empirical observations and contributions by those connected to #Samadhi. The 
 observable point on X-Y plane is connected to stable f_evaluation for custom trainings and 
 treatments.
*/
#[derive(Debug)]
pub struct InnerSpace {
  // observable traits from attached to neutral balance to detached in one's InnerSpace
  pub traits: i32,   // to perceive event via Intuition
  pub f_dimension: i32,   // lumped other factors observable in complex Fibonacci sequences 
  pub y_dimension: i32,   // KP Signed Posts or Gotama Jhanas or Right Samadhi  
  pub x_dimension: i32,   // HuiNeng #WuNien or KP #Awareness or Gotama Mindfulness
}
pub mod traits;
pub mod fdimension;
pub mod ydimension;
pub mod xdimension;

impl InnerSpace { // we enable evaluation from LLM of the mass and from custom AGI
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
}

use std::collections::HashMap;

pub fn build_InnerSpace(_x: i32, _y: i32, _f: i32) -> InnerSpace  {
  let mut _i = 0;  // qualified persona at balanced traits
  let mut _y = 0;  // to be evaluated in blockchain of user self-evaluation and expert
  let mut _x = 0;  // opinions from fact-base tracked records in engaged living.
  let mut _f = 0;  // qualified person

  let mut i = HashMap::new(); // Taxonomy of intuition at different depths in InnerSpace
  i.insert(0, String::from("I 0")); // neutral balance
  
  // the trait qualities vary from neutral balance to degrees if attached and/or detached
  // balanced at 1 word from binding/clinging forces to total detachment
  i.insert(-1, String::from("I-1"));   // trait of being influenced = -1
  i.insert(-2, String::from("I-2"));   // trait of veiled Right and Wrong = -2
  i.insert(-3, String::from("I-3"));   // trait of binding Word = -3
  i.insert(-4, String::from("I-4"));   // trait of binding Image = -4
  i.insert(-5, String::from("I-5"));   // trait of binding Thought = -5
  
  i.insert(1, String::from("I+1"));    // trait of [ Kindness, Empathy ] = 1
  i.insert(2, String::from("I+2"));    // trait of [ Trust, Qi, Art ] for OutsideTheBox = 2
  i.insert(3, String::from("I+3"));    // trait of [ Honesty, Budh, Patience ] = 3
  i.insert(4, String::from("I+4"));    // trait of [ Care / Enganged Living ] = 4
  i.insert(5, String::from("I+5"));    // trait of [ TRUTH ] = 5

  let mut y = HashMap::new(); // Taxonomy of y_dimension
  y.insert(0, String::from("Y 0")); // Peace sub f as the key
  
  // y(-1) = 1 = y(1) Empathy to Tranquility enforced in y(2) = 1 of Equanimity
  y.insert(-1, String::from("Y-1"));   // Empathy = 1
  y.insert(-2, String::from("Y-2"));   // Kindness = -1
  y.insert(-3, String::from("Y-3"));   // Conscience = 2
  y.insert(-4, String::from("Y-4"));   // Conscience-1 = -3
  y.insert(-5, String::from("Y-5"));   // Conscience-2 = 5
  y.insert(-6, String::from("Y-6"));   // Conscience-3 = -8
  
  y.insert(1, String::from("Y+1"));    // Tranquillity = 1
  y.insert(2, String::from("Y+2"));    // Equanimity targeted at community = 1
  y.insert(3, String::from("Y+3"));    // Purity = 2
  y.insert(4, String::from("Y+4"));    // Not-Self = 3
  y.insert(5, String::from("Y+5"));    // Nothingness of Gotama's impass = 5
  y.insert(6, String::from("Y+6"));    // Unmoving of Gotama's impass = 8
  
  let mut x = HashMap::new(); // Taxonomy of x_dimension
  x.insert(0, String::from("X 0")); // Awareness sub f as the key
  
  // x(-1) = 1 = x(1) Culture & HonNhien enforced @ x(2)=1 of proper management in Equanimity
  x.insert(-1, String::from("X-1"));  // Cultural Influence = 1
  x.insert(-2, String::from("X-2"));  // Regional Influence = -1
  x.insert(-3, String::from("X-3"));  // National Influence = 2
  x.insert(-4, String::from("X-4"));  // Veiled Right and Wrong = -3
  x.insert(-5, String::from("X-5"));  // Binding Word = 5
  x.insert(-6, String::from("X-6"));  // Binding Image = -8
  x.insert(-7, String::from("X-7"));  // Clinging Thought = 13 disappearing of dark/vampire 
  x.insert(1, String::from("X+1"));   // HonNhien = 1
  x.insert(2, String::from("X+2"));   // Managing that Freshness, target community = 1
  x.insert(3, String::from("X+3"));   // Managing that Freshness = 2
  x.insert(4, String::from("X+4"));   // Discovering a process to the Freshness = 3
  x.insert(5, String::from("X+5"));   // Knowing one's Thought breakout = 5
  x.insert(6, String::from("X+6"));   // Using cosmic energy = 8 in Care
  x.insert(7, String::from("X+7"));   // Directing cosmic energy to help others = 13
  
  let mut f = HashMap::new(); // Taxonomy of f_dimension
  f.insert(0, String::from("F 0")); //  Person sub f as the key
  
  // f(-1) = 1 = f(1) Empathy Awareness to be qualified as humanitas upward, enforced f(2) = 1
  f.insert(-1, String::from("F-1")); // Empathy Awareness = 1
  f.insert(-2, String::from("F-2")); // Kindness Awareness = -1
  f.insert(-3, String::from("F-3")); // Animal Energy = 2
  f.insert(-4, String::from("F-4")); // Extreme Desire = -3
  f.insert(-5, String::from("F-5")); // Smelly 1  = 5 breaking of animal consciousness
  f.insert(-6, String::from("F-6")); // Smelly 2 = -8 only observable in spirits
  f.insert(-7, String::from("F-7")); // Smelly 3 = 13 battle of dark/vampire and good forces
  f.insert(-8, String::from("F-8")); // Smelly 4 = -34 regulated by natural laws
  
  f.insert(1, String::from("F+1"));  // Empathy Awareness = 1
  f.insert(2, String::from("F+2"));  // Purity Awareness targeted community = 1
  f.insert(3, String::from("F+3"));  // Samadhi Signed Posts = 2
  f.insert(4, String::from("F+4"));  // Selfless Awareness = 3
  f.insert(5, String::from("F+5"));  // Visible Awareness-Prajna = 5
  f.insert(6, String::from("F+6"));  // Awareness-Prajna in engaged Living innovations = 8
  f.insert(7, String::from("F+7"));  // Visible Forecasting and Simulation quantum = 13
  f.insert(8, String::from("F+8"));  // approaching Samadhi-Prajna = 21
  
    
  // dynamic between one's avaluation and the KP's prediction
  _i = i_evaluation(i); // vector of traits based on the outcomes of engaged living
  _f = f_evaluation(f); 
  _y = y_evaluation(y); // transcendental Inner Peace based on the outcomes in engaged living
  _x = x_evaluation(x); // transcendental Awareness based on self evaluations and assessments
  
  // evaluation of InnerSpace
  let me = InnerSpace {
    traits: _i,
    y_dimension: _y,
    x_dimension: _x,
    f_dimension: _f, 
  };

  return me;

} // end of build_InnerSpace from Intuition traits i, SignedPosts y, Awareness x, InnerSpace f

fn i_evaluation(i: HashMap<i32, String>) -> i32 { // return evaluated value to the blockchain

   println!("Valid Intuition / Traits");
   for (key, value) in &i {
        println!("{key}: {value}");
    }
    // evaluate the transient value of i in circular processes of 8 hashtag and record it
    // somewhere for tracking records in different cased of claimed #
    
    return 0; // rated level of Intuition
}

fn y_evaluation(y: HashMap<i32, String>) -> i32 { // return evaluated value to the blockchain

   println!("Valid SignedPost / TranscendentalInnerPeace");
   for (key, value) in &y {
        println!("{key}: {value}");
    }
    // evaluate the transient value of y in circular processes of 8 hashtag and record it
    // somewhere for tracking records in different cased of claimed #
    
    return 0; // rated level of SignedPost
}
fn x_evaluation(x: HashMap<i32, String>) -> i32 { // return evaluated value to the blockchain
   println!("Valid WuNien / Awareness / TranscendentalAwareness");
   for (key, value) in &x {
        println!("{key}: {value}");
    }    
   // evaluate the transient value of x in circular processes of 8 hashtag and record it
   // somewhere for tracking record in different cased of claimed #   
    return 0; // rated level of WuNien
}
fn f_evaluation(f: HashMap<i32, String>) -> i32 {

    println!("Suggested Inner Space for cultivation");
       for (key, value) in &f {
        println!("{key}: {value}");
    }
    return 0; // rated complex Fibinacci level
}

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
}

// Knowledge graph to study the Described focused event and best possible outcomes honestly 
// evaluated by the persona of personal lessons learned relevant to the norm of the mass
pub mod truth;  

pub mod care;  

pub mod honesty;  
pub mod budh; 
pub mod patience;

pub mod trust;
pub mod qi;
pub mod art;


 /*
#[derive(Debug)]
enum Option<T> { // is generic over type T
  Some(T)        // holds once value of type T
  None,          // doesn't hold any value
}
enum Result<T, E> { // for types x, y
  Ok(T),            // for open a block chain
  Err(E),           // and its error
} 
 

#[derive(Debug)]
pub struct InnerSpace<T> { // we can do it in enum Option
  x: T, y: T, f: T,
}

// let implement f as a complex variable
pub struct Fibonacci<i> {
}

/* Given an event, we measure one's perception indirectly via its outcome. It is generally
  recognized t, q, a (trust, qi, art) raises the perception level to OutsideTheBox 2, h, b, p
  (honesty, budh, patience) deepen the perception as introvert 3, c (care) to the Right/Wrong
  and act on it to the level 4, and front-line soldier of the TRUTH @ level 5  */
pub trait Intuition {
  fn perceive(&self) -> i32; 
}

impl<i32> Quality<i32> { // rated qualities from -5 to +5
  fn x(&self> -> &i32 {
    &self.x
  }

  fn h_evaluation(&self) -> i32 { // h measures the invisible part of one's Qualities
    return 1; // LLM from mass data
  }
  fn c_evaluation(&self) -> i32 { // c indicates one's seriousness and commitments
    return 3; // LLM from mass data
  }
  fn b_evaluation(&self) -> i32 { // budh or morality at the bottom line of Right or Wrong
    return 3; // LLM from mass data
  }
  fn p_evaluation(&self) -> i32 { // patience qualified and verifiable in Khương Tử Nha
    return 3; // LLM from mass data
  }
  fn t_evaluation(&self) -> i32 { // t indicates one's trust or faith
    return 3; // LLM from mass data
  }    
  fn q_evaluation(&self) -> i32 { // q indicates one's wusu qi level 
    return 3; // LLM from mass data
  }
  fn a_evaluation(&self) -> i32 { // a indicates one's level of transcending art
    return 3; // LLM from mass data
  }
}

enum RatedMaturity{ // for organization
  Maturity,
}
enum StepOutcomes{ // for foreign affairs: 0 is ready on the pipelie rated at +1 to +5
  Steps,
}
enum VisionOutcomes{ // for country: 0 is ready on the pipelie rated at +1 to +5
  Visions,
}


impl<T> InnerSpace<T> { // X, Y, F is type T for LLM evaluation and from custom AGI
  fn x(&self) -> &T {
    &self.x
  }
}

  
  let mut _h = 0;    // Honesty / TRUTH / Chân
  let mut _c = 0;    // Care
  let mut _b = 0;    // Budh / Morality / Thiện / Right & Wrong 
  let mut _p = 0;    // Patience / Nhẩn
  let mut _t = 0;    // Trust / Faith to perfection 
  let mut _q = 0;    // Qi to perfection
  let mut _a = 0;    // arts to perfection


  let mut y = HashMap::new(); // Taxonomy of y_dimension
  y.insert(0, String::from("Y 0")); // Innate Peace sub f as the key
  

  let mut h = HashMap::new(); // Taxonomy of h_dimension from -5 to +5
  h.insert(0, String::from("H 0")); // Honesty / Chan
  let mut c = HashMap::new(); // Taxonomy of c_dimension from -5 to +5
  c.insert(0, String::from("C 0")); // Care from humanity upward
  let mut b = HashMap::new(); // Taxonomy of b_dimension
  b.insert(0, String::from("B 0")); // Budh Thien
  let mut p = HashMap::new(); // Taxonomy of p_dimension
  p.insert(0, String::from("P 0")); // Patience Nhan
  let mut t = HashMap::new(); // Taxonomy of t_dimension
  t.insert(0, String::from("T 0")); // Trust Faith sub t as the key
  let mut q = HashMap::new(); // Taxonomy of q_dimension
  q.insert(0, String::from("Q 0")); // Qi Khi
  let mut a = HashMap::new(); // Taxonomy of a_dimension
  a.insert(0, String::from("A 0")); // Art

  let _honesty = Qualities::Honesty(String::from("HonestyCases"));  // recorded proof
  let _care = Qualities::Care(String::from("CareCases")); // recorded proof
  let _budh = Qualities::Budh(String::from("BudhCases")); // recorded proof
  let _patience = Qualities::Patience(String::from("PatienceCases")); // recorded proof
  let _trust = Qualities::Trust(String::from("TrustCases"));  // recorded proof
  let _qi = Qualities::Qi(String::from("QiCases"));  // recorded proof
  let _art = Qualities::Art(String::from("ArtCases"));  // recorded proof  
  
  let mut v: Vec<i32> = Vec::new();
  
  _h = h_evaluation(h); // honesty based on the outcomes of engaged living
  _c = c_evaluation(c); // care based on the outcomes in engaged living
  _b = b_evaluation(b); // budh based on the outcomes in engaged living 
  _p = p_evaluation(p); // patience based on the outcomes in engaged living
  _t = t_evaluation(t); // trust based on the outcomes of engaged living
  _q = q_evaluation(q); // qi based on the outcomes of engaged living
  _a = a_evaluation(a); // art based on the outcomes of engaged living
   

Use generics for rating these evaluations applicable of _x, _y, qualities, then f_dimension
fn y_evaluation(y: HashMap<i32, String>) -> i32 { // return evaluated value to the blockchain

   println!("Attributes of Transcendental Inner Peace");
   for (key, value) in &y {
        println!("{key}: {value}");
    }
    // evaluate the transient value of y in circular processes of 8 hashtag and record it
    // somewhere for tracking records in different cased of claimed #
    
    return 0;
}
fn x_evaluation(x: HashMap<i32, String>) -> i32 { // return evaluated value to the blockchain
   println!("Attributes of Transcendental Awareness");
   for (key, value) in &x {
        println!("{key}: {value}");
    }    
   // evaluate the transient value of x in circular processes of 8 hashtag and record it
   // somewhere for tracking record in different cased of claimed #   
    return 0;
}
fn f_evaluation(f: HashMap<i32, String>) -> i32 {

    println!("Suggested Inner Space for cultivation");
       for (key, value) in &f {
        println!("{key}: {value}");
    }
    return 0;
}

/*
 Qualities are evaluated @ -5. -4. -3. -2. -1, 0, 1, 2, 3, 4, 5. Evaluation is based on
 personal subjective rating and community reported cases of the evaluations. They have a
 HashMap of psychological descriptions of the required properties. We introduce each as trait
*/
fn evaluation<T>((HashMap<i32, String>): &[T] -> &T) { // return evaluated value
   println!("Attributes of Honesty / Truth in one's Continuity of the consciousness");
   for (key, value) in &h {
        println!("{key}: {value}");
    }    
   // evaluate the transient value of h in circular processes of 8 hashtag and record it
   // somewhere for tracking record in different cased of claimed #   
    return T;
}
fn c_evaluation(c: HashMap<i32, String>) -> i32 { // return evaluated value to the blockchain
   println!("Attributes of Care for one and the environment");
   for (key, value) in &c {
        println!("{key}: {value}");
    }    
   // evaluate the transient value of c in circular processes of 8 hashtag and record it
   // somewhere for tracking record in different cased of claimed #   
    return 0;
}
fn b_evaluation(b: HashMap<i32, String>) -> i32 { // return evaluated value to the blockchain
   println!("Attributes of Budh sensing the Right from Wrong");
   for (key, value) in &b {
        println!("{key}: {value}");
    }    
   // evaluate the transient value of b in circular processes of 8 hashtag and record it
   // somewhere for tracking record in different cased of claimed #   
    return 0;
}
fn p_evaluation(p: HashMap<i32, String>) -> i32 { // return evaluated value to the blockchain
   println!("Attributes of Patience");
   for (key, value) in &p {
        println!("{key}: {value}");
    }    
   // evaluate the transient value of p in circular processes of 8 hashtag and record it
   // somewhere for tracking record in different cased of claimed #   
    return 0;
}
fn t_evaluation(t: HashMap<i32, String>) -> i32 { // return evaluated value to the blockchain
   println!("Attributes of Trust / Faith in one's Continuity of the consciousness");
   for (key, value) in &t {
        println!("{key}: {value}");
    }    
   // evaluate the transient value of t in circular processes of 8 hashtag and record it
   // somewhere for tracking record in different cased of claimed #   
    return 0;
}
fn q_evaluation(q: HashMap<i32, String>) -> i32 { // return evaluated value to the blockchain
   println!("Attributes of Qi in one's Continuity of the consciousness");
   for (key, value) in &q {
        println!("{key}: {value}");
    }    
   // evaluate the transient value of q in circular processes of 8 hashtag and record it
   // somewhere for tracking record in different cased of claimed #   
    return 0;
}
fn a_evaluation(a: HashMap<i32, String>) -> i32 { // return evaluated value to the blockchain
   println!("Attributes of Trust / Faith in one's Continuity of the consciousness");
   for (key, value) in &a {
        println!("{key}: {value}");
    }    
   // evaluate the transient value of a in circular processes of 8 hashtag and record it
   // somewhere for tracking record in different cased of claimed #   
    return 0;
}

*/
