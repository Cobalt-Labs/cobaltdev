import 'package:flutter/material.dart';
import 'pages/home_page.dart';
import 'pages/service_page.dart';
import 'pages/product_page.dart';
import 'pages/portfolio_page.dart';
import 'pages/about_page.dart';
import 'pages/contact_page.dart';
import 'pages/cloud_page.dart';
import 'widgets/navbar.dart';

void main() {
  runApp(const CobaltDevApp());
}

class CobaltDevApp extends StatelessWidget {
  const CobaltDevApp({super.key});

  @override
  Widget build(BuildContext context) {
    return MaterialApp(
      debugShowCheckedModeBanner: false,
      title: 'CobaltDev',
      theme: ThemeData(
        brightness: Brightness.dark,
        primaryColor: const Color(0xFFA855F7), // Purple 500
        scaffoldBackgroundColor: const Color(0xFF07070F), // Dark slate/purple
        fontFamily: 'Inter',
        textTheme: const TextTheme(
          headlineLarge: TextStyle(fontSize: 48, fontWeight: FontWeight.bold, color: Colors.white, letterSpacing: -0.5),
          headlineMedium: TextStyle(fontSize: 36, fontWeight: FontWeight.bold, color: Colors.white, letterSpacing: -0.5),
          bodyLarge: TextStyle(fontSize: 16, color: Colors.white70),
        ),
        fontFamilyFallback: const ['Noto Color Emoji'],
      ),
      initialRoute: '/',
      onGenerateRoute: (settings) {
        Widget page;
        switch (settings.name) {
          case '/': page = const HomePage(); break;
          case '/about': page = const AboutPage(); break;
          case '/services': page = const ServicesPage(); break;
          case '/products': page = const ProductsPage(); break;
          case '/cloud': page = const CloudPage(); break;
          case '/portfolio': page = const PortfolioPage(); break;
          case '/contact': page = const ContactPage(); break;
          default: page = const HomePage(); break;
        }
        
        return PageRouteBuilder(
          settings: settings,
          pageBuilder: (_, __, ___) => MainLayout(child: page),
          transitionDuration: Duration.zero,
          reverseTransitionDuration: Duration.zero,
        );
      },
    );
  }
}

class MainLayout extends StatelessWidget {
  final Widget child;
  const MainLayout({super.key, required this.child});

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      drawer: Drawer(
        backgroundColor: const Color(0xFF0B091C), // Deep purple-black
        child: ListView(
          padding: EdgeInsets.zero,
          children: [
            const DrawerHeader(
              decoration: BoxDecoration(
                border: Border(bottom: BorderSide(color: Color(0x338B5CF6))),
              ),
              child: Align(
                alignment: Alignment.centerLeft,
                child: Text("Cobalt Cloud", style: TextStyle(fontSize: 20, fontWeight: FontWeight.w600, color: Colors.white)),
              ),
            ),
            _drawerItem(context, "Home", "/"),
            _drawerItem(context, "Services", "/services"),
            _drawerItem(context, "Products", "/products"),
            _drawerItem(context, "Cloud", "/cloud"),
            _drawerItem(context, "Portfolio", "/portfolio"),
            _drawerItem(context, "About", "/about"),
            _drawerItem(context, "Contact", "/contact"),
          ],
        ),
      ),
      body: Column(
        children: [
          const Navbar(),
          Expanded(child: child),
        ],
      ),
    );
  }

  Widget _drawerItem(BuildContext context, String title, String route) {
    final currentRoute = ModalRoute.of(context)?.settings.name ?? '/';
    final isActive = currentRoute == route;

    return Padding(
      padding: const EdgeInsets.symmetric(horizontal: 12.0, vertical: 2.0),
      child: ListTile(
        shape: RoundedRectangleBorder(borderRadius: BorderRadius.circular(8)),
        tileColor: isActive ? const Color(0xFFA855F7).withOpacity(0.15) : Colors.transparent,
        title: Text(
          title,
          style: TextStyle(
            color: isActive ? Colors.white : Colors.white70,
            fontWeight: isActive ? FontWeight.w600 : FontWeight.w400,
            fontSize: 15,
          ),
        ),
        onTap: () {
          Navigator.pop(context);
          if (!isActive) Navigator.pushReplacementNamed(context, route);
        },
      ),
    );
  }
}