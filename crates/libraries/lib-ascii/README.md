# ascii

## Architecture

This component handles all logic related to representing diagrams with ASCII characters.
Following Clean Architecture, this component is a 'plugin' to the main higher level uml component

That means it can theoratically be swapped for another component (ie. a Graphic component)

It is composed of two layers: the Adapters layer and the Infrastructure layer:
In the Adapters layer the main sub-components are the Presenter and the view model. In the Infrastructure layer it is the AsciiRenderer interface

![ascii architecture](docs/diagrams/rendered/ascii_architecture.png "ascii Component Architecture")

### Presenter

Responsible for translating Diagram data structures to ASCII characters draw calls. It primarily achieves this by using an ASCII grid of characters (view model)

### Renderer

This interface defines the high-level capabilities of an ASCII Renderer. Mainly, the ability to visually represent the given ASCII grid of characters.
Its implementations are details not pertinent to this component (we could have a TUI renderer, a CLI renderer on a Web-based one - all created in a separate component)
