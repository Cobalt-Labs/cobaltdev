import 'package:flutter/material.dart';
import 'package:url_launcher/url_launcher.dart';
import 'package:flutter_dropzone/flutter_dropzone.dart';
import '../widgets/animated_section.dart';
import '../widgets/glass_card.dart';

class CloudPage extends StatefulWidget {
  const CloudPage({super.key});

  @override
  State<CloudPage> createState() => _CloudPageState();
}

class _CloudPageState extends State<CloudPage> {
  late DropzoneViewController controller;
  int dropCount = 0;
  bool isHighlighted = false;
  final int dropLimit = 3;
  List<String> uploadedFiles = [];

  Future<void> _openGithub() async {
    final url = Uri.parse("https://github.com/ibrahim-3595/cobaltdev/tree/main/cobalt_cloud");
    if (await canLaunchUrl(url)) {
      await launchUrl(url, mode: LaunchMode.externalApplication);
    }
  }

  void _onDrop(dynamic file) async {
    if (dropCount >= dropLimit) {
      ScaffoldMessenger.of(context).showSnackBar(
        const SnackBar(
          content: Text("Limit reached: You can only drop 3 files in this demo."),
          backgroundColor: Colors.redAccent,
        ),
      );
      return;
    }

    final name = await controller.getFilename(file);
    setState(() {
      dropCount++;
      uploadedFiles.add(name);
      isHighlighted = false;
    });

    ScaffoldMessenger.of(context).showSnackBar(
      SnackBar(
        content: Text("Uploaded $name successfully! ($dropCount/$dropLimit)"),
        backgroundColor: const Color(0xFF4F46E5), // Indigo
      ),
    );
  }

  @override
  Widget build(BuildContext context) {
    final width = MediaQuery.of(context).size.width;
    final isDesktop = width > 900;
    final isMobile = width < 700;

    return Scaffold(
      backgroundColor: Colors.transparent,
      body: SingleChildScrollView(
        child: Padding(
          padding: EdgeInsets.symmetric(horizontal: isMobile ? 24 : 48, vertical: isMobile ? 40 : 80),
          child: Column(
            crossAxisAlignment: CrossAxisAlignment.start,
            children: [
              AnimatedSection(
                child: Column(
                  crossAxisAlignment: CrossAxisAlignment.start,
                  children: [
                    Text(
                      "Cobalt Cloud",
                      style: TextStyle(
                        fontSize: isDesktop ? 48 : 36,
                        fontWeight: FontWeight.w600,
                        letterSpacing: -1,
                        color: Colors.white,
                      ),
                    ),
                    const SizedBox(height: 12),
                    const Text(
                      "A self-hosted private cloud infrastructure built with Rust.",
                      style: TextStyle(fontSize: 18, color: Colors.white70),
                    ),
                    const SizedBox(height: 32),
                    ElevatedButton.icon(
                      onPressed: _openGithub,
                      icon: const Icon(Icons.code, size: 18),
                      label: const Text("View on GitHub", style: TextStyle(fontWeight: FontWeight.w500)),
                      style: ElevatedButton.styleFrom(
                        backgroundColor: const Color(0xFF27272A), // zinc-800
                        foregroundColor: Colors.white,
                        padding: const EdgeInsets.symmetric(horizontal: 24, vertical: 16),
                        shape: RoundedRectangleBorder(borderRadius: BorderRadius.circular(8)),
                        elevation: 0,
                      ),
                    ),
                  ],
                ),
              ),

              const SizedBox(height: 60),

              // DROP ZONE SECTION
              AnimatedSection(
                child: Column(
                  crossAxisAlignment: CrossAxisAlignment.start,
                  children: [
                    const Text(
                      "Upload Files",
                      style: TextStyle(fontSize: 24, fontWeight: FontWeight.w600),
                    ),
                    const SizedBox(height: 8),
                    Text(
                      "Drag and drop files here to upload to the local node.",
                      style: const TextStyle(color: Colors.white54, fontSize: 15),
                    ),
                    const SizedBox(height: 24),
                    Stack(
                      children: [
                        SizedBox(
                          height: 200,
                          child: DropzoneView(
                            operation: DragOperation.copy,
                            cursor: CursorType.Default,
                            onCreated: (ctrl) => controller = ctrl,
                            onDrop: _onDrop,
                            onHover: () => setState(() => isHighlighted = true),
                            onLeave: () => setState(() => isHighlighted = false),
                          ),
                        ),
                        IgnorePointer(
                          child: Container(
                            height: 200,
                            decoration: BoxDecoration(
                              color: isHighlighted 
                                ? const Color(0xFF27272A) // zinc-800
                                : Colors.transparent,
                              borderRadius: BorderRadius.circular(8),
                              border: Border.all(
                                color: isHighlighted ? const Color(0xFFA855F7) : const Color(0xFF3F3F46),
                                width: 1,
                                // Dashed effect visually isn't native to basic Border without custom painter, 
                                // so we use a solid flat border with Vercel aesthetic
                                style: BorderStyle.solid,
                              ),
                            ),
                            child: Center(
                              child: Column(
                                mainAxisAlignment: MainAxisAlignment.center,
                                children: [
                                  Icon(
                                    Icons.cloud_upload_outlined,
                                    size: 32,
                                    color: isHighlighted ? const Color(0xFFA855F7) : Colors.white38,
                                  ),
                                  const SizedBox(height: 16),
                                  Text(
                                    isHighlighted ? "Drop file to upload" : "Select or drag files",
                                    style: TextStyle(
                                      fontSize: 15,
                                      fontWeight: FontWeight.w500,
                                      color: isHighlighted ? Colors.white : Colors.white70,
                                    ),
                                  ),
                                  if (dropCount > 0) ...[
                                    const SizedBox(height: 12),
                                    Text(
                                      "$dropCount / $dropLimit uploaded",
                                      style: const TextStyle(color: Color(0xFFA855F7), fontWeight: FontWeight.w500, fontSize: 13),
                                    ),
                                  ]
                                ],
                              ),
                            ),
                          ),
                        ),
                      ],
                    ),
                    if (uploadedFiles.isNotEmpty) ...[
                      const SizedBox(height: 20),
                      Wrap(
                        spacing: 8,
                        children: uploadedFiles.map((f) => Container(
                          padding: const EdgeInsets.symmetric(horizontal: 12, vertical: 6),
                          decoration: BoxDecoration(
                            color: const Color(0xFF27272A), // zinc-800
                            borderRadius: BorderRadius.circular(4),
                          ),
                          child: Text(f, style: const TextStyle(color: Colors.white70, fontSize: 13)),
                        )).toList(),
                      ),
                    ]
                  ],
                ),
              ),

              const SizedBox(height: 80),

              Wrap(
                spacing: 24,
                runSpacing: 24,
                children: [
                  _featureCard(context,
                    "Frontend (Dioxus)",
                    "A cross-platform Rust frontend framework used to create the client-side app. Enables smooth drag & drop.",
                  ),
                  _featureCard(context,
                    "Backend (Axum)",
                    "High performance backend API running in Rust using Axum and object_store.",
                  ),
                  _featureCard(context,
                    "Self-Hosted",
                    "Designed to run on your own hardware. Your data stays on your local hard drive.",
                  ),
                ],
              ),
              
              const SizedBox(height: 80),

            ],
          ),
        ),
      ),
    );
  }

  Widget _featureCard(BuildContext context, String title, String desc) {
    final sw = MediaQuery.of(context).size.width;
    return SizedBox(
      width: sw < 450 ? sw - 48 : 340,
      child: GlassCard(
        padding: const EdgeInsets.all(24),
        child: Column(
          crossAxisAlignment: CrossAxisAlignment.start,
          children: [
            Text(title, style: const TextStyle(fontSize: 18, fontWeight: FontWeight.w600)),
            const SizedBox(height: 12),
            Text(desc, style: const TextStyle(color: Colors.white70, height: 1.5, fontSize: 15)),
          ],
        ),
      ),
    );
  }
}
