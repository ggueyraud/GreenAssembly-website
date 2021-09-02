import Form from './components/form';
import chk from './components/checkbox';
import Carousel from 'carousel';
import { post } from './utils/http';

const chks = chk('.checkbox', { checked: true });
// chks[1].checked = false
// console.log(chks)

// document.addEventListener('keyup', () => chks[0].checked = false)

class StepperPagination {
    constructor(carousel) {
        this.carousel = carousel;
        this.items = [];
        
        const nav = this.carousel.container.querySelector('.carousel__pagination');

        this.carousel.childrens.forEach((child, index) => {
            const content = document.createElement('div');

            const data_title = child.dataset.title;

            const observer = new MutationObserver((mutations) => {
                mutations.forEach(mutation => {
                    if (mutation.attributeName === 'data-description') {
                        const description = content.querySelector('.descr');

                        if (description) {
                            description.innerHTML = mutation.target.dataset.description;
                        }
                    }
                })
            })

            observer.observe(child, {
                attributes: true,
                childList: false,
                characterData: false
            })

            if (data_title) {
                const index_el = document.createElement('div');
                index_el.innerHTML = child.dataset.title;
                index_el.classList.add('index');

                content.appendChild(index_el);
            }

            const data_description = child.dataset.description;

            if (data_description) {
                const description = document.createElement('div');
                description.innerHTML = child.dataset.description;
                description.classList.add('descr');

                content.appendChild(description);
            }

            let button = document.createElement('button');
            button.classList.add('carousel__pagination__item');

            if (index < this.carousel.current_slide) {
                button.classList.add('carousel__pagination__item--past');
            }

            button.appendChild(content);
            button.addEventListener('click', () => {
                if (button.classList.contains('carousel__pagination__item--past')) {
                    this.carousel.goto_slide(index)
                }
            });

            nav.appendChild(button);
            this.items.push(button);
        });

        this.resize()
    }

    resize() {
        let nb_pages = Math.ceil(this.carousel.childrens.length / this.carousel.active_options.slides_visible);

        this.items.forEach((item, index) => {
            item.classList.remove('carousel__pagination__item--hidden');

            if (index >= nb_pages) {
                item.classList.add('carousel__pagination__item--hidden');
            }
        });

        if (nb_pages === 1) {
            this.items[0].classList.add('carousel__pagination__item--hidden');
        }

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
            const icon = item.querySelector('svg');

            if (index >= this.carousel.current_slide) {
                item.classList.remove('carousel__pagination__item--past');

                if (icon) {
                    item.classList.remove('carousel__pagination__item--past');
                    icon.remove();
                }
            }

            if (index < this.carousel.current_slide) {
                item.classList.add('carousel__pagination__item--past');

                if (!icon) {
                    item.insertAdjacentHTML(
                        'beforeend',
                        `<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 20 20" fill="currentColor">
                            <path fill-rule="evenodd" d="M16.707 5.293a1 1 0 010 1.414l-8 8a1 1 0 01-1.414 0l-4-4a1 1 0 011.414-1.414L8 12.586l7.293-7.293a1 1 0 011.414 0z" clip-rule="evenodd" />
                        </svg>`
                    );
                }
            }
        })
    }
}

const on_mount = () => {
    const body = {};

    const carousel_el = document.querySelector('.carousel');
    let carousel = new Carousel(carousel_el, {
        auto_height: true,
        arrow_navigation: false
    });
    carousel.use([StepperPagination]);
    carousel.on('change', (index) => {
        if (index === 0) {
            document
                .querySelector('[name=why_for]:checked')
                .checked = false
        }

        if (index === 0 || index === 1) {
            document.querySelector('#error').classList.add('hidden');
        }
        
        const label = carousel_el.querySelector(`.label`);
        label.querySelector('.index').innerHTML = `Étape ${index + 1}`;
        const descr = label.querySelector('.descr');
        descr.innerHTML = carousel.childrens[index].dataset.description;
    });
    

    const budget_label = document.querySelector('[for="budget"]');
    const budget_input = document.querySelector('[name="budget"]');
    const services_label = document.querySelector('[for=services]');
    const services = document.querySelector('#services');
    const company_label = document.querySelector('[for="company"]');
    const company_input = document.querySelector('[name="company"]');
    // const resume_label = document.querySelector('[for=resume]');
    // const resume_input = document.querySelector('[name=resume]');  
    
    document
        .querySelector('[name="message"]')
        .addEventListener('input', e => {
            document.querySelector('#message_caracters_counter').innerHTML = e.target.value.length
        })
    
    const personal_informations_form = new Form(document.querySelector('[name="informations"]'))
        .on('valid', e => {
            e.target.querySelector('[type=submit]').classList.add('opacity_100');
        })
        .on('invalid', e => {
            carousel.calculate_height();
            e.target.querySelector('[type=submit]').classList.remove('opacity_100');
        })
        .on('send', e => {
            e.preventDefault();
    
            Object.assign(body, e.detail);
    
            carousel.next();
        });
    const project_form = new Form(document.querySelector('[name="project"]'), {
        fields: {
            services: {
                validators: {
                    notEmpty: 'Sélectionnez au moins une prestation',
                    container: document.querySelector('#services')
                }
            },
            message: {
                validators: {
                    notEmpty: 'Ce champ est obligatoire',
                    stringLength: 'Le message doit faire entre 30 et 500 caractères',
                }
            },
            found_by: {},
            budget: {}
        }
    })
        .on('valid', e => {
            e.target.querySelector('[type=submit]').classList.add('opacity_100');
        })
        .on('invalid', e => {
            carousel.calculate_height();
            e.target.querySelector('[type=submit]').classList.remove('opacity_100');
        })
        .on('send', e => {
            e.preventDefault();
    
            Object.assign(body, e.detail);
            
            if (body.budget) body.budget = parseFloat(body.budget) || null
    
            post('/contact', {
                headers: {
                    'Content-Type': 'application/json'
                },
                validate_status: status => status === 200,
                body
            })
                .then(res => {
                    if (res.ok) {
                        document.querySelector("#success").classList.remove('hidden');
                        document.querySelector('.stepper').classList.add('hidden');
                    }
                })
                .catch(() => {
                    document.querySelector('#error').classList.remove('hidden');
                    carousel.calculate_height()
                })
        });
    
    // Contact-for forms
    new Form(document.querySelector('[name="why-for"]'), {
        fields: {
            why_for: {
                validators: {
                    notEmpty: 'Sélectionnez une __'
                }
            }
        }
    })
        .on('valid', e => {
            document.querySelector('[name=project]').dataset.description = document.querySelector('[name=why_for]:checked').value === 'simple_discussion'
                ? 'Message'
                : 'Informations sur votre projet';
            const message_label = document.querySelector('label[for="message"]');
            setTimeout(() => document.querySelector('[name="lastname"]').focus({ preventScroll: true }), 300);
    
            if (e.detail.why_for === 'new_project') {
                body.new_project = true;
    
                message_label.innerHTML = 'Description *';
            } else {
                body.new_project = false;
    
                personal_informations_form.remove_field('company');
                project_form.remove_field('services');
    
                company_label.classList.add('hidden');
                company_input.classList.add('hidden');
                message_label.innerHTML = 'Message *';
                budget_label.classList.add('hidden');
                budget_input.classList.add('hidden');
                // services.classList.add('hidden');
                // services_label.classList.add('hidden');
                services.style.setProperty('display', 'none', 'important');
                services_label.style.setProperty('display', 'none', 'important');

                // if (e.detail.why_for === 'internship') {
                //     resume_label.classList.remove('hidden');
                //     resume_input.classList.remove('hidden');
                // }
            }
    
            carousel.next();
        });
}

window.addEventListener('onMount', on_mount)
window.addEventListener('onDestroy', () => window.removeEventListener('onMount', on_mount))