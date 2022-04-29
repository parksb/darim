import React from 'react';
import { Link } from 'react-router-dom';
import styled from 'styled-components';
import CopyToClipboard from 'react-copy-to-clipboard';

import { getI18n } from '../../utils/i18n';
import { Button, Container, Section } from '../../components';

interface Props {
  publicKey: string;
  privateKey: string;
}

const InfoSection = styled(Section)`
  line-height: 150%;
`;

const KeySection = styled(Section)`
  padding: 15px;
  background-color: #f0f0f0;
`;

const CopyButton = styled(Button)`
  margin-left: 10px;
`;

const GoToSignInButton = styled(Button)`
  width: 100%;
`;

const Token: React.FC<Props> = ({ publicKey, privateKey }) => {
  const i18n = getI18n({
    info: {
      ko: '👋 환영합니다! 다이어리를 안전하게 암호화하기 위해 사용할 공개키와 비밀키를 준비했습니다:',
      en: '👋 Welcome to Darim! This is your public key and secret key that will be used to encrypt your diary:',
    },
    downloadPublicKey: {
      ko: '공개키 파일 다운로드',
      en: 'Download the public key as file',
    },
    copyPublicKey: {
      ko: '공개키 복사하기',
      en: 'Copy the public key to clipboard',
    },
    downloadPrivateKey: {
      ko: '비밀키 파일 다운로드',
      en: 'Download the secret key as file',
    },
    copyPrivateKey: {
      ko: '비밀키 복사하기',
      en: 'Copy the secret key to clipboard',
    },
    notice: {
      ko: '공개키와 비밀키를 잃어버리지 마세요. 키 파일을 다운로드받아 안전한 곳에 두거나, 키를 복사해 다른 곳에 보관할 것을 강력히 권장합니다. 또한, 절대로 비밀키를 다른 사람에게 알려주지 마세요.',
      en: 'Don\'t lose your public key and secret key. It is strongly recommended that you download the key files and store it in a secure place, or copy the keys to somewhere else. Also, NEVER let anyone know your secret key.',
    },
    goToSignIn: {
      ko: '로그인하러 가기 ↗',
      en: 'Go to sign in ↗',
    },
  });

  const getDownloadURLOfTextFile = (text: string) => {
    const blob = new Blob([text], { type: 'text/plain' });
    return URL.createObjectURL(blob);
  };

  return <Container bottom={30}>
    <InfoSection bottom={20}>
      <Section>{i18n.text('info')}</Section>
      <KeySection top={20} row>
        <a download='darim-public-key.txt' href={getDownloadURLOfTextFile(publicKey)}>
          <Button>{i18n.text('downloadPublicKey')}</Button>
        </a>
        <CopyToClipboard text={publicKey}>
          <CopyButton>{i18n.text('copyPublicKey')}</CopyButton>
        </CopyToClipboard>
      </KeySection>
      <KeySection top={10} bottom={20} row>
        <a download='darim-secret-key.txt' href={getDownloadURLOfTextFile(privateKey)}>
          <Button>{i18n.text('downloadPrivateKey')}</Button>
        </a>
        <CopyToClipboard text={privateKey}>
          <CopyButton>{i18n.text('copyPrivateKey')}</CopyButton>
        </CopyToClipboard>
      </KeySection>
      {i18n.text('notice')}
    </InfoSection>
    <Link to='/'>
      <GoToSignInButton>
        {i18n.text('goToSignIn')}
      </GoToSignInButton>
    </Link>
  </Container>;
};

export default Token;
