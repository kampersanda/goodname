use yew::{function_component, html, Properties};

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub candidates: Vec<(String, usize)>,
}

#[function_component(CandView)]
pub fn cand_view(props: &Props) -> Html {
    let Props { candidates } = props.clone();

    html! {
        <table>
            <thead>
                <tr>
                    <td>{"Rank"}</td>
                    <td>{"Candidate"}</td>
                    <td>{"Score"}</td>
                </tr>
            </thead>
            <tbody>
                {
                    for candidates.into_iter().enumerate().map(|(i, (cand, score))| html! {
                        <tr>
                            <td>{i+1}</td>
                            <td><a href={format!("https://www.google.com/search?q={}", cand)} target="_blank">{cand}</a></td>
                            <td>{score}</td>
                        </tr>
                    })
                }
            </tbody>
        </table>
    }
}
