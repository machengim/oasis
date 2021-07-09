import React from 'react';
import Header from '../sections/Header';
import ServerInfoSetupPanel from '../sections/ServerInfoSetupPanel';

export default function Setup() {
  return (
    <div className="w-full h-full">
      <Header />
      <ServerInfoSetupPanel />
    </div>
  );
}
