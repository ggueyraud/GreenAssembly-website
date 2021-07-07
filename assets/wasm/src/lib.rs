use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{HtmlElement, NodeList, Window};
use js_sys::{Function, Map};
use std::{cell::RefCell, rc::Rc, boxed::Box};

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);

    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen]
#[derive(Clone)]
pub enum Event {
    StepChange,
    Init
}

impl std::convert::Into<JsValue> for Event {
    fn into(self) -> JsValue {
        match self {
            StepChange => JsValue::from_str("step_change"),
            Init => JsValue::from_str("init")
        }
    }
}

#[wasm_bindgen]
pub struct Stepper {
    container: Rc<HtmlElement>,
    wrapper: Rc<HtmlElement>,
    steps: NodeList,
    current_step: Rc<RefCell<u32>>,
    events: Rc<RefCell<Map>>
}

#[wasm_bindgen]
impl Stepper {
    pub fn new(container: HtmlElement) -> Self {
        let container = Rc::new(container);
        let wrapper = container.query_selector(".stepper__wrapper").unwrap().expect("Can't find wrapper element");
        let wrapper = wrapper.dyn_ref::<HtmlElement>().unwrap();
        let steps = wrapper.query_selector_all(".stepper__wrapper__step").expect("No steps");
        let current_step = Rc::new(RefCell::new(0));
        let events = Rc::new(RefCell::new(Map::new()));

        let ratio = steps.length() / 1;
        wrapper.style().set_property("width", &format!("{}%", ratio * 100));
        container.class_list().add_1("stepper--ready");

        let mut index: u32 = 0;
        let container_width = container.get_bounding_client_rect().width() as u32;
        while index < steps.length() {
            let step = steps.get(index).unwrap();
            let step = step.dyn_ref::<HtmlElement>().unwrap();

            step.style().set_property_with_priority(
                "width",
                &format!("{}%", (100.0 / 1.0) / ratio as f64),
                "important"
            ).unwrap();

            index += 1;
        }

        // Handling touch behavior
        let start_x = Rc::new(RefCell::new(0));
        let start_y = Rc::new(RefCell::new(0));
        let wrapper = Rc::new(wrapper.clone());
        let steps_len = Rc::new(steps.length());

        let x = start_x.clone();
        let y = start_y.clone();
        let w = wrapper.clone();
        
        let on_touchstart = Closure::wrap(Box::new(move |e: &web_sys::TouchEvent| {
            (*w).style().set_property("transition-duration", &format!("{}ms", 0));
            (*w).style().set_property("will-change", "transform");
            let touch = e.touches().get(0).unwrap();
            *x.clone().borrow_mut() = touch.page_x();
            *y.clone().borrow_mut() = touch.page_y();
        }) as Box<dyn FnMut(&web_sys::TouchEvent)>);

        wrapper.set_ontouchstart(Some(on_touchstart.as_ref().unchecked_ref()));

        on_touchstart.forget();

        let x = start_x.clone();
        let y = start_y.clone();
        let w = wrapper.clone();
        let cs = current_step.clone();
        let s = steps_len.clone();

        let step_percent = Rc::new(RefCell::new(0.0));
        let sp = step_percent.clone();

        let on_touchmove = Closure::wrap(Box::new(move |e: &web_sys::TouchEvent| {
            e.prevent_default();

            let touch = e.touches().get(0).unwrap();
            let step_width = e.target().unwrap().dyn_ref::<HtmlElement>().unwrap().get_bounding_client_rect().width();
            *sp.borrow_mut() = 1.0 - (((*x.clone().borrow() - touch.page_x()) * 100) as f64 / step_width);
            let wrapper_percent = 1.0 - (((*x.clone().borrow() - touch.page_x()) * 100) as f64 / (*w).get_bounding_client_rect().width());

            (*w).style().set_property(
                "transform",
                &format!(
                    "translate3d({}%, 0, 0)",
                    (*cs.borrow() as i32 * -100 / *s as i32) + wrapper_percent as i32
                )
            );
        }) as Box<dyn FnMut(&web_sys::TouchEvent)>);

        wrapper.set_ontouchmove(Some(on_touchmove.as_ref().unchecked_ref()));

        on_touchmove.forget();

        let w = wrapper.clone();
        let cs = current_step.clone();
        let s = steps_len.clone();
        let sp = step_percent.clone();
        let c = container.clone();
        let evt = events.clone();
        let st = steps.clone();

        let on_touchend = Closure::wrap(Box::new(move |e: &web_sys::TouchEvent| {
            (*w).style().set_property("transition-duration", &format!("{}ms", 300));
            (*w).style().remove_property("will-change");

            let azza = *cs.borrow();
            if *sp.borrow() > 50.0 && azza as i32 - 1 >= 0 {
                Self::change_step(w.clone(), cs.clone(), azza - 1, &st.clone());
                Self::update_nav(c.clone(), cs.clone());

                let func = Function::from(evt.borrow().get(&Event::StepChange.into()));
                func.call1(&wasm_bindgen::JsValue::UNDEFINED, &JsValue::from_f64((azza - 1) as f64));
            } else if *sp.borrow() < -50.0 && azza + 1 < st.length() {
                Self::change_step(w.clone(), cs.clone(), azza + 1, &st.clone());
                Self::update_nav(c.clone(), cs.clone());

                let func = Function::from(evt.borrow().get(&Event::StepChange.into()));
                func.call1(&wasm_bindgen::JsValue::UNDEFINED, &JsValue::from_f64((azza + 1) as f64));
            }

            (*w).style().set_property(
                "transform",
                &format!(
                    "translate3d({}%, 0, 0)",
                    (*cs.borrow() as i32 * -100 / *s as i32)
                )
            );

            // Reset move percentage of the step
            *sp.borrow_mut() = 0.0;
        }) as Box<dyn FnMut(&web_sys::TouchEvent)>);

        wrapper.set_ontouchend(Some(on_touchend.as_ref().unchecked_ref()));

        on_touchend.forget();

        if let Some(step_one) = steps.get(0) {
            let step_one = step_one.dyn_ref::<HtmlElement>().unwrap();

            wrapper.style().set_property(
                "height",
                &format!("{}px", step_one.get_bounding_client_rect().height())
            );
        }

        let s = Self {
            steps,
            container,
            current_step,
            wrapper,
            events
        };

        s.init();

        Self::update_nav(s.container.clone(), s.current_step.clone());

        s
    }

    fn init(&self) {
        let links = Rc::new((*self.container).query_selector_all(".stepper__nav__item").expect("No link found"));
        let mut index: u32 = 0;
        
        while index < links.length() {
            let container = self.container.clone();
            let wrapper = self.wrapper.clone();
            let steps = self.steps.clone();
            let a = self.current_step.clone();
            let events = self.events.clone();
            let lin = links.clone();
            
            let on_click = Closure::wrap(Box::new(move |e: &web_sys::Event| {
                e.prevent_default();
                let item = lin.get(index).expect("");
                let item = item.dyn_ref::<HtmlElement>().expect("");

                // Calculate transition duration
                let duration = index as i32 - *a.borrow() as i32;
                let duration = if duration < 0 {
                    duration * -1
                } else {
                    duration
                };
                (*wrapper).style().set_property("transition-duration", &format!("{}ms", 300 * duration));
                
                if index < *a.borrow() || item.clone().class_list().contains("stepper__nav__item--editable") {
                    Self::change_step(wrapper.clone(), a.clone(), index, &steps);
                    Self::update_nav(container.clone(), a.clone());

                    let func = Function::from(events.borrow().get(&Event::StepChange.into()));
                    func.call1(&wasm_bindgen::JsValue::UNDEFINED, &JsValue::from_f64(*a.borrow() as f64));
                }
            }) as Box<dyn FnMut(&web_sys::Event)>);

            let link = links.item(index).unwrap();
            link.dyn_ref::<HtmlElement>()
            .unwrap()
            .set_onclick(Some(on_click.as_ref().unchecked_ref()));
            
            on_click.forget();

            index += 1;
        }
    }

    fn change_step(wrapper: Rc<HtmlElement>, current_step: Rc<RefCell<u32>>, new_step: u32, steps: &NodeList) {
        let old_step = steps.get(*current_step.borrow()).expect("1");
        let old_step = old_step.dyn_ref::<HtmlElement>().expect("2");
        old_step.class_list().remove_1("active");

        let translate_x = new_step as i32 * -100 / steps.length() as i32;
        wrapper.style().set_property(
            "transform",
            &format!("translate3d({}%, 0, 0)", translate_x)
        );
        // log(&format!("Move old to: {}", -(new_step as i32) * 947));
        log("Ok");
        *current_step.borrow_mut() = new_step;
        log("Ok");
        // log(&format!("New current_step = {}", *current_step.borrow()));
        let ns = steps.get(new_step).expect("1");
        let ns = ns.dyn_ref::<HtmlElement>().expect("2");
        ns.class_list().add_1("active");

        log(&ns.style().get_property_value("height").unwrap());

        wrapper.style().set_property(
            "height",
            &format!("{}px", ns.get_bounding_client_rect().height())
        );
    }

    // TODO : optimiser cette méthode
    fn update_nav(container: Rc<HtmlElement>, current_step: Rc<RefCell<u32>>) {
        let li = (*container).query_selector_all("nav a").expect("nav a not founds");
        let mut index: u32 = 0;

        log(&format!("{{Update nav}} index : {}; current_step : {}; length : {}", index, *current_step.borrow(), li.length()));

        while index < li.length() {
            let item = li.get(index).expect("1");
            let item = item.dyn_ref::<HtmlElement>().expect("2");

            item.class_list().remove_2("stepper__nav__item--current", "stepper__nav__item--completed");

            if index == *current_step.borrow() {
                item.class_list().add_1("stepper__nav__item--current");

                if let Ok(icon) = item.query_selector("svg") {
                    if let Some(icon) = icon {
                        icon.remove();
                    }
                }
            } else if index < *current_step.borrow() {
                item.class_list().add_1("stepper__nav__item--completed");

                if let Ok(icon) =  item.query_selector("svg") {
                    if let None = icon {
                        item.insert_adjacent_html(
                            "beforeend",
                            r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 20 20" fill="currentColor">
                            <path fill-rule="evenodd" d="M16.707 5.293a1 1 0 010 1.414l-8 8a1 1 0 01-1.414 0l-4-4a1 1 0 011.414-1.414L8 12.586l7.293-7.293a1 1 0 011.414 0z" clip-rule="evenodd" />
                        </svg>"#
                        ).unwrap();
                    }
                }
            } else if index > *current_step.borrow() {
                if let Ok(icon) = item.query_selector("svg") {
                    if let Some(icon) = icon {
                        icon.remove();
                    }
                }
            }

            index += 1;
        }
    }

    pub fn prev(&mut self) {
        if *self.current_step.borrow() > 0 {
            let current_step = *self.current_step.clone().borrow();

            Self::change_step(self.wrapper.clone(), self.current_step.clone(), current_step - 1, &self.steps);
            Self::update_nav(self.container.clone(), self.current_step.clone());

            let func = Function::from(self.events.borrow().get(&Event::StepChange.into()));
            func.call1(&wasm_bindgen::JsValue::UNDEFINED, &JsValue::from_f64(*self.current_step.borrow() as f64));
        }
    }

    pub fn on(&mut self, event: Event, f: Function) {
        self.events.borrow_mut().set(&event.into(), &f);
    }

    pub fn next(&mut self) {
        if *self.current_step.borrow() < self.steps.length() {
            let current_step = *self.current_step.clone().borrow();
        
            Self::change_step(self.wrapper.clone(), self.current_step.clone(), current_step + 1, &self.steps);
            Self::update_nav(self.container.clone(), self.current_step.clone());

            let func = Function::from(self.events.borrow().get(&Event::StepChange.into()));
            func.call1(&wasm_bindgen::JsValue::UNDEFINED, &JsValue::from_f64(*self.current_step.borrow() as f64));
        }
    }
}