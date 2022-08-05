use goodname::{Enumerator, Lexicon};
use once_cell::sync::Lazy;
use yew::prelude::*;

use crate::cand_view::CandView;
use crate::text_input::TextInput;

static LEXICON: Lazy<Lexicon> = Lazy::new(|| {
    let words = include_str!("../../wordlist/words.txt");
    Lexicon::new(words.split("\n").filter(|w| !w.is_empty())).unwrap()
});

pub enum Msg {
    SetText(String),
    GenCandidates,
}

#[derive(Debug, Default)]
pub struct App {
    text: String,
    num_matched: usize,
    candidates: Option<Vec<(String, usize)>>,
}

impl App {
    fn gen_candidates(&mut self) {
        if self.text.is_empty() {
            self.candidates = None;
        } else {
            let matched = Enumerator::all_subsequences_sorted(&LEXICON, &self.text).unwrap();
            self.num_matched = matched.len();
            self.candidates = Some(
                matched[..matched.len().min(100)]
                    .iter()
                    .map(|m| (LEXICON.word(m.word_id).to_string(), m.score))
                    .collect(),
            );
        }
    }
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Lazy::force(&LEXICON);
        Self::default()
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::SetText(text) => self.text = text,
            Msg::GenCandidates => self.gen_candidates(),
        };
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let on_change = ctx.link().callback(Msg::SetText);
        let onclick = ctx.link().callback(|_| Msg::GenCandidates);

        let num_matched = self.num_matched;
        let candidates = self.candidates.clone();

        html! {
            <>
                <header>
                    <h1>{"Goodname: Tool to assist you with cool naming of your methods and software"}</h1>
                    <p class="header-link"><a href="https://github.com/kampersanda/goodname">{"[Project Page]"}</a></p>
                </header>
                <main>
                    <div class="entry">
                        <div>
                            {"Enter a brief description of your method or software:"}
                        </div>
                        <div>
                            <TextInput {on_change} value={self.text.clone()} />
                        </div>
                        <button {onclick}>
                            {"Search"}
                        </button>
                    </div>
                    {
                        if let Some(candidates) = candidates {
                            html! {
                                <div class="candidates">
                                    {format!("#matched = {}", num_matched)}
                                    <CandView {candidates} />
                                </div>
                            }
                        } else {
                            html! {}
                        }
                    }
                </main>
            </>
        }
    }
}
