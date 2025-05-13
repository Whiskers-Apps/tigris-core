#[derive(Debug, Clone)]
pub struct SearchQuery {
    pub keyword: Option<String>,
    pub search_text: String,
}

impl SearchQuery {
    pub fn from(query: &str) -> Self {
        let mut keyword = String::new();
        let mut search_text = String::new();
        let mut has_keyword = false;

        for char in query.chars() {
            if char == ' ' && !has_keyword {
                has_keyword = true;
            } else if !has_keyword {
                keyword += &char.to_string();
            } else {
                search_text += &char.to_string();
            }
        }

        if !has_keyword {
            search_text = keyword.to_owned();
        }

        search_text = search_text.trim().to_owned();

        SearchQuery {
            keyword: match has_keyword {
                true => Some(keyword),
                false => None,
            },
            search_text,
        }
    }
}
