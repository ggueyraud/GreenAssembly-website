const validators = {
    not_empty: value => value.length === 0 ? false : true,
    string_length: (value, range = {}) => {
        let valid = true
        if (range.min !== -1 && value.length < range.min) valid = false
        if (range.max !== -1 && value.length > range.max) valid = false
        
        return valid
    },
    less_than: (value, max) => value.length > max,
    greater_than: (value, min) => value.length < min
}

class Field {
    constructor(elements, validators) {
        this.validators = validators;
        this.elements = elements;
    }

    get_value() {
        let values = [];

        this.elements.forEach(el => {
            const { type } = el;

            if (type === 'checkbox' || type === 'radio') {
                if (el.checked) values.push(el.value);
            } else {
                values.push(el.value);
            }
        })

        return values.length === 1 && this.elements[0].type !== 'checkbox' ? values[0] : values;
    }

    validate(display_error = true) {
        if (Object.keys(this.validators).length === 0) return true;

        let error = null;

        for (const validator_name in this.validators) {
            const type = this.elements[0].type;

            if ((type === 'checkbox' || type === 'radio') && validator_name === 'notEmpty') {
                let at_least_one_checked = false;

                this.elements.forEach(el => {
                    if (el.checked === true) {
                        at_least_one_checked = true;
                    }
                });

                if (!at_least_one_checked) {
                    error = this.validators[validator_name];
                }
            } else if (type === 'text' || type === 'email' || type === 'textarea') {
                this.elements.forEach(el => {
                    const { value, minLength, maxLength, name, required } = el;

                    if (value.length > 0 && validator_name === 'stringLength') {
                        if (!validators.string_length(value, {
                            min: minLength,
                            max: maxLength
                        })) {
                            error = this.validators[validator_name];
                        }
                    } else if (required === true && validator_name === 'notEmpty') {
                        if (!validators.not_empty(value)) {
                            error = this.validators[validator_name];
                        }
                    }
                })
            }
        }

        if (display_error) {
            this.update_error(error);
        }

        return error === null;
    }

    update_error(error) {
        let last = this.elements.length === 0
            ? this.elements[0]
            : this.elements[this.elements.length - 1];
        let next_el = this.validators.container
            ? this.validators.container.nextSibling
            : last.nextElementSibling;

        if (error) {
            if (!next_el || (next_el && !next_el.classList.contains('fv_error'))) {
                next_el = document.createElement('div');
                next_el.classList.add('fv_error');

                if (this.validators.container) {
                    this.validators.container.insertAdjacentElement('afterend', next_el);
                } else {
                    last.insertAdjacentElement('afterend', next_el);
                }
            }
            
            next_el.innerHTML = error;
            next_el.classList.add('active');
            last.classList.add('border_warning');
        } else {
            if (next_el && next_el.classList && next_el.classList.contains('fv_error')) {
                // next_el.remove();
                next_el.classList.remove('active');
                last.classList.remove('border_warning');
            }
        }
    }
}

const input_event = (form, fields, current_field) => {
    let is_form_valid = true;
    let detail = {}

    if (current_field.validate()) {
        if (Object.keys(fields).length == 1) {
            detail[current_field.elements[0].name] = current_field.get_value();
        } else {
            for (const f_name in fields) {
                const f = fields[f_name];

                if (f !== current_field) {
                    if (!f.validate(false)) {
                        is_form_valid = false;
                        return;
                    }

                    detail[current_field.elements[0].name] = f.get_value();
                }
            }
        }
    } else {
        is_form_valid = false;
    }

    if (is_form_valid) {
        form.dispatchEvent(new CustomEvent('valid', {
            detail
        }));
    } else {
        form.dispatchEvent(new CustomEvent('invalid', {
            detail
        }));
    }
}

export default class {
    constructor(form, options = {}) {
        this.el = form;
        this.fields = {};

        if (options.fields) {
            for (const field_name in options.fields) {
                let field = new Field(
                    form.querySelectorAll(`[name="${field_name}"]`),
                    options.fields[field_name].validators || {}
                );

                this.fields[field_name] = field;
            }
        } else {
            const fields = form.querySelectorAll('input, textarea, select');

            fields.forEach(native_field => {
                let validators = {};
                let { minLength, maxLength, required, dataset } = native_field;

                if ((minLength !== -1 || maxLength !== -1)) {
                    validators.stringLength = dataset['fvString_lengthMessage'] || `Cannot be under ${minLength} characters`;
                }

                if (required === true) {
                    validators.notEmpty = dataset['fvNot_emptyMessage'] || 'Cannot be empty';
                }

                let field = new Field(
                    [native_field],
                    validators
                );

                this.fields[native_field.name] = field;
            })
        }
    
        this.handle_fields()

        const submit = form.querySelector('[type="submit"]');

        if (submit) {
            submit.addEventListener('click', e => {
                e.preventDefault();
        
                let valid = true;
                let detail = {};

                for (const field_name in this.fields) {
                    const field = this.fields[field_name];

                    if (!field.validate(false)) {
                        valid = false;
                        return
                    }
        
                    detail[field.elements[0].name] = field.get_value() || null;
                }
        
                if (valid) {
                    form.dispatchEvent(new CustomEvent('send', {
                        detail
                    }))
                }
            })
        }
    }

    handle_fields() {
        for (const field_name in this.fields) {
            const field = this.fields[field_name];

            field.elements.forEach(el => {
                el.addEventListener('input', () => {
                    input_event(this.el, this.fields, field)
                })
            })
        }

        // this.fields.forEach(custom_field => {
        //     custom_field.elements.forEach(el => {
        //         el.addEventListener('input', () => {
        //             let is_form_valid = true;
        //             let detail = {}

        //             if (custom_field.validate()) {
        //                 if (this.fields.length == 1) {
        //                     detail[custom_field.elements[0].name] = custom_field.get_value();
        //                 } else {
        //                     this.fields.forEach(f => {
        //                         if (f !== custom_field) {
        //                             if (!f.validate(false)) {
        //                                 is_form_valid = false;
        //                                 return;
        //                             }
    
        //                             detail[custom_field.elements[0].name] = custom_field.get_value();
        //                         }
        //                     })
        //                 }
        //             } else {
        //                 is_form_valid = false;
        //             }

        //             if (is_form_valid) {
        //                 this.el.dispatchEvent(new CustomEvent('valid', {
        //                     detail
        //                 }));
        //             } else {
        //                 this.el.dispatchEvent(new CustomEvent('invalid', {
        //                     detail
        //                 }));
        //             }
        //         })
        //     })
        // })
    }

    on(event_name, callback) {
        this.el.addEventListener(event_name, callback)

        return this
    }

    add_field(field) {
        const type = typeof field;
        let native_field = null;

        if (type === 'object' && field instanceof HTMLElement) {
            native_field = field;
        } else if (type === 'string') {
            const f = this.el.querySelector(`[name="${field}"]`);

            if (f) {
                native_field = f;
            }
        } else {
            throw 'Incorrect field specified';
        }

        let validators = {};
        let { minLength, maxLength, required, dataset } = native_field;

        if ((minLength !== -1 || maxLength !== -1)) {
            validators.stringLength = dataset['fvString_lengthMessage'] || `Cannot be under ${minLength} characters`;
        }

        if (required === true) {
            validators.notEmpty = dataset['fvNot_emptyMessage'] || 'Cannot be empty';
        }

        this.fields[native_field.name] = new Field(
            [native_field],
            validators
        );
    }

    remove_field(field_name) {
        const field = this.fields[field_name];

        if (field) {
            field.elements.forEach(el => {
                // TODO : make sure element is input or textarea
                el.value = '';
                el.removeEventListener('input', input_event);
            });
            delete this.fields[field_name];
        }
    //     this.fields.forEach((field, index) => {
    //         if (field.name === field_name) {
    //             field.removeEventListener('input', input_event);

    //             let error = field.nextElementSibling;

    //             if (error && error.classList.contains('fv_error')) {
    //                 error.remove();
    //                 field.classList.remove('border-yellow-500');
    //             }

    //             this.fields.splice(index, 1);
    //             return;
    //         }
    //     })
    }
}