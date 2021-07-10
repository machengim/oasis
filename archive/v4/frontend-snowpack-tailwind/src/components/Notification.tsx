import React from 'react';
import Icon from './Icon';
import './notification.css';

interface INotificationProps {
  type: 'success' | 'error';
}

export default function Notification(props: INotificationProps) {
  const type = props.type;

  let notificationClass = 'flex flex-row justify-between space-x-2  bg-gray-50 shadow rounded items-center p-2';

  switch (props.type) {
    case 'success':
      notificationClass += ' border-green';
      break;
    case 'error':
      notificationClass += ' border-red';
      break;
    default:
      break;
  }

  return (
    <div className="absolute right-6 top-6 max-w-sm">
      <div className={notificationClass}>
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
