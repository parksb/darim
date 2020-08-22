import React, { useState } from 'react';
import { Link } from 'react-router-dom';
import styled from 'styled-components';

import { getI18n } from '../../utils/i18n';
import * as api from '../../api/auth';
import { TextField, Button, Container, Section, LoadingDots } from '../../components';
import { Session } from '../../models';

interface Props {
  session_state: [Session | null, React.Dispatch<React.SetStateAction<Session | null>>]
}

const SignUpButton = styled(Button)`
  border-left: 0;
`;

const FullWidthTextField = styled(TextField)`
  flex: 1;
`;

const ForgotPasswordLink = styled(Link)`
  font-size: 14px;
  color: #0366d6;
  text-decoration: none;

  &:hover {
    text-decoration: underline;
  }
`;

const Login: React.FC<Props> = ({ session_state }) => {
  const [email, setEmail] = useState('');
  const [password, setPassword] = useState('');
  const setSession = session_state[1];

  const [isSigning, setIsSigning] = useState(false);

  const i18n = getI18n({
    signIn: {
      ko: '로그인 ↗',
      en: 'Sign in ↗',
    },
    signUp: {
      ko: '회원가입 ↗',
      en: 'Sign up ↗',
    },
    signing: {
      ko: '안전한 암호화 알고리즘으로 로그인 중입니다',
      en: 'Signing up with secure encryption algorithm',
    },
    forgotPassword: {
      ko: '비밀번호를 잊으셨나요?',
      en: 'Forgot password?',
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

  return <Container>
    {!isSigning ? (
      <Section>
        <Section row>
          <FullWidthTextField type='email' placeholder={i18n.text('email')} value={email} onChange={({ target: { value } }) => setEmail(value)} />
          <FullWidthTextField type='password' placeholder={i18n.text('password')} value={password} onChange={({ target: { value } }) => setPassword(value)}/>
          <Button onClick={login}>{i18n.text('signIn')}</Button>
          <Link to='/join'>
            <SignUpButton>{i18n.text('signUp')}</SignUpButton>
          </Link>
        </Section>
        <Section top={12}>
          <span><ForgotPasswordLink to='/password_reset'>{i18n.text('forgotPassword')}</ForgotPasswordLink></span>
        </Section>
      </Section>
    ) : (
      <Section row>
        {i18n.text('signing')}
        <LoadingDots />
      </Section>
    )}
  </Container>
};

export default Login;
