use anyhow::Result;
use goodname::{Enumerator, Lexicon};
use once_cell::sync::Lazy;
use yew::prelude::*;

use crate::cand_view::CandView;
use crate::range_input::RangeInput;
use crate::text_input::TextInput;

static LEXICON: Lazy<Lexicon> = Lazy::new(|| {
    let words = include_str!("words.txt");
    Lexicon::new(words.split('\n').filter(|w| !w.is_empty())).unwrap()
});

pub enum Msg {
    SetText(String),
    SetPrefixLen(String),
    GenCandidates,
}

#[derive(Debug)]
pub enum MatchCase {
    NotYet,
    Within,
    Overflow,
    Error(String),
}

impl Default for MatchCase {
    fn default() -> Self {
        Self::NotYet
    }
}

#[derive(Debug)]
pub struct App {
    text: String,
    prefix_len: String,
    match_case: MatchCase,
    num_matched: usize,
    candidates: Vec<(String, String, usize)>,
}

impl Default for App {
    fn default() -> Self {
        Self {
            text: "".to_string(),
            prefix_len: "0".to_string(),
            match_case: MatchCase::NotYet,
            num_matched: 0,
            candidates: vec![],
        }
    }
}

impl App {
    fn gen_candidates(&mut self) {
        if self.text.is_empty() {
            self.match_case = MatchCase::NotYet;
            self.num_matched = 0;
            self.candidates = vec![];
            return;
        }
        match self.enumurate() {
            Ok(_) => {}
            Err(e) => {
                self.match_case = MatchCase::Error(e.to_string());
            }
        }
    }

    fn enumurate(&mut self) -> Result<()> {
        let enumerator = Enumerator::init(&LEXICON, &self.text)?;
        let enumerator = enumerator.prefix_len(self.prefix_len.parse()?)?;
        let matched = enumerator.all_subsequences()?;
        self.num_matched = matched.len();
        if self.num_matched <= 100 {
            self.match_case = MatchCase::Within;
        } else {
            self.match_case = MatchCase::Overflow;
        }
        self.candidates = matched[..matched.len().min(100)]
            .iter()
            .map(|m| {
                let (word, desc) = enumerator.format_match(m);
                (word, desc, m.score)
            })
            .collect();
        Ok(())
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
            Msg::SetPrefixLen(prefix_len) => self.prefix_len = prefix_len,
            Msg::GenCandidates => self.gen_candidates(),
        };
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
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
                            <TextInput on_change={ctx.link().callback(Msg::SetText)} value={self.text.clone()} name="yourdesc" />
                        </div>
                        <div>
                            {format!("Set the maximum number of don't care prefix letters: ")}
                        </div>
                            <label class="range" for="prefix">{self.prefix_len.clone()}</label>
                            <RangeInput on_change={ctx.link().callback(Msg::SetPrefixLen)} value={self.prefix_len.clone()} name="prefix" />
                        <div>
                        </div>
                        <div>
                            <button onclick={ctx.link().callback(|_| Msg::GenCandidates)}>
                                {"Search"}
                            </button>
                        </div>
                    </div>
                    {
                        match &self.match_case {
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
                                        {format!("#matches = {} (Only the top-100 candidates are printed.)", num_matched)}
                                    </div>
                                    <CandView {candidates} />
                                </div>
                            },
                            MatchCase::Error(e) => html! {
                                <div class="error">
                                    {format!("The search was forcibly terminated because {}", e)}
                                </div>
                            },
                        }
                    }
                </main>
                <footer>
                    {"© 2022 Shunsuke Kanda (Kampersanda)"}
                </footer>
            </>
        }
    }
}
