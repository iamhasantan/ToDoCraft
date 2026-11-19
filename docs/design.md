# ToDoCraft design contract

Inspired by the public design-system principles of the [Storytold Craft suite](https://github.com/storytold/craft-libs/issues/617), without using its trademarks or proprietary assets.

## Engine first

- `todocraft-core` owns tasks, lists, recurrence, filters and invariants; it knows nothing about windows or storage.
- `todocraft-storage` owns SQLite schema and transactional persistence; it never decides visual appearance.
- `todocraft` owns UI state, layout, shortcuts, command routing and semantic themes.

## UI principles

- **Meaningful styles**: named roles such as accent, sidebar surface, muted text and important, rather than spreading literal colors.
- **No styling in business logic**: a theme must not change task order or recurrence.
- **Clear information hierarchy**: left navigation, central tasks, optional inspector, persistent quick-add.
- **Native controls**: egui widgets provide keyboard focus and label semantics where supported.
- **Undo before destructive operations**: initial delete uses explicit confirmation; an actual trash/restore model remains open work.
- **No artificial equivalence**: clean-room Microsoft To Do parity is judged by user workflows, not matching proprietary pixels.
- **Portable native app**: the same Rust components run on desktop platforms without a webview.

## Reusing Craft libraries

Before building reusable components, check `storytold/craft-libs`, particularly `craft-ui`, `craft-settings` and `craft-dialogs`. This first change uses egui's built-in controls to keep initial dependencies small; adopting compatible shared components is a separate tracked follow-up.

## Asset policy

All glyphs in this initial UI are ordinary Unicode symbols painted by system/default egui fonts. No Microsoft or Storytold icons, brand marks, custom fonts or media files are copied.

## Microsoft 365-inspired visual direction

The visual language borrows broad **Fluent 2** principles, not Microsoft product identity:

- **Focused, with a little flair**: a calm neutral workspace, soft rounded task surfaces and just one small tinted My Day header.
- **Colour discipline**: original ToDoCraft blue used for selection, navigation and the primary add action. Important tasks use an amber semantic indicator, never decorative yellow.
- **Light first, dark supported**: contrast-aware semantic tokens and a clear mode switch, no branded images or Microsoft marks.
- **Typography**: 12/14/17/25 point hierarchy in a native proportional font, rather than copying or bundling Segoe fonts.
- **Rhythm**: Fluent-inspired 4-point spacing, readable 36 to 38 point navigation controls, minimal separators and generous white space.
- **No ribbon or toolbar sprawl**: the main work is still task completion; controls stay in the sidebar, task rows, bottom quick-add and optional details pane.
- **Original presentation**: independent app name, Unicode symbols and egui-drawn controls. Do not copy any Fluent icon assets, branded artwork or Microsoft To Do screenshots.

Public design references: [Fluent colour](https://fluent2.microsoft.design/color), [typography](https://fluent2.microsoft.design/typography) and [layout](https://fluent2.microsoft.design/layout).

### Visual acceptance

- Both light and dark mode must be tested with a selected list, populated tasks, completed tasks, important tasks, an open detail inspector and empty states.
- The theme switch must not alter the domain model or persisted data.
- Small viewports, keyboard focus, high-DPI scaling and contrast need real GUI inspection before visual parity is claimed.
