import 'dart:ui';
import 'package:flutter/material.dart';

class Navbar extends StatefulWidget {
  const Navbar({super.key});

  @override
  State<Navbar> createState() => _NavbarState();
}

class _NavbarState extends State<Navbar> {
  String currentRoute = '/';

  @override
  void didChangeDependencies() {
    super.didChangeDependencies();
    currentRoute = ModalRoute.of(context)?.settings.name ?? '/';
  }

  @override
  Widget build(BuildContext context) {
    final width = MediaQuery.of(context).size.width;
    final isMobile = width < 900;

    return ClipRRect(
      child: BackdropFilter(
        filter: ImageFilter.blur(sigmaX: 16, sigmaY: 16),
          child: SafeArea(
            bottom: false,
            child: Container(
              padding: EdgeInsets.symmetric(
                horizontal: isMobile ? 24 : 40,
                vertical: 24, // Balanced vertical padding
              ),
              decoration: BoxDecoration(
                color: const Color(0xFF0B091C).withOpacity(0.8),
                border: Border(
                  bottom: BorderSide(color: const Color(0xFFA855F7).withOpacity(0.15)),
                ),
                boxShadow: [
                  BoxShadow(
                    color: Colors.black.withOpacity(0.25),
                    blurRadius: 25,
                    offset: const Offset(0, 8),
                  ),
                ],
              ),
          child: Row(
            mainAxisAlignment: MainAxisAlignment.spaceBetween,
            children: [
              // Gradient Logo
              MouseRegion(
                cursor: SystemMouseCursors.click,
                child: GestureDetector(
                  onTap: () => Navigator.of(context).pushReplacementNamed('/'),
                  child: Row(
                    mainAxisSize: MainAxisSize.min,
                    children: [
                      Image.asset(
                        'assets/images/cobalt_logo.png',
                        height: 28,
                        width: 28,
                      ),
                      const SizedBox(width: 12),
                      ShaderMask(
                        shaderCallback: (bounds) => const LinearGradient(
                          colors: [Color(0xFFC084FC), Color(0xFFA855F7), Color(0xFF818CF8)],
                        ).createShader(bounds),
                        child: const Text(
                          "CobaltDev",
                          style: TextStyle(
                            fontSize: 20,
                            fontWeight: FontWeight.w900,
                            color: Colors.white,
                            letterSpacing: -0.5,
                          ),
                        ),
                      ),
                    ],
                  ),
                ),
              ),

              if (!isMobile)
                Row(
                  children: [
                    NavItem(title: "Home", route: "/", currentRoute: currentRoute),
                    const SizedBox(width: 8),
                    NavItem(title: "Services", route: "/services", currentRoute: currentRoute),
                    const SizedBox(width: 8),
                    NavItem(title: "Products", route: "/products", currentRoute: currentRoute),
                    const SizedBox(width: 8),
                    NavItem(title: "Portfolio", route: "/portfolio", currentRoute: currentRoute),
                    const SizedBox(width: 8),
                    NavItem(title: "About", route: "/about", currentRoute: currentRoute),
                    const SizedBox(width: 8),
                    NavItem(title: "Contact", route: "/contact", currentRoute: currentRoute),
                    const SizedBox(width: 16),
                    // Desktop CTA
                    MouseRegion(
                      cursor: SystemMouseCursors.click,
                      child: GestureDetector(
                        onTap: () => Navigator.of(context).pushReplacementNamed('/contact'),
                        child: Container(
                          padding: const EdgeInsets.symmetric(horizontal: 20, vertical: 10),
                          decoration: BoxDecoration(
                            gradient: const LinearGradient(
                              colors: [Color(0xFF9333EA), Color(0xFF6D28D9)],
                            ),
                            borderRadius: BorderRadius.circular(12),
                            boxShadow: [
                              BoxShadow(
                                color: const Color(0xFF9333EA).withOpacity(0.35),
                                blurRadius: 16,
                              )
                            ],
                          ),
                          child: const Text(
                            "Let's Talk ✦",
                            style: TextStyle(
                              color: Colors.white,
                              fontWeight: FontWeight.w600,
                              fontSize: 14,
                            ),
                          ),
                        ),
                      ),
                    ),
                  ],
                ),

              // Mobile Menu Button
              if (isMobile)
                Builder(
                  builder: (context) => IconButton(
                    icon: const Icon(Icons.menu, color: Colors.white70),
                    onPressed: () => Scaffold.of(context).openDrawer(),
                  ),
                ),
            ],
          ),
        ),
      ),
    ),
  );
  }
}

class NavItem extends StatefulWidget {
  final String title;
  final String route;
  final String currentRoute;

  const NavItem({
    super.key,
    required this.title,
    required this.route,
    required this.currentRoute,
  });

  @override
  State<NavItem> createState() => _NavItemState();
}

class _NavItemState extends State<NavItem> {
  bool isHovered = false;

  @override
  Widget build(BuildContext context) {
    final isActive = widget.currentRoute == widget.route;

    return MouseRegion(
      cursor: SystemMouseCursors.click,
      onEnter: (_) => setState(() => isHovered = true),
      onExit: (_) => setState(() => isHovered = false),
      child: GestureDetector(
        onTap: () {
          if (!isActive) {
            Navigator.of(context).pushReplacementNamed(widget.route);
          }
        },
        child: Container(
          padding: const EdgeInsets.symmetric(horizontal: 16, vertical: 8),
          child: AnimatedDefaultTextStyle(
            duration: const Duration(milliseconds: 200),
            style: TextStyle(
              fontSize: 15,
              fontFamily: 'Inter',
              fontWeight: isActive ? FontWeight.w600 : FontWeight.w500,
              color: isActive || isHovered ? const Color(0xFFE9D5FF) : const Color(0xFF94A3B8), // purple-200 / slate-400
            ),
            child: Text(widget.title),
          ),
        ),
      ),
    );
  }
}
