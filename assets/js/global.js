import LazyLoader from './utils/lazy_loader';
import Router from 'router';
import { get } from './utils/http';

let loader = null;
let navbar = null;

const send_metrics = () => {
    if (!navigator.sendBeacon) return;
    
    const vid = localStorage.getItem('VID');

    if (vid !== null) {
        navigator.sendBeacon('/metrics/log', new URLSearchParams({
            token: vid
        }));

        localStorage.setItem('VID', '');
    }
}

// Check webp support
(() => {
    const img = new Image();
    img.onload = () => document.documentElement.classList.add('webp_supportted');
    img.onerror = () => document.documentElement.classList.add('no_webp');

    img.src = 'data:image/webp;base64,UklGRjoAAABXRUJQVlA4IC4AAACyAgCdASoCAAIALmk0mk0iIiIiIgBoSygABc6WWgAA/veff/0PP8bA//LwYAAA';
})()

const handle_navbar = () => {
    navbar.querySelector('.active').classList.remove('active');
    const new_active = navbar.querySelector(`[href="${location.pathname}"]`);

    switch (location.pathname) {
        case '/':
        case '/creation-site-web':
        case '/agence-digitale-verte':
        case '/portfolio':
        case '/contact':
            new_active.classList.add('active');
        break;
        case '/creation-site-web/onepage':
        case '/creation-site-web/vitrine':
        case '/creation-site-web/e-commerce':
            navbar.querySelector('[href="/creation-site-web"]').classList.add('active');
        break;
    }
}

window.addEventListener('router:change', () => {
    send_metrics();
    LazyLoader();
    Object.assign(loader.style, { transition: 'visibility 100ms ease-out, opacity 100ms ease-out', visibility: 'hidden', opacity: 0 })
    document.documentElement.style.overflow = 'auto';
    handle_navbar();

    get(`/metrics/token?path=${location.pathname}`)
        .then(res => res.text())
        .then(token => localStorage.setItem('VID', token))

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
        loader = document.querySelector('#loading');
        navbar = document.querySelector('#navbar');
        handle_navbar();

        LazyLoader();

        console.log(loader)
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
                    top: window.innerHeight - (window.innerWidth > 737 ? 80 : 0),
                    behavior: 'smooth'
                });
            });
        }
        
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

window.addEventListener('unload', send_metrics, false);