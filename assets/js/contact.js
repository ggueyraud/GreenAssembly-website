import Form from './components/form';
import Carousel, { CarouselPagination, CarouselTouch } from './components/carousel';
import { post } from './utils/http';

const on_mount = () => {
    const body = {};

    let carousel = new Carousel(document.querySelector('.carousel'), {
        // slides_visible: 2,
        auto_height: true,
        breakpoints: {
            768: {},
            1200: {},
            // 600: {
            //     slides_visible: 2,
            // }
        }
    });
    carousel.use(CarouselPagination);
    carousel.use(CarouselTouch);
    carousel.on('change', (index) => {
        if (index === 0) {
            if (document.querySelector('[name=why_for]:checked').value === 'simple_discussion') {
    
            }

            document
                .querySelector('[name=why_for]:checked')
                .checked = false
        }
        console.log('change to', index)
    })

    const budget_label = document.querySelector('[for="budget"]');
    const budget_input = document.querySelector('[name="budget"]');
    const services_label = document.querySelector('[for=services]');
    const services = document.querySelector('#services');
    const company_label = document.querySelector('[for="company"]');
    const company_input = document.querySelector('[name="company"]');
    // const resume_label = document.querySelector('[for=resume]');
    // const resume_input = document.querySelector('[name=resume]');
    
    let stepper = null;
    

    import('../wasm/pkg/wasm_bg.js')
    .then(wasm => {
        stepper = wasm.Stepper.new(document.querySelector('.stepper'), wasm.Options.new(false));
        stepper.on(wasm.Event.StepChange, (index) => {
            if (index === 0) {
                if (body.new_project === false) {
                    personal_informations_form.add_field(company_input);
                    project_form.add_field('services');
        
                    company_label.classList.remove('hidden');
                    company_input.classList.remove('hidden');
                    budget_label.classList.remove('hidden');
                    budget_input.classList.remove('hidden');
                    services.style.removeProperty('display');
                    services_label.style.removeProperty('display');
                    // resume_label.classList.add('hidden');
                    // resume_input.classList.add('hidden');
                }

                document
                    .querySelectorAll('[name=why_for')
                    .forEach(chk => {
                        chk.checked = false
                    })
            }
        })
    })
    .catch(console.error)
    
    
    
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
            budget: {}
        }
    })
        .on('valid', e => {
            e.target.querySelector('[type=submit]').classList.add('opacity_100');
        })
        .on('invalid', e => {
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
                        document.querySelector('.steps').classList.add('hidden');
                    }
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
            const message_label = document.querySelector('label[for="message"]');
            document.querySelector('[name="lastname"]').focus({ preventScroll: true });
    
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