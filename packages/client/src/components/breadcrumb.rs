use yew::prelude::*;
use yew_icons::{Icon, IconId};

#[derive(PartialEq, Properties)]
pub struct Props {
    pub items: Vec<&'static str>,
}

#[function_component(Breadcrumb)]
pub fn breadcrumb(props: &Props) -> Html {
    html! {
        <nav>
            <ol class="flex items-center space-x-2">
                {
                    props.items.iter().enumerate().map(|(index, item)| {
                        html_nested! {
                            <>
                                if index > 0 {
                                    <Icon icon_id={IconId::HeroiconsOutlineChevronRight} class={"w-4 h-4"} />
                                }
                                <li>
                                    <div class="flex items-center">
                                        <a href="#" class="text-xl font-medium text-gray-700 hover:text-gray-900 dark:text-gray-400 dark:hover:text-white">{item}</a>
                                    </div>
                                </li>
                            </>

                        }
                    }).collect::<Html>()
                }
            </ol>
        </nav>
    }
}
