use rocket::serde::{Serialize};

pub trait IContextData {}

#[derive(Serialize)]
pub struct Context<T: IContextData>{
    pub user_id: String,
    pub info: T
}

impl Context<IndexContext>{
    pub fn index(user_id: String) -> Context<IndexContext>{
        Context{
            user_id: user_id,
            info: IndexContext {}
        }
    }
}

impl Context<LoginContext>{
    pub fn login(user_id: String, message: String) -> Context<LoginContext>{
        Context{
            user_id: user_id,
            info: LoginContext {
                message: message
            }
        }
    }
}

impl Context<ProjectContext>{
    pub fn project(user_id: String, paths: Vec<String>, files_names: Vec<String>, project_name: String, project_desc: String) -> Context<ProjectContext>{
        Context{
            user_id: user_id,
            info: ProjectContext { 
                paths: paths, 
                file_names: files_names, 
                project_name: project_name, 
                project_desc: project_desc 
            }
        }
    }
}

#[derive(Serialize)]
pub struct IndexContext {}

impl IContextData for IndexContext {}

#[derive(Serialize)]
pub struct LoginContext {
    //user_id: String,
    pub message: String
}

impl IContextData for LoginContext {}

#[derive(Serialize)]
pub struct AboutContext{
    //user_id: String
}

impl IContextData for AboutContext {}

#[derive(Serialize)]
pub struct ProjectContext{
    pub paths: Vec<String>,
    pub file_names: Vec<String>,
    pub project_name: String,
    pub project_desc: String
}

impl IContextData for ProjectContext {}