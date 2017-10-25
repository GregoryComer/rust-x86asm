use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn movntps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVNTPS, operand1: Some(IndirectScaledIndexed(ECX, EDX, Two, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(XMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 43, 28, 81], OperandSize::Dword)
}

#[test]
fn movntps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVNTPS, operand1: Some(IndirectScaledIndexed(RCX, RSI, Four, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(XMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 43, 28, 177], OperandSize::Qword)
}

