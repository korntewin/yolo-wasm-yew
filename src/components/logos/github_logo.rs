use stylist::yew::styled_component;
use yew::prelude::*;

#[styled_component(GithubLogo)]
pub fn github_logo() -> Html {
    html!(
        <a href="https://github.com/korntewin/" target="_blank" rel="noopener noreferrer">
           <img
               class={css!("
                        height: 50px;
               ")}
               src={"https://encrypted-tbn0.gstatic.com/images?q=tbn:ANd9GcQZkTHDk9wFCnizM9J7jS8FQkSQkY3BPG_HvnkdetOYXw&s"}
           />
        </a>
    )
}
