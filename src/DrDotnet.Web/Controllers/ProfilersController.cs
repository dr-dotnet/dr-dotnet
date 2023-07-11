using System;
using System.Collections.Generic;
using Microsoft.AspNetCore.Mvc;

namespace DrDotnet.Web.Controllers;

[ApiController]
[Route("api/[controller]")]
public class ProfilersController : ControllerBase
{
    private readonly IProfilerDiscovery _profilerDiscovery;

    public ProfilersController(IProfilerDiscovery profilerDiscovery)
    {
        ArgumentNullException.ThrowIfNull(profilerDiscovery);
        _profilerDiscovery = profilerDiscovery;
    }

    [HttpGet]
    public ActionResult<IReadOnlyList<SessionInfo>> GetProfilers()
    {
        return Ok(_profilerDiscovery.GetProfilers());
    }
}