export class CarouselPagination {
    constructor(carousel, options) {
        this.carousel = carousel;
        this.options = Object.assign({}, {
            render: () => `<a href="#" class="carousel__pagination__item"></a>`
        }, options);
        this.items = [];

        const nav = this.carousel.container.querySelector('.carousel__pagination');
        for (let i = 0; i < this.carousel.childrens.length; i++) {
            let button = document.createElement('button');
            button.classList.add('carousel__pagination__item');
            button.addEventListener('click', () => {
                this.carousel.goto_slide(i);
            });

            nav.appendChild(button);
            this.items.push(button);
        }

        // Fire resize event to init pagination html
        this.resize();
    }

    resize() {
        let nb_pages = Math.ceil(this.carousel.childrens.length / this.carousel.active_options.slides_visible);

        this.items.forEach((item, index) => {
            item.classList.remove('carousel__pagination__item--hidden');

            if (index > nb_pages) {
                item.classList.add('carousel__pagination__item--hidden');
            }
        });

        if (this.items.length > 0) {
            this.update();
        }
    }

    update() {
        const current_item = this.carousel.container.querySelector('.carousel__pagination__item--current');

        if (current_item) {
            current_item.classList.remove('carousel__pagination__item--current');
        }

        this.items[this.carousel.current_slide].classList.add("carousel__pagination__item--current");
        this.items.forEach((item, index) => {
            if (index >= this.carousel.current_slide) {
                item.classList.remove('carousel__pagination__item--past');
            }

            if (index < this.carousel.current_slide) {
                item.classList.add('carousel__pagination__item--past');
            }
        })
    }
}

const disable_scroll = (x, y) => {
    console.log('disable scroll')
    window.scrollTo(x, y)
}

export class CarouselTouch {
    constructor(carousel) {
        this.carousel = carousel;

        carousel.wrapper.addEventListener('mousedown', this.start_drag.bind(this));
        carousel.wrapper.addEventListener('touchstart', this.start_drag.bind(this));
        carousel.wrapper.addEventListener('dragstart', e => e.preventDefault());
        window.addEventListener('mousemove', this.drag.bind(this));
        window.addEventListener('touchmove', this.drag.bind(this));
        window.addEventListener('touchend', this.end_drag.bind(this));
        window.addEventListener('mouseup', this.end_drag.bind(this));
        window.addEventListener('touchcancel', this.end_drag.bind(this));
    }

    start_drag(e) {
        if (this.carousel.active_options.slides_visible === this.carousel.childrens.length) {
            return;
        }

        if (e.touches) {
            if (e.touches.length > 1) {
                return;
            } else {
                e = e.touches[0];
            }
        }

        console.log(e)

        this.carousel.wrapper.style.setProperty('transition-duration', '0ms');
        this.carousel.wrapper.style.setProperty('will-change', 'transform');
        this.carousel.wrapper.style.setProperty('cursor', 'grab');
        this.origin = { x: e.pageX, y: e.pageY };
        this.width = this.carousel.container.offsetWidth;
    }

    end_drag() {
        if (this.origin && this.last_translate) {
            this.carousel.wrapper.style.setProperty('transition-duration', '300ms');
            this.carousel.wrapper.style.removeProperty('will-change');
            
            if (Math.abs(this.last_translate.x / this.carousel.container.offsetWidth) > 0.2) {
                if (this.last_translate.x < 0) {
                    if (!this.carousel.next()) {
                        this.carousel.goto_slide(this.carousel.current_slide);
                    }
                } else {
                    if (!this.carousel.prev()) {
                        this.carousel.goto_slide(this.carousel.current_slide);
                    }
                }
            } else {
                this.carousel.goto_slide(this.carousel.current_slide);
            }

            this.carousel.update();
        }

        window.onscroll = null;

        this.carousel.wrapper.style.setProperty('cursor', 'default');
        this.origin = null;
    }

    drag(e) {
        if (this.origin) {
            let point = e.touches ? e.touches[0] : e;
            let translate = {
                x: point.pageX - this.origin.x,
                y: point.pageY - this.origin.y
            };

            if (Math.abs(translate.x) > 20 && window.onscroll === null) {
                const x = window.scrollX;
                const y = window.scrollY;

                window.onscroll = () => disable_scroll(x, y);
            }

            if (e.touches && Math.abs(translate.x) > Math.abs(translate.y)) {
                e.preventDefault();
                e.stopPropagation();
            } else if (e.touches) {
                return;
            }

            this.last_translate = translate;
            let base_translate = this.carousel.current_slide * -100 / this.carousel.childrens.length;


            let p = 1.0 - (((this.origin.x - point.screenX) * 100) / (this.carousel.container.getBoundingClientRect().width));
            let a = 1.0 - (((this.origin.x - point.pageX) * 100) / this.carousel.wrapper.getBoundingClientRect().width);

            this.carousel.wrapper.style.transform = `translate3d(${(this.carousel.current_slide * -100 / this.carousel.childrens.length) + a}%, 0, 0)`;
        }
    }
}

const calculate_height = carousel => {
    const current_step = carousel.childrens[carousel.current_slide];

    carousel.wrapper.style.setProperty('height', `${current_step.getBoundingClientRect().height}px`);
}

const handle_breakpoints = carousel => {
    const breakpoints = carousel.base_options.breakpoints;

    if (breakpoints !== null) {
        const points = Object.keys(breakpoints)
            .map(point => parseInt(point))
            .sort((a, b) => {
                if (a < b) 
                    return 1;
                else if (a > b)
                    return -1;

                return 0;
            });

        for (const point of points) {
            if (window.innerWidth >= point) {
                carousel.active_options = Object.assign({}, carousel.active_options, breakpoints[point])

                carousel.set_style()
                return;
            }
        }
    }
}

export default class {
    constructor(container, options = {}) {
        this.modules = [];
        this.container = container;
        this.container.setAttribute('tabindex', 0);
        this.wrapper = container.querySelector('.carousel__wrapper');
        this.base_options = Object.assign({}, {
            slides_to_scroll: 1,
            slides_visible: 1,
            allow_slide_next:   true,
            allow_slide_prev: true,
            breakpoints: null,
            auto_height: false,
            space_between: '2rem'
        }, options);
        this.active_options = this.base_options;

        window.addEventListener('resize', () => {
            
            this.active_options = this.base_options;
            handle_breakpoints(this);
            if (this.active_options.auto_height) {
                calculate_height(this);
            }

            for (const module of this.modules) {
                if (module.resize) {
                    module.resize();
                }
            }

            this.set_style()
        });

        this.wrapper.style.gap = this.active_options.space_between;
        
        this.childrens = [].slice.call(this.wrapper.querySelectorAll('.carousel__wrapper__item'));
        this.current_slide = 0;
        this.set_style();
        this.events = {};

        this.container.addEventListener('keyup', e => {
            if (e.key === 'ArrowRight') {
                this.next();
            } else if (e.key === 'ArrowLeft') {
                this.prev();
            }
        })

        handle_breakpoints(this);

        if (this.active_options.auto_height) {
            this.wrapper.style.setProperty('align-items', 'flex-start');
            calculate_height(this);
        }
    }

    use(modules) {
        if (Array.isArray(modules)) {
            modules.forEach(module => this.modules.push(new module(this)))
        } else {
            this.modules.push(new modules(this))
        }
    }

    update() {
        for (const module of this.modules) {
            if (module.update !== undefined) {
                module.update();
            }
        }
    }

    set_style() {
        let ratio = this.childrens.length / this.active_options.slides_visible;

        this.wrapper.style.width = `${ratio * 100}%`;
        this.wrapper.style.gridTemplateColumns = `repeat(${this.childrens.length}, ${this.active_options.slides_visible}fr)`
    }

    next() {
        if (
            (this.current_slide +
            (this.active_options.slides_visible - 1) +
            this.active_options.slides_to_scroll) < this.childrens.length && this.active_options.allow_slide_next
        ) {
            this.goto_slide(this.current_slide + this.active_options.slides_to_scroll);

            return true;
        }

        return false;
    }

    prev() {
        if (this.current_slide - this.active_options.slides_to_scroll >= 0 && this.active_options.allow_slide_prev) {
            this.goto_slide(this.current_slide - this.active_options.slides_to_scroll);

            return true;
        }

        return false;
    }

    goto_slide(index) {
        this.current_slide = index;
        this.wrapper.style.transform = `translate3d(-${(100 / this.childrens.length) * index}%, 0, 0)`;
        this.update();

        if (this.active_options.auto_height) {
            calculate_height(this);
        }

        const on_change_evt = this.events['change'];

        if (on_change_evt) {
            on_change_evt(index);
        }
    }

    on(event_name, callback) {
        if (!['change'].includes(event_name)) {
            throw `The event "${event_name}" doesn't exist`;
        }

        this.events[event_name] = callback;
    }
}