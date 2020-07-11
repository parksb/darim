import React, { useState } from 'react';
import { Link } from 'react-router-dom';
import styled from 'styled-components';

import * as api from '../../api/auth';
import { TextField, Button, Container, LoadingDots } from '../../components';
import { Session } from '../../models';
import I18n from '../../utils/i18n';

interface Props {
  session_state: [Session | null, React.Dispatch<React.SetStateAction<Session | null>>]
}

const SignUpButton = styled(Button)`
  border-left: 0;
`;

const FullWidthTextField = styled(TextField)`
  flex: 1;
`;

const Login: React.FC<Props> = ({ session_state }) => {
  const [email, setEmail] = useState('');
  const [password, setPassword] = useState('');
  const [session, setSession] = session_state;

  const [isSigning, setIsSigning] = useState(false);

  const i18n = new I18n({
    signIn: {
      ko: '로그인 ↗',
      en: 'Sign in ↗',
    },
    signUp: {
      ko: '회원가입 ↗',
      en: 'Create account ↗',
    },
    signing: {
      ko: '안전한 암호화 알고리즘으로 로그인 중입니다',
      en: 'Signing up with secure encryption algorithm',
    },
  });

  const login = async () => {
    setIsSigning(true);
    const result = await api.login(email, password);
    if (result) {
      setIsSigning(false);
      setSession(result);
    } else {
      setIsSigning(false);
    }
  };

  return <Container row>
    {!isSigning ? (
      <>
        <FullWidthTextField type='email' placeholder={i18n.text('email')} value={email} onChange={({ target: { value } }) => setEmail(value)} />
        <FullWidthTextField type='password' placeholder={i18n.text('password')} value={password} onChange={({ target: { value } }) => setPassword(value)}/>
        <Button onClick={login}>{i18n.text('signIn')}</Button>
        <Link to='/join'>
          <SignUpButton>{i18n.text('signUp')}</SignUpButton>
        </Link>
      </>
    ) : (
      <>
        {i18n.text('signing')}
        <LoadingDots />
      </>
    )}
  </Container>
};

export default Login;
