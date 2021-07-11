interface IButton {
  value: string;
  upperCase?: boolean;
  onClick?: () => void;
  type?: 'important' | 'normal' | 'bold';
  className?: string;
  disabled?: boolean;
  isSubmit?: boolean;
}

export default function Button(props: IButton) {
  const upper = props.upperCase ? props.upperCase : true;
  const value = upper ? props.value.toUpperCase() : props.value;
  const buttonType = props.isSubmit ? 'submit' : 'button';

  const buildStyle = (): string => {
    let btnStyle = 'cursor-pointer w-min transition duration-200';

    if (props.className) {
      btnStyle += ' ' + props.className;
    }

    switch (props.type) {
      case 'important':
        btnStyle += ' bg-blue-400 hover:bg-blue-500 text-white px-4 py-2 rounded-lg shadow';
        break;
      case 'bold':
        btnStyle += ' border bg-gray-200 hover:bg-gray-50 px-2 rounded shadow-sm';
        break;
      default:
        btnStyle += ' border bg-gray-50 hover:bg-gray-100 px-2 rounded shadow-sm';
        break;
    }

    return btnStyle;
  };

  return (
    <input type={buttonType} value={value} onClick={props.onClick} className={buildStyle()} disabled={props.disabled} />
  );
}
