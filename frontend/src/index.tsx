import React from 'react';
import ReactDom from 'react-dom';

ReactDom.render(<App />, document.getElementById('root'));

function App() {
  return <div className="text-md">Snowpack with React 1</div>;
}
