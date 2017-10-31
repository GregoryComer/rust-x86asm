use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpxor_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPXOR, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM1)), operand3: Some(Direct(XMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 241, 239, 252], OperandSize::Dword)
}

#[test]
fn vpxor_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPXOR, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM6)), operand3: Some(Indirect(EAX, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 201, 239, 16], OperandSize::Dword)
}

#[test]
fn vpxor_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPXOR, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM0)), operand3: Some(Direct(XMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 239, 215], OperandSize::Qword)
}

#[test]
fn vpxor_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPXOR, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM0)), operand3: Some(IndirectScaledIndexed(RDI, RCX, Two, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 239, 28, 79], OperandSize::Qword)
}

#[test]
fn vpxor_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPXOR, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM0)), operand3: Some(Direct(YMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 253, 239, 195], OperandSize::Dword)
}

#[test]
fn vpxor_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPXOR, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM5)), operand3: Some(Indirect(EBX, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 213, 239, 3], OperandSize::Dword)
}

#[test]
fn vpxor_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPXOR, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM1)), operand3: Some(Direct(YMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 245, 239, 234], OperandSize::Qword)
}

#[test]
fn vpxor_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPXOR, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM3)), operand3: Some(IndirectScaledDisplaced(RAX, Two, 768387641, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 229, 239, 12, 69, 57, 170, 204, 45], OperandSize::Qword)
}

