import Form, { Required, Regex } from 'formvalidation';
import { post } from './utils/http';

const on_mount = () => {
    new Form(document.querySelector('form'), {
        fields: {
            email: {
                validators: [
                    new Required(),
                    new Regex(/^(([^<>()[\]\\.,;:\s@"]+(\.[^<>()[\]\\.,;:\s@"]+)*)|(".+"))@((\[[0-9]{1,3}\.[0-9]{1,3}\.[0-9]{1,3}\.[0-9]{1,3}\])|(([a-zA-Z\-0-9]+\.)+[a-zA-Z]{2,}))$/, `L'email saisit n'a pas un format valide`)
                ]
            }
        }
    })
        .on('valid', e => {
            e.target.querySelector('[type=submit]').removeAttribute('disabled');
        })
        .on('invalid', e => {
            e.target.querySelector('[type=submit]').setAttribute('disabled', 'true');
        })
        .on('send', e => {
            e.preventDefault();
            
            post('/newsletter/subscribe', {
                headers: {
                    'Content-Type': 'application/x-www-form-urlencoded'
                },
                body: new URLSearchParams(e.detail)
            })
                .then(res => {
                    if (res.ok) {
                        // document.querySelector("#success").style.setProperty('display', 'block');
                        // document.querySelector('.stepper').classList.add('hidden');
                    }
                })
                .catch(() => {
                    // server_error.classList.remove('hidden');
                })
        });
}

window.addEventListener('onMount', on_mount)
window.addEventListener('onDestroy', () => {
    window.removeEventListener('onMount', on_mount);
});