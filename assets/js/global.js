import LazyLoader from './utils/lazy_loader';
import Router from 'router';
import { get } from './utils/http';

let loader = null;
let navbar = null;

const read_cookie = (cookie_name) => {
    return document.cookie.split('; ').find(row => row.startsWith(cookie_name))?.split('=')?.[1]
};

const send_metrics = () => {
    if (!navigator.sendBeacon) return;
    
    const vid = localStorage.getItem('VID');
    const sid = read_cookie('sid');

    if (vid !== null) {
        navigator.sendBeacon('/metrics/log', new URLSearchParams({
            sid,
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
    navbar.querySelector('.active')?.classList.remove('active');
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

window.addEventListener('router:change', async () => {
    send_metrics();
    LazyLoader();
    Object.assign(loader.style, { transition: 'visibility 100ms ease-out, opacity 100ms ease-out', visibility: 'hidden', opacity: 0 })
    document.documentElement.style.overflow = 'auto';
    handle_navbar();

    // const rgpd_accepted = read_cookie('rgpd_accepted');
    // if(rgpd_accepted) {
    let sid = read_cookie('sid');
    if(!sid) {
        await get('/metrics/session')
            .then(res => res.json())
            .then(session_data => {
                sid = session_data.sid;
                const expires = new Date(session_data.vud);
                document.cookie = 'sid=' + sid + '; expires=' + expires.toUTCString() + '; SameSite=Strict; Secure';
            });
    }

    get(`/metrics/token?path=${location.pathname}&sid=${sid}`)
        .then(res => res.text())
        .then(token => localStorage.setItem('VID', token))
    // }

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

        loader?.style.visibility = 'hidden';
        loader?.style.opacity = '0';
        document.documentElement.style.overflow = 'auto';
        
        window.addEventListener('router:loading', () => {
            Object.assign(loader?.style, { opacity: 100, visibility: 'visible' });
                
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

// ------------------------------------------------------------------ //
// ------------------------------ RGPD ------------------------------ //
// ------------------------------------------------------------------ //

// const show_rgpd_confirmation = () => {
//     if(document.querySelector('#rgpd_wrapper')) return

//     const rgpd_title = document.createElement('div');
//     rgpd_title.classList.add('rgpd-modal__title')
//     rgpd_title.innerText = 'Cookies ! 🍪';
//     const rgpd_wrapper = document.createElement('div');
//     rgpd_wrapper.setAttribute('id', 'rgpd-modal');
//     rgpd_wrapper.classList.add('rgpd-modal');
//     rgpd_wrapper.textContent = '\
//     En naviguant sur notre site web, vous acceptez de recevoir des cookies \
//     de notre agence de communication digitale. Anonymisés, ils permettent \
//     d\'améliorer votre experience sur le site.';
//     rgpd_wrapper.appendChild(rgpd_title);

//     const rgpd_actions = document.createElement('div');
//     rgpd_actions.classList.add('rgpd-modal__actions');
    
//     const rgpd_privacy_button = document.createElement('a');
//     rgpd_privacy_button.classList.add('rgpd-modal__btn');
//     rgpd_privacy_button.setAttribute('o-follow', '');
//     rgpd_privacy_button.setAttribute('href', '/mentions-legales');
//     rgpd_privacy_button.innerText = 'Mentions légales';

//     const rgpd_confirm_button = document.createElement('button');
//     rgpd_confirm_button.classList.add('rgpd-modal__btn', 'rgpd-modal__btn--accent');
//     rgpd_confirm_button.innerText = 'Accepter';

//     rgpd_actions.appendChild(rgpd_privacy_button);
//     rgpd_actions.appendChild(rgpd_confirm_button);
//     rgpd_wrapper.appendChild(rgpd_actions);
//     document.body.appendChild(rgpd_wrapper);

//     rgpd_confirm_button.addEventListener('click', () => {
//         document.cookie = 'rgpd_accepted=1; max-age=31536000; SameSite=Strict; Secure';
//         hide_rgpd_confirmation();
//     });
// }
// function hide_rgpd_confirmation() {
//     let rgpd_wrapper = document.querySelector('#rgpd-modal');
//     if(rgpd_wrapper) {
//         document.body.removeChild(rgpd_wrapper);
//     }
// }

// function check_rgpd() {
//     if(!document.cookie.split('; ').some(row => row.startsWith('rgpd_accepted'))) {
//         show_rgpd_confirmation();
//     }
// }

// const on_mount = () => {
//     check_rgpd();
// }
// window.addEventListener('onMount', on_mount);