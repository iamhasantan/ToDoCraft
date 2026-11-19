# Microsoft To Do feature-parity checklist

Reference: public Microsoft To Do support documentation (November 2026). A UI affordance or stored field is not full feature parity. Statuses: **local** = usable offline, **partial** = incomplete behaviour, **missing** = not implemented.

| Feature | Status | Evidence / limitation |
|---------|--------|-----------------------|
| Create and complete tasks | local | Engine commands and native UI |
| Edit/delete tasks | local | Native inspector |
| Custom lists | local | Create and select lists, delete list missing |
| My Day | local | Current-day smart filter; resets by date |
| My Day suggestions | missing | No suggestion ranking yet |
| Important | local | Mark important and smart list |
| Planned | local | Due-date smart list |
| All / Completed | local | Smart filters |
| Search tasks | local | Case-insensitive title and note match |
| Task details/notes | local | Editable text |
| Steps / subtasks | local | Create, complete and delete |
| Due dates | local | ISO date input, list filtering |
| Reminders | partial | Timestamp metadata; no OS alerts or snooze |
| Repeating tasks | partial | Daily, weekdays, weekly, monthly, yearly successor on completion; custom intervals missing |
| Organise tasks in lists | local | Move between lists |
| Sort/reorder tasks | missing | Standard fixed ordering only |
| List groups | missing | |
| List sharing / invite links | missing | Requires backend design |
| Task assignment | missing | Requires users, permissions and sync |
| Attachments | missing | |
| Tags / categories | missing | |
| Deleted-task restore | missing | |
| Backgrounds / custom themes | missing | Dark semantic theme only |
| Cross-device sync | missing | Local SQLite only |
| Outlook / Planner / flagged email | missing | Requires Microsoft integration |
| Microsoft account sign-in | missing | |
| Push notifications / live reminders | missing | |
| Accessibility, localization | partial | egui controls, English only |
| Mobile, web builds | missing | Native desktop only |
| Offline persistence | local | SQLite transactionally saves |
| Import/export / backup | missing | |
| Global shortcuts | missing | |
| Complete verified Microsoft To Do parity | missing | No 1:1 claim |

Sources:
- https://support.microsoft.com/en-us/todo/managing-tasks-in-microsoft-to-do
- https://support.microsoft.com/en-us/todo/add-due-dates-and-reminders-in-microsoft-to-do
- https://support.microsoft.com/en-us/todo/my-day-and-suggestions
- https://support.microsoft.com/en-us/todo/assign-tasks-in-shared-lists

Every row needs a user-facing acceptance test before being marked fully equivalent.
