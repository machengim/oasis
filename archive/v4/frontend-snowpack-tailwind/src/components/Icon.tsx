import React, { useState, useEffect } from 'react';

interface IIconProps {
  type: 'success' | 'error';
  theme?: 'fill';
  size?: 'small' | 'normal' | 'large';
  color?: 'green' | 'yellow' | 'red';
}

export default function Icon(props: IIconProps) {
  const [svgClass, setSvgClass] = useState('w-8 h-8');

  useEffect(() => {
    if (props.color) {
      let newStyle = svgClass + ' ' + 'svg-' + props.color;
      setSvgClass(newStyle);
    }
  }, []);

  const getIcon = () => {
    switch (props.type) {
      case 'success':
        return (
          <path d="M256,48C141.31,48,48,141.31,48,256s93.31,208,208,208,208-93.31,208-208S370.69,48,256,48ZM364.25,186.29l-134.4,160a16,16,0,0,1-12,5.71h-.27a16,16,0,0,1-11.89-5.3l-57.6-64a16,16,0,1,1,23.78-21.4l45.29,50.32L339.75,165.71a16,16,0,0,1,24.5,20.58Z" />
        );
      case 'error':
        return (
          <path d="M256,48C141.31,48,48,141.31,48,256s93.31,208,208,208,208-93.31,208-208S370.69,48,256,48Zm86.63,272L320,342.63l-64-64-64,64L169.37,320l64-64-64-64L192,169.37l64,64,64-64L342.63,192l-64,64Z" />
        );
      default:
        return null;
    }
  };

  return (
    <svg xmlns="http://www.w3.org/2000/svg" className={svgClass} viewBox="0 0 512 512">
      {getIcon()}
    </svg>
  );
}
