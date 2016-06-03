extern crate xml_writer;
extern crate treexml;

use std::io::{Write, stdin, stdout};
use std::collections::HashMap;
use std::fmt;
use std::fs::File;

use xml_writer::XmlWriter;
use treexml::Document;

struct Module {
    name: String,
    inputs: Vec<String>,
    outputs: Vec<String>,
}

impl Module {
    fn new(s: &str) -> Module {
        Module {
            name: String::from(s),
            inputs: Vec::new(),
            outputs: Vec::new(),
        }
    }

    fn add_input(&mut self, s: &str) {
        self.inputs.push(String::from(s));
    }

    fn add_output(&mut self, s: &str) {
        self.outputs.push(String::from(s));
    }

    fn name(&self) -> &str {
        self.name.as_str()
    }

    fn inputs(&self) -> &Vec<String> {
        &self.inputs
    }

    fn outputs(&self) -> &Vec<String> {
        &self.outputs
    }
}

impl fmt::Display for Module {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} ", self.name);
        write!(f, "{{");
        for i in 0..self.inputs.len() {
            write!(f,
                   "{}{}",
                   self.inputs[i],
                   if i == self.inputs.len() - 1 {
                       ""
                   } else {
                       ","
                   });
        }
        write!(f, "}} -> {{");
        for i in 0..self.outputs.len() {
            write!(f,
                   "{}{}",
                   self.outputs[i],
                   if i == self.outputs.len() - 1 {
                       ""
                   } else {
                       ","
                   });
        }
        write!(f, "}}")
    }
}

struct State {
    modules: Vec<Module>,
}

impl State {
    fn new() -> State {
        State { modules: Vec::new() }
    }

    fn add(&mut self, m: Module) {
        self.modules.push(m);
    }

    fn modules(&self) -> &Vec<Module> {
        &self.modules
    }
}

fn prompt(s: &str) -> Vec<String> {
    print!("{} ", s);
    stdout().flush().unwrap();

    let mut line = String::new();
    stdin().read_line(&mut line).unwrap();

    line.trim().split(" ").map(|s| String::from(s)).collect()
}

fn main() {
    let mut cmds: HashMap<&str, Box<Fn(&mut State)>> = HashMap::new();
    cmds.insert("create",
                Box::new(|st| {
                    let name = prompt("name: ");
                    let inputs = prompt("# of inputs:");
                    let outputs = prompt("# of outputs:");

                    let mut m = Module::new(name.first().unwrap());

                    let n_ins = inputs.first().unwrap().parse::<u32>().unwrap();
                    for i in 0..n_ins {
                        let input = prompt(format!("input {}:", i + 1).as_str());
                        m.add_input(input.first().unwrap().as_str());
                    }

                    let n_ins = outputs.first().unwrap().parse::<u32>().unwrap();
                    for i in 0..n_ins {
                        let output = prompt(format!("output {}:", i + 1).as_str());
                        m.add_output(output.first().unwrap().as_str());
                    }

                    st.add(m);
                }));

    cmds.insert("list",
                Box::new(|st| {
                    for m in st.modules() {
                        println!("{}", m);
                    }
                }));

    cmds.insert("save",
                Box::new(|st| {
                    let mut xml = XmlWriter::new(File::create("data.xml").unwrap());
                    xml.begin_elem("modules");
                    for m in st.modules() {
                        xml.begin_elem("module");
                        xml.begin_elem("name");
                        xml.text(m.name());
                        xml.end_elem();
                        xml.begin_elem("inputs");
                        for i in m.inputs() {
                            xml.begin_elem("input");
                            xml.text(i);
                            xml.end_elem();
                        }
                        xml.end_elem();
                        xml.begin_elem("outputs");
                        for o in m.outputs() {
                            xml.begin_elem("output");
                            xml.text(o);
                            xml.end_elem();
                        }
                        xml.end_elem();
                        xml.end_elem();
                    }
                    xml.end_elem();
                    xml.close();
                    xml.flush();
                }));

    cmds.insert("load",
                Box::new(|st| {
                    let f = File::open("data.xml").unwrap();
                    let d = Document::parse(f).unwrap();
                    let root = d.root.unwrap();
                    for m in root.children {
                        if m.name == "module" {
                            let mut nm = Module::new(m.find_child(|t| t.name == "name")
                                                  .unwrap()
                                                  .clone()
                                                  .text
                                                  .unwrap()
                                                  .as_str());
                                                  
                            for i in m.find_child(|t| t.name == "inputs").unwrap().clone().children {
                                nm.add_input(i.text.unwrap().as_str());
                            }
                                                  
                            for o in m.find_child(|t| t.name == "outputs").unwrap().clone().children {
                                nm.add_output(o.text.unwrap().as_str());
                            }
                            
                            st.add(nm);
                        }
                    }
                }));

    let mut state = State::new();
    loop {
        let v = prompt(">");
        if let Some(s) = v.first() {
            if s == "quit" {break;}
            if let Some(cmd) = cmds.get(s.as_str()) {
                cmd(&mut state);
            } else {
                println!("invalid command");
            }
        }
    }
}
