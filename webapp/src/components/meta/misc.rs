use recordkeeper::flags::FlagType;
use strum::{EnumIter, FromRepr};
use ybc::{Control, Field, Tile, Title};
use yew::prelude::*;

use crate::components::edit::{CheckboxInput, Editor, EnumInput, FlagEditor, ToBool};
use crate::data::Data;
use crate::lang::Text;
use crate::ToHtml;

#[derive(EnumIter, FromRepr, Clone, Copy, PartialEq)]
#[repr(u32)]
enum Difficulty {
    Easy = 1,
    Normal = 0,
    Hard = 2,
    VeryHard = 3,
}

#[derive(Clone, Copy, PartialEq)]
struct DifficultyEditor(FlagEditor);

#[function_component]
pub fn Settings() -> Html {
    let data = use_context::<Data>().unwrap();
    let flags = &data.game().manual.flags;

    let ngp_editor = ToBool(flags.new_game_plus.into());
    let game_clear_editor = ToBool(flags.game_clear.into());
    let difficulty_editor = DifficultyEditor(flags.difficulty.into());

    html! {
        <Tile classes={classes!("is-child", "notification")}>
            <Title><Text path="meta_settings" /></Title>

            <Field>
                <label class="label"><Text path="difficulty" /></label>
                <Control>
                    <EnumInput<DifficultyEditor> editor={difficulty_editor} />
                </Control>
            </Field>

            <Field>
                <Control>
                    <CheckboxInput<ToBool<FlagEditor>> editor={ngp_editor}>
                        {" "}<Text path="meta_ngp" />
                    </CheckboxInput<ToBool<FlagEditor>>>
                </Control>
            </Field>

            <Field>
                <Control>
                    <CheckboxInput<ToBool<FlagEditor>> editor={game_clear_editor}>
                        {" "}<Text path="meta_clear" />
                    </CheckboxInput<ToBool<FlagEditor>>>
                </Control>
            </Field>
        </Tile>
    }
}

impl Editor for DifficultyEditor {
    type Target = Difficulty;

    fn get(&self, save: &recordkeeper::SaveData) -> Self::Target {
        Difficulty::from_repr(self.0.get(save)).expect("unknown difficulty")
    }

    fn set(&self, save: &mut recordkeeper::SaveData, new: Self::Target) {
        self.0.set(save, new as u32);
    }
}

impl ToHtml for Difficulty {
    fn to_html(&self) -> Html {
        let id = match self {
            Difficulty::Easy => "easy",
            Difficulty::Normal => "normal",
            Difficulty::Hard => "hard",
            Difficulty::VeryHard => "veryhard",
        };
        html!(<Text path={format!("difficulty_{id}")} />)
    }
}
