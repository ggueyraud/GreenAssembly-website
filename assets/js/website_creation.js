import Carousel, { CarouselPagination, CarouselTouch } from 'carousel';

export class CarouselPaginationStep extends CarouselPagination {
    constructor(carousel) {
        super(carousel)

        this.items.forEach(item => {
            item.insertAdjacentHTML(
                'beforeend',
                `<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 20 20" fill="currentColor">
                    <path fill-rule="evenodd" d="M16.707 5.293a1 1 0 010 1.414l-8 8a1 1 0 01-1.414 0l-4-4a1 1 0 011.414-1.414L8 12.586l7.293-7.293a1 1 0 011.414 0z" clip-rule="evenodd" />
                </svg>`
            );
        });
    }
}

const on_mount = () => {
    const carousel_formules = new Carousel(document.querySelector('#formules .carousel'));
    carousel_formules.use([CarouselPagination, CarouselTouch]);

    const carousel_what_we_do = new Carousel(document.querySelector('#what_we_do .carousel'), {
        breakpoints: {
            768: {
                slides_visible: 2
            },
            1280: {
                slides_visible: 3,
                space_between: '3rem'
            }
        }
    });
    carousel_what_we_do.use([CarouselPagination, CarouselTouch]);

    const carousel_steps_el = document.querySelector('#steps .carousel');
    const carousel_steps = new Carousel(carousel_steps_el, {
        auto_height: true,
        render_bullet: (carousel, index) => {
            const index_el = document.createElement('div');
            index_el.innerHTML = `Étape ${index + 1}`;
            index_el.classList.add('index');
            
            const descr = document.createElement('div');
            descr.classList.add('descr');
            
            const content = document.createElement('div');
            content.appendChild(index_el);
            content.appendChild(descr);

            switch (index) {
                case 0:
                    descr.innerHTML = 'Étude de votre besoin';
                break;
                case 1:
                    descr.innerHTML = 'Création graphique <span class="block">&</span> Développement';
                break;
                case 2:
                    descr.innerHTML = 'Choix serveur <span class="block">&</span> Optimisation SEO';
                break;
                case 3:
                    descr.innerHTML = 'Formation <span class="block">&</span> Suivi qualité';
                break;
            }

            let button = document.createElement('button');
            button.classList.add('carousel__pagination__item');

            if (index < carousel.current_slide) {
                button.classList.add('carousel__pagination__item--completed');
            }

            button.appendChild(content);

            return button
        }
    });
    carousel_steps.on('change', () => {
        const label = carousel_steps_el.querySelector(`.label`);
        label.querySelector('.index').innerHTML = `Étape ${carousel_steps.current_slide + 1}`;
        const descr = label.querySelector('.descr');

        switch (carousel_steps.current_slide) {
            case 0:
                descr.innerHTML = 'Étude de votre besoin';
            break;
            case 1:
                descr.innerHTML = 'Création graphique <span class="block">&</span> Développement';
            break;
            case 2:
                descr.innerHTML = 'Choix serveur <span class="block">&</span> Optimisation SEO';
            break;
            case 3:
                descr.innerHTML = 'Formation <span class="block">&</span> Suivi qualité';
            break;
        }
    });
    carousel_steps.use([CarouselPaginationStep, CarouselTouch]);
}

window.addEventListener('onMount', on_mount)
window.addEventListener('onDestroy', () => {
    window.removeEventListener('onMount', on_mount);
});