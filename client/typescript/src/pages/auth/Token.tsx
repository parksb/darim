import React, { useState } from 'react';
import { Link, useParams } from "react-router-dom";
import styled from 'styled-components';
import CopyToClipboard from 'react-copy-to-clipboard';

import * as api from '../../api/user';
import { Button, Container, TextField, Section } from '../../components';
import Secret from '../../utils/secret';
import I18n from '../../utils/i18n';

const FullWidthTextField = styled(TextField)`
  flex: 1;
`;

const InfoSection = styled(Section)`
  line-height: 150%;
  margin-bottom: 20px;
`;

const KeySection = styled(Section)`
  padding: 15px;
  background-color: #f0f0f0;
`;

const PublicKeySection = styled(KeySection)`
  margin-top: 20px;
`;

const PrivateKeySection = styled(KeySection)`
  margin: 10px 0 20px;
`;

const CopyButton = styled(Button)`
  margin-left: 10px;
`;

const GoToSignInButton = styled(Button)`
  width: 100%;
`;

const Token: React.FC = () => {
  const { key } = useParams();

  const [pin, setPin] = useState('');
  const [privateKey, setPrivateKey] = useState('');
  const [publicKey, setPublicKey] = useState('');

  const i18n = new I18n({
    info: {
      'ko-KR': '👋 환영합니다! 글을 안전하게 암호화하기 위해 사용할 공개키와 비밀키를 준비했습니다:',
      'en-US': '👋 Welcome to Darim! This is your public key and secret key that will be used to encrypt your posts:',
    },
    verify: {
      'ko-KR': '인증 ↗',
      'en-US': 'Verify ↗',
    },
    pin: {
      'ko-KR': '인증키',
      'en-US': 'Pin',
    },
    downloadPublicKey: {
      'ko-KR': '공개키 파일 다운로드',
      'en-US': 'Download the public key as file',
    },
    copyPublicKey: {
      'ko-KR': '공개키 복사하기',
      'en-US': 'Copy the public key to clipboard',
    },
    downloadPrivateKey: {
      'ko-KR': '비밀키 파일 다운로드',
      'en-US': 'Download the secret key as file',
    },
    copyPrivateKey: {
      'ko-KR': '비밀키 복사하기',
      'en-US': 'Copy the secret key to clipboard',
    },
    notice: {
      'ko-KR': '공개키와 비밀키를 잃어버리지 마세요. 키 파일을 다운로드받아 안전한 곳에 두거나, 키를 복사해 다른 곳에 보관할 것을 강력히 권장합니다. 또한, 절대로 비밀키를 다른 사람에게 알려주지 마세요.',
      'en-US': 'Don\'t lose your public key and secret key. It is strongly recommended that you download the key files and store it in a secure place, or copy the keys to somewhere else. Also, NEVER let anyone know your secret key.',
    },
    goToSignIn: {
      'ko-KR': '로그인하러 가기 ↗',
      'en-US': 'Go to sign in ↗',
    },
  });

  const verify = async () => {
    const generatedPublicKey = Secret.getRandomString();
    const generatedPrivateKey = Secret.getRandomString();
    const encryptedPrivateKey = Secret.encryptAES(generatedPrivateKey, generatedPublicKey);

    const result = await api.createUser(generatedPublicKey, key, pin);
    if (result) {
      Secret.setPrivateKeyToLocalStorage(encryptedPrivateKey);
      setPrivateKey(encryptedPrivateKey);
      setPublicKey(generatedPublicKey);
    }
  };

  const getDownloadURLOfTextFile = (text: string) => {
    const blob = new Blob([text], { type: 'text/plain' });
    return URL.createObjectURL(blob);
  };

  return <Container bottom={30}>
    {!privateKey ? (
      <Section row>
        <FullWidthTextField type='text' placeholder={i18n.text('pin')} value={pin} onChange={({ target: { value } }) => setPin(value)} />
        <Button onClick={verify}>{i18n.text('verify')}</Button>
      </Section>
    ) : (
      <>
        <InfoSection>
          <PublicKeySection row>
            <a download='darim-public-key.txt' href={getDownloadURLOfTextFile(publicKey)}>
              <Button>{i18n.text('downloadPublicKey')}</Button>
            </a>
            <CopyToClipboard text={publicKey}>
              <CopyButton>{i18n.text('copyPublicKey')}</CopyButton>
            </CopyToClipboard>
          </PublicKeySection>
          <PrivateKeySection row>
            <a download='darim-secret-key.txt' href={getDownloadURLOfTextFile(privateKey)}>
              <Button>{i18n.text('downloadPrivateKey')}</Button>
            </a>
            <CopyToClipboard text={privateKey}>
              <CopyButton>{i18n.text('copyPrivateKey')}</CopyButton>
            </CopyToClipboard>
          </PrivateKeySection>
          {i18n.text('notice')}
        </InfoSection>
        <Link to='/'>
          <GoToSignInButton>
            {i18n.text('goToSignIn')}
          </GoToSignInButton>
        </Link>
      </>
    )}
  </Container>
};

export default Token;
