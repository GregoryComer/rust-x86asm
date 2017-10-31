use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn movss_1() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVSS, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 16, 243], OperandSize::Dword)
}

#[test]
fn movss_2() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVSS, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 16, 214], OperandSize::Qword)
}

#[test]
fn movss_3() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVSS, operand1: Some(Direct(XMM2)), operand2: Some(IndirectScaledIndexedDisplaced(EAX, EDX, Two, 2112846415, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 16, 148, 80, 79, 122, 239, 125], OperandSize::Dword)
}

#[test]
fn movss_4() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVSS, operand1: Some(Direct(XMM3)), operand2: Some(Indirect(RBX, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 16, 27], OperandSize::Qword)
}

#[test]
fn movss_5() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVSS, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 16, 249], OperandSize::Dword)
}

#[test]
fn movss_6() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVSS, operand1: Some(IndirectScaledIndexedDisplaced(EDI, EDX, Eight, 1586813792, Some(OperandSize::Dword), None)), operand2: Some(Direct(XMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 17, 172, 215, 96, 219, 148, 94], OperandSize::Dword)
}

#[test]
fn movss_7() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVSS, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 16, 203], OperandSize::Qword)
}

#[test]
fn movss_8() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVSS, operand1: Some(IndirectDisplaced(RAX, 732279245, Some(OperandSize::Dword), None)), operand2: Some(Direct(XMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 17, 152, 205, 177, 165, 43], OperandSize::Qword)
}

