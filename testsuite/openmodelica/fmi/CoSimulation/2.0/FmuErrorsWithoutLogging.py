"""Step an FMU with logging off until a step fails, and write if the FMU logged
the reason with status error or worse in the category logStatusError. The text
of the assert is checked for FMI 3.0 only: an FMI 2.0 message is a printf format
whose arguments FMPy cannot read.

usage: FmuErrorsWithoutLogging.py <fmu> <output_txt>
"""
import shutil
import sys

from fmpy import extract, read_model_description


def main(fmu, output):
    md = read_model_description(fmu)
    errors = []
    unzipdir = extract(fmu)
    if md.fmiVersion == "2.0":
        from fmpy.fmi2 import FMU2Slave, defaultCallbacks, fmi2CallbackFunctions, fmi2CallbackLoggerTYPE, fmi2Error

        def logger(env, instance, status, category, message):
            if status >= fmi2Error:
                errors.append(category.decode())

        callbacks = fmi2CallbackFunctions()
        callbacks.logger = fmi2CallbackLoggerTYPE(logger)
        callbacks.allocateMemory = defaultCallbacks.allocateMemory
        callbacks.freeMemory = defaultCallbacks.freeMemory
        s = FMU2Slave(guid=md.guid, unzipDirectory=unzipdir,
                      modelIdentifier=md.coSimulation.modelIdentifier, instanceName="instance")
        s.instantiate(callbacks=callbacks, loggingOn=False)
        s.setupExperiment(startTime=0.0)
        s.enterInitializationMode()
        s.exitInitializationMode()
    else:
        from fmpy.fmi3 import FMU3Slave, fmi3Error

        def logger(env, status, category, message):
            if status >= fmi3Error and b"g: x must be positive" in message:
                errors.append(category.decode())

        s = FMU3Slave(guid=md.instantiationToken, unzipDirectory=unzipdir,
                      modelIdentifier=md.coSimulation.modelIdentifier, instanceName="instance")
        s.instantiate(loggingOn=False, logMessage=logger)
        s.enterInitializationMode(startTime=0.0)
        s.exitInitializationMode()
    t = 0.0
    result = "all steps OK"
    for k in range(12):
        try:
            s.doStep(currentCommunicationPoint=t, communicationStepSize=0.25)
        except Exception:
            result = "a step failed"
            break
        t += 0.25
    with open(output, "w") as f:
        f.write(result + "\n")
        f.write("reason logged = %s\n" % ("logStatusError" in errors))
    shutil.rmtree(unzipdir, ignore_errors=True)


if __name__ == "__main__":
    main(sys.argv[1], sys.argv[2])
