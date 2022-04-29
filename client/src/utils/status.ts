import { I18n } from './i18n';

enum SaveStatus {
  NONE,
  FAILURE,
  SUCCESS,
  ONGOING,
}

const i18n = new I18n({
  saveStatusOngoing: {
    ko: '저장 중...',
    en: 'Saving...',
  },
  saveStatusSuccess: {
    ko: '✅ 저장되었습니다!',
    en: '✅ Saved!',
  },
  saveStatusFailure: {
    ko: '❌ 저장에 실패했습니다',
    en: '❌ Failed to save',
  },
});

const getSaveStatusText = (status: SaveStatus) => {
  switch (status) {
    case SaveStatus.FAILURE:
      return i18n.text('saveStatusFailure');
    case SaveStatus.SUCCESS:
      return i18n.text('saveStatusSuccess');
    case SaveStatus.ONGOING:
      return i18n.text('saveStatusOngoing');
    default:
      return '';
  }
};

export { SaveStatus, getSaveStatusText };
