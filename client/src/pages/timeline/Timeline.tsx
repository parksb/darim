import React, { useState } from 'react';
import { useParams } from 'react-router-dom';

import Calendar from './Calendar';
import List from './List';
import TimelineHeader from './TimelineHeader';
import { Session, ViewMode, ViewModeMethods } from '../../models';
import Storage from '../../utils/storage';
import { Container } from '../../components'
import { localStorageViewModeKey } from '../../constants';

interface Props {
  session: Session;
}

const Timeline: React.FC<Props> = ({ session }) => {
  const getInitialViewMode = () => {
    const { viewMode } = useParams();
    return viewMode ? ViewModeMethods.convertStringToViewMode(viewMode) : ViewModeMethods.convertStringToViewMode(Storage.get(localStorageViewModeKey));
  }

  const [viewMode, setViewMode] = useState(getInitialViewMode());

  const viewTimelineBody = (mode: ViewMode) => {
    switch (mode) {
      case ViewMode.CALENDAR:
        return <Calendar session={session} />;
      case ViewMode.LIST:
        return <List session={session} />;
    }
  };

  return <Container fullWidth fullHeight>
    <TimelineHeader viewModeState={[viewMode, setViewMode]} />
    {viewTimelineBody(viewMode)}
  </Container>
};

export default Timeline;
