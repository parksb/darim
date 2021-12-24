import React from 'react';
import { Link } from 'react-router-dom';
import styled from 'styled-components';

import Storage from '../../utils/storage';
import { getI18n } from '../../utils/i18n';
import { Button, Container } from '../../components';
import { localStorageViewModeKey } from '../../constants';
import { ViewMode, ViewModeMethods } from '../../models';

interface Props {
  viewModeState: [ViewMode, React.Dispatch<React.SetStateAction<ViewMode>>]
}

const MainContainer = styled(Container)`
  justify-content: space-between;
  padding: 0 20px 0 20px;
`;

const Select = styled.select`
  width: 120px;
`;

const TimelineHeader: React.FC<Props> = ({ viewModeState }) => {
  const [viewMode, setViewMode] = viewModeState;

  const i18n = getI18n({
    listView: {
      ko: '리스트 뷰',
      en: 'List view',
    },
    calendarView: {
      ko: '캘린더 뷰',
      en: 'Calendar view',
    }
  });

  const switchViewMode = (mode: string) => {
    const viewMode = ViewModeMethods.convertNumberToString(Number(mode));
    Storage.set(localStorageViewModeKey, ViewModeMethods.convertViewModeToString(viewMode));
    setViewMode(viewMode);
  };

  return <MainContainer row>
    <Link to={'/post'}>
      <Button>New +</Button>
    </Link>
    <Select value={viewMode} onChange={({ target: { value }}) => switchViewMode(value)}>
      <option value={ViewMode.CALENDAR}>{i18n.text('calendarView')}</option>
      <option value={ViewMode.LIST}>{i18n.text('listView')}</option>
    </Select>
  </MainContainer>
};

export default TimelineHeader;
