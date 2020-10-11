import React from 'react';
import styled from 'styled-components';

import { getI18n } from '../../utils/i18n';
import { Container, Section } from '../../components';
import LoginForm from './LoginForm'
import JoinForm from './JoinForm'
import { Session } from '../../models';

import encryptionImage from './images/encryption.svg';
import markdownImage from './images/markdown.svg';

interface Props {
  session_state: [Session | null, React.Dispatch<React.SetStateAction<Session | null>>]
}

const SectionTitle = styled.h2`
  font-size: 22px;
  font-weight: bold;
  color: #303030;
`;

const SectionContent = styled.p`
  margin-top: 10px;
  line-height: 180%;
`;

const Image = styled.img`
  width: 100%;
`;

const Landing: React.FC<Props> = ({ session_state }) => {
  const i18n = getI18n({
    secureDiary: {
      ko: '📖 안전한 온라인 다이어리',
      en: '📖 Secure online diary',
    },
    secureDescription: {
      ko: '다림은 비밀키 암호화를 사용하는 안전한 온라인 다이어리 서비스입니다.',
      en: 'Darim is a secure online diary service using secret key encryption.',
    },
    keepYourDiaryASecret: {
      ko: '🔏 다이어리의 내용을 안전하게 보호하세요',
      en: '🔏 Keep your diary secure'
    },
    keepYourDiaryASecretDescription: {
      ko: `암호화를 통해 다이어리 내용을 보호할 수 있습니다.
        작성한 다이어리 내용은 저장할 때 비밀키를 이용해 암호화되기 때문에 작성자 외 누구도 그 내용을 읽을 수 없습니다.
        심지어 개발자도 알아낼 수 없습니다.`,
      en: `You can keep your diary a secret via encryption.
         No one but you can read your diary because a diary is encrypted by secret key.
         Even the developer can't figure out it`,
    },
    youCanUseVariousFormatsUsingMarkdown: {
      ko: '🎁 다른 서비스에서 쉽게 다이어리를 가져오고, 내보낼 수 있어요',
      en: '🎁 You can easily import and export a diary from other services',
    },
    youCanUseVariousFormatsUsingMarkdownDescription: {
      ko: `범용적인 마크다운 문법을 사용해 노션(Notion), 베어(Bear) 등 다른 서비스에서 다이어리를 가져오거나,
        내보낼 수 있습니다. 또한 코드 하이라이팅, 테이블, 각주, KaTeX 등 다양한 형식을 사용할 수 있습니다.`,
      en: `You can import and export a diary from other services such as Notion and Bear
        using general markdown syntax. Also, you can use code highlighting, table, footnotes, KaTeX,
        and more.`,
    },
    getStartedNow: {
      ko: '👋 지금 시작해보세요!',
      en: '👋 Get started now!',
    }
  });

  return <Container>
    <Section>
      <SectionTitle>{i18n.text('secureDiary')}</SectionTitle>
      <SectionContent>{i18n.text('secureDescription')}</SectionContent>
    </Section>
    <Section top={30}>
      <JoinForm />
    </Section>
    <Section top={30}>
      <Section>
        <Image src='https://user-images.githubusercontent.com/6410412/87238882-579d4900-c443-11ea-8e81-267b3243237c.png' />
      </Section>
      <Section top={30}>
        <LoginForm sessionState={session_state} />
      </Section>
      <Section top={50}>
        <SectionTitle>{i18n.text('keepYourDiaryASecret')}</SectionTitle>
        <Section top={30} bottom={20}>
          <Image src={encryptionImage} />
        </Section>
        <SectionContent>{i18n.text('keepYourDiaryASecretDescription')}</SectionContent>
      </Section>
      <Section top={50}>
        <SectionTitle>{i18n.text('youCanUseVariousFormatsUsingMarkdown')}</SectionTitle>
        <Section top={10} bottom={10}>
          <Image src={markdownImage} />
        </Section>
        <SectionContent>{i18n.text('youCanUseVariousFormatsUsingMarkdownDescription')}</SectionContent>
      </Section>
      <Section top={50}>
        <SectionTitle>{i18n.text('getStartedNow')}</SectionTitle>
        <Section top={20}>
          <JoinForm />
        </Section>
      </Section>
    </Section>
  </Container>
};

export default Landing;
