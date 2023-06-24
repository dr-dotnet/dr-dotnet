using System;
using System.Collections.Generic;
using System.IO;
using System.IO.Compression;
using System.Linq;
using DrDotnet.Utils;
using Microsoft.AspNetCore.Mvc;
using Microsoft.Extensions.Logging;

namespace DrDotnet.Web.Controllers;

[ApiController]
[Route("api/[controller]")]
public class SessionsController : ControllerBase
{
    private readonly ISessionDiscovery _sessionDiscovery;
    private readonly IProfilerDiscovery _profilerDiscovery;
    private readonly IProcessDiscovery _processDiscovery;
    private readonly ILogger<SessionsController> _logger;

    public SessionsController(
        ISessionDiscovery sessionDiscovery,
        IProfilerDiscovery profilerDiscovery,
        IProcessDiscovery processDiscovery,
        ILogger<SessionsController> logger)
    {
        ArgumentNullException.ThrowIfNull(sessionDiscovery);
        ArgumentNullException.ThrowIfNull(profilerDiscovery);
        ArgumentNullException.ThrowIfNull(processDiscovery);
        ArgumentNullException.ThrowIfNull(logger);
        _sessionDiscovery = sessionDiscovery;
        _profilerDiscovery = profilerDiscovery;
        _processDiscovery = processDiscovery;
        _logger = logger;
    }

    [HttpGet]
    public ActionResult<IReadOnlyList<SessionInfo>> GetSessions()
    {
        return Ok(_sessionDiscovery.GetSessions());
    }
    
    [HttpGet("{sessionGuid}")]
    public ActionResult<IReadOnlyList<SessionInfo>> GetSession(Guid sessionGuid)
    {
        if (!_sessionDiscovery.TryGetSession(sessionGuid, out SessionInfo? sessionInfo))
        {
            return NotFound($"No session found with guid '{sessionGuid}'");
        }

        return Ok(sessionInfo);
    }
    
    [HttpGet("{sessionGuid}/download")]
    public ActionResult<IReadOnlyList<SessionInfo>> DownloadSessionResults(Guid sessionGuid)
    {
        if (!_sessionDiscovery.TryGetSession(sessionGuid, out SessionInfo? sessionInfo))
        {
            return NotFound($"No session found with guid '{sessionGuid}'");
        }
        
        var memoryStream = new MemoryStream();
        using (var archive = new ZipArchive(memoryStream, ZipArchiveMode.Create, leaveOpen: true))
        {
            foreach (var file in sessionInfo.EnumerateReports())
            {
                archive.CreateEntryFromFile(file.FullName, file.Name);
            }
        }

        memoryStream.Position = 0;

        return File(memoryStream, "application/zip", $"session-{sessionGuid}.zip");
    }
    
    [HttpPost]
    public ActionResult<SessionInfo> CreateSession([FromBody] CreateSessionDto createSessionDto)
    {
        var profilers = _profilerDiscovery.GetProfilers().Where(x => x.Guid == createSessionDto.ProfilerGuid).ToArray();
        if (profilers.Length == 0)
        {
            return NotFound($"No profiler found with guid '{createSessionDto.ProfilerGuid}'");
        }
        if (!_processDiscovery.TryGetProcessInfoFromPid(createSessionDto.ProcessId, out ProcessInfo? processInfo))
        {
            return NotFound($"No process found with pid '{createSessionDto.ProcessId}'");
        }

        ProfilerInfo profiler = profilers[0];

        foreach (var parameter in createSessionDto.Parameters)
        {
            var matchingParameters = profiler.Parameters.Where(x => x.Key == parameter.Key).ToArray();
            if (matchingParameters.Length == 0)
            {
                return NotFound($"No parameter found with key '{parameter.Key}'");
            }
            var matchingParameter = matchingParameters[0];
            // Don't bother checking which value type it is, just set them all
            matchingParameter.ValueBoolean = parameter.ValueBoolean;
            matchingParameter.ValueInt32 = parameter.ValueInt32;
            matchingParameter.ValueFloat32 = parameter.ValueFloat32;
            matchingParameter.Value = parameter.Value;
        }

        return Ok(ProfilingExtensions.StartProfilingSession(profilers[0], processInfo, _logger));
    }

    public record CreateSessionDto
    {
        public Guid ProfilerGuid { get; init; }
        public int ProcessId { get; init; }
        public ProfilerParameter[] Parameters { get; init; } = Array.Empty<ProfilerParameter>();
    }
}