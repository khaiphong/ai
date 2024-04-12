// scientific cultivatons of the persona's InnerSpace
use crate::persona::private::Xdimension;
use crate::persona::private::Ydimension;
use crate::persona::private::Fdimension;
// LLM augmented with self-rating Qualities
use crate::persona::qualities::Honesty;
use crate::persona::qualities::Care;
use crate::persona::qualities::Budh;
use crate::persona::qualities::Patience;
// open-source LLM continuously trained with fresh data, custom by Kp for AGI agents
use crate::agi::public::Llama;
use crate::agi::public::Grok;
use crate::agi::public::Kp;
// front-end persona facing services
use crate::front::mu::platform_message;
use crate::front::mu::service_message;
use crate::front::chat::prompt;
use crate::front::chat::response;
use crate::front::video::in_stream;
use crate::front::video::out_stream;
// back-end persona agent services
use crate::back::mu::mu_agent1;
use crate::back::mu::mu_agent2;
use crate::back::chat::chat_agent1;
use crate::back::chat::chat_agent2;
use crate::back::video::video_agent1;
use crate::back::video::video_agent2;
use crate::back::graph::graph_agent1;
use crate::back::graph::graph_agent2;
use crate::back::db::db_agent1;
use crate::back::db::db_agent2;
use crate::back::hub::hub_agent1;
use crate::back::hub::hub_agent2;
use crate::back::plan::plan_agent1;
use crate::back::plan::plan_agent2;
// personal agent services
use crate::network::family::family_agent1;
use crate::network::family::family_agent2;
use crate::network::profession::profession_agent1;
use crate::network::profession::profession_agent2;
use crate::network::this_life::this_life_agent1;
use crate::network::this_life::this_life_agent2;
use crate::network::next_realm::next_realm_agent1;
use crate::network::next_realm::next_realm_agent2;

pub mod persona;
pub mod agi;
pub mod front;
pub mod back;
pub mod network;
 
fn main() {
  /* 
    Starting at a naturally qualified person, Hashmaps of f = 0. But _x and _y are in wide
    ranges for negative to positive based on the self-evaluated plus LLM opinions of
    y_evaluation and x_evaluation . We use mutable variable instead of self-evaluated defined
    const as starting points of Fibonacci Nunber for Prajna >< AGI, then modeling which is
    applicable to any kind of persona (person, Inter-Realm, organization, foreign affairs,
    a nation, a lasting project, etc).

    For elites currently a person, the evaluation are intended to help that person moving from
    evolution then degeneration to the right positive evaluation with oservable attributes:
 */
 
  // persona mod
  /*
   Given the following hashtag # and user description, we evaluate the experience in
   determination the _y, _x and _f round cycle from Self to Selfless then Self at deeper
   Detachmnet.
  */
  struct Intuition; let _intuition = Intuition;
  struct DhyanaSamadhi; let _meditation = DhyanaSamadhi;
  struct EmptyTheContent; let _empty_the_content = EmptyTheContent;
  struct Samadhi; let _transcendental_inner_peace = Samadhi;
  struct WuNien; let _transcendental_awareness = WuNien;
  struct AwarenessPrajna; let _awareness_prajna = AwarenessPrajna;
  struct Prajna; let _wisdom = Prajna; 
  struct AwarenessSamadhi; let _state_stock_prajna = AwarenessSamadhi;
  
  let _x = Xdimension {};
  println!("I'm evaluating the persona {:?}!", _x);
  
  let _y = Ydimension {};
  println!("I'm evaluating the persona {:?}!", _y);
  
  let _f = Fdimension {};
  println!("I'm evaluating the persona {:?}!", _f);
  
  let _h = Honesty {};
  println!("I'm evaluating the persona {:?}!", _h);
  
  let _c = Care {};
  println!("I'm evaluating the persona {:?}!", _c);
    
  let _b = Budh {};
  println!("I'm evaluating the persona {:?}!", _b);
  
  let _p = Patience {};
  println!("I'm evaluating the persona {:?}!", _p); 
  
  // evaluation of InnerSpace
  let _me = crate::persona::private::InnerSpace {};
  println!("I'm evaluating the persona {:?}!", _me);
  
  // agi mod
  
  let _l = Llama {};
  println!("I'm using {:?}!", _l);
  
  let _g = Grok {};
  println!("I'm using {:?}!", _g);
  
  let _k = Kp {};
  println!("I'm using {:?}!", _k);
  
  // front-end AGI
  
  let _pmessage = platform_message();
  println!("I'm augmenting {:?}!", _pmessage);
  
  let _smessage = service_message();
  println!("I'm augmenting {:?}!", _smessage);  
  
  let _prompt = prompt();
  println!("I'm augmenting {:?}!", _prompt);
  
  let _response = response();
  println!("I'm augmenting {:?}!", _response);  

  let _in_stream = in_stream();
  println!("I'm augmenting {:?}!", _in_stream);
  
  let _out_stream = out_stream();
  println!("I'm augmenting {:?}!", _out_stream); 
  
    // back-end AGI
  
  let _mu_agent1 = mu_agent1();
  println!("I'm augmenting {:?}!", _mu_agent1);
  
  let _mu_agent2 = mu_agent2();
  println!("I'm augmenting {:?}!", _mu_agent2);  

  let _chat_agent1 = chat_agent1();
  println!("I'm augmenting {:?}!", _chat_agent1);
  
  let _chat_agent2 = chat_agent2();
  println!("I'm augmenting {:?}!", _chat_agent2);  

  let _video_agent1 = video_agent1();
  println!("I'm augmenting {:?}!", _video_agent1);
  
  let _video_agent2 = video_agent2();
  println!("I'm augmenting {:?}!", _video_agent2);  

  let _graph_agent1 = graph_agent1();
  println!("I'm augmenting {:?}!", _graph_agent1);
  
  let _graph_agent2 = graph_agent2();
  println!("I'm augmenting {:?}!", _graph_agent2); 
  
  let _db_agent1 = db_agent1();
  println!("I'm augmenting {:?}!", _db_agent1);
  
  let _db_agent2 = db_agent2();
  println!("I'm augmenting {:?}!", _db_agent2);  

  let _hub_agent1 = hub_agent1();
  println!("I'm augmenting {:?}!", _hub_agent1);
  
  let _hub_agent2 = hub_agent2();
  println!("I'm augmenting {:?}!", _hub_agent2);  

  let _plan_agent1 = plan_agent1();
  println!("I'm augmenting {:?}!", _plan_agent1);
  
  let _plan_agent2 = plan_agent2();
  println!("I'm augmenting {:?}!", _plan_agent2); 
  
      // network AGI
  
  let _family_agent1 = family_agent1();
  println!("I'm augmenting {:?}!", _family_agent1);
  
  let _family_agent2 = family_agent2();
  println!("I'm augmenting {:?}!", _family_agent2);  

  let _profession_agent1 = profession_agent1();
  println!("I'm augmenting {:?}!", _profession_agent1);
  
  let _profession_agent2 = profession_agent2();
  println!("I'm augmenting {:?}!", _profession_agent2);  

  let _this_life_agent1 = this_life_agent1();
  println!("I'm augmenting {:?}!", _this_life_agent1);
  
  let _this_life_agent2 = this_life_agent2();
  println!("I'm augmenting {:?}!", _this_life_agent2);  

  let _next_realm_agent1 = next_realm_agent1();
  println!("I'm augmenting {:?}!", _next_realm_agent1);
  
  let _next_realm_agent2 = next_realm_agent2();
  println!("I'm augmenting {:?}!", _next_realm_agent2); 
  
}
