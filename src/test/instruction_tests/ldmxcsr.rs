use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn ldmxcsr_1() {
    run_test(&Instruction { mnemonic: Mnemonic::LDMXCSR, operand1: Some(Indirect(EBX, Some(OperandSize::Dword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 174, 19], OperandSize::Dword)
}

fn ldmxcsr_2() {
    run_test(&Instruction { mnemonic: Mnemonic::LDMXCSR, operand1: Some(IndirectScaledDisplaced(RDI, Two, 319721249, Some(OperandSize::Dword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 174, 20, 125, 33, 143, 14, 19], OperandSize::Qword)
}

