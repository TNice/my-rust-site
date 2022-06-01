use rocket_dyn_templates::Template;
use rocket::response::{Redirect, Responder, Response};
use rocket::request::{Request};
use rocket::http::{Status};

pub struct AnyResponse {
    res_type: &'static str,
    template: Option<Template>,
    redirect: Option<Redirect>
}

impl AnyResponse{
    fn get_default() -> Self {
        AnyResponse{ res_type: "", template: None, redirect: None }
    }
    pub fn template (temp: Template) -> Self {
        let mut res = AnyResponse::get_default();
        res.res_type = "template";
        res.template = Some(temp);
        return res
    }
    pub fn redirect (redi: Redirect) -> Self {
        let mut res = AnyResponse::get_default();
        res.res_type = "redirect";
        res.redirect = Some(redi);
        return res
    }
}

impl<'r> Responder<'r, 'r> for AnyResponse {
    fn respond_to(self, req: &Request) -> Result<Response<'r>, Status> {
        if self.res_type == "template" {
            return Some(self.template).respond_to(req)
        }
        return Some(self.redirect).respond_to(req)
    }
}