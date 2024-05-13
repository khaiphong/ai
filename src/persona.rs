/*
    Establish accademia forum for scientifically measuring and rating visible data.
    Each Fibonacci index is a complex Fibonacci function which can be modeled via LLM of
    identified qualities associated to the ontologies of named key and ranges of
    associated behaviors in all Activities, Relationships, and Places observable from
    collected data of the subject versus demonstrated behaviours of advanced students. Using
    AGI and specialized agents custom made for the subject, we can help the subject know
    more about oneself and How to rightly evolve in one's Continuity of the consciousness.
*/

#[derive(Debug)]
pub enum Qualities {
  Honesty(String), // "Honesty / TRUTH / Chân"
  Care(String), 
  Budh(String),    // "Budh / Morality / Thiện"
  Patience(String), // "Patience / Nhẩn"
  Trust(String),
  Qi(String),
  Art(String),
}
 
pub mod honesty;  // Knowledge graph to study the Described focused event and best possible
pub mod care;     // outcomes honestly evaluated by the persona of personal lessons learned
pub mod budh;     // relevant to the norm of the mass
pub mod patience;
pub mod trust;
pub mod qi;
pub mod art;

#[derive(Debug)]
pub struct InnerSpace {

  pub x_dimension: i32,
  pub y_dimension: i32,
  pub f_dimension: i32,
  
  pub qualities: Vec<i32>, // rated proven qualities and their effects at the bottom line
 /* 
  use variants of identified attributes to further classify _y, _x. _f via knowledge graph:  
  _patience, _budh, _care, _honesty,  Maturity for organization, Steps for foreign affairs,
  Visions for country
 */
}

pub mod ydimension;

pub mod xdimension;

pub mod fdimension;


impl Qualities { // rated qualities from -5 to +5

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

/*
 The implementation from self to selfless via _x Detachment, _y InnerPeace / RightOrWrong,
 and _f positive InnerSpace. Evaluation of x_dimension is trained via empirical observations 
 and contributions by those connected to #WuNien. Evaluation of y_dimension is trained via 
 empirical observations and contributions by those connected to #Samadhi. The observable point 
 on X-Y plane is connected to stable f_evaluation for custom trainings and treatments.
 
 Create custom types Y from [-6 to +6], X from [-7 to +7], and F from [-8 to +8] to enforce
 their validity and behaviours in using Rust compiler, LLM models, and KP custom gdb.
*/
impl InnerSpace { // we enable evaluation from LLM of the mass and from custom AGI
  fn f_evaluation(&self) -> i32 { // f(_x,_y) for person
    self.x_dimension * self.y_dimension * self.f_dimension
  
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

/*
  Starting from naturally qualified persona, the build_InnerSpace initially and periodically
  rate the persona for user custom services. The ranges for evaluation of y, x, f
  are updated according to acamedia researches. Same thing is applicable to their rating
  function outcomes: _f, _y, _x. The return is a new "me".
*/

use std::collections::HashMap;

pub fn build_InnerSpace(_x: i32, _y: i32, _f: i32) -> InnerSpace  {
 
  let mut _f = 0;    // qualified person
  let mut _y = 0;    // to be evaluated in blockchain of user self-evaluation and expert
  let mut _x = 0;    // opinions from fact-base tracked records in engaged living.
  
  let mut _h = 0;    // Honesty / TRUTH / Chân
  let mut _c = 0;    // Care
  let mut _b = 0;    // Budh / Morality / Thiện / Right & Wrong 
  let mut _p = 0;    // Patience / Nhẩn
  let mut _t = 0;    // Trust / Faith to perfection 
  let mut _q = 0;    // QiGong to perfection
  let mut _a = 0;    // arts to perfection

  let mut f = HashMap::new(); // Taxonomy of f_dimension
  f.insert(0, String::from("F 0")); //  Person sub f as the key
  
  // f(-1) = 1 = f(1) Empathy Awareness to be qualified as humanitas upward, enforced f(2) = 1
  f.insert(-1, String::from("F-1"));   // Empathy Awareness
  f.insert(-2, String::from("F-2"));   // Kindness Awareness
  f.insert(-3, String::from("F-3"));   // Animal Energy
  f.insert(-4, String::from("F-4"));   // Extreme Desire
  f.insert(-5, String::from("F-5"));   // Smelly 1 only observable in spirits
  f.insert(-6, String::from("F-6"));   // Smelly 2
  f.insert(-7, String::from("F-7"));   // Smelly 3
  f.insert(-8, String::from("F-8"));   // Smelly 4
  
  f.insert(1, String::from("F+1"));  // Empathy Awareness
  f.insert(2, String::from("F+2"));  // Purity Awareness targeted community
  f.insert(3, String::from("F+3"));  // Samadhi Signed Posts
  f.insert(4, String::from("F+4"));  // Selfless Awareness
  f.insert(5, String::from("F+5"));  // Visible Awareness-Prajna practical demonstration
  f.insert(6, String::from("F+6"));  // Awareness-Prajna in engaged Living innovations
  f.insert(7, String::from("F+7"));  // Awareness-Prajna in Forecasting and Simulation quantum
  f.insert(8, String::from("F+8"));  // Samadhi-Prajna a new Era of consciousness technologies
  
  let mut y = HashMap::new(); // Taxonomy of y_dimension
  y.insert(0, String::from("Y 0")); // Peace sub f as the key
  
  // y(-1) = 1 = y(1) Empathy to Tranquility enforced in y(2) = 1 of Equanimity
  y.insert(-1, String::from("Y-1"));   // Empathy
  y.insert(-2, String::from("Y-2"));   // Kindness
  y.insert(-3, String::from("Y-3"));   // Conscience
  y.insert(-4, String::from("Y-4"));   // Conscience-1
  y.insert(-5, String::from("Y-5"));   // Conscience-2
  y.insert(-6, String::from("Y-6"));   // Conscience-3
  
  y.insert(1, String::from("Y+1"));    // Tranquillity
  y.insert(2, String::from("Y+2"));    // Equanimity target community
  y.insert(3, String::from("Y+3"));    // Purity
  y.insert(4, String::from("Y+4"));    // Not-Self
  y.insert(5, String::from("Y+5"));    // Nothingness Gotama's impass
  y.insert(6, String::from("Y+6"));    // Unmoving Gotama's impass
  
  let mut x = HashMap::new(); // Taxonomy of x_dimension
  x.insert(0, String::from("X 0")); // Awareness sub f as the key
  
  // x(-1) = 1 = x(1) Culture & HonNhien enforced @ x(2)=1 of proper management in Equanimity
  x.insert(-1, String::from("X-1"));  // Cultural Influence
  x.insert(-2, String::from("X-2"));  // Regional Influence
  x.insert(-3, String::from("X-3"));  // National Influence
  x.insert(-4, String::from("X-4"));  // Veiled Right and Wrong
  x.insert(-5, String::from("X-5"));  // Binding Word
  x.insert(-6, String::from("X-6"));  // Binding Image
  x.insert(-7, String::from("X-7"));  // Clinging Thought
  
  x.insert(1, String::from("X+1"));   // HonNhien
  x.insert(2, String::from("X+2"));   // Proper Management of that Freshness target community
  x.insert(3, String::from("X+3"));   // Knowing conditions to make up that Freshness
  x.insert(4, String::from("X+4"));   // Discovering process to produce the Freshness
  x.insert(5, String::from("X+5"));   // Knowing the source of one's Thought breakout
  x.insert(6, String::from("X+6"));   // Using cosmic energy for self-protection innovation
  x.insert(7, String::from("X+7"));   // Directing cosmic energy to help others innovation

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
  
  // dynamic between one's avaluation and the community's prediction
  
  _y = y_evaluation(y); // transcendental Inner Peace based on the outcomes in engaged living
  _x = x_evaluation(x); // transcendental Awareness based on self evaluations and assessments
  _f = f_evaluation(f); 
  v = v_evaluation(v); // vector of qualities based on the outcomes of engaged living  

  _h = h_evaluation(h); // honesty based on the outcomes of engaged living
  _c = c_evaluation(c); // care based on the outcomes in engaged living
  _b = b_evaluation(b); // budh based on the outcomes in engaged living 
  _p = p_evaluation(p); // patience based on the outcomes in engaged living
  _t = t_evaluation(t); // trust based on the outcomes of engaged living
  _q = q_evaluation(q); // qi based on the outcomes of engaged living
  _a = a_evaluation(a); // art based on the outcomes of engaged living
  

  // evaluation of InnerSpace
  let me = InnerSpace {
    y_dimension: _y,
    x_dimension: _x,
    
    qualities: v,
    f_dimension: _f, 
  };

  return me;
  
} // end of fyxv

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

// Qualities are evaluated @ -5. -4. -3. -2. -1, 0, 1, 2, 3, 4, 5
fn h_evaluation(h: HashMap<i32, String>) -> i32 { // return evaluated value to the blockchain
   println!("Attributes of Honesty / Truth in one's Continuity of the consciousness");
   for (key, value) in &h {
        println!("{key}: {value}");
    }    
   // evaluate the transient value of h in circular processes of 8 hashtag and record it
   // somewhere for tracking record in different cased of claimed #   
    return 0;
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

fn v_evaluation(v: Vec<i32>) -> Vec<i32> { // return evaluated value to the blockchain
   let mut v = vec![-5, -2, -1, 0, 1, 2, 3]; // copy from past evaluation then evaluated
   println!("Rated measurable Attributes in one's Continuity of the consciousness");
   // evaluation one's honesty (i=0)
   let honesty: &i32 = &v[0];
   println!("The first element is {honesty}");
   
   v[0] = -3;
   // evaluation one's care (i=1)
   // evaluation one's budh (i=2)
   // evaluation one's patience (i=3)
   // evaluation one's trust (i=4)
   // evaluation one's qi (i=5)
   // evaluation one's art (i=6)
   for i in &v {
        println!("{i}");
    }    
   // evaluate the transient value of v in circular processes of 8 hashtag and record it
   // somewhere for tracking record in different cased of claimed #   
    return v;
}

