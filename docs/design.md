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
