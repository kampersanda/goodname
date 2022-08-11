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
                </tr>
            </thead>
            <tbody>
                {
                    for candidates.into_iter().enumerate().map(|(i, (cand, active, score))| html! {
                        <tr>
                            <td>{i+1}</td>
                            <td class="cand-name"><a href={format!("https://www.google.com/search?q={}", cand.to_lowercase())} target="_blank">{cand}</a></td>
                            <td class="cand-desc">{active}</td>
                            <td>{score}</td>
                        </tr>
                    })
                }
            </tbody>
        </table>
    }
}
