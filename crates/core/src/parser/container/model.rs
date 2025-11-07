/// Error types.
pub mod error {
    /// Error from a `TryFrom` or `FromStr` implementation.
    pub struct ConversionError(::std::borrow::Cow<'static, str>);
    impl ::std::error::Error for ConversionError {}
    impl ::std::fmt::Display for ConversionError {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> Result<(), ::std::fmt::Error> {
            ::std::fmt::Display::fmt(&self.0, f)
        }
    }
    impl ::std::fmt::Debug for ConversionError {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> Result<(), ::std::fmt::Error> {
            ::std::fmt::Debug::fmt(&self.0, f)
        }
    }
    impl From<&'static str> for ConversionError {
        fn from(value: &'static str) -> Self {
            Self(value.into())
        }
    }
    impl From<String> for ConversionError {
        fn from(value: String) -> Self {
            Self(value.into())
        }
    }
}
///Synk Container JSON Schema
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "title": "Snyk Container",
///  "description": "Synk Container JSON Schema",
///  "type": "object",
///  "required": [
///    "org",
///    "projectName",
///    "uniqueCount"
///  ],
///  "properties": {
///    "applications": {
///      "type": "array",
///      "items": {
///        "type": "object",
///        "properties": {
///          "dependencyCount": {
///            "type": "number"
///          },
///          "displayTargetFile": {
///            "type": "string"
///          },
///          "docker": {
///            "type": "object"
///          },
///          "filesystemPolicy": {
///            "type": "boolean"
///          },
///          "filtered": {
///            "type": "object",
///            "properties": {
///              "ignore": {
///                "type": "array"
///              },
///              "patch": {
///                "type": "array"
///              }
///            }
///          },
///          "hasUnknownVersions": {
///            "type": "boolean"
///          },
///          "ignoreSettings": {
///            "type": "object",
///            "properties": {
///              "adminOnly": {
///                "type": "boolean"
///              },
///              "disregardFilesystemIgnores": {
///                "type": "boolean"
///              },
///              "reasonRequired": {
///                "type": "boolean"
///              }
///            }
///          },
///          "isPrivate": {
///            "type": "boolean"
///          },
///          "licensesPolicy": {
///            "type": "object",
///            "properties": {
///              "orgLicenseRules": {
///                "type": "object",
///                "properties": {
///                  "AGPL-1.0": {
///                    "type": "object",
///                    "properties": {
///                      "instructions": {
///                        "type": "string"
///                      },
///                      "licenseType": {
///                        "type": "string"
///                      },
///                      "severity": {
///                        "type": "string"
///                      }
///                    }
///                  },
///                  "AGPL-3.0": {
///                    "type": "object",
///                    "properties": {
///                      "instructions": {
///                        "type": "string"
///                      },
///                      "licenseType": {
///                        "type": "string"
///                      },
///                      "severity": {
///                        "type": "string"
///                      }
///                    }
///                  },
///                  "Artistic-1.0": {
///                    "type": "object",
///                    "properties": {
///                      "instructions": {
///                        "type": "string"
///                      },
///                      "licenseType": {
///                        "type": "string"
///                      },
///                      "severity": {
///                        "type": "string"
///                      }
///                    }
///                  },
///                  "Artistic-2.0": {
///                    "type": "object",
///                    "properties": {
///                      "instructions": {
///                        "type": "string"
///                      },
///                      "licenseType": {
///                        "type": "string"
///                      },
///                      "severity": {
///                        "type": "string"
///                      }
///                    }
///                  },
///                  "CDDL-1.0": {
///                    "type": "object",
///                    "properties": {
///                      "instructions": {
///                        "type": "string"
///                      },
///                      "licenseType": {
///                        "type": "string"
///                      },
///                      "severity": {
///                        "type": "string"
///                      }
///                    }
///                  },
///                  "CPOL-1.02": {
///                    "type": "object",
///                    "properties": {
///                      "instructions": {
///                        "type": "string"
///                      },
///                      "licenseType": {
///                        "type": "string"
///                      },
///                      "severity": {
///                        "type": "string"
///                      }
///                    }
///                  },
///                  "EPL-1.0": {
///                    "type": "object",
///                    "properties": {
///                      "instructions": {
///                        "type": "string"
///                      },
///                      "licenseType": {
///                        "type": "string"
///                      },
///                      "severity": {
///                        "type": "string"
///                      }
///                    }
///                  },
///                  "GPL-2.0": {
///                    "type": "object",
///                    "properties": {
///                      "instructions": {
///                        "type": "string"
///                      },
///                      "licenseType": {
///                        "type": "string"
///                      },
///                      "severity": {
///                        "type": "string"
///                      }
///                    }
///                  },
///                  "GPL-3.0": {
///                    "type": "object",
///                    "properties": {
///                      "instructions": {
///                        "type": "string"
///                      },
///                      "licenseType": {
///                        "type": "string"
///                      },
///                      "severity": {
///                        "type": "string"
///                      }
///                    }
///                  },
///                  "LGPL-2.0": {
///                    "type": "object",
///                    "properties": {
///                      "instructions": {
///                        "type": "string"
///                      },
///                      "licenseType": {
///                        "type": "string"
///                      },
///                      "severity": {
///                        "type": "string"
///                      }
///                    }
///                  },
///                  "LGPL-2.1": {
///                    "type": "object",
///                    "properties": {
///                      "instructions": {
///                        "type": "string"
///                      },
///                      "licenseType": {
///                        "type": "string"
///                      },
///                      "severity": {
///                        "type": "string"
///                      }
///                    }
///                  },
///                  "LGPL-3.0": {
///                    "type": "object",
///                    "properties": {
///                      "instructions": {
///                        "type": "string"
///                      },
///                      "licenseType": {
///                        "type": "string"
///                      },
///                      "severity": {
///                        "type": "string"
///                      }
///                    }
///                  },
///                  "MPL-1.1": {
///                    "type": "object",
///                    "properties": {
///                      "instructions": {
///                        "type": "string"
///                      },
///                      "licenseType": {
///                        "type": "string"
///                      },
///                      "severity": {
///                        "type": "string"
///                      }
///                    }
///                  },
///                  "MPL-2.0": {
///                    "type": "object",
///                    "properties": {
///                      "instructions": {
///                        "type": "string"
///                      },
///                      "licenseType": {
///                        "type": "string"
///                      },
///                      "severity": {
///                        "type": "string"
///                      }
///                    }
///                  },
///                  "MS-RL": {
///                    "type": "object",
///                    "properties": {
///                      "instructions": {
///                        "type": "string"
///                      },
///                      "licenseType": {
///                        "type": "string"
///                      },
///                      "severity": {
///                        "type": "string"
///                      }
///                    }
///                  },
///                  "SimPL-2.0": {
///                    "type": "object",
///                    "properties": {
///                      "instructions": {
///                        "type": "string"
///                      },
///                      "licenseType": {
///                        "type": "string"
///                      },
///                      "severity": {
///                        "type": "string"
///                      }
///                    }
///                  }
///                }
///              },
///              "severities": {
///                "type": "object"
///              }
///            }
///          },
///          "ok": {
///            "type": "boolean"
///          },
///          "org": {
///            "type": "string"
///          },
///          "packageManager": {
///            "type": "string"
///          },
///          "path": {
///            "type": "string"
///          },
///          "policy": {
///            "type": "string"
///          },
///          "projectName": {
///            "type": "string"
///          },
///          "remediation": {
///            "type": "object",
///            "properties": {
///              "ignore": {
///                "type": "object"
///              },
///              "patch": {
///                "type": "object"
///              },
///              "pin": {
///                "type": "object"
///              },
///              "unresolved": {
///                "type": "array",
///                "items": {
///                  "type": "object",
///                  "properties": {
///                    "creationTime": {
///                      "type": "string"
///                    },
///                    "description": {
///                      "type": "string"
///                    },
///                    "from": {
///                      "type": "array",
///                      "items": {
///                        "type": "string"
///                      }
///                    },
///                    "id": {
///                      "type": "string"
///                    },
///                    "isPatchable": {
///                      "type": "boolean"
///                    },
///                    "isPinnable": {
///                      "type": "boolean"
///                    },
///                    "isRuntime": {
///                      "type": "boolean"
///                    },
///                    "isUpgradable": {
///                      "type": "boolean"
///                    },
///                    "language": {
///                      "type": "string"
///                    },
///                    "license": {
///                      "type": "string"
///                    },
///                    "name": {
///                      "type": "string"
///                    },
///                    "packageManager": {
///                      "type": "string"
///                    },
///                    "packageName": {
///                      "type": "string"
///                    },
///                    "publicationTime": {
///                      "type": "string"
///                    },
///                    "semver": {
///                      "type": "object",
///                      "properties": {
///                        "vulnerable": {
///                          "type": "array",
///                          "items": {
///                            "type": "string"
///                          }
///                        }
///                      }
///                    },
///                    "severity": {
///                      "type": "string"
///                    },
///                    "severityWithCritical": {
///                      "type": "string"
///                    },
///                    "title": {
///                      "type": "string"
///                    },
///                    "type": {
///                      "type": "string"
///                    },
///                    "upgradePath": {
///                      "type": "array",
///                      "items": {
///                        "type": [
///                          "boolean",
///                          "string"
///                        ]
///                      }
///                    },
///                    "version": {
///                      "type": "string"
///                    }
///                  }
///                }
///              },
///              "upgrade": {
///                "type": "object",
///                "properties": {
///                  "ch.qos.logback:logback-core@1.5.13": {
///                    "type": "object",
///                    "properties": {
///                      "upgradeTo": {
///                        "type": "string"
///                      },
///                      "upgrades": {
///                        "type": "array",
///                        "items": {
///                          "type": "string"
///                        }
///                      },
///                      "vulns": {
///                        "type": "array",
///                        "items": {
///                          "type": "string"
///                        }
///                      }
///                    }
///                  },
///                  "org.apache.tomcat.embed:tomcat-embed-core@10.1.46": {
///                    "type": "object",
///                    "properties": {
///                      "upgradeTo": {
///                        "type": "string"
///                      },
///                      "upgrades": {
///                        "type": "array",
///                        "items": {
///                          "type": "string"
///                        }
///                      },
///                      "vulns": {
///                        "type": "array",
///                        "items": {
///                          "type": "string"
///                        }
///                      }
///                    }
///                  }
///                }
///              }
///            }
///          },
///          "summary": {
///            "type": "string"
///          },
///          "targetFile": {
///            "type": "string"
///          },
///          "uniqueCount": {
///            "type": "number"
///          },
///          "vulnerabilities": {
///            "type": "array",
///            "items": {
///              "type": "object",
///              "properties": {
///                "CVSSv3": {
///                  "type": "string"
///                },
///                "alternativeIds": {
///                  "type": "array"
///                },
///                "creationTime": {
///                  "type": "string"
///                },
///                "credit": {
///                  "type": "array",
///                  "items": {
///                    "type": "string"
///                  }
///                },
///                "cvssDetails": {
///                  "type": "array",
///                  "items": {
///                    "type": "object",
///                    "properties": {
///                      "assigner": {
///                        "type": "string"
///                      },
///                      "cvssV3BaseScore": {
///                        "type": "number"
///                      },
///                      "cvssV3Vector": {
///                        "type": "string"
///                      },
///                      "modificationTime": {
///                        "type": "string"
///                      },
///                      "severity": {
///                        "type": "string"
///                      }
///                    }
///                  }
///                },
///                "cvssScore": {
///                  "type": "number"
///                },
///                "cvssSources": {
///                  "type": "array",
///                  "items": {
///                    "type": "object",
///                    "properties": {
///                      "assigner": {
///                        "type": "string"
///                      },
///                      "baseScore": {
///                        "type": "number"
///                      },
///                      "cvssVersion": {
///                        "type": "string"
///                      },
///                      "modificationTime": {
///                        "type": "string"
///                      },
///                      "severity": {
///                        "type": "string"
///                      },
///                      "type": {
///                        "type": "string"
///                      },
///                      "vector": {
///                        "type": "string"
///                      }
///                    }
///                  }
///                },
///                "description": {
///                  "type": "string"
///                },
///                "disclosureTime": {
///                  "type": "string"
///                },
///                "epssDetails": {
///                  "type": "object",
///                  "properties": {
///                    "modelVersion": {
///                      "type": "string"
///                    },
///                    "percentile": {
///                      "type": "string"
///                    },
///                    "probability": {
///                      "type": "string"
///                    }
///                  }
///                },
///                "exploit": {
///                  "type": "string"
///                },
///                "exploitDetails": {
///                  "type": "object",
///                  "properties": {
///                    "maturityLevels": {
///                      "type": "array",
///                      "items": {
///                        "type": "object",
///                        "properties": {
///                          "format": {
///                            "type": "string"
///                          },
///                          "level": {
///                            "type": "string"
///                          },
///                          "type": {
///                            "type": "string"
///                          }
///                        }
///                      }
///                    },
///                    "sources": {
///                      "type": "array"
///                    }
///                  }
///                },
///                "fixedIn": {
///                  "type": "array",
///                  "items": {
///                    "type": "string"
///                  }
///                },
///                "from": {
///                  "type": "array",
///                  "items": {
///                    "type": "string"
///                  }
///                },
///                "functions": {
///                  "type": "array"
///                },
///                "functions_new": {
///                  "type": "array"
///                },
///                "id": {
///                  "type": "string"
///                },
///                "identifiers": {
///                  "type": "object",
///                  "properties": {
///                    "CVE": {
///                      "type": "array",
///                      "items": {
///                        "type": "string"
///                      }
///                    },
///                    "CWE": {
///                      "type": "array",
///                      "items": {
///                        "type": "string"
///                      }
///                    }
///                  }
///                },
///                "insights": {
///                  "type": "object",
///                  "properties": {
///                    "triageAdvice": {
///                      "type": "null"
///                    }
///                  }
///                },
///                "isDisputed": {
///                  "type": "boolean"
///                },
///                "isPatchable": {
///                  "type": "boolean"
///                },
///                "isUpgradable": {
///                  "type": "boolean"
///                },
///                "language": {
///                  "type": "string"
///                },
///                "malicious": {
///                  "type": "boolean"
///                },
///                "mavenModuleName": {
///                  "type": "object",
///                  "properties": {
///                    "artifactId": {
///                      "type": "string"
///                    },
///                    "groupId": {
///                      "type": "string"
///                    }
///                  }
///                },
///                "modificationTime": {
///                  "type": "string"
///                },
///                "moduleName": {
///                  "type": "string"
///                },
///                "name": {
///                  "type": "string"
///                },
///                "packageManager": {
///                  "type": "string"
///                },
///                "packageName": {
///                  "type": "string"
///                },
///                "patches": {
///                  "type": "array"
///                },
///                "proprietary": {
///                  "type": "boolean"
///                },
///                "publicationTime": {
///                  "type": "string"
///                },
///                "references": {
///                  "type": "array",
///                  "items": {
///                    "type": "object",
///                    "properties": {
///                      "title": {
///                        "type": "string"
///                      },
///                      "url": {
///                        "type": "string"
///                      }
///                    }
///                  }
///                },
///                "semver": {
///                  "type": "object",
///                  "properties": {
///                    "vulnerable": {
///                      "type": "array",
///                      "items": {
///                        "type": "string"
///                      }
///                    }
///                  }
///                },
///                "severity": {
///                  "type": "string"
///                },
///                "severityBasedOn": {
///                  "type": "string"
///                },
///                "severityWithCritical": {
///                  "type": "string"
///                },
///                "socialTrendAlert": {
///                  "type": "boolean"
///                },
///                "title": {
///                  "type": "string"
///                },
///                "upgradePath": {
///                  "type": "array",
///                  "items": {
///                    "type": [
///                      "boolean",
///                      "string"
///                    ]
///                  }
///                },
///                "version": {
///                  "type": "string"
///                }
///              }
///            }
///          }
///        }
///      }
///    },
///    "dependencyCount": {
///      "type": "number"
///    },
///    "displayTargetFile": {
///      "type": "string"
///    },
///    "docker": {
///      "type": "object",
///      "properties": {
///        "baseImage": {
///          "type": "string"
///        },
///        "baseImageRemediation": {
///          "type": "object",
///          "properties": {
///            "advice": {
///              "type": "array",
///              "items": {
///                "type": "object",
///                "properties": {
///                  "bold": {
///                    "type": "boolean"
///                  },
///                  "message": {
///                    "type": "string"
///                  }
///                }
///              }
///            },
///            "code": {
///              "type": "string"
///            }
///          }
///        },
///        "binariesVulns": {
///          "type": "object",
///          "properties": {
///            "affectedPkgs": {
///              "type": "object"
///            },
///            "issuesData": {
///              "type": "object"
///            }
///          }
///        },
///        "os": {
///          "type": "object",
///          "properties": {
///            "prettyName": {
///              "type": "string"
///            }
///          }
///        }
///      }
///    },
///    "filesystemPolicy": {
///      "type": "boolean"
///    },
///    "filtered": {
///      "type": "object",
///      "properties": {
///        "ignore": {
///          "type": "array"
///        },
///        "patch": {
///          "type": "array"
///        }
///      }
///    },
///    "hasUnknownVersions": {
///      "type": "boolean"
///    },
///    "ignoreSettings": {
///      "type": "object",
///      "properties": {
///        "adminOnly": {
///          "type": "boolean"
///        },
///        "disregardFilesystemIgnores": {
///          "type": "boolean"
///        },
///        "reasonRequired": {
///          "type": "boolean"
///        }
///      }
///    },
///    "isPrivate": {
///      "type": "boolean"
///    },
///    "licensesPolicy": {
///      "type": "object",
///      "properties": {
///        "orgLicenseRules": {
///          "type": "object",
///          "properties": {
///            "AGPL-1.0": {
///              "type": "object",
///              "properties": {
///                "instructions": {
///                  "type": "string"
///                },
///                "licenseType": {
///                  "type": "string"
///                },
///                "severity": {
///                  "type": "string"
///                }
///              }
///            },
///            "AGPL-3.0": {
///              "type": "object",
///              "properties": {
///                "instructions": {
///                  "type": "string"
///                },
///                "licenseType": {
///                  "type": "string"
///                },
///                "severity": {
///                  "type": "string"
///                }
///              }
///            },
///            "Artistic-1.0": {
///              "type": "object",
///              "properties": {
///                "instructions": {
///                  "type": "string"
///                },
///                "licenseType": {
///                  "type": "string"
///                },
///                "severity": {
///                  "type": "string"
///                }
///              }
///            },
///            "Artistic-2.0": {
///              "type": "object",
///              "properties": {
///                "instructions": {
///                  "type": "string"
///                },
///                "licenseType": {
///                  "type": "string"
///                },
///                "severity": {
///                  "type": "string"
///                }
///              }
///            },
///            "CDDL-1.0": {
///              "type": "object",
///              "properties": {
///                "instructions": {
///                  "type": "string"
///                },
///                "licenseType": {
///                  "type": "string"
///                },
///                "severity": {
///                  "type": "string"
///                }
///              }
///            },
///            "CPOL-1.02": {
///              "type": "object",
///              "properties": {
///                "instructions": {
///                  "type": "string"
///                },
///                "licenseType": {
///                  "type": "string"
///                },
///                "severity": {
///                  "type": "string"
///                }
///              }
///            },
///            "EPL-1.0": {
///              "type": "object",
///              "properties": {
///                "instructions": {
///                  "type": "string"
///                },
///                "licenseType": {
///                  "type": "string"
///                },
///                "severity": {
///                  "type": "string"
///                }
///              }
///            },
///            "GPL-2.0": {
///              "type": "object",
///              "properties": {
///                "instructions": {
///                  "type": "string"
///                },
///                "licenseType": {
///                  "type": "string"
///                },
///                "severity": {
///                  "type": "string"
///                }
///              }
///            },
///            "GPL-3.0": {
///              "type": "object",
///              "properties": {
///                "instructions": {
///                  "type": "string"
///                },
///                "licenseType": {
///                  "type": "string"
///                },
///                "severity": {
///                  "type": "string"
///                }
///              }
///            },
///            "LGPL-2.0": {
///              "type": "object",
///              "properties": {
///                "instructions": {
///                  "type": "string"
///                },
///                "licenseType": {
///                  "type": "string"
///                },
///                "severity": {
///                  "type": "string"
///                }
///              }
///            },
///            "LGPL-2.1": {
///              "type": "object",
///              "properties": {
///                "instructions": {
///                  "type": "string"
///                },
///                "licenseType": {
///                  "type": "string"
///                },
///                "severity": {
///                  "type": "string"
///                }
///              }
///            },
///            "LGPL-3.0": {
///              "type": "object",
///              "properties": {
///                "instructions": {
///                  "type": "string"
///                },
///                "licenseType": {
///                  "type": "string"
///                },
///                "severity": {
///                  "type": "string"
///                }
///              }
///            },
///            "MPL-1.1": {
///              "type": "object",
///              "properties": {
///                "instructions": {
///                  "type": "string"
///                },
///                "licenseType": {
///                  "type": "string"
///                },
///                "severity": {
///                  "type": "string"
///                }
///              }
///            },
///            "MPL-2.0": {
///              "type": "object",
///              "properties": {
///                "instructions": {
///                  "type": "string"
///                },
///                "licenseType": {
///                  "type": "string"
///                },
///                "severity": {
///                  "type": "string"
///                }
///              }
///            },
///            "MS-RL": {
///              "type": "object",
///              "properties": {
///                "instructions": {
///                  "type": "string"
///                },
///                "licenseType": {
///                  "type": "string"
///                },
///                "severity": {
///                  "type": "string"
///                }
///              }
///            },
///            "SimPL-2.0": {
///              "type": "object",
///              "properties": {
///                "instructions": {
///                  "type": "string"
///                },
///                "licenseType": {
///                  "type": "string"
///                },
///                "severity": {
///                  "type": "string"
///                }
///              }
///            }
///          }
///        },
///        "severities": {
///          "type": "object"
///        }
///      }
///    },
///    "ok": {
///      "type": "boolean"
///    },
///    "org": {
///      "type": "string"
///    },
///    "packageManager": {
///      "type": "string"
///    },
///    "path": {
///      "type": "string"
///    },
///    "platform": {
///      "type": "string"
///    },
///    "policy": {
///      "type": "string"
///    },
///    "projectName": {
///      "type": "string"
///    },
///    "summary": {
///      "type": "string"
///    },
///    "targetFile": {
///      "type": "string"
///    },
///    "uniqueCount": {
///      "type": "number"
///    },
///    "vulnerabilities": {
///      "type": "array",
///      "items": {
///        "type": "object",
///        "properties": {
///          "CVSSv3": {
///            "type": "string"
///          },
///          "cpes": {
///            "type": "array"
///          },
///          "creationTime": {
///            "type": "string"
///          },
///          "credit": {
///            "type": "array",
///            "items": {
///              "type": "string"
///            }
///          },
///          "cvssDetails": {
///            "type": "array",
///            "items": {
///              "type": "object",
///              "properties": {
///                "assigner": {
///                  "type": "string"
///                },
///                "cvssV3BaseScore": {
///                  "type": "number"
///                },
///                "cvssV3Vector": {
///                  "type": "string"
///                },
///                "modificationTime": {
///                  "type": "string"
///                },
///                "severity": {
///                  "type": "string"
///                }
///              }
///            }
///          },
///          "cvssScore": {
///            "type": "number"
///          },
///          "cvssSources": {
///            "type": "array",
///            "items": {
///              "type": "object",
///              "properties": {
///                "assigner": {
///                  "type": "string"
///                },
///                "baseScore": {
///                  "type": "number"
///                },
///                "cvssVersion": {
///                  "type": "string"
///                },
///                "modificationTime": {
///                  "type": "string"
///                },
///                "severity": {
///                  "type": "string"
///                },
///                "type": {
///                  "type": "string"
///                },
///                "vector": {
///                  "type": "string"
///                }
///              }
///            }
///          },
///          "description": {
///            "type": "string"
///          },
///          "disclosureTime": {
///            "type": "string"
///          },
///          "dockerBaseImage": {
///            "type": "string"
///          },
///          "epssDetails": {
///            "type": "object",
///            "properties": {
///              "modelVersion": {
///                "type": "string"
///              },
///              "percentile": {
///                "type": "string"
///              },
///              "probability": {
///                "type": "string"
///              }
///            }
///          },
///          "exploit": {
///            "type": "string"
///          },
///          "exploitDetails": {
///            "type": "object",
///            "properties": {
///              "maturityLevels": {
///                "type": "array",
///                "items": {
///                  "type": "object",
///                  "properties": {
///                    "format": {
///                      "type": "string"
///                    },
///                    "level": {
///                      "type": "string"
///                    },
///                    "type": {
///                      "type": "string"
///                    }
///                  }
///                }
///              },
///              "sources": {
///                "type": "array"
///              }
///            }
///          },
///          "fixedIn": {
///            "type": "array"
///          },
///          "from": {
///            "type": "array",
///            "items": {
///              "type": "string"
///            }
///          },
///          "id": {
///            "type": "string"
///          },
///          "identifiers": {
///            "type": "object",
///            "properties": {
///              "ALTERNATIVE": {
///                "type": "array"
///              },
///              "CVE": {
///                "type": "array",
///                "items": {
///                  "type": "string"
///                }
///              },
///              "CWE": {
///                "type": "array",
///                "items": {
///                  "type": "string"
///                }
///              }
///            }
///          },
///          "insights": {
///            "type": "object",
///            "properties": {
///              "triageAdvice": {
///                "type": "null"
///              }
///            }
///          },
///          "isDisputed": {
///            "type": "boolean"
///          },
///          "isPatchable": {
///            "type": "boolean"
///          },
///          "isUpgradable": {
///            "type": "boolean"
///          },
///          "language": {
///            "type": "string"
///          },
///          "malicious": {
///            "type": "boolean"
///          },
///          "modificationTime": {
///            "type": "string"
///          },
///          "name": {
///            "type": "string"
///          },
///          "nvdSeverity": {
///            "type": "string"
///          },
///          "packageManager": {
///            "type": "string"
///          },
///          "packageName": {
///            "type": "string"
///          },
///          "patches": {
///            "type": "array"
///          },
///          "publicationTime": {
///            "type": "string"
///          },
///          "references": {
///            "type": "array",
///            "items": {
///              "type": "object",
///              "properties": {
///                "title": {
///                  "type": "string"
///                },
///                "url": {
///                  "type": "string"
///                }
///              }
///            }
///          },
///          "relativeImportance": {
///            "type": "string"
///          },
///          "semver": {
///            "type": "object",
///            "properties": {
///              "vulnerable": {
///                "type": "array",
///                "items": {
///                  "type": "string"
///                }
///              }
///            }
///          },
///          "severity": {
///            "type": "string"
///          },
///          "severityBasedOn": {
///            "type": "string"
///          },
///          "severityWithCritical": {
///            "type": "string"
///          },
///          "socialTrendAlert": {
///            "type": "boolean"
///          },
///          "title": {
///            "type": "string"
///          },
///          "upgradePath": {
///            "type": "array",
///            "items": {
///              "type": [
///                "boolean",
///                "string"
///              ]
///            }
///          },
///          "version": {
///            "type": "string"
///          }
///        }
///      }
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct SnykContainer {
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub applications: ::std::vec::Vec<SnykContainerApplicationsItem>,
    #[serde(
        rename = "dependencyCount",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub dependency_count: ::std::option::Option<f64>,
    #[serde(
        rename = "displayTargetFile",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub display_target_file: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub docker: ::std::option::Option<SnykContainerDocker>,
    #[serde(
        rename = "filesystemPolicy",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub filesystem_policy: ::std::option::Option<bool>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub filtered: ::std::option::Option<SnykContainerFiltered>,
    #[serde(
        rename = "hasUnknownVersions",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub has_unknown_versions: ::std::option::Option<bool>,
    #[serde(
        rename = "ignoreSettings",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub ignore_settings: ::std::option::Option<SnykContainerIgnoreSettings>,
    #[serde(
        rename = "isPrivate",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub is_private: ::std::option::Option<bool>,
    #[serde(
        rename = "licensesPolicy",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub licenses_policy: ::std::option::Option<SnykContainerLicensesPolicy>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub ok: ::std::option::Option<bool>,
    pub org: ::std::string::String,
    #[serde(
        rename = "packageManager",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub package_manager: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub path: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub platform: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub policy: ::std::option::Option<::std::string::String>,
    #[serde(rename = "projectName")]
    pub project_name: ::std::string::String,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub summary: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "targetFile",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub target_file: ::std::option::Option<::std::string::String>,
    #[serde(rename = "uniqueCount")]
    pub unique_count: f64,
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub vulnerabilities: ::std::vec::Vec<SnykContainerVulnerabilitiesItem>,
}
impl ::std::convert::From<&SnykContainer> for SnykContainer {
    fn from(value: &SnykContainer) -> Self {
        value.clone()
    }
}
///`SnykContainerApplicationsItem`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "properties": {
///    "dependencyCount": {
///      "type": "number"
///    },
///    "displayTargetFile": {
///      "type": "string"
///    },
///    "docker": {
///      "type": "object"
///    },
///    "filesystemPolicy": {
///      "type": "boolean"
///    },
///    "filtered": {
///      "type": "object",
///      "properties": {
///        "ignore": {
///          "type": "array"
///        },
///        "patch": {
///          "type": "array"
///        }
///      }
///    },
///    "hasUnknownVersions": {
///      "type": "boolean"
///    },
///    "ignoreSettings": {
///      "type": "object",
///      "properties": {
///        "adminOnly": {
///          "type": "boolean"
///        },
///        "disregardFilesystemIgnores": {
///          "type": "boolean"
///        },
///        "reasonRequired": {
///          "type": "boolean"
///        }
///      }
///    },
///    "isPrivate": {
///      "type": "boolean"
///    },
///    "licensesPolicy": {
///      "type": "object",
///      "properties": {
///        "orgLicenseRules": {
///          "type": "object",
///          "properties": {
///            "AGPL-1.0": {
///              "type": "object",
///              "properties": {
///                "instructions": {
///                  "type": "string"
///                },
///                "licenseType": {
///                  "type": "string"
///                },
///                "severity": {
///                  "type": "string"
///                }
///              }
///            },
///            "AGPL-3.0": {
///              "type": "object",
///              "properties": {
///                "instructions": {
///                  "type": "string"
///                },
///                "licenseType": {
///                  "type": "string"
///                },
///                "severity": {
///                  "type": "string"
///                }
///              }
///            },
///            "Artistic-1.0": {
///              "type": "object",
///              "properties": {
///                "instructions": {
///                  "type": "string"
///                },
///                "licenseType": {
///                  "type": "string"
///                },
///                "severity": {
///                  "type": "string"
///                }
///              }
///            },
///            "Artistic-2.0": {
///              "type": "object",
///              "properties": {
///                "instructions": {
///                  "type": "string"
///                },
///                "licenseType": {
///                  "type": "string"
///                },
///                "severity": {
///                  "type": "string"
///                }
///              }
///            },
///            "CDDL-1.0": {
///              "type": "object",
///              "properties": {
///                "instructions": {
///                  "type": "string"
///                },
///                "licenseType": {
///                  "type": "string"
///                },
///                "severity": {
///                  "type": "string"
///                }
///              }
///            },
///            "CPOL-1.02": {
///              "type": "object",
///              "properties": {
///                "instructions": {
///                  "type": "string"
///                },
///                "licenseType": {
///                  "type": "string"
///                },
///                "severity": {
///                  "type": "string"
///                }
///              }
///            },
///            "EPL-1.0": {
///              "type": "object",
///              "properties": {
///                "instructions": {
///                  "type": "string"
///                },
///                "licenseType": {
///                  "type": "string"
///                },
///                "severity": {
///                  "type": "string"
///                }
///              }
///            },
///            "GPL-2.0": {
///              "type": "object",
///              "properties": {
///                "instructions": {
///                  "type": "string"
///                },
///                "licenseType": {
///                  "type": "string"
///                },
///                "severity": {
///                  "type": "string"
///                }
///              }
///            },
///            "GPL-3.0": {
///              "type": "object",
///              "properties": {
///                "instructions": {
///                  "type": "string"
///                },
///                "licenseType": {
///                  "type": "string"
///                },
///                "severity": {
///                  "type": "string"
///                }
///              }
///            },
///            "LGPL-2.0": {
///              "type": "object",
///              "properties": {
///                "instructions": {
///                  "type": "string"
///                },
///                "licenseType": {
///                  "type": "string"
///                },
///                "severity": {
///                  "type": "string"
///                }
///              }
///            },
///            "LGPL-2.1": {
///              "type": "object",
///              "properties": {
///                "instructions": {
///                  "type": "string"
///                },
///                "licenseType": {
///                  "type": "string"
///                },
///                "severity": {
///                  "type": "string"
///                }
///              }
///            },
///            "LGPL-3.0": {
///              "type": "object",
///              "properties": {
///                "instructions": {
///                  "type": "string"
///                },
///                "licenseType": {
///                  "type": "string"
///                },
///                "severity": {
///                  "type": "string"
///                }
///              }
///            },
///            "MPL-1.1": {
///              "type": "object",
///              "properties": {
///                "instructions": {
///                  "type": "string"
///                },
///                "licenseType": {
///                  "type": "string"
///                },
///                "severity": {
///                  "type": "string"
///                }
///              }
///            },
///            "MPL-2.0": {
///              "type": "object",
///              "properties": {
///                "instructions": {
///                  "type": "string"
///                },
///                "licenseType": {
///                  "type": "string"
///                },
///                "severity": {
///                  "type": "string"
///                }
///              }
///            },
///            "MS-RL": {
///              "type": "object",
///              "properties": {
///                "instructions": {
///                  "type": "string"
///                },
///                "licenseType": {
///                  "type": "string"
///                },
///                "severity": {
///                  "type": "string"
///                }
///              }
///            },
///            "SimPL-2.0": {
///              "type": "object",
///              "properties": {
///                "instructions": {
///                  "type": "string"
///                },
///                "licenseType": {
///                  "type": "string"
///                },
///                "severity": {
///                  "type": "string"
///                }
///              }
///            }
///          }
///        },
///        "severities": {
///          "type": "object"
///        }
///      }
///    },
///    "ok": {
///      "type": "boolean"
///    },
///    "org": {
///      "type": "string"
///    },
///    "packageManager": {
///      "type": "string"
///    },
///    "path": {
///      "type": "string"
///    },
///    "policy": {
///      "type": "string"
///    },
///    "projectName": {
///      "type": "string"
///    },
///    "remediation": {
///      "type": "object",
///      "properties": {
///        "ignore": {
///          "type": "object"
///        },
///        "patch": {
///          "type": "object"
///        },
///        "pin": {
///          "type": "object"
///        },
///        "unresolved": {
///          "type": "array",
///          "items": {
///            "type": "object",
///            "properties": {
///              "creationTime": {
///                "type": "string"
///              },
///              "description": {
///                "type": "string"
///              },
///              "from": {
///                "type": "array",
///                "items": {
///                  "type": "string"
///                }
///              },
///              "id": {
///                "type": "string"
///              },
///              "isPatchable": {
///                "type": "boolean"
///              },
///              "isPinnable": {
///                "type": "boolean"
///              },
///              "isRuntime": {
///                "type": "boolean"
///              },
///              "isUpgradable": {
///                "type": "boolean"
///              },
///              "language": {
///                "type": "string"
///              },
///              "license": {
///                "type": "string"
///              },
///              "name": {
///                "type": "string"
///              },
///              "packageManager": {
///                "type": "string"
///              },
///              "packageName": {
///                "type": "string"
///              },
///              "publicationTime": {
///                "type": "string"
///              },
///              "semver": {
///                "type": "object",
///                "properties": {
///                  "vulnerable": {
///                    "type": "array",
///                    "items": {
///                      "type": "string"
///                    }
///                  }
///                }
///              },
///              "severity": {
///                "type": "string"
///              },
///              "severityWithCritical": {
///                "type": "string"
///              },
///              "title": {
///                "type": "string"
///              },
///              "type": {
///                "type": "string"
///              },
///              "upgradePath": {
///                "type": "array",
///                "items": {
///                  "type": [
///                    "boolean",
///                    "string"
///                  ]
///                }
///              },
///              "version": {
///                "type": "string"
///              }
///            }
///          }
///        },
///        "upgrade": {
///          "type": "object",
///          "properties": {
///            "ch.qos.logback:logback-core@1.5.13": {
///              "type": "object",
///              "properties": {
///                "upgradeTo": {
///                  "type": "string"
///                },
///                "upgrades": {
///                  "type": "array",
///                  "items": {
///                    "type": "string"
///                  }
///                },
///                "vulns": {
///                  "type": "array",
///                  "items": {
///                    "type": "string"
///                  }
///                }
///              }
///            },
///            "org.apache.tomcat.embed:tomcat-embed-core@10.1.46": {
///              "type": "object",
///              "properties": {
///                "upgradeTo": {
///                  "type": "string"
///                },
///                "upgrades": {
///                  "type": "array",
///                  "items": {
///                    "type": "string"
///                  }
///                },
///                "vulns": {
///                  "type": "array",
///                  "items": {
///                    "type": "string"
///                  }
///                }
///              }
///            }
///          }
///        }
///      }
///    },
///    "summary": {
///      "type": "string"
///    },
///    "targetFile": {
///      "type": "string"
///    },
///    "uniqueCount": {
///      "type": "number"
///    },
///    "vulnerabilities": {
///      "type": "array",
///      "items": {
///        "type": "object",
///        "properties": {
///          "CVSSv3": {
///            "type": "string"
///          },
///          "alternativeIds": {
///            "type": "array"
///          },
///          "creationTime": {
///            "type": "string"
///          },
///          "credit": {
///            "type": "array",
///            "items": {
///              "type": "string"
///            }
///          },
///          "cvssDetails": {
///            "type": "array",
///            "items": {
///              "type": "object",
///              "properties": {
///                "assigner": {
///                  "type": "string"
///                },
///                "cvssV3BaseScore": {
///                  "type": "number"
///                },
///                "cvssV3Vector": {
///                  "type": "string"
///                },
///                "modificationTime": {
///                  "type": "string"
///                },
///                "severity": {
///                  "type": "string"
///                }
///              }
///            }
///          },
///          "cvssScore": {
///            "type": "number"
///          },
///          "cvssSources": {
///            "type": "array",
///            "items": {
///              "type": "object",
///              "properties": {
///                "assigner": {
///                  "type": "string"
///                },
///                "baseScore": {
///                  "type": "number"
///                },
///                "cvssVersion": {
///                  "type": "string"
///                },
///                "modificationTime": {
///                  "type": "string"
///                },
///                "severity": {
///                  "type": "string"
///                },
///                "type": {
///                  "type": "string"
///                },
///                "vector": {
///                  "type": "string"
///                }
///              }
///            }
///          },
///          "description": {
///            "type": "string"
///          },
///          "disclosureTime": {
///            "type": "string"
///          },
///          "epssDetails": {
///            "type": "object",
///            "properties": {
///              "modelVersion": {
///                "type": "string"
///              },
///              "percentile": {
///                "type": "string"
///              },
///              "probability": {
///                "type": "string"
///              }
///            }
///          },
///          "exploit": {
///            "type": "string"
///          },
///          "exploitDetails": {
///            "type": "object",
///            "properties": {
///              "maturityLevels": {
///                "type": "array",
///                "items": {
///                  "type": "object",
///                  "properties": {
///                    "format": {
///                      "type": "string"
///                    },
///                    "level": {
///                      "type": "string"
///                    },
///                    "type": {
///                      "type": "string"
///                    }
///                  }
///                }
///              },
///              "sources": {
///                "type": "array"
///              }
///            }
///          },
///          "fixedIn": {
///            "type": "array",
///            "items": {
///              "type": "string"
///            }
///          },
///          "from": {
///            "type": "array",
///            "items": {
///              "type": "string"
///            }
///          },
///          "functions": {
///            "type": "array"
///          },
///          "functions_new": {
///            "type": "array"
///          },
///          "id": {
///            "type": "string"
///          },
///          "identifiers": {
///            "type": "object",
///            "properties": {
///              "CVE": {
///                "type": "array",
///                "items": {
///                  "type": "string"
///                }
///              },
///              "CWE": {
///                "type": "array",
///                "items": {
///                  "type": "string"
///                }
///              }
///            }
///          },
///          "insights": {
///            "type": "object",
///            "properties": {
///              "triageAdvice": {
///                "type": "null"
///              }
///            }
///          },
///          "isDisputed": {
///            "type": "boolean"
///          },
///          "isPatchable": {
///            "type": "boolean"
///          },
///          "isUpgradable": {
///            "type": "boolean"
///          },
///          "language": {
///            "type": "string"
///          },
///          "malicious": {
///            "type": "boolean"
///          },
///          "mavenModuleName": {
///            "type": "object",
///            "properties": {
///              "artifactId": {
///                "type": "string"
///              },
///              "groupId": {
///                "type": "string"
///              }
///            }
///          },
///          "modificationTime": {
///            "type": "string"
///          },
///          "moduleName": {
///            "type": "string"
///          },
///          "name": {
///            "type": "string"
///          },
///          "packageManager": {
///            "type": "string"
///          },
///          "packageName": {
///            "type": "string"
///          },
///          "patches": {
///            "type": "array"
///          },
///          "proprietary": {
///            "type": "boolean"
///          },
///          "publicationTime": {
///            "type": "string"
///          },
///          "references": {
///            "type": "array",
///            "items": {
///              "type": "object",
///              "properties": {
///                "title": {
///                  "type": "string"
///                },
///                "url": {
///                  "type": "string"
///                }
///              }
///            }
///          },
///          "semver": {
///            "type": "object",
///            "properties": {
///              "vulnerable": {
///                "type": "array",
///                "items": {
///                  "type": "string"
///                }
///              }
///            }
///          },
///          "severity": {
///            "type": "string"
///          },
///          "severityBasedOn": {
///            "type": "string"
///          },
///          "severityWithCritical": {
///            "type": "string"
///          },
///          "socialTrendAlert": {
///            "type": "boolean"
///          },
///          "title": {
///            "type": "string"
///          },
///          "upgradePath": {
///            "type": "array",
///            "items": {
///              "type": [
///                "boolean",
///                "string"
///              ]
///            }
///          },
///          "version": {
///            "type": "string"
///          }
///        }
///      }
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct SnykContainerApplicationsItem {
    #[serde(
        rename = "dependencyCount",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub dependency_count: ::std::option::Option<f64>,
    #[serde(
        rename = "displayTargetFile",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub display_target_file: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::serde_json::Map::is_empty")]
    pub docker: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
    #[serde(
        rename = "filesystemPolicy",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub filesystem_policy: ::std::option::Option<bool>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub filtered: ::std::option::Option<SnykContainerApplicationsItemFiltered>,
    #[serde(
        rename = "hasUnknownVersions",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub has_unknown_versions: ::std::option::Option<bool>,
    #[serde(
        rename = "ignoreSettings",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub ignore_settings: ::std::option::Option<SnykContainerApplicationsItemIgnoreSettings>,
    #[serde(
        rename = "isPrivate",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub is_private: ::std::option::Option<bool>,
    #[serde(
        rename = "licensesPolicy",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub licenses_policy: ::std::option::Option<SnykContainerApplicationsItemLicensesPolicy>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub ok: ::std::option::Option<bool>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub org: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "packageManager",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub package_manager: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub path: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub policy: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "projectName",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub project_name: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub remediation: ::std::option::Option<SnykContainerApplicationsItemRemediation>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub summary: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "targetFile",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub target_file: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "uniqueCount",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub unique_count: ::std::option::Option<f64>,
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub vulnerabilities: ::std::vec::Vec<SnykContainerApplicationsItemVulnerabilitiesItem>,
}
impl ::std::convert::From<&SnykContainerApplicationsItem> for SnykContainerApplicationsItem {
    fn from(value: &SnykContainerApplicationsItem) -> Self {
        value.clone()
    }
}
impl ::std::default::Default for SnykContainerApplicationsItem {
    fn default() -> Self {
        Self {
            dependency_count: Default::default(),
            display_target_file: Default::default(),
            docker: Default::default(),
            filesystem_policy: Default::default(),
            filtered: Default::default(),
            has_unknown_versions: Default::default(),
            ignore_settings: Default::default(),
            is_private: Default::default(),
            licenses_policy: Default::default(),
            ok: Default::default(),
            org: Default::default(),
            package_manager: Default::default(),
            path: Default::default(),
            policy: Default::default(),
            project_name: Default::default(),
            remediation: Default::default(),
            summary: Default::default(),
            target_file: Default::default(),
            unique_count: Default::default(),
            vulnerabilities: Default::default(),
        }
    }
}
///`SnykContainerApplicationsItemFiltered`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "properties": {
///    "ignore": {
///      "type": "array"
///    },
///    "patch": {
///      "type": "array"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct SnykContainerApplicationsItemFiltered {
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub ignore: ::std::vec::Vec<::serde_json::Value>,
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub patch: ::std::vec::Vec<::serde_json::Value>,
}
impl ::std::convert::From<&SnykContainerApplicationsItemFiltered>
    for SnykContainerApplicationsItemFiltered
{
    fn from(value: &SnykContainerApplicationsItemFiltered) -> Self {
        value.clone()
    }
}
impl ::std::default::Default for SnykContainerApplicationsItemFiltered {
    fn default() -> Self {
        Self {
            ignore: Default::default(),
            patch: Default::default(),
        }
    }
}
///`SnykContainerApplicationsItemIgnoreSettings`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "properties": {
///    "adminOnly": {
///      "type": "boolean"
///    },
///    "disregardFilesystemIgnores": {
///      "type": "boolean"
///    },
///    "reasonRequired": {
///      "type": "boolean"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct SnykContainerApplicationsItemIgnoreSettings {
    #[serde(
        rename = "adminOnly",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub admin_only: ::std::option::Option<bool>,
    #[serde(
        rename = "disregardFilesystemIgnores",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub disregard_filesystem_ignores: ::std::option::Option<bool>,
    #[serde(
        rename = "reasonRequired",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub reason_required: ::std::option::Option<bool>,
}
impl ::std::convert::From<&SnykContainerApplicationsItemIgnoreSettings>
    for SnykContainerApplicationsItemIgnoreSettings
{
    fn from(value: &SnykContainerApplicationsItemIgnoreSettings) -> Self {
        value.clone()
    }
}
impl ::std::default::Default for SnykContainerApplicationsItemIgnoreSettings {
    fn default() -> Self {
        Self {
            admin_only: Default::default(),
            disregard_filesystem_ignores: Default::default(),
            reason_required: Default::default(),
        }
    }
}
///`SnykContainerApplicationsItemLicensesPolicy`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "properties": {
///    "orgLicenseRules": {
///      "type": "object",
///      "properties": {
///        "AGPL-1.0": {
///          "type": "object",
///          "properties": {
///            "instructions": {
///              "type": "string"
///            },
///            "licenseType": {
///              "type": "string"
///            },
///            "severity": {
///              "type": "string"
///            }
///          }
///        },
///        "AGPL-3.0": {
///          "type": "object",
///          "properties": {
///            "instructions": {
///              "type": "string"
///            },
///            "licenseType": {
///              "type": "string"
///            },
///            "severity": {
///              "type": "string"
///            }
///          }
///        },
///        "Artistic-1.0": {
///          "type": "object",
///          "properties": {
///            "instructions": {
///              "type": "string"
///            },
///            "licenseType": {
///              "type": "string"
///            },
///            "severity": {
///              "type": "string"
///            }
///          }
///        },
///        "Artistic-2.0": {
///          "type": "object",
///          "properties": {
///            "instructions": {
///              "type": "string"
///            },
///            "licenseType": {
///              "type": "string"
///            },
///            "severity": {
///              "type": "string"
///            }
///          }
///        },
///        "CDDL-1.0": {
///          "type": "object",
///          "properties": {
///            "instructions": {
///              "type": "string"
///            },
///            "licenseType": {
///              "type": "string"
///            },
///            "severity": {
///              "type": "string"
///            }
///          }
///        },
///        "CPOL-1.02": {
///          "type": "object",
///          "properties": {
///            "instructions": {
///              "type": "string"
///            },
///            "licenseType": {
///              "type": "string"
///            },
///            "severity": {
///              "type": "string"
///            }
///          }
///        },
///        "EPL-1.0": {
///          "type": "object",
///          "properties": {
///            "instructions": {
///              "type": "string"
///            },
///            "licenseType": {
///              "type": "string"
///            },
///            "severity": {
///              "type": "string"
///            }
///          }
///        },
///        "GPL-2.0": {
///          "type": "object",
///          "properties": {
///            "instructions": {
///              "type": "string"
///            },
///            "licenseType": {
///              "type": "string"
///            },
///            "severity": {
///              "type": "string"
///            }
///          }
///        },
///        "GPL-3.0": {
///          "type": "object",
///          "properties": {
///            "instructions": {
///              "type": "string"
///            },
///            "licenseType": {
///              "type": "string"
///            },
///            "severity": {
///              "type": "string"
///            }
///          }
///        },
///        "LGPL-2.0": {
///          "type": "object",
///          "properties": {
///            "instructions": {
///              "type": "string"
///            },
///            "licenseType": {
///              "type": "string"
///            },
///            "severity": {
///              "type": "string"
///            }
///          }
///        },
///        "LGPL-2.1": {
///          "type": "object",
///          "properties": {
///            "instructions": {
///              "type": "string"
///            },
///            "licenseType": {
///              "type": "string"
///            },
///            "severity": {
///              "type": "string"
///            }
///          }
///        },
///        "LGPL-3.0": {
///          "type": "object",
///          "properties": {
///            "instructions": {
///              "type": "string"
///            },
///            "licenseType": {
///              "type": "string"
///            },
///            "severity": {
///              "type": "string"
///            }
///          }
///        },
///        "MPL-1.1": {
///          "type": "object",
///          "properties": {
///            "instructions": {
///              "type": "string"
///            },
///            "licenseType": {
///              "type": "string"
///            },
///            "severity": {
///              "type": "string"
///            }
///          }
///        },
///        "MPL-2.0": {
///          "type": "object",
///          "properties": {
///            "instructions": {
///              "type": "string"
///            },
///            "licenseType": {
///              "type": "string"
///            },
///            "severity": {
///              "type": "string"
///            }
///          }
///        },
///        "MS-RL": {
///          "type": "object",
///          "properties": {
///            "instructions": {
///              "type": "string"
///            },
///            "licenseType": {
///              "type": "string"
///            },
///            "severity": {
///              "type": "string"
///            }
///          }
///        },
///        "SimPL-2.0": {
///          "type": "object",
///          "properties": {
///            "instructions": {
///              "type": "string"
///            },
///            "licenseType": {
///              "type": "string"
///            },
///            "severity": {
///              "type": "string"
///            }
///          }
///        }
///      }
///    },
///    "severities": {
///      "type": "object"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct SnykContainerApplicationsItemLicensesPolicy {
    #[serde(
        rename = "orgLicenseRules",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub org_license_rules:
        ::std::option::Option<SnykContainerApplicationsItemLicensesPolicyOrgLicenseRules>,
    #[serde(default, skip_serializing_if = "::serde_json::Map::is_empty")]
    pub severities: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
}
impl ::std::convert::From<&SnykContainerApplicationsItemLicensesPolicy>
    for SnykContainerApplicationsItemLicensesPolicy
{
    fn from(value: &SnykContainerApplicationsItemLicensesPolicy) -> Self {
        value.clone()
    }
}
impl ::std::default::Default for SnykContainerApplicationsItemLicensesPolicy {
    fn default() -> Self {
        Self {
            org_license_rules: Default::default(),
            severities: Default::default(),
        }
    }
}
///`SnykContainerApplicationsItemLicensesPolicyOrgLicenseRules`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "properties": {
///    "AGPL-1.0": {
///      "type": "object",
///      "properties": {
///        "instructions": {
///          "type": "string"
///        },
///        "licenseType": {
///          "type": "string"
///        },
///        "severity": {
///          "type": "string"
///        }
///      }
///    },
///    "AGPL-3.0": {
///      "type": "object",
///      "properties": {
///        "instructions": {
///          "type": "string"
///        },
///        "licenseType": {
///          "type": "string"
///        },
///        "severity": {
///          "type": "string"
///        }
///      }
///    },
///    "Artistic-1.0": {
///      "type": "object",
///      "properties": {
///        "instructions": {
///          "type": "string"
///        },
///        "licenseType": {
///          "type": "string"
///        },
///        "severity": {
///          "type": "string"
///        }
///      }
///    },
///    "Artistic-2.0": {
///      "type": "object",
///      "properties": {
///        "instructions": {
///          "type": "string"
///        },
///        "licenseType": {
///          "type": "string"
///        },
///        "severity": {
///          "type": "string"
///        }
///      }
///    },
///    "CDDL-1.0": {
///      "type": "object",
///      "properties": {
///        "instructions": {
///          "type": "string"
///        },
///        "licenseType": {
///          "type": "string"
///        },
///        "severity": {
///          "type": "string"
///        }
///      }
///    },
///    "CPOL-1.02": {
///      "type": "object",
///      "properties": {
///        "instructions": {
///          "type": "string"
///        },
///        "licenseType": {
///          "type": "string"
///        },
///        "severity": {
///          "type": "string"
///        }
///      }
///    },
///    "EPL-1.0": {
///      "type": "object",
///      "properties": {
///        "instructions": {
///          "type": "string"
///        },
///        "licenseType": {
///          "type": "string"
///        },
///        "severity": {
///          "type": "string"
///        }
///      }
///    },
///    "GPL-2.0": {
///      "type": "object",
///      "properties": {
///        "instructions": {
///          "type": "string"
///        },
///        "licenseType": {
///          "type": "string"
///        },
///        "severity": {
///          "type": "string"
///        }
///      }
///    },
///    "GPL-3.0": {
///      "type": "object",
///      "properties": {
///        "instructions": {
///          "type": "string"
///        },
///        "licenseType": {
///          "type": "string"
///        },
///        "severity": {
///          "type": "string"
///        }
///      }
///    },
///    "LGPL-2.0": {
///      "type": "object",
///      "properties": {
///        "instructions": {
///          "type": "string"
///        },
///        "licenseType": {
///          "type": "string"
///        },
///        "severity": {
///          "type": "string"
///        }
///      }
///    },
///    "LGPL-2.1": {
///      "type": "object",
///      "properties": {
///        "instructions": {
///          "type": "string"
///        },
///        "licenseType": {
///          "type": "string"
///        },
///        "severity": {
///          "type": "string"
///        }
///      }
///    },
///    "LGPL-3.0": {
///      "type": "object",
///      "properties": {
///        "instructions": {
///          "type": "string"
///        },
///        "licenseType": {
///          "type": "string"
///        },
///        "severity": {
///          "type": "string"
///        }
///      }
///    },
///    "MPL-1.1": {
///      "type": "object",
///      "properties": {
///        "instructions": {
///          "type": "string"
///        },
///        "licenseType": {
///          "type": "string"
///        },
///        "severity": {
///          "type": "string"
///        }
///      }
///    },
///    "MPL-2.0": {
///      "type": "object",
///      "properties": {
///        "instructions": {
///          "type": "string"
///        },
///        "licenseType": {
///          "type": "string"
///        },
///        "severity": {
///          "type": "string"
///        }
///      }
///    },
///    "MS-RL": {
///      "type": "object",
///      "properties": {
///        "instructions": {
///          "type": "string"
///        },
///        "licenseType": {
///          "type": "string"
///        },
///        "severity": {
///          "type": "string"
///        }
///      }
///    },
///    "SimPL-2.0": {
///      "type": "object",
///      "properties": {
///        "instructions": {
///          "type": "string"
///        },
///        "licenseType": {
///          "type": "string"
///        },
///        "severity": {
///          "type": "string"
///        }
///      }
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct SnykContainerApplicationsItemLicensesPolicyOrgLicenseRules {
    #[serde(
        rename = "AGPL-1.0",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub agpl_1_0:
        ::std::option::Option<SnykContainerApplicationsItemLicensesPolicyOrgLicenseRulesAgpl10>,
    #[serde(
        rename = "AGPL-3.0",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub agpl_3_0:
        ::std::option::Option<SnykContainerApplicationsItemLicensesPolicyOrgLicenseRulesAgpl30>,
    #[serde(
        rename = "Artistic-1.0",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub artistic_1_0:
        ::std::option::Option<SnykContainerApplicationsItemLicensesPolicyOrgLicenseRulesArtistic10>,
    #[serde(
        rename = "Artistic-2.0",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub artistic_2_0:
        ::std::option::Option<SnykContainerApplicationsItemLicensesPolicyOrgLicenseRulesArtistic20>,
    #[serde(
        rename = "CDDL-1.0",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub cddl_1_0:
        ::std::option::Option<SnykContainerApplicationsItemLicensesPolicyOrgLicenseRulesCddl10>,
    #[serde(
        rename = "CPOL-1.02",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub cpol_1_02:
        ::std::option::Option<SnykContainerApplicationsItemLicensesPolicyOrgLicenseRulesCpol102>,
    #[serde(
        rename = "EPL-1.0",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub epl_1_0:
        ::std::option::Option<SnykContainerApplicationsItemLicensesPolicyOrgLicenseRulesEpl10>,
    #[serde(
        rename = "GPL-2.0",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub gpl_2_0:
        ::std::option::Option<SnykContainerApplicationsItemLicensesPolicyOrgLicenseRulesGpl20>,
    #[serde(
        rename = "GPL-3.0",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub gpl_3_0:
        ::std::option::Option<SnykContainerApplicationsItemLicensesPolicyOrgLicenseRulesGpl30>,
    #[serde(
        rename = "LGPL-2.0",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub lgpl_2_0:
        ::std::option::Option<SnykContainerApplicationsItemLicensesPolicyOrgLicenseRulesLgpl20>,
    #[serde(
        rename = "LGPL-2.1",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub lgpl_2_1:
        ::std::option::Option<SnykContainerApplicationsItemLicensesPolicyOrgLicenseRulesLgpl21>,
    #[serde(
        rename = "LGPL-3.0",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub lgpl_3_0:
        ::std::option::Option<SnykContainerApplicationsItemLicensesPolicyOrgLicenseRulesLgpl30>,
    #[serde(
        rename = "MPL-1.1",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub mpl_1_1:
        ::std::option::Option<SnykContainerApplicationsItemLicensesPolicyOrgLicenseRulesMpl11>,
    #[serde(
        rename = "MPL-2.0",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub mpl_2_0:
        ::std::option::Option<SnykContainerApplicationsItemLicensesPolicyOrgLicenseRulesMpl20>,
    #[serde(
        rename = "MS-RL",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub ms_rl:
        ::std::option::Option<SnykContainerApplicationsItemLicensesPolicyOrgLicenseRulesMsRl>,
    #[serde(
        rename = "SimPL-2.0",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub sim_pl_2_0:
        ::std::option::Option<SnykContainerApplicationsItemLicensesPolicyOrgLicenseRulesSimPl20>,
}
impl ::std::convert::From<&SnykContainerApplicationsItemLicensesPolicyOrgLicenseRules>
    for SnykContainerApplicationsItemLicensesPolicyOrgLicenseRules
{
    fn from(value: &SnykContainerApplicationsItemLicensesPolicyOrgLicenseRules) -> Self {
        value.clone()
    }
}
impl ::std::default::Default for SnykContainerApplicationsItemLicensesPolicyOrgLicenseRules {
    fn default() -> Self {
        Self {
            agpl_1_0: Default::default(),
            agpl_3_0: Default::default(),
            artistic_1_0: Default::default(),
            artistic_2_0: Default::default(),
            cddl_1_0: Default::default(),
            cpol_1_02: Default::default(),
            epl_1_0: Default::default(),
            gpl_2_0: Default::default(),
            gpl_3_0: Default::default(),
            lgpl_2_0: Default::default(),
            lgpl_2_1: Default::default(),
            lgpl_3_0: Default::default(),
            mpl_1_1: Default::default(),
            mpl_2_0: Default::default(),
            ms_rl: Default::default(),
            sim_pl_2_0: Default::default(),
        }
    }
}
///`SnykContainerApplicationsItemLicensesPolicyOrgLicenseRulesAgpl10`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "properties": {
///    "instructions": {
///      "type": "string"
///    },
///    "licenseType": {
///      "type": "string"
///    },
///    "severity": {
///      "type": "string"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct SnykContainerApplicationsItemLicensesPolicyOrgLicenseRulesAgpl10 {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub instructions: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "licenseType",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub license_type: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub severity: ::std::option::Option<::std::string::String>,
}
impl ::std::convert::From<&SnykContainerApplicationsItemLicensesPolicyOrgLicenseRulesAgpl10>
    for SnykContainerApplicationsItemLicensesPolicyOrgLicenseRulesAgpl10
{
    fn from(value: &SnykContainerApplicationsItemLicensesPolicyOrgLicenseRulesAgpl10) -> Self {
        value.clone()
    }
}
impl ::std::default::Default for SnykContainerApplicationsItemLicensesPolicyOrgLicenseRulesAgpl10 {
    fn default() -> Self {
        Self {
            instructions: Default::default(),
            license_type: Default::default(),
            severity: Default::default(),
        }
    }
}
///`SnykContainerApplicationsItemLicensesPolicyOrgLicenseRulesAgpl30`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "properties": {
///    "instructions": {
///      "type": "string"
///    },
///    "licenseType": {
///      "type": "string"
///    },
///    "severity": {
///      "type": "string"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct SnykContainerApplicationsItemLicensesPolicyOrgLicenseRulesAgpl30 {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub instructions: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "licenseType",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub license_type: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub severity: ::std::option::Option<::std::string::String>,
}
impl ::std::convert::From<&SnykContainerApplicationsItemLicensesPolicyOrgLicenseRulesAgpl30>
    for SnykContainerApplicationsItemLicensesPolicyOrgLicenseRulesAgpl30
{
    fn from(value: &SnykContainerApplicationsItemLicensesPolicyOrgLicenseRulesAgpl30) -> Self {
        value.clone()
    }
}
impl ::std::default::Default for SnykContainerApplicationsItemLicensesPolicyOrgLicenseRulesAgpl30 {
    fn default() -> Self {
        Self {
            instructions: Default::default(),
            license_type: Default::default(),
            severity: Default::default(),
        }
    }
}
///`SnykContainerApplicationsItemLicensesPolicyOrgLicenseRulesArtistic10`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "properties": {
///    "instructions": {
///      "type": "string"
///    },
///    "licenseType": {
///      "type": "string"
///    },
///    "severity": {
///      "type": "string"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct SnykContainerApplicationsItemLicensesPolicyOrgLicenseRulesArtistic10 {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub instructions: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "licenseType",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub license_type: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub severity: ::std::option::Option<::std::string::String>,
}
impl ::std::convert::From<&SnykContainerApplicationsItemLicensesPolicyOrgLicenseRulesArtistic10>
    for SnykContainerApplicationsItemLicensesPolicyOrgLicenseRulesArtistic10
{
    fn from(value: &SnykContainerApplicationsItemLicensesPolicyOrgLicenseRulesArtistic10) -> Self {
        value.clone()
    }
}
impl ::std::default::Default
    for SnykContainerApplicationsItemLicensesPolicyOrgLicenseRulesArtistic10
{
    fn default() -> Self {
        Self {
            instructions: Default::default(),
            license_type: Default::default(),
            severity: Default::default(),
        }
    }
}
///`SnykContainerApplicationsItemLicensesPolicyOrgLicenseRulesArtistic20`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "properties": {
///    "instructions": {
///      "type": "string"
///    },
///    "licenseType": {
///      "type": "string"
///    },
///    "severity": {
///      "type": "string"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct SnykContainerApplicationsItemLicensesPolicyOrgLicenseRulesArtistic20 {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub instructions: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "licenseType",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub license_type: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub severity: ::std::option::Option<::std::string::String>,
}
impl ::std::convert::From<&SnykContainerApplicationsItemLicensesPolicyOrgLicenseRulesArtistic20>
    for SnykContainerApplicationsItemLicensesPolicyOrgLicenseRulesArtistic20
{
    fn from(value: &SnykContainerApplicationsItemLicensesPolicyOrgLicenseRulesArtistic20) -> Self {
        value.clone()
    }
}
impl ::std::default::Default
    for SnykContainerApplicationsItemLicensesPolicyOrgLicenseRulesArtistic20
{
    fn default() -> Self {
        Self {
            instructions: Default::default(),
            license_type: Default::default(),
            severity: Default::default(),
        }
    }
}
///`SnykContainerApplicationsItemLicensesPolicyOrgLicenseRulesCddl10`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "properties": {
///    "instructions": {
///      "type": "string"
///    },
///    "licenseType": {
///      "type": "string"
///    },
///    "severity": {
///      "type": "string"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct SnykContainerApplicationsItemLicensesPolicyOrgLicenseRulesCddl10 {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub instructions: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "licenseType",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub license_type: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub severity: ::std::option::Option<::std::string::String>,
}
impl ::std::convert::From<&SnykContainerApplicationsItemLicensesPolicyOrgLicenseRulesCddl10>
    for SnykContainerApplicationsItemLicensesPolicyOrgLicenseRulesCddl10
{
    fn from(value: &SnykContainerApplicationsItemLicensesPolicyOrgLicenseRulesCddl10) -> Self {
        value.clone()
    }
}
impl ::std::default::Default for SnykContainerApplicationsItemLicensesPolicyOrgLicenseRulesCddl10 {
    fn default() -> Self {
        Self {
            instructions: Default::default(),
            license_type: Default::default(),
            severity: Default::default(),
        }
    }
}
///`SnykContainerApplicationsItemLicensesPolicyOrgLicenseRulesCpol102`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "properties": {
///    "instructions": {
///      "type": "string"
///    },
///    "licenseType": {
///      "type": "string"
///    },
///    "severity": {
///      "type": "string"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct SnykContainerApplicationsItemLicensesPolicyOrgLicenseRulesCpol102 {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub instructions: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "licenseType",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub license_type: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub severity: ::std::option::Option<::std::string::String>,
}
impl ::std::convert::From<&SnykContainerApplicationsItemLicensesPolicyOrgLicenseRulesCpol102>
    for SnykContainerApplicationsItemLicensesPolicyOrgLicenseRulesCpol102
{
    fn from(value: &SnykContainerApplicationsItemLicensesPolicyOrgLicenseRulesCpol102) -> Self {
        value.clone()
    }
}
impl ::std::default::Default for SnykContainerApplicationsItemLicensesPolicyOrgLicenseRulesCpol102 {
    fn default() -> Self {
        Self {
            instructions: Default::default(),
            license_type: Default::default(),
            severity: Default::default(),
        }
    }
}
///`SnykContainerApplicationsItemLicensesPolicyOrgLicenseRulesEpl10`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "properties": {
///    "instructions": {
///      "type": "string"
///    },
///    "licenseType": {
///      "type": "string"
///    },
///    "severity": {
///      "type": "string"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct SnykContainerApplicationsItemLicensesPolicyOrgLicenseRulesEpl10 {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub instructions: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "licenseType",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub license_type: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub severity: ::std::option::Option<::std::string::String>,
}
impl ::std::convert::From<&SnykContainerApplicationsItemLicensesPolicyOrgLicenseRulesEpl10>
    for SnykContainerApplicationsItemLicensesPolicyOrgLicenseRulesEpl10
{
    fn from(value: &SnykContainerApplicationsItemLicensesPolicyOrgLicenseRulesEpl10) -> Self {
        value.clone()
    }
}
impl ::std::default::Default for SnykContainerApplicationsItemLicensesPolicyOrgLicenseRulesEpl10 {
    fn default() -> Self {
        Self {
            instructions: Default::default(),
            license_type: Default::default(),
            severity: Default::default(),
        }
    }
}
///`SnykContainerApplicationsItemLicensesPolicyOrgLicenseRulesGpl20`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "properties": {
///    "instructions": {
///      "type": "string"
///    },
///    "licenseType": {
///      "type": "string"
///    },
///    "severity": {
///      "type": "string"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct SnykContainerApplicationsItemLicensesPolicyOrgLicenseRulesGpl20 {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub instructions: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "licenseType",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub license_type: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub severity: ::std::option::Option<::std::string::String>,
}
impl ::std::convert::From<&SnykContainerApplicationsItemLicensesPolicyOrgLicenseRulesGpl20>
    for SnykContainerApplicationsItemLicensesPolicyOrgLicenseRulesGpl20
{
    fn from(value: &SnykContainerApplicationsItemLicensesPolicyOrgLicenseRulesGpl20) -> Self {
        value.clone()
    }
}
impl ::std::default::Default for SnykContainerApplicationsItemLicensesPolicyOrgLicenseRulesGpl20 {
    fn default() -> Self {
        Self {
            instructions: Default::default(),
            license_type: Default::default(),
            severity: Default::default(),
        }
    }
}
///`SnykContainerApplicationsItemLicensesPolicyOrgLicenseRulesGpl30`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "properties": {
///    "instructions": {
///      "type": "string"
///    },
///    "licenseType": {
///      "type": "string"
///    },
///    "severity": {
///      "type": "string"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct SnykContainerApplicationsItemLicensesPolicyOrgLicenseRulesGpl30 {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub instructions: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "licenseType",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub license_type: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub severity: ::std::option::Option<::std::string::String>,
}
impl ::std::convert::From<&SnykContainerApplicationsItemLicensesPolicyOrgLicenseRulesGpl30>
    for SnykContainerApplicationsItemLicensesPolicyOrgLicenseRulesGpl30
{
    fn from(value: &SnykContainerApplicationsItemLicensesPolicyOrgLicenseRulesGpl30) -> Self {
        value.clone()
    }
}
impl ::std::default::Default for SnykContainerApplicationsItemLicensesPolicyOrgLicenseRulesGpl30 {
    fn default() -> Self {
        Self {
            instructions: Default::default(),
            license_type: Default::default(),
            severity: Default::default(),
        }
    }
}
///`SnykContainerApplicationsItemLicensesPolicyOrgLicenseRulesLgpl20`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "properties": {
///    "instructions": {
///      "type": "string"
///    },
///    "licenseType": {
///      "type": "string"
///    },
///    "severity": {
///      "type": "string"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct SnykContainerApplicationsItemLicensesPolicyOrgLicenseRulesLgpl20 {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub instructions: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "licenseType",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub license_type: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub severity: ::std::option::Option<::std::string::String>,
}
impl ::std::convert::From<&SnykContainerApplicationsItemLicensesPolicyOrgLicenseRulesLgpl20>
    for SnykContainerApplicationsItemLicensesPolicyOrgLicenseRulesLgpl20
{
    fn from(value: &SnykContainerApplicationsItemLicensesPolicyOrgLicenseRulesLgpl20) -> Self {
        value.clone()
    }
}
impl ::std::default::Default for SnykContainerApplicationsItemLicensesPolicyOrgLicenseRulesLgpl20 {
    fn default() -> Self {
        Self {
            instructions: Default::default(),
            license_type: Default::default(),
            severity: Default::default(),
        }
    }
}
///`SnykContainerApplicationsItemLicensesPolicyOrgLicenseRulesLgpl21`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "properties": {
///    "instructions": {
///      "type": "string"
///    },
///    "licenseType": {
///      "type": "string"
///    },
///    "severity": {
///      "type": "string"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct SnykContainerApplicationsItemLicensesPolicyOrgLicenseRulesLgpl21 {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub instructions: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "licenseType",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub license_type: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub severity: ::std::option::Option<::std::string::String>,
}
impl ::std::convert::From<&SnykContainerApplicationsItemLicensesPolicyOrgLicenseRulesLgpl21>
    for SnykContainerApplicationsItemLicensesPolicyOrgLicenseRulesLgpl21
{
    fn from(value: &SnykContainerApplicationsItemLicensesPolicyOrgLicenseRulesLgpl21) -> Self {
        value.clone()
    }
}
impl ::std::default::Default for SnykContainerApplicationsItemLicensesPolicyOrgLicenseRulesLgpl21 {
    fn default() -> Self {
        Self {
            instructions: Default::default(),
            license_type: Default::default(),
            severity: Default::default(),
        }
    }
}
///`SnykContainerApplicationsItemLicensesPolicyOrgLicenseRulesLgpl30`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "properties": {
///    "instructions": {
///      "type": "string"
///    },
///    "licenseType": {
///      "type": "string"
///    },
///    "severity": {
///      "type": "string"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct SnykContainerApplicationsItemLicensesPolicyOrgLicenseRulesLgpl30 {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub instructions: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "licenseType",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub license_type: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub severity: ::std::option::Option<::std::string::String>,
}
impl ::std::convert::From<&SnykContainerApplicationsItemLicensesPolicyOrgLicenseRulesLgpl30>
    for SnykContainerApplicationsItemLicensesPolicyOrgLicenseRulesLgpl30
{
    fn from(value: &SnykContainerApplicationsItemLicensesPolicyOrgLicenseRulesLgpl30) -> Self {
        value.clone()
    }
}
impl ::std::default::Default for SnykContainerApplicationsItemLicensesPolicyOrgLicenseRulesLgpl30 {
    fn default() -> Self {
        Self {
            instructions: Default::default(),
            license_type: Default::default(),
            severity: Default::default(),
        }
    }
}
///`SnykContainerApplicationsItemLicensesPolicyOrgLicenseRulesMpl11`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "properties": {
///    "instructions": {
///      "type": "string"
///    },
///    "licenseType": {
///      "type": "string"
///    },
///    "severity": {
///      "type": "string"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct SnykContainerApplicationsItemLicensesPolicyOrgLicenseRulesMpl11 {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub instructions: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "licenseType",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub license_type: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub severity: ::std::option::Option<::std::string::String>,
}
impl ::std::convert::From<&SnykContainerApplicationsItemLicensesPolicyOrgLicenseRulesMpl11>
    for SnykContainerApplicationsItemLicensesPolicyOrgLicenseRulesMpl11
{
    fn from(value: &SnykContainerApplicationsItemLicensesPolicyOrgLicenseRulesMpl11) -> Self {
        value.clone()
    }
}
impl ::std::default::Default for SnykContainerApplicationsItemLicensesPolicyOrgLicenseRulesMpl11 {
    fn default() -> Self {
        Self {
            instructions: Default::default(),
            license_type: Default::default(),
            severity: Default::default(),
        }
    }
}
///`SnykContainerApplicationsItemLicensesPolicyOrgLicenseRulesMpl20`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "properties": {
///    "instructions": {
///      "type": "string"
///    },
///    "licenseType": {
///      "type": "string"
///    },
///    "severity": {
///      "type": "string"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct SnykContainerApplicationsItemLicensesPolicyOrgLicenseRulesMpl20 {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub instructions: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "licenseType",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub license_type: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub severity: ::std::option::Option<::std::string::String>,
}
impl ::std::convert::From<&SnykContainerApplicationsItemLicensesPolicyOrgLicenseRulesMpl20>
    for SnykContainerApplicationsItemLicensesPolicyOrgLicenseRulesMpl20
{
    fn from(value: &SnykContainerApplicationsItemLicensesPolicyOrgLicenseRulesMpl20) -> Self {
        value.clone()
    }
}
impl ::std::default::Default for SnykContainerApplicationsItemLicensesPolicyOrgLicenseRulesMpl20 {
    fn default() -> Self {
        Self {
            instructions: Default::default(),
            license_type: Default::default(),
            severity: Default::default(),
        }
    }
}
///`SnykContainerApplicationsItemLicensesPolicyOrgLicenseRulesMsRl`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "properties": {
///    "instructions": {
///      "type": "string"
///    },
///    "licenseType": {
///      "type": "string"
///    },
///    "severity": {
///      "type": "string"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct SnykContainerApplicationsItemLicensesPolicyOrgLicenseRulesMsRl {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub instructions: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "licenseType",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub license_type: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub severity: ::std::option::Option<::std::string::String>,
}
impl ::std::convert::From<&SnykContainerApplicationsItemLicensesPolicyOrgLicenseRulesMsRl>
    for SnykContainerApplicationsItemLicensesPolicyOrgLicenseRulesMsRl
{
    fn from(value: &SnykContainerApplicationsItemLicensesPolicyOrgLicenseRulesMsRl) -> Self {
        value.clone()
    }
}
impl ::std::default::Default for SnykContainerApplicationsItemLicensesPolicyOrgLicenseRulesMsRl {
    fn default() -> Self {
        Self {
            instructions: Default::default(),
            license_type: Default::default(),
            severity: Default::default(),
        }
    }
}
///`SnykContainerApplicationsItemLicensesPolicyOrgLicenseRulesSimPl20`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "properties": {
///    "instructions": {
///      "type": "string"
///    },
///    "licenseType": {
///      "type": "string"
///    },
///    "severity": {
///      "type": "string"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct SnykContainerApplicationsItemLicensesPolicyOrgLicenseRulesSimPl20 {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub instructions: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "licenseType",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub license_type: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub severity: ::std::option::Option<::std::string::String>,
}
impl ::std::convert::From<&SnykContainerApplicationsItemLicensesPolicyOrgLicenseRulesSimPl20>
    for SnykContainerApplicationsItemLicensesPolicyOrgLicenseRulesSimPl20
{
    fn from(value: &SnykContainerApplicationsItemLicensesPolicyOrgLicenseRulesSimPl20) -> Self {
        value.clone()
    }
}
impl ::std::default::Default for SnykContainerApplicationsItemLicensesPolicyOrgLicenseRulesSimPl20 {
    fn default() -> Self {
        Self {
            instructions: Default::default(),
            license_type: Default::default(),
            severity: Default::default(),
        }
    }
}
///`SnykContainerApplicationsItemRemediation`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "properties": {
///    "ignore": {
///      "type": "object"
///    },
///    "patch": {
///      "type": "object"
///    },
///    "pin": {
///      "type": "object"
///    },
///    "unresolved": {
///      "type": "array",
///      "items": {
///        "type": "object",
///        "properties": {
///          "creationTime": {
///            "type": "string"
///          },
///          "description": {
///            "type": "string"
///          },
///          "from": {
///            "type": "array",
///            "items": {
///              "type": "string"
///            }
///          },
///          "id": {
///            "type": "string"
///          },
///          "isPatchable": {
///            "type": "boolean"
///          },
///          "isPinnable": {
///            "type": "boolean"
///          },
///          "isRuntime": {
///            "type": "boolean"
///          },
///          "isUpgradable": {
///            "type": "boolean"
///          },
///          "language": {
///            "type": "string"
///          },
///          "license": {
///            "type": "string"
///          },
///          "name": {
///            "type": "string"
///          },
///          "packageManager": {
///            "type": "string"
///          },
///          "packageName": {
///            "type": "string"
///          },
///          "publicationTime": {
///            "type": "string"
///          },
///          "semver": {
///            "type": "object",
///            "properties": {
///              "vulnerable": {
///                "type": "array",
///                "items": {
///                  "type": "string"
///                }
///              }
///            }
///          },
///          "severity": {
///            "type": "string"
///          },
///          "severityWithCritical": {
///            "type": "string"
///          },
///          "title": {
///            "type": "string"
///          },
///          "type": {
///            "type": "string"
///          },
///          "upgradePath": {
///            "type": "array",
///            "items": {
///              "type": [
///                "boolean",
///                "string"
///              ]
///            }
///          },
///          "version": {
///            "type": "string"
///          }
///        }
///      }
///    },
///    "upgrade": {
///      "type": "object",
///      "properties": {
///        "ch.qos.logback:logback-core@1.5.13": {
///          "type": "object",
///          "properties": {
///            "upgradeTo": {
///              "type": "string"
///            },
///            "upgrades": {
///              "type": "array",
///              "items": {
///                "type": "string"
///              }
///            },
///            "vulns": {
///              "type": "array",
///              "items": {
///                "type": "string"
///              }
///            }
///          }
///        },
///        "org.apache.tomcat.embed:tomcat-embed-core@10.1.46": {
///          "type": "object",
///          "properties": {
///            "upgradeTo": {
///              "type": "string"
///            },
///            "upgrades": {
///              "type": "array",
///              "items": {
///                "type": "string"
///              }
///            },
///            "vulns": {
///              "type": "array",
///              "items": {
///                "type": "string"
///              }
///            }
///          }
///        }
///      }
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct SnykContainerApplicationsItemRemediation {
    #[serde(default, skip_serializing_if = "::serde_json::Map::is_empty")]
    pub ignore: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
    #[serde(default, skip_serializing_if = "::serde_json::Map::is_empty")]
    pub patch: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
    #[serde(default, skip_serializing_if = "::serde_json::Map::is_empty")]
    pub pin: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub unresolved: ::std::vec::Vec<SnykContainerApplicationsItemRemediationUnresolvedItem>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub upgrade: ::std::option::Option<SnykContainerApplicationsItemRemediationUpgrade>,
}
impl ::std::convert::From<&SnykContainerApplicationsItemRemediation>
    for SnykContainerApplicationsItemRemediation
{
    fn from(value: &SnykContainerApplicationsItemRemediation) -> Self {
        value.clone()
    }
}
impl ::std::default::Default for SnykContainerApplicationsItemRemediation {
    fn default() -> Self {
        Self {
            ignore: Default::default(),
            patch: Default::default(),
            pin: Default::default(),
            unresolved: Default::default(),
            upgrade: Default::default(),
        }
    }
}
///`SnykContainerApplicationsItemRemediationUnresolvedItem`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "properties": {
///    "creationTime": {
///      "type": "string"
///    },
///    "description": {
///      "type": "string"
///    },
///    "from": {
///      "type": "array",
///      "items": {
///        "type": "string"
///      }
///    },
///    "id": {
///      "type": "string"
///    },
///    "isPatchable": {
///      "type": "boolean"
///    },
///    "isPinnable": {
///      "type": "boolean"
///    },
///    "isRuntime": {
///      "type": "boolean"
///    },
///    "isUpgradable": {
///      "type": "boolean"
///    },
///    "language": {
///      "type": "string"
///    },
///    "license": {
///      "type": "string"
///    },
///    "name": {
///      "type": "string"
///    },
///    "packageManager": {
///      "type": "string"
///    },
///    "packageName": {
///      "type": "string"
///    },
///    "publicationTime": {
///      "type": "string"
///    },
///    "semver": {
///      "type": "object",
///      "properties": {
///        "vulnerable": {
///          "type": "array",
///          "items": {
///            "type": "string"
///          }
///        }
///      }
///    },
///    "severity": {
///      "type": "string"
///    },
///    "severityWithCritical": {
///      "type": "string"
///    },
///    "title": {
///      "type": "string"
///    },
///    "type": {
///      "type": "string"
///    },
///    "upgradePath": {
///      "type": "array",
///      "items": {
///        "type": [
///          "boolean",
///          "string"
///        ]
///      }
///    },
///    "version": {
///      "type": "string"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct SnykContainerApplicationsItemRemediationUnresolvedItem {
    #[serde(
        rename = "creationTime",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub creation_time: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub description: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub from: ::std::vec::Vec<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub id: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "isPatchable",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub is_patchable: ::std::option::Option<bool>,
    #[serde(
        rename = "isPinnable",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub is_pinnable: ::std::option::Option<bool>,
    #[serde(
        rename = "isRuntime",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub is_runtime: ::std::option::Option<bool>,
    #[serde(
        rename = "isUpgradable",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub is_upgradable: ::std::option::Option<bool>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub language: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub license: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "packageManager",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub package_manager: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "packageName",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub package_name: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "publicationTime",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub publication_time: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub semver: ::std::option::Option<SnykContainerApplicationsItemRemediationUnresolvedItemSemver>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub severity: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "severityWithCritical",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub severity_with_critical: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub title: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "type",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub type_: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "upgradePath",
        default,
        skip_serializing_if = "::std::vec::Vec::is_empty"
    )]
    pub upgrade_path:
        ::std::vec::Vec<SnykContainerApplicationsItemRemediationUnresolvedItemUpgradePathItem>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub version: ::std::option::Option<::std::string::String>,
}
impl ::std::convert::From<&SnykContainerApplicationsItemRemediationUnresolvedItem>
    for SnykContainerApplicationsItemRemediationUnresolvedItem
{
    fn from(value: &SnykContainerApplicationsItemRemediationUnresolvedItem) -> Self {
        value.clone()
    }
}
impl ::std::default::Default for SnykContainerApplicationsItemRemediationUnresolvedItem {
    fn default() -> Self {
        Self {
            creation_time: Default::default(),
            description: Default::default(),
            from: Default::default(),
            id: Default::default(),
            is_patchable: Default::default(),
            is_pinnable: Default::default(),
            is_runtime: Default::default(),
            is_upgradable: Default::default(),
            language: Default::default(),
            license: Default::default(),
            name: Default::default(),
            package_manager: Default::default(),
            package_name: Default::default(),
            publication_time: Default::default(),
            semver: Default::default(),
            severity: Default::default(),
            severity_with_critical: Default::default(),
            title: Default::default(),
            type_: Default::default(),
            upgrade_path: Default::default(),
            version: Default::default(),
        }
    }
}
///`SnykContainerApplicationsItemRemediationUnresolvedItemSemver`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "properties": {
///    "vulnerable": {
///      "type": "array",
///      "items": {
///        "type": "string"
///      }
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct SnykContainerApplicationsItemRemediationUnresolvedItemSemver {
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub vulnerable: ::std::vec::Vec<::std::string::String>,
}
impl ::std::convert::From<&SnykContainerApplicationsItemRemediationUnresolvedItemSemver>
    for SnykContainerApplicationsItemRemediationUnresolvedItemSemver
{
    fn from(value: &SnykContainerApplicationsItemRemediationUnresolvedItemSemver) -> Self {
        value.clone()
    }
}
impl ::std::default::Default for SnykContainerApplicationsItemRemediationUnresolvedItemSemver {
    fn default() -> Self {
        Self {
            vulnerable: Default::default(),
        }
    }
}
///`SnykContainerApplicationsItemRemediationUnresolvedItemUpgradePathItem`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": [
///    "boolean",
///    "string"
///  ]
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(untagged)]
pub enum SnykContainerApplicationsItemRemediationUnresolvedItemUpgradePathItem {
    Boolean(bool),
    String(::std::string::String),
}
impl ::std::convert::From<&Self>
    for SnykContainerApplicationsItemRemediationUnresolvedItemUpgradePathItem
{
    fn from(value: &SnykContainerApplicationsItemRemediationUnresolvedItemUpgradePathItem) -> Self {
        value.clone()
    }
}
impl ::std::fmt::Display for SnykContainerApplicationsItemRemediationUnresolvedItemUpgradePathItem {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match self {
            Self::Boolean(x) => x.fmt(f),
            Self::String(x) => x.fmt(f),
        }
    }
}
impl ::std::convert::From<bool>
    for SnykContainerApplicationsItemRemediationUnresolvedItemUpgradePathItem
{
    fn from(value: bool) -> Self {
        Self::Boolean(value)
    }
}
///`SnykContainerApplicationsItemRemediationUpgrade`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "properties": {
///    "ch.qos.logback:logback-core@1.5.13": {
///      "type": "object",
///      "properties": {
///        "upgradeTo": {
///          "type": "string"
///        },
///        "upgrades": {
///          "type": "array",
///          "items": {
///            "type": "string"
///          }
///        },
///        "vulns": {
///          "type": "array",
///          "items": {
///            "type": "string"
///          }
///        }
///      }
///    },
///    "org.apache.tomcat.embed:tomcat-embed-core@10.1.46": {
///      "type": "object",
///      "properties": {
///        "upgradeTo": {
///          "type": "string"
///        },
///        "upgrades": {
///          "type": "array",
///          "items": {
///            "type": "string"
///          }
///        },
///        "vulns": {
///          "type": "array",
///          "items": {
///            "type": "string"
///          }
///        }
///      }
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct SnykContainerApplicationsItemRemediationUpgrade {
    #[serde(
        rename = "ch.qos.logback:logback-core@1.5.13",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub ch_qos_logback_logback_core_1_5_13: ::std::option::Option<
        SnykContainerApplicationsItemRemediationUpgradeChQosLogbackLogbackCore1513,
    >,
    #[serde(
        rename = "org.apache.tomcat.embed:tomcat-embed-core@10.1.46",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub org_apache_tomcat_embed_tomcat_embed_core_10_1_46: ::std::option::Option<
        SnykContainerApplicationsItemRemediationUpgradeOrgApacheTomcatEmbedTomcatEmbedCore10146,
    >,
}
impl ::std::convert::From<&SnykContainerApplicationsItemRemediationUpgrade>
    for SnykContainerApplicationsItemRemediationUpgrade
{
    fn from(value: &SnykContainerApplicationsItemRemediationUpgrade) -> Self {
        value.clone()
    }
}
impl ::std::default::Default for SnykContainerApplicationsItemRemediationUpgrade {
    fn default() -> Self {
        Self {
            ch_qos_logback_logback_core_1_5_13: Default::default(),
            org_apache_tomcat_embed_tomcat_embed_core_10_1_46: Default::default(),
        }
    }
}
///`SnykContainerApplicationsItemRemediationUpgradeChQosLogbackLogbackCore1513`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "properties": {
///    "upgradeTo": {
///      "type": "string"
///    },
///    "upgrades": {
///      "type": "array",
///      "items": {
///        "type": "string"
///      }
///    },
///    "vulns": {
///      "type": "array",
///      "items": {
///        "type": "string"
///      }
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct SnykContainerApplicationsItemRemediationUpgradeChQosLogbackLogbackCore1513 {
    #[serde(
        rename = "upgradeTo",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub upgrade_to: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub upgrades: ::std::vec::Vec<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub vulns: ::std::vec::Vec<::std::string::String>,
}
impl
    ::std::convert::From<
        &SnykContainerApplicationsItemRemediationUpgradeChQosLogbackLogbackCore1513,
    > for SnykContainerApplicationsItemRemediationUpgradeChQosLogbackLogbackCore1513
{
    fn from(
        value: &SnykContainerApplicationsItemRemediationUpgradeChQosLogbackLogbackCore1513,
    ) -> Self {
        value.clone()
    }
}
impl ::std::default::Default
    for SnykContainerApplicationsItemRemediationUpgradeChQosLogbackLogbackCore1513
{
    fn default() -> Self {
        Self {
            upgrade_to: Default::default(),
            upgrades: Default::default(),
            vulns: Default::default(),
        }
    }
}
///`SnykContainerApplicationsItemRemediationUpgradeOrgApacheTomcatEmbedTomcatEmbedCore10146`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "properties": {
///    "upgradeTo": {
///      "type": "string"
///    },
///    "upgrades": {
///      "type": "array",
///      "items": {
///        "type": "string"
///      }
///    },
///    "vulns": {
///      "type": "array",
///      "items": {
///        "type": "string"
///      }
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct SnykContainerApplicationsItemRemediationUpgradeOrgApacheTomcatEmbedTomcatEmbedCore10146 {
    #[serde(
        rename = "upgradeTo",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub upgrade_to: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub upgrades: ::std::vec::Vec<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub vulns: ::std::vec::Vec<::std::string::String>,
}
impl
    ::std::convert::From<
        &SnykContainerApplicationsItemRemediationUpgradeOrgApacheTomcatEmbedTomcatEmbedCore10146,
    > for SnykContainerApplicationsItemRemediationUpgradeOrgApacheTomcatEmbedTomcatEmbedCore10146
{
    fn from(
        value: &SnykContainerApplicationsItemRemediationUpgradeOrgApacheTomcatEmbedTomcatEmbedCore10146,
    ) -> Self {
        value.clone()
    }
}
impl ::std::default::Default
    for SnykContainerApplicationsItemRemediationUpgradeOrgApacheTomcatEmbedTomcatEmbedCore10146
{
    fn default() -> Self {
        Self {
            upgrade_to: Default::default(),
            upgrades: Default::default(),
            vulns: Default::default(),
        }
    }
}
///`SnykContainerApplicationsItemVulnerabilitiesItem`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "properties": {
///    "CVSSv3": {
///      "type": "string"
///    },
///    "alternativeIds": {
///      "type": "array"
///    },
///    "creationTime": {
///      "type": "string"
///    },
///    "credit": {
///      "type": "array",
///      "items": {
///        "type": "string"
///      }
///    },
///    "cvssDetails": {
///      "type": "array",
///      "items": {
///        "type": "object",
///        "properties": {
///          "assigner": {
///            "type": "string"
///          },
///          "cvssV3BaseScore": {
///            "type": "number"
///          },
///          "cvssV3Vector": {
///            "type": "string"
///          },
///          "modificationTime": {
///            "type": "string"
///          },
///          "severity": {
///            "type": "string"
///          }
///        }
///      }
///    },
///    "cvssScore": {
///      "type": "number"
///    },
///    "cvssSources": {
///      "type": "array",
///      "items": {
///        "type": "object",
///        "properties": {
///          "assigner": {
///            "type": "string"
///          },
///          "baseScore": {
///            "type": "number"
///          },
///          "cvssVersion": {
///            "type": "string"
///          },
///          "modificationTime": {
///            "type": "string"
///          },
///          "severity": {
///            "type": "string"
///          },
///          "type": {
///            "type": "string"
///          },
///          "vector": {
///            "type": "string"
///          }
///        }
///      }
///    },
///    "description": {
///      "type": "string"
///    },
///    "disclosureTime": {
///      "type": "string"
///    },
///    "epssDetails": {
///      "type": "object",
///      "properties": {
///        "modelVersion": {
///          "type": "string"
///        },
///        "percentile": {
///          "type": "string"
///        },
///        "probability": {
///          "type": "string"
///        }
///      }
///    },
///    "exploit": {
///      "type": "string"
///    },
///    "exploitDetails": {
///      "type": "object",
///      "properties": {
///        "maturityLevels": {
///          "type": "array",
///          "items": {
///            "type": "object",
///            "properties": {
///              "format": {
///                "type": "string"
///              },
///              "level": {
///                "type": "string"
///              },
///              "type": {
///                "type": "string"
///              }
///            }
///          }
///        },
///        "sources": {
///          "type": "array"
///        }
///      }
///    },
///    "fixedIn": {
///      "type": "array",
///      "items": {
///        "type": "string"
///      }
///    },
///    "from": {
///      "type": "array",
///      "items": {
///        "type": "string"
///      }
///    },
///    "functions": {
///      "type": "array"
///    },
///    "functions_new": {
///      "type": "array"
///    },
///    "id": {
///      "type": "string"
///    },
///    "identifiers": {
///      "type": "object",
///      "properties": {
///        "CVE": {
///          "type": "array",
///          "items": {
///            "type": "string"
///          }
///        },
///        "CWE": {
///          "type": "array",
///          "items": {
///            "type": "string"
///          }
///        }
///      }
///    },
///    "insights": {
///      "type": "object",
///      "properties": {
///        "triageAdvice": {
///          "type": "null"
///        }
///      }
///    },
///    "isDisputed": {
///      "type": "boolean"
///    },
///    "isPatchable": {
///      "type": "boolean"
///    },
///    "isUpgradable": {
///      "type": "boolean"
///    },
///    "language": {
///      "type": "string"
///    },
///    "malicious": {
///      "type": "boolean"
///    },
///    "mavenModuleName": {
///      "type": "object",
///      "properties": {
///        "artifactId": {
///          "type": "string"
///        },
///        "groupId": {
///          "type": "string"
///        }
///      }
///    },
///    "modificationTime": {
///      "type": "string"
///    },
///    "moduleName": {
///      "type": "string"
///    },
///    "name": {
///      "type": "string"
///    },
///    "packageManager": {
///      "type": "string"
///    },
///    "packageName": {
///      "type": "string"
///    },
///    "patches": {
///      "type": "array"
///    },
///    "proprietary": {
///      "type": "boolean"
///    },
///    "publicationTime": {
///      "type": "string"
///    },
///    "references": {
///      "type": "array",
///      "items": {
///        "type": "object",
///        "properties": {
///          "title": {
///            "type": "string"
///          },
///          "url": {
///            "type": "string"
///          }
///        }
///      }
///    },
///    "semver": {
///      "type": "object",
///      "properties": {
///        "vulnerable": {
///          "type": "array",
///          "items": {
///            "type": "string"
///          }
///        }
///      }
///    },
///    "severity": {
///      "type": "string"
///    },
///    "severityBasedOn": {
///      "type": "string"
///    },
///    "severityWithCritical": {
///      "type": "string"
///    },
///    "socialTrendAlert": {
///      "type": "boolean"
///    },
///    "title": {
///      "type": "string"
///    },
///    "upgradePath": {
///      "type": "array",
///      "items": {
///        "type": [
///          "boolean",
///          "string"
///        ]
///      }
///    },
///    "version": {
///      "type": "string"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct SnykContainerApplicationsItemVulnerabilitiesItem {
    #[serde(
        rename = "alternativeIds",
        default,
        skip_serializing_if = "::std::vec::Vec::is_empty"
    )]
    pub alternative_ids: ::std::vec::Vec<::serde_json::Value>,
    #[serde(
        rename = "creationTime",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub creation_time: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub credit: ::std::vec::Vec<::std::string::String>,
    #[serde(
        rename = "CVSSv3",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub cvs_sv3: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "cvssDetails",
        default,
        skip_serializing_if = "::std::vec::Vec::is_empty"
    )]
    pub cvss_details:
        ::std::vec::Vec<SnykContainerApplicationsItemVulnerabilitiesItemCvssDetailsItem>,
    #[serde(
        rename = "cvssScore",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub cvss_score: ::std::option::Option<f64>,
    #[serde(
        rename = "cvssSources",
        default,
        skip_serializing_if = "::std::vec::Vec::is_empty"
    )]
    pub cvss_sources:
        ::std::vec::Vec<SnykContainerApplicationsItemVulnerabilitiesItemCvssSourcesItem>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub description: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "disclosureTime",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub disclosure_time: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "epssDetails",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub epss_details:
        ::std::option::Option<SnykContainerApplicationsItemVulnerabilitiesItemEpssDetails>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub exploit: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "exploitDetails",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub exploit_details:
        ::std::option::Option<SnykContainerApplicationsItemVulnerabilitiesItemExploitDetails>,
    #[serde(
        rename = "fixedIn",
        default,
        skip_serializing_if = "::std::vec::Vec::is_empty"
    )]
    pub fixed_in: ::std::vec::Vec<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub from: ::std::vec::Vec<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub functions: ::std::vec::Vec<::serde_json::Value>,
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub functions_new: ::std::vec::Vec<::serde_json::Value>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub id: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub identifiers:
        ::std::option::Option<SnykContainerApplicationsItemVulnerabilitiesItemIdentifiers>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub insights: ::std::option::Option<SnykContainerApplicationsItemVulnerabilitiesItemInsights>,
    #[serde(
        rename = "isDisputed",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub is_disputed: ::std::option::Option<bool>,
    #[serde(
        rename = "isPatchable",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub is_patchable: ::std::option::Option<bool>,
    #[serde(
        rename = "isUpgradable",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub is_upgradable: ::std::option::Option<bool>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub language: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub malicious: ::std::option::Option<bool>,
    #[serde(
        rename = "mavenModuleName",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub maven_module_name:
        ::std::option::Option<SnykContainerApplicationsItemVulnerabilitiesItemMavenModuleName>,
    #[serde(
        rename = "modificationTime",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub modification_time: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "moduleName",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub module_name: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "packageManager",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub package_manager: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "packageName",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub package_name: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub patches: ::std::vec::Vec<::serde_json::Value>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub proprietary: ::std::option::Option<bool>,
    #[serde(
        rename = "publicationTime",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub publication_time: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub references: ::std::vec::Vec<SnykContainerApplicationsItemVulnerabilitiesItemReferencesItem>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub semver: ::std::option::Option<SnykContainerApplicationsItemVulnerabilitiesItemSemver>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub severity: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "severityBasedOn",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub severity_based_on: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "severityWithCritical",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub severity_with_critical: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "socialTrendAlert",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub social_trend_alert: ::std::option::Option<bool>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub title: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "upgradePath",
        default,
        skip_serializing_if = "::std::vec::Vec::is_empty"
    )]
    pub upgrade_path:
        ::std::vec::Vec<SnykContainerApplicationsItemVulnerabilitiesItemUpgradePathItem>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub version: ::std::option::Option<::std::string::String>,
}
impl ::std::convert::From<&SnykContainerApplicationsItemVulnerabilitiesItem>
    for SnykContainerApplicationsItemVulnerabilitiesItem
{
    fn from(value: &SnykContainerApplicationsItemVulnerabilitiesItem) -> Self {
        value.clone()
    }
}
impl ::std::default::Default for SnykContainerApplicationsItemVulnerabilitiesItem {
    fn default() -> Self {
        Self {
            alternative_ids: Default::default(),
            creation_time: Default::default(),
            credit: Default::default(),
            cvs_sv3: Default::default(),
            cvss_details: Default::default(),
            cvss_score: Default::default(),
            cvss_sources: Default::default(),
            description: Default::default(),
            disclosure_time: Default::default(),
            epss_details: Default::default(),
            exploit: Default::default(),
            exploit_details: Default::default(),
            fixed_in: Default::default(),
            from: Default::default(),
            functions: Default::default(),
            functions_new: Default::default(),
            id: Default::default(),
            identifiers: Default::default(),
            insights: Default::default(),
            is_disputed: Default::default(),
            is_patchable: Default::default(),
            is_upgradable: Default::default(),
            language: Default::default(),
            malicious: Default::default(),
            maven_module_name: Default::default(),
            modification_time: Default::default(),
            module_name: Default::default(),
            name: Default::default(),
            package_manager: Default::default(),
            package_name: Default::default(),
            patches: Default::default(),
            proprietary: Default::default(),
            publication_time: Default::default(),
            references: Default::default(),
            semver: Default::default(),
            severity: Default::default(),
            severity_based_on: Default::default(),
            severity_with_critical: Default::default(),
            social_trend_alert: Default::default(),
            title: Default::default(),
            upgrade_path: Default::default(),
            version: Default::default(),
        }
    }
}
///`SnykContainerApplicationsItemVulnerabilitiesItemCvssDetailsItem`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "properties": {
///    "assigner": {
///      "type": "string"
///    },
///    "cvssV3BaseScore": {
///      "type": "number"
///    },
///    "cvssV3Vector": {
///      "type": "string"
///    },
///    "modificationTime": {
///      "type": "string"
///    },
///    "severity": {
///      "type": "string"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct SnykContainerApplicationsItemVulnerabilitiesItemCvssDetailsItem {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub assigner: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "cvssV3BaseScore",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub cvss_v3_base_score: ::std::option::Option<f64>,
    #[serde(
        rename = "cvssV3Vector",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub cvss_v3_vector: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "modificationTime",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub modification_time: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub severity: ::std::option::Option<::std::string::String>,
}
impl ::std::convert::From<&SnykContainerApplicationsItemVulnerabilitiesItemCvssDetailsItem>
    for SnykContainerApplicationsItemVulnerabilitiesItemCvssDetailsItem
{
    fn from(value: &SnykContainerApplicationsItemVulnerabilitiesItemCvssDetailsItem) -> Self {
        value.clone()
    }
}
impl ::std::default::Default for SnykContainerApplicationsItemVulnerabilitiesItemCvssDetailsItem {
    fn default() -> Self {
        Self {
            assigner: Default::default(),
            cvss_v3_base_score: Default::default(),
            cvss_v3_vector: Default::default(),
            modification_time: Default::default(),
            severity: Default::default(),
        }
    }
}
///`SnykContainerApplicationsItemVulnerabilitiesItemCvssSourcesItem`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "properties": {
///    "assigner": {
///      "type": "string"
///    },
///    "baseScore": {
///      "type": "number"
///    },
///    "cvssVersion": {
///      "type": "string"
///    },
///    "modificationTime": {
///      "type": "string"
///    },
///    "severity": {
///      "type": "string"
///    },
///    "type": {
///      "type": "string"
///    },
///    "vector": {
///      "type": "string"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct SnykContainerApplicationsItemVulnerabilitiesItemCvssSourcesItem {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub assigner: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "baseScore",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub base_score: ::std::option::Option<f64>,
    #[serde(
        rename = "cvssVersion",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub cvss_version: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "modificationTime",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub modification_time: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub severity: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "type",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub type_: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub vector: ::std::option::Option<::std::string::String>,
}
impl ::std::convert::From<&SnykContainerApplicationsItemVulnerabilitiesItemCvssSourcesItem>
    for SnykContainerApplicationsItemVulnerabilitiesItemCvssSourcesItem
{
    fn from(value: &SnykContainerApplicationsItemVulnerabilitiesItemCvssSourcesItem) -> Self {
        value.clone()
    }
}
impl ::std::default::Default for SnykContainerApplicationsItemVulnerabilitiesItemCvssSourcesItem {
    fn default() -> Self {
        Self {
            assigner: Default::default(),
            base_score: Default::default(),
            cvss_version: Default::default(),
            modification_time: Default::default(),
            severity: Default::default(),
            type_: Default::default(),
            vector: Default::default(),
        }
    }
}
///`SnykContainerApplicationsItemVulnerabilitiesItemEpssDetails`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "properties": {
///    "modelVersion": {
///      "type": "string"
///    },
///    "percentile": {
///      "type": "string"
///    },
///    "probability": {
///      "type": "string"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct SnykContainerApplicationsItemVulnerabilitiesItemEpssDetails {
    #[serde(
        rename = "modelVersion",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub model_version: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub percentile: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub probability: ::std::option::Option<::std::string::String>,
}
impl ::std::convert::From<&SnykContainerApplicationsItemVulnerabilitiesItemEpssDetails>
    for SnykContainerApplicationsItemVulnerabilitiesItemEpssDetails
{
    fn from(value: &SnykContainerApplicationsItemVulnerabilitiesItemEpssDetails) -> Self {
        value.clone()
    }
}
impl ::std::default::Default for SnykContainerApplicationsItemVulnerabilitiesItemEpssDetails {
    fn default() -> Self {
        Self {
            model_version: Default::default(),
            percentile: Default::default(),
            probability: Default::default(),
        }
    }
}
///`SnykContainerApplicationsItemVulnerabilitiesItemExploitDetails`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "properties": {
///    "maturityLevels": {
///      "type": "array",
///      "items": {
///        "type": "object",
///        "properties": {
///          "format": {
///            "type": "string"
///          },
///          "level": {
///            "type": "string"
///          },
///          "type": {
///            "type": "string"
///          }
///        }
///      }
///    },
///    "sources": {
///      "type": "array"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct SnykContainerApplicationsItemVulnerabilitiesItemExploitDetails {
    #[serde(
        rename = "maturityLevels",
        default,
        skip_serializing_if = "::std::vec::Vec::is_empty"
    )]
    pub maturity_levels: ::std::vec::Vec<
        SnykContainerApplicationsItemVulnerabilitiesItemExploitDetailsMaturityLevelsItem,
    >,
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub sources: ::std::vec::Vec<::serde_json::Value>,
}
impl ::std::convert::From<&SnykContainerApplicationsItemVulnerabilitiesItemExploitDetails>
    for SnykContainerApplicationsItemVulnerabilitiesItemExploitDetails
{
    fn from(value: &SnykContainerApplicationsItemVulnerabilitiesItemExploitDetails) -> Self {
        value.clone()
    }
}
impl ::std::default::Default for SnykContainerApplicationsItemVulnerabilitiesItemExploitDetails {
    fn default() -> Self {
        Self {
            maturity_levels: Default::default(),
            sources: Default::default(),
        }
    }
}
///`SnykContainerApplicationsItemVulnerabilitiesItemExploitDetailsMaturityLevelsItem`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "properties": {
///    "format": {
///      "type": "string"
///    },
///    "level": {
///      "type": "string"
///    },
///    "type": {
///      "type": "string"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct SnykContainerApplicationsItemVulnerabilitiesItemExploitDetailsMaturityLevelsItem {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub format: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub level: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "type",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub type_: ::std::option::Option<::std::string::String>,
}
impl
    ::std::convert::From<
        &SnykContainerApplicationsItemVulnerabilitiesItemExploitDetailsMaturityLevelsItem,
    > for SnykContainerApplicationsItemVulnerabilitiesItemExploitDetailsMaturityLevelsItem
{
    fn from(
        value: &SnykContainerApplicationsItemVulnerabilitiesItemExploitDetailsMaturityLevelsItem,
    ) -> Self {
        value.clone()
    }
}
impl ::std::default::Default
    for SnykContainerApplicationsItemVulnerabilitiesItemExploitDetailsMaturityLevelsItem
{
    fn default() -> Self {
        Self {
            format: Default::default(),
            level: Default::default(),
            type_: Default::default(),
        }
    }
}
///`SnykContainerApplicationsItemVulnerabilitiesItemIdentifiers`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "properties": {
///    "CVE": {
///      "type": "array",
///      "items": {
///        "type": "string"
///      }
///    },
///    "CWE": {
///      "type": "array",
///      "items": {
///        "type": "string"
///      }
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct SnykContainerApplicationsItemVulnerabilitiesItemIdentifiers {
    #[serde(
        rename = "CVE",
        default,
        skip_serializing_if = "::std::vec::Vec::is_empty"
    )]
    pub cve: ::std::vec::Vec<::std::string::String>,
    #[serde(
        rename = "CWE",
        default,
        skip_serializing_if = "::std::vec::Vec::is_empty"
    )]
    pub cwe: ::std::vec::Vec<::std::string::String>,
}
impl ::std::convert::From<&SnykContainerApplicationsItemVulnerabilitiesItemIdentifiers>
    for SnykContainerApplicationsItemVulnerabilitiesItemIdentifiers
{
    fn from(value: &SnykContainerApplicationsItemVulnerabilitiesItemIdentifiers) -> Self {
        value.clone()
    }
}
impl ::std::default::Default for SnykContainerApplicationsItemVulnerabilitiesItemIdentifiers {
    fn default() -> Self {
        Self {
            cve: Default::default(),
            cwe: Default::default(),
        }
    }
}
///`SnykContainerApplicationsItemVulnerabilitiesItemInsights`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "properties": {
///    "triageAdvice": {
///      "type": "null"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct SnykContainerApplicationsItemVulnerabilitiesItemInsights {
    #[serde(rename = "triageAdvice", default)]
    pub triage_advice: (),
}
impl ::std::convert::From<&SnykContainerApplicationsItemVulnerabilitiesItemInsights>
    for SnykContainerApplicationsItemVulnerabilitiesItemInsights
{
    fn from(value: &SnykContainerApplicationsItemVulnerabilitiesItemInsights) -> Self {
        value.clone()
    }
}
impl ::std::default::Default for SnykContainerApplicationsItemVulnerabilitiesItemInsights {
    fn default() -> Self {
        Self {
            triage_advice: Default::default(),
        }
    }
}
///`SnykContainerApplicationsItemVulnerabilitiesItemMavenModuleName`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "properties": {
///    "artifactId": {
///      "type": "string"
///    },
///    "groupId": {
///      "type": "string"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct SnykContainerApplicationsItemVulnerabilitiesItemMavenModuleName {
    #[serde(
        rename = "artifactId",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub artifact_id: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "groupId",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub group_id: ::std::option::Option<::std::string::String>,
}
impl ::std::convert::From<&SnykContainerApplicationsItemVulnerabilitiesItemMavenModuleName>
    for SnykContainerApplicationsItemVulnerabilitiesItemMavenModuleName
{
    fn from(value: &SnykContainerApplicationsItemVulnerabilitiesItemMavenModuleName) -> Self {
        value.clone()
    }
}
impl ::std::default::Default for SnykContainerApplicationsItemVulnerabilitiesItemMavenModuleName {
    fn default() -> Self {
        Self {
            artifact_id: Default::default(),
            group_id: Default::default(),
        }
    }
}
///`SnykContainerApplicationsItemVulnerabilitiesItemReferencesItem`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "properties": {
///    "title": {
///      "type": "string"
///    },
///    "url": {
///      "type": "string"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct SnykContainerApplicationsItemVulnerabilitiesItemReferencesItem {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub title: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub url: ::std::option::Option<::std::string::String>,
}
impl ::std::convert::From<&SnykContainerApplicationsItemVulnerabilitiesItemReferencesItem>
    for SnykContainerApplicationsItemVulnerabilitiesItemReferencesItem
{
    fn from(value: &SnykContainerApplicationsItemVulnerabilitiesItemReferencesItem) -> Self {
        value.clone()
    }
}
impl ::std::default::Default for SnykContainerApplicationsItemVulnerabilitiesItemReferencesItem {
    fn default() -> Self {
        Self {
            title: Default::default(),
            url: Default::default(),
        }
    }
}
///`SnykContainerApplicationsItemVulnerabilitiesItemSemver`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "properties": {
///    "vulnerable": {
///      "type": "array",
///      "items": {
///        "type": "string"
///      }
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct SnykContainerApplicationsItemVulnerabilitiesItemSemver {
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub vulnerable: ::std::vec::Vec<::std::string::String>,
}
impl ::std::convert::From<&SnykContainerApplicationsItemVulnerabilitiesItemSemver>
    for SnykContainerApplicationsItemVulnerabilitiesItemSemver
{
    fn from(value: &SnykContainerApplicationsItemVulnerabilitiesItemSemver) -> Self {
        value.clone()
    }
}
impl ::std::default::Default for SnykContainerApplicationsItemVulnerabilitiesItemSemver {
    fn default() -> Self {
        Self {
            vulnerable: Default::default(),
        }
    }
}
///`SnykContainerApplicationsItemVulnerabilitiesItemUpgradePathItem`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": [
///    "boolean",
///    "string"
///  ]
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(untagged)]
pub enum SnykContainerApplicationsItemVulnerabilitiesItemUpgradePathItem {
    Boolean(bool),
    String(::std::string::String),
}
impl ::std::convert::From<&Self>
    for SnykContainerApplicationsItemVulnerabilitiesItemUpgradePathItem
{
    fn from(value: &SnykContainerApplicationsItemVulnerabilitiesItemUpgradePathItem) -> Self {
        value.clone()
    }
}
impl ::std::fmt::Display for SnykContainerApplicationsItemVulnerabilitiesItemUpgradePathItem {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match self {
            Self::Boolean(x) => x.fmt(f),
            Self::String(x) => x.fmt(f),
        }
    }
}
impl ::std::convert::From<bool>
    for SnykContainerApplicationsItemVulnerabilitiesItemUpgradePathItem
{
    fn from(value: bool) -> Self {
        Self::Boolean(value)
    }
}
///`SnykContainerDocker`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "properties": {
///    "baseImage": {
///      "type": "string"
///    },
///    "baseImageRemediation": {
///      "type": "object",
///      "properties": {
///        "advice": {
///          "type": "array",
///          "items": {
///            "type": "object",
///            "properties": {
///              "bold": {
///                "type": "boolean"
///              },
///              "message": {
///                "type": "string"
///              }
///            }
///          }
///        },
///        "code": {
///          "type": "string"
///        }
///      }
///    },
///    "binariesVulns": {
///      "type": "object",
///      "properties": {
///        "affectedPkgs": {
///          "type": "object"
///        },
///        "issuesData": {
///          "type": "object"
///        }
///      }
///    },
///    "os": {
///      "type": "object",
///      "properties": {
///        "prettyName": {
///          "type": "string"
///        }
///      }
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct SnykContainerDocker {
    #[serde(
        rename = "baseImage",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub base_image: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "baseImageRemediation",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub base_image_remediation: ::std::option::Option<SnykContainerDockerBaseImageRemediation>,
    #[serde(
        rename = "binariesVulns",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub binaries_vulns: ::std::option::Option<SnykContainerDockerBinariesVulns>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub os: ::std::option::Option<SnykContainerDockerOs>,
}
impl ::std::convert::From<&SnykContainerDocker> for SnykContainerDocker {
    fn from(value: &SnykContainerDocker) -> Self {
        value.clone()
    }
}
impl ::std::default::Default for SnykContainerDocker {
    fn default() -> Self {
        Self {
            base_image: Default::default(),
            base_image_remediation: Default::default(),
            binaries_vulns: Default::default(),
            os: Default::default(),
        }
    }
}
///`SnykContainerDockerBaseImageRemediation`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "properties": {
///    "advice": {
///      "type": "array",
///      "items": {
///        "type": "object",
///        "properties": {
///          "bold": {
///            "type": "boolean"
///          },
///          "message": {
///            "type": "string"
///          }
///        }
///      }
///    },
///    "code": {
///      "type": "string"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct SnykContainerDockerBaseImageRemediation {
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub advice: ::std::vec::Vec<SnykContainerDockerBaseImageRemediationAdviceItem>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub code: ::std::option::Option<::std::string::String>,
}
impl ::std::convert::From<&SnykContainerDockerBaseImageRemediation>
    for SnykContainerDockerBaseImageRemediation
{
    fn from(value: &SnykContainerDockerBaseImageRemediation) -> Self {
        value.clone()
    }
}
impl ::std::default::Default for SnykContainerDockerBaseImageRemediation {
    fn default() -> Self {
        Self {
            advice: Default::default(),
            code: Default::default(),
        }
    }
}
///`SnykContainerDockerBaseImageRemediationAdviceItem`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "properties": {
///    "bold": {
///      "type": "boolean"
///    },
///    "message": {
///      "type": "string"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct SnykContainerDockerBaseImageRemediationAdviceItem {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub bold: ::std::option::Option<bool>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub message: ::std::option::Option<::std::string::String>,
}
impl ::std::convert::From<&SnykContainerDockerBaseImageRemediationAdviceItem>
    for SnykContainerDockerBaseImageRemediationAdviceItem
{
    fn from(value: &SnykContainerDockerBaseImageRemediationAdviceItem) -> Self {
        value.clone()
    }
}
impl ::std::default::Default for SnykContainerDockerBaseImageRemediationAdviceItem {
    fn default() -> Self {
        Self {
            bold: Default::default(),
            message: Default::default(),
        }
    }
}
///`SnykContainerDockerBinariesVulns`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "properties": {
///    "affectedPkgs": {
///      "type": "object"
///    },
///    "issuesData": {
///      "type": "object"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct SnykContainerDockerBinariesVulns {
    #[serde(
        rename = "affectedPkgs",
        default,
        skip_serializing_if = "::serde_json::Map::is_empty"
    )]
    pub affected_pkgs: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
    #[serde(
        rename = "issuesData",
        default,
        skip_serializing_if = "::serde_json::Map::is_empty"
    )]
    pub issues_data: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
}
impl ::std::convert::From<&SnykContainerDockerBinariesVulns> for SnykContainerDockerBinariesVulns {
    fn from(value: &SnykContainerDockerBinariesVulns) -> Self {
        value.clone()
    }
}
impl ::std::default::Default for SnykContainerDockerBinariesVulns {
    fn default() -> Self {
        Self {
            affected_pkgs: Default::default(),
            issues_data: Default::default(),
        }
    }
}
///`SnykContainerDockerOs`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "properties": {
///    "prettyName": {
///      "type": "string"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct SnykContainerDockerOs {
    #[serde(
        rename = "prettyName",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub pretty_name: ::std::option::Option<::std::string::String>,
}
impl ::std::convert::From<&SnykContainerDockerOs> for SnykContainerDockerOs {
    fn from(value: &SnykContainerDockerOs) -> Self {
        value.clone()
    }
}
impl ::std::default::Default for SnykContainerDockerOs {
    fn default() -> Self {
        Self {
            pretty_name: Default::default(),
        }
    }
}
///`SnykContainerFiltered`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "properties": {
///    "ignore": {
///      "type": "array"
///    },
///    "patch": {
///      "type": "array"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct SnykContainerFiltered {
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub ignore: ::std::vec::Vec<::serde_json::Value>,
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub patch: ::std::vec::Vec<::serde_json::Value>,
}
impl ::std::convert::From<&SnykContainerFiltered> for SnykContainerFiltered {
    fn from(value: &SnykContainerFiltered) -> Self {
        value.clone()
    }
}
impl ::std::default::Default for SnykContainerFiltered {
    fn default() -> Self {
        Self {
            ignore: Default::default(),
            patch: Default::default(),
        }
    }
}
///`SnykContainerIgnoreSettings`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "properties": {
///    "adminOnly": {
///      "type": "boolean"
///    },
///    "disregardFilesystemIgnores": {
///      "type": "boolean"
///    },
///    "reasonRequired": {
///      "type": "boolean"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct SnykContainerIgnoreSettings {
    #[serde(
        rename = "adminOnly",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub admin_only: ::std::option::Option<bool>,
    #[serde(
        rename = "disregardFilesystemIgnores",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub disregard_filesystem_ignores: ::std::option::Option<bool>,
    #[serde(
        rename = "reasonRequired",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub reason_required: ::std::option::Option<bool>,
}
impl ::std::convert::From<&SnykContainerIgnoreSettings> for SnykContainerIgnoreSettings {
    fn from(value: &SnykContainerIgnoreSettings) -> Self {
        value.clone()
    }
}
impl ::std::default::Default for SnykContainerIgnoreSettings {
    fn default() -> Self {
        Self {
            admin_only: Default::default(),
            disregard_filesystem_ignores: Default::default(),
            reason_required: Default::default(),
        }
    }
}
///`SnykContainerLicensesPolicy`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "properties": {
///    "orgLicenseRules": {
///      "type": "object",
///      "properties": {
///        "AGPL-1.0": {
///          "type": "object",
///          "properties": {
///            "instructions": {
///              "type": "string"
///            },
///            "licenseType": {
///              "type": "string"
///            },
///            "severity": {
///              "type": "string"
///            }
///          }
///        },
///        "AGPL-3.0": {
///          "type": "object",
///          "properties": {
///            "instructions": {
///              "type": "string"
///            },
///            "licenseType": {
///              "type": "string"
///            },
///            "severity": {
///              "type": "string"
///            }
///          }
///        },
///        "Artistic-1.0": {
///          "type": "object",
///          "properties": {
///            "instructions": {
///              "type": "string"
///            },
///            "licenseType": {
///              "type": "string"
///            },
///            "severity": {
///              "type": "string"
///            }
///          }
///        },
///        "Artistic-2.0": {
///          "type": "object",
///          "properties": {
///            "instructions": {
///              "type": "string"
///            },
///            "licenseType": {
///              "type": "string"
///            },
///            "severity": {
///              "type": "string"
///            }
///          }
///        },
///        "CDDL-1.0": {
///          "type": "object",
///          "properties": {
///            "instructions": {
///              "type": "string"
///            },
///            "licenseType": {
///              "type": "string"
///            },
///            "severity": {
///              "type": "string"
///            }
///          }
///        },
///        "CPOL-1.02": {
///          "type": "object",
///          "properties": {
///            "instructions": {
///              "type": "string"
///            },
///            "licenseType": {
///              "type": "string"
///            },
///            "severity": {
///              "type": "string"
///            }
///          }
///        },
///        "EPL-1.0": {
///          "type": "object",
///          "properties": {
///            "instructions": {
///              "type": "string"
///            },
///            "licenseType": {
///              "type": "string"
///            },
///            "severity": {
///              "type": "string"
///            }
///          }
///        },
///        "GPL-2.0": {
///          "type": "object",
///          "properties": {
///            "instructions": {
///              "type": "string"
///            },
///            "licenseType": {
///              "type": "string"
///            },
///            "severity": {
///              "type": "string"
///            }
///          }
///        },
///        "GPL-3.0": {
///          "type": "object",
///          "properties": {
///            "instructions": {
///              "type": "string"
///            },
///            "licenseType": {
///              "type": "string"
///            },
///            "severity": {
///              "type": "string"
///            }
///          }
///        },
///        "LGPL-2.0": {
///          "type": "object",
///          "properties": {
///            "instructions": {
///              "type": "string"
///            },
///            "licenseType": {
///              "type": "string"
///            },
///            "severity": {
///              "type": "string"
///            }
///          }
///        },
///        "LGPL-2.1": {
///          "type": "object",
///          "properties": {
///            "instructions": {
///              "type": "string"
///            },
///            "licenseType": {
///              "type": "string"
///            },
///            "severity": {
///              "type": "string"
///            }
///          }
///        },
///        "LGPL-3.0": {
///          "type": "object",
///          "properties": {
///            "instructions": {
///              "type": "string"
///            },
///            "licenseType": {
///              "type": "string"
///            },
///            "severity": {
///              "type": "string"
///            }
///          }
///        },
///        "MPL-1.1": {
///          "type": "object",
///          "properties": {
///            "instructions": {
///              "type": "string"
///            },
///            "licenseType": {
///              "type": "string"
///            },
///            "severity": {
///              "type": "string"
///            }
///          }
///        },
///        "MPL-2.0": {
///          "type": "object",
///          "properties": {
///            "instructions": {
///              "type": "string"
///            },
///            "licenseType": {
///              "type": "string"
///            },
///            "severity": {
///              "type": "string"
///            }
///          }
///        },
///        "MS-RL": {
///          "type": "object",
///          "properties": {
///            "instructions": {
///              "type": "string"
///            },
///            "licenseType": {
///              "type": "string"
///            },
///            "severity": {
///              "type": "string"
///            }
///          }
///        },
///        "SimPL-2.0": {
///          "type": "object",
///          "properties": {
///            "instructions": {
///              "type": "string"
///            },
///            "licenseType": {
///              "type": "string"
///            },
///            "severity": {
///              "type": "string"
///            }
///          }
///        }
///      }
///    },
///    "severities": {
///      "type": "object"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct SnykContainerLicensesPolicy {
    #[serde(
        rename = "orgLicenseRules",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub org_license_rules: ::std::option::Option<SnykContainerLicensesPolicyOrgLicenseRules>,
    #[serde(default, skip_serializing_if = "::serde_json::Map::is_empty")]
    pub severities: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
}
impl ::std::convert::From<&SnykContainerLicensesPolicy> for SnykContainerLicensesPolicy {
    fn from(value: &SnykContainerLicensesPolicy) -> Self {
        value.clone()
    }
}
impl ::std::default::Default for SnykContainerLicensesPolicy {
    fn default() -> Self {
        Self {
            org_license_rules: Default::default(),
            severities: Default::default(),
        }
    }
}
///`SnykContainerLicensesPolicyOrgLicenseRules`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "properties": {
///    "AGPL-1.0": {
///      "type": "object",
///      "properties": {
///        "instructions": {
///          "type": "string"
///        },
///        "licenseType": {
///          "type": "string"
///        },
///        "severity": {
///          "type": "string"
///        }
///      }
///    },
///    "AGPL-3.0": {
///      "type": "object",
///      "properties": {
///        "instructions": {
///          "type": "string"
///        },
///        "licenseType": {
///          "type": "string"
///        },
///        "severity": {
///          "type": "string"
///        }
///      }
///    },
///    "Artistic-1.0": {
///      "type": "object",
///      "properties": {
///        "instructions": {
///          "type": "string"
///        },
///        "licenseType": {
///          "type": "string"
///        },
///        "severity": {
///          "type": "string"
///        }
///      }
///    },
///    "Artistic-2.0": {
///      "type": "object",
///      "properties": {
///        "instructions": {
///          "type": "string"
///        },
///        "licenseType": {
///          "type": "string"
///        },
///        "severity": {
///          "type": "string"
///        }
///      }
///    },
///    "CDDL-1.0": {
///      "type": "object",
///      "properties": {
///        "instructions": {
///          "type": "string"
///        },
///        "licenseType": {
///          "type": "string"
///        },
///        "severity": {
///          "type": "string"
///        }
///      }
///    },
///    "CPOL-1.02": {
///      "type": "object",
///      "properties": {
///        "instructions": {
///          "type": "string"
///        },
///        "licenseType": {
///          "type": "string"
///        },
///        "severity": {
///          "type": "string"
///        }
///      }
///    },
///    "EPL-1.0": {
///      "type": "object",
///      "properties": {
///        "instructions": {
///          "type": "string"
///        },
///        "licenseType": {
///          "type": "string"
///        },
///        "severity": {
///          "type": "string"
///        }
///      }
///    },
///    "GPL-2.0": {
///      "type": "object",
///      "properties": {
///        "instructions": {
///          "type": "string"
///        },
///        "licenseType": {
///          "type": "string"
///        },
///        "severity": {
///          "type": "string"
///        }
///      }
///    },
///    "GPL-3.0": {
///      "type": "object",
///      "properties": {
///        "instructions": {
///          "type": "string"
///        },
///        "licenseType": {
///          "type": "string"
///        },
///        "severity": {
///          "type": "string"
///        }
///      }
///    },
///    "LGPL-2.0": {
///      "type": "object",
///      "properties": {
///        "instructions": {
///          "type": "string"
///        },
///        "licenseType": {
///          "type": "string"
///        },
///        "severity": {
///          "type": "string"
///        }
///      }
///    },
///    "LGPL-2.1": {
///      "type": "object",
///      "properties": {
///        "instructions": {
///          "type": "string"
///        },
///        "licenseType": {
///          "type": "string"
///        },
///        "severity": {
///          "type": "string"
///        }
///      }
///    },
///    "LGPL-3.0": {
///      "type": "object",
///      "properties": {
///        "instructions": {
///          "type": "string"
///        },
///        "licenseType": {
///          "type": "string"
///        },
///        "severity": {
///          "type": "string"
///        }
///      }
///    },
///    "MPL-1.1": {
///      "type": "object",
///      "properties": {
///        "instructions": {
///          "type": "string"
///        },
///        "licenseType": {
///          "type": "string"
///        },
///        "severity": {
///          "type": "string"
///        }
///      }
///    },
///    "MPL-2.0": {
///      "type": "object",
///      "properties": {
///        "instructions": {
///          "type": "string"
///        },
///        "licenseType": {
///          "type": "string"
///        },
///        "severity": {
///          "type": "string"
///        }
///      }
///    },
///    "MS-RL": {
///      "type": "object",
///      "properties": {
///        "instructions": {
///          "type": "string"
///        },
///        "licenseType": {
///          "type": "string"
///        },
///        "severity": {
///          "type": "string"
///        }
///      }
///    },
///    "SimPL-2.0": {
///      "type": "object",
///      "properties": {
///        "instructions": {
///          "type": "string"
///        },
///        "licenseType": {
///          "type": "string"
///        },
///        "severity": {
///          "type": "string"
///        }
///      }
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct SnykContainerLicensesPolicyOrgLicenseRules {
    #[serde(
        rename = "AGPL-1.0",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub agpl_1_0: ::std::option::Option<SnykContainerLicensesPolicyOrgLicenseRulesAgpl10>,
    #[serde(
        rename = "AGPL-3.0",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub agpl_3_0: ::std::option::Option<SnykContainerLicensesPolicyOrgLicenseRulesAgpl30>,
    #[serde(
        rename = "Artistic-1.0",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub artistic_1_0: ::std::option::Option<SnykContainerLicensesPolicyOrgLicenseRulesArtistic10>,
    #[serde(
        rename = "Artistic-2.0",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub artistic_2_0: ::std::option::Option<SnykContainerLicensesPolicyOrgLicenseRulesArtistic20>,
    #[serde(
        rename = "CDDL-1.0",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub cddl_1_0: ::std::option::Option<SnykContainerLicensesPolicyOrgLicenseRulesCddl10>,
    #[serde(
        rename = "CPOL-1.02",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub cpol_1_02: ::std::option::Option<SnykContainerLicensesPolicyOrgLicenseRulesCpol102>,
    #[serde(
        rename = "EPL-1.0",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub epl_1_0: ::std::option::Option<SnykContainerLicensesPolicyOrgLicenseRulesEpl10>,
    #[serde(
        rename = "GPL-2.0",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub gpl_2_0: ::std::option::Option<SnykContainerLicensesPolicyOrgLicenseRulesGpl20>,
    #[serde(
        rename = "GPL-3.0",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub gpl_3_0: ::std::option::Option<SnykContainerLicensesPolicyOrgLicenseRulesGpl30>,
    #[serde(
        rename = "LGPL-2.0",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub lgpl_2_0: ::std::option::Option<SnykContainerLicensesPolicyOrgLicenseRulesLgpl20>,
    #[serde(
        rename = "LGPL-2.1",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub lgpl_2_1: ::std::option::Option<SnykContainerLicensesPolicyOrgLicenseRulesLgpl21>,
    #[serde(
        rename = "LGPL-3.0",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub lgpl_3_0: ::std::option::Option<SnykContainerLicensesPolicyOrgLicenseRulesLgpl30>,
    #[serde(
        rename = "MPL-1.1",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub mpl_1_1: ::std::option::Option<SnykContainerLicensesPolicyOrgLicenseRulesMpl11>,
    #[serde(
        rename = "MPL-2.0",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub mpl_2_0: ::std::option::Option<SnykContainerLicensesPolicyOrgLicenseRulesMpl20>,
    #[serde(
        rename = "MS-RL",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub ms_rl: ::std::option::Option<SnykContainerLicensesPolicyOrgLicenseRulesMsRl>,
    #[serde(
        rename = "SimPL-2.0",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub sim_pl_2_0: ::std::option::Option<SnykContainerLicensesPolicyOrgLicenseRulesSimPl20>,
}
impl ::std::convert::From<&SnykContainerLicensesPolicyOrgLicenseRules>
    for SnykContainerLicensesPolicyOrgLicenseRules
{
    fn from(value: &SnykContainerLicensesPolicyOrgLicenseRules) -> Self {
        value.clone()
    }
}
impl ::std::default::Default for SnykContainerLicensesPolicyOrgLicenseRules {
    fn default() -> Self {
        Self {
            agpl_1_0: Default::default(),
            agpl_3_0: Default::default(),
            artistic_1_0: Default::default(),
            artistic_2_0: Default::default(),
            cddl_1_0: Default::default(),
            cpol_1_02: Default::default(),
            epl_1_0: Default::default(),
            gpl_2_0: Default::default(),
            gpl_3_0: Default::default(),
            lgpl_2_0: Default::default(),
            lgpl_2_1: Default::default(),
            lgpl_3_0: Default::default(),
            mpl_1_1: Default::default(),
            mpl_2_0: Default::default(),
            ms_rl: Default::default(),
            sim_pl_2_0: Default::default(),
        }
    }
}
///`SnykContainerLicensesPolicyOrgLicenseRulesAgpl10`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "properties": {
///    "instructions": {
///      "type": "string"
///    },
///    "licenseType": {
///      "type": "string"
///    },
///    "severity": {
///      "type": "string"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct SnykContainerLicensesPolicyOrgLicenseRulesAgpl10 {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub instructions: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "licenseType",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub license_type: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub severity: ::std::option::Option<::std::string::String>,
}
impl ::std::convert::From<&SnykContainerLicensesPolicyOrgLicenseRulesAgpl10>
    for SnykContainerLicensesPolicyOrgLicenseRulesAgpl10
{
    fn from(value: &SnykContainerLicensesPolicyOrgLicenseRulesAgpl10) -> Self {
        value.clone()
    }
}
impl ::std::default::Default for SnykContainerLicensesPolicyOrgLicenseRulesAgpl10 {
    fn default() -> Self {
        Self {
            instructions: Default::default(),
            license_type: Default::default(),
            severity: Default::default(),
        }
    }
}
///`SnykContainerLicensesPolicyOrgLicenseRulesAgpl30`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "properties": {
///    "instructions": {
///      "type": "string"
///    },
///    "licenseType": {
///      "type": "string"
///    },
///    "severity": {
///      "type": "string"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct SnykContainerLicensesPolicyOrgLicenseRulesAgpl30 {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub instructions: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "licenseType",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub license_type: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub severity: ::std::option::Option<::std::string::String>,
}
impl ::std::convert::From<&SnykContainerLicensesPolicyOrgLicenseRulesAgpl30>
    for SnykContainerLicensesPolicyOrgLicenseRulesAgpl30
{
    fn from(value: &SnykContainerLicensesPolicyOrgLicenseRulesAgpl30) -> Self {
        value.clone()
    }
}
impl ::std::default::Default for SnykContainerLicensesPolicyOrgLicenseRulesAgpl30 {
    fn default() -> Self {
        Self {
            instructions: Default::default(),
            license_type: Default::default(),
            severity: Default::default(),
        }
    }
}
///`SnykContainerLicensesPolicyOrgLicenseRulesArtistic10`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "properties": {
///    "instructions": {
///      "type": "string"
///    },
///    "licenseType": {
///      "type": "string"
///    },
///    "severity": {
///      "type": "string"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct SnykContainerLicensesPolicyOrgLicenseRulesArtistic10 {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub instructions: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "licenseType",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub license_type: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub severity: ::std::option::Option<::std::string::String>,
}
impl ::std::convert::From<&SnykContainerLicensesPolicyOrgLicenseRulesArtistic10>
    for SnykContainerLicensesPolicyOrgLicenseRulesArtistic10
{
    fn from(value: &SnykContainerLicensesPolicyOrgLicenseRulesArtistic10) -> Self {
        value.clone()
    }
}
impl ::std::default::Default for SnykContainerLicensesPolicyOrgLicenseRulesArtistic10 {
    fn default() -> Self {
        Self {
            instructions: Default::default(),
            license_type: Default::default(),
            severity: Default::default(),
        }
    }
}
///`SnykContainerLicensesPolicyOrgLicenseRulesArtistic20`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "properties": {
///    "instructions": {
///      "type": "string"
///    },
///    "licenseType": {
///      "type": "string"
///    },
///    "severity": {
///      "type": "string"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct SnykContainerLicensesPolicyOrgLicenseRulesArtistic20 {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub instructions: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "licenseType",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub license_type: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub severity: ::std::option::Option<::std::string::String>,
}
impl ::std::convert::From<&SnykContainerLicensesPolicyOrgLicenseRulesArtistic20>
    for SnykContainerLicensesPolicyOrgLicenseRulesArtistic20
{
    fn from(value: &SnykContainerLicensesPolicyOrgLicenseRulesArtistic20) -> Self {
        value.clone()
    }
}
impl ::std::default::Default for SnykContainerLicensesPolicyOrgLicenseRulesArtistic20 {
    fn default() -> Self {
        Self {
            instructions: Default::default(),
            license_type: Default::default(),
            severity: Default::default(),
        }
    }
}
///`SnykContainerLicensesPolicyOrgLicenseRulesCddl10`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "properties": {
///    "instructions": {
///      "type": "string"
///    },
///    "licenseType": {
///      "type": "string"
///    },
///    "severity": {
///      "type": "string"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct SnykContainerLicensesPolicyOrgLicenseRulesCddl10 {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub instructions: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "licenseType",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub license_type: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub severity: ::std::option::Option<::std::string::String>,
}
impl ::std::convert::From<&SnykContainerLicensesPolicyOrgLicenseRulesCddl10>
    for SnykContainerLicensesPolicyOrgLicenseRulesCddl10
{
    fn from(value: &SnykContainerLicensesPolicyOrgLicenseRulesCddl10) -> Self {
        value.clone()
    }
}
impl ::std::default::Default for SnykContainerLicensesPolicyOrgLicenseRulesCddl10 {
    fn default() -> Self {
        Self {
            instructions: Default::default(),
            license_type: Default::default(),
            severity: Default::default(),
        }
    }
}
///`SnykContainerLicensesPolicyOrgLicenseRulesCpol102`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "properties": {
///    "instructions": {
///      "type": "string"
///    },
///    "licenseType": {
///      "type": "string"
///    },
///    "severity": {
///      "type": "string"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct SnykContainerLicensesPolicyOrgLicenseRulesCpol102 {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub instructions: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "licenseType",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub license_type: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub severity: ::std::option::Option<::std::string::String>,
}
impl ::std::convert::From<&SnykContainerLicensesPolicyOrgLicenseRulesCpol102>
    for SnykContainerLicensesPolicyOrgLicenseRulesCpol102
{
    fn from(value: &SnykContainerLicensesPolicyOrgLicenseRulesCpol102) -> Self {
        value.clone()
    }
}
impl ::std::default::Default for SnykContainerLicensesPolicyOrgLicenseRulesCpol102 {
    fn default() -> Self {
        Self {
            instructions: Default::default(),
            license_type: Default::default(),
            severity: Default::default(),
        }
    }
}
///`SnykContainerLicensesPolicyOrgLicenseRulesEpl10`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "properties": {
///    "instructions": {
///      "type": "string"
///    },
///    "licenseType": {
///      "type": "string"
///    },
///    "severity": {
///      "type": "string"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct SnykContainerLicensesPolicyOrgLicenseRulesEpl10 {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub instructions: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "licenseType",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub license_type: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub severity: ::std::option::Option<::std::string::String>,
}
impl ::std::convert::From<&SnykContainerLicensesPolicyOrgLicenseRulesEpl10>
    for SnykContainerLicensesPolicyOrgLicenseRulesEpl10
{
    fn from(value: &SnykContainerLicensesPolicyOrgLicenseRulesEpl10) -> Self {
        value.clone()
    }
}
impl ::std::default::Default for SnykContainerLicensesPolicyOrgLicenseRulesEpl10 {
    fn default() -> Self {
        Self {
            instructions: Default::default(),
            license_type: Default::default(),
            severity: Default::default(),
        }
    }
}
///`SnykContainerLicensesPolicyOrgLicenseRulesGpl20`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "properties": {
///    "instructions": {
///      "type": "string"
///    },
///    "licenseType": {
///      "type": "string"
///    },
///    "severity": {
///      "type": "string"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct SnykContainerLicensesPolicyOrgLicenseRulesGpl20 {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub instructions: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "licenseType",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub license_type: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub severity: ::std::option::Option<::std::string::String>,
}
impl ::std::convert::From<&SnykContainerLicensesPolicyOrgLicenseRulesGpl20>
    for SnykContainerLicensesPolicyOrgLicenseRulesGpl20
{
    fn from(value: &SnykContainerLicensesPolicyOrgLicenseRulesGpl20) -> Self {
        value.clone()
    }
}
impl ::std::default::Default for SnykContainerLicensesPolicyOrgLicenseRulesGpl20 {
    fn default() -> Self {
        Self {
            instructions: Default::default(),
            license_type: Default::default(),
            severity: Default::default(),
        }
    }
}
///`SnykContainerLicensesPolicyOrgLicenseRulesGpl30`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "properties": {
///    "instructions": {
///      "type": "string"
///    },
///    "licenseType": {
///      "type": "string"
///    },
///    "severity": {
///      "type": "string"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct SnykContainerLicensesPolicyOrgLicenseRulesGpl30 {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub instructions: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "licenseType",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub license_type: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub severity: ::std::option::Option<::std::string::String>,
}
impl ::std::convert::From<&SnykContainerLicensesPolicyOrgLicenseRulesGpl30>
    for SnykContainerLicensesPolicyOrgLicenseRulesGpl30
{
    fn from(value: &SnykContainerLicensesPolicyOrgLicenseRulesGpl30) -> Self {
        value.clone()
    }
}
impl ::std::default::Default for SnykContainerLicensesPolicyOrgLicenseRulesGpl30 {
    fn default() -> Self {
        Self {
            instructions: Default::default(),
            license_type: Default::default(),
            severity: Default::default(),
        }
    }
}
///`SnykContainerLicensesPolicyOrgLicenseRulesLgpl20`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "properties": {
///    "instructions": {
///      "type": "string"
///    },
///    "licenseType": {
///      "type": "string"
///    },
///    "severity": {
///      "type": "string"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct SnykContainerLicensesPolicyOrgLicenseRulesLgpl20 {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub instructions: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "licenseType",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub license_type: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub severity: ::std::option::Option<::std::string::String>,
}
impl ::std::convert::From<&SnykContainerLicensesPolicyOrgLicenseRulesLgpl20>
    for SnykContainerLicensesPolicyOrgLicenseRulesLgpl20
{
    fn from(value: &SnykContainerLicensesPolicyOrgLicenseRulesLgpl20) -> Self {
        value.clone()
    }
}
impl ::std::default::Default for SnykContainerLicensesPolicyOrgLicenseRulesLgpl20 {
    fn default() -> Self {
        Self {
            instructions: Default::default(),
            license_type: Default::default(),
            severity: Default::default(),
        }
    }
}
///`SnykContainerLicensesPolicyOrgLicenseRulesLgpl21`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "properties": {
///    "instructions": {
///      "type": "string"
///    },
///    "licenseType": {
///      "type": "string"
///    },
///    "severity": {
///      "type": "string"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct SnykContainerLicensesPolicyOrgLicenseRulesLgpl21 {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub instructions: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "licenseType",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub license_type: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub severity: ::std::option::Option<::std::string::String>,
}
impl ::std::convert::From<&SnykContainerLicensesPolicyOrgLicenseRulesLgpl21>
    for SnykContainerLicensesPolicyOrgLicenseRulesLgpl21
{
    fn from(value: &SnykContainerLicensesPolicyOrgLicenseRulesLgpl21) -> Self {
        value.clone()
    }
}
impl ::std::default::Default for SnykContainerLicensesPolicyOrgLicenseRulesLgpl21 {
    fn default() -> Self {
        Self {
            instructions: Default::default(),
            license_type: Default::default(),
            severity: Default::default(),
        }
    }
}
///`SnykContainerLicensesPolicyOrgLicenseRulesLgpl30`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "properties": {
///    "instructions": {
///      "type": "string"
///    },
///    "licenseType": {
///      "type": "string"
///    },
///    "severity": {
///      "type": "string"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct SnykContainerLicensesPolicyOrgLicenseRulesLgpl30 {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub instructions: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "licenseType",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub license_type: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub severity: ::std::option::Option<::std::string::String>,
}
impl ::std::convert::From<&SnykContainerLicensesPolicyOrgLicenseRulesLgpl30>
    for SnykContainerLicensesPolicyOrgLicenseRulesLgpl30
{
    fn from(value: &SnykContainerLicensesPolicyOrgLicenseRulesLgpl30) -> Self {
        value.clone()
    }
}
impl ::std::default::Default for SnykContainerLicensesPolicyOrgLicenseRulesLgpl30 {
    fn default() -> Self {
        Self {
            instructions: Default::default(),
            license_type: Default::default(),
            severity: Default::default(),
        }
    }
}
///`SnykContainerLicensesPolicyOrgLicenseRulesMpl11`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "properties": {
///    "instructions": {
///      "type": "string"
///    },
///    "licenseType": {
///      "type": "string"
///    },
///    "severity": {
///      "type": "string"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct SnykContainerLicensesPolicyOrgLicenseRulesMpl11 {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub instructions: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "licenseType",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub license_type: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub severity: ::std::option::Option<::std::string::String>,
}
impl ::std::convert::From<&SnykContainerLicensesPolicyOrgLicenseRulesMpl11>
    for SnykContainerLicensesPolicyOrgLicenseRulesMpl11
{
    fn from(value: &SnykContainerLicensesPolicyOrgLicenseRulesMpl11) -> Self {
        value.clone()
    }
}
impl ::std::default::Default for SnykContainerLicensesPolicyOrgLicenseRulesMpl11 {
    fn default() -> Self {
        Self {
            instructions: Default::default(),
            license_type: Default::default(),
            severity: Default::default(),
        }
    }
}
///`SnykContainerLicensesPolicyOrgLicenseRulesMpl20`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "properties": {
///    "instructions": {
///      "type": "string"
///    },
///    "licenseType": {
///      "type": "string"
///    },
///    "severity": {
///      "type": "string"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct SnykContainerLicensesPolicyOrgLicenseRulesMpl20 {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub instructions: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "licenseType",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub license_type: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub severity: ::std::option::Option<::std::string::String>,
}
impl ::std::convert::From<&SnykContainerLicensesPolicyOrgLicenseRulesMpl20>
    for SnykContainerLicensesPolicyOrgLicenseRulesMpl20
{
    fn from(value: &SnykContainerLicensesPolicyOrgLicenseRulesMpl20) -> Self {
        value.clone()
    }
}
impl ::std::default::Default for SnykContainerLicensesPolicyOrgLicenseRulesMpl20 {
    fn default() -> Self {
        Self {
            instructions: Default::default(),
            license_type: Default::default(),
            severity: Default::default(),
        }
    }
}
///`SnykContainerLicensesPolicyOrgLicenseRulesMsRl`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "properties": {
///    "instructions": {
///      "type": "string"
///    },
///    "licenseType": {
///      "type": "string"
///    },
///    "severity": {
///      "type": "string"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct SnykContainerLicensesPolicyOrgLicenseRulesMsRl {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub instructions: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "licenseType",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub license_type: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub severity: ::std::option::Option<::std::string::String>,
}
impl ::std::convert::From<&SnykContainerLicensesPolicyOrgLicenseRulesMsRl>
    for SnykContainerLicensesPolicyOrgLicenseRulesMsRl
{
    fn from(value: &SnykContainerLicensesPolicyOrgLicenseRulesMsRl) -> Self {
        value.clone()
    }
}
impl ::std::default::Default for SnykContainerLicensesPolicyOrgLicenseRulesMsRl {
    fn default() -> Self {
        Self {
            instructions: Default::default(),
            license_type: Default::default(),
            severity: Default::default(),
        }
    }
}
///`SnykContainerLicensesPolicyOrgLicenseRulesSimPl20`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "properties": {
///    "instructions": {
///      "type": "string"
///    },
///    "licenseType": {
///      "type": "string"
///    },
///    "severity": {
///      "type": "string"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct SnykContainerLicensesPolicyOrgLicenseRulesSimPl20 {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub instructions: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "licenseType",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub license_type: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub severity: ::std::option::Option<::std::string::String>,
}
impl ::std::convert::From<&SnykContainerLicensesPolicyOrgLicenseRulesSimPl20>
    for SnykContainerLicensesPolicyOrgLicenseRulesSimPl20
{
    fn from(value: &SnykContainerLicensesPolicyOrgLicenseRulesSimPl20) -> Self {
        value.clone()
    }
}
impl ::std::default::Default for SnykContainerLicensesPolicyOrgLicenseRulesSimPl20 {
    fn default() -> Self {
        Self {
            instructions: Default::default(),
            license_type: Default::default(),
            severity: Default::default(),
        }
    }
}
///`SnykContainerVulnerabilitiesItem`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "properties": {
///    "CVSSv3": {
///      "type": "string"
///    },
///    "cpes": {
///      "type": "array"
///    },
///    "creationTime": {
///      "type": "string"
///    },
///    "credit": {
///      "type": "array",
///      "items": {
///        "type": "string"
///      }
///    },
///    "cvssDetails": {
///      "type": "array",
///      "items": {
///        "type": "object",
///        "properties": {
///          "assigner": {
///            "type": "string"
///          },
///          "cvssV3BaseScore": {
///            "type": "number"
///          },
///          "cvssV3Vector": {
///            "type": "string"
///          },
///          "modificationTime": {
///            "type": "string"
///          },
///          "severity": {
///            "type": "string"
///          }
///        }
///      }
///    },
///    "cvssScore": {
///      "type": "number"
///    },
///    "cvssSources": {
///      "type": "array",
///      "items": {
///        "type": "object",
///        "properties": {
///          "assigner": {
///            "type": "string"
///          },
///          "baseScore": {
///            "type": "number"
///          },
///          "cvssVersion": {
///            "type": "string"
///          },
///          "modificationTime": {
///            "type": "string"
///          },
///          "severity": {
///            "type": "string"
///          },
///          "type": {
///            "type": "string"
///          },
///          "vector": {
///            "type": "string"
///          }
///        }
///      }
///    },
///    "description": {
///      "type": "string"
///    },
///    "disclosureTime": {
///      "type": "string"
///    },
///    "dockerBaseImage": {
///      "type": "string"
///    },
///    "epssDetails": {
///      "type": "object",
///      "properties": {
///        "modelVersion": {
///          "type": "string"
///        },
///        "percentile": {
///          "type": "string"
///        },
///        "probability": {
///          "type": "string"
///        }
///      }
///    },
///    "exploit": {
///      "type": "string"
///    },
///    "exploitDetails": {
///      "type": "object",
///      "properties": {
///        "maturityLevels": {
///          "type": "array",
///          "items": {
///            "type": "object",
///            "properties": {
///              "format": {
///                "type": "string"
///              },
///              "level": {
///                "type": "string"
///              },
///              "type": {
///                "type": "string"
///              }
///            }
///          }
///        },
///        "sources": {
///          "type": "array"
///        }
///      }
///    },
///    "fixedIn": {
///      "type": "array"
///    },
///    "from": {
///      "type": "array",
///      "items": {
///        "type": "string"
///      }
///    },
///    "id": {
///      "type": "string"
///    },
///    "identifiers": {
///      "type": "object",
///      "properties": {
///        "ALTERNATIVE": {
///          "type": "array"
///        },
///        "CVE": {
///          "type": "array",
///          "items": {
///            "type": "string"
///          }
///        },
///        "CWE": {
///          "type": "array",
///          "items": {
///            "type": "string"
///          }
///        }
///      }
///    },
///    "insights": {
///      "type": "object",
///      "properties": {
///        "triageAdvice": {
///          "type": "null"
///        }
///      }
///    },
///    "isDisputed": {
///      "type": "boolean"
///    },
///    "isPatchable": {
///      "type": "boolean"
///    },
///    "isUpgradable": {
///      "type": "boolean"
///    },
///    "language": {
///      "type": "string"
///    },
///    "malicious": {
///      "type": "boolean"
///    },
///    "modificationTime": {
///      "type": "string"
///    },
///    "name": {
///      "type": "string"
///    },
///    "nvdSeverity": {
///      "type": "string"
///    },
///    "packageManager": {
///      "type": "string"
///    },
///    "packageName": {
///      "type": "string"
///    },
///    "patches": {
///      "type": "array"
///    },
///    "publicationTime": {
///      "type": "string"
///    },
///    "references": {
///      "type": "array",
///      "items": {
///        "type": "object",
///        "properties": {
///          "title": {
///            "type": "string"
///          },
///          "url": {
///            "type": "string"
///          }
///        }
///      }
///    },
///    "relativeImportance": {
///      "type": "string"
///    },
///    "semver": {
///      "type": "object",
///      "properties": {
///        "vulnerable": {
///          "type": "array",
///          "items": {
///            "type": "string"
///          }
///        }
///      }
///    },
///    "severity": {
///      "type": "string"
///    },
///    "severityBasedOn": {
///      "type": "string"
///    },
///    "severityWithCritical": {
///      "type": "string"
///    },
///    "socialTrendAlert": {
///      "type": "boolean"
///    },
///    "title": {
///      "type": "string"
///    },
///    "upgradePath": {
///      "type": "array",
///      "items": {
///        "type": [
///          "boolean",
///          "string"
///        ]
///      }
///    },
///    "version": {
///      "type": "string"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct SnykContainerVulnerabilitiesItem {
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub cpes: ::std::vec::Vec<::serde_json::Value>,
    #[serde(
        rename = "creationTime",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub creation_time: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub credit: ::std::vec::Vec<::std::string::String>,
    #[serde(
        rename = "CVSSv3",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub cvs_sv3: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "cvssDetails",
        default,
        skip_serializing_if = "::std::vec::Vec::is_empty"
    )]
    pub cvss_details: ::std::vec::Vec<SnykContainerVulnerabilitiesItemCvssDetailsItem>,
    #[serde(
        rename = "cvssScore",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub cvss_score: ::std::option::Option<f64>,
    #[serde(
        rename = "cvssSources",
        default,
        skip_serializing_if = "::std::vec::Vec::is_empty"
    )]
    pub cvss_sources: ::std::vec::Vec<SnykContainerVulnerabilitiesItemCvssSourcesItem>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub description: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "disclosureTime",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub disclosure_time: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "dockerBaseImage",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub docker_base_image: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "epssDetails",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub epss_details: ::std::option::Option<SnykContainerVulnerabilitiesItemEpssDetails>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub exploit: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "exploitDetails",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub exploit_details: ::std::option::Option<SnykContainerVulnerabilitiesItemExploitDetails>,
    #[serde(
        rename = "fixedIn",
        default,
        skip_serializing_if = "::std::vec::Vec::is_empty"
    )]
    pub fixed_in: ::std::vec::Vec<::serde_json::Value>,
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub from: ::std::vec::Vec<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub id: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub identifiers: ::std::option::Option<SnykContainerVulnerabilitiesItemIdentifiers>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub insights: ::std::option::Option<SnykContainerVulnerabilitiesItemInsights>,
    #[serde(
        rename = "isDisputed",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub is_disputed: ::std::option::Option<bool>,
    #[serde(
        rename = "isPatchable",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub is_patchable: ::std::option::Option<bool>,
    #[serde(
        rename = "isUpgradable",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub is_upgradable: ::std::option::Option<bool>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub language: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub malicious: ::std::option::Option<bool>,
    #[serde(
        rename = "modificationTime",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub modification_time: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "nvdSeverity",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub nvd_severity: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "packageManager",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub package_manager: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "packageName",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub package_name: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub patches: ::std::vec::Vec<::serde_json::Value>,
    #[serde(
        rename = "publicationTime",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub publication_time: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub references: ::std::vec::Vec<SnykContainerVulnerabilitiesItemReferencesItem>,
    #[serde(
        rename = "relativeImportance",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub relative_importance: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub semver: ::std::option::Option<SnykContainerVulnerabilitiesItemSemver>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub severity: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "severityBasedOn",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub severity_based_on: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "severityWithCritical",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub severity_with_critical: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "socialTrendAlert",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub social_trend_alert: ::std::option::Option<bool>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub title: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "upgradePath",
        default,
        skip_serializing_if = "::std::vec::Vec::is_empty"
    )]
    pub upgrade_path: ::std::vec::Vec<SnykContainerVulnerabilitiesItemUpgradePathItem>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub version: ::std::option::Option<::std::string::String>,
}
impl ::std::convert::From<&SnykContainerVulnerabilitiesItem> for SnykContainerVulnerabilitiesItem {
    fn from(value: &SnykContainerVulnerabilitiesItem) -> Self {
        value.clone()
    }
}
impl ::std::default::Default for SnykContainerVulnerabilitiesItem {
    fn default() -> Self {
        Self {
            cpes: Default::default(),
            creation_time: Default::default(),
            credit: Default::default(),
            cvs_sv3: Default::default(),
            cvss_details: Default::default(),
            cvss_score: Default::default(),
            cvss_sources: Default::default(),
            description: Default::default(),
            disclosure_time: Default::default(),
            docker_base_image: Default::default(),
            epss_details: Default::default(),
            exploit: Default::default(),
            exploit_details: Default::default(),
            fixed_in: Default::default(),
            from: Default::default(),
            id: Default::default(),
            identifiers: Default::default(),
            insights: Default::default(),
            is_disputed: Default::default(),
            is_patchable: Default::default(),
            is_upgradable: Default::default(),
            language: Default::default(),
            malicious: Default::default(),
            modification_time: Default::default(),
            name: Default::default(),
            nvd_severity: Default::default(),
            package_manager: Default::default(),
            package_name: Default::default(),
            patches: Default::default(),
            publication_time: Default::default(),
            references: Default::default(),
            relative_importance: Default::default(),
            semver: Default::default(),
            severity: Default::default(),
            severity_based_on: Default::default(),
            severity_with_critical: Default::default(),
            social_trend_alert: Default::default(),
            title: Default::default(),
            upgrade_path: Default::default(),
            version: Default::default(),
        }
    }
}
///`SnykContainerVulnerabilitiesItemCvssDetailsItem`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "properties": {
///    "assigner": {
///      "type": "string"
///    },
///    "cvssV3BaseScore": {
///      "type": "number"
///    },
///    "cvssV3Vector": {
///      "type": "string"
///    },
///    "modificationTime": {
///      "type": "string"
///    },
///    "severity": {
///      "type": "string"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct SnykContainerVulnerabilitiesItemCvssDetailsItem {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub assigner: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "cvssV3BaseScore",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub cvss_v3_base_score: ::std::option::Option<f64>,
    #[serde(
        rename = "cvssV3Vector",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub cvss_v3_vector: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "modificationTime",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub modification_time: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub severity: ::std::option::Option<::std::string::String>,
}
impl ::std::convert::From<&SnykContainerVulnerabilitiesItemCvssDetailsItem>
    for SnykContainerVulnerabilitiesItemCvssDetailsItem
{
    fn from(value: &SnykContainerVulnerabilitiesItemCvssDetailsItem) -> Self {
        value.clone()
    }
}
impl ::std::default::Default for SnykContainerVulnerabilitiesItemCvssDetailsItem {
    fn default() -> Self {
        Self {
            assigner: Default::default(),
            cvss_v3_base_score: Default::default(),
            cvss_v3_vector: Default::default(),
            modification_time: Default::default(),
            severity: Default::default(),
        }
    }
}
///`SnykContainerVulnerabilitiesItemCvssSourcesItem`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "properties": {
///    "assigner": {
///      "type": "string"
///    },
///    "baseScore": {
///      "type": "number"
///    },
///    "cvssVersion": {
///      "type": "string"
///    },
///    "modificationTime": {
///      "type": "string"
///    },
///    "severity": {
///      "type": "string"
///    },
///    "type": {
///      "type": "string"
///    },
///    "vector": {
///      "type": "string"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct SnykContainerVulnerabilitiesItemCvssSourcesItem {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub assigner: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "baseScore",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub base_score: ::std::option::Option<f64>,
    #[serde(
        rename = "cvssVersion",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub cvss_version: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "modificationTime",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub modification_time: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub severity: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "type",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub type_: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub vector: ::std::option::Option<::std::string::String>,
}
impl ::std::convert::From<&SnykContainerVulnerabilitiesItemCvssSourcesItem>
    for SnykContainerVulnerabilitiesItemCvssSourcesItem
{
    fn from(value: &SnykContainerVulnerabilitiesItemCvssSourcesItem) -> Self {
        value.clone()
    }
}
impl ::std::default::Default for SnykContainerVulnerabilitiesItemCvssSourcesItem {
    fn default() -> Self {
        Self {
            assigner: Default::default(),
            base_score: Default::default(),
            cvss_version: Default::default(),
            modification_time: Default::default(),
            severity: Default::default(),
            type_: Default::default(),
            vector: Default::default(),
        }
    }
}
///`SnykContainerVulnerabilitiesItemEpssDetails`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "properties": {
///    "modelVersion": {
///      "type": "string"
///    },
///    "percentile": {
///      "type": "string"
///    },
///    "probability": {
///      "type": "string"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct SnykContainerVulnerabilitiesItemEpssDetails {
    #[serde(
        rename = "modelVersion",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub model_version: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub percentile: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub probability: ::std::option::Option<::std::string::String>,
}
impl ::std::convert::From<&SnykContainerVulnerabilitiesItemEpssDetails>
    for SnykContainerVulnerabilitiesItemEpssDetails
{
    fn from(value: &SnykContainerVulnerabilitiesItemEpssDetails) -> Self {
        value.clone()
    }
}
impl ::std::default::Default for SnykContainerVulnerabilitiesItemEpssDetails {
    fn default() -> Self {
        Self {
            model_version: Default::default(),
            percentile: Default::default(),
            probability: Default::default(),
        }
    }
}
///`SnykContainerVulnerabilitiesItemExploitDetails`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "properties": {
///    "maturityLevels": {
///      "type": "array",
///      "items": {
///        "type": "object",
///        "properties": {
///          "format": {
///            "type": "string"
///          },
///          "level": {
///            "type": "string"
///          },
///          "type": {
///            "type": "string"
///          }
///        }
///      }
///    },
///    "sources": {
///      "type": "array"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct SnykContainerVulnerabilitiesItemExploitDetails {
    #[serde(
        rename = "maturityLevels",
        default,
        skip_serializing_if = "::std::vec::Vec::is_empty"
    )]
    pub maturity_levels:
        ::std::vec::Vec<SnykContainerVulnerabilitiesItemExploitDetailsMaturityLevelsItem>,
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub sources: ::std::vec::Vec<::serde_json::Value>,
}
impl ::std::convert::From<&SnykContainerVulnerabilitiesItemExploitDetails>
    for SnykContainerVulnerabilitiesItemExploitDetails
{
    fn from(value: &SnykContainerVulnerabilitiesItemExploitDetails) -> Self {
        value.clone()
    }
}
impl ::std::default::Default for SnykContainerVulnerabilitiesItemExploitDetails {
    fn default() -> Self {
        Self {
            maturity_levels: Default::default(),
            sources: Default::default(),
        }
    }
}
///`SnykContainerVulnerabilitiesItemExploitDetailsMaturityLevelsItem`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "properties": {
///    "format": {
///      "type": "string"
///    },
///    "level": {
///      "type": "string"
///    },
///    "type": {
///      "type": "string"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct SnykContainerVulnerabilitiesItemExploitDetailsMaturityLevelsItem {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub format: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub level: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "type",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub type_: ::std::option::Option<::std::string::String>,
}
impl ::std::convert::From<&SnykContainerVulnerabilitiesItemExploitDetailsMaturityLevelsItem>
    for SnykContainerVulnerabilitiesItemExploitDetailsMaturityLevelsItem
{
    fn from(value: &SnykContainerVulnerabilitiesItemExploitDetailsMaturityLevelsItem) -> Self {
        value.clone()
    }
}
impl ::std::default::Default for SnykContainerVulnerabilitiesItemExploitDetailsMaturityLevelsItem {
    fn default() -> Self {
        Self {
            format: Default::default(),
            level: Default::default(),
            type_: Default::default(),
        }
    }
}
///`SnykContainerVulnerabilitiesItemIdentifiers`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "properties": {
///    "ALTERNATIVE": {
///      "type": "array"
///    },
///    "CVE": {
///      "type": "array",
///      "items": {
///        "type": "string"
///      }
///    },
///    "CWE": {
///      "type": "array",
///      "items": {
///        "type": "string"
///      }
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct SnykContainerVulnerabilitiesItemIdentifiers {
    #[serde(
        rename = "ALTERNATIVE",
        default,
        skip_serializing_if = "::std::vec::Vec::is_empty"
    )]
    pub alternative: ::std::vec::Vec<::serde_json::Value>,
    #[serde(
        rename = "CVE",
        default,
        skip_serializing_if = "::std::vec::Vec::is_empty"
    )]
    pub cve: ::std::vec::Vec<::std::string::String>,
    #[serde(
        rename = "CWE",
        default,
        skip_serializing_if = "::std::vec::Vec::is_empty"
    )]
    pub cwe: ::std::vec::Vec<::std::string::String>,
}
impl ::std::convert::From<&SnykContainerVulnerabilitiesItemIdentifiers>
    for SnykContainerVulnerabilitiesItemIdentifiers
{
    fn from(value: &SnykContainerVulnerabilitiesItemIdentifiers) -> Self {
        value.clone()
    }
}
impl ::std::default::Default for SnykContainerVulnerabilitiesItemIdentifiers {
    fn default() -> Self {
        Self {
            alternative: Default::default(),
            cve: Default::default(),
            cwe: Default::default(),
        }
    }
}
///`SnykContainerVulnerabilitiesItemInsights`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "properties": {
///    "triageAdvice": {
///      "type": "null"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct SnykContainerVulnerabilitiesItemInsights {
    #[serde(rename = "triageAdvice", default)]
    pub triage_advice: (),
}
impl ::std::convert::From<&SnykContainerVulnerabilitiesItemInsights>
    for SnykContainerVulnerabilitiesItemInsights
{
    fn from(value: &SnykContainerVulnerabilitiesItemInsights) -> Self {
        value.clone()
    }
}
impl ::std::default::Default for SnykContainerVulnerabilitiesItemInsights {
    fn default() -> Self {
        Self {
            triage_advice: Default::default(),
        }
    }
}
///`SnykContainerVulnerabilitiesItemReferencesItem`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "properties": {
///    "title": {
///      "type": "string"
///    },
///    "url": {
///      "type": "string"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct SnykContainerVulnerabilitiesItemReferencesItem {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub title: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub url: ::std::option::Option<::std::string::String>,
}
impl ::std::convert::From<&SnykContainerVulnerabilitiesItemReferencesItem>
    for SnykContainerVulnerabilitiesItemReferencesItem
{
    fn from(value: &SnykContainerVulnerabilitiesItemReferencesItem) -> Self {
        value.clone()
    }
}
impl ::std::default::Default for SnykContainerVulnerabilitiesItemReferencesItem {
    fn default() -> Self {
        Self {
            title: Default::default(),
            url: Default::default(),
        }
    }
}
///`SnykContainerVulnerabilitiesItemSemver`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "properties": {
///    "vulnerable": {
///      "type": "array",
///      "items": {
///        "type": "string"
///      }
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct SnykContainerVulnerabilitiesItemSemver {
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub vulnerable: ::std::vec::Vec<::std::string::String>,
}
impl ::std::convert::From<&SnykContainerVulnerabilitiesItemSemver>
    for SnykContainerVulnerabilitiesItemSemver
{
    fn from(value: &SnykContainerVulnerabilitiesItemSemver) -> Self {
        value.clone()
    }
}
impl ::std::default::Default for SnykContainerVulnerabilitiesItemSemver {
    fn default() -> Self {
        Self {
            vulnerable: Default::default(),
        }
    }
}
///`SnykContainerVulnerabilitiesItemUpgradePathItem`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": [
///    "boolean",
///    "string"
///  ]
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(untagged)]
pub enum SnykContainerVulnerabilitiesItemUpgradePathItem {
    Boolean(bool),
    String(::std::string::String),
}
impl ::std::convert::From<&Self> for SnykContainerVulnerabilitiesItemUpgradePathItem {
    fn from(value: &SnykContainerVulnerabilitiesItemUpgradePathItem) -> Self {
        value.clone()
    }
}
impl ::std::fmt::Display for SnykContainerVulnerabilitiesItemUpgradePathItem {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match self {
            Self::Boolean(x) => x.fmt(f),
            Self::String(x) => x.fmt(f),
        }
    }
}
impl ::std::convert::From<bool> for SnykContainerVulnerabilitiesItemUpgradePathItem {
    fn from(value: bool) -> Self {
        Self::Boolean(value)
    }
}
