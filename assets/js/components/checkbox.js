export default (el, options) => {
    const elements = [];

    options = Object.assign(
        {
            role: 'checkbox',
            checked: false
        },
        options
    );

    if (!['checkbox', ' menuitemradio'].includes(options.role)) {
        throw 'Unexpected value for options.role'
    }

    document
        .querySelectorAll(el)
        .forEach(chk => {
            const input = chk.querySelector('input');

            chk.setAttribute('aria-checked', options.checked);
            chk.setAttribute('role', options.role);

            const new_element = {
                set checked(checked) {
                    chk.setAttribute('aria-checked', checked);
                    input.checked = false;
                },
                get checked() {
                    Boolean(this.container.getAttribute('aria-checked'))
                },
                // container: chk
            };

            if (options.checked) {
                // new_element.checked = options.checked;
                input.checked = options.checked;
            }
            
            input.addEventListener('click', () => {
                new_element.checked = input.checked;
            });

            elements.push(new_element);
        })

    return elements
}