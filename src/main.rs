use yew::prelude::*;

struct AnimationCanvas {
    canvas: NodeRef
}

pub enum AnimationCanvasMsg {

}

impl Component for AnimationCanvas {
    type Message = AnimationCanvasMsg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            canvas: NodeRef::default(),
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div>
                <canvas
                    id="canvas"
                    ref={self.canvas.clone()}>
                </canvas>
            </div>
        }
    }
}

#[function_component(App)]
fn app_body() -> Html {
    html! {
        <>
            <AnimationCanvas/>
        </>
    }
}


fn main() {
    yew::Renderer::<App>::new().render();
}