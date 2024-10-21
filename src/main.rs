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
use crate::persona::OneToDependent;     // Oneness >< Diversities
use crate::persona::Xtraits;
use crate::persona::Xlevels;
use crate::persona::Ypointers;
use crate::persona::Ylevels;
use crate::persona::Flevels;

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
  
  let _empty_the_content =  Ypointers::EmptyTheContent(String::from("#EmptyTheContent"));
  let _meditation = Ypointers::DhyanaSamadhi(String::from("#DhyanaSamadhi")); 
  let _transcendental_inner_peace = Ypointers::Samadhi(String::from("#Samadhi"));
  let _transcendental_awareness = Ypointers::Awareness(String::from("#Awareness"));
  let _wisdom = Ypointers::Prajna(String::from("#Prajna")); 
  let _awarenessprajna = Ypointers::AwarenessPrajna(String::from("#AwarenessPrajna")); 
  let _samadhiprajna = Ypointers::SamadhiPrajna(String::from("#SamadhiPrajna"));
  let _prajnatip1 = Ypointers::PrajnaTIP1(String::from("#PrajnaTIP1"));
  let _prajnatip2 = Ypointers::PrajnaTIP2(String::from("#PrajnaTIP2"));
  
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
  let _truth = Xtraits::Truth(String::from("#Truth"));
  let _honesty = Xtraits::Honesty(String::from("#Honesty"));  
  let _care = Xtraits::Care(String::from("#TCare"));
  let _intuition = Xtraits::Intuition(String::from("#Intuition"));
  let _balanced = Xtraits::Balanced(String::from("#Balanced"));
  let _kindnessempathy = Xtraits::KindnessEmpathy(String::from("#KindnessEmpathy"));  
  let _influenced = Xtraits::Influenced(String::from("#Influenced"));
  let _veiled = Xtraits::Veiled(String::from("#Veiled"));
  let _indoctrinated = Xtraits::Indoctrinated(String::from("#Indoctrinated"));

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
  let _oneness = OneToDependent::Oneness(String::from("Oneness"));
  println!("I'm connecting to node {:?}!", _oneness);
  let _diversities = OneToDependent::Diversities(String::from("Diversities"));
  println!("I'm connecting to node {:?}!", _diversities);
    
  // Dversities via I-Ching and LaoTzu
  let _heaven = OneToDependent::Heaven(String::from("Heaven"));
  println!("I'm connecting to node {:?}!", _heaven);
  let _earth = OneToDependent::Earth(String::from("Earth"));
  println!("I'm connecting to node {:?}!", _earth);
  let _people = OneToDependent::People(String::from("People"));
  println!("I'm connecting to node {:?}!", _people);
    
  // Dversities via KungTzu
  let _nhan = OneToDependent::Nhan(String::from("Nhan"));
  println!("I'm connecting to node {:?}!", _nhan);
  let _nghia = OneToDependent::Nghia(String::from("Nghia"));
  println!("I'm connecting to node {:?}!", _nghia);
  let _le = OneToDependent::Le(String::from("Le"));
  println!("I'm connecting to node {:?}!", _le);
  let _tri = OneToDependent::Tri(String::from("Tri"));
  println!("I'm connecting to node {:?}!", _tri);
  let _tin = OneToDependent::Tin(String::from("Tin"));
  println!("I'm connecting to node {:?}!", _tin);

  // Dversities via Falun: Honesty (identified trait), Morality, Patience
  let _morality = OneToDependent::Morality(String::from("Morality"));
  println!("I'm connecting to node {:?}!", _morality);
  let _patience = OneToDependent::Patience(String::from("Patience"));
  println!("I'm connecting to node {:?}!", _patience);


  // Diversities via Y-dimension
  let _tranquility =  Ylevels::Tranquility(String::from("Tranquility"));
  println!("I'm connecting to node {:?}!", _tranquility);
  let _equanimity =  Ylevels::Equanimity(String::from("Equanimity"));
  println!("I'm connecting to node {:?}!", _equanimity);
  let _purity =  Ylevels::Purity(String::from("Purity"));
  println!("I'm connecting to node {:?}!", _purity);
  let _selfless =  Ylevels::Selfless(String::from("Selfless"));
  println!("I'm connecting to node {:?}!", _selfless);  
  let _nonthingness =  Ylevels::NonThingness(String::from("NonThingness"));
  println!("I'm connecting to node {:?}!", _nonthingness);
  let _unmoving =  Ylevels::Unmoving(String::from("Unmoving"));  
  println!("I'm connecting to node {:?}!", _unmoving);
 
  let _empathy =  Ylevels::Empathy(String::from("Empathy"));
  println!("I'm connecting to node {:?}!", _empathy);
  let _kindness =  Ylevels::Kindness(String::from("Kindness"));
  println!("I'm connecting to node {:?}!", _kindness);
  let _conscience =  Ylevels::Conscience(String::from("Conscience"));
  println!("I'm connecting to node {:?}!", _conscience);
  let _conscience1 =  Ylevels::Conscience1(String::from("Conscience1"));
  println!("I'm connecting to node {:?}!", _conscience1);  
  let _conscience2 =  Ylevels::Conscience2(String::from("Conscience2"));
  println!("I'm connecting to node {:?}!", _conscience2);
  let _conscience3 =  Ylevels::Conscience3(String::from("Conscience3"));
  println!("I'm connecting to node {:?}!", _conscience3);

  // Diversities via X-dimension
  let _honnhien =  Xlevels::HonNhien(String::from("HonNhien"));
  println!("I'm connecting to node {:?}!", _honnhien);
  let _managing_freshness =  Xlevels::ManagingFreshness(String::from("ManagingFreshness"));
  println!("I'm connecting to node {:?}!", _managing_freshness);
  
  let _knowing_freshness =  Xlevels::KnowingFreshness(String::from("KnowingFreshness"));
  println!("I'm connecting to node {:?}!", _knowing_freshness);
  let _process_discovered =  Xlevels::ProcessDiscovered(String::from("ProcessDiscovered"));
  println!("I'm connecting to node {:?}!", _process_discovered);  
  let _knowing_thought =  Xlevels::KnowingThought(String::from("KnowingThought"));
  println!("I'm connecting to node {:?}!", _knowing_thought);
  let _care_via_cosmic_energy =  	
      Xlevels::CareViaCosmicEnergy(String::from("CareViaCosmicEnergy"));
  println!("I'm connecting to node {:?}!", _care_via_cosmic_energy);
  let _help_via_cosmic_energy =  	
      Xlevels::HelpViaCosmicEnergy(String::from("HelpViaCosmicEnergy"));
  println!("I'm connecting to node {:?}!", _help_via_cosmic_energy);
  
  let _cultural_influenced =  Xlevels::CulturalInfluenced(String::from("CulturalInfluenced"));
  println!("I'm connecting to node {:?}!", _cultural_influenced);
  let _regional_influenced =  Xlevels::RegionalInfluenced(String::from("RegionalInfluenced"));
  println!("I'm connecting to node {:?}!", _regional_influenced);
  let _national_influenced =  Xlevels::NationalInfluenced(String::from("NationalInfluenced"));
  println!("I'm connecting to node {:?}!", _national_influenced);
  let _veiled_type =  Xlevels::VeiledType(String::from("VeiledType"));
  println!("I'm connecting to node {:?}!", _veiled_type);  
  let _binding_word =  Xlevels::BindingWord(String::from("BindingWord"));
  println!("I'm connecting to node {:?}!", _binding_word);
  let _binding_image =  Xlevels::BindingImage(String::from("BindingImage"));
  println!("I'm connecting to node {:?}!", _binding_image);
  let _clinging_thought =  Xlevels::ClingingThought(String::from("ClingingThought"));
  println!("I'm connecting to node {:?}!", _clinging_thought);  
    
  // Diversities via F-dimension
  let _equanimity_awareness =  
      Flevels::EquanimityAwareness(String::from("EquanimityAwareness"));
  println!("I'm connecting to node {:?}!", _equanimity_awareness);
  let _purity_awareness = Flevels::PurityAwareness(String::from("PurityAwareness"));
  println!("I'm connecting to node {:?}!", _purity_awareness);
  let _signed_posts = Flevels::SignedPosts(String::from("SignedPosts"));
  println!("I'm connecting to node {:?}!", _signed_posts);
  let _selfless_awareness = Flevels::SelflessAwareness(String::from("SelflessAwareness"));
  println!("I'm connecting to node {:?}!", _selfless_awareness);  
  let _visible_awareness_prajna =  
      Flevels::VisibleAwarenessPrajna(String::from("VisibleAwarenessPrajna"));
  println!("I'm connecting to node {:?}!", _visible_awareness_prajna);
  let _engaged_awareness_prajna =  
      Flevels::EngagedAwarenessPrajna(String::from("EngagedAwarenessPrajna"));
  println!("I'm connecting to node {:?}!", _engaged_awareness_prajna);
  let _forecasting_awareness_prajna =  
      Flevels::ForecastingAwarenessPrajna(String::from("ForecastingAwarenessPrajna"));
  println!("I'm connecting to node {:?}!", _forecasting_awareness_prajna);
  let _visible_samadhi_prajna =  
      Flevels::VisibleSamadhiPrajna(String::from("VisibleSamadhiPrajna"));
  println!("I'm connecting to node {:?}!", _visible_samadhi_prajna);
    
  let _empathy_awareness = Flevels::EmpathyAwareness(String::from("EmpathyAwareness"));
  println!("I'm connecting to node {:?}!", _empathy_awareness);
  let _kindness_awareness = Flevels::KindnessAwareness(String::from("KindnessAwareness"));
  println!("I'm connecting to node {:?}!", _kindness_awareness);
  let _animal_energy = Flevels::AnimalEnergy(String::from("AnimalEnergy"));
  println!("I'm connecting to node {:?}!", _animal_energy);
  let _extreme_desire = Flevels::ExtremeDesire(String::from("ExtremeDesire"));
  println!("I'm connecting to node {:?}!", _extreme_desire);  
  let _animal_consciousness = 
      Flevels::AnimalConsciousness(String::from("AnimalConsciousness"));
  println!("I'm connecting to node {:?}!", _animal_consciousness);
  let _smelly1 = Flevels::Smelly1(String::from("Smelly1"));
  println!("I'm connecting to node {:?}!", _smelly1);
  let _smelly2 = Flevels::Smelly2(String::from("Smelly2"));
  println!("I'm connecting to node {:?}!", _smelly2); 
  let _smelly3 = Flevels::Smelly3(String::from("Smelly3"));
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
