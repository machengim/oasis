import Icon from './Icon';
import './Notification.css';

interface INotificationProps {
  type: 'success' | 'error';
}

export default function Notification(props: INotificationProps) {
  const type = props.type;

  const buildStyle = (): string => {
    let notificationClass =
      'flex flex-row justify-between space-x-2  bg-gray-50 shadow rounded border-l-4 items-center p-2 notification';

    switch (props.type) {
      case 'success':
        notificationClass += ' border-green-400';
        break;
      case 'error':
        notificationClass += ' border-red-400';
        break;
      default:
        break;
    }
    return notificationClass;
  };

  return (
    <div className="absolute right-6 top-6 max-w-sm overflow-x-hidden">
      <div className={buildStyle()}>
        <div>
          <Icon type={type} theme="fill" color={type === 'success' ? 'green' : 'red'} />
        </div>
        <div className="flex flex-col">
          <div className="text-gray-700 font-bold">Success</div>
          <div className="text-sm">You've successfully added something</div>
        </div>
        <div className="text-3xl">&times;</div>
      </div>
    </div>
  );
}
