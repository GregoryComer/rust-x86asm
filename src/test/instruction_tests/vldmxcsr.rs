use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn vldmxcsr_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VLDMXCSR, operand1: Some(IndirectScaledIndexedDisplaced(ESI, EDX, Two, 683715666, Some(OperandSize::Dword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 248, 174, 148, 86, 82, 172, 192, 40], OperandSize::Dword)
}

fn vldmxcsr_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VLDMXCSR, operand1: Some(IndirectDisplaced(RAX, 2124348144, Some(OperandSize::Dword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 248, 174, 144, 240, 250, 158, 126], OperandSize::Qword)
}

