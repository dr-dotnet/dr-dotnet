using System.IO;
using Microsoft.AspNetCore.Builder;
using Microsoft.AspNetCore.Hosting;
using Microsoft.Extensions.DependencyInjection;
using Microsoft.Extensions.Hosting;
using System.Net.Http;
using DrDotnet.Logging;
using DrDotnet.Utils;
using Microsoft.Extensions.Logging;
using MudBlazor.Services;

namespace DrDotnet.Web;

public class Startup
{
    // This method gets called by the runtime. Use this method to add services to the container.
    // For more information on how to configure your application, visit https://go.microsoft.com/fwlink/?LinkID=398940
    public void ConfigureServices(IServiceCollection services)
    {
        services.AddRazorPages();
        services.AddServerSideBlazor();
        services.AddMudServices();

        services.AddSingleton<HttpClient>();

        services.AddLogging(lb => lb
            .AddSimpleConsole()
            .AddFileLogger(Path.Combine(PathUtils.DrDotnetBaseDirectory, "app.debug.log")));
        
        services.AddSingleton<ISessionDiscovery, SessionsDiscovery>();
        services.AddSingleton<IProcessDiscovery, ProcessDiscovery>();
        services.AddSingleton<IProfilerDiscovery, ProfilersDiscovery>();
    }

    // This method gets called by the runtime. Use this method to configure the HTTP request pipeline.
    public void Configure(IApplicationBuilder app, IWebHostEnvironment env)
    {
        if (env.IsDevelopment())
        {
            app.UseDeveloperExceptionPage();
        }
        else
        {
            app.UseExceptionHandler("/Error");
            // The default HSTS value is 30 days. You may want to change this for production scenarios, see https://aka.ms/aspnetcore-hsts.
            app.UseHsts();
        }

        //app.UseHttpsRedirection();
        app.UseStaticFiles();

        app.UseRouting();

        app.UseEndpoints(endpoints =>
        {
            endpoints.MapBlazorHub();
            endpoints.MapFallbackToPage("/_Host");
        });
    }
}