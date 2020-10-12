import React from 'react';
import styled from 'styled-components';
import { Link } from 'react-router-dom';

import { getI18n } from '../utils/i18n';
import { Section } from './index';

const StyledSection = styled(Section)`
  height: 15px;
  background-color: #e01060;
  justify-content: center;
  padding: 10px;
  color: #ffffff;
  font-size: 14px;

  & > a {
    margin-left: 5px;
    color: #ffce05;
    text-decoration: underline;

    &:hover {
     color: #e01060;
     background-color: #ffce05;
    }
  }
`;

const i18n = getI18n({
  secretKeyWarning: {
    ko: '⚠️ 비밀키가 설정되어 있지 않아 글을 볼 수 없습니다.',
    en: '⚠️ Unable to load posts because the secret key is not set.',
  },
  setSecretKey: {
    ko: '비밀키 설정하기',
    en: 'Set a secret key',
  },
});

const SecretKeyWarningBar: React.FC = ({ ...props }) => {
  return <StyledSection row {...props}>
    {i18n.text('secretKeyWarning')}
    <Link to='/settings/security'>{i18n.text('setSecretKey')}</Link>
  </StyledSection>
};

export default SecretKeyWarningBar;
