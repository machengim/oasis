import React from 'react';

interface IButton {
  value: string;
  upperCase?: boolean;
  onClick?: () => void;
  style?: 'important' | 'normal' | 'bold';
  className?: string;
  disabled?: boolean;
}

export default function Button(props: IButton) {
  const upper = props.upperCase ? props.upperCase : true;
  const value = upper ? props.value.toUpperCase() : props.value;
  const optionalStyle =
    props.style === 'important'
      ? 'bg-blue-400 hover:bg-blue-500 text-white px-4 py-2 rounded-lg shadow'
      : props.style === 'bold'
      ? 'border bg-gray-200 hover:bg-gray-50 px-2 rounded shadow-sm'
      : 'border bg-gray-50 hover:bg-gray-100 px-2 rounded shadow-sm';
  const style = 'cursor-pointer w-min transition duration-200 ' + optionalStyle + ' ' + props.className;

  return <input type="button" value={value} onClick={props.onClick} className={style} disabled={props.disabled} />;
}
