import 'package:flutter/material.dart';
import 'package:video_player/video_player.dart';
import 'package:url_launcher/url_launcher.dart';

import '../widgets/animated_section.dart';
import '../widgets/glass_card.dart';

class HomePage extends StatefulWidget {
  const HomePage({super.key});

  @override
  State<HomePage> createState() => _HomePageState();
}

class _HomePageState extends State<HomePage> {
  late VideoPlayerController _controller;

  @override
  void initState() {
    super.initState();

    _controller =
        // VideoPlayerController.networkUrl(
        //     Uri.parse(
        //       'https://videos.pexels.com/video-files/35623364/35623364-hd_1920_1080_30fps.mp4',
        //     ),
        //   )
        VideoPlayerController.asset('assets/videos/bg.mp4')
          ..initialize().then((_) {
            setState(() {});
            Future.delayed(Duration(milliseconds: 500), () {
              _controller
                ..setLooping(true)
                ..setVolume(0)
                ..play();
            });
          });
  }

  @override
  void dispose() {
    _controller.dispose();
    super.dispose();
  }

  Future<void> _openLink(String url) async {
    final Uri uri = Uri.parse(url);
    if (!await launchUrl(uri)) {
      throw 'Could not launch $url';
    }
  }

  @override
  Widget build(BuildContext context) {
    final width = MediaQuery.of(context).size.width;
    final height = MediaQuery.of(context).size.height;

    final isDesktop = width > 1000;
    final isTablet = width > 600 && width <= 1000;

    return Scaffold(
      body: Column(
        children: [
          Expanded(
            child: SingleChildScrollView(
              child: Column(
                children: [
                  /// 🔥 HERO
                  SizedBox(
                    height: height,
                    child: Stack(
                      children: [
                        if (_controller.value.isInitialized)
                          SizedBox.expand(
                            child: FittedBox(
                              fit: BoxFit.cover,
                              child: SizedBox(
                                width: _controller.value.size.width,
                                height: _controller.value.size.height,
                                child: VideoPlayer(_controller),
                              ),
                            ),
                          )
                        else
                          Container(color: Colors.black),

                        Container(color: Colors.black.withOpacity(0.6)),

                        Center(
                          child: ConstrainedBox(
                            constraints: const BoxConstraints(maxWidth: 1200),
                            child: AnimatedSection(
                              child: Column(
                                mainAxisAlignment: MainAxisAlignment.center,
                                children: [
                                  Text(
                                    "CobaltDev",
                                    style: TextStyle(
                                      fontSize: isDesktop
                                          ? 60
                                          : isTablet
                                          ? 42
                                          : 32,
                                      fontWeight: FontWeight.bold,
                                    ),
                                  ),
                                  const SizedBox(height: 20),

                                  const Text(
                                    "I build fast, scalable apps with Flutter & Rust",
                                    style: TextStyle(color: Colors.white70),
                                  ),

                                  const SizedBox(height: 20),

                                  GlassCard(
                                    child: Column(
                                      children: [
                                        Text(
                                          "Flutter + Rust Developer",
                                          style: TextStyle(
                                            fontSize: isDesktop ? 24 : 18,
                                          ),
                                        ),
                                        const SizedBox(height: 10),
                                        const Text(
                                          "Production-grade apps & backend systems",
                                          textAlign: TextAlign.center,
                                        ),
                                      ],
                                    ),
                                  ),

                                  const SizedBox(height: 30),

                                  ElevatedButton(
                                    onPressed: () {
                                      Navigator.pushNamed(context, "/services");
                                    },
                                    child: const Text("Get Started"),
                                  ),
                                ],
                              ),
                            ),
                          ),
                        ),
                      ],
                    ),
                  ),

                  /// 🔥 TRUST
                  AnimatedSection(
                    child: Padding(
                      padding: const EdgeInsets.all(40),
                      child: Column(
                        children: const [
                          Text("Trusted By", style: TextStyle(fontSize: 28)),
                          SizedBox(height: 20),
                          Wrap(
                            spacing: 30,
                            alignment: WrapAlignment.center,
                            children: [
                              Text("🚀 10+ Projects Built"),
                              Text("⚡ High Performance Systems"),
                              Text("🦀 Rust Backend Specialist"),
                              Text("📱 Cross Platform Apps"),
                            ],
                          ),
                        ],
                      ),
                    ),
                  ),

                  /// 🔥 FEATURED
                  AnimatedSection(
                    child: Padding(
                      padding: const EdgeInsets.all(40),
                      child: Column(
                        children: [
                          const Text(
                            "Featured Work",
                            style: TextStyle(fontSize: 32),
                          ),
                          const SizedBox(height: 20),

                          Wrap(
                            spacing: 20,
                            children: [
                              _featuredCard(
                                "Encrypted Notepad",
                                "Secure Flutter + Rust backend",
                                "https://github.com/ibrahim-3595/Encrypt-Notepad",
                                _openLink,
                              ),
                              _featuredCard(
                                "Secure Journal",
                                "Rust backend + Dioxus frontend + Axum framework + SQliteDb",
                                "https://github.com/ibrahim-3595/Secure-Journal",
                                _openLink,
                              ),
                            ],
                          ),

                          const SizedBox(height: 20),

                          TextButton(
                            onPressed: () {
                              Navigator.pushNamed(context, "/portfolio");
                            },
                            child: const Text("View All Projects →"),
                          ),
                        ],
                      ),
                    ),
                  ),

                  /// 🔥 WHY ME (WITH HOVER SCALE)
                  AnimatedSection(
                    child: Padding(
                      padding: const EdgeInsets.all(40),
                      child: Column(
                        children: [
                          const Text(
                            "Why Work With Me?",
                            style: TextStyle(fontSize: 32),
                          ),
                          const SizedBox(height: 20),

                          Wrap(
                            spacing: 20,
                            children: [
                              _hoverPoint("⚡ Speed", "Fast delivery"),
                              _hoverPoint("🧠 Deep Tech", "Rust expertise"),
                              _hoverPoint("🎯 Clean UI", "Modern design"),
                              _hoverPoint("📈 Scalable", "Production ready"),
                            ],
                          ),
                        ],
                      ),
                    ),
                  ),

                  /// 🔥 CTA
                  AnimatedSection(
                    child: Padding(
                      padding: const EdgeInsets.all(40),
                      child: Column(
                        children: [
                          const Text(
                            "Have an idea? Let’s build it.",
                            style: TextStyle(fontSize: 28),
                          ),
                          const SizedBox(height: 20),
                          ElevatedButton(
                            onPressed: () {
                              Navigator.pushNamed(context, "/contact");
                            },
                            child: const Text("Contact Me"),
                          ),
                        ],
                      ),
                    ),
                  ),

                  /// 🔥 TECH STACK
                  AnimatedSection(
                    child: Padding(
                      padding: const EdgeInsets.all(40),
                      child: Wrap(
                        spacing: 20,
                        alignment: WrapAlignment.center,
                        children: const [
                          Chip(label: Text("Rust")),
                          Chip(label: Text("Flutter")),
                          Chip(label: Text("Axum")),
                          Chip(label: Text("SQLx")),
                          Chip(label: Text("Docker")),
                        ],
                      ),
                    ),
                  ),

                  /// FOOTER
                  Container(
                    padding: const EdgeInsets.all(20),
                    color: Colors.black,
                    child: const Column(
                      children: [
                        Text("© 2026 CobaltDev"),
                        SizedBox(height: 10),
                        Text("Built with Flutter & Rust"),
                      ],
                    ),
                  ),
                ],
              ),
            ),
          ),
        ],
      ),
    );
  }
}

/// 🔥 FEATURE CARD
Widget _featuredCard(
  String title,
  String desc,
  String url,
  Function(String) openLink,
) {
  return SizedBox(
    width: 300,
    child: GlassCard(
      child: Column(
        crossAxisAlignment: CrossAxisAlignment.start,
        children: [
          Container(
            height: 160,
            decoration: BoxDecoration(
              color: Colors.grey[800],
              borderRadius: BorderRadius.circular(10),
            ),
            child: const Center(child: Text("Preview")),
          ),
          const SizedBox(height: 12),
          Text(
            title,
            style: const TextStyle(fontSize: 20, fontWeight: FontWeight.bold),
          ),
          const SizedBox(height: 6),
          Text(desc),
          const SizedBox(height: 10),
          TextButton(
            onPressed: () => openLink(url),
            child: const Text("View Details →"),
          ),
        ],
      ),
    ),
  );
}

/// 🔥 HOVER POINT
Widget _hoverPoint(String title, String desc) {
  return _HoverCard(title: title, desc: desc);
}

class _HoverCard extends StatefulWidget {
  final String title;
  final String desc;

  const _HoverCard({required this.title, required this.desc});

  @override
  State<_HoverCard> createState() => _HoverCardState();
}

class _HoverCardState extends State<_HoverCard> {
  bool isHovered = false;

  @override
  Widget build(BuildContext context) {
    return MouseRegion(
      onEnter: (_) => setState(() => isHovered = true),
      onExit: (_) => setState(() => isHovered = false),
      child: AnimatedContainer(
        duration: const Duration(milliseconds: 200),
        transform: Matrix4.identity()..scale(isHovered ? 1.08 : 1.0),
        child: SizedBox(
          width: 220,
          child: GlassCard(
            child: Column(
              children: [
                Text(
                  widget.title,
                  style: const TextStyle(
                    fontSize: 18,
                    fontWeight: FontWeight.bold,
                  ),
                ),
                const SizedBox(height: 10),
                Text(widget.desc, textAlign: TextAlign.center),
              ],
            ),
          ),
        ),
      ),
    );
  }
}
