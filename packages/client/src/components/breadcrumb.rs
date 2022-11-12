use yew::prelude::*;

#[derive(PartialEq, Properties)]
pub struct Props {
    pub items: Vec<&'static str>,
}

pub struct Breadcrumb;

impl Component for Breadcrumb {
    type Properties = Props;
    type Message = ();

    fn create(ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <nav>
                <ol>
                    <li>
                              <div class="flex items-center">
                                <svg class="w-6 h-6 text-gray-400" fill="currentColor" viewBox="0 0 20 20" xmlns="http://www.w3.org/2000/svg"><path fill-rule="evenodd" d="M7.293 14.707a1 1 0 010-1.414L10.586 10 7.293 6.707a1 1 0 011.414-1.414l4 4a1 1 0 010 1.414l-4 4a1 1 0 01-1.414 0z" clip-rule="evenodd"></path></svg>
                    <a href="#" class="text-xl ml-1 font-medium text-gray-700 hover:text-gray-900 md:ml-2 dark:text-gray-400 dark:hover:text-white">{"projects"}</a>
                  </div>
                    </li>
                </ol>
            </nav>
        }
    }
}
