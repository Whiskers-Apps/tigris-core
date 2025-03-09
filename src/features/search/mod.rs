#[derive(Debug, Clone)]
pub struct SearchQuery {
    pub keyword: Option<String>,
    pub search_text: String,
}

/// Returns an object containing the keyword and the search text
pub fn get_search_query(input: &str) -> SearchQuery {
    let mut keyword = String::new();
    let mut search_text = String::new();
    let mut has_keyword = false;

    for char in input.chars() {
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