use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpsignw_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSIGNW, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM7)), operand3: Some(Direct(XMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 65, 9, 197], OperandSize::Dword)
}

#[test]
fn vpsignw_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSIGNW, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM4)), operand3: Some(IndirectScaledIndexedDisplaced(EDI, ESI, Four, 1448611108, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 89, 9, 172, 183, 36, 13, 88, 86], OperandSize::Dword)
}

#[test]
fn vpsignw_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSIGNW, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM0)), operand3: Some(Direct(XMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 9, 228], OperandSize::Qword)
}

#[test]
fn vpsignw_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSIGNW, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM1)), operand3: Some(IndirectScaledIndexed(RAX, RBX, Eight, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 113, 9, 12, 216], OperandSize::Qword)
}

#[test]
fn vpsignw_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSIGNW, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM6)), operand3: Some(Direct(YMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 77, 9, 202], OperandSize::Dword)
}

#[test]
fn vpsignw_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSIGNW, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM2)), operand3: Some(Indirect(EDX, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 109, 9, 26], OperandSize::Dword)
}

#[test]
fn vpsignw_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSIGNW, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM4)), operand3: Some(Direct(YMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 93, 9, 227], OperandSize::Qword)
}

#[test]
fn vpsignw_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSIGNW, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM6)), operand3: Some(IndirectScaledIndexed(RDX, RAX, Eight, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 77, 9, 36, 194], OperandSize::Qword)
}

