interface ISpinnerProps {
  size?: 'small' | 'normal' | 'large';
}

export default function Spinner(props: ISpinnerProps) {
  let width: string;
  switch (props.size) {
    case 'small':
      width = '1.25rem';
      break;
    case 'large':
      width = '2rem';
      break;
    default:
      width = '1.5rem';
      break;
  }

  return (
    <div className="transition ease-in-out duration-150 flex flex-row items-center justify-center">
      <svg
        className="animate-spin mr-3"
        width={width}
        xmlns="http://www.w3.org/2000/svg"
        fill="none"
        viewBox="0 0 24 24"
      >
        <circle className="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" strokeWidth="4"></circle>
        <path
          className="opacity-75"
          fill="currentColor"
          d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"
        ></path>
      </svg>
      <span>Loading..</span>
    </div>
  );
}
