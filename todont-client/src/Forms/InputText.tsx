interface InputTextProps {
    id: string;
    name: string;
    className: string;
    value: string;
    onChange: (value: string) => void;
    errors: string[];
}

export default function InputText(props: InputTextProps) {
    function borderClass() {
        if (props.value)
            return props.errors.length === 0 ? 'border-success-500' : 'border-danger-500';
        return '';
    }

    function errorList() {
        if (props.errors.length === 0)
            return;

        return (
            <ul className="mt-1 text-danger-500 list-disc list-inside">
                {props.errors.map((e) => (
                    <li>{e}</li>
                ))}
            </ul>
        )
    }

    return (
        <>
            <input
                id={props.id}
                name={props.name}
                className={borderClass() + ' ' + props.className}
                value={props.value}
                onChange={e => props.onChange(e.target.value)}/>
            {errorList()}
        </>
    )
}
