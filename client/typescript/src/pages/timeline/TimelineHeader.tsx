import React from 'react';
import { Link } from "react-router-dom";
import styled from 'styled-components';

import { Button, Section } from '../../components';
import { ViewMode } from "./Timeline";

interface Props {
  viewModeState: [ViewMode, React.Dispatch<React.SetStateAction<ViewMode>>]
}

const Container = styled(Section)`
  margin-bottom: 40px;
  justify-content: space-between;
`;

const Select = styled.select`
  width: 120px;
`;

const TimelineHeader: React.FC<Props> = ({ viewModeState }) => {
  const [viewMode, setViewMode] = viewModeState;

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

  return <Container row>
    <Link to={'/post'}>
      <Button>New +</Button>
    </Link>
    <Select value={viewMode} onChange={({ target: { value }}) => setViewMode(convertStringToViewMode(value))}>
      <option value={ViewMode.CALENDAR}>Calendar view</option>
      <option value={ViewMode.LIST}>List view</option>
    </Select>
  </Container>
};

export default TimelineHeader;
