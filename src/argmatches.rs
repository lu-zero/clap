use std::collections::HashMap;

use app::App;
use args::{ FlagArg, OptArg, PosArg };

pub struct ArgMatches {
    pub required: Vec<&'static str>,
    pub blacklist: Vec<&'static str>,
    pub about: Option<&'static str>,
    pub name: &'static str,
    pub author: Option<&'static str>,
    pub version: Option<&'static str>,
    pub flags: HashMap<&'static str, FlagArg>,
    pub opts: HashMap<&'static str, OptArg>,
    pub positionals: HashMap<&'static str, PosArg>,
}

impl ArgMatches {
	pub fn new(name: &'static str) -> ArgMatches {
		ArgMatches {
            flags: HashMap::new(),
            opts: HashMap::new(),
            positionals: HashMap::new(),
    		required: vec![],
    		blacklist: vec![],
    		about: None,
    		name: name,
    		author: None,
    		version: None,
    	}
	}

    pub fn fill_with(&mut self, app: &App) {
        self.about = app.about;
        self.author = app.author;
        self.version = app.version;
    }

	pub fn value_of(&self, name: &'static str) -> Option<&'static str> {
        if let Some(opt) = self.opts.get(&(name)) {
            return opt.value;
        }
        if let Some(pos) = self.positionals.get(name) {
            return pos.value; 
        }
        None
	}

	pub fn is_present(&self, name: &'static str) -> bool {
        if let Some(_) = self.flags.get(&(name)) {
            return true;
        }
        false
	}

    pub fn occurrences_of(&self, name: &'static str) -> u8 {
        if let Some(f) = self.flags.get(&(name)) {
            return f.occurrences;
        }
        0
    }
}