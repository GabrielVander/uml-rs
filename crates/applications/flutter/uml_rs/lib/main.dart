import 'package:flutter/material.dart';
import 'package:highlight/languages/xml.dart';
import 'package:multi_split_view/multi_split_view.dart';
import 'package:uml_rs/src/rust/frb_generated.dart';
import 'package:highlight/languages/plaintext.dart';
import 'package:flutter_highlight/themes/monokai-sublime.dart';
import 'package:code_text_field/code_text_field.dart';
import 'package:flutter_svg/flutter_svg.dart';

Future<void> main() async {
  await RustLib.init();
  runApp(const MyApp());
}

class MyApp extends StatelessWidget {
  const MyApp({super.key});

  @override
  Widget build(BuildContext context) {
    return MaterialApp(home: SvgEditorPage());
  }
}

class SvgEditorPage extends StatefulWidget {
  const SvgEditorPage({super.key});

  @override
  State<SvgEditorPage> createState() => _SvgEditorPageState();
}

class _SvgEditorPageState extends State<SvgEditorPage> {
  late CodeController _codeController;

  // Initial default SVG code
  String _svgCode = '''
<svg viewBox="0 0 100 100" xmlns="http://www.w3.org/2000/svg">
  <circle cx="50" cy="50" r="40" stroke="green" stroke-width="4" fill="yellow" />
  <rect x="20" y="20" width="60" height="60" rx="10" ry="10" fill="none" stroke="red" stroke-width="2" />
</svg>
''';

  @override
  void initState() {
    super.initState();
    // Initialize the editor controller with XML/SVG syntax highlighting
    _codeController = CodeController(
      text: _svgCode,
      language: xml, // SVG uses XML syntax
    );
  }

  @override
  void dispose() {
    _codeController.dispose();
    super.dispose();
  }

  @override
  Widget build(BuildContext context) {
    // MultiSplitView allows resizing between the two children
    return Scaffold(
      appBar: AppBar(title: const Text('Flutter SVG Live Editor')),
      body: MultiSplitView(
        axis: Axis.horizontal, // Split vertically (Side by Side)
        initialAreas: [
          Area(id: "editor", flex: 0.5), // Each takes 50% initially
          Area(id: "preview", flex: 0.5),
        ],
        builder: (context, area) =>
            area.id == "editor" ? _buildEditorPane() : _buildPreviewPane(),
      ),
    );
  }

  Widget _buildEditorPane() {
    return CodeTheme(
      data: CodeThemeData(styles: monokaiSublimeTheme),
      child: Container(
        color: const Color(0xFF272822), // Background color fills the pane
        height: double.infinity, // Forces container to take full height
        child: SingleChildScrollView(
          child: CodeField(
            controller: _codeController,
            // 1. Enable Line Numbers
            lineNumbers: true,

            // 2. Style the text
            textStyle: const TextStyle(fontFamily: 'monospace', fontSize: 12),

            // 3. Optional: Customize the Gutter (Line Number area) styles
            lineNumberStyle: const LineNumberStyle(
              width: 45, // Width of the gutter
              margin: 5, // Space between numbers and code
              textStyle: TextStyle(
                color: Colors.grey, // Color of the line numbers
                fontSize: 14,
              ),
            ),

            // 4. Ensure the field expands to fill the SingleChildScrollView
            expands: false,
            wrap: true, // Set to true if you want code to wrap lines

            onChanged: (value) {
              setState(() {
                _svgCode = value;
              });
            },
          ),
        ),
      ),
    );
  }

  // Section 2: The SVG Visualizer
  Widget _buildPreviewPane() {
    return Container(
      color: Colors.grey.shade200, // Light background to see the SVG clearly
      alignment: Alignment.center,
      child: _svgCode.trim().isEmpty
          ? const Text("Enter SVG Code")
          : _SafeSvgDisplay(svgString: _svgCode),
    );
  }
}

// Helper widget to handle invalid SVG strings gracefully
class _SafeSvgDisplay extends StatelessWidget {
  final String svgString;

  const _SafeSvgDisplay({required this.svgString});

  @override
  Widget build(BuildContext context) {
    try {
      return SvgPicture.string(
        svgString,
        width: 300,
        height: 300,
        placeholderBuilder: (context) => const CircularProgressIndicator(),
      );
    } catch (e) {
      // If the user types invalid XML, show an error icon or nothing
      return const Icon(Icons.error_outline, color: Colors.red, size: 48);
    }
  }
}
