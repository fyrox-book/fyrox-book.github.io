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
        this.innerHTML = '<ol class="chapter"><li class="chapter-item expanded "><a href="introduction.html"><strong aria-hidden="true">1.</strong> About the Book</a></li><li class="chapter-item expanded "><a href="introduction/index.html"><strong aria-hidden="true">2.</strong> Introduction</a><a class="toggle"><div>❱</div></a></li><li><ol class="section"><li class="chapter-item expanded "><a href="introduction/introduction.html"><strong aria-hidden="true">2.1.</strong> Introduction to Fyrox</a></li><li class="chapter-item expanded "><a href="introduction/requirements.html"><strong aria-hidden="true">2.2.</strong> System Requirements and Supported Platforms</a></li><li class="chapter-item expanded "><a href="introduction/basic_concepts.html"><strong aria-hidden="true">2.3.</strong> Basic concepts</a></li><li class="chapter-item expanded "><a href="introduction/philosophy_and_goals.html"><strong aria-hidden="true">2.4.</strong> Design Philosophy and Goals</a></li><li class="chapter-item expanded "><a href="introduction/faq.html"><strong aria-hidden="true">2.5.</strong> Frequently Asked Questions</a></li></ol></li><li class="chapter-item expanded "><a href="beginning/getting_started.html"><strong aria-hidden="true">3.</strong> Getting started</a><a class="toggle"><div>❱</div></a></li><li><ol class="section"><li class="chapter-item expanded "><a href="beginning/scripting.html"><strong aria-hidden="true">3.1.</strong> Editor, Plugins and Scripts</a></li><li class="chapter-item expanded "><a href="beginning/project_manager.html"><strong aria-hidden="true">3.2.</strong> Project Manager</a></li><li class="chapter-item expanded "><a href="beginning/hot_reloading.html"><strong aria-hidden="true">3.3.</strong> Hot Reloading</a></li><li class="chapter-item expanded "><a href="beginning/editor_overview.html"><strong aria-hidden="true">3.4.</strong> Editor Overview</a></li><li class="chapter-item expanded "><a href="beginning/scene_and_scene_graph.html"><strong aria-hidden="true">3.5.</strong> Scene and Scene Graph</a></li><li class="chapter-item expanded "><a href="beginning/assets.html"><strong aria-hidden="true">3.6.</strong> Assets</a></li><li class="chapter-item expanded "><a href="beginning/data_management.html"><strong aria-hidden="true">3.7.</strong> Data Management</a></li><li class="chapter-item expanded "><a href="beginning/borrow_checker.html"><strong aria-hidden="true">3.8.</strong> Borrow Checker</a></li></ol></li><li class="chapter-item expanded "><a href="scripting/scripting.html"><strong aria-hidden="true">4.</strong> Scripting</a><a class="toggle"><div>❱</div></a></li><li><ol class="section"><li class="chapter-item expanded "><a href="scripting/plugin.html"><strong aria-hidden="true">4.1.</strong> Plugins</a></li><li class="chapter-item expanded "><a href="scripting/executor.html"><strong aria-hidden="true">4.2.</strong> Executor</a></li><li class="chapter-item expanded "><a href="scripting/script.html"><strong aria-hidden="true">4.3.</strong> Scripts</a></li><li class="chapter-item expanded "><a href="scripting/tasks.html"><strong aria-hidden="true">4.4.</strong> Tasks</a></li></ol></li><li class="chapter-item expanded "><a href="scene/scene.html"><strong aria-hidden="true">5.</strong> Scene</a><a class="toggle"><div>❱</div></a></li><li><ol class="section"><li class="chapter-item expanded "><a href="scene/graph.html"><strong aria-hidden="true">5.1.</strong> Graph</a></li><li class="chapter-item expanded "><a href="scene/transform.html"><strong aria-hidden="true">5.2.</strong> Transformation</a></li><li class="chapter-item expanded "><a href="scene/prefab.html"><strong aria-hidden="true">5.3.</strong> Prefabs</a></li><li class="chapter-item expanded "><a href="scene/inheritance.html"><strong aria-hidden="true">5.4.</strong> Property Inheritance</a></li><li class="chapter-item expanded "><a href="scene/base_node.html"><strong aria-hidden="true">5.5.</strong> Base Node</a></li><li class="chapter-item expanded "><a href="scene/mesh_node.html"><strong aria-hidden="true">5.6.</strong> Mesh Node</a></li><li class="chapter-item expanded "><a href="scene/light_node.html"><strong aria-hidden="true">5.7.</strong> Light Node</a></li><li class="chapter-item expanded "><a href="scene/sprite_node.html"><strong aria-hidden="true">5.8.</strong> Sprite Node</a></li><li class="chapter-item expanded "><a href="scene/particle_system_node.html"><strong aria-hidden="true">5.9.</strong> Particle System Node</a></li><li class="chapter-item expanded "><a href="scene/terrain_node.html"><strong aria-hidden="true">5.10.</strong> Terrain Node</a></li><li class="chapter-item expanded "><a href="scene/camera_node.html"><strong aria-hidden="true">5.11.</strong> Camera Node</a></li><li class="chapter-item expanded "><a href="scene/decal_node.html"><strong aria-hidden="true">5.12.</strong> Decal Node</a></li><li class="chapter-item expanded "><a href="scene/rectangle.html"><strong aria-hidden="true">5.13.</strong> Rectangle Node</a></li><li class="chapter-item expanded "><a href="scene/tilemap.html"><strong aria-hidden="true">5.14.</strong> Tile Map</a></li><li class="chapter-item expanded "><a href="scene/custom_node.html"><strong aria-hidden="true">5.15.</strong> Custom Node</a></li><li class="chapter-item expanded "><a href="physics/physics.html"><strong aria-hidden="true">5.16.</strong> Physics</a><a class="toggle"><div>❱</div></a></li><li><ol class="section"><li class="chapter-item expanded "><a href="physics/rigid_body.html"><strong aria-hidden="true">5.16.1.</strong> Rigid Body</a></li><li class="chapter-item expanded "><a href="physics/collider.html"><strong aria-hidden="true">5.16.2.</strong> Collider</a></li><li class="chapter-item expanded "><a href="physics/joint.html"><strong aria-hidden="true">5.16.3.</strong> Joint</a></li><li class="chapter-item expanded "><a href="physics/ray.html"><strong aria-hidden="true">5.16.4.</strong> Ray Casting</a></li><li class="chapter-item expanded "><a href="physics/ragdoll.html"><strong aria-hidden="true">5.16.5.</strong> Ragdoll</a></li></ol></li><li class="chapter-item expanded "><a href="sound/index.html"><strong aria-hidden="true">5.17.</strong> Sound</a><a class="toggle"><div>❱</div></a></li><li><ol class="section"><li class="chapter-item expanded "><a href="sound/bus.html"><strong aria-hidden="true">5.17.1.</strong> Audio Bus</a></li><li class="chapter-item expanded "><a href="sound/sound.html"><strong aria-hidden="true">5.17.2.</strong> Sound Node</a></li><li class="chapter-item expanded "><a href="sound/hrtf.html"><strong aria-hidden="true">5.17.3.</strong> HRTF</a></li></ol></li><li class="chapter-item expanded "><a href="animation/animation.html"><strong aria-hidden="true">5.18.</strong> Animation</a><a class="toggle"><div>❱</div></a></li><li><ol class="section"><li class="chapter-item expanded "><a href="animation/anim_editor.html"><strong aria-hidden="true">5.18.1.</strong> Animation Editor</a></li><li class="chapter-item expanded "><a href="animation/blending.html"><strong aria-hidden="true">5.18.2.</strong> Animation Blending</a></li><li class="chapter-item expanded "><a href="animation/absm_editor.html"><strong aria-hidden="true">5.18.3.</strong> ABSM Editor</a></li><li class="chapter-item expanded "><a href="animation/signal.html"><strong aria-hidden="true">5.18.4.</strong> Signals</a></li><li class="chapter-item expanded "><a href="animation/root_motion/root_motion.html"><strong aria-hidden="true">5.18.5.</strong> Root Motion</a></li><li class="chapter-item expanded "><a href="animation/spritesheet/spritesheet.html"><strong aria-hidden="true">5.18.6.</strong> Sprite Animation</a></li></ol></li><li class="chapter-item expanded "><a href="scene/debug.html"><strong aria-hidden="true">5.19.</strong> Debugging</a></li></ol></li><li class="chapter-item expanded "><a href="input/input.html"><strong aria-hidden="true">6.</strong> Input Handling</a><a class="toggle"><div>❱</div></a></li><li><ol class="section"><li class="chapter-item expanded "><a href="input/keyboard.html"><strong aria-hidden="true">6.1.</strong> Keyboard</a></li><li class="chapter-item expanded "><a href="input/mouse.html"><strong aria-hidden="true">6.2.</strong> Mouse</a></li><li class="chapter-item expanded "><a href="input/text.html"><strong aria-hidden="true">6.3.</strong> Raw Text Input (WIP)</a></li></ol></li><li class="chapter-item expanded "><a href="ai/ai.html"><strong aria-hidden="true">7.</strong> Artificial Intelligence (WIP)</a><a class="toggle"><div>❱</div></a></li><li><ol class="section"><li class="chapter-item expanded "><a href="ai/beh_tree.html"><strong aria-hidden="true">7.1.</strong> Behaviour Trees (WIP)</a></li><li class="chapter-item expanded "><a href="ai/pathfinding.html"><strong aria-hidden="true">7.2.</strong> Path Finding</a></li><li class="chapter-item expanded "><a href="ai/navmesh.html"><strong aria-hidden="true">7.3.</strong> Navigational Meshes</a></li></ol></li><li class="chapter-item expanded "><a href="rendering/rendering.html"><strong aria-hidden="true">8.</strong> Rendering</a><a class="toggle"><div>❱</div></a></li><li><ol class="section"><li class="chapter-item expanded "><a href="rendering/shaders.html"><strong aria-hidden="true">8.1.</strong> Shaders</a></li><li class="chapter-item expanded "><a href="rendering/materials.html"><strong aria-hidden="true">8.2.</strong> Materials</a></li><li class="chapter-item expanded "><a href="rendering/lightmaps.html"><strong aria-hidden="true">8.3.</strong> Light Maps</a></li><li class="chapter-item expanded "><a href="rendering/settings.html"><strong aria-hidden="true">8.4.</strong> Settings</a></li><li class="chapter-item expanded "><a href="rendering/render_pass.html"><strong aria-hidden="true">8.5.</strong> Render Pass</a></li><li class="chapter-item expanded "><a href="rendering/normal_maps.html"><strong aria-hidden="true">8.6.</strong> Normal Maps</a></li></ol></li><li class="chapter-item expanded "><a href="resources/resources.html"><strong aria-hidden="true">9.</strong> Resource Management</a><a class="toggle"><div>❱</div></a></li><li><ol class="section"><li class="chapter-item expanded "><a href="resources/model.html"><strong aria-hidden="true">9.1.</strong> Models</a></li><li class="chapter-item expanded "><a href="resources/texture.html"><strong aria-hidden="true">9.2.</strong> Textures</a></li><li class="chapter-item expanded "><a href="resources/sound.html"><strong aria-hidden="true">9.3.</strong> Sound Buffers</a></li><li class="chapter-item expanded "><a href="resources/curve.html"><strong aria-hidden="true">9.4.</strong> Curves (WIP)</a></li><li class="chapter-item expanded "><a href="resources/custom.html"><strong aria-hidden="true">9.5.</strong> Custom Resource</a></li><li class="chapter-item expanded "><a href="resources/hot_reloading.html"><strong aria-hidden="true">9.6.</strong> Asset Hot Reloading</a></li><li class="chapter-item expanded "><a href="resources/events.html"><strong aria-hidden="true">9.7.</strong> Events</a></li></ol></li><li class="chapter-item expanded "><a href="engine/engine.html"><strong aria-hidden="true">10.</strong> Engine</a><a class="toggle"><div>❱</div></a></li><li><ol class="section"><li class="chapter-item expanded "><a href="engine/graphics_context.html"><strong aria-hidden="true">10.1.</strong> Graphics Context</a></li><li class="chapter-item expanded "><a href="engine/windows.html"><strong aria-hidden="true">10.2.</strong> Window Management</a></li><li class="chapter-item expanded "><a href="engine/manual_initialization.html"><strong aria-hidden="true">10.3.</strong> Manual Initialization</a></li></ol></li><li class="chapter-item expanded "><a href="net/net.html"><strong aria-hidden="true">11.</strong> Network</a><a class="toggle"><div>❱</div></a></li><li><ol class="section"><li class="chapter-item expanded "><a href="net/basics.html"><strong aria-hidden="true">11.1.</strong> Basics</a></li><li class="chapter-item expanded "><a href="net/sync.html"><strong aria-hidden="true">11.2.</strong> Synchronization</a></li></ol></li><li class="chapter-item expanded "><a href="ui/ui.html"><strong aria-hidden="true">12.</strong> User Interface</a><a class="toggle"><div>❱</div></a></li><li><ol class="section"><li class="chapter-item expanded "><a href="ui/basic_concepts/basic_concepts.html"><strong aria-hidden="true">12.1.</strong> Basic concepts</a></li><li class="chapter-item expanded "><a href="ui/editor/editor.html"><strong aria-hidden="true">12.2.</strong> Editor</a></li><li class="chapter-item expanded "><a href="ui/rendering.html"><strong aria-hidden="true">12.3.</strong> Rendering</a></li><li class="chapter-item expanded "><a href="ui/font.html"><strong aria-hidden="true">12.4.</strong> Fonts</a></li><li class="chapter-item expanded "><a href="ui/style.html"><strong aria-hidden="true">12.5.</strong> Style</a></li><li class="chapter-item expanded "><a href="ui/widgets.html"><strong aria-hidden="true">12.6.</strong> Widgets</a><a class="toggle"><div>❱</div></a></li><li><ol class="section"><li class="chapter-item expanded "><a href="ui/custom.html"><strong aria-hidden="true">12.6.1.</strong> Custom widget</a></li><li class="chapter-item expanded "><a href="ui/button.html"><strong aria-hidden="true">12.6.2.</strong> Button</a></li><li class="chapter-item expanded "><a href="ui/border.html"><strong aria-hidden="true">12.6.3.</strong> Border</a></li><li class="chapter-item expanded "><a href="ui/canvas.html"><strong aria-hidden="true">12.6.4.</strong> Canvas</a></li><li class="chapter-item expanded "><a href="ui/checkbox/check_box.html"><strong aria-hidden="true">12.6.5.</strong> Check box</a></li><li class="chapter-item expanded "><a href="ui/curve_editor.html"><strong aria-hidden="true">12.6.6.</strong> Curve editor (WIP)</a></li><li class="chapter-item expanded "><a href="ui/decorator.html"><strong aria-hidden="true">12.6.7.</strong> Decorator</a></li><li class="chapter-item expanded "><a href="ui/dock.html"><strong aria-hidden="true">12.6.8.</strong> Docking manager (WIP)</a></li><li class="chapter-item expanded "><a href="ui/dropdown_list.html"><strong aria-hidden="true">12.6.9.</strong> Dropdown list (WIP)</a></li><li class="chapter-item expanded "><a href="ui/expander.html"><strong aria-hidden="true">12.6.10.</strong> Expander</a></li><li class="chapter-item expanded "><a href="ui/file_browser.html"><strong aria-hidden="true">12.6.11.</strong> File browser (WIP)</a></li><li class="chapter-item expanded "><a href="ui/grid.html"><strong aria-hidden="true">12.6.12.</strong> Grid</a></li><li class="chapter-item expanded "><a href="ui/image.html"><strong aria-hidden="true">12.6.13.</strong> Image</a></li><li class="chapter-item expanded "><a href="ui/inspector.html"><strong aria-hidden="true">12.6.14.</strong> Inspector (WIP)</a></li><li class="chapter-item expanded "><a href="ui/list_view.html"><strong aria-hidden="true">12.6.15.</strong> List view (WIP)</a></li><li class="chapter-item expanded "><a href="ui/menu.html"><strong aria-hidden="true">12.6.16.</strong> Menu (WIP)</a></li><li class="chapter-item expanded "><a href="ui/message_box.html"><strong aria-hidden="true">12.6.17.</strong> Message box (WIP)</a></li><li class="chapter-item expanded "><a href="ui/numeric.html"><strong aria-hidden="true">12.6.18.</strong> Numeric field</a></li><li class="chapter-item expanded "><a href="ui/popup.html"><strong aria-hidden="true">12.6.19.</strong> Popup (WIP)</a></li><li class="chapter-item expanded "><a href="ui/progress_bar.html"><strong aria-hidden="true">12.6.20.</strong> Progress bar (WIP)</a></li><li class="chapter-item expanded "><a href="ui/range.html"><strong aria-hidden="true">12.6.21.</strong> Range (WIP)</a></li><li class="chapter-item expanded "><a href="ui/rect.html"><strong aria-hidden="true">12.6.22.</strong> Rect (WIP)</a></li><li class="chapter-item expanded "><a href="ui/scroll_bar.html"><strong aria-hidden="true">12.6.23.</strong> Scroll bar</a></li><li class="chapter-item expanded "><a href="ui/scroll_panel.html"><strong aria-hidden="true">12.6.24.</strong> Scroll panel</a></li><li class="chapter-item expanded "><a href="ui/scroll_viewer.html"><strong aria-hidden="true">12.6.25.</strong> Scroll viewer</a></li><li class="chapter-item expanded "><a href="ui/screen.html"><strong aria-hidden="true">12.6.26.</strong> Screen</a></li><li class="chapter-item expanded "><a href="ui/stack_panel.html"><strong aria-hidden="true">12.6.27.</strong> Stack panel</a></li><li class="chapter-item expanded "><a href="ui/tab_control.html"><strong aria-hidden="true">12.6.28.</strong> Tab control</a></li><li class="chapter-item expanded "><a href="ui/text.html"><strong aria-hidden="true">12.6.29.</strong> Text</a></li><li class="chapter-item expanded "><a href="ui/text_box.html"><strong aria-hidden="true">12.6.30.</strong> Text box</a></li><li class="chapter-item expanded "><a href="ui/tree.html"><strong aria-hidden="true">12.6.31.</strong> Tree (WIP)</a></li><li class="chapter-item expanded "><a href="ui/vector_image.html"><strong aria-hidden="true">12.6.32.</strong> Vector image</a></li><li class="chapter-item expanded "><a href="ui/window.html"><strong aria-hidden="true">12.6.33.</strong> Window</a></li><li class="chapter-item expanded "><a href="ui/wrap_panel.html"><strong aria-hidden="true">12.6.34.</strong> Wrap panel</a></li></ol></li></ol></li><li class="chapter-item expanded "><a href="serialization/index.html"><strong aria-hidden="true">13.</strong> Serialization</a><a class="toggle"><div>❱</div></a></li><li><ol class="section"><li class="chapter-item expanded "><a href="serialization/save.html"><strong aria-hidden="true">13.1.</strong> Saved Games</a></li></ol></li><li class="chapter-item expanded "><a href="editor/index.html"><strong aria-hidden="true">14.</strong> Editor</a><a class="toggle"><div>❱</div></a></li><li><ol class="section"><li class="chapter-item expanded "><a href="editor/property_editors.html"><strong aria-hidden="true">14.1.</strong> Property Editors</a></li><li class="chapter-item expanded "><a href="editor/settings.html"><strong aria-hidden="true">14.2.</strong> Settings</a></li><li class="chapter-item expanded "><a href="editor/plugins.html"><strong aria-hidden="true">14.3.</strong> Plugins</a></li></ol></li><li class="chapter-item expanded "><a href="misc/misc.html"><strong aria-hidden="true">15.</strong> Miscellaneous</a><a class="toggle"><div>❱</div></a></li><li><ol class="section"><li class="chapter-item expanded "><a href="misc/log.html"><strong aria-hidden="true">15.1.</strong> Log</a></li></ol></li><li class="chapter-item expanded "><a href="shipping/shipping.html"><strong aria-hidden="true">16.</strong> Shipping</a><a class="toggle"><div>❱</div></a></li><li><ol class="section"><li class="chapter-item expanded "><a href="shipping/pc.html"><strong aria-hidden="true">16.1.</strong> PC</a></li><li class="chapter-item expanded "><a href="shipping/wasm.html"><strong aria-hidden="true">16.2.</strong> WebAssembly</a></li><li class="chapter-item expanded "><a href="shipping/android.html"><strong aria-hidden="true">16.3.</strong> Android</a></li></ol></li><li class="chapter-item expanded "><a href="tutorials/tutorials.html"><strong aria-hidden="true">17.</strong> Tutorials</a><a class="toggle"><div>❱</div></a></li><li><ol class="section"><li class="chapter-item expanded "><a href="tutorials/platformer/intro.html"><strong aria-hidden="true">17.1.</strong> 2D Platformer Tutorial</a><a class="toggle"><div>❱</div></a></li><li><ol class="section"><li class="chapter-item expanded "><a href="tutorials/platformer/part1.html"><strong aria-hidden="true">17.1.1.</strong> Character Controller</a></li><li class="chapter-item expanded "><a href="tutorials/platformer/part2.html"><strong aria-hidden="true">17.1.2.</strong> Bots and AI</a></li></ol></li><li class="chapter-item expanded "><a href="tutorials/rpg/intro.html"><strong aria-hidden="true">17.2.</strong> RPG Tutorial</a><a class="toggle"><div>❱</div></a></li><li><ol class="section"><li class="chapter-item expanded "><a href="tutorials/rpg/tutorial-1/tutorial-part-1.html"><strong aria-hidden="true">17.2.1.</strong> Character Controller</a></li></ol></li><li class="chapter-item expanded "><a href="tutorials/fps/fps-intro.html"><strong aria-hidden="true">17.3.</strong> FPS Tutorial</a><a class="toggle"><div>❱</div></a></li><li><ol class="section"><li class="chapter-item expanded "><a href="tutorials/fps/tutorial-1/fps-tutorial.html"><strong aria-hidden="true">17.3.1.</strong> Character Controller</a></li><li class="chapter-item expanded "><a href="tutorials/fps/tutorial-2/fps-tutorial-2.html"><strong aria-hidden="true">17.3.2.</strong> Weapons</a></li><li class="chapter-item expanded "><a href="tutorials/fps/tutorial-3/fps-tutorial-3.html"><strong aria-hidden="true">17.3.3.</strong> Bots and AI</a></li></ol></li><li class="chapter-item expanded "><a href="tutorials/ui/ui-tutorial.html"><strong aria-hidden="true">17.4.</strong> User Interface Tutorial (WIP)</a></li><li class="chapter-item expanded "><a href="tutorials/community.html"><strong aria-hidden="true">17.5.</strong> Community Tutorials</a></li></ol></li><li class="chapter-item expanded "><a href="performance/index.html"><strong aria-hidden="true">18.</strong> Performance</a></li></ol>';
        // Set the current, active page, and reveal it if it's hidden
        let current_page = document.location.href.toString().split("#")[0];
        if (current_page.endsWith("/")) {
            current_page += "index.html";
        }
        var links = Array.prototype.slice.call(this.querySelectorAll("a"));
        var l = links.length;
        for (var i = 0; i < l; ++i) {
            var link = links[i];
            var href = link.getAttribute("href");
            if (href && !href.startsWith("#") && !/^(?:[a-z+]+:)?\/\//.test(href)) {
                link.href = path_to_root + href;
            }
            // The "index" page is supposed to alias the first chapter in the book.
            if (link.href === current_page || (i === 0 && path_to_root === "" && current_page.endsWith("/index.html"))) {
                link.classList.add("active");
                var parent = link.parentElement;
                if (parent && parent.classList.contains("chapter-item")) {
                    parent.classList.add("expanded");
                }
                while (parent) {
                    if (parent.tagName === "LI" && parent.previousElementSibling) {
                        if (parent.previousElementSibling.classList.contains("chapter-item")) {
                            parent.previousElementSibling.classList.add("expanded");
                        }
                    }
                    parent = parent.parentElement;
                }
            }
        }
        // Track and set sidebar scroll position
        this.addEventListener('click', function(e) {
            if (e.target.tagName === 'A') {
                sessionStorage.setItem('sidebar-scroll', this.scrollTop);
            }
        }, { passive: true });
        var sidebarScrollTop = sessionStorage.getItem('sidebar-scroll');
        sessionStorage.removeItem('sidebar-scroll');
        if (sidebarScrollTop) {
            // preserve sidebar scroll position when navigating via links within sidebar
            this.scrollTop = sidebarScrollTop;
        } else {
            // scroll sidebar to current active section when navigating via "next/previous chapter" buttons
            var activeSection = document.querySelector('#sidebar .active');
            if (activeSection) {
                activeSection.scrollIntoView({ block: 'center' });
            }
        }
        // Toggle buttons
        var sidebarAnchorToggles = document.querySelectorAll('#sidebar a.toggle');
        function toggleSection(ev) {
            ev.currentTarget.parentElement.classList.toggle('expanded');
        }
        Array.from(sidebarAnchorToggles).forEach(function (el) {
            el.addEventListener('click', toggleSection);
        });
    }
}
window.customElements.define("mdbook-sidebar-scrollbox", MDBookSidebarScrollbox);
