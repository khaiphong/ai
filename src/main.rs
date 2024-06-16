/*
  Starting at a naturally qualified person of this cycle modeled at balanced intuition
  "I 0", there is a wide range of awareness _x and inner_peace from meditation (samadhy) _y
  from negative of binding-clinging forces to positive in detachment based on self-evaluated 
  plus LLM opinions of x_evaluation and y_evaluation. We use mutable variable instead of
  self-evaluated defined const as visible x_traits and y_traints leading to higher 
  conmsciousness at rated points of complex Fibonacci Nunber for Prajna >< AGI (Agentic 
  Retrieval Augmented Generation / 
  https://dev.to/shuttle_dev/building-agentic-rag-with-rust-openai-qdrant-3bjd), then modeling
  which is applicable to any kind of persona (person, Inter-Realm, organization, foreign 
  affairs, a nation, a family, project, etc). For elites currently a person, the evaluation 
  are intended to help that person moving from evolution then degeneration to the right  
  positive evaluation with oservable attributes of persona mod: Given the following hashtag # 
  and user description, we evaluate the experience in [ observable traits from attached 
  totally detached in TRUTH ] to the outcome of _y, _x and then _f round cycle from Self to 
  Selfless at deeper Detachment.
      
  Proven qualities of [ indoctrinate (binding_clinging forces I-4 ],
  [ Veiled Right and Wrong I-3 ], [ Influenced I-2 ], [ KindnessEmpathy I-1 ], balanced I 0 ], 
  [ Intuition (Trust, Qi, Art) I+1 ], [ Falun (Honesty, Budh, Patience / Chân, Thiện, Nhẩn)
  I+2 ], [ Care I+3 ], [ TRUTH I+4 ] from heavily attached to totally detached, capable to 
  enable wider angle of a focused event are from [ -4 to +4 ] where 0 is the base of a
  qualified humanitas.
      
  The Description is not the Described! We use LLM to enable this possibility the persona may 
  take in ChangeManagement and/or personal rating of the hard-evidenced outcomes for lessons 
  learned based on its private ratings of the suggested positive intelligence and its 
  normative intelligence. The outcome is its vector of these rated qualities for self 
  cultivations and their effects on the persona Ignorance or Prajna, measurable at the bottom 
  line of important focused events.
*/

use crate::persona::x_traits::{KpI};
//use crate::persona::y_traits::{KpS};

use crate::persona::ydimension::{TranscendentalMeditation, Vipassana, KpY}; 
use crate::persona::xdimension::{WuNien, KpX};
use crate::persona::fdimension::{HuiNeng, Gotama, KpF};

// LLM augmented with self-rating of Intuition traits x-dimension pointing to SmartPointer
use crate::persona::truth::{truth_agent1, truth_agent2};
use crate::persona::care::{care_agent1, care_agent2};
use crate::persona::falun::{falun_agent1, falun_agent2};
use crate::persona::intuition::{intuition_agent1, intuition_agent2};

use crate::persona::balanced::{balanced_agent1, balanced_agent2};

use crate::persona::kindness_empathy::{kindness_empathy_agent1, kindness_empathy_agent2};
use crate::persona::influenced::{influenced_agent1, influenced_agent2};
use crate::persona::veiled::{veiled_agent1, veiled_agent2};
use crate::persona::indoctrinated::{indoctrinated_agent1, indoctrinated_agent2};

// LLM augmented with self-rating of Meditation Samadhi y-dimension of SmartPointer
use crate::persona::empty_the_content::{empty_the_content_agent1, empty_the_content_agent2};
use crate::persona::dhyana_samadhi::{dhyana_samadhi_agent1, dhyana_samadhi_agent2};
use crate::persona::meditation::{meditation_agent1, meditation_agent2};
use crate::persona::kien_tanh::{kien_tanh_agent1, kien_tanh_agent2};
//use crate::persona::awareness_prajna::{awareness_prajna_agent1, awareness_prajna_agent2};
//use crate::persona::samadhi_prajna::{samadhi_prajna_agent1, samadhi_prajna_agent2};
//use crate::persona::prajna_tip::{prajna_tip_agent1, prajna_tip_agent2};

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
  // 9 public Hashtag # to be connected in learning and sharing
  #[derive(Debug)] 
  struct Intuition; let _intuition = Intuition;
  #[derive(Debug)]
  struct DhyanaSamadhi; let _meditation = DhyanaSamadhi;
  #[derive(Debug)]
  struct EmptyTheContent; let _empty_the_content = EmptyTheContent;
  #[derive(Debug)]
  struct Samadhi; let _transcendental_inner_peace = Samadhi;
  #[derive(Debug)]
  struct Awareness; let _transcendental_awareness = Awareness;
  #[derive(Debug)]
  struct Prajna; let _wisdom = Prajna;
  #[derive(Debug)]
  struct AwarenessPrajna; let _awareness_prajna = AwarenessPrajna;
  #[derive(Debug)]
  struct SamadhiPrajna; let _samadhi_prajna = SamadhiPrajna;
  #[derive(Debug)] 
  struct PrajnaTIP; let _prajna_tip = PrajnaTIP;

  println!("I'm connecting to node {:?}!", _intuition);
  println!("I'm connecting to node {:?}!", _meditation);
  println!("I'm connecting to node {:?}!", _empty_the_content);
  println!("I'm connecting to node {:?}!", _transcendental_inner_peace);
  println!("I'm connecting to node {:?}!", _transcendental_awareness);
  println!("I'm connecting to node {:?}!", _wisdom);
  println!("I'm connecting to node {:?}!", _awareness_prajna);
  println!("I'm connecting to node {:?}!", _samadhi_prajna);
  println!("I'm connecting to node {:?}!", _prajna_tip);

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
  let _kpx = KpX::new(0);
  println!("I'm evaluating my {:?}!", _kpx);
  
  let _hn = HuiNeng {};
  println!("I'm evaluating {:?}!", _hn);
  let _go = Gotama {};
  println!("I'm evaluating {:?}!", _go);
  let _kpf = KpF::new(0);
  println!("I'm evaluating my {:?}!", _kpf);
  
  
  // x_traits of intuition level from high to low
  let _truth_agent1 = truth_agent1();
  println!("I'm augmenting {:?}!", _truth_agent1);
  let _truth_agent2 = truth_agent2();
  println!("I'm augmenting {:?}!", _truth_agent2);    
  let _care_agent1 = care_agent1();
  println!("I'm augmenting {:?}!", _care_agent1);
  let _care_agent2 = care_agent2();
  println!("I'm augmenting {:?}!", _care_agent2);  
  let _falun_agent1 = falun_agent1();
  println!("I'm augmenting {:?}!", _falun_agent1);
  let _falun_agent2 = falun_agent2();
  println!("I'm augmenting {:?}!", _falun_agent2); 
  let _intuition_agent1 = intuition_agent1();
  println!("I'm augmenting {:?}!", _intuition_agent1);
  let _intuition_agent2 = intuition_agent2();
  println!("I'm augmenting {:?}!", _intuition_agent2); 

  let _balanced_agent1 = balanced_agent1();
  println!("I'm augmenting {:?}!", _balanced_agent1);
  let _balanced_agent2 = balanced_agent2();
  println!("I'm augmenting {:?}!", _balanced_agent2);  

  let _kindness_empathy_agent1 = kindness_empathy_agent1();
  println!("I'm augmenting {:?}!", _kindness_empathy_agent1);
  let _kindness_empathy_agent2 = kindness_empathy_agent2();
  println!("I'm augmenting {:?}!", _kindness_empathy_agent2);  
  let _influenced_agent1 = influenced_agent1();
  println!("I'm augmenting {:?}!", _influenced_agent1);
  let _influenced_agent2 = influenced_agent2();
  println!("I'm augmenting {:?}!", _influenced_agent2); 
  let _veiled_agent1 = veiled_agent1();
  println!("I'm augmenting {:?}!", _veiled_agent1);
  let _veiled_agent2 = veiled_agent2();
  println!("I'm augmenting {:?}!", _veiled_agent2);  
  let _indoctrinated_agent1 = indoctrinated_agent1();
  println!("I'm augmenting {:?}!", _indoctrinated_agent1);
  let _indoctrinated_agent2 = indoctrinated_agent2();
  println!("I'm augmenting {:?}!", _indoctrinated_agent2); 

  // y_traits of meditation from low from high where Kien_tanh is the target for average
  let _empty_the_content_agent1 = empty_the_content_agent1();
  println!("I'm augmenting {:?}!", _empty_the_content_agent1);
  let _empty_the_content_agent2 = empty_the_content_agent2();
  println!("I'm augmenting {:?}!", _empty_the_content_agent2);    
  let _dhyana_samadhi_agent1 = dhyana_samadhi_agent1();
  println!("I'm augmenting {:?}!", _dhyana_samadhi_agent1);
  let _dhyana_samadhi_agent2 = dhyana_samadhi_agent2();
  println!("I'm augmenting {:?}!", _dhyana_samadhi_agent2);  
  let _meditation_agent1 = meditation_agent1();
  println!("I'm augmenting {:?}!", _meditation_agent1);
  let _meditation_agent2 = meditation_agent2();
  println!("I'm augmenting {:?}!", _meditation_agent2);
  let _kien_tanh_agent1 = kien_tanh_agent1();
  println!("I'm augmenting {:?}!", _kien_tanh_agent1);
  let _kien_tanh_agent2 = kien_tanh_agent2();
  println!("I'm augmenting {:?}!", _kien_tanh_agent2); 
//  let _awareness_prajna_agent1 = awareness_prajna_agent1();
//  println!("I'm augmenting {:?}!", _awareness_prajna_agent1);
//  let _awareness_prajna_agent2 = awareness_prajna_agent2();
//  println!("I'm augmenting {:?}!", _awareness_prajna_agent2); 
//  let _samadhi_prajna_agent1 = samadhi_prajna_agent1();
//  println!("I'm augmenting {:?}!", _samadhi_prajna_agent1);
//  let _samadhi_prajna_agent2 = samadhi_prajna_agent2();
//  println!("I'm augmenting {:?}!", _samadhi_prajna_agent2); 
//  let _prajna_tip_agent1 = prajna_tip_agent1();
//  println!("I'm augmenting {:?}!", _prajna_tip_agent1);
//  let _prajna_tip_agent2 = prajna_tip_agent2();
//  println!("I'm augmenting {:?}!", _prajna_tip_agent2); 

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
