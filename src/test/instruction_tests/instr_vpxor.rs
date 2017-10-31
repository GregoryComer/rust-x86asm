use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpxor_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPXOR, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM4)), operand3: Some(Direct(XMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 217, 239, 251], OperandSize::Dword)
}

#[test]
fn vpxor_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPXOR, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM3)), operand3: Some(Indirect(ESI, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 225, 239, 6], OperandSize::Dword)
}

#[test]
fn vpxor_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPXOR, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM6)), operand3: Some(Direct(XMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 201, 239, 246], OperandSize::Qword)
}

#[test]
fn vpxor_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPXOR, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM7)), operand3: Some(Indirect(RBX, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 193, 239, 19], OperandSize::Qword)
}

#[test]
fn vpxor_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPXOR, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM0)), operand3: Some(Direct(YMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 253, 239, 199], OperandSize::Dword)
}

#[test]
fn vpxor_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPXOR, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM1)), operand3: Some(IndirectDisplaced(EAX, 143445979, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 245, 239, 152, 219, 207, 140, 8], OperandSize::Dword)
}

#[test]
fn vpxor_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPXOR, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM1)), operand3: Some(Direct(YMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 245, 239, 249], OperandSize::Qword)
}

#[test]
fn vpxor_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPXOR, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM4)), operand3: Some(IndirectScaledIndexed(RDX, RAX, Four, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 221, 239, 52, 130], OperandSize::Qword)
}

