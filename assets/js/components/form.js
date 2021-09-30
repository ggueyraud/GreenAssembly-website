/** Abstract class representing a form validator */
export class Validator {
    /**
     * @param {String} message Error message
     */
    constructor(message) {
        // Prevent instanciate it self
        if (this.constructor === Validator) {
            throw new TypeError('Abstract class "Validator" cannot be instantiated directly');
        }

        this.message = message;
    }

    /**
     * Check if a field matching with Validator logic
     * @param {Field} field The field to check
     */
    validate(field) {
        throw 'Cannot call parent method';
    }

    update_error(field) {
        if (field.error_handling_disabled) {
            return;
        }

        let error_element = null;

        if (field instanceof GroupField) {
            if (field.container) {
                error_element = field.container.nextElementSibling;
            }
        } else {
            error_element = field.element.nextElementSibling;
        }

        if (field.is_valid) {
            if (error_element && error_element.classList && error_element.classList.contains('fv_error')) {
                error_element.classList.remove('active');

                if (!(field instanceof GroupField && field.container)) {
                    field.element.classList.remove('border_warning');
                }
            }
        } else {
            // If no error exist, create it
            if (!error_element || (error_element && !error_element.classList.contains('fv_error'))) {
                error_element = document.createElement('div');
                error_element.classList.add('fv_error');

                console.log(field instanceof GroupField, field.container)

                if (field instanceof GroupField && field.container) {
                    field.container.insertAdjacentElement('afterend', error_element)
                } else {
                    field.element.insertAdjacentElement('afterend', error_element);
                    field.element.classList.add('border_warning');
                }
            }

            if (!(field instanceof GroupField && field.container)) {
                field.element.classList.add('border_warning');
            }

            error_element.innerHTML = this.message;
            error_element.classList.add('active');
        }
    }
}

/**
 * Validator which check if a Field has it's value length between a range
 */
export class StringLength extends Validator {
    constructor(min, max, message = `La valeur doit être comprise entre ${min} et ${max}`) {
        super(message);

        this.min = min;
        this.max = max;
    }

    validate(field) {
        let is_valid = true;

        const test_length = length => {
            if (length < this.min || length > this.max) {
                is_valid = false;
            }
        }

        if (field instanceof GroupField) {
            for (const ipt of field.element) {
                if (!test_length(ipt.value.length)) {
                    break;
                }
            }
        } else {
            const length = field.element.value.length;

            test_length(length)
        }

        field.is_valid = is_valid;
        super.update_error(field);
        return is_valid
    }
}

/**
 * Validator which check if Field has it's value validate by regex
 */
export class Regex extends Validator {
    constructor(regex, message) {
        super(message);

        this.regex = new RegExp(regex);
    }

    validate(field) {
        let is_valid = true;

        if (field.value) {
            is_valid = this.regex.test(field.element.value);
        }

        field.is_valid = is_valid;
        super.update_error(field);
        return is_valid
    }
}

/**
 * Validator which check if Field has a value
 */
export class Required extends Validator {
    constructor(message = 'Ce champ ne peut être vide') {
        super(message);
    }

    validate(field) {
        let is_valid = false;

        if (field instanceof GroupField) {
            for (const input of field.element) {
                if (input.checked) {
                    is_valid = true;
                    break;
                }
            }
        } else if (field.element.value.length !== 0) {
            is_valid = true;
        }

        field.is_valid = is_valid;
        super.update_error(field);
        return is_valid
    }
}

class Field {
    constructor(element, validators = []) {
        this.element = element;
        this.validators = validators;
        this.type = this.element.type;
        this.is_valid = false;
        this.error_handling_disabled = false;
    }

    get value() {
        return this.element.value
    }

    get name() {
        return this.element.name
    }
}

class GroupField extends Field {
    constructor(elements, validators = [], container = null) {
        super(elements, validators);

        this.container = container;
        this.type = this.element[0].type;
    }

    get value() {
        if (this.type === 'radio') {
            // console.log(this.element)
            return [...this.element].find(element => element.checked === true).value
        } else {
            let values = [];
    
            this.element.forEach(el => {
                values.push(el.value);
            });
    
            return values;
        }
    }

    get name() {
        return this.element[0].name
    }
}

const check_form = fv => {
    // if (this.check_timeout) {
    //     clearTimeout(this.check_timeout);
    // }

    // this.check_timeout = setTimeout(() => {
        fv.check(false)
    // }, 50);
}

function input_event(fv, field) {
    for (const validator of field.validators) {
        if (!validator.validate(field)) {
            fv.form.dispatchEvent(new CustomEvent('invalid'));

            break;
        }
    }

    check_form(fv);
}

const get_detail = fields => {
    let detail = {};
    
    // Extract fields value
    fields.forEach(field => {
        detail[field.name] = field.value;
    });

    return detail;
}

export default class FormValidation {
    constructor(form, options = {}) {
        this.form = form;
        this.fields = new Map();
        this.check_timeout = null;

        if (options.fields) {
            for (const [key, value] of Object.entries(options.fields)) {
                const validators = value.validators;

                // If it's a group of inputs
                if (key.endsWith('[]')) {
                    this.fields.set(key, new GroupField(
                        form.querySelectorAll(`[name=${key.substr(0, key.length - 2)}]`),
                        validators,
                        value.container
                    ));
                // Overwise it's a simple input
                } else {
                    this.fields.set(key, new Field(
                        form.querySelector(`[name=${key}]`),
                        validators
                    ));
                }
            }
        }

        const t = this;

        // Attach for each fields input event
        this.fields.forEach(field => {
            if (field instanceof GroupField) {
                field.element.forEach(el => {
                    el.addEventListener('input', () => {
                        input_event(t, field)
                    });
                });
            } else {
                field.element.addEventListener('input', () => {
                    input_event(t, field)
                });
            }
        });

        const submit_btn = form.querySelector('button[type=submit]');

        if (submit_btn) {
            submit_btn.addEventListener('click', e => {
                e.preventDefault();
    
                if (this.check(true)) {
                    form.dispatchEvent(new CustomEvent('send', { detail: get_detail(this.fields) }));
                }
            });
        }
    }

    is_valid(handle_errors = false) {
        let is_valid = true;

        [...this.fields]
            .filter(([_, value]) => value.is_valid === false)
            .forEach(([_, value]) => {
                // Temporarily deactivates display management errors
                if (!handle_errors) {
                    value.error_handling_disabled = true;
                }

                for (const validator of value.validators) {
                    if (!validator.validate(value)) {
                        if (!handle_errors) {
                            value.error_handling_disabled = false;
                        }

                        is_valid = false;
                        break;
                    }
                }

                if (!handle_errors) {
                    value.error_handling_disabled = false;
                }
            });

        return is_valid;
    }

    check(handle_errors = false) {
        if (this.is_valid(handle_errors)) {
            this.form.dispatchEvent(new CustomEvent('valid', { detail: get_detail(this.fields) }));

            return true
        }

        this.form.dispatchEvent(new CustomEvent('invalid'));

        return false
    }

    on(event_name, callback) {
        this.form.addEventListener(event_name, callback)

        return this
    }

    /**
     * Add a field to be handle by the form
     * @param {string|HTMLElement} field String selector of input or HTMLElement
     * @param {Object} options Options of the field
     */
    add_field(field, options = {}) {
        const type = typeof field;
        let native_field = null;

        if (type === 'object' && field instanceof HTMLElement) {
            native_field = field;
        } else if (type === 'string') {
            if (field.endsWith('[]')) {
                this.fields.set(field, new GroupField(
                    this.form.querySelectorAll(`[name=${field.substr(0, field.length - 2)}]`),
                    options.validators,
                    options.container
                ))

                return;
            }
            
            const input = this.form.querySelector(`[name="${field}"]`);

            if (input) {
                native_field = input;
            }
        } else {
            throw new TypeError('Incorrect field specified');
        }

        let new_field = new Field(native_field, options.validators);
        this.fields.set(native_field.name, new_field);
        native_field.addEventListener('input', () => {
            input_event(this, new_field)
        });
    }

    /**
     * Remove field from form validation
     * @param {string|HTMLElement} field 
     */
    remove_field(field) {
        if (!this.fields.has(
            field instanceof HTMLElement
                ? field.name
                : field
        )) {
            return;
        }

        const type = typeof field;
        let internal_field = null;

        if (type === 'object' && field instanceof HTMLElement) {
            internal_field = this.fields.get(field.name);
        } else if (type === 'string') {
            internal_field = this.fields.get(field);
        } else {
            throw new TypeError('Incorrect field specified');
        }

        // TODO : finish implementation
        if (internal_field instanceof GroupField) {
            internal_field.element.forEach(el => {
                el.removeEventListener('input', input_event)
            });

            // If field displayed an error remove it
            if (internal_field.is_valid === false) {
                if (internal_field.container) {
                    const error = internal_field.container.nextElementSibling;

                    if (error && error.classList && error.classList.contains('fv_error')) {
                        error.remove()
                    }
                }
            }
        } else {
            internal_field.element.removeEventListener('input', input_event);
        }

        this.fields.delete(field);
    }
}