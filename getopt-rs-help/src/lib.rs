use std::fmt::Debug;

pub trait Generator {
    fn new_sec(&mut self, sec: &str) -> SectionMut;

    fn new_cmd(&mut self, cmd: &str) -> CmdMut;

    fn new_pos(&mut self, cmd: &str, pos: &str) -> CmdOptMut;

    fn new_opt(&mut self, cmd: &str, opt: &str) -> CmdOptMut;

    fn attach_cmd(&mut self, sec: &str, cmd: &str);

    fn add_sec(&mut self, sec: SecStore);

    fn add_cmd(&mut self, cmd: CmdStore);

    fn add_pos(&mut self, cmd: &str, pos: PosStore);

    fn add_opt(&mut self, cmd: &str, opt: OptStore);

    fn get_sec(&self, sec: &str) -> Option<&SecStore>;

    fn get_cmd(&self, cmd: &str) -> Option<&CmdStore>;

    fn get_pos(&self, pos: &str) -> Option<&PosStore>;

    fn get_opt(&self, opt: &str) -> Option<&OptStore>;

    fn get_sec_mut(&mut self, sec: &str) -> Option<&mut SecStore>;

    fn get_cmd_mut(&mut self, cmd: &str) -> Option<&mut CmdStore>;

    fn get_pos_mut(&mut self, pos: &str) -> Option<&mut PosStore>;

    fn get_opt_mut(&mut self, opt: &str) -> Option<&mut OptStore>;

    fn set_simple_style(&mut self, simple: bool);

    fn get_global(&self) -> &CmdStore;

    fn get_global_mut(&mut self) -> &mut CmdStore;

    fn gen_cmd_help(&self, cmd: &str) -> String;

    fn gen_help(&self) -> String;
}

impl<'a> Debug for &'a mut dyn Generator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Generator")
         .finish()
    }
}

#[derive(Debug)]
pub struct SectionMut<'a> {
    g: &'a mut dyn Generator,
    s: SecStore,
}

impl<'a> SectionMut<'a> {
    pub fn new(g: &'a mut dyn Generator, s: &str) -> Self {
        let mut ss = SecStore::default();
        ss.set_name(s);
        Self { g, s: ss }
    }

    pub fn set_name(&mut self, name: &str) -> &mut Self {
        self.s.set_name(name);
        self
    }

    pub fn set_hint(&mut self, hint: &str) -> &mut Self {
        self.s.set_hint(hint);
        self
    }

    pub fn set_help(&mut self, help: &str) -> &mut Self {
        self.s.set_help(help);
        self
    }

    pub fn attach_cmd(&mut self, cmd: &str) -> &mut Self {
        self.s.attach_cmd(cmd);
        self
    }

    pub fn commit(&mut self) {
        self.g.add_sec(self.s.clone());
    }
}

#[derive(Debug)]
pub struct CmdMut<'a> {
    g: &'a mut dyn Generator,
    c: CmdStore,
}

impl<'a> CmdMut<'a> {
    pub fn new(g: &'a mut dyn Generator, c: &str) -> Self {
        let mut cs = CmdStore::default();
        cs.set_name(c);
        Self { g, c: cs }
    }

    pub fn set_name(&mut self, name: &str) -> &mut Self {
        self.c.set_name(name);
        self
    }

    pub fn set_usage(&mut self, help: &str) -> &mut Self {
        self.c.set_usage(help);
        self
    }

    pub fn set_footer(&mut self, help: &str) -> &mut Self {
        self.c.set_footer(help);
        self
    }

    pub fn set_header(&mut self, help: &str) -> &mut Self {
        self.c.set_header(help);
        self
    }

    pub fn set_hint(&mut self, hint: &str) -> &mut Self {
        self.c.set_hint(hint);
        self
    }

    pub fn set_help(&mut self, help: &str) -> &mut Self {
        self.c.set_help(help);
        self
    }

    pub fn add_pos(&mut self, pos: PosStore) -> &mut Self {
        self.c.add_pos(pos);
        self
    }

    pub fn add_opt(&mut self, opt: OptStore) -> &mut Self {
        self.c.add_opt(opt);
        self
    }

    pub fn new_pos(&mut self, pos: &str) -> CmdPosMut {
        CmdPosMut::new(&mut self.c, pos)
    }

    pub fn new_opt(&mut self, opt: &str) -> CmdOptMut {
        CmdOptMut::new(&mut self.c, opt)
    }

    pub fn commit(&mut self) {
        self.g.add_cmd(self.c.clone());
    }
}

#[derive(Debug)]
pub struct CmdPosMut<'a> {
    c: &'a mut CmdStore,
    p: PosStore,
}

impl<'a> CmdPosMut<'a> {
    pub fn new(c: &'a mut CmdStore, p: &str) -> Self {
        let mut ps = PosStore::default();
        ps.set_name(p);
        Self { c, p: ps }
    }

    pub fn set_name(&mut self, name: &str) -> &mut Self {
        self.p.set_name(name);
        self
    }


    pub fn set_hint(&mut self, hint: &str) -> &mut Self {
        self.p.set_hint(hint);
        self
    }

    pub fn set_help(&mut self, help: &str) -> &mut Self {
        self.p.set_help(help);
        self
    }

    pub fn set_optional(&mut self, optional: bool) -> &mut Self {
        self.p.set_optional(optional);
        self
    }

    pub fn set_index(&mut self, index: i64) -> &mut Self {
        self.p.set_index(index);
        self
    }

    pub fn commit(&mut self) {
        self.c.add_pos(self.p.clone());
    }
}

#[derive(Debug)]
pub struct CmdOptMut<'a> {
    c: &'a mut CmdStore,
    o: OptStore,
}

impl<'a> CmdOptMut<'a> {
    pub fn new(c: &'a mut CmdStore, o: &str) -> Self {
        let mut os = OptStore::default();
        os.set_name(o);
        Self { c, o: os }
    }

    pub fn set_name(&mut self, name: &str) -> &mut Self {
        self.o.set_name(name);
        self
    }

    pub fn set_hint(&mut self, hint: &str) -> &mut Self {
        self.o.set_hint(hint);
        self
    }

    pub fn set_help(&mut self, help: &str) -> &mut Self {
        self.o.set_help(help);
        self
    }

    pub fn set_optional(&mut self, optional: bool) -> &mut Self {
        self.o.set_optional(optional);
        self
    }

    pub fn commit(&mut self) {
        self.c.add_opt(self.o.clone());
    }
}

#[derive(Debug, Default, Clone)]
pub struct OptStore {
    name: String,

    hint: String,

    help: String,

    optional: bool,
}

impl OptStore {
    pub fn set_name(&mut self, name: &str) -> &mut Self {
        self.name = name.to_owned();
        self
    }

    pub fn set_hint(&mut self, hint: &str) -> &mut Self {
        self.hint = hint.to_owned();
        self
    }

    pub fn set_help(&mut self, help: &str) -> &mut Self {
        self.help = help.to_owned();
        self
    }

    pub fn set_optional(&mut self, optional: bool) -> &mut Self {
        self.optional = optional;
        self
    }

    pub fn get_name(&self) -> &str {
        self.name.as_str()
    }

    pub fn get_hint(&self) -> &str {
        self.hint.as_str()
    }

    pub fn get_help(&self) -> &str {
        self.help.as_str()
    }

    pub fn get_optional(&self) -> bool {
        self.optional
    }
}

#[derive(Debug, Default, Clone)]
pub struct PosStore {
    name: String,

    hint: String,

    help: String,

    index: i64,

    optional: bool,
}

impl PosStore {
    pub fn set_name(&mut self, name: &str) -> &mut Self {
        self.name = name.to_owned();
        self
    }

    pub fn set_hint(&mut self, hint: &str) -> &mut Self {
        self.hint = hint.to_owned();
        self
    }

    pub fn set_help(&mut self, help: &str) -> &mut Self {
        self.help = help.to_owned();
        self
    }

    pub fn set_optional(&mut self, optional: bool) -> &mut Self {
        self.optional = optional;
        self
    }

    pub fn set_index(&mut self, index: i64) -> &mut Self {
        self.index = index;
        self
    }

    pub fn get_name(&self) -> &str {
        self.name.as_str()
    }

    pub fn get_hint(&self) -> &str {
        self.hint.as_str()
    }

    pub fn get_help(&self) -> &str {
        self.help.as_str()
    }

    pub fn get_optional(&self) -> bool {
        self.optional
    }

    pub fn get_index(&self) -> i64 {
        self.index
    }
}

#[derive(Debug, Default, Clone)]
pub struct CmdStore {
    name: String,

    usage: String,

    footer: String,

    header: String,

    hint: String,

    help: String,

    pos_store: Vec<PosStore>,

    opt_store: Vec<OptStore>,
}

impl CmdStore {
    pub fn set_name(&mut self, name: &str) -> &mut Self {
        self.name = name.to_owned();
        self
    }

    
    pub fn set_usage(&mut self, help: &str) -> &mut Self {
        self.usage = help.to_owned();
        self
    }

    pub fn set_footer(&mut self, help: &str) -> &mut Self {
        self.footer = help.to_owned();
        self
    }

    pub fn set_header(&mut self, help: &str) -> &mut Self {
        self.header = help.to_owned();
        self
    }

    pub fn set_hint(&mut self, hint: &str) -> &mut Self {
        self.hint = hint.to_owned();
        self
    }

    pub fn set_help(&mut self, help: &str) -> &mut Self {
        self.help = help.to_owned();
        self
    }

    pub fn add_pos(&mut self, pos: PosStore) -> &mut Self {
        self.pos_store.push(pos);
        self
    }

    pub fn add_opt(&mut self, opt: OptStore) -> &mut Self {
        self.opt_store.push(opt);
        self
    }

    pub fn get_name(&self) -> &str {
        self.name.as_str()
    }

    pub fn get_hint(&self) -> &str {
        self.hint.as_str()
    }

    pub fn get_help(&self) -> &str {
        self.help.as_str()
    }

    pub fn get_pos(&self, pos: &str) -> Option<&PosStore> {
        self.pos_store.iter().find(|&v| {
            v.name == pos
        })
    }

    pub fn get_opt(&self, opt: &str) -> Option<&OptStore> {
        self.opt_store.iter().find(|&v| {
            v.name == opt
        })
    }
}

#[derive(Debug, Default, Clone)]
pub struct SecStore {
    name: String,

    hint: String,

    help: String,

    cmd_attach: Vec<String>,
}

impl SecStore {
    pub fn set_name(&mut self, name: &str) -> &mut Self {
        self.name = name.to_owned();
        self
    }

    pub fn set_hint(&mut self, hint: &str) -> &mut Self {
        self.hint = hint.to_owned();
        self
    }

    pub fn set_help(&mut self, help: &str) -> &mut Self {
        self.help = help.to_owned();
        self
    }

    pub fn attach_cmd(&mut self, cmd: &str) -> &mut Self {
        self.cmd_attach.push(cmd.to_owned());
        self
    }

    pub fn has_cmd(&self, cmd: &str) -> bool {
        self.cmd_attach.iter().find(|&v| { v == cmd }).is_some()
    }

    pub fn get_name(&self) -> &str {
        self.name.as_str()
    }

    pub fn get_hint(&self) -> &str {
        self.hint.as_str()
    }

    pub fn get_help(&self) -> &str {
        self.help.as_str()
    }
}