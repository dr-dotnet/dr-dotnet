using System.Net.Http;
using System.Windows;
using DrDotnet;
using MatBlazor;
using Microsoft.Extensions.DependencyInjection;

namespace DrDotnet.Desktop
{
    /// <summary>
    /// Interaction logic for MainWindow.xaml
    /// </summary>
    public partial class MainWindow : Window
    {
        public MainWindow()
        {
            var services = new ServiceCollection();
            services.AddBlazorWebView();
            services.AddMatBlazor();

            services.AddSingleton<HttpClient>();
            services.AddSingleton<ILogger, Logger>();
            services.AddSingleton<ISessionDiscovery, SessionDiscovery>();
            services.AddSingleton<IProcessDiscovery, ProcessDiscovery>();
            services.AddSingleton<IProfilerDiscovery, ProfilesDiscovery>();

            Resources.Add("services", services.BuildServiceProvider());

            InitializeComponent();
        }
    }

    // Workaround for compiler error "error MC3050: Cannot find the type 'local:Main'"
    // It seems that, although WPF's design-time build can see Razor components, its runtime build cannot.
    public partial class Main { }
}
