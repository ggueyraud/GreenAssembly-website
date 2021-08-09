export class CarouselPagination {
    constructor(carousel) {
        this.carousel = carousel;
        this.items = carousel.container.querySelectorAll('.carousel__pagination__item');
        this.items.forEach((item, index) => {
            item.addEventListener('click', e => {
                e.preventDefault();

                this.carousel.goto_slide(index);
                this.update();
            })
        })
        this.update()
    }

    update() {
        console.log('update pagination', this.carousel.current_slide);
        const current_item = this.carousel.container.querySelector('.carousel__pagination__item--current');
        console.log(current_item)
        if (current_item) {
            current_item.classList.remove('carousel__pagination__item--current');
        }

        this.items[this.carousel.current_slide].classList.add("carousel__pagination__item--current");
        this.items.forEach((item, index) => {
            const icon = item.querySelector('svg');
                
            if (icon) icon.remove();

            if (index >= this.carousel.current_slide) {
                item.classList.remove('carousel__pagination__item--past');
            }

            if (index < this.carousel.current_slide) {
                item.classList.add('carousel__pagination__item--past');
                item.insertAdjacentHTML(
                    'beforeend',
                    `<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 20 20" fill="currentColor">
                        <path fill-rule="evenodd" d="M16.707 5.293a1 1 0 010 1.414l-8 8a1 1 0 01-1.414 0l-4-4a1 1 0 011.414-1.414L8 12.586l7.293-7.293a1 1 0 011.414 0z" clip-rule="evenodd" />
                    </svg>`
                );
            }
        })
    }
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
        if (e.touches) {
            if (e.touches.length > 1) {
                return;
            } else {
                e = e.touches[0];
            }

            this.carousel.wrapper.style.setProperty('transition-duration', '0ms');
            this.carousel.wrapper.style.setProperty('will-change', 'transform');
            this.origin = { x: e.screenX, y: e.screenY };
            this.width = this.carousel.container.offsetWidth;
        }
    }

    end_drag(e) {
        console.log(e)
        if (this.origin && this.last_translate) {
            this.carousel.wrapper.style.setProperty('transition-duration', '300ms');
            this.carousel.wrapper.style.removeProperty('will-change');
            
            if (Math.abs(this.last_translate.x / this.carousel.container.offsetWidth) > 0.2) {
                if (this.last_translate.x < 0) {
                    this.carousel.next();
                } else {
                    this.carousel.prev();
                }
            } else {
                console.log('return to current item')
                this.carousel.goto_slide(this.carousel.current_slide);
            }

            this.carousel.update();
        }

        this.origin = null;
    }

    drag(e) {
        if (this.origin) {
            let point = e.touches ? e.touches[0] : e;
            let translate = {
                x: point.screenX - this.origin.x,
                y: point.screenY - this.origin.y
            };

            if (e.touches && Math.abs(translate.x) > Math.abs(translate.y)) {
                e.preventDefault();
                e.stopPropagation();
            } else if (e.touches) {
                return;
            }

            this.last_translate = translate;
            let base_translate = this.carousel.current_slide * -100 / this.carousel.childrens.length;
            this.carousel.wrapper.style.transform = `translate3d(${base_translate + 100 * translate.x / this.width}%, 0, 0)`;
        }
    }
}

const calculate_height = carousel => {
    const current_step = carousel.childrens[carousel.current_slide];
    console.log('Height', current_step.getBoundingClientRect().height)
    carousel.wrapper.style.setProperty('height', `${current_step.getBoundingClientRect().height}px`);
}

export default class {
    constructor(container, options = {}) {
        this.modules = [];
        this.container = container;
        this.wrapper = container.querySelector('.carousel__wrapper');
        this.base_options = Object.assign({}, {
            slides_to_scroll: 1,
            slides_visible: 1,
            allow_slide_next:   true,
            allow_slide_prev: true,
            breakpoints: null,
            auto_height: false
        }, options);
        this.active_options = this.base_options;

        window.addEventListener('resize', () => {
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
    
                for (const point of points) {
                    if (window.innerWidth >= point) {
                        this.active_options = Object.assign({}, this.active_options, this.base_options.breakpoints[point])
                        this.set_style()
                        console.log(this.active_options, this.base_options)
                        break;
                    }
                }
            }

            
            this.active_options = this.base_options;
            if (this.active_options.auto_height) {
                calculate_height(this);
            }
            // console.log(this.active_options, this.base_options)
            this.set_style()
        });
        
        this.childrens = [].slice.call(this.wrapper.querySelectorAll('.carousel__wrapper__item'));
        this.current_slide = 0;
        this.set_style();
        this.events = {};

        console.log(this.active_options)
        if (this.active_options.auto_height) {
            calculate_height(this);
        }
    }

    use(module) {
        this.modules.push(new module(this))
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
        if (this.current_slide + this.active_options.slides_to_scroll < this.childrens.length && this.active_options.allow_slide_next)
            this.goto_slide(this.current_slide + this.active_options.slides_to_scroll);
    }

    prev() {
        if (this.current_slide - this.active_options.slides_to_scroll > 0 && this.active_options.allow_slide_prev)
            this.goto_slide(this.current_slide - this.active_options.slides_to_scroll);
    }

    goto_slide(index) {
        this.current_slide = index;
        this.wrapper.style.transform = `translate3d(-${(100 / this.childrens.length) * index}%, 0, 0)`;
        this.update();
        calculate_height(this);
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