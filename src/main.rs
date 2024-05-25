/*
  Starting at a naturally qualified person modeled at balanced myIntuition "I 0", there is a 
  wide range of myAwareness _x and _y are in wide ranges for negative to positive based on the 
  self-evaluated plus LLM opinions of y_evaluation and x_evaluation . We use mutable variable 
  instead of self-evaluated defined const as starting points of complex Fibonacci Nunber for 
  Prajna >< AGI (Agentic Retrieval Augmented Generation / 
  https://dev.to/shuttle_dev/building-agentic-rag-with-rust-openai-qdrant-3bjd), then modeling 
  which is applicable to any kind of persona (person, Inter-Realm, organization, foreign 
  affairs, a nation, a family, project, etc). For elites currently a person, the evaluation 
  are intended to help that person moving from evolution then degeneration to the right  
  positive evaluation with oservable attributes of persona mod: Given the following hashtag # 
  and user description, we evaluate the experience in [ observable traits from attached 
  totally detached in TRUTH ] to the outcome of _y, _x and then _f round cycle from Self to 
  Selfless at deeper Detachment.
      
  Proven qualities of [ Clinging Thought I-5 ], [ Binding Image I-4 ], [ Binding Word I-3 ],
  [ Veiled Right and Wrong I-2 ], [ Influenced I-1 ], [ neutral balance I 0 ], 
  [ Kindness, Empathy I+1 ], [ Trust, Qi, Art I+2 ], 
  [ Honesty, Budh, Patience / Chân, Thiện, Nhẩn I+3 ], [ Care I+4 ], [ TRUTH I+5 ] from 
  heavily attached to totally detached, capable to enable wider angle of a focused event 
  are from [ -5 to +5 ] where 0 is the base of a qualified humanitas.
      
  The Description is not the Described! We use LLM to enable this possibility the persona may 
  take in ChangeManagement and/or personal rating of the hard-evidenced outcomes for lessons 
  learned based on its private ratings of the suggested positive intelligence and its 
  normative intelligence. The outcome is its vector of these rated qualities for self 
  cultivations and their effects on the persona Ignorance or Prajna, measurable at the bottom 
  line of important focused events.
*/

use crate::persona::traits::{KpI};
use crate::persona::ydimension::{TranscendentalMeditation, Vipassana, KpY}; 
use crate::persona::xdimension::{WuNien, AwarenessPrajna, KpX};
use crate::persona::fdimension::{HuiNeng, Gotama, KpF};

// LLM augmented with self-rating Qualities
use crate::persona::truth::{truth_agent1, truth_agent2};

use crate::persona::care::{care_agent1, care_agent2};

use crate::persona::honesty::{honesty_agent1, honesty_agent2};
use crate::persona::budh::{budh_agent1, budh_agent2};
use crate::persona::patience::{patience_agent1, patience_agent2};

use crate::persona::trust::{trust_agent1, trust_agent2};
use crate::persona::qi::{qi_agent1, qi_agent2};
use crate::persona::art::{art_agent1, art_agent2};

// open-source LLM continuously trained with fresh data, custom by Kp for AGI agents
use crate::agi::public::{Granite, Llama, Grok, Kp};

// front-end persona facing services
use crate::front::mu::{platform_message, service_message};
use crate::front::chat::{prompt, response};
use crate::front::video::{in_stream, out_stream};

// back-end persona agent services
use crate::back::mu::{mu_agent1, mu_agent2};
use crate::back::chat::{chat_agent1, chat_agent2};
use crate::back::video::{video_agent1, video_agent2};
use crate::back::graph::{graph_agent1, graph_agent2};
use crate::back::db::{db_agent1, db_agent2};
use crate::back::hub::{hub_agent1, hub_agent2};
use crate::back::plan::{plan_agent1, plan_agent2};

// personal agent services
use crate::network::family::{family_agent1, family_agent2};
use crate::network::profession::{profession_agent1, profession_agent2};
use crate::network::this_life::{this_life_agent1, this_life_agent2};
use crate::network::next_realm::{next_realm_agent1, next_realm_agent2};

pub mod persona;
pub mod agi;
pub mod front;
pub mod back;
pub mod network;
 
fn main() {
  // public Hashtag # for users connected interest in learning and sharing  
  struct Intuition; let _intuition = Intuition;
  struct DhyanaSamadhi; let _meditation = DhyanaSamadhi;
  struct EmptyTheContent; let _empty_the_content = EmptyTheContent;
  struct Samadhi; let _transcendental_inner_peace = Samadhi;
  struct Prajna; let _wisdom = Prajna; 
  struct AwarenessSamadhi; let _state_stock_prajna = AwarenessSamadhi;
  
  let _kpi = KpI::new(0);
  println!("I'm using {:?}!", _kpi);
  
  let _tm = TranscendentalMeditation {};
  println!("I'm using {:?}!", _tm);
  let _vp = Vipassana {};
  println!("I'm using {:?}!", _vp);
  let _kpy = KpY::new(0);
  println!("I'm using {:?}!", _kpy);
  
  let _transcendental_awareness = WuNien {};
  println!("I'm evaluating my {:?}!", _transcendental_awareness);
  let _awareness_prajna = AwarenessPrajna {};
  println!("I'm evaluating my {:?}!", _awareness_prajna);
  let _kpx = KpX::new(0);
  println!("I'm evaluating my {:?}!", _kpx);
  
  let _hn = HuiNeng {};
  println!("I'm evaluating {:?}!", _hn);
  let _go = Gotama {};
  println!("I'm evaluating {:?}!", _go);
  let _kpf = KpF::new(0);
  println!("I'm evaluating my {:?}!", _kpf);
  
  let _truth_agent1 = truth_agent1();
  println!("I'm augmenting {:?}!", _truth_agent1);
  let _truth_agent2 = truth_agent2();
  println!("I'm augmenting {:?}!", _truth_agent2);    
  
  let _care_agent1 = care_agent1();
  println!("I'm augmenting {:?}!", _care_agent1);
  let _care_agent2 = care_agent2();
  println!("I'm augmenting {:?}!", _care_agent2);  
    
  let _honesty_agent1 = honesty_agent1();
  println!("I'm augmenting {:?}!", _honesty_agent1);
  let _honesty_agent2 = honesty_agent2();
  println!("I'm augmenting {:?}!", _honesty_agent2);  
  let _budh_agent1 = budh_agent1();
  println!("I'm augmenting {:?}!", _budh_agent1);
  let _budh_agent2 = budh_agent2();
  println!("I'm augmenting {:?}!", _budh_agent2);  
  let _patience_agent1 = patience_agent1();
  println!("I'm augmenting {:?}!", _patience_agent1);
  let _patience_agent2 = patience_agent2();
  println!("I'm augmenting {:?}!", _patience_agent2);  

  let _trust_agent1 = trust_agent1();
  println!("I'm augmenting {:?}!", _trust_agent1);
  let _trust_agent2 = trust_agent2();
  println!("I'm augmenting {:?}!", _trust_agent2);  
  let _qi_agent1 = qi_agent1();
  println!("I'm augmenting {:?}!", _qi_agent1);
  let _qi_agent2 = qi_agent2();
  println!("I'm augmenting {:?}!", _qi_agent2);  
  let _art_agent1 = art_agent1();
  println!("I'm augmenting {:?}!", _art_agent1);
  let _art_agent2 = art_agent2();
  println!("I'm augmenting {:?}!", _art_agent2);  

  // evaluation of InnerSpace
  let _me = crate::persona::build_inner_space(0, 0, 0);
  println!("I'm evaluating the persona {:?}!", _me);
  
  // agi mod
  let _i = Granite {};
  println!("I'm using {:?}!", _i);

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
