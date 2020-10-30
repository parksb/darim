import React from 'react';
import { Redirect, Switch, Route, useRouteMatch } from 'react-router-dom';
import styled from 'styled-components';

import privacy from '../../../public/static/privacy.md';
import terms from '../../../public/static/terms.md';

import { Section } from '../../components';

const Frame = styled(Section)`
  word-wrap: break-word;
  line-height: 170%;

  h1, h2, h3 { font-weight: bold; margin-bottom: 10px; }
  h2, h3 { margin-top: 20px; }
  h1 { font-size: 22px; }
  h2 { font-size: 18px; }
  h3 { font-size: 14px; }
  p { margin-top: 10px; }
  p, li { font-size: 14px; }
  ul, ol { margin-top: 10px; }
  ul { list-style: disc inside; }
  ol { list-style: numeric inside; }
`;

const Static: React.FC = () => {
  const { path } = useRouteMatch();

  return <Section>
      <Switch>
        <Route path={`${path}/terms`}>
          <Frame dangerouslySetInnerHTML={{__html: terms}} />
        </Route>
        <Route path={`${path}/privacy`}>
          <Frame dangerouslySetInnerHTML={{__html: privacy}} />
        </Route>
        <Redirect to='/' />
      </Switch>
  </Section>;
};

export default Static;
