import React, { useState } from 'react';
import styled from 'styled-components';

import * as api from '../../api/user';
import { Button, Section, TextField } from '../../components';
import I18n from '../../utils/i18n';
import { SaveStatus, getSaveStatusText } from '../../utils/status';

interface Props {
  userId: string;
}

const SectionTitle = styled.h2`
  font-size: 24px;
  font-weight: 700;
  margin-bottom: 15px;
`;

const FullWidthTextField = styled(TextField)`
  flex: 1;
  height: 21px;
`;

const SaveStatusText = styled.span`
  margin-top: 5px;
  font-size: 12px;
  color: #c0c0c0;
`;

const ProfileSettings: React.FC<Props> = ({ userId }) => {
  const [newName, setNewName] = useState('');
  const [newAvatar, setNewAvatar] = useState('');

  const [newNameSaveStatus, setNewNameSaveStatus] = useState(SaveStatus.NONE);
  const [newAvatarSaveStatus, setNewAvatarSaveStatus] = useState(SaveStatus.NONE);

  const i18n = new I18n({
    newName: {
      ko: '새 이름',
      en: 'New name',
    },
    newAvatar: {
      ko: '새 프로필 사진',
      en: 'New avatar',
    },
  });

  const saveNewName = async () => {
    setNewNameSaveStatus(SaveStatus.ONGOING);
    const result = await api.updateUser(userId, undefined, newName);

    setNewName('');

    if (result) {
      setNewNameSaveStatus(SaveStatus.SUCCESS);
    } else {
      setNewNameSaveStatus(SaveStatus.FAILURE);
    }
  };

  const saveNewAvatar = async () => {
    setNewAvatarSaveStatus(SaveStatus.ONGOING);
    const result = await api.updateUser(userId, undefined, undefined, newAvatar);

    setNewAvatar('');

    if (result) {
      setNewAvatarSaveStatus(SaveStatus.SUCCESS);
    } else {
      setNewAvatarSaveStatus(SaveStatus.FAILURE);
    }
  };

  return <Section>
    <Section bottom={30}>
      <SectionTitle>{i18n.text('name')}</SectionTitle>
      <Section row>
        <FullWidthTextField
          type='text'
          placeholder={i18n.text('newName')}
          value={newName}
          onChange={({ target: { value } }) => setNewName(value)}
        />
        <Button onClick={() => saveNewName()}>{i18n.text('save')}</Button>
      </Section>
      <SaveStatusText>{getSaveStatusText(newNameSaveStatus)}</SaveStatusText>
    </Section>
    <Section bottom={30}>
      <SectionTitle>{i18n.text('avatar')}</SectionTitle>
      <Section row>
        <FullWidthTextField
          type='text'
          placeholder={i18n.text('newAvatar')}
          value={newAvatar}
          onChange={({ target: { value } }) => setNewAvatar(value)}
        />
        <Button onClick={() => saveNewAvatar()}>{i18n.text('save')}</Button>
      </Section>
      <SaveStatusText>{getSaveStatusText(newAvatarSaveStatus)}</SaveStatusText>
    </Section>
  </Section>;
};

export default ProfileSettings;
