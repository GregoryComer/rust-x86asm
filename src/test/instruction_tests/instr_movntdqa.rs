use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn movntdqa_1() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVNTDQA, operand1: Some(Direct(XMM6)), operand2: Some(IndirectScaledIndexedDisplaced(EDI, EBX, Four, 23325921, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 42, 180, 159, 225, 236, 99, 1], OperandSize::Dword)
}

#[test]
fn movntdqa_2() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVNTDQA, operand1: Some(Direct(XMM4)), operand2: Some(IndirectScaledIndexed(RAX, RSI, Four, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 42, 36, 176], OperandSize::Qword)
}

