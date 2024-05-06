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
pub struct InnerSpace {

  pub x_dimension: i32,
  pub y_dimension: i32,
  pub f_dimension: i32,

 /* 
  use variants of identified attributes to further classify _y, _x. _f via knowledge graph:  
  _patience, _budh, _care, _honesty,  Maturity for organization, Steps for foreign affairs,
  Visions for country
 */
}

pub mod Ydimension;

pub mod Xdimension;

pub mod Fdimension;


#[derive(Debug)]
pub enum Qualities {
  Patience(String), // "Patience / Nhẩn"
  Budh(String),    // "Budh / Morality / Thiện"
  Care(String), 
  Honesty(String), // "Honesty / TRUTH / Chân"
  Trust(String),
  QiGong(String),
  Art(String),
}

pub mod Patience;
pub mod Budh;
pub mod Care;
pub mod Honesty;
pub mod Trust;
pub mod QiGong;
pub mod Art;

/*
impl Qualities {
  fn p_evaluation(&self) -> i32 { // patience qualified and verifiable in Khương Tử Nha
    return 3; // LLM from mass data
  }

  fn b_evaluation(&self) -> i32 { // budh or morality at the bottom line of Right or Wrong
    return 3; // LLM from mass data
  }

  fn c_evaluation(&self) -> i32 { // c indicates one's seriousness and commitments
    return 3; // LLM from mass data
  }

  fn h_evaluation(&self) -> i32 { // h measures the invisible part of one's Qualities
    return 1; // LLM from mass data
  }
}

enum RatedMaturity{ // for organization
  Maturity,
}
enum StepOutcomes{ // for foreign affairs
  Steps,
}
enum VisionOutcomes{ // for country
  Visions,

 The implementation from self to selfless via _x Detachment, _y InnerPeace / RightOrWrong,
 and _f positive InnerSpace. Evaluation of x_dimension is trained via empirical observations 
 and contributions by those connected to #WuNien. Evaluation of y_dimension is trained via 
 empirical observations and contributions by those connected to #Samadhi. The observable point 
 on X-Y plane is connected to stable f_evaluation for custom trainings and treatments.

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
*/

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
  
  let mut _p = 0;    // Patience / Nhẩn
  let mut _b = 0;    // Budh / Morality / Thiện / Right & Wrong 
  let mut _c = 0;    // Care
  let mut _h = 0;    // Honesty / TRUTH / Chân
  let mut _t = 0;    // Trust / Faith to perfection 
  let mut _q = 0;    // QiGong to perfection
  let mut _a = 0;    // arts to perfection

  let mut f = HashMap::new(); // Taxonomy of f_dimension
  f.insert(0, "Person".to_string()); // sub f as the key
  
  f.insert(-1, "Empathy Awareness".to_string());   
  f.insert(-2, "Kindness Awareness".to_string());
  f.insert(-3, "Animal Energy".to_string());   
  f.insert(-4, "Extreme Desire".to_string()); 
  f.insert(-5, "Smelly 1".to_string());   // only observable in spirits
  f.insert(-6, "Smelly 2".to_string());
  f.insert(-7, "Smelly 3".to_string());  
  f.insert(-8, "Smelly 4".to_string());
  
  f.insert(1, "Empathy Awareness".to_string()); 
  f.insert(2, "Purity Awareness".to_string());   // targeted community
  f.insert(3, "Samadhi Signed Posts".to_string()); 
  f.insert(4, "Selfless Awareness".to_string()); 
  f.insert(5, "Visible Awareness-Prajna".to_string());           // practical demonstration
  f.insert(6, "Awareness-Prajna in engaged Living".to_string()); // practical innovations
  f.insert(7, "Awareness-Prajna in Forecasting and Simulation".to_string());// quantum effects
  f.insert(8, "Samadhi-Prajna".to_string());     // a new Era of consciousness technologies
  
  let mut y = HashMap::new(); // Taxonomy of y_dimension
  y.insert(0, "Peace".to_string()); //sub f as the key
  
  y.insert(-1, "Empathy".to_string());      
  y.insert(-2, "Kindness".to_string());
  y.insert(-3, "Conscience 4".to_string()); 
  y.insert(-4, "Conscience 3".to_string()); 
  y.insert(-5, "Conscience 2".to_string()); 
  y.insert(-6, "Conscience 1".to_string());
  
  y.insert(1, "Tranquillity".to_string());  
  y.insert(2, "Equanimity".to_string());     // target community
  y.insert(3, "Purity".to_string()); 
  y.insert(4, "Not-Self".to_string()); 
  y.insert(5, "Nothingness".to_string());   // Gotama's impass
  y.insert(6, "Unmoving".to_string());      // Gotama's impass
  
  let mut x = HashMap::new(); // Taxonomy of x_dimension
  x.insert(0, "Awareness".to_string()); // sub f as the key
  
  x.insert(-1, "Cultural Influence".to_string());
  x.insert(-2, "Regional Influence".to_string());
  x.insert(-3, "National Influence".to_string()); 
  x.insert(-4, "Veiled Right and Wrong".to_string()); 
  x.insert(-5, "Binding Word".to_string()); 
  x.insert(-6, "Binding Image".to_string());
  x.insert(-7, "Clinging Thought".to_string()); 
  
  x.insert(1, "HonNhien".to_string()); 
  x.insert(2, "Proper Management of that Freshness".to_string());  // target community
  x.insert(3, "Knowing conditions to make up that Freshness".to_string()); 
  x.insert(4, "Discovering process to produce the Freshness".to_string()); 
  x.insert(5, "Knowing the source of one's Thought".to_string());          // breakout
  x.insert(6, "Using cosmic energy for self-protection".to_string());   // deeper innovation
  x.insert(7, "Directing cosmic energy to help others".to_string());    // deeper innovation

  let mut p = HashMap::new(); // Taxonomy of p_dimension
  p.insert(0, "Nhan".to_string()); // sub p as the key

  let mut b = HashMap::new(); // Taxonomy of b_dimension
  b.insert(0, "Thien".to_string()); // sub b as the key

  let mut c = HashMap::new(); // Taxonomy of c_dimension
  c.insert(0, "Human".to_string()); // sub c as the key

  let mut h = HashMap::new(); // Taxonomy of h_dimension
  h.insert(0, "Chan".to_string()); // sub c as the key
  
  
  let _patience = Qualities::Patience(String::from("PatienceCases")); // recorded proof
  let _budh = Qualities::Budh(String::from("BudhCases")); // recorded proof
  let _care = Qualities::Care(String::from("CareCases")); // recorded proof
  let _honesty = Qualities::Honesty(String::from("HonestyCases"));  // recorded proof
  
  _f = f_initiation(f); 
  _y = y_evaluation(y); // transcendental Inner Peace based on the outcomes in engaged living
  _x = x_evaluation(x); // transcendental Awareness based on self evaluations and assessments

  _p = p_evaluation(p); // patience based on the outcomes in engaged living   
  _b = b_evaluation(b); // budh based on the outcomes in engaged living 
  _c = c_evaluation(c); // care based on the outcomes in engaged living 
  _h = h_evaluation(h); // honesty based on the outcomes of engaged living
  
  // evaluation of InnerSpace
  let me = InnerSpace {
    f_dimension: _f, 
    y_dimension: _y,
    x_dimension: _x,
    
  };

  return me;
  
} // end of fyx

fn f_initiation(f: HashMap<i32, String>) -> i32 {

    println!("Suggested Inner Space for cultivation");
       for (key, value) in &f {
        println!("{key}: {value}");
    }
    return 0;
}
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

fn p_evaluation(p: HashMap<i32, String>) -> i32 { // return evaluated value to the blockchain
   println!("Attributes of Patience");
   for (key, value) in &p {
        println!("{key}: {value}");
    }    
   // evaluate the transient value of b in circular processes of 8 hashtag and record it
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

fn c_evaluation(c: HashMap<i32, String>) -> i32 { // return evaluated value to the blockchain
   println!("Attributes of Care for one and the environment");
   for (key, value) in &c {
        println!("{key}: {value}");
    }    
   // evaluate the transient value of c in circular processes of 8 hashtag and record it
   // somewhere for tracking record in different cased of claimed #   
    return 0;
}
fn h_evaluation(h: HashMap<i32, String>) -> i32 { // return evaluated value to the blockchain
   println!("Attributes of Honesty / Truth in one's Continuity of the consciousness");
   for (key, value) in &h {
        println!("{key}: {value}");
    }    
   // evaluate the transient value of h in circular processes of 8 hashtag and record it
   // somewhere for tracking record in different cased of claimed #   
    return 0;
}


