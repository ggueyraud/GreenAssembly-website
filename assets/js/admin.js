import Router from 'router';

document.addEventListener('readystatechange', e => {
    if (e.target.readyState === 'complete') {
        window.router = new Router();
    }
});