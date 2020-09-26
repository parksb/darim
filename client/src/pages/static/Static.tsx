import React from 'react';
import { Redirect, Switch, Route, useRouteMatch } from 'react-router-dom';

// eslint-disable-next-line @typescript-eslint/ban-ts-comment
// @ts-ignore
import privacy from '../../../public/static/privacy.html';

// eslint-disable-next-line @typescript-eslint/ban-ts-comment
// @ts-ignore
import terms from '../../../public/static/terms.html';

import { Section } from '../../components';

const Static: React.FC = () => {
  const { path } = useRouteMatch();

  return <Section>
      <Switch>
        <Route path={`${path}/terms`}>
          <Section dangerouslySetInnerHTML={{__html: terms}} />
        </Route>
        <Route path={`${path}/privacy`}>
          <Section dangerouslySetInnerHTML={{__html: privacy}} />
        </Route>
        <Redirect to='/' />
      </Switch>
  </Section>;
};

export default Static;
