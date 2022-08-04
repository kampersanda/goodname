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
                    <td>{"Candidate"}</td>
                    <td>{"Score"}</td>
                </tr>
            </thead>
            <tbody>
                {
                    for candidates.into_iter().map(|(cand, score)| html! {
                        <tr>
                            <td>{cand}</td>
                            <td>{score}</td>
                        </tr>
                    })
                }
            </tbody>
        </table>
    }
}
