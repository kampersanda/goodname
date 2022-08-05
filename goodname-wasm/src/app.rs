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

#[derive(Debug)]
pub enum MatchCase {
    NotYet,
    Within,
    Overflow,
    TooMany,
}

impl Default for MatchCase {
    fn default() -> Self {
        Self::NotYet
    }
}

#[derive(Debug, Default)]
pub struct App {
    text: String,
    match_case: MatchCase,
    num_matched: usize,
    candidates: Vec<(String, usize)>,
}

impl App {
    fn gen_candidates(&mut self) {
        if self.text.is_empty() {
            self.candidates = vec![];
        } else {
            let matched = Enumerator::all_subsequences_sorted(&LEXICON, &self.text);
            if let Ok(matched) = matched {
                self.num_matched = matched.len();
                if self.num_matched <= 100 {
                    self.match_case = MatchCase::Within;
                } else {
                    self.match_case = MatchCase::Overflow;
                }
                self.candidates = matched[..matched.len().min(100)]
                    .iter()
                    .map(|m| (LEXICON.word(m.word_id).to_string(), m.score))
                    .collect();
            } else {
                self.match_case = MatchCase::TooMany;
            }
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
                </header>
                <main>
                    <div class="description">
                        <div>
                            <p class="header-link"><a href="https://github.com/kampersanda/goodname" target="_blank">{"[Project Page]"}</a></p>
                        </div>
                        <h2>{"What is this?"}</h2>
                        <div>
                            {"Given a brief description of your method or software, this tool enumerates name candidates forming subsequences of the description, i.e., "}
                            <i>{"abbreviation."}</i>
                        </div>
                        <div>
                            {"(e.g., \"Character wise double array dictionary\" ⇒ \"crawdad\", \"cheddar\", and so on.)"}
                        </div>
                        <h2>{"How to use?"}</h2>
                        <div>
                            {"Enter your description using only lowercase letters or spaces basically. Set UPPERCASE only for letters that you want to be always included in a name candidate."}
                        </div>
                        <div>
                            {"(e.g., when entering \"Character wise Double array Dictionary\", subsequence ('C','D','D') is always included in the candidates.)"}
                        </div>
                        <h2>{"How to rank?"}</h2>
                        <div>
                            {"The name candidates are shown in score order. The scores are assigned based on the following ideas:"}
                            <ul>
                                <li>{"The more forward letters of each word in a description, the higher the score."}</li>
                                <li>{"The more letters matched, the higher the score."}</li>
                            </ul>
                        </div>
                    </div>
                    <div class="entry">
                        <h2>{"Let's try!"}</h2>
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
                        match self.match_case {
                            MatchCase::NotYet => html! {},
                            MatchCase::Within => html! {
                                <div class="candidates">
                                    <div class="nummatches">
                                        {format!("#matches = {}", num_matched)}
                                    </div>
                                    <CandView {candidates} />
                                </div>
                            },
                            MatchCase::Overflow => html! {
                                <div class="candidates">
                                    <div class="nummatches">
                                        {format!("#matches = {} (the top-100 candidates are printed)", num_matched)}
                                    </div>
                                    <CandView {candidates} />
                                </div>
                            },
                            MatchCase::TooMany => html! {
                                <div class="toomany">
                                    {"The search was forcibly terminated because #matches was too many. Adjust the number by shortening the description or specifying more uppercase letters."}
                                </div>
                            },
                        }
                    }
                </main>
                <footer>
                    {"© 2022 Shunsuke Kanda"}
                </footer>
            </>
        }
    }
}
