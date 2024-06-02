# Tab Control

The Tab Control handles the visibility of several tabs, only showing a single tab that the user has selected via the 
tab header buttons. Each tab is defined via a Tab Definition struct which takes two widgets, one representing the tab
header and the other representing the tab's contents.

The following example makes a 2 tab, Tab Control containing some simple text widgets:

```rust,no_run
{{#include ../code/snippets/src/ui/tab.rs:create_tab_control}}
```

As usual, we create the widget via the builder TabControlBuilder. Tabs are added via the *with_tab* function in the 
order you want them to appear, passing each call to the function a directly constructed TabDefinition struct. Tab 
headers will appear from left to right at the top with tab contents shown directly below the tabs. As usual, if no 
constraints are given to the base WidgetBuilder of the TabControlBuilder, then the tab content area will resize to fit
whatever is in the current tab.

Each tab's content is made up of one widget, so to be useful you will want to use one of the container widgets to help 
arrange additional widgets within the tab.

## Tab Header Styling

Notice that you can put any widget into the tab header, so if you want images to denote each tab you can add an Image
widget to each header, and if you want an image *and* some text you can insert a stack panel with an image on top and 
text below it. 

You will also likely want to style whatever widgets you add. As can be seen when running the code example above, the 
tab headers are scrunched when there are no margins provided to your text widgets. Simply add something like the below 
code example and you will get a decent look:

```rust,no_run
{{#include ../code/snippets/src/ui/tab.rs:create_tab_control_with_header}}
```