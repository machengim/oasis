import React from 'react';

interface IButton {
  value: string;
  upperCase?: boolean;
}

export default function Button(props: IButton) {
  const upper = props.upperCase ? props.upperCase : true;
  const value = upper ? props.value.toUpperCase() : props.value;

  return (
    <input
      type="button"
      value={value}
      className="cursor-pointer shadow bg-blue-400 hover:bg-blue-500 transition duration-200 text-white px-4 py-2 rounded-lg"
    />
  );
}
