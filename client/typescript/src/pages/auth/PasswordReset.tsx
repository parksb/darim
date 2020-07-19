import React, { useState } from 'react';
import { Redirect } from 'react-router-dom';
import styled from 'styled-components';

import * as authApi from '../../api/auth';
import * as userApi from '../../api/user';
import { Button, Container, Section, TextField } from '../../components';
import I18n from '../../utils/i18n';

const FullWidthTextField = styled(TextField)`
  flex: 1;
`;

const NonBorderFullWidthTextField = styled(FullWidthTextField)`
  border-top: 0;
`;

const NonBorderButton = styled(Button)`
  border-top: 0;
`;

const PasswordReset: React.FC = () => {
  const [isSentEmail, setIsSentEmail] = useState(false);
  const [isResetPassword, setIsResetPassword] = useState(false);

  const [email, setEmail] = useState('');
  const [tokenId, setTokenId] = useState('');
  const [temporaryPassword, setTemporaryPassword] = useState('');
  const [newPassword, setNewPassword] = useState('');

  const i18n = new I18n({
    send: {
      ko: '메일 보내기',
      en: 'Send email',
    },
    tokenId: {
      ko: '토큰 ID',
      en: 'Token ID',
    },
    temporaryPassword: {
      ko: '임시 비밀번호',
      en: 'Temporary password',
    },
    newPassword: {
      ko: '새 비밀번호',
      en: 'New password',
    },
    confirm: {
      ko: '확인 ↗',
      en: 'Confirm ↗',
    },
    info: {
      ko: `새 비밀번호를 만들기 위한 토큰과 임시 비밀번호가 발송되었습니다. 이메일(${email})을 확인해주세요.`,
      en: `Please check your email (${email}) to reset your password`,
    },
  });

  const sendEmail = async () => {
    const result = await authApi.setPasswordToken(email);
    if (result) {
      setIsSentEmail(true);
    }
  };

  const resetPassword = async () => {
    const result = await userApi.resetPassword(email, tokenId, temporaryPassword, newPassword);
    if (result) {
      setIsResetPassword(true);
    }
  };

  return <Container bottom={30}>
    {!isSentEmail ? (
      <Section row>
        <FullWidthTextField type='email' placeholder={i18n.text('email')} value={email} onChange={({ target: { value } }) => setEmail(value)} />
        <Button onClick={() => email && sendEmail()}>{i18n.text('send')}</Button>
      </Section>
    ) : (
      <Section>
        {i18n.text('info')}
        <Section top={30} row>
          <FullWidthTextField type='text' placeholder={i18n.text('tokenId')} value={tokenId} onChange={({ target: { value } }) => setTokenId(value)} />
          <FullWidthTextField type='password' placeholder={i18n.text('temporaryPassword')} value={temporaryPassword} onChange={({ target: { value } }) => setTemporaryPassword(value)} />
        </Section>
        <Section row>
          <NonBorderFullWidthTextField type='password' placeholder={i18n.text('newPassword')} value={newPassword} onChange={({ target: { value } }) => setNewPassword(value)} />
          <NonBorderButton onClick={resetPassword}>{i18n.text('confirm')}</NonBorderButton>
        </Section>
        {isResetPassword && <Redirect to='/' />}
      </Section>
    )}
  </Container>
};

export default PasswordReset;
