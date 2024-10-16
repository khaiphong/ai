/*
  Starting at a naturally qualified person of this cycle modeled at balanced intuition
  "I 0", there is a wide range of awareness _x and inner_peace from meditation (samadhi) _y
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
      
  Proven qualities of [ indoctrinate (binding_clinging forces T-4 ],
  [ Veiled Right and Wrong T-3 ], [ Influenced T-2 ], [ KindnessEmpathy T-1 ], balanced I 0 ], 
  [ Intuition (Trust, Qi, Art) T+1 ], [ Falun (Honesty, Budh, Patience / Chân, Thiện, Nhẩn)
  T+2 ], [ Care T+3 ], [ TRUTH T+4 ] from heavily attached to totally detached, capable to 
  enable wider angle of a focused event are from [ -4 to +4 ] where 0 is the base of a
  qualified humanitas.
      
  The Description is not the Described! We use LLM to enable this possibility the persona may 
  take in ChangeManagement and/or personal rating of the hard-evidenced outcomes for lessons 
  learned based on its private ratings of the suggested positive intelligence and its 
  normative intelligence. The outcome is its vector of these rated qualities for self 
  cultivations and their effects on the persona Ignorance or #Prajna, measurable at the bottom 
  line of important focused events.
*/

use crate::persona::x_traits::{KpT};
use crate::persona::y_pointers::{KpP};

use crate::persona::ydimension::{TranscendentalMeditation, Vipassana, KpY}; 
use crate::persona::xdimension::{WuNien, KpX};
use crate::persona::fdimension::{HuiNeng, Gotama, KpF};

// LLM augmented with self-rating of traits x-dimension pointing to by SmartPointers
use crate::persona::truth::{truth_agent1, truth_agent2};
use crate::persona::care::{care_agent1, care_agent2};
use crate::persona::honesty::{honesty_agent1, honesty_agent2};
use crate::persona::intuition::{intuition_agent1, intuition_agent2};

use crate::persona::balanced::{balanced_agent1, balanced_agent2};

use crate::persona::kindnessempathy::{kindnessempathy_agent1, kindnessempathy_agent2};
use crate::persona::influenced::{influenced_agent1, influenced_agent2};
use crate::persona::veiled::{veiled_agent1, veiled_agent2};
use crate::persona::indoctrinated::{indoctrinated_agent1, indoctrinated_agent2};

// LLM augmented with self-rating of Meditation Samadhi y-dimension of SmartPointer
use crate::persona::empty_the_content::{empty_the_content_agent1, empty_the_content_agent2};
use crate::persona::dhyanasamadhi::{dhyanasamadhi_agent1, dhyanasamadhi_agent2};
use crate::persona::samadhi::{samadhi_agent1, samadhi_agent2};
use crate::persona::awareness::{awareness_agent1, awareness_agent2};

use crate::persona::prajna::{prajna_agent1, prajna_agent2};

use crate::persona::awarenessprajna::{awarenessprajna_agent1, awarenessprajna_agent2};
use crate::persona::samadhiprajna::{samadhiprajna_agent1, samadhiprajna_agent2};
use crate::persona::prajnatip1::{prajnatip1_agent1, prajnatip1_agent2};
use crate::persona::prajnatip2::{prajnatip2_agent1, prajnatip2_agent2};

// open-source LLM continuously trained with fresh data, custom by Kp for AGI agents
use crate::agi::public::{Granite, Llama, Phi, Grok, Kp};

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
  // 9 smart pointer # to be connected in learning and sharing
  #[derive(Debug)]
  struct EmptyTheContent; let _empty_the_content = EmptyTheContent;
  #[derive(Debug)]
  struct DhyanaSamadhi; let _meditation = DhyanaSamadhi;  
  #[derive(Debug)]
  struct Samadhi; let _transcendental_inner_peace = Samadhi;
  #[derive(Debug)]
  struct Awareness; let _transcendental_awareness = Awareness;
  
  #[derive(Debug)]
  struct Prajna; let _wisdom = Prajna; 
  
  #[derive(Debug)]
  struct AwarenessPrajna; let _awarenessprajna = AwarenessPrajna;
  #[derive(Debug)]
  struct SamadhiPrajna; let _samadhiprajna = SamadhiPrajna;
  #[derive(Debug)] 
  struct PrajnaTIP1; let _prajnatip1 = PrajnaTIP1;
  #[derive(Debug)] 
  struct PrajnaTIP2; let _prajnatip2 = PrajnaTIP2;
  
  println!("I'm connecting to node {:?}!", _empty_the_content);
  println!("I'm connecting to node {:?}!", _meditation);
  println!("I'm connecting to node {:?}!", _transcendental_inner_peace);
  println!("I'm connecting to node {:?}!", _transcendental_awareness);
  
  println!("I'm connecting to node {:?}!", _wisdom);
  
  println!("I'm connecting to node {:?}!", _awarenessprajna);
  println!("I'm connecting to node {:?}!", _samadhiprajna);
  println!("I'm connecting to node {:?}!", _prajnatip1);
  println!("I'm connecting to node {:?}!", _prajnatip2);
  
  // 9 trait # to be connected in learning and sharing
  #[derive(Debug)] 
  struct Truth; let _truth = Truth;
  #[derive(Debug)] 
  struct Honesty; let _honesty = Honesty;  
  #[derive(Debug)] 
  struct Care; let _care = Care;
  #[derive(Debug)] 
  struct Intuition; let _intuition = Intuition;
  
  #[derive(Debug)] 
  struct Balanced; let _balanced = Balanced;
  
  #[derive(Debug)] 
  struct KindnessEmpathy; let _kindnessempathy = KindnessEmpathy;
  #[derive(Debug)] 
  struct Influenced; let _influenced = Influenced;
  #[derive(Debug)] 
  struct Veiled; let _veiled = Veiled;
  #[derive(Debug)] 
  struct Indoctrinated; let _indoctrinated = Indoctrinated;

  println!("I'm connecting to node {:?}!", _truth);
  println!("I'm connecting to node {:?}!", _honesty);
  println!("I'm connecting to node {:?}!", _care);
  println!("I'm connecting to node {:?}!", _intuition);
  
  println!("I'm connecting to node {:?}!", _balanced);
  
  println!("I'm connecting to node {:?}!", _kindnessempathy);
  println!("I'm connecting to node {:?}!", _influenced);
  println!("I'm connecting to node {:?}!", _veiled);
  println!("I'm connecting to node {:?}!", _indoctrinated);

  // macroscopic levels of [ Oneness >< Diversities ]
  #[derive(Debug)] 
  struct Oneness; let _oneness = Oneness;
  println!("I'm connecting to node {:?}!", _oneness);
  #[derive(Debug)] 
  struct Diversities; let _diversities = Diversities;
  println!("I'm connecting to node {:?}!", _diversities);
  
  // Dversities via I-Ching and LaoTzu
  #[derive(Debug)] 
  struct Heaven; let _heaven = Heaven;
  println!("I'm connecting to node {:?}!", _heaven);
  #[derive(Debug)] 
  struct Earth; let _earth = Earth;
  println!("I'm connecting to node {:?}!", _earth);
  #[derive(Debug)] 
  struct People; let _people = People;
  println!("I'm connecting to node {:?}!", _people);
  
  // Dversities via KungTzu
  #[derive(Debug)] 
  struct Nhan; let _nhan = Nhan;
  println!("I'm connecting to node {:?}!", _nhan);
  #[derive(Debug)] 
  struct Nghia; let _nghia = Nghia;
  println!("I'm connecting to node {:?}!", _nghia);
  #[derive(Debug)] 
  struct Le; let _le = Le;
  println!("I'm connecting to node {:?}!", _le);
  #[derive(Debug)] 
  struct Tri; let _tri = Tri;
  println!("I'm connecting to node {:?}!", _tri);
  #[derive(Debug)] 
  struct Tin; let _tin = Tin;
  println!("I'm connecting to node {:?}!", _tin);

  // Dversities via Falun Gong: Honesty (identified trait), Morality, Patience
  #[derive(Debug)] 
  struct Morality; let _morality = Morality;
  println!("I'm connecting to node {:?}!", _morality);
  #[derive(Debug)] 
  struct Patience; let _patience = Patience;
  println!("I'm connecting to node {:?}!", _patience);

  // Diversities via Y-dimension
  #[derive(Debug)] 
  struct Tranquility; let _tranquility = Tranquility;
  println!("I'm connecting to node {:?}!", _tranquility);
  #[derive(Debug)] 
  struct Equanimity; let _equanimity = Equanimity;
  println!("I'm connecting to node {:?}!", _equanimity);
  #[derive(Debug)] 
  struct Purity; let _purity = Purity;
  println!("I'm connecting to node {:?}!", _purity);
  #[derive(Debug)] 
  struct Selfless; let _selfless = Selfless;
  println!("I'm connecting to node {:?}!", _selfless);  
  #[derive(Debug)] 
  struct NonThingness; let _non_thingness = NonThingness;
  println!("I'm connecting to node {:?}!", _non_thingness);
  #[derive(Debug)] 
  struct Unmoving; let _unmoving = Unmoving;
  println!("I'm connecting to node {:?}!", _unmoving);
  
  #[derive(Debug)] 
  struct Empathy; let _empathy = Empathy;
  println!("I'm connecting to node {:?}!", _empathy);
  #[derive(Debug)] 
  struct Kindness; let _kindness = Kindness;
  println!("I'm connecting to node {:?}!", _kindness);
  #[derive(Debug)] 
  struct Conscience; let _conscience = Conscience;
  println!("I'm connecting to node {:?}!", _conscience);
  #[derive(Debug)] 
  struct Conscience1; let _conscience1 = Conscience1;
  println!("I'm connecting to node {:?}!", _conscience1);  
  #[derive(Debug)] 
  struct Conscience2; let _conscience2 = Conscience2;
  println!("I'm connecting to node {:?}!", _conscience2);
  #[derive(Debug)] 
  struct Conscience3; let _conscience3 = Conscience3;
  println!("I'm connecting to node {:?}!", _conscience3);
  
  // Diversities via X-dimension
  #[derive(Debug)] 
  struct HonNhien; let _hon_nhien = HonNhien;
  println!("I'm connecting to node {:?}!", _hon_nhien);
  #[derive(Debug)] 
  struct ManagingFreshness; let _managing_freshness = ManagingFreshness;
  println!("I'm connecting to node {:?}!", _managing_freshness);
  #[derive(Debug)] 
  struct KnowingFreshness; let _knowing_freshness = KnowingFreshness;
  println!("I'm connecting to node {:?}!", _knowing_freshness);
  #[derive(Debug)] 
  struct ProcessDiscovered; let _process_discovered = ProcessDiscovered;
  println!("I'm connecting to node {:?}!", _process_discovered);  
  #[derive(Debug)] 
  struct KnowingThought; let _knowing_thought = KnowingThought;
  println!("I'm connecting to node {:?}!", _knowing_thought);
  #[derive(Debug)] 
  struct CareViaCosmicEnergy; let _care_via_cosmic_energy = CareViaCosmicEnergy;
  println!("I'm connecting to node {:?}!", _care_via_cosmic_energy);
  #[derive(Debug)] 
  struct HelpViaCosmicEnergy; let _help_via_cosmic_energy = HelpViaCosmicEnergy;
  println!("I'm connecting to node {:?}!", _help_via_cosmic_energy);
  
  #[derive(Debug)] 
  struct CulturalInfluence; let _cultural_influence = CulturalInfluence;
  println!("I'm connecting to node {:?}!", _cultural_influence);
  #[derive(Debug)] 
  struct RegionalInfluence; let _regional_influence = RegionalInfluence;
  println!("I'm connecting to node {:?}!", _regional_influence);
  #[derive(Debug)] 
  struct NationalInfluence; let _national_influence = NationalInfluence;
  println!("I'm connecting to node {:?}!", _national_influence);
  #[derive(Debug)] 
  struct VeiledType; let _veiled_type = VeiledType;
  println!("I'm connecting to node {:?}!", _veiled_type);  
  #[derive(Debug)] 
  struct BindingWord; let _binding_word = BindingWord;
  println!("I'm connecting to node {:?}!", _binding_word);
  #[derive(Debug)] 
  struct BindingImage; let _binding_image = BindingImage;
  println!("I'm connecting to node {:?}!", _binding_image);
  #[derive(Debug)] 
  struct ClingingThought; let _clinging_thought = ClingingThought;
  println!("I'm connecting to node {:?}!", _clinging_thought);  
  
  // Diversities via F-dimension
  #[derive(Debug)] 
  struct EquanimityAwareness; let _equanimity_awareness = EquanimityAwareness;
  println!("I'm connecting to node {:?}!", _equanimity_awareness);
  #[derive(Debug)] 
  struct PurityAwareness; let _purity_awareness = PurityAwareness;
  println!("I'm connecting to node {:?}!", _purity_awareness);
  #[derive(Debug)] 
  struct SignedPosts; let _signed_posts = SignedPosts;
  println!("I'm connecting to node {:?}!", _signed_posts);
  #[derive(Debug)] 
  struct SelflessAwareness; let _selfless_awareness = SelflessAwareness;
  println!("I'm connecting to node {:?}!", _selfless_awareness);  
  #[derive(Debug)] 
  struct VisibleAwarenessPrajna; let _visible_awareness_prajna = VisibleAwarenessPrajna;
  println!("I'm connecting to node {:?}!", _visible_awareness_prajna);
  #[derive(Debug)] 
  struct EngagedAwarenessPrajna; let _engaged_awareness_prajna = EngagedAwarenessPrajna;
  println!("I'm connecting to node {:?}!", _engaged_awareness_prajna);
  #[derive(Debug)] 
  struct ForecastingAwarenessPrajna; let _forecasting_awareness_prajna = ForecastingAwarenessPrajna;
  println!("I'm connecting to node {:?}!", _forecasting_awareness_prajna);
  #[derive(Debug)] 
  struct VisibleSamadhiPrajna; let _visible_samadhi_prajna = VisibleSamadhiPrajna;
  println!("I'm connecting to node {:?}!", _visible_samadhi_prajna);
  
  #[derive(Debug)] 
  struct EmpathyAwareness; let _empathy_awareness = EmpathyAwareness;
  println!("I'm connecting to node {:?}!", _empathy_awareness);
  #[derive(Debug)] 
  struct KindnessAwareness; let _kindness_awareness = KindnessAwareness;
  println!("I'm connecting to node {:?}!", _kindness_awareness);
  #[derive(Debug)] 
  struct AnimalEnergy; let _animal_energy = AnimalEnergy;
  println!("I'm connecting to node {:?}!", _animal_energy);
  #[derive(Debug)] 
  struct ExtremeDesire; let _extreme_desire = ExtremeDesire;
  println!("I'm connecting to node {:?}!", _extreme_desire);  
  #[derive(Debug)] 
  struct AnimalConsciousness; let _animal_consciousness = AnimalConsciousness;
  println!("I'm connecting to node {:?}!", _animal_consciousness);
  #[derive(Debug)] 
  struct Smelly1; let _smelly1 = Smelly1;
  println!("I'm connecting to node {:?}!", _smelly1);
  #[derive(Debug)] 
  struct Smelly2; let _smelly2 = Smelly2;
  println!("I'm connecting to node {:?}!", _smelly2); 
  #[derive(Debug)] 
  struct Smelly3; let _smelly3 = Smelly3;
  println!("I'm connecting to node {:?}!", _smelly3); 


  let _kpt = KpT::new(0);	// traits on X-dimension
  println!("I'm using {:?}!", _kpt);
  
  let _kpp = KpP::new(1);	// pointers on Y-dimension
  println!("I'm using {:?}!", _kpp);
  
  let _tm = TranscendentalMeditation {}; // TM contribution
  println!("I'm using {:?}!", _tm);
  let _vp = Vipassana {};                // Vipassana contribution
  println!("I'm using {:?}!", _vp);
  let _kpy = KpY::new(0);                // Signed Posts
  println!("I'm using {:?}!", _kpy);
  
  let _transcendental_awareness = WuNien {};                      // HuiNeng zen
  println!("I'm evaluating my {:?}!", _transcendental_awareness); 
  let _kpx = KpX::new(0);                                         // KP Awareness  
  println!("I'm evaluating my {:?}!", _kpx);
  
  let _hn = HuiNeng {};
  println!("I'm evaluating {:?}!", _hn);
  let _go = Gotama {};
  println!("I'm evaluating {:?}!", _go);
  let _kpf = KpF::new(0);
  println!("I'm evaluating my {:?}!", _kpf);
  
  
  // y_pointers of meditation where Kien_tanh is the target for average
  let _empty_the_content_agent1 = empty_the_content_agent1();
  println!("I'm augmenting {:?}!", _empty_the_content_agent1);
  let _empty_the_content_agent2 = empty_the_content_agent2();
  println!("I'm augmenting {:?}!", _empty_the_content_agent2);    
  let _dhyanasamadhi_agent1 = dhyanasamadhi_agent1();
  println!("I'm augmenting {:?}!", _dhyanasamadhi_agent1);
  let _dhyanasamadhi_agent2 = dhyanasamadhi_agent2();
  println!("I'm augmenting {:?}!", _dhyanasamadhi_agent2);  
  let _samadhi_agent1 = samadhi_agent1();
  println!("I'm augmenting {:?}!", _samadhi_agent1);
  let _samadhi_agent2 = samadhi_agent2();
  println!("I'm augmenting {:?}!", _samadhi_agent2);
  let _awareness_agent1 = awareness_agent1();
  println!("I'm augmenting {:?}!", _awareness_agent1);
  let _awareness_agent2 = awareness_agent2();
  
  let _prajna_agent1 = prajna_agent1();
  println!("I'm augmenting {:?}!", _prajna_agent1);
  let _prajna_agent2 = prajna_agent2();
  
  println!("I'm augmenting {:?}!", _awareness_agent2); 
  let _awarenessprajna_agent1 = awarenessprajna_agent1();
  println!("I'm augmenting {:?}!", _awarenessprajna_agent1);
  let _awarenessprajna_agent2 = awarenessprajna_agent2();
  println!("I'm augmenting {:?}!", _awarenessprajna_agent2); 
  let _samadhiprajna_agent1 = samadhiprajna_agent1();
  println!("I'm augmenting {:?}!", _samadhiprajna_agent1);
  let _samadhiprajna_agent2 = samadhiprajna_agent2();
  println!("I'm augmenting {:?}!", _samadhiprajna_agent2); 
  let _prajnatip1_agent1 = prajnatip1_agent1();
  println!("I'm augmenting {:?}!", _prajnatip1_agent1);
  let _prajnatip1_agent2 = prajnatip1_agent2();
  println!("I'm augmenting {:?}!", _prajnatip1_agent2); 
  let _prajnatip2_agent1 = prajnatip2_agent1();
  println!("I'm augmenting {:?}!", _prajnatip2_agent1);
  let _prajnatip2_agent2 = prajnatip2_agent2();
  println!("I'm augmenting {:?}!", _prajnatip2_agent2); 
  
  
  // x_traits of intuition level from high to low
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
  let _intuition_agent1 = intuition_agent1();
  println!("I'm augmenting {:?}!", _intuition_agent1);
  let _intuition_agent2 = intuition_agent2();
  println!("I'm augmenting {:?}!", _intuition_agent2); 

  let _balanced_agent1 = balanced_agent1();
  println!("I'm augmenting {:?}!", _balanced_agent1);
  let _balanced_agent2 = balanced_agent2();
  println!("I'm augmenting {:?}!", _balanced_agent2);  

  let _kindnessempathy_agent1 = kindnessempathy_agent1();
  println!("I'm augmenting {:?}!", _kindnessempathy_agent1);
  let _kindnessempathy_agent2 = kindnessempathy_agent2();
  println!("I'm augmenting {:?}!", _kindnessempathy_agent2);  
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


  // evaluation of InnerSpace
  let _me = crate::persona::build_inner_space(0, 0, 0);
  println!("I'm evaluating the persona {:?}!", _me);
  
  // agi mod
  let _i = Granite {};
  println!("I'm using {:?}!", _i);

  let _l = Llama {};
  println!("I'm using {:?}!", _l);
  
  let _p = Phi {};
  println!("I'm using {:?}!", _p);

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
