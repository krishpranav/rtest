use std::collections::HashMap;
use std::{fmt::Display, rc::Rc, cell::RefCell};
use crate::spec::{Spec, SpecContext, SpecOptions};
use crate::suite::{Suite};

pub type State<T> = HashMap<&'static str, T>;

pub struct SuiteContext<T> {
    pub state: Rc<RefCell<State<T>>>,
    pub after_all_hook: Option<Rc<dyn Fn(&mut State<T>) + 'static>>,
    pub after_each_hook: Option<Rc<dyn Fn(&mut State<T>) + 'static>>,
    pub before_all_hook: Option<Rc<dyn Fn(&mut State<T>) + 'static>>,
    pub before_each_hook: Option<Rc<dyn Fn(&mut State<T>) + 'static>>,
    pub specs: Vec<Spec<T>>,
    pub suites: Vec<Suite<T>>,
    pub retries_: Option<u32>,
    pub skip_: bool,
    pub slow_: Option<u128>,
    pub passed: u32,
    pub failed: u32,
    pub ignored: u32,
    pub fail: bool
}

impl<T> SuiteContext<T> {
    pub fn new() -> SuiteContext<T> {
        SuiteContext {
            state: Rc::new(RefCell::new(HashMap::new())),
            after_all_hook: None,
            after_each_hook: None,
            before_all_hook: None,
            before_each_hook: None,
            specs: vec![],
            suites: vec![],
            retries_: None,
            skip_: false,
            slow_: None,
            passed: 0,
            failed: 0,
            ignored: 0,
            fail: false
        }
    }

    pub fn before_all<H: Fn(&mut State<T>) + 'static>(&mut self, hook: H) -> &mut Self {
        self.before_all_hook = Some(Rc::new(hook));
        self
    }

    pub fn before_each<H: Fn(&mut State<T>) + 'static>(&mut self, hook: H) -> &mut Self {
        self.before_each_hook = Some(Rc::new(hook));
        self
    }

    pub fn after_all<H: Fn(&mut State<T>) + 'static>(&mut self, hook: H) -> &mut Self {
        self.after_all_hook = Some(Rc::new(hook));
        self
    }

    pub fn after_each<H: Fn(&mut State<T>) + 'static>(&mut self, hook: H) -> &mut Self {
        self.after_each_hook = Some(Rc::new(hook));
        self
    }

    pub fn it<S, H>(&mut self, name: S, hook: H) -> &mut Self
    where
        S: Into<String> + Display,
        H: Fn(&mut SpecContext<T>) -> Result<(), String> + 'static
    {
        self.specs.push(Spec::new(name.to_string(), self.state.clone(), Box::new(hook)));
        self
    }

    pub fn it_skip<S, H>(&mut self, name: S, hook: H) -> &mut Self
    where
        S: Into<String> + Display,
        H: Fn(&mut SpecContext<T>) -> Result<(), String> + 'static
    {
        let mut spec = Spec::new(name.to_string(), self.state.clone(), Box::new(hook));
        spec.skip = true;
        self.specs.push(spec);
        self
    }

    pub fn it_only<S, H>(&mut self, name: S, hook: H) -> &mut Self
    where
        S: Into<String> + Display,
        H: Fn(&mut SpecContext<T>) -> Result<(), String> + 'static
    {
        let mut spec = Spec::new(name.to_string(), self.state.clone(), Box::new(hook));
        spec.only = true;
        self.specs.push(spec);
        self
    }

    pub fn spec<H>(&mut self, cb: H) -> &mut Self
    where
        H: Fn(&mut SpecOptions<T>)
    {
        let mut options: SpecOptions<T> = SpecOptions::new();
        (cb)(&mut options);
        match options.name {
            Some(name) => match options.hook {
                Some(hook) => {
                    let mut spec = Spec::new(name.to_string(), self.state.clone(), hook);
                    spec.context.retries_ = options.retries_;
                    spec.context.slow_ = options.slow_;
                    self.specs.push(spec);
                },
                None => { }
            },
            None => {  }
        }

        self
    }

    pub fn describe<S, H>(&mut self, name: S, cb: H) -> &mut Self
    where
        S: Into<String> + Display,
        H: Fn(&mut SuiteContext<T>) + 'static
    {
        let mut suite = Suite::new(name, cb);

        suite.context.state = self.state.clone();

        self.suites.push(suite);
        self
    }

    pub fn describe_skip<S, H>(&mut self, name: S, cb: H) -> &mut Self
    where
        S: Into<String> + Display,
        H: Fn(&mut SuiteContext<T>) + 'static
    {
        let mut suite = Suite::new(name, cb);

        suite.context.state = self.state.clone();
        suite.context.skip_ = true;

        self.suites.push(suite);
        self
    }

    pub fn describe_only<S, H>(&mut self, name: S, cb: H) -> &mut Self
    where
        S: Into<String> + Display,
        H: Fn(&mut SuiteContext<T>) + 'static
    {
        let mut suite = Suite::new(name, cb);
        suite.context.state = self.state.clone();
        suite.only = true;

        self.suites.push(suite);
        self
    }
    pub fn describe_import(&mut self, mut suite: Suite<T>) -> &mut Self {
        suite.context.state = self.state.clone();
        self.suites.push(suite);
        self
    }

    pub fn describe_import_skip(&mut self, mut suite: Suite<T>) -> &mut Self {
        suite.context.state = self.state.clone();
        suite.context.skip_ = true;
        self.suites.push(suite);
        self
    }

    pub fn describe_import_only(&mut self, mut suite: Suite<T>) -> &mut Self {
        suite.context.state = self.state.clone();
        suite.only = true;
        self.suites.push(suite);
        self
    }

    pub fn skip(&mut self) -> &mut Self {
        self.skip_ = true;
        self
    }

    pub fn retries(&mut self, count: u32) -> &mut Self {
        self.retries_ = Some(count);
        self
    }

    pub fn slow(&mut self, count: u128) -> &mut Self {
        self.slow_ = Some(count);
        self
    }
}