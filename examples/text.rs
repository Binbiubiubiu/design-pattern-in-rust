pub struct Article {
    status: Option<Box<dyn Status>>,
    body: String,
}


impl Article {
    pub fn new() -> Article {
       Article { 
           status:Some(Box::new(Draft{})),
           body: String::new(),
       }
    }

    pub fn write(&mut  self,body_text:&str){
        self.body.push_str(body_text);
    }

    pub fn request_next(&mut self){
        if let Some(s) =self.status.take(){
            self.status =Some(s.request_next());
        }
    }

    pub fn view(&self)->&str{
        self.status.as_ref().unwrap().view(self)
    }

    pub fn self_view(&self)->&str{
        self.status.as_ref().unwrap().self_view(self)
    }
}

trait Status{
    fn view<'a>(&self,_:&'a Article)->&'a str{
        ""
    }
    fn self_view<'a>(&self,article:&'a Article)->&'a  str{
        &article.body
    }
    fn request_next(self:Box<Self>)->Box<dyn Status>;
}

struct Draft {}

impl Status for Draft {
    fn request_next(self:Box<Self>) ->Box<dyn Status> {
        Box::new(Opened{})
    }
}



struct Opened {}

impl Status for Opened {
    fn request_next(self:Box<Self>) ->Box<dyn Status> {
        self
    }

    fn view<'a>(&self, article:&'a Article) ->&'a str {
        &article.body
    }
}

fn main() {
    let mut a = Article::new();
    a.write("原子之音");
    // println!("草稿:{}",a.view());
    // a.request_next();
    // println!("已发布:{}",a.view());

    println!("草稿: {}",a.self_view());
    a.request_next();
    println!("已发布:{}",a.self_view());

}
