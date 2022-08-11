use yew::{function_component, html, Properties};

#[derive(Clone, PartialEq, Eq, Properties)]
pub struct Props {
    pub candidates: Vec<(String, String, usize)>,
}

#[function_component(CandView)]
pub fn cand_view(props: &Props) -> Html {
    let Props { candidates } = props.clone();

    html! {
        <table>
            <thead>
                <tr>
                    <th>{"Rank"}</th>
                    <th>{"Name"}</th>
                    <th>{"Description"}</th>
                    <th>{"Score"}</th>
                    <th>{"Google"}</th>
                    <th>{"GitHub"}</th>
                </tr>
            </thead>
            <tbody>
                {
                    for candidates.into_iter().enumerate().map(|(i, (cand, active, score))| html! {
                        <tr>
                            <td>{i+1}</td>
                            <td class="cand-name">{cand.clone()}</td>
                            <td class="cand-desc">{active}</td>
                            <td>{score}</td>
                            <td><a href={format!("https://www.google.com/search?q={}", cand.to_lowercase())} target="_blank"><span class="material-symbols-outlined">{"search"}</span></a></td>
                            <td><a href={format!("https://github.com/search?q={}", cand.to_lowercase())} target="_blank"><span class="material-symbols-outlined">{"search"}</span></a></td>
                        </tr>
                    })
                }
            </tbody>
        </table>
    }
}
