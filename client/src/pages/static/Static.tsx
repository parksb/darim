import React from 'react';
import { Redirect, Route, Switch, useRouteMatch } from 'react-router-dom';
import styled from 'styled-components';
import { I18n, I18nLanguages } from 'snowball-js';

import privacyKo from '../../../public/static/privacy_ko.md';
import termsKo from '../../../public/static/terms_ko.md';
import privacyEn from '../../../public/static/privacy_en.md';
import termsEn from '../../../public/static/terms_en.md';

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
  const userLanguage = I18n.getUserLanguage()

  const getTerms = () => {
    switch (userLanguage) {
      case I18nLanguages.KO:
        return termsKo;
      default:
        return termsEn;
    }
  }

  const getPrivacy = () => {
    switch (userLanguage) {
      case I18nLanguages.KO:
        return privacyKo;
      default:
        return privacyEn;
    }
  }

  return <Section>
      <Switch>
        <Route path={`${path}/terms`}>
          <Frame dangerouslySetInnerHTML={{__html: getTerms()}} />
        </Route>
        <Route path={`${path}/privacy`}>
          <Frame dangerouslySetInnerHTML={{__html: getPrivacy()}} />
        </Route>
        <Redirect to='/' />
      </Switch>
  </Section>;
};

export default Static;
