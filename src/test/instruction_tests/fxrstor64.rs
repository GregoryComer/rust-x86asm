use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn fxrstor64_1() {
    run_test(&Instruction { mnemonic: Mnemonic::FXRSTOR64, operand1: Some(IndirectScaledIndexedDisplaced(RCX, RDI, Four, 1964928521, Some(OperandSize::Unsized), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 15, 174, 140, 185, 9, 110, 30, 117], OperandSize::Qword)
}

