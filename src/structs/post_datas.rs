use chrono::{Local, NaiveDate};

pub struct PostDatas {
    pub name: String,
    pub title: String,
    pub date: NaiveDate,
}

impl PostDatas {
    pub fn new() -> Self {
        PostDatas {
            name: String::new(),
            title: String::new(),
            date: Local::now().date_naive(),
        }
    }

    pub fn sort_by_date(vec_posts: &mut Vec<PostDatas>) {
        loop {
            let mut exchanged = false;

            for n in 0..vec_posts.len() {
                let current = vec_posts.get(n).unwrap();

                if vec_posts.get(n + 1).is_none() {
                    continue;
                }

                let next = vec_posts.get(n + 1).unwrap();

                if current.date < next.date {
                    vec_posts.swap(n, n + 1);
                    exchanged = true;
                }
            }

            if !exchanged {
                break;
            }
        }
    }
}
