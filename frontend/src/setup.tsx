import React from 'react';
import ReactDom from 'react-dom';
import Header from './sections/Header';
import ServerInfoSetupPanel from './sections/ServerInfoSetupPanel';

ReactDom.render(<Setup />, document.getElementById('root'));

function Setup() {
  return (
    <div className="w-full h-full">
      <Header />
      <ServerInfoSetupPanel />
    </div>
  );
}
