import React from 'react';
import { Link } from 'react-router-dom';
import styled from 'styled-components';

import { Button, Container } from '../../components';
import { ViewMode } from './Timeline';
import I18n from '../../utils/i18n';

interface Props {
  viewModeState: [ViewMode, React.Dispatch<React.SetStateAction<ViewMode>>]
}

const MainContainer = styled(Container)`
  justify-content: space-between;
`;

const Select = styled.select`
  width: 120px;
`;

const TimelineHeader: React.FC<Props> = ({ viewModeState }) => {
  const [viewMode, setViewMode] = viewModeState;

  const i18n = new I18n({
    listView: {
      ko: '리스트 뷰',
      en: 'List view',
    },
    calendarView: {
      ko: '캘린더 뷰',
      en: 'Calendar view',
    }
  });

  const convertStringToViewMode = (mode: string) => {
    switch (mode) {
      case '0':
        return ViewMode.CALENDAR;
      case '1':
        return ViewMode.LIST;
      default:
        return ViewMode.CALENDAR;
    }
  };

  return <MainContainer row>
    <Link to={'/post'}>
      <Button>New +</Button>
    </Link>
    <Select value={viewMode} onChange={({ target: { value }}) => setViewMode(convertStringToViewMode(value))}>
      <option value={ViewMode.CALENDAR}>{i18n.text('calendarView')}</option>
      <option value={ViewMode.LIST}>{i18n.text('listView')}</option>
    </Select>
  </MainContainer>
};

export default TimelineHeader;
