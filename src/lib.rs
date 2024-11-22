// modules specific to KpPlatform

pub mod front {
    pub mod mu {
		pub fn platform_message() {}
		pub fn service_mesaage() {}
    }
    
    pub mod chat {
		pub fn prompt() {}
		pub fn response() {}
    }
    
    pub mod vdeo {
		pub fn in_stream() {}
		pub fn out_stream() {}
    }  

}

pub mod back {
    pub mod mu {
		pub fn mu_agent1() {}
		pub fn mu_agent2() {}
    }
    
    pub mod chat {
		pub fn chat_agent1() {}
		pub fn chat_agent2() {}
    }

    pub mod vdeo {
		pub fn video_agent1() {}
		pub fn video_agent2() {}
    } 
   
    pub mod graph {
		pub fn graph_agent1() {}
		pub fn graph_agent2() {}
    } 
     
    pub mod db {
		pub fn db_agent1() {}
		pub fn db_agent2() {}
    }
    
    pub mod hub {
		pub fn hub_agent1() {}
		pub fn hub_agent2() {}
    } 
        
    pub mod plan {
		pub fn plan_agent1() {}
		pub fn plan_agent2() {}
    }

}
