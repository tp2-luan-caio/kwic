pub mod imput_manager {
	

	pub trait InputManager {
		 fn new(&mut self);
	    
	    // Get a specific line
   		 fn line(&mut self, index: i32) -> String;

    	 fn length(&mut self) -> i32;
    	// Get the total number of lines
	}
}	