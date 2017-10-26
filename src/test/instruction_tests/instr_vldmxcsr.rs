use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vldmxcsr_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VLDMXCSR, operand1: Some(IndirectScaledDisplaced(EBX, Four, 453769634, Some(OperandSize::Dword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 248, 174, 20, 157, 162, 249, 11, 27], OperandSize::Dword)
}

#[test]
fn vldmxcsr_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VLDMXCSR, operand1: Some(IndirectScaledIndexedDisplaced(RCX, RDI, Four, 9554946, Some(OperandSize::Dword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 248, 174, 148, 185, 2, 204, 145, 0], OperandSize::Qword)
}

