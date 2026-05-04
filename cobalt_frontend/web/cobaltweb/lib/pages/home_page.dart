import 'dart:ui';
import 'dart:async';
import 'package:flutter/material.dart';
import 'package:url_launcher/url_launcher.dart';
import '../widgets/animated_section.dart';
import '../widgets/glass_card.dart';

class HomePage extends StatefulWidget {
  const HomePage({super.key});

  @override
  State<HomePage> createState() => _HomePageState();
}

class _HomePageState extends State<HomePage> {
  Future<void> _openLink(String url) async {
    final uri = Uri.parse(url);
    if (!await launchUrl(uri, mode: LaunchMode.externalApplication)) {
      throw 'Could not launch $url';
    }
  }

  @override
  Widget build(BuildContext context) {
    final width = MediaQuery.of(context).size.width;
    final isDesktop = width > 1100;
    final isTablet = width > 700 && width <= 1100;
    final isMobile = width <= 700;

    return Container(
      decoration: const BoxDecoration(
        gradient: LinearGradient(
          begin: Alignment.topLeft,
          end: Alignment.bottomRight,
          colors: [
            Color(0xFF07070F),
            Color(0xFF0F0B1E),
            Color(0xFF150D2E),
            Color(0xFF1A0B33)
          ],
          stops: [0.0, 0.4, 0.7, 1.0],
        ),
      ),
      child: Stack(
        children: [
          // Ambient Blobs
          Positioned(
            top: -200,
            left: -100,
            child: Container(
              width: width * 0.5,
              height: width * 0.5,
              decoration: BoxDecoration(
                shape: BoxShape.circle,
                gradient: RadialGradient(
                  colors: [
                    const Color(0xFF8B5CF6).withOpacity(0.18),
                    Colors.transparent
                  ],
                  stops: const [0.0, 0.7],
                ),
              ),
              child: BackdropFilter(
                filter: ImageFilter.blur(sigmaX: 70, sigmaY: 70),
                child: Container(color: Colors.transparent),
              ),
            ),
          ),
          Positioned(
            bottom: -200,
            right: -100,
            child: Container(
              width: width * 0.55,
              height: width * 0.55,
              decoration: BoxDecoration(
                shape: BoxShape.circle,
                gradient: RadialGradient(
                  colors: [
                    const Color(0xFFA855F7).withOpacity(0.22),
                    const Color(0xFFA855F7).withOpacity(0.1),
                    Colors.transparent
                  ],
                  stops: const [0.0, 0.4, 0.7],
                ),
              ),
              child: BackdropFilter(
                filter: ImageFilter.blur(sigmaX: 90, sigmaY: 90),
                child: Container(color: Colors.transparent),
              ),
            ),
          ),

          // Main Content
          SingleChildScrollView(
            child: Column(
              children: [
                // HERO SECTION
                SizedBox(
                  height: MediaQuery.of(context).size.height * 0.9,
                  child: Center(
                    child: ConstrainedBox(
                      constraints: const BoxConstraints(maxWidth: 1200),
                      child: Padding(
                        padding: EdgeInsets.symmetric(horizontal: isMobile ? 24 : 48),
                        child: AnimatedSection(
                          child: Column(
                            mainAxisAlignment: MainAxisAlignment.center,
                            crossAxisAlignment: CrossAxisAlignment.start,
                            children: [
                              // Available Badge
                              Container(
                                padding: const EdgeInsets.symmetric(horizontal: 16, vertical: 6),
                                margin: const EdgeInsets.only(bottom: 16),
                                decoration: BoxDecoration(
                                  color: const Color(0xFF8B5CF6).withOpacity(0.08),
                                  borderRadius: BorderRadius.circular(30),
                                  border: Border.all(color: const Color(0xFF8B5CF6).withOpacity(0.4)),
                                ),
                                child: Row(
                                  mainAxisSize: MainAxisSize.min,
                                  children: [
                                    _PulsingDot(),
                                    const SizedBox(width: 8),
                                    const Text(
                                      "AVAILABLE FOR WORK",
                                      style: TextStyle(
                                        color: Color(0xFFC084FC),
                                        fontSize: 11,
                                        fontWeight: FontWeight.bold,
                                        letterSpacing: 1.2,
                                      ),
                                    ),
                                  ],
                                ),
                              ),

                              ShaderMask(
                                shaderCallback: (bounds) => const LinearGradient(
                                  colors: [Color(0xFFC084FC), Color(0xFFA855F7), Color(0xFF7C3AED), Color(0xFF818CF8)],
                                  begin: Alignment.topLeft,
                                  end: Alignment.bottomRight,
                                ).createShader(bounds),
                                child: Text(
                                  "Ibrahim Haji",
                                  style: TextStyle(
                                    fontSize: isDesktop ? 64 : isTablet ? 48 : 40,
                                    fontWeight: FontWeight.w900,
                                    letterSpacing: -1,
                                    height: 1.1,
                                    color: Colors.white, // Required for ShaderMask
                                  ),
                                ),
                              ),
                              const SizedBox(height: 12),
                              
                              // Typing Text
                              _TypingSubtitle(
                                titles: const [
                                  "Flutter + Rust Developer",
                                  "Creative Problem Solver",
                                  "UI/UX Enthusiast",
                                  "Systems Engineer"
                                ],
                                fontSize: isDesktop ? 28 : 22,
                              ),
                              const SizedBox(height: 24),

                              const Text(
                                "Building production-grade mobile apps, high-performance backends, and private cloud infrastructure with Flutter & Rust.",
                                style: TextStyle(
                                  fontSize: 18,
                                  color: Color(0xFF64748B), // Slate 500
                                  height: 1.6,
                                ),
                              ),

                              const SizedBox(height: 48),

                              Wrap(
                                spacing: 16,
                                runSpacing: 16,
                                children: [
                                  // Primary Gradient Button
                                  MouseRegion(
                                    cursor: SystemMouseCursors.click,
                                    child: GestureDetector(
                                      onTap: () => Navigator.pushReplacementNamed(context, '/portfolio'),
                                      child: Container(
                                        padding: const EdgeInsets.symmetric(horizontal: 32, vertical: 20),
                                        decoration: BoxDecoration(
                                          gradient: const LinearGradient(
                                            colors: [Color(0xFF9333EA), Color(0xFF7C3AED)],
                                            begin: Alignment.topLeft,
                                            end: Alignment.bottomRight,
                                          ),
                                          borderRadius: BorderRadius.circular(12),
                                          boxShadow: [
                                            BoxShadow(
                                              color: const Color(0xFF9333EA).withOpacity(0.42),
                                              blurRadius: 22,
                                              offset: const Offset(0, 4),
                                            ),
                                          ],
                                        ),
                                        child: const Text(
                                          "View Projects",
                                          style: TextStyle(fontSize: 16, fontWeight: FontWeight.w600, color: Colors.white),
                                        ),
                                      ),
                                    ),
                                  ),
                                  // Outline Button
                                  MouseRegion(
                                    cursor: SystemMouseCursors.click,
                                    child: GestureDetector(
                                      onTap: () => Navigator.pushReplacementNamed(context, '/contact'),
                                      child: Container(
                                        padding: const EdgeInsets.symmetric(horizontal: 32, vertical: 20),
                                        decoration: BoxDecoration(
                                          color: const Color(0xFF8B5CF6).withOpacity(0.07),
                                          border: Border.all(color: const Color(0xFFA855F7).withOpacity(0.4)),
                                          borderRadius: BorderRadius.circular(12),
                                        ),
                                        child: const Text(
                                          "Contact Me",
                                          style: TextStyle(fontSize: 16, fontWeight: FontWeight.w600, color: Color(0xFFC084FC)),
                                        ),
                                      ),
                                    ),
                                  ),
                                ],
                              ),
                            ],
                          ),
                        ),
                      ),
                    ),
                  ),
                ),

                // FEATURED WORK
                Container(
                  width: double.infinity,
                  padding: EdgeInsets.symmetric(horizontal: isMobile ? 24 : 48, vertical: isMobile ? 60 : 100),
                  // Transparent so stack gradient shows
                  child: AnimatedSection(
                    child: Center(
                      child: ConstrainedBox(
                        constraints: const BoxConstraints(maxWidth: 1200),
                        child: Column(
                          crossAxisAlignment: CrossAxisAlignment.start,
                          children: [
                            // Section Heading
                            Row(
                              children: [
                                Container(
                                  width: 6,
                                  height: 6,
                                  decoration: const BoxDecoration(
                                    color: Color(0xFFA855F7),
                                    shape: BoxShape.circle,
                                    boxShadow: [BoxShadow(color: Color(0xFFA855F7), blurRadius: 6)],
                                  ),
                                ),
                                const SizedBox(width: 8),
                                const Text(
                                  "PORTFOLIO",
                                  style: TextStyle(fontSize: 12, fontWeight: FontWeight.bold, letterSpacing: 1.5, color: Color(0xFFC084FC)),
                                ),
                              ],
                            ),
                            const SizedBox(height: 12),
                            RichText(
                              text: const TextSpan(
                                text: "Featured ",
                                style: TextStyle(fontSize: 40, fontWeight: FontWeight.w900, color: Colors.white, fontFamily: 'Inter'),
                                children: [
                                  TextSpan(
                                    text: "Projects",
                                    style: TextStyle(color: Color(0xFFA855F7)), // Solid purple for simplicity without shader mask inside rich text
                                  ),
                                ],
                              ),
                            ),
                            const SizedBox(height: 16),
                            const Text(
                              "Real stuff I've built with passion and code.",
                              style: TextStyle(fontSize: 18, color: Color(0xFF64748B)),
                            ),
                            const SizedBox(height: 60),

                            Wrap(
                              spacing: 24,
                              runSpacing: 24,
                              children: [
                                _projectCard(context,
                                  "Secure Journal",
                                  "CLI + Dioxus + Axum + SQLx",
                                  "A private journaling app with end-to-end encryption and Rust backend.",
                                  "https://github.com/Cobalt-Labs/cobalt_journal",
                                  "01",
                                  "Private App"
                                ),
                                _projectCard(context,
                                  "Cobalt Cloud",
                                  "Rust Backend + Dioxus Frontend",
                                  "Self-hosted private cloud running on my laptop HDD.",
                                  "https://github.com/Cobalt-Labs/cobaltdev/tree/main/cobalt_cloud",
                                  "02",
                                  "Infrastructure"
                                ),
                                _projectCard(context,
                                  "Encrypt Notepad",
                                  "Hybrid Mobile + Desktop",
                                  "Production apps using Flutter frontend + Rust core via FFI.",
                                  "https://github.com/ibrahim-3595/Encrypt-Notepad",
                                  "03",
                                  "Production App"
                                ),
                              ],
                            ),
                          ],
                        ),
                      ),
                    ),
                  ),
                ),

                // FOOTER
                Container(
                  width: double.infinity,
                  padding: const EdgeInsets.all(60),
                  color: const Color(0xFF07070F).withOpacity(0.5), // Semi-transparent over gradient
                  child: const Center(
                    child: Text(
                      "© 2026 CobaltDev",
                      style: TextStyle(color: Color(0xFF64748B), fontSize: 14),
                    ),
                  ),
                ),
              ],
            ),
          ),
        ],
      ),
    );
  }

  Widget _projectCard(BuildContext context, String title, String subtitle, String desc, String url, String number, String highlight) {
    final sw = MediaQuery.of(context).size.width;
    return SizedBox(
      width: sw < 450 ? (sw - 48).clamp(0.0, double.infinity).toDouble() : 360.0,
      child: GlassCard(
        onTap: () => _openLink(url),
        padding: const EdgeInsets.all(32),
        child: Column(
          crossAxisAlignment: CrossAxisAlignment.start,
          children: [
            // Top row: Number and Highlight
            Row(
              children: [
                Text(number, style: TextStyle(fontSize: 12, fontWeight: FontWeight.w900, color: const Color(0xFFA855F7).withOpacity(0.45), letterSpacing: 2)),
                const Expanded(child: Divider(color: Colors.white10, indent: 12, endIndent: 12)),
                Container(
                  padding: const EdgeInsets.symmetric(horizontal: 12, vertical: 4),
                  decoration: BoxDecoration(
                    color: const Color(0xFFA855F7).withOpacity(0.1),
                    border: Border.all(color: const Color(0xFFA855F7).withOpacity(0.25)),
                    borderRadius: BorderRadius.circular(20),
                  ),
                  child: Text(highlight.toUpperCase(), style: const TextStyle(fontSize: 10, fontWeight: FontWeight.bold, color: Color(0xFFC084FC))),
                ),
              ],
            ),
            const SizedBox(height: 24),
            Text(title, style: const TextStyle(fontSize: 24, fontWeight: FontWeight.w900, color: Colors.white, height: 1.2)),
            const SizedBox(height: 8),
            Text(desc, style: const TextStyle(color: Color(0xFF64748B), height: 1.6, fontSize: 15)),
            const SizedBox(height: 24),
            // Tech tags
            Wrap(
              spacing: 8,
              runSpacing: 8,
              children: subtitle.split(' + ').map((tech) => Container(
                padding: const EdgeInsets.symmetric(horizontal: 10, vertical: 4),
                decoration: BoxDecoration(
                  color: Colors.white.withOpacity(0.05),
                  border: Border.all(color: Colors.white10),
                  borderRadius: BorderRadius.circular(8),
                ),
                child: Text(tech, style: const TextStyle(fontSize: 11, color: Color(0xFF94A3B8))),
              )).toList(),
            ),
            const SizedBox(height: 32),
            Container(
              padding: const EdgeInsets.symmetric(horizontal: 20, vertical: 10),
              decoration: BoxDecoration(
                gradient: const LinearGradient(colors: [Color(0xFF9333EA), Color(0xFF6D28D9)]),
                borderRadius: BorderRadius.circular(12),
                boxShadow: [BoxShadow(color: const Color(0xFF9333EA).withOpacity(0.3), blurRadius: 16)],
              ),
              child: const Row(
                mainAxisSize: MainAxisSize.min,
                children: [
                  Text("View Project", style: TextStyle(color: Colors.white, fontSize: 14, fontWeight: FontWeight.w600)),
                  SizedBox(width: 8),
                  Icon(Icons.arrow_forward, color: Colors.white, size: 16),
                ],
              ),
            ),
          ],
        ),
      ),
    );
  }
}

class _PulsingDot extends StatefulWidget {
  @override
  _PulsingDotState createState() => _PulsingDotState();
}

class _PulsingDotState extends State<_PulsingDot> with SingleTickerProviderStateMixin {
  late AnimationController _controller;

  @override
  void initState() {
    super.initState();
    _controller = AnimationController(vsync: this, duration: const Duration(seconds: 2))..repeat(reverse: true);
  }
  @override
  void dispose() {
    _controller.dispose();
    super.dispose();
  }

  @override
  Widget build(BuildContext context) {
    return AnimatedBuilder(
      animation: _controller,
      builder: (context, child) {
        return Container(
          width: 8,
          height: 8,
          decoration: BoxDecoration(
            color: const Color(0xFFA855F7),
            shape: BoxShape.circle,
            boxShadow: [
              BoxShadow(
                color: const Color(0xFFA855F7),
                blurRadius: 6 + (10 * _controller.value),
                spreadRadius: 2 * _controller.value,
              ),
            ],
          ),
        );
      },
    );
  }
}

class _TypingSubtitle extends StatefulWidget {
  final List<String> titles;
  final double fontSize;

  const _TypingSubtitle({required this.titles, required this.fontSize});

  @override
  _TypingSubtitleState createState() => _TypingSubtitleState();
}

class _TypingSubtitleState extends State<_TypingSubtitle> {
  int _titleIndex = 0;
  String _displayed = "";
  bool _isDeleting = false;
  Timer? _timer;

  @override
  void initState() {
    super.initState();
    _startTyping();
  }

  @override
  void dispose() {
    _timer?.cancel();
    super.dispose();
  }

  void _startTyping() {
    final current = widget.titles[_titleIndex];
    final speed = _isDeleting ? 40 : 80;

    _timer = Timer(Duration(milliseconds: speed), () {
      if (!mounted) return;

      setState(() {
        if (!_isDeleting && _displayed.length < current.length) {
          _displayed = current.substring(0, _displayed.length + 1);
          _startTyping();
        } else if (!_isDeleting && _displayed.length == current.length) {
          _timer = Timer(const Duration(milliseconds: 1800), () {
            if (mounted) {
              setState(() => _isDeleting = true);
              _startTyping();
            }
          });
        } else if (_isDeleting && _displayed.isNotEmpty) {
          _displayed = current.substring(0, _displayed.length - 1);
          _startTyping();
        } else if (_isDeleting && _displayed.isEmpty) {
          _isDeleting = false;
          _titleIndex = (_titleIndex + 1) % widget.titles.length;
          _startTyping();
        }
      });
    });
  }

  @override
  Widget build(BuildContext context) {
    return Row(
      mainAxisSize: MainAxisSize.min,
      children: [
        Text(
          _displayed,
          style: TextStyle(
            fontSize: widget.fontSize,
            color: const Color(0xFFC084FC),
            fontWeight: FontWeight.w600,
          ),
        ),
        _BlinkingCursor(fontSize: widget.fontSize),
      ],
    );
  }
}

class _BlinkingCursor extends StatefulWidget {
  final double fontSize;
  const _BlinkingCursor({required this.fontSize});

  @override
  _BlinkingCursorState createState() => _BlinkingCursorState();
}

class _BlinkingCursorState extends State<_BlinkingCursor> with SingleTickerProviderStateMixin {
  late AnimationController _controller;

  @override
  void initState() {
    super.initState();
    _controller = AnimationController(vsync: this, duration: const Duration(milliseconds: 500))..repeat(reverse: true);
  }
  @override
  void dispose() {
    _controller.dispose();
    super.dispose();
  }

  @override
  Widget build(BuildContext context) {
    return FadeTransition(
      opacity: _controller,
      child: Container(
        margin: const EdgeInsets.only(left: 4),
        width: 3,
        height: widget.fontSize * 0.9,
        color: const Color(0xFFA855F7),
      ),
    );
  }
}