use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpxor_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPXOR, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM7)), operand3: Some(Direct(XMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 193, 239, 242], OperandSize::Dword)
}

#[test]
fn vpxor_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPXOR, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM2)), operand3: Some(IndirectDisplaced(EDX, 45788959, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 233, 239, 178, 31, 175, 186, 2], OperandSize::Dword)
}

#[test]
fn vpxor_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPXOR, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM7)), operand3: Some(Direct(XMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 193, 239, 198], OperandSize::Qword)
}

#[test]
fn vpxor_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPXOR, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM5)), operand3: Some(IndirectScaledDisplaced(RAX, Four, 1697749546, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 209, 239, 20, 133, 42, 154, 49, 101], OperandSize::Qword)
}

#[test]
fn vpxor_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPXOR, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM4)), operand3: Some(Direct(YMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 221, 239, 244], OperandSize::Dword)
}

#[test]
fn vpxor_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPXOR, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM3)), operand3: Some(IndirectScaledIndexed(EBX, ESI, Eight, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 229, 239, 28, 243], OperandSize::Dword)
}

#[test]
fn vpxor_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPXOR, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM6)), operand3: Some(Direct(YMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 205, 239, 253], OperandSize::Qword)
}

#[test]
fn vpxor_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPXOR, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM4)), operand3: Some(IndirectDisplaced(RAX, 2042031814, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 221, 239, 168, 198, 238, 182, 121], OperandSize::Qword)
}

