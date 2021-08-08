export default class {
    constructor(container, options = {}) {
        this.container = container;
        this.wrapper = container.querySelector('.carousel__wrapper');
        this.base_options = Object.assign({}, {
            slides_to_scroll: 1,
            slides_visible: 1,
            allow_slide_next:   true,
            allow_slide_prev: true,
            breakpoints: null
        }, options);
        this.active_options = this.base_options;

        if (this.base_options.breakpoints !== null) {
            const points = Object.keys(this.base_options.breakpoints)
                .map(point => parseInt(point))
                .sort((a, b) => {
                    if (a < b) 
                        return 1;
                    else if (a > b)
                        return -1;

                    return 0;
                });

            window.addEventListener('resize', () => {
                for (const point of points) {
                    if (window.innerWidth >= point) {
                        // console.log(point, this.options.breakpoints)
                        this.active_options = Object.assign({}, this.active_options, this.base_options.breakpoints[point])
                        this.set_style()
                        // console.log('current breakpoint is', point)
                        console.log(this.active_options, this.base_options)
                        return;
                        // break;
                    }
                }

                
                this.active_options = this.base_options;
                console.log(this.active_options, this.base_options)
                // console.log(options)
                this.set_style()

                // console.log('ok')
            });
        }
        
        this.childrens = [].slice.call(this.wrapper.querySelectorAll('.stepper__wrapper__item'));
        this.current_slide = 0;
        this.set_style();
        this.events = {};
    }

    set_style() {
        let ratio = this.childrens.length / this.active_options.slides_visible;

        this.wrapper.style.width = `${ratio * 100}%`;
        this.wrapper.style.gridTemplateColumns = `repeat(${this.childrens.length}, ${this.active_options.slides_visible}fr)`
    }

    next() {
        if (this.current_slide + this.active_options.slides_to_scroll < this.childrens.length && this.active_options.allow_slide_next)
            this.goto_slide(this.current_slide + this.active_options.slides_to_scroll);
    }

    prev() {
        if (this.current_slide - this.active_options.slides_to_scroll > 0 && this.active_options.allow_slide_prev)
            this.goto_slide(this.current_slide - this.options.slides_to_scroll);
    }

    goto_slide(index) {
        this.current_slide = index;
        this.wrapper.style.transform = `translate3d(-${(100 / this.childrens.length) * index}%, 0, 0)`;

        const on_change_evt = this.events['change'];

        if (on_change_evt) {
            on_change_evt.call(this);
        }
    }

    on(event_name, callback) {
        if (!['change'].includes(event_name)) {
            throw `The event "${event_name}" doesn't exist`;
        }

        this.events[event_name] = callback;
    }
}