// Populate the sidebar
//
// This is a script, and not included directly in the page, to control the total size of the book.
// The TOC contains an entry for each page, so if each page includes a copy of the TOC,
// the total size of the page becomes O(n**2).
class MDBookSidebarScrollbox extends HTMLElement {
    constructor() {
        super();
    }
    connectedCallback() {
        this.innerHTML = '<ol class="chapter"><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="introduction.html"><strong aria-hidden="true">1.</strong> About the Book</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="introduction/index.html"><strong aria-hidden="true">2.</strong> Introduction</a><a class="chapter-fold-toggle"><div>❱</div></a></span><ol class="section"><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="introduction/introduction.html"><strong aria-hidden="true">2.1.</strong> Introduction to Fyrox</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="introduction/requirements.html"><strong aria-hidden="true">2.2.</strong> System Requirements and Supported Platforms</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="introduction/basic_concepts.html"><strong aria-hidden="true">2.3.</strong> Basic concepts</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="introduction/philosophy_and_goals.html"><strong aria-hidden="true">2.4.</strong> Design Philosophy and Goals</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="introduction/faq.html"><strong aria-hidden="true">2.5.</strong> Frequently Asked Questions</a></span></li></ol><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="beginning/getting_started.html"><strong aria-hidden="true">3.</strong> Getting started</a><a class="chapter-fold-toggle"><div>❱</div></a></span><ol class="section"><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="beginning/scripting.html"><strong aria-hidden="true">3.1.</strong> Installation and Project Creation</a><a class="chapter-fold-toggle"><div>❱</div></a></span><ol class="section"><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="beginning/manual_installation.html"><strong aria-hidden="true">3.1.1.</strong> Manual Installation</a></span></li></ol><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="beginning/editor_overview.html"><strong aria-hidden="true">3.2.</strong> Editor Overview</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="beginning/scene_and_scene_graph.html"><strong aria-hidden="true">3.3.</strong> Scene and Scene Graph</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="beginning/assets.html"><strong aria-hidden="true">3.4.</strong> Assets</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="beginning/data_management.html"><strong aria-hidden="true">3.5.</strong> Data Management</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="beginning/borrow_checker.html"><strong aria-hidden="true">3.6.</strong> Borrow Checker</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="beginning/project_manager.html"><strong aria-hidden="true">3.7.</strong> Project Manager</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="beginning/hot_reloading.html"><strong aria-hidden="true">3.8.</strong> Hot Reloading</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="beginning/ide.html"><strong aria-hidden="true">3.9.</strong> IDE for code</a></span></li></ol><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="scripting/scripting.html"><strong aria-hidden="true">4.</strong> Game Logic</a><a class="chapter-fold-toggle"><div>❱</div></a></span><ol class="section"><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="scripting/plugin.html"><strong aria-hidden="true">4.1.</strong> Plugins</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="scripting/script.html"><strong aria-hidden="true">4.2.</strong> Scripts</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="scripting/tasks.html"><strong aria-hidden="true">4.3.</strong> Tasks</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="scripting/error.html"><strong aria-hidden="true">4.4.</strong> Error Handling</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="scripting/executor.html"><strong aria-hidden="true">4.5.</strong> Executor</a></span></li></ol><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="scene/scene.html"><strong aria-hidden="true">5.</strong> Scene</a><a class="chapter-fold-toggle"><div>❱</div></a></span><ol class="section"><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="scene/graph.html"><strong aria-hidden="true">5.1.</strong> Graph</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="scene/transform.html"><strong aria-hidden="true">5.2.</strong> Transformation</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="scene/prefab.html"><strong aria-hidden="true">5.3.</strong> Prefabs</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="scene/inheritance.html"><strong aria-hidden="true">5.4.</strong> Property Inheritance</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="scene/base_node.html"><strong aria-hidden="true">5.5.</strong> Base Node</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="scene/mesh_node.html"><strong aria-hidden="true">5.6.</strong> Mesh Node</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="scene/lighting.html"><strong aria-hidden="true">5.7.</strong> Lighting</a><a class="chapter-fold-toggle"><div>❱</div></a></span><ol class="section"><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="scene/light_node.html"><strong aria-hidden="true">5.7.1.</strong> Light Node</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="scene/probe.html"><strong aria-hidden="true">5.7.2.</strong> Reflection Probe</a></span></li></ol><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="scene/sprite_node.html"><strong aria-hidden="true">5.8.</strong> Sprite Node</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="scene/particle_system_node.html"><strong aria-hidden="true">5.9.</strong> Particle System Node</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="scene/terrain_node.html"><strong aria-hidden="true">5.10.</strong> Terrain Node</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="scene/camera_node.html"><strong aria-hidden="true">5.11.</strong> Camera Node</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="scene/decal_node.html"><strong aria-hidden="true">5.12.</strong> Decal Node</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="scene/rectangle.html"><strong aria-hidden="true">5.13.</strong> Rectangle Node</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="scene/tilemap.html"><strong aria-hidden="true">5.14.</strong> Tile Map</a><a class="chapter-fold-toggle"><div>❱</div></a></span><ol class="section"><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="scene/autotiling.html"><strong aria-hidden="true">5.14.1.</strong> Autotiling Macros</a></span></li></ol><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="scene/custom_node.html"><strong aria-hidden="true">5.15.</strong> Custom Node</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="physics/physics.html"><strong aria-hidden="true">5.16.</strong> Physics</a><a class="chapter-fold-toggle"><div>❱</div></a></span><ol class="section"><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="physics/rigid_body.html"><strong aria-hidden="true">5.16.1.</strong> Rigid Body</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="physics/collider.html"><strong aria-hidden="true">5.16.2.</strong> Collider</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="physics/joint.html"><strong aria-hidden="true">5.16.3.</strong> Joint</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="physics/ray.html"><strong aria-hidden="true">5.16.4.</strong> Ray Casting</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="physics/ragdoll.html"><strong aria-hidden="true">5.16.5.</strong> Ragdoll</a></span></li></ol><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="sound/index.html"><strong aria-hidden="true">5.17.</strong> Sound</a><a class="chapter-fold-toggle"><div>❱</div></a></span><ol class="section"><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="sound/bus.html"><strong aria-hidden="true">5.17.1.</strong> Audio Bus</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="sound/sound.html"><strong aria-hidden="true">5.17.2.</strong> Sound Node</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="sound/listener.html"><strong aria-hidden="true">5.17.3.</strong> Listener</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="sound/hrtf.html"><strong aria-hidden="true">5.17.4.</strong> HRTF</a></span></li></ol><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="animation/animation.html"><strong aria-hidden="true">5.18.</strong> Animation</a><a class="chapter-fold-toggle"><div>❱</div></a></span><ol class="section"><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="animation/anim_editor.html"><strong aria-hidden="true">5.18.1.</strong> Animation Editor</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="animation/blending.html"><strong aria-hidden="true">5.18.2.</strong> Animation Blending</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="animation/absm_editor.html"><strong aria-hidden="true">5.18.3.</strong> ABSM Editor</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="animation/signal.html"><strong aria-hidden="true">5.18.4.</strong> Signals</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="animation/root_motion/root_motion.html"><strong aria-hidden="true">5.18.5.</strong> Root Motion</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="animation/spritesheet/spritesheet.html"><strong aria-hidden="true">5.18.6.</strong> Sprite Animation</a></span></li></ol><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="scene/debug.html"><strong aria-hidden="true">5.19.</strong> Debugging</a></span></li></ol><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="input/input.html"><strong aria-hidden="true">6.</strong> Input Handling</a><a class="chapter-fold-toggle"><div>❱</div></a></span><ol class="section"><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="input/keyboard.html"><strong aria-hidden="true">6.1.</strong> Keyboard</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="input/mouse.html"><strong aria-hidden="true">6.2.</strong> Mouse</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="input/text.html"><strong aria-hidden="true">6.3.</strong> Raw Text Input</a></span></li></ol><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="ai/ai.html"><strong aria-hidden="true">7.</strong> Artificial Intelligence (WIP)</a><a class="chapter-fold-toggle"><div>❱</div></a></span><ol class="section"><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="ai/beh_tree.html"><strong aria-hidden="true">7.1.</strong> Behaviour Trees (WIP)</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="ai/pathfinding.html"><strong aria-hidden="true">7.2.</strong> Path Finding</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="ai/navmesh.html"><strong aria-hidden="true">7.3.</strong> Navigational Meshes</a></span></li></ol><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="rendering/rendering.html"><strong aria-hidden="true">8.</strong> Rendering</a><a class="chapter-fold-toggle"><div>❱</div></a></span><ol class="section"><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="rendering/shaders.html"><strong aria-hidden="true">8.1.</strong> Shaders</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="rendering/materials.html"><strong aria-hidden="true">8.2.</strong> Materials</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="rendering/lightmaps.html"><strong aria-hidden="true">8.3.</strong> Light Maps</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="rendering/settings.html"><strong aria-hidden="true">8.4.</strong> Settings</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="rendering/render_pass.html"><strong aria-hidden="true">8.5.</strong> Render Pass</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="rendering/normal_maps.html"><strong aria-hidden="true">8.6.</strong> Normal Maps</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="rendering/server.html"><strong aria-hidden="true">8.7.</strong> Graphics Server</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="rendering/debug.html"><strong aria-hidden="true">8.8.</strong> Debugging and Profiling</a></span></li></ol><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="resources/resources.html"><strong aria-hidden="true">9.</strong> Resource Management</a><a class="chapter-fold-toggle"><div>❱</div></a></span><ol class="section"><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="resources/model.html"><strong aria-hidden="true">9.1.</strong> Models</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="resources/texture.html"><strong aria-hidden="true">9.2.</strong> Textures</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="resources/sound.html"><strong aria-hidden="true">9.3.</strong> Sound Buffers</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="resources/custom.html"><strong aria-hidden="true">9.4.</strong> Custom Resource</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="resources/hot_reloading.html"><strong aria-hidden="true">9.5.</strong> Hot Reloading</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="resources/events.html"><strong aria-hidden="true">9.6.</strong> Events</a></span></li></ol><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="engine/engine.html"><strong aria-hidden="true">10.</strong> Engine</a><a class="chapter-fold-toggle"><div>❱</div></a></span><ol class="section"><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="engine/graphics_context.html"><strong aria-hidden="true">10.1.</strong> Graphics Context</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="engine/windows.html"><strong aria-hidden="true">10.2.</strong> Window Management</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="engine/manual_initialization.html"><strong aria-hidden="true">10.3.</strong> Manual Initialization</a></span></li></ol><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="net/net.html"><strong aria-hidden="true">11.</strong> Network</a><a class="chapter-fold-toggle"><div>❱</div></a></span><ol class="section"><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="net/basics.html"><strong aria-hidden="true">11.1.</strong> Basics</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="net/sync.html"><strong aria-hidden="true">11.2.</strong> Synchronization</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="net/hosting.html"><strong aria-hidden="true">11.3.</strong> Hosting</a></span></li></ol><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="ui/ui.html"><strong aria-hidden="true">12.</strong> User Interface</a><a class="chapter-fold-toggle"><div>❱</div></a></span><ol class="section"><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="ui/basic_concepts/basic_concepts.html"><strong aria-hidden="true">12.1.</strong> Basic concepts</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="ui/editor/editor.html"><strong aria-hidden="true">12.2.</strong> Editor</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="ui/rendering.html"><strong aria-hidden="true">12.3.</strong> Rendering</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="ui/font.html"><strong aria-hidden="true">12.4.</strong> Fonts</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="ui/style.html"><strong aria-hidden="true">12.5.</strong> Style</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="ui/widgets.html"><strong aria-hidden="true">12.6.</strong> Widgets</a><a class="chapter-fold-toggle"><div>❱</div></a></span><ol class="section"><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="ui/custom.html"><strong aria-hidden="true">12.6.1.</strong> Custom widget</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="ui/button.html"><strong aria-hidden="true">12.6.2.</strong> Button</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="ui/border.html"><strong aria-hidden="true">12.6.3.</strong> Border</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="ui/canvas.html"><strong aria-hidden="true">12.6.4.</strong> Canvas</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="ui/checkbox/check_box.html"><strong aria-hidden="true">12.6.5.</strong> Check box</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="ui/curve_editor.html"><strong aria-hidden="true">12.6.6.</strong> Curve editor (WIP)</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="ui/decorator.html"><strong aria-hidden="true">12.6.7.</strong> Decorator</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="ui/dock.html"><strong aria-hidden="true">12.6.8.</strong> Docking manager</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="ui/dropdown_list.html"><strong aria-hidden="true">12.6.9.</strong> Dropdown list</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="ui/expander.html"><strong aria-hidden="true">12.6.10.</strong> Expander</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="ui/file_browser.html"><strong aria-hidden="true">12.6.11.</strong> File browser (WIP)</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="ui/grid.html"><strong aria-hidden="true">12.6.12.</strong> Grid</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="ui/image.html"><strong aria-hidden="true">12.6.13.</strong> Image</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="ui/inspector.html"><strong aria-hidden="true">12.6.14.</strong> Inspector (WIP)</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="ui/list_view.html"><strong aria-hidden="true">12.6.15.</strong> List view</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="ui/menu.html"><strong aria-hidden="true">12.6.16.</strong> Menu (WIP)</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="ui/message_box.html"><strong aria-hidden="true">12.6.17.</strong> Message box (WIP)</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="ui/numeric.html"><strong aria-hidden="true">12.6.18.</strong> Numeric field</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="ui/popup.html"><strong aria-hidden="true">12.6.19.</strong> Popup</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="ui/progress_bar.html"><strong aria-hidden="true">12.6.20.</strong> Progress bar</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="ui/range.html"><strong aria-hidden="true">12.6.21.</strong> Range (WIP)</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="ui/rect.html"><strong aria-hidden="true">12.6.22.</strong> Rect (WIP)</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="ui/scroll_bar.html"><strong aria-hidden="true">12.6.23.</strong> Scroll bar</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="ui/scroll_panel.html"><strong aria-hidden="true">12.6.24.</strong> Scroll panel</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="ui/scroll_viewer.html"><strong aria-hidden="true">12.6.25.</strong> Scroll viewer</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="ui/screen.html"><strong aria-hidden="true">12.6.26.</strong> Screen</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="ui/stack_panel.html"><strong aria-hidden="true">12.6.27.</strong> Stack panel</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="ui/tab_control.html"><strong aria-hidden="true">12.6.28.</strong> Tab control</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="ui/text.html"><strong aria-hidden="true">12.6.29.</strong> Text</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="ui/text_box.html"><strong aria-hidden="true">12.6.30.</strong> Text box</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="ui/tree.html"><strong aria-hidden="true">12.6.31.</strong> Tree</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="ui/vector_image.html"><strong aria-hidden="true">12.6.32.</strong> Vector image</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="ui/window.html"><strong aria-hidden="true">12.6.33.</strong> Window</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="ui/wrap_panel.html"><strong aria-hidden="true">12.6.34.</strong> Wrap panel</a></span></li></ol></li></ol><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="serialization/index.html"><strong aria-hidden="true">13.</strong> Serialization</a><a class="chapter-fold-toggle"><div>❱</div></a></span><ol class="section"><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="serialization/save.html"><strong aria-hidden="true">13.1.</strong> Saved Games</a></span></li></ol><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="reflection/index.html"><strong aria-hidden="true">14.</strong> Reflection</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="editor/index.html"><strong aria-hidden="true">15.</strong> Editor</a><a class="chapter-fold-toggle"><div>❱</div></a></span><ol class="section"><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="editor/property_editors.html"><strong aria-hidden="true">15.1.</strong> Property Editors</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="editor/settings.html"><strong aria-hidden="true">15.2.</strong> Settings</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="editor/plugins.html"><strong aria-hidden="true">15.3.</strong> Plugins</a></span></li></ol><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="misc/misc.html"><strong aria-hidden="true">16.</strong> Miscellaneous</a><a class="chapter-fold-toggle"><div>❱</div></a></span><ol class="section"><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="misc/log.html"><strong aria-hidden="true">16.1.</strong> Log</a></span></li></ol><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="shipping/shipping.html"><strong aria-hidden="true">17.</strong> Shipping</a><a class="chapter-fold-toggle"><div>❱</div></a></span><ol class="section"><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="shipping/pc.html"><strong aria-hidden="true">17.1.</strong> PC</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="shipping/wasm.html"><strong aria-hidden="true">17.2.</strong> WebAssembly</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="shipping/android.html"><strong aria-hidden="true">17.3.</strong> Android</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="shipping/ci_cd.html"><strong aria-hidden="true">17.4.</strong> CI/CD Integration</a></span></li></ol><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="tutorials/tutorials.html"><strong aria-hidden="true">18.</strong> Tutorials</a><a class="chapter-fold-toggle"><div>❱</div></a></span><ol class="section"><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="tutorials/platformer/intro.html"><strong aria-hidden="true">18.1.</strong> 2D Platformer Tutorial</a><a class="chapter-fold-toggle"><div>❱</div></a></span><ol class="section"><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="tutorials/platformer/part1.html"><strong aria-hidden="true">18.1.1.</strong> Character Controller</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="tutorials/platformer/part2.html"><strong aria-hidden="true">18.1.2.</strong> Bots and AI</a></span></li></ol><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="tutorials/rpg/intro.html"><strong aria-hidden="true">18.2.</strong> RPG Tutorial</a><a class="chapter-fold-toggle"><div>❱</div></a></span><ol class="section"><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="tutorials/rpg/tutorial-1/tutorial-part-1.html"><strong aria-hidden="true">18.2.1.</strong> Character Controller</a></span></li></ol><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="tutorials/fps/fps-intro.html"><strong aria-hidden="true">18.3.</strong> FPS Tutorial</a><a class="chapter-fold-toggle"><div>❱</div></a></span><ol class="section"><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="tutorials/fps/tutorial-1/fps-tutorial.html"><strong aria-hidden="true">18.3.1.</strong> Character Controller</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="tutorials/fps/tutorial-2/fps-tutorial-2.html"><strong aria-hidden="true">18.3.2.</strong> Weapons</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="tutorials/fps/tutorial-3/fps-tutorial-3.html"><strong aria-hidden="true">18.3.3.</strong> Bots and AI</a></span></li></ol><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="tutorials/ui/ui-tutorial.html"><strong aria-hidden="true">18.4.</strong> User Interface Tutorial (WIP)</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="tutorials/community.html"><strong aria-hidden="true">18.5.</strong> Community Tutorials</a></span></li></ol><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="performance/index.html"><strong aria-hidden="true">19.</strong> Performance</a></span></li></ol>';
        // Set the current, active page, and reveal it if it's hidden
        let current_page = document.location.href.toString().split('#')[0].split('?')[0];
        if (current_page.endsWith('/')) {
            current_page += 'index.html';
        }
        const links = Array.prototype.slice.call(this.querySelectorAll('a'));
        const l = links.length;
        for (let i = 0; i < l; ++i) {
            const link = links[i];
            const href = link.getAttribute('href');
            if (href && !href.startsWith('#') && !/^(?:[a-z+]+:)?\/\//.test(href)) {
                link.href = path_to_root + href;
            }
            // The 'index' page is supposed to alias the first chapter in the book.
            if (link.href === current_page
                || i === 0
                && path_to_root === ''
                && current_page.endsWith('/index.html')) {
                link.classList.add('active');
                let parent = link.parentElement;
                while (parent) {
                    if (parent.tagName === 'LI' && parent.classList.contains('chapter-item')) {
                        parent.classList.add('expanded');
                    }
                    parent = parent.parentElement;
                }
            }
        }
        // Track and set sidebar scroll position
        this.addEventListener('click', e => {
            if (e.target.tagName === 'A') {
                const clientRect = e.target.getBoundingClientRect();
                const sidebarRect = this.getBoundingClientRect();
                sessionStorage.setItem('sidebar-scroll-offset', clientRect.top - sidebarRect.top);
            }
        }, { passive: true });
        const sidebarScrollOffset = sessionStorage.getItem('sidebar-scroll-offset');
        sessionStorage.removeItem('sidebar-scroll-offset');
        if (sidebarScrollOffset !== null) {
            // preserve sidebar scroll position when navigating via links within sidebar
            const activeSection = this.querySelector('.active');
            if (activeSection) {
                const clientRect = activeSection.getBoundingClientRect();
                const sidebarRect = this.getBoundingClientRect();
                const currentOffset = clientRect.top - sidebarRect.top;
                this.scrollTop += currentOffset - parseFloat(sidebarScrollOffset);
            }
        } else {
            // scroll sidebar to current active section when navigating via
            // 'next/previous chapter' buttons
            const activeSection = document.querySelector('#mdbook-sidebar .active');
            if (activeSection) {
                activeSection.scrollIntoView({ block: 'center' });
            }
        }
        // Toggle buttons
        const sidebarAnchorToggles = document.querySelectorAll('.chapter-fold-toggle');
        function toggleSection(ev) {
            ev.currentTarget.parentElement.parentElement.classList.toggle('expanded');
        }
        Array.from(sidebarAnchorToggles).forEach(el => {
            el.addEventListener('click', toggleSection);
        });
    }
}
window.customElements.define('mdbook-sidebar-scrollbox', MDBookSidebarScrollbox);


// ---------------------------------------------------------------------------
// Support for dynamically adding headers to the sidebar.

(function() {
    // This is used to detect which direction the page has scrolled since the
    // last scroll event.
    let lastKnownScrollPosition = 0;
    // This is the threshold in px from the top of the screen where it will
    // consider a header the "current" header when scrolling down.
    const defaultDownThreshold = 150;
    // Same as defaultDownThreshold, except when scrolling up.
    const defaultUpThreshold = 300;
    // The threshold is a virtual horizontal line on the screen where it
    // considers the "current" header to be above the line. The threshold is
    // modified dynamically to handle headers that are near the bottom of the
    // screen, and to slightly offset the behavior when scrolling up vs down.
    let threshold = defaultDownThreshold;
    // This is used to disable updates while scrolling. This is needed when
    // clicking the header in the sidebar, which triggers a scroll event. It
    // is somewhat finicky to detect when the scroll has finished, so this
    // uses a relatively dumb system of disabling scroll updates for a short
    // time after the click.
    let disableScroll = false;
    // Array of header elements on the page.
    let headers;
    // Array of li elements that are initially collapsed headers in the sidebar.
    // I'm not sure why eslint seems to have a false positive here.
    // eslint-disable-next-line prefer-const
    let headerToggles = [];
    // This is a debugging tool for the threshold which you can enable in the console.
    let thresholdDebug = false;

    // Updates the threshold based on the scroll position.
    function updateThreshold() {
        const scrollTop = window.pageYOffset || document.documentElement.scrollTop;
        const windowHeight = window.innerHeight;
        const documentHeight = document.documentElement.scrollHeight;

        // The number of pixels below the viewport, at most documentHeight.
        // This is used to push the threshold down to the bottom of the page
        // as the user scrolls towards the bottom.
        const pixelsBelow = Math.max(0, documentHeight - (scrollTop + windowHeight));
        // The number of pixels above the viewport, at least defaultDownThreshold.
        // Similar to pixelsBelow, this is used to push the threshold back towards
        // the top when reaching the top of the page.
        const pixelsAbove = Math.max(0, defaultDownThreshold - scrollTop);
        // How much the threshold should be offset once it gets close to the
        // bottom of the page.
        const bottomAdd = Math.max(0, windowHeight - pixelsBelow - defaultDownThreshold);
        let adjustedBottomAdd = bottomAdd;

        // Adjusts bottomAdd for a small document. The calculation above
        // assumes the document is at least twice the windowheight in size. If
        // it is less than that, then bottomAdd needs to be shrunk
        // proportional to the difference in size.
        if (documentHeight < windowHeight * 2) {
            const maxPixelsBelow = documentHeight - windowHeight;
            const t = 1 - pixelsBelow / Math.max(1, maxPixelsBelow);
            const clamp = Math.max(0, Math.min(1, t));
            adjustedBottomAdd *= clamp;
        }

        let scrollingDown = true;
        if (scrollTop < lastKnownScrollPosition) {
            scrollingDown = false;
        }

        if (scrollingDown) {
            // When scrolling down, move the threshold up towards the default
            // downwards threshold position. If near the bottom of the page,
            // adjustedBottomAdd will offset the threshold towards the bottom
            // of the page.
            const amountScrolledDown = scrollTop - lastKnownScrollPosition;
            const adjustedDefault = defaultDownThreshold + adjustedBottomAdd;
            threshold = Math.max(adjustedDefault, threshold - amountScrolledDown);
        } else {
            // When scrolling up, move the threshold down towards the default
            // upwards threshold position. If near the bottom of the page,
            // quickly transition the threshold back up where it normally
            // belongs.
            const amountScrolledUp = lastKnownScrollPosition - scrollTop;
            const adjustedDefault = defaultUpThreshold - pixelsAbove
                + Math.max(0, adjustedBottomAdd - defaultDownThreshold);
            threshold = Math.min(adjustedDefault, threshold + amountScrolledUp);
        }

        if (documentHeight <= windowHeight) {
            threshold = 0;
        }

        if (thresholdDebug) {
            const id = 'mdbook-threshold-debug-data';
            let data = document.getElementById(id);
            if (data === null) {
                data = document.createElement('div');
                data.id = id;
                data.style.cssText = `
                    position: fixed;
                    top: 50px;
                    right: 10px;
                    background-color: 0xeeeeee;
                    z-index: 9999;
                    pointer-events: none;
                `;
                document.body.appendChild(data);
            }
            data.innerHTML = `
                <table>
                  <tr><td>documentHeight</td><td>${documentHeight.toFixed(1)}</td></tr>
                  <tr><td>windowHeight</td><td>${windowHeight.toFixed(1)}</td></tr>
                  <tr><td>scrollTop</td><td>${scrollTop.toFixed(1)}</td></tr>
                  <tr><td>pixelsAbove</td><td>${pixelsAbove.toFixed(1)}</td></tr>
                  <tr><td>pixelsBelow</td><td>${pixelsBelow.toFixed(1)}</td></tr>
                  <tr><td>bottomAdd</td><td>${bottomAdd.toFixed(1)}</td></tr>
                  <tr><td>adjustedBottomAdd</td><td>${adjustedBottomAdd.toFixed(1)}</td></tr>
                  <tr><td>scrollingDown</td><td>${scrollingDown}</td></tr>
                  <tr><td>threshold</td><td>${threshold.toFixed(1)}</td></tr>
                </table>
            `;
            drawDebugLine();
        }

        lastKnownScrollPosition = scrollTop;
    }

    function drawDebugLine() {
        if (!document.body) {
            return;
        }
        const id = 'mdbook-threshold-debug-line';
        const existingLine = document.getElementById(id);
        if (existingLine) {
            existingLine.remove();
        }
        const line = document.createElement('div');
        line.id = id;
        line.style.cssText = `
            position: fixed;
            top: ${threshold}px;
            left: 0;
            width: 100vw;
            height: 2px;
            background-color: red;
            z-index: 9999;
            pointer-events: none;
        `;
        document.body.appendChild(line);
    }

    function mdbookEnableThresholdDebug() {
        thresholdDebug = true;
        updateThreshold();
        drawDebugLine();
    }

    window.mdbookEnableThresholdDebug = mdbookEnableThresholdDebug;

    // Updates which headers in the sidebar should be expanded. If the current
    // header is inside a collapsed group, then it, and all its parents should
    // be expanded.
    function updateHeaderExpanded(currentA) {
        // Add expanded to all header-item li ancestors.
        let current = currentA.parentElement;
        while (current) {
            if (current.tagName === 'LI' && current.classList.contains('header-item')) {
                current.classList.add('expanded');
            }
            current = current.parentElement;
        }
    }

    // Updates which header is marked as the "current" header in the sidebar.
    // This is done with a virtual Y threshold, where headers at or below
    // that line will be considered the current one.
    function updateCurrentHeader() {
        if (!headers || !headers.length) {
            return;
        }

        // Reset the classes, which will be rebuilt below.
        const els = document.getElementsByClassName('current-header');
        for (const el of els) {
            el.classList.remove('current-header');
        }
        for (const toggle of headerToggles) {
            toggle.classList.remove('expanded');
        }

        // Find the last header that is above the threshold.
        let lastHeader = null;
        for (const header of headers) {
            const rect = header.getBoundingClientRect();
            if (rect.top <= threshold) {
                lastHeader = header;
            } else {
                break;
            }
        }
        if (lastHeader === null) {
            lastHeader = headers[0];
            const rect = lastHeader.getBoundingClientRect();
            const windowHeight = window.innerHeight;
            if (rect.top >= windowHeight) {
                return;
            }
        }

        // Get the anchor in the summary.
        const href = '#' + lastHeader.id;
        const a = [...document.querySelectorAll('.header-in-summary')]
            .find(element => element.getAttribute('href') === href);
        if (!a) {
            return;
        }

        a.classList.add('current-header');

        updateHeaderExpanded(a);
    }

    // Updates which header is "current" based on the threshold line.
    function reloadCurrentHeader() {
        if (disableScroll) {
            return;
        }
        updateThreshold();
        updateCurrentHeader();
    }


    // When clicking on a header in the sidebar, this adjusts the threshold so
    // that it is located next to the header. This is so that header becomes
    // "current".
    function headerThresholdClick(event) {
        // See disableScroll description why this is done.
        disableScroll = true;
        setTimeout(() => {
            disableScroll = false;
        }, 100);
        // requestAnimationFrame is used to delay the update of the "current"
        // header until after the scroll is done, and the header is in the new
        // position.
        requestAnimationFrame(() => {
            requestAnimationFrame(() => {
                // Closest is needed because if it has child elements like <code>.
                const a = event.target.closest('a');
                const href = a.getAttribute('href');
                const targetId = href.substring(1);
                const targetElement = document.getElementById(targetId);
                if (targetElement) {
                    threshold = targetElement.getBoundingClientRect().bottom;
                    updateCurrentHeader();
                }
            });
        });
    }

    // Takes the nodes from the given head and copies them over to the
    // destination, along with some filtering.
    function filterHeader(source, dest) {
        const clone = source.cloneNode(true);
        clone.querySelectorAll('mark').forEach(mark => {
            mark.replaceWith(...mark.childNodes);
        });
        dest.append(...clone.childNodes);
    }

    // Scans page for headers and adds them to the sidebar.
    document.addEventListener('DOMContentLoaded', function() {
        const activeSection = document.querySelector('#mdbook-sidebar .active');
        if (activeSection === null) {
            return;
        }

        const main = document.getElementsByTagName('main')[0];
        headers = Array.from(main.querySelectorAll('h2, h3, h4, h5, h6'))
            .filter(h => h.id !== '' && h.children.length && h.children[0].tagName === 'A');

        if (headers.length === 0) {
            return;
        }

        // Build a tree of headers in the sidebar.

        const stack = [];

        const firstLevel = parseInt(headers[0].tagName.charAt(1));
        for (let i = 1; i < firstLevel; i++) {
            const ol = document.createElement('ol');
            ol.classList.add('section');
            if (stack.length > 0) {
                stack[stack.length - 1].ol.appendChild(ol);
            }
            stack.push({level: i + 1, ol: ol});
        }

        // The level where it will start folding deeply nested headers.
        const foldLevel = 3;

        for (let i = 0; i < headers.length; i++) {
            const header = headers[i];
            const level = parseInt(header.tagName.charAt(1));

            const currentLevel = stack[stack.length - 1].level;
            if (level > currentLevel) {
                // Begin nesting to this level.
                for (let nextLevel = currentLevel + 1; nextLevel <= level; nextLevel++) {
                    const ol = document.createElement('ol');
                    ol.classList.add('section');
                    const last = stack[stack.length - 1];
                    const lastChild = last.ol.lastChild;
                    // Handle the case where jumping more than one nesting
                    // level, which doesn't have a list item to place this new
                    // list inside of.
                    if (lastChild) {
                        lastChild.appendChild(ol);
                    } else {
                        last.ol.appendChild(ol);
                    }
                    stack.push({level: nextLevel, ol: ol});
                }
            } else if (level < currentLevel) {
                while (stack.length > 1 && stack[stack.length - 1].level > level) {
                    stack.pop();
                }
            }

            const li = document.createElement('li');
            li.classList.add('header-item');
            li.classList.add('expanded');
            if (level < foldLevel) {
                li.classList.add('expanded');
            }
            const span = document.createElement('span');
            span.classList.add('chapter-link-wrapper');
            const a = document.createElement('a');
            span.appendChild(a);
            a.href = '#' + header.id;
            a.classList.add('header-in-summary');
            filterHeader(header.children[0], a);
            a.addEventListener('click', headerThresholdClick);
            const nextHeader = headers[i + 1];
            if (nextHeader !== undefined) {
                const nextLevel = parseInt(nextHeader.tagName.charAt(1));
                if (nextLevel > level && level >= foldLevel) {
                    const toggle = document.createElement('a');
                    toggle.classList.add('chapter-fold-toggle');
                    toggle.classList.add('header-toggle');
                    toggle.addEventListener('click', () => {
                        li.classList.toggle('expanded');
                    });
                    const toggleDiv = document.createElement('div');
                    toggleDiv.textContent = '❱';
                    toggle.appendChild(toggleDiv);
                    span.appendChild(toggle);
                    headerToggles.push(li);
                }
            }
            li.appendChild(span);

            const currentParent = stack[stack.length - 1];
            currentParent.ol.appendChild(li);
        }

        const onThisPage = document.createElement('div');
        onThisPage.classList.add('on-this-page');
        onThisPage.append(stack[0].ol);
        const activeItemSpan = activeSection.parentElement;
        activeItemSpan.after(onThisPage);
    });

    document.addEventListener('DOMContentLoaded', reloadCurrentHeader);
    document.addEventListener('scroll', reloadCurrentHeader, { passive: true });
})();

