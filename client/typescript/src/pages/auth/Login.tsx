import React, { useState } from 'react';
import { Link } from 'react-router-dom';
import styled from 'styled-components';

import * as api from './api';
import { TextField, Button, Section } from '../../components';
import { Session } from '../../models';

interface Props {
  session_state: [Session | null, React.Dispatch<React.SetStateAction<Session | null>>]
}

const Container = styled(Section)`
  margin-bottom: 30px;
`;

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

  const login = async () => {
    const result = await api.login(email, password);
    if (result) {
      setSession(result);
    } else {
      alert('로그인실패했습니다.');
    }
  };

  return <Container row>
    <FullWidthTextField type='email' placeholder='Email' value={email} onChange={({ target: { value } }) => setEmail(value)} />
    <FullWidthTextField type='password' placeholder='Password' value={password} onChange={({ target: { value } }) => setPassword(value)}/>
    <Button onClick={login}>Sign in</Button>
    <Link to='/join'>
      <SignUpButton>Sign up ↗</SignUpButton>
    </Link>
  </Container>
};

export default Login;
