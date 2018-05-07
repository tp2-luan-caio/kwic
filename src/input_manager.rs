pub mod input_manager {
	
	pub trait InputManager {
		 fn new(&mut self);
	    
	    // Get a specific line
   		 fn line(&mut self, index: usize) -> String;

    	 fn length(&mut self) -> usize;
    	// Get the total number of lines
	}
}	