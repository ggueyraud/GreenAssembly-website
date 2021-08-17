import LazyLoader from './utils/lazy_loader';
import Router from './utils/router';

const loader = document.querySelector('#loading');

// Check webp support
(() => {
    const img = new Image();
    img.onload = () => {
        document.documentElement.classList.add('webp_support')
    }

    img.src = 'data:image/webp;base64,UklGRjoAAABXRUJQVlA4IC4AAACyAgCdASoCAAIALmk0mk0iIiIiIgBoSygABc6WWgAA/veff/0PP8bA//LwYAAA';
})()

window.addEventListener('router:change', () => {
    LazyLoader();
    Object.assign(loader.style, { transition: 'visibility 100ms ease-out, opacity 100ms ease-out', visibility: 'hidden', opacity: 0 })
    document.documentElement.style.overflow = 'auto'

    setTimeout(() => {
        loader.style.transition = null;
    }, 100)

    const scroll_down = document.querySelector('#scroll_down');
        
    if (scroll_down) {
        scroll_down.addEventListener('click', () => {
            window.scrollTo({
                top: window.innerHeight,
                behavior: 'smooth'
            });
        })
    }
})

document.addEventListener('readystatechange', e => {
    if (e.target.readyState === 'complete') {
        LazyLoader();

        loader.style.visibility = 'hidden';
        loader.style.opacity = '0';
        document.documentElement.style.overflow = 'auto';
        
        window.addEventListener('router:loading', () => {
            Object.assign(loader.style, { opacity: 100, visibility: 'visible' });
                
            document.documentElement.style.overflow = 'hidden';
        })

        window.router = new Router();
        
        const scroll_down = document.querySelector('#scroll_down');
        
        if (scroll_down) {
            scroll_down.addEventListener('click', () => {
                window.scrollTo({
                    top: window.innerHeight,
                    behavior: 'smooth'
                });
            });
        }
        
        const navbar = document.querySelector('#navbar');
        const html = document.querySelector('html');

        const close_mobile_menu = () => {
            navbar.classList.remove('show');
            document.documentElement.style.overflow = null;
            window.history.pushState(null, null, ' ');
        }
        
        // Open menu
        document
            .querySelector('#open-mobile-menu')
            .addEventListener('click', e => {
                e.preventDefault();
                window.history.pushState({ menu_opened: true }, null, '#menu-opened');
                navbar.classList.add('show');
                document.documentElement.style.overflow = 'hidden';
            });
        
        // Close menu
        document
            .querySelector('#close-mobile-menu')
            .addEventListener('click', close_mobile_menu);

        navbar
            .querySelectorAll('#menu a')
            .forEach(link => {
                link
                    .addEventListener('click', e => {
                        console.log(link)
                        if (!link.classList.contains('social')) {
                            e.preventDefault();
                        }
        
                        close_mobile_menu();
                    })
            });

        // TODO : finir d'implémenter le nouveau comportement du menu
        // window.addEventListener('popstate', (e) => {
        //     console.log(e)
        //     console.log(window.history)
        // })
        
        // Listen window resizing
        window.addEventListener('resize', (e) => {
            if (e.target.innerWidth >= 640 && navbar.classList.contains('show')) {
                navbar.classList.remove('show');
                html.classList.remove('overflow_hidden');
                window.history.pushState(null, null, ' ');
            }
        });
    }
});