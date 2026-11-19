# Known gaps and delivery priorities

1. **Cloud identity and sync**. Design a pluggable sync interface and review Microsoft Graph To Do permissions, delta behaviour, conflict resolution, pagination, rate limits and consent. Avoid pretending local tasks are already synchronized.
2. **Reminders**. Native scheduled notifications, rescheduling, time zone and DST behaviour, snooze, missed alerts on relaunch.
3. **List collaboration**. Shared lists, invitations, members, assignments, permissions and security.
4. **Task fidelity**. Attachments, tags/categories, deleted-task restore, sorting, drag reorder, list groups, custom recurrence, import and export.
5. **My Day suggestions**. Match daily reset and suggestion rules using observable behaviour, not proprietary algorithms.
6. **Quality**. UI automation, screen-reader labels, keyboard navigation, localisation, migration tests, backup/restore, performance tests.
7. **Multi-platform**. Windows/macOS/Linux validation, packaging, update signing; web/mobile are separate later decisions.

## Release gates

- Pre-alpha: core task flow usable, local persistence tested.
- Alpha: lossless backup/restore, reminders, stability and UI smoke tests.
- Beta: comprehensive task/list fidelity, real sync and documented conflicts.
- 1:1 parity: side-by-side behavioural acceptance suite passing across supported platforms, with honest exceptions for Microsoft-only services.

No parity percentage is claimed without evidence.
