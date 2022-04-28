import React, { useState } from 'react';
import { Navigate } from 'react-router-dom';
import styled from 'styled-components';

import { getI18n } from '../../utils/i18n';
import * as api from '../../api/auth';
import { Button, Container, TextField, Section, LoadingDots } from '../../components';

const FullWidthTextField = styled(TextField)`
  flex: 3;
  width: 100%;
`;

const JoinForm: React.FC = () => {
  const [email, setEmail] = useState('');
  const [password, setPassword] = useState('');
  const [name, setName] = useState('');

  const [key, setKey] = useState('');
  const [isSettingToken, setIsSettingToken] = useState(false);
  const [isSetToken, setIsSetToken] = useState(false);

  const i18n = getI18n({
    signingUp: {
      ko: '안전한 암호화 알고리즘으로 회원가입 중입니다',
      en: 'Signing up with secure encryption algorithm',
    },
    signUp: {
      ko: '회원가입',
      en: 'Sign up',
    }
  });

  const setSignUpToken = async () => {
    setIsSettingToken(true);
    const result = await api.setSignUpToken(name, email, password);

    if (result) {
      setKey(result);
      setIsSettingToken(false);
      setIsSetToken(true);
    } else {
      setIsSettingToken(false);
    }
  };

  return <Container>
    {isSetToken ? (
      <Navigate to={`/join/${key}`} />
    ) : (
      !isSettingToken ? (
        <form onSubmit={setSignUpToken}>
          <Section row nowrap>
            <FullWidthTextField type='email' placeholder={i18n.text('email')} value={email} onChange={({ target: { value } }) => setEmail(value)} />
            <FullWidthTextField type='password' placeholder={i18n.text('password')} value={password} onChange={({ target: { value } }) => setPassword(value)}/>
            <FullWidthTextField type='text' placeholder={i18n.text('name')} value={name} onChange={({ target: { value } }) => setName(value)} />
            <Button type='submit'>{`${i18n.text('signUp')} ↗`}</Button>
          </Section>
        </form>
      ) : (
        <Section row>
          {i18n.text('signingUp')}
          <LoadingDots />
        </Section>
      )
    )}
  </Container>
};

export default JoinForm;
