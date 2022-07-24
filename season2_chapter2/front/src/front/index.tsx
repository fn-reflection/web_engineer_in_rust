import React from 'react';
import ReactDOM from 'react-dom';
import {Ruitter} from './Ruitter';

const rootPage = (
  <React.StrictMode>
    <Ruitter />
  </React.StrictMode>
);

const domRoot = document.querySelector('#root');
ReactDOM.render(rootPage,domRoot);