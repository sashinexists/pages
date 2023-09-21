use std::path::PathBuf;

#[derive(Debug, Clone)]
pub enum PageData {
    None,
    Projects(Projects),
    Testimonials(Testimonials),
}

// there is an empty PageData enum (with only None as an option) here by default, everything else is custome

#[derive(Debug, Clone)]
pub struct Home {
    testimonials: Testimonials,
    past_projects: Projects,
    skills: Skills,
}

#[derive(Debug, Clone)]
pub struct Testimonials(Vec<Testimonial>);

#[derive(Debug, Clone)]
pub struct Testimonial {
    author: Person,
    text: String,
    slug: String,
}

#[derive(Debug, Clone)]
pub struct Image {
    src: PathBuf,
    alt: String,
}

#[derive(Debug, Clone)]
pub struct Person {
    name: String,
    title: String,
    photo: Image,
    website: Option<PathBuf>,
    organisation: String,
}

#[derive(Debug, Clone)]
pub struct Projects(Vec<Projects>);

#[derive(Debug, Clone)]
pub struct Project {
    title: String,
    screenshot: Image,
    github_url: Option<PathBuf>,
    description: String,
    about: String,
    testimonial: Option<Testimonial>,
    website_url: Option<PathBuf>,
    skills: Vec<Skill>,
    slug: String,
}

#[derive(Debug, Clone)]
struct Skills(Vec<Skill>);

#[derive(Debug, Clone)]
struct Skill {
    name: String,
    description: String,
    thumbnail: Image,
    about: String,
    slug: String,
}

#[derive(Debug, Clone)]
struct BlogPosts(Vec<BlogPost>);

#[derive(Debug, Clone)]
struct BlogPost {
    title: String,
    slug: String,
    content: String,
}
