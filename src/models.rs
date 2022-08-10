use std::collections::HashMap;
use std::str::FromStr;
use std::{fs, process::Command};

use sqlite::Connection;

#[derive(Debug, Clone)]
pub enum ServiceTypes {
    Node,
    Python,
    None,
}

#[derive(Debug, Clone)]
pub struct Runner {
    run_type: ServiceTypes,
    exec_path: String,
    entry_point: String,
}

impl Runner {
    fn new(run_type: ServiceTypes, exec_path: String, entry_point: String) -> Self {
        Self {
            run_type,
            exec_path,
            entry_point,
        }
    }

    fn run(&self) -> String {
        match self.run_type {
            ServiceTypes::Node => {
                let mut cmd = Command::new(&self.exec_path);
                cmd.arg(&self.entry_point);
                // TODO: add envs using below
                cmd.env("MSM", "true");

                let run_output = cmd.output().expect("Error");
                let err = run_output.stderr;
                let out = run_output.stdout;
                if err.is_empty() {
                    String::from_utf8(out).unwrap()
                } else {
                    String::from_utf8(err).unwrap()
                }
            }
            ServiceTypes::Python => {
                let mut cmd = Command::new(&self.exec_path);
                cmd.arg(&self.entry_point);
                // TODO: add envs using below
                cmd.env("MSM", "true");

                let run_output = cmd.output().expect("Error");
                let err = run_output.stderr;
                let out = run_output.stdout;
                if err.is_empty() {
                    String::from_utf8(out).unwrap()
                } else {
                    String::from_utf8(err).unwrap()
                }
            }
            _ => String::from("None"),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Service {
    pub name: String,
    pub availible: bool,
    path: String,
    envs: HashMap<String, String>,
    runner: Runner,
}

impl Service {
    fn new(
        name: String,
        availible: bool,
        run: String,
        path: String,
        envs: HashMap<String, String>,
    ) -> Self {
        let runner: Runner;
        let mut entry = path.clone();
        if run == "node" {
            entry.push_str("/index.js");
            runner = Runner::new(ServiceTypes::Node, String::from("node"), entry);
        } else if run == "python" {
            entry.push_str("/main.py");
            runner = Runner::new(ServiceTypes::Python, String::from("python3"), entry);
        } else {
            runner = Runner::new(ServiceTypes::None, String::from(""), String::from(""));
        }

        Self {
            name,
            availible,
            runner,
            path,
            envs,
        }
    }

    pub fn set_envs(&mut self, key: String, value: String) {
        self.envs.insert(key, value);
    }

    pub fn execute(self) -> String {
        self.runner.run()
    }

    // Scans given path to find out what type of service it is
    // Type of services are enum -> ServiceTypes
    pub fn scanner(path: String) -> Service {
        let dir_paths = fs::read_dir(&path).unwrap();
        let folder_path = &path.split('/').collect::<Vec<&str>>();
        let service_name = folder_path.last().unwrap();
        let file_names = dir_paths
            .map(|entry| {
                let entry = entry.unwrap();
                let entry_path = entry.path();
                let file_name = entry_path.file_name().unwrap();
                let file_name_as_str = file_name.to_str().unwrap();
                String::from(file_name_as_str)
            })
            .collect::<Vec<String>>();
        if file_names
            .iter()
            .any(|i| i == "package.json" || i.contains(".js"))
        {
            Service::new(
                String::from(*service_name),
                true,
                String::from("node"),
                path,
                HashMap::new(),
            )
        } else if file_names.iter().any(|i| i.contains(".py")) {
            Service::new(
                String::from(*service_name),
                true,
                String::from("python"),
                path,
                HashMap::new(),
            )
        } else {
            Service::new(
                String::from("blank"),
                false,
                String::from(""),
                String::from(""),
                HashMap::new(),
            )
        }
    }
}

#[derive(Clone)]
pub struct MSM {
    pub services: Vec<Service>,
}

impl MSM {
    pub fn new() -> Self {
        let mut services: Vec<Service> = Vec::new();
        let service_dir = fs::read_dir("./src/services").unwrap();
        for service in service_dir {
            let dir = service.unwrap();
            let dir_str = dir.path();
            let dir_str = dir_str.to_str().unwrap();
            services.push(Service::scanner(String::from_str(dir_str).unwrap()));
        }

        MSM { services }
    }
}
