import React from 'react';
import { Navigate, Route, Routes } from 'react-router-dom';
import styled from 'styled-components';
import MarkdownIt from 'markdown-it';

import { I18n, I18nLanguages } from '../../utils/i18n';
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
  const userLanguage = I18n.getUserLanguage();
  const md = new MarkdownIt({
    html: false,
    xhtmlOut: false,
    breaks: false,
    langPrefix: 'language-',
    linkify: true,
    typographer: true,
    quotes: '“”‘’',
  });

  const getTerms = () => {
    switch (userLanguage) {
      case I18nLanguages.KO:
        return md.render(termsKo);
      default:
        return md.render(termsEn);
    }
  };

  const getPrivacy = () => {
    switch (userLanguage) {
      case I18nLanguages.KO:
        return md.render(privacyKo);
      default:
        return md.render(privacyEn);
    }
  };

  return <Section>
      <Routes>
        <Route path='terms' element={<Frame dangerouslySetInnerHTML={{ __html: getTerms() }} />} />
        <Route path='privacy' element={<Frame dangerouslySetInnerHTML={{ __html: getPrivacy() }} />} />
        <Route path='*' element={<Navigate to='/' />} />
      </Routes>
  </Section>;
};

export default Static;
