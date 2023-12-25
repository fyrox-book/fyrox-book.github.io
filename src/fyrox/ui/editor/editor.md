# UI Editor

![UI Editor](ui_editor.png)

User interface (UI) editor is used to create and edit UI scenes. Unlike many other game engines, UI scenes are 
completely different kind of scenes - their content **cannot** be mixed with standard game scenes. This is
made on purpose - such separation enables a variety of optimizations and greatly improves flexibility.

## Appearance

UI editor looks pretty much the same as the editor for game scenes, with the only difference that it is 
2D only. On the left side it contains the world viewer, which is used to manipulate the tree of widgets.
On the right side it has the inspector, which is used to edit properties of widgets. It also has just a 
few tools in the toolbar - selection and move tool. 

## Introduction

To start making a UI scene all you need to do is to create a new UI scene. This could be done from 
`File -> New UI Scene` menu. After this you'll see an example scene with a button and a text. You can either
delete these widgets, or use them as you want. You can create any kinds of widgets in the UI scene, even 
custom-made ones. All you need to do is to click `Create -> UI` and select a desired widget, or you can also
right-click on a widget in the world viewer and create a widget from the context menu. The widget created
from the context menu will become a child widget of the widget from which you've opened the context menu.

Widgets can form an arbitrary hierarchy, which could be changed by dragging a widget and dropping it on some
other widget in the world viewer. Keep in mind, that some widgets may contain handles to other widgets, and 
you need to set them too. For example, the Button widget contains a handle of its content which is used to
delete a current content when changing it to some other. If you'll leave button's content as unassigned handle,
then your button may behave weirdly. Some of the widgets (like layout panels) works directly with their 
children widgets and do not have "external" handles.

## Widgets 

See a respective chapter for each widget to learn how it can be created and tweaked. Keep in mind, that UI 
editor is a new feature of the engine and the book's chapters could lack some information about the editor. 

## Video 

The following video shows how to create a simple main menu for a 2D platformer.

<iframe width="560" height="315" src="https://www.youtube.com/embed/qQTxEK5TTxM" title="YouTube video player" frameborder="0" allow="accelerometer; autoplay; clipboard-write; encrypted-media; gyroscope; picture-in-picture" allowfullscreen></iframe>