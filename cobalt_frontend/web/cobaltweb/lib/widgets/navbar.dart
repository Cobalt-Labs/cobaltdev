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

  Widget navItem(String title, String route) {
    final isActive = currentRoute == route;

    return MouseRegion(
      cursor: SystemMouseCursors.click,
      child: GestureDetector(
        onTap: () {
          if (!isActive) {
            setState(() => currentRoute = route);
            Navigator.pushReplacementNamed(context, route);
          }
        },
        child: AnimatedContainer(
          duration: const Duration(milliseconds: 150),
          padding: const EdgeInsets.symmetric(horizontal: 16, vertical: 8),
          decoration: BoxDecoration(
            borderRadius: BorderRadius.circular(8),
            color: isActive ? Colors.white.withOpacity(0.08) : Colors.transparent,
          ),
          child: Text(
            title,
            style: TextStyle(
              fontSize: 14,
              fontWeight: isActive ? FontWeight.w500 : FontWeight.w400,
              color: isActive ? Colors.white : Colors.white70,
            ),
          ),
        ),
      ),
    );
  }

  @override
  Widget build(BuildContext context) {
    final width = MediaQuery.of(context).size.width;
    final isMobile = width < 900;

    return Container(
      padding: EdgeInsets.symmetric(
        horizontal: isMobile ? 24 : 40,
        vertical: 16,
      ),
      decoration: const BoxDecoration(
        color: Color(0xFF18181B), // zinc-900
        border: Border(bottom: BorderSide(color: Color(0xFF27272A))), // border-zinc-800
      ),
      child: Row(
        mainAxisAlignment: MainAxisAlignment.spaceBetween,
        children: [
          // Wordmark logo without emojis
          MouseRegion(
            cursor: SystemMouseCursors.click,
            child: GestureDetector(
              onTap: () => Navigator.pushReplacementNamed(context, '/'),
              child: const Text(
                "Cobalt Cloud",
                style: TextStyle(
                  fontSize: 16,
                  fontWeight: FontWeight.w500,
                  color: Colors.white,
                  letterSpacing: -0.3,
                ),
              ),
            ),
          ),

          if (!isMobile)
            Row(
              children: [
                navItem("Home", "/"),
                const SizedBox(width: 4),
                navItem("Services", "/services"),
                const SizedBox(width: 4),
                navItem("Products", "/products"),
                const SizedBox(width: 4),
                navItem("Cloud", "/cloud"),
                const SizedBox(width: 4),
                navItem("Portfolio", "/portfolio"),
                const SizedBox(width: 4),
                navItem("About", "/about"),
                const SizedBox(width: 4),
                navItem("Contact", "/contact"),
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
    );
  }
}
